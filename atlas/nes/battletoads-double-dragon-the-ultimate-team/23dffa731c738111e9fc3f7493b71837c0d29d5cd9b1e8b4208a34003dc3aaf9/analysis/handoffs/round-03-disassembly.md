# Round 3 disassembly pass 3 handoff: INCOMPLETE (16 bytes unknown)

- Checkpoint: pass-003 (maps/pass-003.json == maps/current.json == analysis/state.json, 1,653 regions,
  map sha256 92863db2...452733; generated/pass-003; analysis/pass-003-verification.json ok=true,
  byte_identical=true).
- ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).
- Code 61,109 (+395), Data 201,035 (+1,280); 103 new Static regions (analysis/tools/round03/apply.py).
- Commentary: source must be re-prepared from pass-003 (103 new regions, 9 table labels + 15
  MotionScript_B6_xxxx labels; earlier commentary notes on the old unknown runs are obsolete).

## Lead outcomes
| Lead | Outcome | Bytes |
|---|---|---|
| L-023 | verified (motion scripts + $EFA6 generic) | 300 Data |
| L-024 | verified (bounded consumer tables, labelled) | 312 Data |
| L-025 | verified (generic data incl. tier B singles) | 668 Data |
| L-026 | verified except b0 $E536-$E537 | 153 Code |
| L-006 | verified (bank-switch module copies) | 236 Code |
| L-004 | verified (jmp-table slots, targets not followed) | 6 Code |
| L-027 | re-examined, blocked | 16 unknown |

## Remaining unknown (all L-027)
| Range | Bytes | Reason |
|---|---|---|
| b0 $8D92 | 60 | lone byte after terminal branch pair; dead rts vs pad; no reference |
| b0 $E536-$E537 | A9 02 | lda #$02 prefix of called $E538 vs tail of PlayerFrameMapA/B ($E4EB,y Y<$5B reaches it at Y=$4B; $E4F5,y at Y=$41); frame domain unproven |
| b4 $B979 | 60 | after jmp $B987, before RamRoutineTemplate_B4; no reference |
| b5 $ACFA | 60 | after jmp $AFB9; no reference |
| b6 $9745 | 60 | index 6 of StageNextTables, never read (cpy #$07) |
| b6 $EFE5 | 60 | after always-taken bne $F00C, before MotionScript_B6_EFE6 |
| b6 $FE24-$FE2C | 20 19 FE 8E 03 20 A9 02 8D | last opcode's operand overlaps classified ldx #$00 at $FE2D; overlapping-entry trick vs leftover |

## Concrete next question (redirect)
- $E536-$E537: bound the player frame values ($03B6,y) seen by MirrorPlayerAppearance (called from
  TryLatchOntoPlayer at $E5D1). Recipe: reach an enemy latch-on-player state (bank 0 object that
  calls TryLatchOntoPlayer; RAM shortcut acceptable) and trace Read events on phys
  [0x064FB, 0x06557) (analysis/tools/round03/framemap.py takes snapshot + cycle count), or prove the
  frame domain statically. If reads never reach $E536 and the domain is bounded below $4B/$41, the
  2 bytes are code; if reads reach them, they are table data.
- Lone $60 / $FE24 bytes: no new evidence; reopen only with a new reference (rejection review in
  analysis/handoffs/round-02-redirect.md still holds).

## Gameplay
- 2P Game A replay (1,200 frames) did not run the frame-map reader; scenarios.json updated
  (status attempted). All levels, ending, game over and 2P A were visited in earlier rounds.

## Housekeeping
- Server (/tmp/r03.sock daemon) shut down. Scratch 3.2 MB; ~4.3 GB free.
- Toolkit: no new toolkit issues found (analysis/toolkit-issues.md).
