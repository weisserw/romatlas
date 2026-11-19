# Round 2 commentary handoff - readability gate COMPLETE (editorial audit pending by design)

Baseline: checkpoint pass-002 (maps/current.json == pass-002 == commentary/baseline.json). Classification
is unchanged by this phase: 100.00% classified (393223 / 393232 B; 9 B unknown, lead L017). Readability
and classification are reported separately; this phase did not change the map.

## Baseline change handling
- Round-1 commentary (baseline pass-001) preserved first: analysis/round-1/commentary/ (whole directory,
  diff-verified identical before removal of the working copy) and analysis/round-1/tools-commentary/.
- Fresh baseline via `commentary.py prepare` (byte-identical) -> commentary/before, baseline.json,
  phase1-state.json, review.json scaffold (1909 entries).
- Carried forward by physical identity: ann/*.txt keys are (bank, CPU address). All carried label/comment/
  operand keys landed on instruction starts except one comment (moved $8368 -> $836E). Carried prose was
  re-checked where pass-002 changed facts (unknown ranges, jump-table extents, descent roots, wrong roles).

## How to rebuild (resume here)
Edit commentary/ann/*.txt (DSL in analysis/tools-commentary/build_ann.py; never hand-edit source/game.asm), then
  sh analysis/tools-commentary/rebuild.sh        # build_ann + jtables-based render + gen_auto (00-auto.txt)
  python3 analysis/tools-commentary/review.py    # refresh review.json units
  python3 /root/romatlas/scripts/commentary.py verify <ws>; ... read <ws>
jtables.py regenerates jtables.json from map evidence (run after a map change). ann/00-auto.txt is generated.

## Scope covered
- All ~780 new pass-002 regions plus carried round-1 source: 0 address-only routine names (Sub_Bnn = 0),
  0 numeric RAM operands, all switchable-bank operands symbolised except 4 documented ($8000 bank-ID read,
  ObjT33's $BE86/$8754 with unknown pair, logo checksum $8159).
- 193 code-pointer tables rendered as `.word` label lists; every dispatch owner has a role comment;
  ~640 table entries carry structural names (<Owner>_StNN, ObjTNN_Handler, VehTypeNN, IntroStepNN,
  EndingStep...) plus a generated "Entry / Mechanics" line; ~400 new names were written by hand.
- New/rewritten explanations: GameOverScreen (Mode 6, continue penalty, high score, password), EndingUpdate
  (Mode 7, 22 steps, branch by DamageTotal, credits), AttractStoryUpdate, story scenes A/B/C (sub-events),
  stage-event scripts by stage (bank 5, incl. FinalBossCleared -> GameMode 7), 62 ObjHandlerTable types
  (item carriers/pick-ups, weapon 1/2, enemy families, bank-15 boss effects), bosses W1-W4/W6 handlers in
  bank 11 (W3 = stage 3-3, new), vehicle steering/crash/collision reactions, WaveEntry placements (bank 12),
  math helpers (DivideU16x8/MultiplyU16x8), BgAnim channels, three anti-tamper checks.
- The 9 L017 unknown bytes have per-range notes. Data regions show their phase-1 evidence; 322 Data_Bnn
  equates keep mechanical names with consumer notes.
- review.json: 1909/1909 scaffold entries reviewed with source lines, 2113 units; editorial_audit pending.

## Examples of corrected interpretations
Lives -> WeaponAmmo; ScoreSecondary -> DamageTotal (ending selector); ResetWord108 -> HighScore;
VehicleWaveObjDispatch -> ObjBoxTestByType; VehObjExplode -> VehObjOffscreen; ObjClearVelocity ->
CutsceneAddVelX; Mode6/Mode7 dispatchers -> GameOver/Ending. Details: commentary/uncertainties.md.

## Verification
commentary.py verify: ok=true, byte_identical=true (commentary/verification.json, source sha256 e1c2ef15...).
Reading copy refreshed: commentary/reading-after.asm. input/game.nes unchanged (sha256 9a385527...).
Bounded runtime check: typeprobe.py (22 artificial stage starts x 1500 frames, ObjType census).

## Remaining (not part of this gate)
- Per-state game meaning of the structurally named dispatch entries; enemy character identities.
- Systematic local branch labels (_Skip/_Loop/_Return) in new code; final editorial audit.

## Structural questions for redirect (bounded checks suggested)
- L017 (9 B): dead RTS/JMP tails after terminal instructions at 0x08743, 0x0B1BE, 0x0B7A2, 0x14CCA,
  0x17BC6, 0x1C1E2-4, 0x1FD5A - only a supported convention or a reference could classify them.
- ObjHandlerTable pair ambiguity: are any non-boss types dispatched under pair 4/5 or 10/11 (BossClass != 0
  while minions run in loop 1)? Check: trace RunObjHandler with BossClass and ObjType in boss stages.
- ObjType $33 (fixed-bank spawner, jumps to $BE86 / calls $8754): which stage spawns it, and which pair is
  mapped (bounded: watch ObjType writes of $33 across the stage tour).
- StageEventIndex 3 set by event 9 (3-1 part 1) would index code bytes in StageEventByIndex; confirm it is
  consumed by StageLoadGraphics before StageEventDispatch runs.
- Hidden inputs found statically (ending replay A+B+Left on pad 2; ammo cheat Start + pad-2 A+B at story
  scene end) - runtime confirmation optional.
