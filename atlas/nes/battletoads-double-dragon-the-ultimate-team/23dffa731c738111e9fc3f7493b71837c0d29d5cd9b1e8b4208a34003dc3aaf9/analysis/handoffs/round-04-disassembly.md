# Round 4 disassembly pass 4 handoff: INCOMPLETE (5 bytes unknown)

- Checkpoint: pass-004 (maps/pass-004.json == maps/current.json == analysis/state.json, 1,656 regions,
  map sha256 514387bd...94bd3b96; generated/pass-004; analysis/pass-004-verification.json ok=true,
  byte_identical=true).
- ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown). Rounded; not complete.
- Code 61,119 (+10), Data 201,036 (+1); 3 new Static regions (analysis/tools/round04/apply.py).

## Lead outcomes
| Lead | Outcome | Bytes |
|---|---|---|
| L-028 b6 $FE24-$FE2B | verified, Code conf 1.0 (truncated sprite-clear + OAM-DMA idiom; jsr/stx/lda) | 8 |
| L-028 b6 $FE2C | verified, generic Data conf 0.8 (truncated $8D; overlapping sta $00A2 would clobber RunRamColumnCode input $A2) | 1 |
| L-028 b0 $E536-$E537 | verified, Code conf 0.8 (lda #$02 alternate entry of TryLatchOntoPlayer; $D1 consumers accept id 2) | 2 |
| L-027 | not retried; blocked | 5 unknown |
Reachability unresolved for all L-028 bytes (no caller or reference found by byte search).

## Remaining unknown (L-027, lone $60: dead rts vs unread pad, no reference/consumer)
b0 $8D92, b4 $B979, b5 $ACFA, b6 $9745, b6 $EFE5. Rejection review: analysis/handoffs/round-03-redirect.md.
Reopen only with a new reference, consumer or materially different justified method.

## Next
- Commentary: carry pass-003 commentary to pass-004 (3 new regions: b6 $FE24 code, b6 $FE2C data,
  b0 $E536 code; the old unknown-run notes for these are obsolete).
- Redirect: only L-027 remains; prior review found no discriminating test.
- Gameplay: complete for representative milestones; scenarios.json unchanged.
- Housekeeping: server (/tmp/r04.sock daemon) shut down; ~4.1 GB free. No new toolkit issues found
  (analysis/toolkit-issues.md).
