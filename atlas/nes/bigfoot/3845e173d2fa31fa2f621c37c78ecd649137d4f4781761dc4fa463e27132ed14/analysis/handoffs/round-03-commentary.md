# Round 3 commentary handoff (final acceptance)

Status: COMPLETE. The current-source readability scope (pass-003, 100.00% classified) is
done and the final editorial review is recorded. `commentary.py audit` passed once, at
the end: 1,398 regions and 1,317 units reviewed, editorial_audit true, fresh rebuild
byte_identical true, reading copy refreshed.

## Baseline / carry-forward
- The map changed (pass-003: 118 edits; 1,385 -> 1,398 regions; no kind changes). The
  whole round-2 commentary is preserved untouched at analysis/round-2/commentary/. The
  round-1 copy at analysis/round-1/commentary/ was not touched.
- A fresh frozen baseline was prepared from pass-003 (commentary/before, baseline.json,
  phase1-state.json). Notes and tools were carried by bank/CPU address, i.e. by
  physical range. checknotes reports 0 misplaced keys.
- review.json carries the earlier credit for unchanged ranges: 1,085 entries from
  round 1 and 296 from round 2. The 17 new or changed entries are marked round 3.
- Name conflicts reconciled:
  - RaceSpriteLayouts became RaceSpriteLayoutPtrs, matching the map.
  - The map labels are now used as given: RaceSpriteLayoutData, MetatileTerrainMap,
    TerrainEffectA-D, TitleRevealScriptPtrs/RowTiles, SecretButtonSequences.
  - LoadScreen_Tbl became ScreenRecords.

## Scope / readability
- **G-series semantics written into source:**
  - Terrain lookup chain: RaceTrackRows -> MetatileTerrainMap -> TerrainEffect. The
    TerrainEffect value is an object-script id, so RaceObjFrame was renamed in meaning.
  - Race sprite layouts: layout 50 is never selected; $AC37 is a spare byte.
  - Title palette scripts and reveal tiles.
  - SecretButtonSequences, decoded with the effect of each sequence; SelectStartOffset.
  - Shop exit rule.
  - ScriptOp1D is dead.
  - NmiHandler2 and NmiHandler13 now use symbolic bank-0 and bank-2 operands.
  - Palette slot 29 is PaletteBuf.
  - SFX ids are 0-10.
  - The bank-5 $8388 pattern.
  - HillSpecial is reached in normal play.
- **Final cleanup across the whole source:**
  - 166 reader-named tables were renamed by role, for example SurfaceClassGrid,
    NeedleTiles, DebrisVelX and ChampionFlashPalette. Interpretations are marked
    tentative where needed.
  - Prose references to the old names and about 40 address-only references were
    replaced.
  - All 210 @skip labels and 71 of 73 plain @loop labels now have mechanical
    descriptive names, such as @if_HeatNumber_eq_03, @if_PlayerFlags_01_set and
    @loop_store_X.
  - 18 regions that still showed the generator's consumer list, and 14 data labels
    with no comment, now have format notes.
- **Stale aliases fixed and raw RAM operands named:**
  - $04D3 in race code, which showed TempMoney+6. This is also a dead load.
  - The bank-3 RowOffsetLo/Hi.
  - RaceDistLo in race scripts.
  - The bank-6 PaletteBuf+32/33, HillRebuildDelay and HillReplRowC.
  - The fuel-gauge tiles.
  - OamWriteIndex.
  - The text NMI pointers.
- **Remaining generic names:**
  - 0 @skip.
  - 2 plain @loop.
  - 5 @join labels at if/else merge points.
  - 12 condition labels without a symbolic subject.
  - 0 X_Tbl names.
  - Raw addresses remain only in the RAM-clear loops, the dead `jsr $0014`, the
    `and $EF` slip and the timing pads.

## Verification
- `commentary.py verify` returned byte_identical true after every batch.
- `commentary.py audit` returned ok, byte_identical true, reviewed_regions 1398,
  reviewed_units 1317 and editorial_audit true. Results are in commentary/audit.json
  and commentary/verification.json.
- The reading copy is commentary/reading-after.asm.

## Remaining uncertainties
Details are in commentary/uncertainties.md. The round-2 items 1-5 are marked resolved
there.
- **Data with no reader found:**
  - 2:$B0E7-$B4A9, $B059 (spare script pointers), $B4AA, $AC37.
  - 4:$A474-$A873.
  - 5:$8388-$8777 and $88F4.
  - 3:$8991 (multiply-like tables) and $894D.
  - 6:$8400 and $8701.
  - 0:$8F84, $9087-$9180 and $BE6F.
  - F:$E9AC, $FF2C and a few unread table tails.
- **Interpretations:**
  - Some round-3 table names come from consumer code only, for example
    SurfaceSlopeGrid, SpeedAnimClass, TruckMessageOffsets and ThrottleGearOffset.
  - The purpose of the hill lane-shift sequence.
  - Only RampAirVel/RampBumpSpeed entry 1 is read.
  - Partly decoded record fields: Metatiles, CpuPathRecords, ObjectAnimTables and
    MapTruckLayout.
  - The music byte format per song.

## Structural questions for redirect
None blocking. No map change is proposed. Optional bounded read watches on
3:$8991-$89B4, F:$FF2C-$FF2F and 0:$AA46-$AA50.

## Artifacts
- **Source and review files:** commentary/source/{game.asm,ram.inc,hardware.inc},
  commentary/review.json, commentary/audit.json, commentary/verification.json,
  commentary/reading-after.asm.
- **Records:** commentary/progress.md and commentary/uncertainties.md.
- **Notes:** commentary/notes/, with the new files r3_tables_F.txt, r3_tables_06.txt,
  r3_regions.txt and r3_ram.txt.
- **Tools:** commentary/tools/annotate.py, which has the new label heuristics, plus
  review.py, checknotes.py, tblinfo.py, skipinfo.py and r3_renames.json.
- **Archives:** analysis/round-2/commentary/ and analysis/round-1/commentary/.
- **Toolkit:** no new issues. TK-001 to TK-006 are unchanged and the toolkit was not
  modified.
