- unit 10: DONE (round 1, worker 1f) - physical 0x16CDB-0x16F03 (tools/knowledge_rest.py)
- unit 12: DONE (round 1, worker 1f) - physical 0x1A4DE-0x1A82C (tools/knowledge_rest.py)
- unit 14: DONE (round 1, worker 1f) - physical 0x1F0CC-0x1F230 (tools/knowledge_rest.py)
- unit 16: DONE (round 1, worker 3) - all review units reviewed, physical 0x20033-0x23924 (tools/knowledge_u16.py)
- unit 18: DONE (round 1, worker 1f) - physical 0x24019-0x27905 (tools/knowledge_rest.py)
- unit 20: DONE (round 1, worker 3) - all review units reviewed, physical 0x28021-0x2A6F1 (tools/knowledge_u20.py)
- unit 22: DONE (round 1, worker 2) - all review units reviewed, physical 0x2C02B-0x2EBA4 (tools/knowledge_u22.py)
- unit 24: DONE (round 1, worker 1f) - physical 0x30015-0x339BD (tools/knowledge_rest.py)
- unit 26: DONE (round 1, workers 1c + 2) - all review units reviewed, physical 0x34017-0x36768 ($8007-$A757)
- unit 28: DONE (round 1 worker 1c) - all 368 review units reviewed, physical 0x38027-0x3BA0C
(Historical note from preparation:) These routines had generated names (`Uxx_<Topic|Hint>_<addr>`, `<Dispatcher>_CaseNN`)
and mechanical header notes only; their game-level roles need reading and
hand-written names/prose (add to knowledge_units.py). Branch labels there are generic
(@Loop_/@Skip_/@Return_). Suggested order: unit 28 and 26 case handlers (GameSubMode
scenes), unit 22 engine services, unit 20 front-end scenes, unit 16 text/screen code.
Also still numeric: ~150 less-used RAM addresses (e.g. $91-$9E, $0353, $0372, $0381,
$042D-$0432) whose roles were not established.

## Unit 28 completed (round 1, worker 1c)
- Knowledge in tools/knowledge_u28.py (merged by knowledge_units.py; RAM names via UNIT_RAM).
  All 246 routines/case handlers hand-named with headers; 10 shared mid-routine labels
  and ~27 branch labels named; 20 data tables named (SineTable/ArcTanTable at $84D8/$85D8,
  pitch/bat/catcher tables); ~40 RAM addresses named (BatterState record $0200-$0212,
  BallVelX/Y/Z $74-$76, PitchAim/Shift/Type/Hold $7B-$7E, Strikes/Balls $037C/$037D,
  PitchFlags $0372, AtBatResult $0378, UmpireCall $0400, Hit* $0411-$0416, SavedPalette
  $044F (PlayerCounters shrunk to 13 bytes), BatterSide/PitcherSide $0335/$0334, ...).
  Var0200 renamed BatterState.
- annotate.py fix: review-unit continuation owner now searched among routine starts of
  the same unit (previously the 0x38027 region was attributed to a unit-26 routine).
- Disassembly lead (for a later disassembly round): unknown $9734-$974D (unit 28) is the
  9-word case table + repertoire mask table of U28_ClampPitchToRepertoire; its case
  target $976B lies in the unknown run $976B-$9774, which is therefore code (pitch type 5).
- Remaining: units 26, 22, 20, 16, 24, 18, 14, 12, 10 as listed above.

## Unit 26 batch 1 (round 1, worker 1c)
- tools/knowledge_u26.py: $8007-$95AF named and described (mode-1 init cases, end-of-play
  logic: foul/home-run/return-to-at-bat, ball flight planning and physics, fielder states
  $40-$45, catch boxes, base coverage). RAM: BallLand1-3 $033E-$0349, BallHangTime $034A,
  BallVertSpeed $7F, BallAirFrames $86, BallNextPos $89; Var88 and Var0376 comments
  extended (bounce count; foul = 5 / 1-4 home run tentative); $0373 renamed PlayTimer.
- Naming question: RadarMirror ($0333) is also the ball-held/stopped flag in unit 26.
- Next: unit 26 from $95B0 (U26_Dispatch_95B0) to $A758, then units 22, 20, 16, ...

