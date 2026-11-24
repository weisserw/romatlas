# Round 1 redirect handoff

Status: COMPLETE. Attempt directory: analysis/redirect/r1-a1/ (notes.md, ledger.json,
tools/, leads-before.json). No map, state, source or ROM changes.

## Scope / checkpoint
- Input: pass-001 (maps/pass-001.json == analysis/state.json, map sha256
  21a4a6bf92e38b70721466baef5bb1678a40d11854f5996071bc531931e1a946); 68.00%,
  89,140 classified / 41,948 unknown (67 ranges). Commentary handoff gate met
  (analysis/handoffs/round-1-commentary.md, byte-identical).
- Method: bank-aware static consumer/extent analysis over the classified code
  (tools/refs.py, imm.py, disasm.py), a driver-grammar walker for the sound data
  (tools/soundwalk.py), the exact unknown-complement ledger (tools/ledger.py ->
  ledger.json), and one bounded runtime run (idle title -> attract mode) on a scratch
  copy of the store.

## Result
Proposals now cover 41,946 of 41,948 unknown bytes (ledger.json lists every range,
its bank/CPU context, proposed kind, role and lead). Residual: 2 bytes.

| Lead | Kind | Bytes | Key evidence |
|---|---|---|---|
| L1-01 | Data | 11,378 | bank 3: player frames (unrolled 144 x 43 columns), font (9 pages), $FF fill, 40 x 64 B bitmap pairs, lift tiles, NEW tower palettes $BD38-$BDF7 (ptr tables $CC22/$CC2A, bank 3) |
| L1-02 | Data | 13,720 | bank 5: title graphics $8063-$9FDF (TitleScreen loop counts tile the range); NEW bonus maps towers 4-7 $A9E5-$BFE4 (BonusMapBank $A905 = 5 for towers 4-7, 4 x $580) + $FF fill |
| L1-03 | Data | 11,530 | bank 6: four gaps fully chained by consumers ($969A+121=$9713, +4x121=$98F7, 16 CHR pages, 3 maps x $580 ending at code $B98C), path/list tables with terminators |
| L1-04 | Data | 2,523 | NotePeriods; sound data walker covers 2378/2382 bytes of $AD38-$B685 |
| L1-05 | Code 189 / Data 24 | 213 | bank question resolved: `lda #$05/jsr SwitchPrgBank` precedes both indirect jumps; 8 bank-5 drawers end `jmp $F197`/`jmp $F11A` (the two 7-byte stubs) |
| L1-06 | Code 93 / Data 32 | 125 | $A019 = `jmp $A0EC` sound-API slot + target; RAM routine image $B692 (exact 38-byte copy to $06D5); palettes $B6B8; $FEBD data; orphan $E4B3/$E585 (lower confidence) |
| L1-07 | (in L1-08) | - | AttractMode writer $FC0E; attract mode reached naturally after ~2700 idle frames; AttractInput executed |
| L1-08 | Data | 2,149 | 46 small consumer-bounded tables (commentary-identified, re-checked); $C6F9-$C6FF $FF fill |
| L1-09 (new) | Code | 308 | $909D/$9864 chain blocks = entry 11 of overlapping RTS tables (lo[11]=hi[0]); BottomRows default 11 ($EF9A) |

Expected impact if accepted: ~99.998% classification (41,946 bytes), leaving 2 bytes.

## Corrections to earlier records
- L1-02 "tower map rows" hypothesis for bank 5 $A9E5-$BFFF rejected (bonus column maps).
- L1-05 bank was not unknown: explicit `lda #$05` before the jumps.
- L1-09: round-1 "never referenced" note missed the overlapping RTS-table entry.
- Title attribute block is 56 bytes ($9D60-$9D97) overlapping the $9D90 tile block.
- $FEBD is data (DmcRatePerTower), as commentary noted.

## Residuals / deferred
- bank 3 $B6B0 (phys 63168, `$60` after `jmp $FD17`) and bank 4 $B685 (phys 79509,
  `$EA` before CopyRamTileRoutine): no branch/abs/pointer reference; 1 byte each; no
  discriminating test available (an unreferenced rts/nop vs pad byte). Keep unknown.
- Semantic meaning (not classification) of bank 3 $A500-$AEFF bitmaps and unreferenced
  sound pattern $B67A: no consumer found; generic data suffices.
- Orphan code $E4B3/$E585: structural evidence only (twin idiom, internal jmp, shared
  RAM); disassembly may keep them unknown if judged insufficient (38 bytes).
- Gameplay: attract/demo mode visited naturally (S12; no patches): AttractMode set at
  idle frame ~2650-2700, demo plays tower 4 and executes the L1-05 bank-5 drawers +
  stubs and the L1-09 chain blocks (Observed fetches, attract-unknown-execution.json). PLAYER 1/2 WINS and score-based paths not attempted: no
  unknown range depends on them (all residual unknown is 2 bytes) - diminished prospect.

## Recommendation
Another round: run disassembly on the ready batch L1-01..L1-09 (apply ledger.json
proposals, verify byte_identical, checkpoint), then commentary for the newly
classified regions, then a short redirect for the 2-byte residual/terminal review.
Not terminal.
