# Commentary progress

## Round 7 (complete, final acceptance, 2026-11-20)
Baseline: pass-007 changed only the evidence text of region 742 (bank 2 $85C6-$860F,
"SongHeaders" -> "SongPatternBases"; extent/kind/label unchanged).  The whole round-6
commentary was moved, untouched, to analysis/round-6/commentary/ (round-1..5 copies
kept); `commentary.py prepare` built a fresh frozen baseline from pass-007 (before/game.asm
identical to round 6).  notes/, tools/, progress.md and uncertainties.md carried;
tools/review.py now diffs against round 6.  Rebuilt source (game.asm, ram.inc,
hardware.inc) and tools/lines.json are byte-identical to round 6; checknotes 0
problems; 0 generic labels.  review.json: 1,399 entries keep earlier credit; region
742 re-reviewed as round 7 (source comment agrees with the new evidence).  No
names/comments changed.  Verification and audit: see
analysis/handoffs/round-07-commentary.md.

## Round 6 (complete, final acceptance, 2026-11-20)
Baseline: pass-006 changed the map (L-029..L-031 merges/splits/relabels, 1,401 ->
1,400 regions, no kind changes, 100.00% classified).  The whole round-5 commentary was
moved, untouched, to analysis/round-5/commentary/ (round-1..4 copies kept);
`commentary.py prepare` built a fresh frozen baseline from pass-006.  notes/, tools/,
progress.md and uncertainties.md were carried and re-applied by bank/CPU address;
checknotes first reported 5 stale comment keys (0:$90ED/$9138/$9154, 2:$8B45/$8B74 at
removed splits), now 0.  tools/review.py diffs against the round-5 source: 1,350
entries keep earlier credit, 50 entries whose source text changed were re-reviewed as
round 6.

Changes (notes r2_b012, b2, b0_3, r2_headers, r3_tables_06, bF_3, new r6_sound):
- 0:$90B2 StoryTileRecords (27 records, size byte + tiles, separators) replaces the
  four "map/story data, no reader" notes; StoryTextBlocks / StepTextBlock now point
  to it.
- Bank 2 sound: ChannelOrderListPtrs ($83D0, replaces SoundDataLow), InstrumentTable
  field layout (31 x 8), SoundOrderLists command format, SoundPatternArea
  ($8C15-$9C85: offset tables then patterns; the 28 continuation notes point there),
  NotePeriods 28 bytes.  Wrong descriptions fixed: SongHeaders renamed
  SongPatternBases (pattern base addresses, not song headers), SongChannelOffsets
  (absolute table bases), InstrumentNoiseBytes (field label, not a 1,329-byte
  block), SoundTick / StartMusicChannel(s) / StartSfxChannel(s) comments, sound RAM
  comments.  Sound-driver branch labels renamed descriptively (@order_play_pattern,
  @play_square1 ...).
- 2:$B63D ChampionPictureData renamed ChampionPalette2 (32-byte palette only);
  Bank2FillFF and F:$FF57 FixedBankFillFF described as unread $FF fill.
Generic labels: 0 (only @return names on rts, as before).  Verification: verify
byte_identical true; final audit see analysis/handoffs/round-06-commentary.md.

## Round 5 (complete, final acceptance, 2026-11-20)
Baseline: pass-005 changed the map (L-025..L-027 splits/merges/relabels, 1,401
regions, no kind changes, 100.00% classified).  The whole round-4 commentary was moved,
untouched, to analysis/round-4/commentary/ (round-1..3 copies kept); `commentary.py
prepare` built a fresh frozen baseline from pass-005.  notes/ and tools/ were copied and
re-applied by bank/CPU address; checknotes first reported 7 stale keys (the removed
3:$892C/$894D/$8975/$8991 and 0:$9087/$BE40/$BE6F splits), now 0.  review.json: 1,384
entries carry earlier credit; 17 entries whose source text changed (tools/review.py diff
against the round-4 source) were re-reviewed as round 5.

