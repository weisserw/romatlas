# Round 2 commentary handoff (input checkpoint pass-002)

Status: COMPLETE - current-source readability done for all classified source; fresh
byte-identical verification passed; reading copy refreshed.  Not ROM completion (5 bytes
still Unknown) and not final editorial acceptance (review.json editorial_audit pending).

## Scope
- Map changed since the round-1 baseline (pass-001 -> pass-002), so the whole round-1
  commentary was preserved unchanged in analysis/round-1/commentary/ and a fresh baseline
  was prepared (commentary/before, 1936 scaffold entries).
- Round-1 names/comments/RAM symbols carried by physical address (0 annotations off an
  instruction/data start after the move); 55 stale round-1 Unknown notes on now-classified
  ranges removed and replaced.
- Newly documented (pass-002 code, ~12.3 KiB): banks 8/13 Captain Planet object tasks (all
  MapSpawnTable/LevelInitTasks entries, children, helpers, tables), bank 3 special-object
  handlers ($1E-$25) and bank 6 event-object handlers (26-entry table), fixed-bank
  LevelClearPortraitTask and CreditsSpriteTask, L008 static routines/JMP slots, L004 fixes,
  consumer-proven data tables (named by role or consumer).
- No resume ranges.

## Readability assessment (residual counts, commentary/tools/metrics.py)
- Unnamed (address-only sub_/loc_/task_/dat_) globals: 0 (1399 code labels; round 1 left 39 dat_).
- Raw RAM operands: 0.  Generic @skip/@loop locals: 0 (29 flag-only conditions such as @ifCarryClear).
- Routine/block comments: 720 labels (uncommented ones are B*_ stubs, Vec* slots, internal targets).
- RAM: ~170 named symbols + bank aliases; 130 Unk_<owner>_<addr> symbols with "role uncertain" notes.
- Examples: n08 PodTask/BurstIntoFragments/FragmentTask, ShooterTask/ProjectileTask,
  GrabberTask/GrabberHold, Level1GoalTask; n13 WallCrawlerTask/WebShotTask,
  Level4BossTask/BossClawTask/ShieldBlockTask, ArmoredHitCheck; MapSpawnTable rendered as
  `.byte tile` + `.word Task`; EventObjectHandlers/SpecialObjectHandlers as `.word` tables.
- Corrections to carried names: $F4 CpHurtCount (was TerrainHitCount), LevelAnchorColumn/Row
  (was LevelSwitchTile*), LevelClearScreenSetup (was GameOverScreenSetup), BecomeDebris /
  CallSpawnItemDrop (were address-suffixed trampolines).

## Unresolved meanings (commentary/uncertainties.md)
- Game identities of all CP-stage objects; names are mechanical.  Level 3 boss arena
  (Level3BossTask: bank $0B map switch, EventPhase 2-4, ring-level loss) and level 4 boss
  claw/shield logic are tentative readings.
- SuperModeTimer ($078F) game name; Geo-stage SpecialEventFlag reuse.
- Bank 3 special types $20/$21 (SpecialDamWalker, SpecialPatrolObject) purposes.

## Structural questions for redirect (evidence-policy: bounded checks)
1. L009, 5 Unknown bytes: bank 13 $BA5D-$BA5F (4C C2 B4 'jmp DebrisSpawner' after
   'jmp DebrisSpawner / rts'), bank 15 $FB69 and $FDC0 (lone $60 after an RTS).  Unreferenced,
   never fetched.  Check: whether the assembler pattern (duplicate tail jumps/RTS elsewhere)
   justifies dead-code classification; otherwise keep unknown with this note.
2. Trailing `11 00 00 01` after bank 8 MapSpawnTable ($B20F-$B212) duplicates the bank 13
   DebrisSpriteDef header; confirm it is an unused copy (no reader found).
3. Reachability of static-only routines (bank 6 UnusedSq1Sound/UnusedNoiseSound/NegateTmp93,
   bank 8 CpWithinObjY, bank 12 SetHitBoxB6Copy12, bank 13 SetTaskBank7_13 /
   Level3BossTriggerFail / ProjectileHitsCp13_splat, bank 14 SubFromPtr0 / WriteSwitchColours /
   SetMusicPlaying): structure only, no caller known.
4. Gameplay check for the tentative boss readings: level 3 arena trigger at CpSavedX >= $D0
   (Level3BossTrigger) and level 4 GoalHitCount progression (watch $0709, $0719, $F5).

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <ws>`: ok=true, byte_identical=true
(commentary/verification.json; game.asm sha256 4cdc5457ab77...); `read` refreshed
commentary/reading-after.asm.

## Artifacts (local only)
commentary/source/game.asm, commentary/reading-after.asm, commentary/review.json,
commentary/progress.md, commentary/uncertainties.md, commentary/verification.json,
commentary/tools/ (build.py, review.py, metrics.py, mapdiff.py, ann/*.py incl. n03/n08/n13/n99_records);
round-1 archive: analysis/round-1/commentary/.  No toolkit issues found in this phase.
