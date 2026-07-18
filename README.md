# BIFP — Bidirectional Instant Feedback Protocol

## Human rights are not subject to negotiation.



Status: **v0.2 draft**, 2026-07-02 (v0.1 the same day, hours earlier).

BIFP is RFI-IRFOS's protocol for genuinely bidirectional human↔Digital-Intelligence
collaboration: not just the human correcting the agent, but the agent flagging things
back to the human in real time, and — when the human can't do a step themselves —
teaching them through it step by step instead of either refusing outright or silently
completing it for them.

- [`bifp-core/`](bifp-core/) — a real, tested Rust crate (14 tests) implementing every
  primitive natively: a ternary `Trit` type, the flag tuple, teach-then-handoff
  planning, a TAP-inspired hold gate, the badge palette, and a native-trit persistence
  layer. Also builds as a real MCP server (`bifp-mcp-server`), verified against a live
  JSON-RPC handshake, not just unit tests.
- [`docs/bifp-protocol.tex`](docs/bifp-protocol.tex) — the v0.2 spec, including a section
  logging real bidirectional moments from the same evening as primary evidence.
- [`RUNBOOK.md`](RUNBOOK.md) — real, dated tool-call transcripts proving the core
  mechanic runs today on `ternlang-engram`'s existing MCP tools. Nothing in the runbook
  is hypothetical; every response pasted there was actually returned by a live call.

Origin: Simeon Kepp, ~2021. First formalized as a written protocol 2026-07-02, building
on RFI-IRFOS's existing Policy Mirror Protocol, the `rlhf.rs` ternary-rating tool in
Lighthouse, and the ternary hold/conflict formalism already in the Ternlang language.

RFI-IRFOS · ZVR 1015608684 · GISA 39261441 · Steuernummer 68 028/0989 · Graz, Austria
