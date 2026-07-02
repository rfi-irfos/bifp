//! A real, honestly-scoped implementation of the *behavior* the Ternlang whitepaper's
//! Implementation Status section describes for "TAP" (Ternary Actuator Protocol) in one
//! sentence: an interception layer that suspends uncertain (tend) nodes until a human or
//! upstream agent resolves the signal.
//!
//! No TAP source code was found anywhere on this machine, under any name, at the time
//! this was written (2026-07-02) — this module does not extend, wrap, or claim to be
//! that project's TAP. It is BIFP's own primitive, built to match the *described*
//! behavior, named separately so the two are never confused in provenance.

use crate::trit::{decide, Trit};

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    /// The evidence was decisive — the action may proceed with this trit.
    Resolved(Trit),
    /// The evidence landed in the tend zone. This is not a failure state — it is the
    /// point of the mechanism: don't force a decision, suspend and wait.
    Held { confidence: f64 },
}

/// Evaluate evidence. Reject/Affirm resolve immediately; Tend holds instead of forcing
/// a binary decision, mirroring the whitepaper's described suspend-on-uncertainty behavior.
pub fn evaluate(evidence: &[f64]) -> Signal {
    let (trit, confidence) = decide(evidence);
    match trit {
        Trit::Tend => Signal::Held { confidence },
        resolved => Signal::Resolved(resolved),
    }
}

/// A human or upstream agent resolves a held signal explicitly — the suspend is lifted
/// only by an outside decision, never by the same evidence re-evaluating itself.
pub fn resolve(_held: Signal, resolution: Trit) -> Trit {
    resolution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decisive_evidence_resolves_immediately() {
        assert_eq!(evaluate(&[0.9]), Signal::Resolved(Trit::Affirm));
        assert_eq!(evaluate(&[-0.9]), Signal::Resolved(Trit::Reject));
    }

    #[test]
    fn ambiguous_evidence_holds_instead_of_forcing_a_decision() {
        match evaluate(&[0.1, -0.05]) {
            Signal::Held { .. } => {}
            other => panic!("expected Held, got {other:?}"),
        }
    }

    #[test]
    fn a_held_signal_can_be_explicitly_resolved() {
        let held = evaluate(&[0.0]);
        assert_eq!(resolve(held, Trit::Affirm), Trit::Affirm);
    }
}
