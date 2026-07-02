# BIFP — Bidirectional Instant Feedback Protocol

Status: **v0.1 draft**, 2026-07-02.

BIFP is RFI-IRFOS's protocol for genuinely bidirectional human↔agent collaboration: not
just the human correcting the agent, but the agent flagging things back to the human in
real time, and — when the human can't do a step themselves — teaching them through it
step by step instead of either refusing outright or silently completing it for them.

- [`docs/bifp-protocol.tex`](docs/bifp-protocol.tex) — the v0.1 spec.
- [`RUNBOOK.md`](RUNBOOK.md) — real, dated tool-call transcripts proving the core
  mechanic runs today on `ternlang-engram`'s existing MCP tools. Nothing in the runbook
  is hypothetical; every response pasted there was actually returned by a live call.

Origin: Simeon Kepp, ~2021. First formalized as a written protocol 2026-07-02, building
on RFI-IRFOS's existing Policy Mirror Protocol, the `rlhf.rs` ternary-rating tool in
Lighthouse, and the ternary hold/conflict formalism already in the Ternlang language.

RFI-IRFOS · ZVR 1015608684 · GISA 39261441 · Steuernummer 68 028/0989 · Graz, Austria
