# Commentary progress

## Round 1 (input checkpoint pass-001) - INTERRUPTED

Prepared a verified baseline with scripts/commentary.py prepare (before/ frozen,
baseline-verification.json byte_identical true).

Method: commentary/source/game.asm is produced from the frozen before/game.asm by
commentary/tools/annotate.py plus the curated database commentary/tools/notes.txt
(names, header comments, RAM/data symbols, unknown-range notes, per-site bank
overrides). Only labels, operand spelling (symbols with preserved z:/a: widths),
`.word` jump tables and comments change. Re-run annotate.py after editing notes.txt,
then `scripts/commentary.py verify` / `read`. review.json units/statuses are filled
by commentary/tools/review_update.py. Treat source/ as canonical in later rounds:
if notes.txt is no longer used, edit source/ directly and never regenerate over it.

Done this pass:
- All 692 global labels carry descriptive names (no Sub_/Loc_/Handler_ address
  defaults remain); the 992 remaining branch targets are cheap locals.
- 122 RAM symbols (game state, timers, player, sound, spawner, crusher, object
  field arrays $0300-$059F as 28-slot arrays) and 30 hardware register names.
- 113 ROM data labels; 16 inline jump tables and 2 pushed return words emitted as
  `.word Label` (byte-identical).
- Header explanations for ~350 routines; notes for all 39 unknown ranges.
- Bank context: fixed-bank calls into $8000-$BFFF are labeled from `ldy #n / jsr
  BankSelectY` brackets or the default bank 6 ($24), with per-site overrides where
  the bank differs (object handlers run in bank 5; the NMI lag-frame call is unresolved).
- review.json: 610 units (405 reviewed, 205 pending); Data/Padding/Unknown entries
  reviewed with notes; all 528 Code entries pending (see below).

Verification: commentary/verification.json, ok true, byte_identical true,
game.asm sha256 35bf5f184f2b4fd06ba85c5029cca084aea9222f79a883359ba96289c3abf479.
Reading copy: commentary/reading-after.asm (refreshed).

Remaining (resume here, same phase):
1. Descriptive local branch labels: 992 cheap locals are structural only
   (@loop/@skip/@return); every code range in all banks.
2. Inline comments inside routine bodies (branch purposes, field use) - sparse
   everywhere; most needed in bank 0 sound driver (physical 407-1946), the fixed-bank
   object/collision code (physical 121391-127945) and bank 6.
3. Bank 6 physical 104295-113227 ($9757-$BA4A: player movement/stairs, camera, HUD,
   fixed objects, triggers, breakable blocks, platforms, title/attract states):
   named and summarised from header-level reading only; bodies need a full read
   and verification of the tentative meanings (units marked pending).
4. Field arrays still named by address (ObjVar04F8, ObjVar0514, ObjVar0530,
   SpawnerVar0178/018A, StageTableFBC8..FC4D, StageNibblesA/B).

## Round 1 resume - COMPLETE

Resumed the interrupted pass with the same tooling: notes.txt + annotate.py regenerate
source/game.asm from the frozen before/ listing; new local helpers tools/dump.py
(source view with [B:]XXXX keys) and tools/remaining.sh (structural-label counter).
annotate.py gained @RAMRANGE (range-limited RAM aliases for reused work bytes), drops
phase-1 one-line summaries duplicated by curated headers, and names rts targets
@Return. review_update.py now marks fully read code reviewed (no skim ranges left).

Scope covered (all banks: bank 0 sound/VRAM/OAM, bank 5 handlers, bank 6, fixed bank):
- Every structural @loop/@skip label replaced by a descriptive local name (992 locals;
  0 structural left); about 1070 inline comments on branches, field use and data access.
- Headers written for the 136 routines that had none; all 692 routines explained.
- Full-body read of bank 6 physical 104295-113227 ($9757-$BA4A). Corrections: the
  "BlockScreenPosition"/"SpawnBlockDebris" pair actually spawns block fragments /
  the hidden item (renamed SpawnBlockFragments/SpawnBlockItem, tables
  BlockFragmentTable/BlockItemKinds); "BreakableBlockPtrs" is the whip block reach
  (WhipBlockReach); "PlatformStepX/Y/Pos" are a discrete harmonic oscillator
  (OscAdvancePos/OscAccelerate/OscWorldPos + Alt field-set variants);
  "UpdateWhipFlash" places the whip sprites (PlaceWhipSprites); title/intro routines
  renamed (InitIntroScene, UpdateIntroScene, DemoStageTable, TitleObjectPath, ...).
  Data blocks there identified: TitleScreenStreams ($AEDC), IntroChrTiles ($B117,
  graphics set 1), IntroScreenStreams ($B637).
- Address-named fields resolved: StageTableFBC8/FBEE/FC14/FC3A/FC4D ->
  StageStairListPtrs/StageExitPoints/StageTriggerListPtrs/StageCameraLockTable/
  StageBreakableBlockPtrs; StageNibblesA/B -> StageRightLimitPages/StageLeftLimitPages
  (GetRightScrollLimit/GetLeftScrollLimit); SpawnerVar018A -> SpawnerWaveCount;
  SpawnerVar0178 -> SpawnerUnused0178 (never read); ObjVar04F8/0514/0530 ->
  ObjAux04F8/0514/0530 (multi-use, roles listed); Data_B0_8007 -> VramModeCtrlBits,
  Data_E9A4 -> ObjectHandlerReturnWord. Other renames: ReleaseCandleSlot ->
  ReleaseSpawnerSlot, CallBank6Routine96C1 -> CallPlayerTakeHit.
- 49 new RAM symbols (sound driver work bytes, stage streaming, collision/whip boxes,
  camera lock, HUD shadows, trigger/block bit sets) plus @RAMRANGE aliases for the
  credits and picture scenes that reuse the streaming bytes.
- Stale address names in prose (Loc_/Sub_/Data_ and ObjVar03FC-style) fixed.

Verification: commentary/verification.json ok true, byte_identical true. Reading copy
refreshed. review.json: 685 scaffold entries all reviewed (528 Code), 615 units all
reviewed; editorial_audit left pending for final acceptance. Classification unchanged
(93.42%). Unresolved meanings: commentary/uncertainties.md. No toolkit issues.
