# Round 4 disassembly handoff

Status: COMPLETE (pass finished; only commentary updates remain for the changed regions).
Checkpoint: pass-004 (maps/pass-004.json, generated/pass-004, analysis/pass-004-verification.json): validate ok, byte_identical true, map sha256 70b38672ccd610e42eb4267017cc43f04873b3c1ff3290896c15caba3b5251e5.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## What changed (6 map.annotate edits; no kind changes; 1,398 -> 1,401 regions)
- Bank 5 (PRG 10/11), lead L-024: region 824 (phys $142C7, 1235 B) split into
  - 824 phys $142C7-$14397 ($82B7-$8387, 209 B): tail of the 96 x 4 terrain attribute records (continues 1871).
  - 2022 phys $14398-$14617 ($8388-$8607, 640 B): generic all-zero run; no reader found; role unresolved.
  - 2023 phys $14618-$14787 ($8608-$8777, 368 B) EventGroundHeights: 46 column records x 8 bytes; reader UpdateGroundHeight lda (Tmp14),y at F:$D2AC with pointer $8600+id*8+sub+8; extent from column-id domain 0-45; observed reads in r03 event stage 1 (artificial snapshot).
  - 2024 phys $14788-$14799 ($8778-$8789, 18 B): head of ColumnRecordPtrs (continues 1872).
- Bank 2 (PRG 4/5), lead L-023: 2021 phys $0B0F7 960 B LegalScreenTiles ($B0E7-$B4A6) and 1812 phys $0B4B7 64 B LegalScreenAttrs ($B4A7-$B4E6); the old split at $B4AA was 3 bytes late. Legal/trademark nametable image; reader unknown (not claimed unused).

## Lead outcomes
- L-024: verified, applied. L-023: verified, applied. Rejected: none. No ready leads remain.

## For commentary (round 4)
- Map changed, so commentary needs a refresh from pass-004 (4 changed + 3 new regions; all others unchanged by range).
- Name/document EventGroundHeights in UpdateGroundHeight (record layout); label LegalScreenTiles/LegalScreenAttrs; mark 5:$8388 as zero run with no known reader; update uncertainties.md: 2:$B0E7/$B4AA and 5:$8608 resolved structurally, RampAirVel index 1 observed, F:$FF2C and 6:$8701 reads are RTS dummy reads (r03 H6).

## Evidence and tools
- analysis/disasm-r04/plan.json, tools/plan.py (asserts), tools/apply.py. Saved r03 evidence: analysis/redirect/r03-redirect-a1/.
- Gameplay: none this pass. Toolkit: no new issues (analysis/toolkit-issues.md, TK-001..TK-006).
