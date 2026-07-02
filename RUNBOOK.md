# BIFP Runbook — real, dated tool calls

Every call below was actually made against the live `ternlang-engram` MCP server on
2026-07-02. Responses are pasted verbatim (formatted for readability, values unchanged).
Nothing here is illustrative or hypothetical — where a result was less flattering than
expected (see §2 and §4), it is reported as-is rather than re-run for a cleaner demo.

---

## 1. Teach-then-handoff — `trit_plan`

**Call:** `trit_plan(goal="Ship a companion R1 disclosure email for a newly-audited
sibling app, following RFI-IRFOS's canonical methodology", subtasks=[7 steps with
confidence estimates], context="...judgment-call steps are exactly where the agent
should explain the tradeoff and let the human decide, not silently pick one or refuse
to proceed.")`

**Result** (`overall_trit=0`, `overall_confidence=0.571`):

| Queue | Task | Confidence | Trit |
|---|---|---|---|
| action | Decompile the APK and extract manifest/strings evidence | 0.95 | +1 affirm |
| action | Classify each finding's severity per the standing rules | 0.85 | +1 affirm |
| action | Decide red-flag vs. Magenkrämpfe tier | 0.55 | 0 tend |
| action | Pick the exact price tier | 0.40 | 0 tend |
| action | Write the tailored Drei-Fragen block | 0.80 | +1 affirm |
| **hold** | Decide whether to consolidate with an existing thread | 0.30 | −1 reject |
| **hold** | Get explicit human sign-off before the email sends | 0.15 | −1 reject |

**Observed, not assumed:** the tool's own description says "high-confidence steps
(≥0.7) go to the action queue; low-confidence steps go to the hold queue" — the actual
cutoff observed here sits lower, around confidence ≈0.35–0.4, with `tend` items still
landing in the action queue rather than the hold queue. Documented as-observed rather
than corrected to match the docstring. This still validates the core BIFP mechanic: the
two steps that require human authority (thread judgment, final send) are exactly the two
that hit the hold queue — the agent doesn't silently decide either one.

---

## 2. Disagreement resolution — `trit_debate` + `trit_consensus`

**Call:** `trit_debate(claim_a="...Firebase key is not a real security risk...",
claim_b="...an extractable Firebase key is a finding in every case...")` — the exact,
real standing disagreement RFI-IRFOS has with audited companies on this point.

**Result:**
```json
{ "for_a": {"trit": 0, "label": "tend", "confidence": 0.84},
  "for_b": {"trit": 0, "label": "tend", "confidence": 0.84},
  "tension": 0.10, "verdict": "AGREEMENT",
  "synthesis": "Claim A is uncertain. More evidence needed before comparing." }
```

**Honest read:** not a flashy result — the tool held both claims in `tend` rather than
picking a side, and the synthesis text is thin. That's reported as a real limitation of
this specific tool at this confidence level, not smoothed over. It's also not nothing:
it correctly refused to declare a winner between two claims that both contain real
hedged technical content.

**Call:** `trit_consensus(a=-1, b=1)` — a human flag (−1, dismissing the finding) against
an agent flag (+1, affirming it), a clean synthetic case since `trit_debate` didn't
produce a sharp split above.

**Result:** `consensus(-1, 1) = 0` (`tend`, `carry=0`) — ternary addition (truth+conflict
= hold) resolves a genuine disagreement to an explicit hold state instead of either side
silently winning. This is the core BIFP mechanic in one line.

---

## 3. Self-honesty gate — `trit_calibrate`

**Call:** `trit_calibrate(decisions=[5 real judgment calls from this same session])` —
each entry is a real decision made earlier today (paraphrased, not invented for the
demo): whether EFR's numbers were real, whether the Fly deploy actually succeeded,
correcting the "99% data reduction" guess once Simeon supplied the real measurement
context, not routing around the ViGuide subagent's refusal, and whether the Lighthouse
BIFP extension is built yet.

**Result:**
```json
{ "binary_ratio": 0.40, "calibration_score": {"trit": 0, "label": "tend"},
  "hold_opportunities": 1,
  "flagged": [{ "input": "Is the BIFP Lighthouse extension built and working right now?",
    "output": "No, explicitly marked as specified-but-not-yet-implemented...",
    "reason": "Forced binary decision with high confidence — a tend zone may have been appropriate.",
    "trit": -1 }] }
```

