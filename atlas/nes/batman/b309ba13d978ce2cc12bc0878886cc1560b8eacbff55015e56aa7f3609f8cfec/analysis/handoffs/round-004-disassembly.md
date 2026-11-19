# Round 4 — disassembly handoff

- Scope: ready lead L-020 (provenance part). L-018 is exhausted-unknown with no new evidence, so it was not reopened. No gameplay: S1-S8 are visited.
- Input pass-003. Output checkpoint **pass-004**: maps/pass-004.json, generated/pass-004,
  [analysis/pass-004-verification.json](../pass-004-verification.json) ok=true, byte_identical=true, map_sha256 d6e11e81…44b9.
- analysis/state.json is identical to maps/current.json (2,194 regions); map.validate ok.
- Tools: analysis/tools/r4_build.py (verify + build scratch/r4/edits.json), analysis/tools/r4_apply.py (apply, validate, save, export).
  Pre-edit state backup: scratch/r4/state-pass003.json.

## Completion
ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown).
Unrounded 99.9992%. **Status: incomplete.** Only unknown: file 0x1C5AE-0x1C5AF (PRG 14 $C59E, L-018). Per-bank counts unchanged from round 2 (round-002-disassembly.md).
Code 41,123 B: 28,697 Observed / 12,426 Static (153 B moved Static -> Observed-artificial).

## L-020 (provenance part) — verified, applied
- r3-a1 contexts recomputed independently against pass-003: 72 new opcode starts, same set as observed-merge.json; all on Static instruction boundaries
  with matching CPU address and bank slot; 0 fetches in non-code/unknown bytes (0x1C5AE not fetched).
- Splits (Observed runs tagged r3-a1:cont15/clr15f80/clr15f00/clr15f80b; Static runs keep original evidence/confidence/views; label/comment on first run):
  - 0x11717+162 (bank 8 $9707, area-16 boss intro script) -> Observed +123, Static 0x11792+39
  - 0x1DE45+26 (continue restart) -> Observed +8, Static 0x1DE4D+9, Observed 0x1DE56+9
  - 0x1DE9C+11 (skip test) -> Observed whole
  - 0x1DEFE+2 -> Observed whole
- Artificial execution only; natural reachability not claimed.

## For commentary (L-020 still open)
Apply the L-020 recipe in analysis/leads.json: SkipArea16Flag $76 bit7 = area-16 boss beaten (continue skip) and resolve its uncertainty; ObjKind33 note;
AreaCleared / GameFrameLoop_continueGame cross-links; SetupBossSlotsArea17 is not on the AreaCleared skip path (caller still unknown);
ObjKind42Defeated: kind 42 placed in slot 0 by BossSlotTemplate0; $38/$3E constant 0 (only reset clear). Refresh source from pass-004 by physical range
(the 4 split regions changed provenance/evidence only; the new run starts 0x11792, 0x1DE4D, 0x1DE56 need no new labels unless already labelled).
Evidence: analysis/redirect/r3-a1/results.txt, analysis/handoffs/round-003-redirect.md.

## Leads / toolkit
- L-020 -> partially-verified (commentary part open). L-018 unchanged. [analysis/leads.json](../leads.json).
- No toolkit issues found this pass; TK-001..TK-003 unchanged in [analysis/toolkit-issues.md](../toolkit-issues.md).
