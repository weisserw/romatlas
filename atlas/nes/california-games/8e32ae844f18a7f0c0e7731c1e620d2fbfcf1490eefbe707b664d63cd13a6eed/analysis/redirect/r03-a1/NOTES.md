# Redirect r03-a1 notes (round 3)

Input: maps/round03-final.json aab2fb6c... (= maps/current.json = analysis/state.json; state.before.sha re-checked OK
after the work), source b70bdbb2..., ROM 8e32ae84.... Only emulator calls were made (no map edits).

## Residual/unknown ledger (exact complement of the internal map)
| Physical | CPU / bank | Bytes | Neighbours | Disposition |
|---|---|---|---|---|
| 0x400E-0x400F | b0 $BFFE-$BFFF, view [0,1,14,15] | BD 49 | Code Observed 0x3FFD+17 (BmxFrameSprites tail, ends $BFFD); Data Static 0x4010 (bank 1 $8000 table) | TK-04 blocked (L-R1-01) |
| 0x1C010 | b7 $C000 fixed | C1 | Data Static 0x1BFF0+32 (bank 6 tail); Code Observed 0x1C011 ($C001 BmxFrameTail) | TK-04 blocked (same instruction) |
Total 3 B, one structure: observed `lda $C149,x`, opcode at $BFFE in bank 0, final operand byte at $C000 in bank 7.

## Work log
1. Reachability scan for round-3 no-caller code: tools/refscan.py -> evidence/refscan.json (negative).
2. S12 stall: static cause = flying disk round ends only when the disk lands (DiskFlightStep $AE >= $A8), no timer;
   mashing never completes the meter throw. tools/disk_natural.py run 1 (bug: kept pressing at CurrentEvent 6 ->
   main menu) and run 2 (fixed) -> natural FINAL RESULTS.
3. Flying disk RAM contract: static reading + watches disk/disk2/disk3 -> L-R3R-01.
4. Half pipe record layout: watch pipe -> L-R3R-02.
5. Text tables $EE7D/$EE86 -> L-R3R-03.
6. BMX/skating/surfing single watches: ranges only (watch-bmx/skate/surf.txt); not enough for naming proposals.