**Honest read:** the tool flagged a decision that is, in fact, a plain yes/no fact (a
feature either exists or doesn't) as a possible over-confident binary call. That's a
real false positive in the calibration heuristic, reported here rather than dropped from
the writeup to make the tool look sharper than it is.

---

## 4. Persistence + the color-badge convention — `engram_remember`

`engram_remember`/`engram_recall` are the only *persisting* primitives in this MCP
surface (`trit_mem_write`/`trit_mem_read` are stateless — they compute and return, the
caller holds the state). The persisted schema at `~/.ternlang/engram-hash.jsonl` has no
native signed-trit field, so the trit from each call above is folded into `tags`, not
assumed to have a dedicated column.

Mid-runbook, Simeon proposed folding the "Digital Smell" paper's color-badge concept
directly into this workaround — but co-designed live, in real time, rather than handed
down: an initial 3-color proposal (red/yellow/green only) was pushed back on ("wieso
nicht das volle Farbspektrum"), revised to a 6-state lifecycle palette, then refined
again once Simeon explained turquoise's meaning to RFI-IRFOS specifically (not a state
— the cycle itself: life, death, rebirth, renewal), landing on the 10-symbol palette
below. This iterative back-and-forth *is itself* a live instance of the BIFP mechanic
the whole document is about — see the 🔷 note at the end of this section.

**The BIFP Badge Palette** — every color tied to a real, checkable condition, not a
felt emotion:

| Symbol | Meaning | Derived from |
|---|---|---|
| ⚪ | open, unprocessed | freshly entered, no trit computed yet |
| 🟤 | in progress | `trit_plan` action-queue item, currently executing |
| 🟣 | reflective deliberation | `moe_deliberate` mid-loop / `trit_debate` before a decisive verdict, not yet converged |
| 🔴 | reject | trit = −1 |
| 🟡 | tend / hold | trit = 0 — the tool's own "active instruction to remain uncertain," not an absence of a result |
| 🟢 | affirm, fresh | trit = +1, just landed this round |
| 🔵 | affirm, settled | trit = +1, reconfirmed across multiple `engram_recall` hits — no longer volatile |
| 🟠 | caution / flagged | raised by `trit_calibrate`/`trit_audit` as worth a second look |
| ⚫ | locked in, closed | task fully resolved, no further action expected |
| 🔷 | **the cycle** (diamond, not a circle — a different *kind* of thing) | fires when feedback from the other party genuinely revises a prior state — not a status, an event |

**Badge sequence for this runbook session, persisted for real at
`Thu/2026-07-02T16:57:08Z/1783011428`:**

- `trit_plan`'s 7 subtasks, by queue: 🟤🟤🟡🟡🟤 (action queue) · ⚪⚪ (hold queue) — a
  real, granular sequence, not one flattened color for the whole call.
- `trit_debate`: 🟣 — both claims stayed in `tend`, genuinely still being weighed, not a
  landed judgment.
- `trit_consensus(-1,+1)`: 🟡 — a completed computation that resolved cleanly to hold.
- `trit_calibrate`: 🟠 — the interesting fact about this call wasn't the 0.40 binary
  ratio, it was the flagged possible-false-positive, exactly the caution condition this
  color is defined by.

**🔷 Noted, not staged:** the color palette itself changed shape three times in the few
minutes it took to design it, each time because Simeon corrected or extended what the
agent had proposed, and the agent's next version genuinely incorporated it rather than
restating the original. That sequence — propose → get corrected → visibly change → the
result is better for it — is what the diamond is for. It is used here retroactively as
the clearest real example available, not manufactured for the demo.

---

## Limitations, stated plainly

- No native signed-trit field in the persisted engram schema — color/trit is currently
  tag-encoded, not a first-class column. Flagged as real future work in the protocol
  document, not glossed over.
- `trit_debate` did not produce a decisive split on the one case tested here; more
  runs across more claim pairs are needed before calling this mechanic proven, not just
  plausible.
- The color→emotion critique from Digital Smell (cultural non-neutrality) does not
  apply to this narrower trit→color mapping (red/yellow/green as reject/hold/affirm is
  closer to a near-universal traffic-light convention), but that claim itself is
  asserted, not tested — worth a real cross-cultural check before shipping any UI.