## Unit 26 batch 2 (round 1, worker 2) - unit 26 DONE
- knowledge_u26.py batch 2: $95B0-$A757 named (fielder dive/jump states $50-$5A,
  object movement with per-layout fence/wall collision U26_FenceCheck/Field0/Field2,
  HitFence/WallBounce, home-run detection via FenceZone $0377 -> Var0376, facing poses,
  D-pad/AI movement, fielding assignments AssignListPtrs/CoverPoints, FielderReassess,
  CoverBasesStep, ChaseLooseBall, SetVelocityTowardTarget). RAM: AimPoint $77, BasesCoveredMask
  $BC, BallChaserSlot $BD, ThrowerSlot $BE, ThrowTargetBase $0350, BaseUnderFielder $0351,
  BasesOccupiedByFielders $0352, FenceZone $0377, ThrowCharge $043E, SineArg $0479.
- New knowledge hook: UNIT_BLOCK (block comments at arbitrary instructions) merged into
  knowledge.BLOCK; used for the fence cases reached only through raw (unknown) case tables.
- UNKNOWN_CONTEXT notes for unit-26 unknowns $9949 (field-0 case table), $9A18 (field-0 case 9
  code), $9A64 (field-2 case table + case 5 code), $981E (unreferenced copy), $9DD6, $A025.
  Disassembly lead: $9949-$995C and $9A64-$9A77 are 10-word jump tables; $9A18-$9A59 and
  $9A78-$9AAD are code reached through them.

## Unit 22 (round 1, worker 2) - DONE
- tools/knowledge_u22.py (merged by knowledge_units.py, incl. FILE_NOTES and UNKNOWN_CONTEXT):
  all 173 routines/case handlers + 42 shared mid-routine labels named; 20 branch labels;
  16 data tables. Entry 0 DrawBatterSprite (+ DrawMetaspriteMirrored), 1 BatterBecomesRunner,
  2 RunnersStep (runner state machine per path segment: lead-off, steal, force/tag/fly-out/
  double-off, scoring rules), 3 AwardBaseToBatter (walk, forced chain), 4 FieldUmpireStep,
  12 CpuInputService (CPU pitcher/batter/fielder/base-runner input, substitution menus, demo).
- RAM: FieldUmpire $02C8 (was Var02C8), RunnerSegments $0353, BatterReachState $043A,
  BatterRunnerTag $0440, CpuInputTimer $62, CpuRequestFlags $63, DemoPlay $69; SideMapMode
  meaning settled (0 = 1P vs CPU, 1 = 2P, 2 = CPU vs CPU); RunnerTable comment updated.
  U22_ObjApplyDescent renamed U22_RunnerRetreat, U22_Oam_812B -> U22_DrawMetaspriteMirrored.
- Unknown-range notes: $81F4-$9044 batter frame records, $9B26 (dead code + tables of
  U22_RunnerForcedCheck at $9B5C/$9B5F), $A3EE dead code, $AAAA screen-sign metasprites,
  $AB94+ sprite/animation data.
- Still numeric in unit 22: $0355 (tag-up throw code), $0406/$0408/$0409 (CPU timers),
  $0358-$035A (batter stats), $64/$91/$92 (menu cursor state), $042D (difficulty?).

## Remaining (after round 1 worker 2)
- unit 20: 273 pending (0x28021-0x2A6F1), unit 16: 108 (0x20033-0x23924), unit 12: 16,
  unit 24: 14, unit 14: 11, unit 18: 8, unit 10: 6 (ranges as listed at the top).
- Then editorial cleanup and the final audit.

## Unit 20 (round 1, worker 3) - DONE
- tools/knowledge_u20.py (merged by knowledge_units.py incl. FILE_NOTES/UNKNOWN_CONTEXT): all 273
  pending review units of 0x28021-0x2A6F1 reviewed; every GameMode 3 sub-mode identified
  (0 title/demo, 1 team select, 2 sound test, 3/4 BATTING ORDER, 5 game intro, 6/7 pause
  SUBSTITUTION, 8 half-inning scoreboard (mercy rule 9 runs, game-over/X rules), 9 stadium
  intro, 10 password display, 11 final scoreboard + season progress (70 wins pennant,
  61 non-wins GAME OVER, series to 4 from SeasonGame $C8), 12 season cut-scene, 13/14 forced
  pitcher change, 15 ENTER PASSWORD, 16 GAME OVER, 17 CHR viewer, 18 player cut-scene,
  19 ending, 20 OPTIONS). Team block layout, player/pitching record tables, default lineups.
- RAM: MenuStep..MenuDone $91-$9B, PlayerLeague $B7, CutsceneId $B9, CutsceneFielder $BA,
  BatterMood $035A, PitcherCondition $0364, PlayerTeam $0381, PitcherRest $03EE[16],
  SeasonWins $042D, SeasonLosses $042E, DhOption $04A3.
