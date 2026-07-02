//! bifp-mcp-server — exposes bifp-core's primitives as real MCP tools over stdio.
//!
//! Run: `bifp-mcp-server` (stdio transport, same convention as any local MCP server).
//! Flags persist to `~/.bifp/flags.jsonl` by default (override with `BIFP_STORE_PATH`).

use bifp_core::{
    badge::{derive as derive_badge, BadgeContext},
    plan::{plan as make_plan, Plan, Subtask},
    store::FlagStore,
    tap::{evaluate as tap_evaluate, Signal},
    trit::{decide, Trit},
    Flag, Source,
};
use rmcp::{
    handler::server::wrapper::{Json, Parameters},
    model::ServerInfo,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler, ServiceExt,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn store_path() -> PathBuf {
    std::env::var("BIFP_STORE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut p = dirs_home();
            p.push(".bifp");
            p.push("flags.jsonl");
            p
        })
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("."))
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct DecideParams {
    /// Evidence values on [-1.0, 1.0]. Positive = supporting, negative = contradicting.
    evidence: Vec<f64>,
}
#[derive(Debug, Serialize, schemars::JsonSchema)]
struct DecideOutput {
    trit: i8,
    label: String,
    confidence: f64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct ConsensusParams {
    /// -1 / 0 / +1
    a: i8,
    /// -1 / 0 / +1
    b: i8,
}
#[derive(Debug, Serialize, schemars::JsonSchema)]
struct ConsensusOutput {
    trit: i8,
    label: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct SubtaskIn {
    label: String,
    /// 0..1 — how confidently the agent can do this step alone.
    confidence: f64,
}
#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct PlanParams {
    goal: String,
    subtasks: Vec<SubtaskIn>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct TapParams {
    evidence: Vec<f64>,
}
#[derive(Debug, Serialize, schemars::JsonSchema)]
struct TapOutput {
    held: bool,
    trit: Option<i8>,
    confidence: f64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct BadgeParams {
    trit: i8,
    #[serde(default)]
    in_progress: bool,
    #[serde(default)]
    deliberating: bool,
    #[serde(default)]
    flagged_by_calibration: bool,
    #[serde(default)]
    closed: bool,
    #[serde(default)]
    reconfirm_count: u32,
    #[serde(default)]
    is_cycle_event: bool,
}
#[derive(Debug, Serialize, schemars::JsonSchema)]
struct BadgeOutput {
    badge: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct RememberParams {
    trit: i8,
    confidence: f64,
    note: String,
    /// "human" or "agent"
    source: String,
    ts: String,
}
#[derive(Debug, Serialize, schemars::JsonSchema)]
struct RememberOutput {
    stored: bool,
    total_flags: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct RecallParams {
    /// Only return flags whose note starts with this prefix. Empty = all.
    #[serde(default)]
    note_prefix: String,
}
#[derive(Debug, Serialize, schemars::JsonSchema)]
struct RecallOutput {
    flags: Vec<Flag>,
}

#[derive(Clone)]
struct BifpServer;

#[tool_router]
impl BifpServer {
    #[tool(name = "bifp_decide", description = "Scalar ternary decision from an evidence vector on [-1,1]. Returns reject/tend/affirm plus confidence.")]
    fn bifp_decide(&self, Parameters(p): Parameters<DecideParams>) -> Json<DecideOutput> {
        let (trit, confidence) = decide(&p.evidence);
        Json(DecideOutput { trit: trit.as_i8(), label: trit.label().to_string(), confidence })
    }

    #[tool(name = "bifp_consensus", description = "Ternary consensus between two trits (-1/0/+1): truth+truth=truth, conflict+conflict=conflict, mixed=hold.")]
    fn bifp_consensus(&self, Parameters(p): Parameters<ConsensusParams>) -> Json<ConsensusOutput> {
        let t = Trit::consensus(Trit::from_i8(p.a), Trit::from_i8(p.b));
        Json(ConsensusOutput { trit: t.as_i8(), label: t.label().to_string() })
    }

    #[tool(name = "bifp_plan", description = "Teach-then-handoff task decomposition. Splits subtasks into an action queue (agent proceeds) and a hold queue (needs human input/authority).")]
    fn bifp_plan(&self, Parameters(p): Parameters<PlanParams>) -> Json<Plan> {
        let subtasks: Vec<Subtask> = p
            .subtasks
            .into_iter()
            .map(|s| Subtask { label: s.label, confidence: s.confidence })
            .collect();
        Json(make_plan(p.goal, &subtasks))
    }

    #[tool(name = "bifp_tap_evaluate", description = "TAP-inspired hold-until-resolved gate. Decisive evidence resolves immediately; ambiguous (tend) evidence holds instead of forcing a decision, until explicitly resolved elsewhere.")]
    fn bifp_tap_evaluate(&self, Parameters(p): Parameters<TapParams>) -> Json<TapOutput> {
        match tap_evaluate(&p.evidence) {
            Signal::Resolved(t) => Json(TapOutput { held: false, trit: Some(t.as_i8()), confidence: 1.0 }),
            Signal::Held { confidence } => Json(TapOutput { held: true, trit: None, confidence }),
        }
    }

    #[tool(name = "bifp_badge", description = "Derive one of the 10 BIFP badge palette symbols from a trit plus lifecycle context (in_progress/deliberating/flagged/closed/reconfirm_count/is_cycle_event).")]
    fn bifp_badge(&self, Parameters(p): Parameters<BadgeParams>) -> Json<BadgeOutput> {
        let ctx = BadgeContext {
            in_progress: p.in_progress,
            deliberating: p.deliberating,
            flagged_by_calibration: p.flagged_by_calibration,
            closed: p.closed,
            reconfirm_count: p.reconfirm_count,
            is_cycle_event: p.is_cycle_event,
        };
        let badge = derive_badge(Trit::from_i8(p.trit), ctx);
        Json(BadgeOutput { badge: format!("{badge:?}") })
    }

    #[tool(name = "bifp_remember", description = "Persist a BIFP flag {trit, confidence, note, source, ts} with a real native trit field — not tag-encoded.")]
    fn bifp_remember(&self, Parameters(p): Parameters<RememberParams>) -> Json<RememberOutput> {
        let source = if p.source.eq_ignore_ascii_case("human") { Source::Human } else { Source::Agent };
        let flag = Flag::new(Trit::from_i8(p.trit), p.confidence, p.note, source, p.ts);
        let store = FlagStore::open(store_path());
        let ok = store.remember(&flag).is_ok();
        let total = store.recall_all().map(|v| v.len() as u32).unwrap_or(0);
        Json(RememberOutput { stored: ok, total_flags: total })
    }

    #[tool(name = "bifp_recall", description = "Recall persisted BIFP flags, optionally filtered by a note prefix.")]
    fn bifp_recall(&self, Parameters(p): Parameters<RecallParams>) -> Json<RecallOutput> {
        let store = FlagStore::open(store_path());
        let all = store.recall_all().unwrap_or_default();
        let filtered: Vec<_> = if p.note_prefix.is_empty() {
            all
        } else {
            all.into_iter().filter(|f| f.note.starts_with(&p.note_prefix)).collect()
        };
        Json(RecallOutput { flags: filtered })
    }
}

#[tool_handler]
impl ServerHandler for BifpServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.instructions = Some(
            "BIFP (Bidirectional Instant Feedback Protocol) primitives: ternary decide/consensus, \
             teach-then-handoff planning, a TAP-inspired hold gate, the 10-symbol badge palette, \
             and native-trit flag persistence. See github.com/rfi-irfos/bifp".into(),
        );
        info
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = BifpServer;
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
