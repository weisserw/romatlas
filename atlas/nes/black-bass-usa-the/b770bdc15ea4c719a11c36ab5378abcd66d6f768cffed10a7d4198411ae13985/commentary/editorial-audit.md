# Final editorial audit (final acceptance, 2026-11-20)

Input: checkpoint commentary-002 (byte-identical; 767 regions, 131088/131088 bytes
classified).  Output: checkpoint final-001 (maps/final-001.json, generated/final-001,
analysis/final-001-verification.json; byte_identical=true, map sha256 1ea41d78...435d).
Classification is unchanged: same 767 regions with identical offset, length, kind,
provenance and confidence.  Only 42 region-start labels changed.

Method: source/game.asm is still regenerated from before/ by tools/annotate.py and
tools/spec.txt (byte-neutral; ca65 checks every symbolic operand at verify).  All edits
went into spec.txt plus small renderer additions in annotate.py.  review.json was rebuilt
by tools/build_review.py (scaffold kept).  Valid prior review credit was reused; only the
listed findings and concrete inconsistencies were re-read (no whole-source rewrite).

## What was checked

- Redirect readability findings R-01..R-14 (analysis/handoffs/round-002-redirect.md).
- Known issues from the commentary-002 handoff: generic labels, PPU/character constants,
  CopyTilesInline arguments, address-based data names, sound-number prose, review
  summaries.
- Consistency between names and the data they describe (table entries, pointer targets,
  display-list sources), header-less routines and unlabelled or unannotated data tables.
- Inventory: 767/767 scaffold entries and 1048/1048 units reviewed (no truncated or
  placeholder summaries left, except the "continuation" units of multi-region routines,
  which point at the routine header).

## Fixes

