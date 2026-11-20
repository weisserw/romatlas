# Round 5 / batch final acceptance with USER-DIRECTED MANUAL FALLBACK (2026-11-20)

## Authorization
The batch runner rejected the round-4 terminal incomplete handoff (5 unknown bytes) without a
final audit. The user explicitly authorized a batch-only exception: when at most 10 physical ROM
bytes remain unclassified, mark exactly those bytes as generic Data with UNKNOWN MEANING using
Manual provenance. This is a user-directed fallback, not an observed or statically proven data
role. It overrides workspace rules requiring further structural proof for these bytes only.

## Verified complement (before annotation)
Live store (analysis/state.json == maps/pass-004.json, 1,656 regions, no overlap) left exactly
5 unknown bytes, identical to the user's list (analysis/tools/round05/complement.py):

| Physical offset | Length | Bank:CPU | Prior ambiguity (L-027) |
|---|---|---|---|
| 3490 (0x00DA2) | 1 | b0 $8D92 | after an always-leaving beq/bne pair; dead rts vs pad; no reference |
| 145801 (0x23989) | 1 | b4 $B979 | after jmp $B987, before RamRoutineTemplate_B4; no reference |
| 175370 (0x2AD0A) | 1 | b5 $ACFA | after jmp $AFB9; no reference |
| 202581 (0x31755) | 1 | b6 $9745 | slot 6 of StageNextTables_B6, never read (loader uses Y >= 7) |
| 225269 (0x36FF5) | 1 | b6 $EFE5 | after always-taken bne, before MotionScript_B6_EFE6; no reference |

Rejection reviews (preserved, unchanged): analysis/handoffs/round-03-redirect.md,
analysis/handoffs/round-04-redirect.md, analysis/redirect/r04-a1/. No redirect research was
reopened for these bytes, per the user's instruction.

## Action
- map.annotate (analysis/tools/round05/apply_fallback.py): 5 one-byte regions, kind Data,
  provenance Manual, confidence 0.5, evidence text stating the user-directed fallback, unknown
  meaning and the prior ambiguity. Region ids 1657-1661. ROM bytes unchanged.
- map.validate ok; map.save analysis/state.json; map.export maps/current.json; checkpoint
  pass-005 (maps/pass-005.json, generated/pass-005, analysis/pass-005-verification.json:
  ok=true, byte_identical=true, map sha256 a4669c66...7e23f894c).

## Classification report (kept separate)
- Evidence-backed classification (Static/Observed, rounds 1-4): 262,155 / 262,160 bytes
  (99.998%); unchanged.
- User-directed Manual fallback (generic Data, meaning unknown, NOT evidence-backed): 5 bytes.
- Map coverage after fallback: 262,160 / 262,160 (100%); unknown 0. Code 61,119, Data 201,041.
- Whole-ROM evidence-backed completion is NOT claimed for the 5 fallback bytes.

## Commentary and final audit
- Round-4 commentary preserved at analysis/round-4/commentary/ (diff-identical copy); fresh
  commentary/ prepared from pass-005 and round-4 notes/tools/uncertainties carried by
  physical range (pass 5 only re-kinds the five former unknown bytes).
- Notes for the five bytes rewritten as manual-fallback data; local tools fixed (annot.py:
  no duplicate note when the generator emits a region comment; header note; fill_review.py:
  unit per unlabelled fragment; tools/r5_fallback_review.py: untruncated fallback summaries).
- Strict editorial audit recorded in commentary/review.json (editorial_audit reviewed).
- `commentary.py audit`: ok=true, byte_identical=true, 1661 regions, 1761 units,
  editorial_audit=true (commentary/audit.json); game.asm sha256 818af6656e375c6e...;
  reading copy commentary/reading-after.asm refreshed.

## Other blockers
None outstanding. Toolkit issues unchanged (analysis/toolkit-issues.md, TK-001..TK-003).
Archive not run; archive-options.json prepared.
