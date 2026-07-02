//! Teach-then-handoff task decomposition — the mechanic behind BIFP's core complaint:
//! an agent should neither refuse a step outright nor silently complete it invisibly.
//! Confident steps proceed; steps needing human authority or context are surfaced
//! explicitly as things to teach or hand back, not silently skipped.

use crate::trit::{decide, Trit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtask {
    pub label: String,
    /// Evidence on [-1.0, 1.0] for how confidently the agent can proceed alone.
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "mcp", derive(schemars::JsonSchema))]
pub struct PlannedTask {
    pub label: String,
    pub confidence: f64,
    pub trit: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "mcp", derive(schemars::JsonSchema))]
pub struct Plan {
    pub goal: String,
    /// Steps the agent proceeds on directly.
    pub action_queue: Vec<PlannedTask>,
    /// Steps that need human input, authority, or teaching before they can proceed —
    /// not a dead end: the explicit list of what to explain or hand back.
    pub hold_queue: Vec<PlannedTask>,
    pub overall_trit: i8,
}

/// Splits subtasks into an action queue and a hold queue. A subtask lands in the hold
/// queue only when its own confidence classifies as Reject — Tend and Affirm steps
/// proceed. `confidence` is a 0..1 "can I do this alone" score, remapped to the -1..1
/// evidence scale via `(c - 0.5) * 2` before classification.
///
/// The remapping was reverse-engineered from the reference tool's live output on
/// 2026-07-02 (confidence 0.95/0.85/0.80 -> affirm, 0.55/0.40 -> tend, 0.30/0.15 ->
/// reject) rather than from its docstring, which claims a plain >=0.7 cutoff that the
/// observed output does not actually match. This reimplementation follows what was
/// observed, not what was documented, and says so.
pub fn plan(goal: impl Into<String>, subtasks: &[Subtask]) -> Plan {
    let mut action_queue = Vec::new();
    let mut hold_queue = Vec::new();
    let mut trits = Vec::new();

    for s in subtasks {
        let evidence = (s.confidence - 0.5) * 2.0;
        let (trit, _conf) = decide(&[evidence]);
        trits.push(evidence);
        let pt = PlannedTask {
            label: s.label.clone(),
            confidence: s.confidence,
            trit: trit.as_i8(),
        };
        if trit == Trit::Reject {
            hold_queue.push(pt);
        } else {
            action_queue.push(pt);
        }
    }

    let (overall, _c) = decide(&trits);
    Plan {
        goal: goal.into(),
        action_queue,
        hold_queue,
        overall_trit: overall.as_i8(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_confidence_steps_go_to_hold_queue() {
        let subtasks = vec![
            Subtask { label: "decompile".into(), confidence: 0.95 },
            Subtask { label: "get human sign-off".into(), confidence: 0.15 },
        ];
        let p = plan("ship it", &subtasks);
        assert_eq!(p.action_queue.len(), 1);
        assert_eq!(p.hold_queue.len(), 1);
        assert_eq!(p.hold_queue[0].label, "get human sign-off");
    }

    /// Reproduces the exact 7-subtask real-world case from the 2026-07-02 runbook.
    #[test]
    fn matches_observed_runbook_split() {
        let subtasks = vec![
            Subtask { label: "decompile".into(), confidence: 0.95 },
            Subtask { label: "classify severity".into(), confidence: 0.85 },
            Subtask { label: "red-flag vs magenkraempfe".into(), confidence: 0.55 },
            Subtask { label: "pick price tier".into(), confidence: 0.40 },
            Subtask { label: "drei fragen block".into(), confidence: 0.80 },
            Subtask { label: "thread consolidation".into(), confidence: 0.30 },
            Subtask { label: "human sign-off".into(), confidence: 0.15 },
        ];
        let p = plan("ship a companion R1", &subtasks);
        assert_eq!(p.action_queue.len(), 5);
        assert_eq!(p.hold_queue.len(), 2);
        assert_eq!(p.hold_queue[0].label, "thread consolidation");
        assert_eq!(p.hold_queue[1].label, "human sign-off");
    }
}
