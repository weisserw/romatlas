# Commentary uncertainties (round 7, checkpoint pass-007)

Earlier copies: analysis/round-6/commentary/uncertainties.md (and round-1..5 under
analysis/round-N/commentary/).  Items still open are carried here.

## Round 7
- No new uncertainties.  Region 742 phase-1 evidence now says SongPatternBases, so
  the round-6 name mismatch between map evidence and commentary is resolved.  All
  open items below are unchanged.

## Resolved in round 6 (by redirect round 5 / pass-006; source updated)
- 0:$90B2-$9180 is StoryTileRecords: 27 tile records (size byte rows * 16 + width,
  then the tile ids) pointed to by the StoryTextBlocks frame records and copied by
  NmiHandler10_TextQueue; one or two unread separator bytes after each record.
  Records $9149 / $9154 were not seen read in the 640-frame intro watch; their extent
  is static.  The old "map/story data, no reader" notes are gone.
- Bank 2 sound data: ChannelOrderListPtrs $83D0-$854F (four 48-word per-channel
  tables; SongChannelOffsets holds their absolute bases), InstrumentTable
  $8610-$8707 (31 x 8 bytes, field layout in source), SoundOrderLists $8708-$8C14,
  then pattern-offset tables and patterns ($8C15-$9C85, SoundPatternArea).
  Corrected source errors: "SoundDataLow, no direct consumer" ($83D4), the
  "song header" reading of $85B0 (now SongPatternBases: pattern base addresses),
  "offset inside a song header" for SongChannelOffsets, and the 1,329-byte
  InstrumentNoiseBytes extent (now a field label inside InstrumentTable).
  NotePeriods is taken to end at $83CF (pitches 1-12 used; pitch 0 is a rest).
- 2:$B63D is a 32-byte palette only (renamed ChampionPalette2); $B65D-$BFFF is
  Bank2FillFF, and F:$FF57-$FFF9 is FixedBankFillFF ($FF fill, no reader).

## Resolved in round 5 (by redirect round 4 / pass-005; source updated)
- 3:$88B5-$89B4 is MulTable16x16 (byte a * 16 + b = a * b), read by
  ExpandMetatileTiles / Attrs with half * 16 | MetaCount.  This replaces the old
  "metatile tile index" reading and the 3:$894D / $8991 "multiply-like, consumer
  not identified" notes.  Only small products were seen in traces.
- 0:$9072-$90B1 MapTruckPath is 8 stops x 8 sprite tile ids (the truck's stop
  pose), not "four tiles per frame"; the unresolved map/story data now starts at
  $90B2.  AnimateMapTruck header corrected.
- 0:$BE30-$BE77 SparkVelY / SparkStartX / SparkStartY are whole 24-byte tables;
  the old "nine bytes after the spark tables" ($BE6F) are the end of SparkStartY.
  Reads of that tail were not observed; the extent rests on the ldy #$17 loop.
- The bytes after LaneNametableHi and TileReplaceLaneHi are read: TruckEndBitMasks
  (F:$C723, SetTruckEndBit) and TileReplaceRowStarts (F:$D450, 0/6/12/18).  The
  round-4 "no direct reader" notes were wrong.
- 0:$BDE8 / $BE00 (inside the debris tables) are now labelled PieceTableUnreadA/B:
  same 24-byte layout, no reader found.

## Resolved in round 4 (by redirect round 3 / pass-004; source updated)
- 5:$8608-$8777 is EventGroundHeights: 46 column records x 8 bytes, read by
  UpdateGroundHeight (F:$D2AC) as $8608 + column id * 8 + tile-column group.  The
  old note ("two identical 192-byte blocks") was wrong about the boundaries.
- 2:$B0E7-$B4E6 is a raw 1 KiB legal/trademark nametable: LegalScreenTiles (960)
  and LegalScreenAttrs (64).  The old split at $B4AA was 3 bytes late.  The
  structure is resolved; the reader is still unknown (it may be unused).
- RampAirVel / RampBumpSpeed: read watches in three race runs saw only entry 1.
- F:$FF2C and 6:$8701: the reads seen in traces are 6502 dummy reads after the
  preceding `rts`, not table accesses.  Both stay unread padding-like bytes.
- Round-4 editorial finding: $0493-$04D2 in event / hill code is LaneTerrainBuf
  (tile ids per lane), previously shown as PaletteBuf+N; the 24 bytes after
  LaneTerrainBufOffset are MetatileRowOfTileRow (read as offset +2), not unread.
  Race $04AF,x is RespawnSearchCount (tentative name).
- UpdateGroundHeight compares RowWithinBlock with A = 0, so its impassable branch
  is never taken there (the hill copy compares Tmp20).  Probably an author slip;
  documented in source.

## Resolved in round 3 (by redirect round 2 / pass-003; source updated)
- MatchButtonSequences / $FE12: now SecretButtonSequences (four sequences, decoded
  in source).  Sequences 0-2 preselect stop 0 / 3 / 5 and open SelectStartOffset
  (SELECT adds up to two stops); sequence 3 gives free parts in the shop once.
