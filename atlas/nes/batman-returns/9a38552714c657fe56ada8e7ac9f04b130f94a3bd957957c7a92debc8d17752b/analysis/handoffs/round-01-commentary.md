# Round 1 commentary handoff - INCOMPLETE (readability gate not fully met)

Baseline: checkpoint pass-001 (maps/current.json == maps/pass-001.json; frozen in
commentary/baseline.json + phase1-state.json). Classification unchanged: 86.76%
(341171 / 393232 bytes classified; 52061 unknown).

## How the source is maintained (resume here)
commentary/source/game.asm is RENDERED - do not hand-edit it. Edit commentary/ann/*.txt
(DSL documented in analysis/tools-commentary/build_ann.py), then:
  python3 analysis/tools-commentary/build_ann.py && python3 analysis/tools-commentary/render.py
  python3 analysis/tools-commentary/review.py
  python3 /root/romatlas/scripts/commentary.py verify <ws>   # then: read <ws>
Helpers: cm.py bb BANK [lo hi] (compact listing), summ.py BANK (per-routine feature summary).

## Covered scope (all classified source, every bank with code)
- All 636 routine/entry points named descriptively (0 address-only routine names) and each
  has a block comment; core engine (banks 14/15), sound driver (bank 0), player logic
  (bank 4), combat/stage events (bank 5) explained in most detail.
- ~1500 branch targets labelled <Routine>_Loop/_Skip/_Return/_Continue; ~250 RAM symbols
  (object arrays with PLAYER_SLOT, sound channel arrays, IRQ split arrays, HUD/scroll
  state); hardware registers; 77 named data tables + 213 Data_Bnn_XXXX equates.
- 38 inline jump tables emitted as `.word` label lists; data regions show phase-1 evidence;
  all 233 unknown ranges carry "Unclassified bytes" notes (range, consumers, hedged roles).
- Formats documented in source: PPU screen script (PpuScriptRun), VramBuffer (VramBufferWrite),
  string scripts (PpuScriptQueue), metasprites (EmitMetaSprite), animation/motion scripts,
  IRQ split lists, sound header/sequence commands, text scripts, spawn batches.
- review.json: 1355/1355 scaffold entries reviewed with source lines, 1374 units; editorial
  audit pending (final acceptance only).

Examples: JumpEngine/BankPushAndMapY/BankPopAndRestore (bank-number byte at $8000 of each even
bank), NMI/IRQ split handler, Mode1_KonamiLogoScreen anti-tamper checksum -> DamageDoubler,
StageSelectByP2 debug select, SoundPlayRequest id ranges, PlayerAttackInput 4-frame buffer.

## Verification
commentary.py verify: ok=true, byte_identical=true (commentary/verification.json).
Reading copy refreshed: commentary/reading-after.asm.

## Remaining gaps (why INCOMPLETE) - exact ranges
1. In-routine control-flow comments and descriptive branch labels still missing in these
   large routines (physical ranges; systematic _Skip/_Loop labels only):
   StageLoadGraphics 0x081FB-0x083CC; PlayerJumpInput 0x08848-0x0893B; PlayerAttackInput
   0x0895B-0x08B6B (partly done); PlayerAirborneUpdate 0x08C4B-0x08D99; PlayerSetPose
   0x08D9A-0x08EFB; PlayerStartAction 0x08EFC-0x09023; EnemyTakeHit 0x0A205-0x0A30C;
   EnemyHitReact_Knockdown 0x0A3A8-0x0A48E; PlayerAttackCollide 0x0A83D-0x0A9FB; BossTakeHit
   0x0A9FC-0x0AADA; EnemyAttackHitsPlayer 0x0AADB-0x0AC0A; VehObj_Enemy 0x0D1AB-0x0D364;
   VehObj_Boss 0x0D54C-0x0DCA1 (union of its units across intervening unknown bytes); MenuInitNewGame 0x14011-0x14195; CutsceneUpdate
   0x14AC8-0x14E6A; CutsceneObj_Mover 0x15197-0x15418; BossA_Attack 0x16503-0x165F9;
   BossE_Fight 0x17BC7-0x17D91; StageSpawnScript 0x18F81-0x190F3.
   (IRQ, DrawObjectSpritesBody, ScrollQueueColumn, ScrollUp, DrawMapBlockAt, SndPlayNote have
   flow notes.)
2. Banks 6/7/10/11 boss/vehicle routines have one-line, partly generic explanations
   (BossA..E, EnemyB/C, Vehicle*) - group identities not established.
3. No symbolic constants yet for sound ids (PlaySound arguments), ObjState values, ObjType ids.
4. ~45 scene/vehicle scratch RAM addresses ($05CC-$05E7, $0600-$0619, $E4, $9F, $BB, $0108/9)
   remain numeric; 213 table equates keep Data_Bnn names.

## Unresolved meanings
See commentary/uncertainties.md (pair roles, vehicle engine, anti-tamper bit4 consumer, P2
stage-select enable, sound channel 4/5, weapon semantics, unresolved jumps into unknown bytes).

## Structural questions for redirect (bounded checks suggested)
- L001: ObjHandlerTable $DE19 (phys 0x1DE29, 124 unknown bytes): entry count is bounded by
  ObjType values written by SpawnEnemyGroup (0x190F4) and BgEvent_Spawn* - collect the
  immediate/table ObjType values to fix the extent, then classify entries per EnemyGroupBank pair.
- Hit-box tables $FAF2-$FBCB (bank 15): consumers EnemyHurtBox/EnemyAttackBox index by
  ObjVar47C/ObjVar490/ObjVar4A4; index domain could bound the tables (data, not code).
- QuarterWaveTable $C314 (64 B): consumer masks index to 6 bits -> extent proof candidate.
- Bank 6 `jsr $A303` (from 0xD25B) and bank 7 `jmp $89BE`/`$8759`: targets in unknown bytes
  with known pair context - static roots for descent.
- $8159 checksum source pair (logo screen) - confirm mapped pair by a bounded trace.

## Toolkit
No new toolkit issues found in this phase (analysis/toolkit-issues.md unchanged). No server
was started.

## Part 2 (resumed worker) - progress log
Tooling (workspace-local, analysis/tools-commentary/): render.py now (a) emits a one-line role or
mechanical consumer description for every data equate (`dat ADDR Name | text`), (b) supports
`sym ADDR EXPR` operand overrides (same value) for shared RAM, (c) picks the most specific RAM
range (fixes old mislabels such as sound vars shown as BgClassMap+N), (d) allows two RAM symbols
with one base. New helpers: sv.py BANK LO HI (symbolic compact view), hsum.py (range summary).
Backups of the Part-1 scripts: *.bak-part1. New ann files: p2-04, p2-05, p2-06, p2-10, p2-11, p2-ram.
Corrections of earlier names: BgClassMap is $0590-$064F (two 96-byte pages); PlayerStartDeath ->
PlayerSetStanding ($C1 PlayerStandingFlag); PlayerSetPose -> PlayerClearMoveParams; PlayerStartAction ->
PlayerSetJumpFrame; PlayerAirborneUpdate -> PlayerMoveGround (+PlayerJumpArc); bank-5 collision set
renamed (PlayerAttackHitEnemy, Weapon1/2HitEnemy, EnemyContact_*, BossW5_* = stage 5-2 boss);
bank 6 VehObj_Enemy/ScriptedMover/Boss -> VehObjActivate/VehObjSetAnimCtl/VehObjRemoveOffLeft and
VehObjHandlerTable entries labelled VehTypeNN; MenuInitNewGame -> PasswordEncodeShow (+PasswordDecode);
CutsceneUpdate -> IntroWaitTimer with IntroStepNN handlers; CutsceneObj_Mover -> CutsceneSpawnParticle.
Boss identification (bounded runs, /tmp probe, StageIndex write + ~60 traced frames): 1-2 -> BossA
(now BossW1), 2-3 -> BossB/C (BossW2), 4-3 -> BossD (BossW4), 5-2 -> stage-$12 code in bank 5 (BossW5),
6-2b -> BossE (BossW6); 4-1 runs the bank-6 vehicle objects, 6-1 the bank-13 ones. $B7 renamed BossClass.
Done in-routine flow: StageLoadGraphics, PlayerWalkInput, PlayerJumpInput, PlayerAttackInput,
PlayerMoveGround/JumpArc, PlayerWalkSteer/AirSteer, action continuations, EnemyTakeHit,
EnemyHitReact_*, EnemyContactUpdate, weapon hits, BossW5 set, VehType handlers, password,
intro steps, cutscene particles, BossW1_Attack, BossW6_Fight. Scratch RAM: all numeric RAM
operands named (p2-ram.txt). Verify byte-identical after these batches.

### Part 2 final status - readability gate: COMPLETE (editorial audit still pending, by design)
All gaps listed in "Remaining gaps" above are closed:
1. All 19 large routines (ranges in item 1) now have block comments on their control flow and
   descriptive branch labels (only `_Return` labels on rts and genuine `_Loop` labels remain
   systematic inside them). Several routines were renamed because the old names were wrong
   (see commentary/uncertainties.md, "Renamed after Part-2 review").
2. Boss groups identified by stage (bounded runs; evidence: analysis/tools-commentary/bossprobe.py
   + bossprobe-out.txt, artificial StageIndex writes): BossW1 (1-2), BossW2 (2-3), BossW4 (4-3),
   BossW5 (5-2, bank 5), BossW6 (6-2 part 2); 3-3 boss handlers unidentified (likely unclassified
   bank-11 gaps). EnemyB/EnemyC kept as arbitrary letters with explicit uncertainty notes.
   Vehicle code: 4-1 = bank-6 objects, 6-1 = bank-13 objects (VehTypeNN handler labels).
3. Constants: 45 sound ids (SFX_*/MUS_*/SND_FADE_OUT, 67 operands), player hit types (PHIT_*),
   object types (OBJT_*), boss W1 states, OBJST_IDLE (31 operands); notes in the Constants block.
4. No numeric RAM operand remains (all ~41 named in p2-ram.txt, shared bytes via `sym`);
   generic Var*/Unk* symbols carry mechanical usage notes (p2-ramnotes.txt).
5. Data equates: 180 named by role; the remaining 110 Data_Bnn_XXXX keep mechanical names with
   an automatic one-line note (consumer routine + access mode) on every equate.
Verification: fresh commentary.py verify ok=true, byte_identical=true; reading copy refreshed;
review.json 1355/1355 reviewed, 1414 units; classification unchanged (maps/current.json ==
pass-001, 86.76%).
Remaining (not part of this gate): systematic _Skip/_Loop labels outside the 19 listed routines
(~1300, e.g. sound driver, PPU helpers) are still mechanical; final editorial audit pending.
Structural questions for redirect: unchanged list above, plus: stage 3-3 boss handlers are
probably inside bank-11 unknown ranges $AEE5-$B536 / $B642-$B992 (observed-execution
candidate: run stage $0D to its wave $0B/$1F); VehObjHandlerTable ($910E, 45 entries) and
IntroStepTable ($8A0D, 35) / CutsceneObjHandlers ($8EE8, 13) are pointer tables whose
unclassified targets are static descent roots.