- Correction in knowledge.py: HomeMorale/AwayMorale are the condition of the pitcher facing
  the home/away batters (ScoreRun lowers HomeMorale when the HOME team scores); the fixed-bank
  ScoreRun local labels were swapped accordingly. Names kept (renaming left for editorial pass:
  suggest AwayPitcherCondition for $0417, HomePitcherCondition for $0418).
- Unknown-range notes: $93D7 is the inline jump table + case code of U20_SubstitutionDrawStep
  (disassembly lead: code); $91FC-$9313/$9325-$93A3/$8936/$A372 unreferenced code variants;
  $8B45 OPTIONS strings, $9E76 cursor tables, $A6E1 team colours, $A935 non-DH lineup ptrs.
- Open: PlayerRecordByte3 (entry 7) indexes Arg10*8+Arg11*4 (caller passes Arg11 = 2) -
  misaligned for 12-byte records; $0379/$037A (cleared on pitcher change, no reader found);
  $0363/$0368/$0369 pitcher stat meanings.

## Unit 16 (round 1, worker 3) - DONE
- tools/knowledge_u16.py: all 108 pending review units of 0x20033-0x23924 reviewed. Entries:
  0 backdrop loader, 1 at-bat info panel (16-case item rotation: batter/pitcher names and
  stats, score line, pitch speed $03FF, inning, B/S/O lamps, umpire call, mood/condition
  bars), 2 board animation (GameMode 2 sub-mode 1, $F4/$F5), 3/4 in-game player cut-scene
  (sub-mode 18: scene load, fielder mood +/-, batting order advance, half-inning end),
  5 scoreboard clock, 6 GAME n, 7 password display, 8 ENTER PASSWORD editor + INVALID menu,
  9 field message sprite ($0438/$0439), 10 team-select emblems, 12-16 team names/texts.
  Password format decoded (EncodePassword/DecodePassword/PasswordChecksum; $042F-$0433).
  U16_NumberEntryInput renamed U16_PasswordEditInput.
- RAM: PasswordKeyRepeat $9D, PasswordState $9E, PasswordBytes $042F[5], FieldMessage
  $0438, FieldMessageTimer $0439, PasswordMenuChoice/Step $04A6/$04A7.
- Still numeric in unit 16: $F4/$F5 (board/panel record request and glyph variant),
  $9F/$A0 (cut-scene palette flash), $03FF (pitch speed source), $0358/$0359/$0365 stats.
- Remaining after worker 3: unit 24 (14), unit 12 (16), unit 14 (11), unit 18 (8),
  unit 10 (6) as listed at the top; then editorial cleanup and the final audit.

## Units 24/12/14/18/10 and readability sweep (round 1, worker 1f) - READABILITY COMPLETE
- tools/knowledge_rest.py (merged last by knowledge_units.py): unit 10 pitcher/batter close-ups,
  unit 12 title menu (8 MenuStep cases), unit 14 season schedule (SetUpNextGame, SetUpSeriesGame,
  CheckGamesPlayed; schedule $AC5A, NightGameBits), unit 18 scene loader + season-end cut-scenes,
  unit 24 pitcher (uniform-number digits) and catcher drawing. UNIT_ROLE/FILE_NOTES updated.
- annotate.py fix: data sub-labels are only shared for the unit-0 streamer copies (units 12/14 no
  longer inherit unit-10 data names).
- Sweep: every raw CPU RAM operand now symbolic (~75 new names, e.g. ScheduleDay $67, BallFlags
  $6C, SpinRate/SpinAccum $81/$82, Gravity $84, BallLiveliness $AF, ThrowPlayState $BF,
  BoardRequest/BoardVariant $F4/$F5, PitcherNumber $0366, PitchRepertoire $0367,
  PitchSpeedNext/Shown $03FE/$03FF, CpuPitchTimer $0406, OutsAtPlayStart $037F, TeamHits $039E,
  AwayErrors/HomeErrors $03A0/$03A1, PitcherCloseupDelay $04A8); unread bytes keep VarXX names.
  Renamed AwayPitcherCondition/HomePitcherCondition ($0417/$0418, ex HomeMorale/AwayMorale) and
  AwayPitcherRunsAllowed/HomePitcherRunsAllowed ($039C/$039D). Prose in knowledge files updated
  to the new names. Fixed-bank and unit-8 mid-routine labels named; unit-8 command handlers named;
  generic data labels (U08 $860F/$879D, U16 $9B0A, U26 $9270/$A758, U28 $907B, bank 31 $E000) named.
- review.json: 0 pending entries/units (editorial_audit pending by design). verify byte-identical;
  reading copy refreshed. Handoff: analysis/commentary-handoff-round1.md.
- Not done (editorial pass): ~1600 generic @Loop_/@Skip_/@Return_ labels inside explained unit
  routines; final audit.