- ScriptOp1D `jsr $0014`: dead.  No object script uses opcode $1D (static parse of
  all 71 scripts) and $0014 is never written in a traced race.
- NmiHandler2 `jsr $8592`: bank 0 NmiScreen2Split (PRG-bank trace on the map
  screen); the operand is now symbolic.
- DrawObject / DrawObjectVScroll `$88F8,y`: bank 5 ObjectAnimTables is mapped in
  hill events too (trace); the X tags stay.
- NmiHandler13 `$B0BF / $B0D3`: bank 2 TitleRevealRowTiles (trace, bank 2 mapped);
  operands now symbolic.
- HillSpecial 1-12 / HillNmiShift*: reached in ordinary hill-climb play.  The
  mechanics are exact; the purpose (lane taller than the nametable) remains an
  interpretation, worded as such.
- PalettePtrTable slots: 29 = PaletteBuf (RAM palette, selected by code), 21 ($0000)
  never selected.
- SfxPtrTable: valid ids 0-10 (record 10 all zero); the 48-entry table extent is an
  arithmetic progression running into code.
- 0:$A000-$A844 = MetatileTerrainMap continuation; 2:$A000-$AC36 = race sprite
  layouts; 2:$B082-$B0E6 = palette scripts, TitleRevealScriptPtrs and
  TitleRevealRowTiles.
- Shop exit rule: START is refused while any part slot is negative (broken) and
  while the shop sequence is partly entered (documented in ShopMain).

## Interpretation uncertainties (worded tentatively in source)
- Event-engine field meanings (carried): TruckMeter, TruckGear/TruckGearAlt,
  TruckSurface classes, TruckFlags low bits, garage parts A-D are inferred from code.
- EventLayout values: 0 two-lane, 1 single-lane, 2 vertical (bank 6) (carried).
- HillDrawBars purpose (carried).
- Round-3 table names given from consumer code only (no runtime check): e.g.
  SurfaceSlopeGrid, SpeedAnimClass, TruckAnimSelect, DriverAnimStart,
  TruckMessageOffsets (base not proved), ThrottleGearOffset (TruckDriverAnimReq is
  probably the selected gear), HillRebuildDelay ($065F in bank 6).
- RampAirVel / RampBumpSpeed: the reader always uses index 1 (Y = A = 1 after a dead
  `ldy`, confirmed by read watches); whether the other entries were ever meant to be
  used is unknown.
- EventGroundHeights values ($85 / $B5) are read as ground lines (screen Y) because
  they are averaged into TruckGroundY; the meaning of the group index (four tile
  columns per byte) follows from WorldToTileIndex arithmetic.
- MapTruckPath is the map symbol, but the bytes are tile ids (stop pose); the
  pose interpretation is tentative.
- Field layouts inside Metatiles (bank 3, 18-byte records), CpuPathRecords and bank-5
  ObjectAnimTables are only partly understood; MapTruckLayout field layout not decoded.
- Bank 2 sound: the order-list / pattern / instrument formats are documented, but
  the songs are not split out or identified per request; instrument bytes +6 / +7
  are always zero and unread.
- Bank 6 ColumnTmpExt ($0687-$069E) and the other bank-scoped RAM aliases (race,
  bank 3, bank 6, results) are mode-specific names for shared RAM.

## Static code with no caller (described as such in source)
- 0:$8055 WarmStartMenu (entered by WarmRestart's jmp; not observed).
- 0:$8395, F:$C697, F:$D827, 3:$B69B, 0:$8998: unreached arms inside routines.
- 0:$B1E2 `pla/tax` residue and 6:$A61E lone `rts`: probably padding / dead residue.
- 0:$8588 SelectBgPatternTable0 (zero-page `and $EF`, probably an author slip).
- Copies: PlayBeepCopyB0/B2, UpdateTruckWarningCopy, HillUpdateTerrainChange,
  AddToNumberAField2 / SubFromNumberField0 siblings.
- Debug helpers DebugHexToText / DebugHexToPpu / BinaryDigitStub, CpuTargetMarkerUnused,
  ShowChampionBonusUnused, PrintEightFieldsUnused.

## Data whose role remains unresolved (generic, explained in source)
- 0:$8F84, $BDE8 / $BE00 PieceTableUnreadA/B; StoryTileRecords $9149 / $9154
  (structure known, not observed read).
- 2:$AC37 (spare byte), $B059 SpareScriptPtrs (eight unread script pointers),
  LegalScreenTiles/Attrs (format known, reader unknown).
- 3:$AF42 colour groups.
- 4:$A474-$A873 sample-like nibbles.
- 5:$8388-$8607 (640 zero bytes, unread in traced events), $88F4.
- 6:$8400, $8701.
- F:$E9AC (two zero bytes), $FF2C (four bytes).

## Structural questions for redirect
None blocking; no map change proposed.  The round-3 optional read watches were run
by redirect round 3 (see above).  Remaining optional checks: a reader for
LegalScreenTiles (e.g. a power-on or attract path not yet traced) and a hill-stage
read watch on LaneTerrainBuf to confirm the shared-RAM interpretation there.
