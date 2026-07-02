# bifp-core

Native Rust primitives for BIFP (Bidirectional Instant Feedback Protocol) — RFI-IRFOS's
protocol for genuinely bidirectional human/Digital-Intelligence collaboration: the agent
flags things back to the human in real time too, and when a human can't do a step alone,
the agent teaches it step by step instead of refusing outright or silently completing it.

Full protocol spec: [`../docs/bifp-protocol.tex`](../docs/bifp-protocol.tex) (also in this
repo). Real, dated tool-call transcripts: [`../RUNBOOK.md`](../RUNBOOK.md).

## What's in here

- `trit` — the native `-1/0/+1` value, `decide()` (scalar evidence → reject/tend/affirm),
  `Trit::consensus()` (ternary addition for resolving disagreement).
- `flag` — the BIFP flag tuple `{trit, confidence, note, ts, source, revises}`.
- `plan` — teach-then-handoff task decomposition: an action queue the agent proceeds on,
  and a hold queue of steps needing human input or authority.
- `tap` — a TAP-inspired hold-until-resolved gate (see module docs for exactly what this
  does and does not claim to be).
- `badge` — the 10-symbol BIFP Badge Palette, co-designed live with Simeon Kepp on
  2026-07-02. Every symbol maps to a checkable condition, not a felt emotion.
- `store` — real, native-trit flag persistence (JSONL, append-only). The trit is a first-
  class field here, unlike the tag-encoded workaround this crate exists to replace.

## Provenance discipline

Every module states plainly what's original code, what was designed to match an observed
external behavior (and how that was verified), and what remains explicitly unimplemented.
See each module's doc comment. Nothing in this crate claims to be, extend, or patch
RFI-IRFOS's separately published `ternlang-engram`/`ternlang-py` packages, or the (not
found on disk, unverified) "TAP" mentioned in the Ternlang whitepaper's Implementation
Status section.

## MCP server

Build with the `mcp` feature to get a real, tested MCP server over stdio:

```bash
cargo build --release --features mcp
./target/release/bifp-mcp-server
```

Exposes `bifp_decide`, `bifp_consensus`, `bifp_plan`, `bifp_tap_evaluate`, `bifp_badge`,
`bifp_remember`, `bifp_recall`. Flags persist to `~/.bifp/flags.jsonl` by default
(override with `BIFP_STORE_PATH`).

RFI-IRFOS · ZVR 1015608684 · GISA 39261441 · Steuernummer 68 028/0989 · Graz, Austria
