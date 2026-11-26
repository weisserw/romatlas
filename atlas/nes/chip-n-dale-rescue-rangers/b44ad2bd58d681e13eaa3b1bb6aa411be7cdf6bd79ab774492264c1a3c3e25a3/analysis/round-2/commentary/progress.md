# Commentary progress (round 2, checkpoint pass-002)

Method (unchanged from round 1): source/game.asm and source/symbols.inc are
generated from the frozen before/game.asm by tools/annotate.py using the
annotation files in notes/ (symbols.txt + *.ann, keyed by bank:CPU address =
physical offset). Edit notes/, rerun `python3 tools/annotate.py`, then
`commentary.py verify`. tools/lst.py BANK START END prints a per-instruction
listing with addresses; tools/review.py refreshes review.json.

## Round transition
- maps/current.json and analysis/state.json (sha256 8bc887dd...) differed from
  the round-1 frozen baseline (8404126a...). The complete round-1 commentary
  directory was copied to analysis/round-1/commentary (diff -r identical) before
  `commentary.py prepare` built a fresh baseline from pass-002 (1031 scaffold
  entries, byte-identical).
- Carried: notes/ and tools/ from round 1. Annotations are keyed by physical
  offset, so names/comments/operand symbols reattach to the same bytes. All
  round-1 annotations were placed (no unplaced-key warnings) and the build was
  verified before new edits.
- Invalidated/revised carried text: unknown-range notes for ranges classified
  in pass 002 (notes/unknown.ann rewritten), "phase 1 left unknown"/"code lead"
  notes in b2/b3/b6c, SoundCommandTable start ($816C, 8 entries; was $816E),
  the $CB9C LiftedBlock note (now LiftTileAnim data), the Select/tally
  descriptions in MainLoop/UpdatePlayers, old Sub_XXXX references in prose.

## Round-2 batches
- r2-b2.ann: SelectPauseScreen (save/restore scheme, figure tables, palette),
  SelectSubScreen ("GIVE UP?" -> PlayerAction 10), BonusStageIntro /
  BonusStageItems / BonusIntroWait (was "level-clear tally") and their tables
  (ZoneLetterTiles, BonusItemPos/Pick/Anims, PauseFigure*, SubScreenOam...).
- b3.ann: SoundCommandTable entry 0 SoundStopAll; commands 5-7 renamed
  SoundCmd5RampFlagged / SoundCmd6Ramp / SoundCmd7Offset.
- r2-b6.ann: MovingItemHandler23/24/25/27 with helpers Item23RestOnItem23 /
  Item23BlocksItem23; pick-up effects PickupAddSecondCounter, PickupRestoreHeart,
  PickupRefillHearts and the pick-up / per-player tables; PlatformAdvanceRightEdge.
- r2-b7.ann: MoveDownProbe/MoveUpProbe (callers now known), PpuCtrl helpers,
  FillRamPages, ClearSpritesAndFadeIn, InlinePointerDispatch,
  PlaceCarriedObjectAlt, StandOnPartnerOrphanTest, DemoInputRecorder, TaskFree,
  TaskExitCurrent (structure-only code marked as such).
- r2-fixups.ann: labels for pass-002 data tables read with base offsets
  (CarryHeightByAnim/HeadOffsetByAnim/PlaceOffsetByAnim, TileSignedOffsets,
  LiftTileAnim, DroppedBlockYVel*, BossPartMoveDir, PlayerWindowBounds,
  MetaspriteHitAdjust, Boss4F shot tables).
- tools/annotate.py: evidence-specific notes for the new data region kinds
  (consumer tables, record patterns, filler, fills), wrapped; repeated generic
  notes shortened per bank; phase-1 labels whose name moved are dropped.
- HUD counter layout documented (InitTable05A0, HudCountersP1).

## Coverage
- All 24 newly classified code ranges (2,115 bytes) are inside named,
  commented routines with descriptive branch labels; no address-only labels.
- 2,080 labels (1,795 hand-named), 2,359 annotations; 28 raw base-offset
  operands remain where the index domain is not established (listed in
  uncertainties.md), all present in round 1.
- review.json: 1031/1031 scaffold entries reviewed, 679 units;
  editorial_audit pending (final acceptance only).

## Verification
- `commentary.py verify`: ok, byte_identical true (commentary/verification.json).
- Reading copy refreshed (`commentary.py read`).
