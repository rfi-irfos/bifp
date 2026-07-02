//! bifp-core — Bidirectional Instant Feedback Protocol, native primitives.
//!
//! Real, tested Rust implementing the mechanics described in BIFP v0.1
//! (`docs/bifp-protocol.tex` in this repository): a ternary flag tuple, teach-then-handoff
//! task decomposition, ternary consensus for resolving disagreement, a TAP-inspired
//! hold-until-resolved primitive, the co-designed badge palette, and — the thing the
//! hosted `ternlang-engram` MCP server's persisted schema doesn't have — a store where
//! the trit is a real, native field, not a value folded into a tags array.
//!
//! Provenance discipline (same one the protocol document itself insists on): every
//! module here is original code written for BIFP. Nothing in this crate is, extends, or
//! claims to be part of the separately published `ternlang-engram`/`ternlang-py`
//! packages or the (not present on disk, unverified) "TAP" mentioned in the Ternlang
//! whitepaper. Where this crate's behavior was designed to match something observed
//! (e.g. `Trit::consensus`), that provenance is stated in the relevant module's docs.

pub mod badge;
pub mod flag;
pub mod plan;
pub mod store;
pub mod tap;
pub mod trit;

pub use badge::{derive as derive_badge, Badge, BadgeContext};
pub use flag::{Flag, Source};
pub use plan::{plan, Plan, Subtask};
pub use store::FlagStore;
pub use tap::{evaluate as tap_evaluate, resolve as tap_resolve, Signal};
pub use trit::{decide, Trit};
