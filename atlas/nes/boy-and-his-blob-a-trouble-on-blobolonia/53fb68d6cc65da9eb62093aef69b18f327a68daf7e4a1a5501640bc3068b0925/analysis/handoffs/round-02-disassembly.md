# Round 2 — disassembly handoff (COMPLETE, result: incomplete ROM classification, 1 byte unknown)

- Scope: apply/verify ready redirect leads L-010..L-021 (incl. L-015a-g, L-016a-c), L-007 spot check,
  residual $9AB8 review. Resumed from analysis/state.json (= pass-002). No baseline/reinit, no toolkit edits,
  no gameplay runs (every remaining unknown was statically decidable; analysis/scenarios.json unchanged).
- Output checkpoint: **pass-003** (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json).
- Verification: ok true, byte_identical true, map_sha256 b5c795028ae6582fbdb64bcbbd7ff876c216d8ea114a5beb02e4fb5fc3b7d01a.
  analysis/state.json == maps/current.json == maps/pass-003.json; map.validate ok; 1364 regions.
- Tools/evidence: analysis/pass-003/{verify_leads.py, verify.log (28/28 PASS), plan.json, apply.py,
  l007_spotcheck.py, l007.log, stats.py}.

ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).

(Rounded 100.00% is not completion: 1 byte remains unknown.)

| Space | Code | Data | Text | Padding | Unknown |
|---|---|---|---|---|---|
| iNES header | 0 | 16 | 0 | 0 | 0 |
| PRG 16K bank 0 | 11732 | 4369 | 111 | 171 | 1 |
| PRG 16K bank 1 | 1425 | 12855 | 0 | 2104 | 0 |
| PRG 16K bank 2 | 1578 | 14752 | 0 | 54 | 0 |
| PRG 16K bank 3 | 1578 | 14752 | 0 | 54 | 0 |
| PRG 16K bank 4 | 1578 | 11231 | 0 | 3575 | 0 |
| PRG 16K bank 5 | 192 | 12637 | 0 | 3555 | 0 |
| PRG 16K bank 6 | 5118 | 6201 | 0 | 5065 | 0 |
| PRG 16K bank 7 | 1970 | 9322 | 5 | 5087 | 0 |
| CHR ROM | 0 | 131072 | 0 | 0 | 0 |
| **Total** | **25171** | **217207** | **116** | **19665** | **1** |

Changes vs pass-002: +3993 classified (3868 Data, 116 Text, 9 Code), plus two Data->Code corrections (8+8 B). Code 25146 -> 25171 (+9 L-014 vector heads, +8 $C088, +8 $85E0); Data 213355 -> 217207
(+3868 new, -16 reclassified); Text 0 -> 116; Padding 19665 unchanged; Unknown 3994 -> 1.

## Leads
| Lead | Outcome |
|---|---|
| L-010 | consumed: b7 $C3E2-$C421, $C422-$C521 (RoomFloorPtrsHigh), $C522 (RoomFloorLists). Refinement: words may point to shared list tails ($C5B4, $C69B) |
| L-011 | consumed: 19 DPCM gaps (642 B) |
| L-012 | consumed: 26 PCM gaps (872 B) |
| L-013 | consumed: SparkleSpriteTable $856A-$856E |
| L-014 | consumed: Code b1 $8000-$8002, $8009-$800B, b6 $8000-$8002; **b7 $C088-$C08F Data->Code (RestartViaBank0)** |
| L-015a-g | consumed (1126 B); refinement: alt metasprite word $A296 -> $BC19 is a shared record tail |
| L-016a-c | consumed (943 B); L-016c is 31 gaps (ledger said 36 ranges; bytes 54 match) |
| L-017, L-019, L-020 | consumed (44, 4 GridSlotX, 16 B) |
| L-018 | consumed: $BA3E-$BA41 table; **$BA42-$BA43 accepted as generic data, pattern-only (conf 0.7)** |
| L-021 | consumed: "BOYH",$FF as Text |
| L-007 | consumed: bounded spot check of 95 Data blocks adjacent to code; zero code control-flow targets into Data; produced L-022 |
| L-022 (new) | consumed: **b1 $85E0-$85E7 Data->Code AddAToZpWord** (CLC/ADC $00,X/BCC $85E7/INC $01,X/RTS; unreferenced; conf 0.75); region $85E8-$861D kept Data (ChannelOffMask/OnMask/NotePeriodTable start) |
| L-023 (new) | ready (terminal review only): b0 $9AB8 residual |
Rejected: none of the ready leads. L-007 candidates $8D22 (ASCII text) and b6 $883D (VitaminSpeed table) rejected as code.
L-006, L-008, L-009 untouched (no unknown bytes; verification/reachability only).

## Remaining unknown
- physical 0x1AC8, bank 0 CPU $9AB8 (prg_banks [0,1,14,15]), 1 byte $60 between RTS $9AB7 and LandOnFloors $9AB9.
  Checked: no JSR/JMP/branch target, no $9AB8/$9AB7 word (RTS-trick), no LDA #$9A/PHA pair. Code (dead RTS) vs
  data/padding is undecidable from available evidence; retained unknown (L-023).

## Notes for commentary
- New code labels: RestartViaBank0 (b7 $C088), AddAToZpWord (b1 $85E0); vector heads b1 $8000/$8009, b6 $8000 now code.
- New data labels: RoomFloorPtrsHigh, RoomFloorLists, SparkleSpriteTable, GridSlotX (map labels); Text regions
  PharmacyLabelText gap ($9025-$9093) and $CBB7 "BOYH".
- Former Unknown sections in commentary/source must be rewritten as Data/Code; carry round-1 redirect corrections
  (records start $A2E8; NotePeriodTable to $8671; GridSlotX operand; L-005 text cites L-016 not L-018).
- Commentary's "8 unidentified bytes" at $85E0 are now the AddAToZpWord routine.
- New annotated gap regions are separate from neighbouring table regions (not merged); merging is optional.

## Toolkit findings
No new toolkit issues found in this pass; TK-001..TK-003 stand (analysis/toolkit-issues.md). map.save and
map.export write the same internal-format file (as documented for NES); used both.
