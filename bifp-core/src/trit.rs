//! The native ternary value BIFP is built on: -1 (reject), 0 (tend/hold), +1 (affirm).
//!
//! This is a from-scratch, honestly-scoped reimplementation matching the *observed I/O
//! behavior* of RFI-IRFOS's hosted `ternlang-engram` MCP tools (whose source is not
//! available on this machine — only their live responses were used as a behavioral
//! reference). It does not claim to be, extend, or replicate that tool's internals.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Trit {
    Reject,
    Tend,
    Affirm,
}

impl Trit {
    /// Signed integer form: -1 / 0 / +1.
    pub fn as_i8(self) -> i8 {
        match self {
            Trit::Reject => -1,
            Trit::Tend => 0,
            Trit::Affirm => 1,
        }
    }

    pub fn from_i8(v: i8) -> Self {
        match v {
            v if v < 0 => Trit::Reject,
            0 => Trit::Tend,
            _ => Trit::Affirm,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Trit::Reject => "reject",
            Trit::Tend => "tend",
            Trit::Affirm => "affirm",
        }
    }

    /// Ternary consensus (balanced-ternary addition, saturating): truth+truth=truth,
    /// conflict+conflict=conflict, anything mixed with the other polarity = hold.
    /// Matches the observed behavior of `trit_consensus(-1, 1) = 0` from the live runbook.
    pub fn consensus(a: Trit, b: Trit) -> Trit {
        match (a, b) {
            (Trit::Affirm, Trit::Affirm) => Trit::Affirm,
            (Trit::Reject, Trit::Reject) => Trit::Reject,
            (Trit::Tend, x) | (x, Trit::Tend) => x,
            _ => Trit::Tend, // Affirm vs Reject: genuine disagreement, hold.
        }
    }

    /// Scalar evidence -> zone classification. Mirrors the three-zone model observed
    /// from `trit_decide`: affirm (+0.33, +1.0], tend [-0.33, +0.33], reject [-1.0, -0.33).
    pub fn from_evidence(mean: f64) -> Trit {
        if mean > 0.33 {
            Trit::Affirm
        } else if mean < -0.33 {
            Trit::Reject
        } else {
            Trit::Tend
        }
    }
}

/// Mean of a evidence vector on [-1.0, 1.0], plus the classified trit and a confidence
/// score (distance from the nearest zone boundary, normalized to [0,1]).
pub fn decide(evidence: &[f64]) -> (Trit, f64) {
    if evidence.is_empty() {
        return (Trit::Tend, 0.0);
    }
    let mean: f64 = evidence.iter().sum::<f64>() / evidence.len() as f64;
    let trit = Trit::from_evidence(mean);
    let confidence = (mean.abs()).min(1.0);
    (trit, confidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consensus_matches_observed_runbook_result() {
        // trit_consensus(-1, 1) = 0, observed live 2026-07-02.
        assert_eq!(Trit::consensus(Trit::Reject, Trit::Affirm), Trit::Tend);
    }

    #[test]
    fn consensus_agreement_holds() {
        assert_eq!(Trit::consensus(Trit::Affirm, Trit::Affirm), Trit::Affirm);
        assert_eq!(Trit::consensus(Trit::Reject, Trit::Reject), Trit::Reject);
    }

    #[test]
    fn consensus_tend_is_absorbing_into_the_other_side() {
        assert_eq!(Trit::consensus(Trit::Tend, Trit::Affirm), Trit::Affirm);
        assert_eq!(Trit::consensus(Trit::Reject, Trit::Tend), Trit::Reject);
    }

    #[test]
    fn decide_zones() {
        assert_eq!(decide(&[0.9, 0.8]).0, Trit::Affirm);
        assert_eq!(decide(&[-0.9]).0, Trit::Reject);
        assert_eq!(decide(&[0.1, -0.1]).0, Trit::Tend);
        assert_eq!(decide(&[]).0, Trit::Tend);
    }
}