Changes: MetatileTileIndex renamed MulTable16x16 (product table, read with
half * 16 | MetaCount; inline notes at 3:$8517/$8579/$8597); MapTruckPath described as
8 stops x 8 sprite tile ids and AnimateMapTruck header corrected; 0:$90B2 generic note;
spark tables' notes updated (truck-1 SparkStartY tail unobserved); L-028 tables named
TruckEndBitMasks (F:$C723) and TileReplaceRowStarts (F:$D450); PieceTableUnreadA/B at
0:$BDE8/$BE00 (DebrisAttrs now 24 bytes).  uncertainties.md updated.
Generic labels: 0.  Verification: verify byte_identical true; final audit see
analysis/handoffs/round-05-commentary.md.

## Round 4 (complete, final acceptance, 2026-11-20)
Baseline: pass-004 changed the map (6 annotate edits, 1,398 -> 1,401 regions, no
kind changes, 100.00% classified).  The whole round-3 commentary was moved, untouched,
to analysis/round-3/commentary/ (round-1/round-2 copies kept); `commentary.py prepare`
built a fresh frozen baseline from pass-004.  notes/ and tools/ were copied and
re-applied by bank/CPU address; checknotes reports 0 misplaced keys.  review.json:
1,244 entries carry earlier credit (rounds 1-3); 157 entries whose source text changed
this round (found by a per-region diff against the round-3 source, tools/review.py)
were re-reviewed as round 4.

Changed regions: LegalScreenTiles / LegalScreenAttrs (2:$B0E7 / $B4A7, raw legal
nametable, reader unknown); bank 5 split into TerrainAttrRecords tail, Data_5_8388
(640 zero bytes, 640-byte length fixed), EventGroundHeights (46 x 8 record format) and
ColumnRecordPtrs head.  UpdateGroundHeight header and inline notes now explain the
pointer arithmetic, the 9-bit average and the dead `cmp` with A = 0.
Redirect corrections: RampAirVel/RampBumpSpeed entry-1 reads observed; F:$FF2C and
6:$8701 reads are rts dummy reads.

Editorial fixes: event/hill RAM $0493.. renamed LaneTerrainBuf (was PaletteBuf+N, 80
operands incl. HillSwapTerrainRows); MetatileRowOfTileRow (F:$D157, previously called
unread) named; race $04AF,x RespawnSearchCount (was PaletteBuf+30); CheckCanAfford
pointer constants symbolic (TempMoney, GarageSlotA); WorldToTileIndex header rewritten;
223 comment blocks lost a stray leading "| " (notes `C X:Y | text` form).  The 17
remaining generic branch labels (@if_zero/@if_pos/@join/@loop ...) got descriptive
names (notes/r4_labels.txt).

Remaining generic names: 0 @skip, 0 plain @loop, 0 @join, 0 subject-less condition
labels.  Raw addresses only in the RAM-clear loops, the dead `jsr $0014`, the `and $EF`
slip and the `lda ($00,x)` timing pads.
Verification: commentary.py verify byte_identical true after each batch; final
`commentary.py audit` (see analysis/handoffs/round-04-commentary.md).

## Round 3 (complete, final acceptance, 2026-11-20)
Baseline: pass-003 changed the map (118 edits: splits/relabels/provenance; 1,385 ->
1,398 regions; 100.00% classified).  The whole round-2 commentary was moved, untouched,
to analysis/round-2/commentary/ (round-1 copy at analysis/round-1/commentary/ kept);
`commentary.py prepare` built a fresh frozen baseline from pass-003.  notes/ and tools/
were copied and re-applied by bank/CPU address (physical ranges); tools/checknotes.py
reports 0 misplaced keys.  review.json: 1,381 entries carry their earlier review credit
(review_round 1/2), 17 new/changed entries are round 3.

Carried records invalidated and rewritten: 0:$A000 and 2:$A000 / $B082 block comments
(now continuation / format notes), DrawRaceObject, SampleRaceTerrainTruck,
MatchButtonSequences, TitleMenu, SelectStartOffset, ShopMain, ScriptOp1D, NmiHandler2,
NmiHandler13, PalettePtrTable, SfxPtrTable / SfxRecords, Data_5_8388, the hill
special-sequence note and RaceObjFrame (an object-script id, not a sprite frame).