Content and logic:
- Ranking (R-04..R-09): PlayerBeatsRivals renamed PlayerBehindRivals.  It returns A=$FF
  when PlayerTopTotal < RivalTotal (the old comment had this reversed).  CompareTotals now
  says a deficit worsens the place by (RivalTotal-PlayerTopTotal)/LakeTotalDivisor, up to
  the class worst place.  RankPoints is renamed ParticipantRank (overall place 1..200,
  lower is better; A 1-50, B 51-100, C 101-200).  Also renamed: LakeBestPlace,
  LakePlaceLimit, ClassPlaceBase, ToInClassPlace/ToOverallPlace, SetClassBest/WorstPlace,
  ImprovePlaceForRival (3/2/1 for beating rival best #1/#2/#3) and SwapBoundaryPlaces
  (promotion/relegation swap).  Place comparisons are now written in decimal.
- Ceremony (R-08): in-class place 1-5; Txt_YouGotA2nd is a template whose "2ND" is
  overwritten by PrintPlace.
- R-01: DataB858 is now CastAnimFrames, PowerCursorFrames, PowerCursorSprite and
  CastAnimFrame0-4, rendered as word tables and sprite rows.  The bank-7 records
  (LureObjCast, FishObjCast, PowerCursorObj, CastAnimObj) are rendered with their Frames
  field as a label.
- R-02: TileMapAB35 is now LandingBoxTilesCopy (byte-identical copy; no reader found).
- R-03: sprite-list entries are in the order attributes, dX, dY, tile.  The DrawObject
  and FishK*Frames notes now say so.
- R-10: MUS_SCREEN_END is now MUS_STOP (measured silence).
- R-11: CastPhase5Splash is now CastPhase5Bounce, with BounceOffsets and BounceStep.
  Phase 3 is described mechanically.
- R-12: the DrawGreeting class order is fixed.  R-13: the $C040 and lone-rts notes cite
  the slot layout and the 13-of-16 epilogue survey.  R-14: commentary/uncertainties.md
  has a final-acceptance section.
- New inconsistency found: the handlers SndEnvelope1-9 had been numbered by address,
  not by their SndEnvelopeTable entry (for example, table entry 2 pointed to
  "SndEnvelope3").  They are renumbered to match the table, the table is rendered as
  .word handler labels, and SoundCmdJumpTable is renamed SndEnvelopeTable.
- New inconsistency found: DL_LakeLabels0-3 / DrawLakeLabels actually draw the 16x22
  lake map picture at row 6, column 2, which is the origin MoveBoatOnMap uses.  They are
  renamed DL_LakeMap0-3 / DrawLakeMapPicture.  Their palette, attribute, terrain and
  picture blocks are labelled (LakeMapPalette/Attr/Terrain/Picture0-3), and the
  LakeMapPalettes/Attrs/Terrain pointer tables are rendered as labels.  The attribute
  and terrain blocks were previously described as "non-referenced".

Readability:
- About 330 generic @skip/@loop/@jump labels renamed, plus 14 cross-scope
  "Owner_skip" globals.  The remaining 56 @loop and 82 @return labels are in routines
  of at most 15 instructions.
- NT_ADDR(nt, row, col) is used for name-table addresses in PPU pointer pairs and
  display-list records (274 uses).  Name/code entry uses ca65 character literals ('A',
  'Z'+1, '0'-26 ...).  Joypad compare chains use BTN_* masks.
- CopyTilesInline arguments now name their source with the bank mapped at each call site
  (IntroTiles, LureMenuTiles, CommonTiles2bpp, FishingTiles2bpp_*, LakeMapDataBank0+off,
  FishLandedTiles, SwimAnimTiles, CeremonyTiles, CeremonySpriteTiles).
- Address-based names replaced: Tiles1bpp_/Tiles2bpp_* became FishingTiles{1,2}bpp_<PPU
  destination> and CommonTiles2bpp.  Display-list Tiles_XXXX blocks became TitleLogoTiles,
  TitlePictureA/BTiles, IntroPictureTiles, CeremonyPicture*, BlankRowTiles, DashTile,
  ParticipantDashTile and FrameBorderTiles.
- Stale "sound N"/"music N" prose replaced by MUS_*/SFX_* names.
- Headers added for 50 routines that had none (sound envelope tails, cursor
  initialisers, code packing, condition bonuses, terrain probe steps, fish distance
  helpers, NmiNormalFrame, ...).  Notes added for about 70 data tables (palettes,
  attribute tables, cursor parameters, name tables, bonus tables, lake data banks 1-4,
  zero fills, the "1988. Another Ltd. Ver." text).
- review.json summaries are cut at sentence boundaries.  Region summaries list their
  units with a first-sentence description.  Display-list and text units get generated
  descriptions.

## Verification

- `commentary.py verify`: byte_identical=true after every batch.  Final source sha256:
  3bb9ca97505e304c19fafcda5a90570aec8232c7520aec0205a6e0008e127cec.
- `commentary.py audit`: ok, byte_identical=true, reviewed_regions 767, reviewed_units 1048,
  editorial_audit true.  The reading copy (commentary/reading-after.asm) was refreshed.
- Map: labels were updated through map.annotate (see TI-006).  map.validate ok;
  analysis/state.json saved and maps/current.json exported.  A diff against
  commentary-002 shows only label changes.  `run.py checkpoint final-001` passed:
  validate, generate and byte-identical verify.

## Residual uncertainties (documented in source and uncertainties.md)

- SFX_QUIET (effect 1) is probably silence; it was not measured.
- The on-screen look of the fight reactions and retrieve actions was not checked.
- Per-frame extents inside FishK*Frames, FishSharedSprites, LureFrames and SplashFrames
  are not split; the format is documented.  The last LureMotionTables extent is unproven.
- Lake data banks 1-4 and LakeMapDataBank0 stay as single blocks; their header layout
  is documented, but the internal tables are not split.
- Visual content of the fishing tile sources is not identified; the names give the format
  and PPU destination.
- Some PPU high-byte immediates that are not in lo/hi pairs (for example $24 in
  DrawHudMeter and $23/$27/$2B in RequestAttrUpload) and a few address-suffixed
  duplicate text labels (Txt_Date_A0D3, ...) remain.  Their meaning is stated in nearby
  comments.
- The weakest structural claims are unchanged: $C040 (0.7) and the lone rts bytes
  $D76C/$E24A/$E30F (0.6).  These are classification notes, not editorial defects.
