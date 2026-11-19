# Final fallback record — user-directed residual Data (2026-11-18)

Retained for archive. This records a **user-directed fallback**, not an evidence-backed
classification. It is reported separately from proven classification everywhere.

## Authorization
Explicit user instruction for this batch: when at most 10 physical ROM bytes remain
unclassified after a terminal incomplete handoff, mark them as generic Data with
unknown meaning using map.annotate with Manual provenance. The user stated this
overrides the workspace rules requiring further structural proof for these residual
bytes only, and forbade returning to open-ended redirect research for them.

## Exact range (verified before editing)
- Complement recomputed from analysis/state.json at pass-003 (sha a60ef9e5…): exactly
  1 unknown byte. Physical offset 9732 (0x2604), length 1; CPU $E5F4, PRG bank 1
  (context prg_banks [0,1,0,1]); between DrawTitleCursor's final RTS and Random.
- 1 <= 10, so the exception applied. No other bytes were touched.

## Prior ambiguity (unchanged; lead L-009)
The byte is a single $60 (it would decode as RTS). No ROM-encoded reference, pointer,
fall-through, RTS-dispatch entry, split hi/lo table, inline argument, indirect or stack
entry reaches it; the ROM's padding convention is $FF. Rejection reviews:
analysis/handoffs/round-1-redirect.md, analysis/handoffs/round-2-redirect.md
(evidence analysis/redirect/r2-a/); coordinator assessed the review sound and reused it
in round 3. The code-versus-data question remains materially ambiguous on evidence.
This fallback does not resolve it.

## Action
- map.annotate: region id 796 {offset 9732, length 1, kind Data, provenance Manual,
  confidence 1, label none}; comment/evidence state the fallback and the absence of
  structural evidence. Script: scratch/fallback/annotate.py.
- map.validate ok; map.save analysis/state.json; map.export maps/current.json (sha fc5d69ba…).
- Checkpoint pass-004-fallback: maps/pass-004-fallback.json, verification
  analysis/pass-004-fallback-verification.json — ok, byte_identical. ROM unchanged.
- Commentary: source label Data_E5F4 (neutral, mechanical) with an explicit fallback note;
  review entry region-796; L-009 kept open in commentary/uncertainties.md.
  Handoff: analysis/handoffs/round-4-commentary.md.

## Counts
- Bytes carrying a region kind: 24592 / 24592.
- Evidence-backed classification: 24591 / 24592 (99.996%).
- User-directed Manual fallback (meaning unknown, not evidence-backed): 1 byte (0x2604).
- Evidence-backed whole-ROM classification is NOT claimed complete.
