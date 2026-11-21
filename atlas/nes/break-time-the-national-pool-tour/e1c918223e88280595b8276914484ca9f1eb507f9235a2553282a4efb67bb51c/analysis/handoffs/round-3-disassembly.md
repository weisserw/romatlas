# Round 3 disassembly handoff

## Checkpoint
- Input: pass-002 (state restored via map.load; baseline not re-created; prior classifications preserved).
- Output: **pass-003** — maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json:
  ok=true, byte_identical=true, map sha256 e4d17a5803fbf13fd0a3d931c2cfe936784e571914921c0ebec229276930e0ff.
  map.validate ok; analysis/state.json saved; maps/current.json exported (identical to pass-003 map).
  input/game.nes unchanged.

ROM classification: 99.99% (163845 / 163856 bytes classified; 11 bytes unknown).

Status: **incomplete** (11 bytes unknown). Pass ended at easy-gain exhaustion.

## Per-bank counts (code / data+image / text / padding / unknown)
| Bank | Code | Data/Image | Text | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 |
| prg00 | 6205 | 1692 | 294 | 0 | 1 |
| prg01 | 0 | 6476 | 0 | 1716 | 0 |
| prg02 | 9 | 8183 | 0 | 0 | 0 |
| prg03 | 53 | 6788 | 0 | 1351 | 0 |
| prg04 | 9 | 8183 | 0 | 0 | 0 |
| prg05 | 53 | 7368 | 0 | 771 | 0 |
| prg06 | 6931 | 1110 | 148 | 0 | 3 |
| prg07 | 3315 | 4106 | 0 | 771 | 0 |
| prg08 | 1937 | 1625 | 241 | 4389 | 0 |
| prg09 | 0 | 8 | 0 | 8184 | 0 |
| prg10 | 5593 | 1628 | 966 | 0 | 5 |
| prg11 | 6103 | 197 | 514 | 1378 | 0 |
| prg12 | 1004 | 7188 | 0 | 0 | 0 |
| prg13 | 0 | 6476 | 0 | 1716 | 0 |
| prg14 | 4093 | 111 | 38 | 3948 | 2 |
| prg15 | 0 | 16 | 0 | 8176 | 0 |
| chr | 0 | 32768 | 0 | 0 | 0 |

Code 35,305 (Observed 19,530; Static 15,775). Regions: 724 Code, 437 Data, 140 Text, 8 Padding, 1 Image.

## Leads
- L-020: **accepted** 12 / rejected 0. Four "JMP ENDIF" fragments (phys 4964, 84584, 84607, 117621)
  annotated Code, Static, confidence 0.8 (region ids 1328-1331), with views in the neighbouring
  regions' bank context. Reachability is not claimed. Commentary should add labels and notes for them.
- L-019: **verified** (0 net bytes). Region 1254 split into 88856+32 (X/Y pairs), 88888+624
  (RackLayouts, id 1332) and 89512+5 (string framing, id 1333). All three remain Data.
  Commentary already documents this split.
- L-021: not touched; this is commentary-only work.
- L-017: still **blocked**, 11 bytes, not retried. The r2-a1 rejection review still holds. See
  analysis/investigation.md "Round 3".

## Remaining unknown ranges (11 bytes)
- phys 6173..6174 (bank 0 $980D, RTS)
- phys 51444..51447 (bank 6 $88E4, LDA #imm/RTS)
- phys 85771..85776 (bank 10 $8EFB, LDA #imm/STA/RTS)
- phys 117501..117502 (bank 14 $CAED, RTS)
- phys 118162..118163 (bank 14 $CD82, RTS)

## Gameplay
Not exercised this pass. SC-014..SC-016 already cover the tour, ending, passwords and trick shots,
and no unknown byte depends on an unvisited state.

## Toolkit issues
No new toolkit issues found. TK-001..TK-004 are unchanged: three missing features in tracing and
data-read provenance and one in source readability. None are blocking. See analysis/toolkit-issues.md.

## Next
Commentary: label and note the four new Code regions and apply L-021. Redirect: the 11 bytes stay
unknown unless new reference evidence appears. Their terminal rejection review is in
analysis/handoffs/round-2-redirect.md §2.
