# Round 1 commentary handoff

- Status: **COMPLETE** (current-source readability scope done; ready for redirect).
- Input checkpoint: pass-001 (maps/pass-001.json = maps/current.json, unchanged).
  ROM classification unchanged: 93.42% (122467 / 131088 bytes; 8621 unknown).
- Phase workspace: [commentary/](../../commentary/) (prepared by worker 1, resumed by
  worker 2; before/ frozen, never regenerated).
- Verification: [commentary/verification.json](../../commentary/verification.json) -
  ok true, **byte_identical true** (fresh run after the final edit batch).
- Reading copy (refreshed): [commentary/reading-after.asm](../../commentary/reading-after.asm).
- Records: [commentary/progress.md](../../commentary/progress.md),
  [commentary/uncertainties.md](../../commentary/uncertainties.md),
  [commentary/review.json](../../commentary/review.json): 685 scaffold entries all
  reviewed (528 Code, 0 pending), 615 units all reviewed; editorial_audit pending
  (final acceptance only; audit not run).
- Tooling (local helpers, not toolkit changes): commentary/tools/annotate.py +
  notes.txt (regenerates source/ from before/), review_update.py, dump.py,
  remaining.sh. No toolkit issues found in this phase.

## Scope covered

- All banks with code (bank 0 sound/VRAM/OAM, bank 5 handlers, bank 6, fixed bank):
  692 described routines, 992 descriptive local branch labels (no @loop/@skip left),
  about 1070 inline comments, 171 RAM symbols plus range-limited aliases for reused
  work bytes, 116 data labels, jump tables as `.word`.
- Bank 6 $9757-$BA4A (physical 104295-113227) fully read; several skim-level names
  corrected (block fragments/items, whip block reach, oscillator platforms, whip
  sprite placement, title/intro scene) and its data blocks partly identified
  (TitleScreenStreams, IntroChrTiles, IntroScreenStreams).
- Address-named fields replaced: StageStairListPtrs, StageExitPoints,
  StageTriggerListPtrs, StageCameraLockTable, StageBreakableBlockPtrs,
  StageRightLimitPages/StageLeftLimitPages, SpawnerWaveCount; kept-with-explanation:
  SpawnerUnused0178 (never read), ObjAux04F8/0514/0530 (multi-use, roles listed).

Examples: `SoundReadCommand` (stream command decoding with labelled branches),
`CheckObjectCollisions` (@WhipBox/@WeaponLoop/@ItemContact, item kinds commented),
`UpdateMovingPlatforms` + `OscAdvancePos` (oscillator motion), `CheckStageTriggers`
(trigger entry format), `State0F_Steps` (credits with @RAMRANGE aliases).

## Remaining gaps

None in the commentary readability scope. Whole-ROM classification (8621 unknown
bytes) and final editorial acceptance/audit remain separate gates.

## Unresolved meanings

Enemy/object identities, sub-weapon and item identities (inferred from cost and
behaviour), scene states 9/10/11/14, collision-map codes, unused-looking state bytes
($7D, $0178, $23 writer) - see commentary/uncertainties.md.

## Structural questions for redirect (7)

1. DPCM sample: `PlayDmcSample` ($FE72) plays $FF40 len 177 -> physical 130896-131072
   inside unknown 130838-131081. Likely classifiable as DMC sample data.
2. Indexed/absolute reads into unknown bytes: physical 24-25 (VramModeCtrlBits+x,
   FlushVramQueue), 405-406 (PlaySoundBank0 base pointer $8185), 115811-115812
   ($C453,x in SetupExitDoorObject), 84065-84066 (bank 5 $8851,y), and the first
   bytes of 118639 (EntryTriggerBoxes record 2, X = 3).
3. Bank-5 inline jump tables: physical 83872-83881 (phase-1 generic data; words
   $87AA/$87F6/$881F/$8853/$8899 = decoded states) and unknown 86433-86444
   (ObjT13_Crawler table). Could become `.word` tables and support L-001 context.
4. Bank context of `StageSpecialHandler` ($ED25) / `ObjT10_StageObject` ($ED4B):
   reached from object handlers (bank 5; handlers re-select bank 5 after
   PlaySound), yet phase-1 bank-6 code at $82A2/$84FF/$8C9A may partly rest on
   these jumps; also `CallStageSplashRoutine` `jsr $88B7` (bank 6 mid-routine vs
   bank-5 splash routine). Check evidence in maps/current.json for those regions.
5. Unknown 120971-121390 ($D87B-$DA1E) likely continues the item/candle record data
   (ItemRecordPtrsA/B); unknown 102577-103259 ($90A1-$934B) likely continues credit
   pages (CreditPagePtrs) - pointer-target checks could bound them.
6. NmiLagFrame `jsr $838A` without bank switch (physical 114914, $C0D2): effective
   target depends on the interrupted bank; phase-1 static entry `Chain_Next`-style
   mid-routine starts in bank 6 at $838A/$88B7 may be artefacts.
7. Text screen in unknown bytes: TextScreenPtrs entry 4 = $918D (bank 6, read by
   DrawTextScreen with X = 8 from State0F_Ending) points to physical 102813, inside
   unknown 102577-103259. That range therefore holds at least one RLE text-screen
   stream set (four pointers + $D4/$D9 streams); pointer-following could classify it
   together with question 5.

