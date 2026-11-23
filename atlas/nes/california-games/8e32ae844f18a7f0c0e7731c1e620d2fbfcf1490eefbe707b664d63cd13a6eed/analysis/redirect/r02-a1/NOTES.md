# Redirect r02-a1 (round 2) - working notes

Input checkpoint: maps/round02-final.json = maps/current.json, sha256 53e08b7b... (99.89%, 149 B unknown in 18 gaps).
Source: commentary/source/game.asm 2ed461a9... The ROM, map, state, source and binary are not modified by this attempt.

## Tools (all read-only on ROM/map)
- tools/dis.py PHYS_HEX COUNT: linear official-6502 decode (uses analysis/tools/m6502.py).
- tools/xref.py LO HI ...: absolute operands of classified Code that point into CPU ranges. It also prints immediate-pair
  noise, which is not used as evidence.
- tools/proposals.py: writes evidence/proposals.json and asserts every proposed byte is still Unknown.
- tools/natural_end.py: bounded natural-gameplay check of the compete-all ending from scratch/snap_r7.json. It makes no
  RAM writes. Results are in evidence/natural-ending-run.json.
- evidence/toolkit-decode.txt: configured-binary `romatlas decode` of every proposed Code range. All instructions are
  official; each range ends in JMP or flows into an established routine.

## Per-gap decisions (S = structural evidence; R = reachability)
| Phys | CPU | n | Decision | Key evidence |
|---|---|---|---|---|
| 00018 | b0 $8008 | 3 | keep unknown | JSR UpdateSound in the jump-table slot 0 position. The return would land in the copyright text (Data), and UpdateSound is not an inline-argument routine. Ambiguous: a vestigial entry vs non-code. |
| 00239 | b0 $8229 | 2 | Code (R2R-01) | lda #$60 feeds UploadStatusRow's A (VRAM row low byte; callers use $20/$40) |
| 007DB | b0 $87CB | 5 | Data (R2R-03) | FrameHandlerTable entries $18hi/$1A/$1C. The round-2 commentary's "index never exceeds $1C" was misread: $1C is the last entry, at $87CE. |
| 00A95 | b0 $8A85 | 3 | Code (R2R-02) | lone jmp IdleLoop |
| 00CFD | b0 $8CED | 9 | Code (R2R-01) | ClearStatusLineY Y contract; the same shape as FootBagSummaryDone |
| 01B56 | b0 $9B46 | 2 | Data (R2R-04) | ora ($02,x) before a routine that overwrites A first; the bytes fit the $9B3E table |
| 03EB3 | b0 $BEA3 | 5 | Data (R2R-03) | BmxStart OAM copy ldx #$1B from $BE8C -> ends at $BEA7 exactly |
| 0400E | b0 $BFFE | 2 | blocked | TK-04 |
| 087EF | b2 $87DF | 8 | Data (R2R-04) | $00 run after the forced branch ldx #$0F/bne; the BRK vector is reset |
| 1C010 | b7 $C000 | 1 | blocked | TK-04 |
| 1D7C9 | b7 $D7B9 | 5 | Code (R2R-01) | QueueSfx slot stride $10/$14/$18/$1C |
| 1DECF | b7 $DEBF | 3 | Code (R2R-02) | lone jmp SurfUpdate |
| 1EEC5 | b7 $EEB5 | 2 | Data (R2R-03) | $EEAF,y player offsets (Y = CurrentPlayer 0..7), a copy of PlayerNameOffsets |
| 1F082 | b7 $F072 | 8 | Data (R2R-04) | unofficial opcodes in every alignment |
| 1F15A | b7 $F14A | 6 | Data (R2R-03) | BinToBcdTable 94..99 |
| 1F160 | b7 $F150 | 10 | Code (R2R-01) | self-closed fill loop |
| 1F2CA | b7 $F2BA | 3 | Code (R2R-01) | CallBank0 BIT-chain link; needs re-split of region 578 |
| 1F893 | b7 $F883 | 45 | Data (R2R-04) | ROM-write/unmapped/unofficial content inside the $F69E-$FF68 data block |
| 1FFEF | b7 $FFDF | 24 | Data (R2R-04) | illegal $FF, ROM write, unmapped $5A61 |
| 20007 | b7 $FFF7 | 3 | Code (R2R-02) | lone jmp ResetVector |

Total proposed: 143 B (Code 38, Data 105). Kept unknown: 6 B.

## Side findings
- PlayerNames records have stride 7 (6 name bytes + team byte at +6; PlayerTeams = $03E0 = $03DA+6), not 6 as the
  commentary symbol notes say. PlayerNameOffsets = 00 07 0E 15 1C 23 2A 31.
- CurrentPlayer is capped at 7 (TeamSelectFrame cmp #$07), so there are up to 8 players.
- The S12 "event loop" is explained statically. With CurrentEvent = 6, EventIntroFrame jumps to ShowMainMenu on any new
  button press, so a random mash that includes A skips FINAL RESULTS within a few frames and then restarts compete.
  The natural-route check is in evidence/natural-ending-run.json.
