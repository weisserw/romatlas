# Redirect attempt r1-a1 (round 1)

- Phase: redirect, round 1. Input checkpoint: pass-001 (maps/pass-001.json ==
  analysis/state.json, map sha256 21a4a6bf...1a946; 89,140 classified, 41,948 unknown).
- Commentary handoff: analysis/handoffs/round-1-commentary.md (COMPLETE, byte-identical).
- No map/state/source changes were made. Runtime experiments used a scratch COPY of
  the store (scratch/store-copy.json) with the configured binary; map.save never called.

## Method
Static, bank-aware consumer and extent analysis over the classified code:
- tools/refs.py  - all abs/abs,x/abs,y/ind operands of Code regions into a CPU range
  (bank filter: same switchable bank or fixed bank).
- tools/imm.py   - immediate lo/hi pointer pairs within +-6 instructions.
- tools/disasm.py - linear bank-aware disassembly of a range (renamed from dis.py:
  a file named dis.py in sys.path shadows Python's stdlib module).
- tools/soundwalk.py - walks the bank-4 sound data from the consumer tables using the
  exact driver grammar (Ch1_ReadByte) and reports coverage.
- tools/ledger.py - exact unknown complement from analysis/state.json plus a
  disposition/proposal for every range -> ledger.json.
- tools/emu.py + scratch/attract.py - bounded runtime (idle title, attract mode).

## Key findings (details in analysis/leads.json, ledger.json)
1. L1-05 resolved statically: `lda #$05 / jsr SwitchPrgBank` immediately precedes both
   `jmp ($01EB)` ($F117) and `jmp ($0063)` ($F194); targets $A533..$A5CB in bank 5 are 8
   drawers ending `jmp $F197` / `jmp $F11A`, which are the two 7-byte return stubs.
   $A51B-$A532 is their 2-bit pattern table.
2. Bank 5 $A9E5-$BFE4 = bonus-stage column maps of towers 4-7 (BonusMapBank table $A905
   = 06 06 06 05 05 05 05). 4 x $580 ends at the $FF fill $BFE5-$BFFF. Rejects the
   round-1 "tower map rows" hypothesis for this range.
3. Bank 6 $9536-$B98B fully chained by consumers: palettes -> shared colours ->
   $969A (121) -> $9713 (4x121) -> $98F7 CHR (16 pages) -> map ptr tables -> 3 maps
   ending exactly at code $B98C.
4. Bank 3 $BD38-$BDF7 = 8 tower palettes (UploadTowerPalette, bank 3, ptr tables $CC22/$CC2A).
5. Bank 4 $909D/$9864 chain blocks are entry 11 of overlapping RTS tables (lo[11] = hi[0]);
   BottomRows default is 11 ($EF9A). New lead L1-09.
6. Bank 4 $A019 = `jmp $A0EC` (sound API slot); $A0EC routine is its target.
7. Sound data walker covers 2378/2382 bytes of $AD38-$B685.
8. Title data: attribute block $9D60 is 56 bytes (7x8) and overlaps the first 8 bytes of
   the $9D90 tile block (commentary said $9D60-$9D8F).
9. Bank 3 $A500-$AEFF: 40 x 64-byte 16x16 mask/image bitmap pairs; no consumer found
   (generic data by pattern).

## Residuals (2 bytes)
- bank 3 $B6B0 (phys 63168): `$60` after `jmp $FD17`, before strings; no branch/abs ref.
- bank 4 $B685 (phys 79509): `$EA` after the last pattern `$FF`, before CopyRamTileRoutine.
Both code-vs-data ambiguous; 1 byte each; no credible discriminating test.

## Runtime
See "Runtime results" below (appended when the idle-title run finished).

## Runtime results (natural, no patches)
Recipe: `python3 analysis/redirect/r1-a1/tools/attract_run.py` from the workspace root
(spawns the configured binary on a scratch copy of the store; emulator.start, 2600
untraced frames, then 1200 traced frames, no input). Log: attract-run.log; frame:
attract-demo-tower4.png; executed unknown-range offsets: attract-unknown-execution.json.
- AttractMode ($3F) became $FF between frames 2650 and 2700; the demo plays tower
  index 3 (TOWER 4) with AttractInput ($BC70.., previously Static) executing.
- Observed instruction fetches inside pass-001 unknown ranges (natural play):
  bank 4 $909D-$9134 (52) and $9864-$98FB (52) [L1-09 chain blocks];
  bank 5 $A533-$A5DF (86, physical bank-5 offsets) [L1-05 drawers];
  fixed $F11A-$F11E and $F197-$F19B (3 each) [L1-05 stubs].
- 62 Static code regions of pass-001 gained observed execution in this run.
- Speed: traced stepping ~50 frames/min (TK-004); untraced 2600 frames in 67 s.