New notes files: r3_tables_F.txt / r3_tables_06.txt (role names for the 166 reader-named
tables), r3_regions.txt (format notes for regions still showing generic consumer lists
and for uncommented labels), r3_ram.txt (RAM names, stale-alias fixes, symbolic
cross-bank operands).  tools/annotate.py: new mechanical names for unnamed forward
targets (branch condition) and loops (body), so no @skip label remains.
tools/tblinfo.py / skipinfo.py: editorial listing helpers; tools/r3_renames.json:
old -> new table names.

Remaining generic names: 0 @skip, 2 plain @loop, 5 @join (if/else merge points),
12 condition labels without a symbolic subject (@if_zero, @if_pos ...), 0 X_Tbl names.
Verification: commentary.py verify byte_identical true after each batch; final
`commentary.py audit` (see handoff).

## Round 2 (complete, 2026-11-20)
Baseline: phase-1 checkpoint pass-002 changed the map (295 new regions, region 657
bank 0 $8055-$805D Data->Code; 100.00% classified).  The whole round-1 commentary was
moved to analysis/round-1/commentary/ (only copy, untouched), then `commentary.py
prepare` built a fresh frozen before/, baseline.json and phase1-state.json.

Method (unchanged): source/ is generated from before/game.asm + notes/*.txt by
tools/annotate.py (symbol substitution, labels, comments only).  Rebuild:
`python3 tools/annotate.py build && python3 tools/checknotes.py && python3 tools/review.py`,
then `commentary.py verify` and `read`.  Edit notes, never source/game.asm.

Carry-forward: round-1 notes copied and re-applied by bank/CPU address (they are
keyed by context, so they follow the physical ranges).  Invalidated and rewritten:
82 comment blocks that described now-classified unknown ranges, the per-bank
"unclassified" defaults (NB), the bank-1 / bank-5 / bank-4 overviews, ScriptOpTable,
and prose that cited auto-generated table names whose numbering shifted (found by a
round-1/round-2 name-to-address diff; 10 references fixed).  Round-1 defects fixed:
four cross-bank X tags at wrong instruction addresses (C13F/C14F/C47F/C497), bank-6
alias of $0687-$069E (was shown with race names), stale cross references.

New notes: notes/r2_b012.txt, r2_b3456.txt, r2_bF.txt.
- Named data layouts with explicit extents (D label + length; each region inside
  a layout prints "Continues <name>"): bank 1 EventScreenRaw + 21 RLE screen streams
  named by screen id/user; bank 0 PalettePtrTable/Palettes, ScriptPtrTable/
  ObjectScripts, StoryTextBlocks, StopNameStrings, DistanceScoreTable, MapRouteDots;
  bank 2 SongPatternTables/SongHeaders/NotePeriods, title palette tables; bank 3
  TrackBlockPtrs/Metatiles/TrackBlocks/TrackMaps, CpuPathListPtrs/Lists/Records;
  bank 4 AnnouncerSample, SfxRecords, ResultsText, FontTileMap; bank 5 full event
  data layout (ThrottleCurves ... SubColumnOffsets) and engine tables; bank 6
  HillNmiStepTable, hill terrain records/index/blocks/height profile; fixed bank
  LoadScreen_Tbl (record format), SfxPtrTable, SampleDescriptor, short tables.
- New code described: WarmStartMenu, HillNmiShift* handlers (6 routines), debug hex
  print pair, copies/siblings, unreached arms and residue (explicitly uncertain).
- Cross-bank operand resolution extended (X tags): PalettePtrTable, FinishLinePage,
  EventColumn tables, SceneryBlockPtr, Reset/WarmRestart targets, FontTileMap.
- Branch labels: unnamed targets now get mechanical descriptive names from what the
  target does (@check_X, @call_X, @set_X, @store_X, @loop_...); 210 plain @skip
  labels remain where the target has no symbolic operand.  No address-derived
  routine labels remain (stats: auto 0).
- review.json: 1385 scaffold entries reviewed (review_round 1 = 1089 carried with
  unchanged ranges, 2 = 296 new/changed); 810 units; editorial_audit pending.

Verification: commentary.py verify byte_identical true (verification.json); reading
copy refreshed (reading-after.asm).

Unresolved / deferred: uncertainties.md (interpretations, uncalled static code,
generic data roles, raw banked operands, redirect questions).

## Round 1 (complete)
See analysis/round-1/commentary/progress.md.
