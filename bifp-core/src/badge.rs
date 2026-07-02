//! The BIFP Badge Palette — 10 symbols, co-designed live with Simeon Kepp on 2026-07-02.
//! Nine states plus one event. Every symbol maps to a concrete, checkable condition; none
//! are a felt emotion assigned by unexplained intuition (the gap identified in the older
//! "Digital Smell and Chromatic Emotion Framework" concept paper this protocol responds to).

use crate::trit::Trit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Badge {
    /// Freshly entered, no trit computed yet.
    White,
    /// Currently executing in the action queue.
    Brown,
    /// Mid-deliberation, not yet converged (e.g. inside a multi-round evidence loop).
    Purple,
    /// trit = reject.
    Red,
    /// trit = tend/hold.
    Yellow,
    /// trit = affirm, just landed this round.
    GreenFresh,
    /// trit = affirm, reconfirmed across multiple independent recalls — settled.
    BlueSettled,
    /// Raised by a calibration/audit pass as worth a second look.
    Orange,
    /// Task fully resolved, no further action expected. Rendered as a muted dark tone in
    /// UI contexts with a dark background, not literal black — see docs/bifp-spec.json.
    Closed,
    /// Not a status. Fires exactly when feedback from the other party genuinely revises
    /// a prior state. The only non-circular symbol in the set, deliberately.
    CycleEvent,
}

/// Inputs a caller supplies alongside a bare trit, since lifecycle position isn't
/// derivable from a trit+confidence pair alone.
#[derive(Debug, Clone, Copy, Default)]
pub struct BadgeContext {
    pub in_progress: bool,
    pub deliberating: bool,
    pub flagged_by_calibration: bool,
    pub closed: bool,
    /// How many independent recalls have reconfirmed this same affirm. >1 => settled/blue.
    pub reconfirm_count: u32,
    /// True exactly when this flag revises an earlier one because of feedback from the
    /// other party (Flag::revises being Some(..) is the usual trigger).
    pub is_cycle_event: bool,
}

pub fn derive(trit: Trit, ctx: BadgeContext) -> Badge {
    if ctx.is_cycle_event {
        return Badge::CycleEvent;
    }
    if ctx.closed {
        return Badge::Closed;
    }
    if ctx.flagged_by_calibration {
        return Badge::Orange;
    }
    if ctx.deliberating {
        return Badge::Purple;
    }
    if ctx.in_progress {
        return Badge::Brown;
    }
    match trit {
        Trit::Reject => Badge::Red,
        Trit::Tend => Badge::Yellow,
        Trit::Affirm => {
            if ctx.reconfirm_count > 1 {
                Badge::BlueSettled
            } else {
                Badge::GreenFresh
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_event_overrides_everything() {
        let ctx = BadgeContext { closed: true, is_cycle_event: true, ..Default::default() };
        assert_eq!(derive(Trit::Affirm, ctx), Badge::CycleEvent);
    }

    #[test]
    fn settled_vs_fresh_affirm() {
        let fresh = BadgeContext::default();
        let settled = BadgeContext { reconfirm_count: 3, ..Default::default() };
        assert_eq!(derive(Trit::Affirm, fresh), Badge::GreenFresh);
        assert_eq!(derive(Trit::Affirm, settled), Badge::BlueSettled);
    }
}
