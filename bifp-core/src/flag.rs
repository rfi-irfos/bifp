//! The BIFP flag tuple — the unit both parties emit to each other.

use crate::trit::Trit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "mcp", derive(schemars::JsonSchema))]
pub enum Source {
    Human,
    Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "mcp", derive(schemars::JsonSchema))]
pub struct Flag {
    pub trit: i8,
    pub confidence: f64,
    pub note: String,
    pub ts: String,
    pub source: Source,
    /// Set only when this flag revises an earlier one because the other party's feedback
    /// changed it — the palette's diamond event, not a status. None for a fresh flag.
    pub revises: Option<u64>,
}

impl Flag {
    pub fn new(trit: Trit, confidence: f64, note: impl Into<String>, source: Source, ts: impl Into<String>) -> Self {
        Flag {
            trit: trit.as_i8(),
            confidence: confidence.clamp(0.0, 1.0),
            note: note.into(),
            ts: ts.into(),
            source,
            revises: None,
        }
    }

    pub fn trit(&self) -> Trit {
        Trit::from_i8(self.trit)
    }

    pub fn revising(mut self, earlier_id: u64) -> Self {
        self.revises = Some(earlier_id);
        self
    }
}
