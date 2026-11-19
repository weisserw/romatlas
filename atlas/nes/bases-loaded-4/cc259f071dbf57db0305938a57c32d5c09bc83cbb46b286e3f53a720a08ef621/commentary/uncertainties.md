# Commentary uncertainties

## Carried from round 1 (still valid under pass-002)

- GameMode meaning: 3 = front end (power-on mode, unit 20); 1 = scrolling field view
  (unit 26, camera/streamer calls); 2 = pitcher/batter view (unit 28, pause allowed).
  Modes 1/2 assignment is inferred from code, not checked per screen.
- SeasonGame ($66): games played by PlayerTeam (checked against the unit-14 schedule by
  U14_CheckGamesPlayed); $C8+ = championship series, $FF = exhibition (U12_MenuPickExhibition).
- Outs ($037E), Var0376, PitchPhase ($6A), morale/runs-allowed ($0417/$0418, $039C/$039D),
  PlayerCounters ($0442), team/pitcher/roster blocks ($0382-$0385, $03A4/$03C9) are
  tentative readings of fixed-bank bookkeeping code.
- UpdatePlayClock ($0435-$0437) drives the stadium clock hands (U16_DrawScoreboardClock); reading
  the start values 2/0 and 13/6 (U14_SetUpNextGame) as 1:00 day and 6:30 night games is tentative.
- ObjectTable ($0214, 9 x 20 bytes) = fielders and RunnerTable ($02DC, 4 x 20) = runners
  are inferred from the radar map drawing; field offsets beyond +0..+8/+$10/+$13 unknown.
- Units 2/6 field streamer copies: no caller identified (FieldUnitTable only yields units 0/4
  through FieldUnitIndex values 0/2) - lead L007 remains.
- Sound driver command semantics ($A0-$AF sub-commands, envelope tables) are read from
  the interpreter; exact musical meaning of each parameter is not verified by ear.
- PowerOnInit's A+B check has no effect (both paths reach ModeTransition).
- AwayPitcherCondition ($0417, was HomeMorale) / HomePitcherCondition ($0418, was AwayMorale) were
  renamed in round 1 from their readers: ScoreRun lowers $0417 when the HOME team scores and
  U20_LoadCurrentPitcher applies $0418 in top halves. Likewise $039C/$039D are now
  AwayPitcherRunsAllowed/HomePitcherRunsAllowed. The exact game effect of the condition is tentative.
- U20_PlayerRecordByte3 (unit 20 entry 7) indexes Arg10*8 + Arg11*4 with Arg11 = 2 from unit 22,
  which does not match the 12-byte player records; intent unknown.
- Season thresholds (70 wins pennant, 61 non-wins GAME OVER, best-of-7 series from SeasonGame
  $C8) are read from U20_SeasonProgress / U16_DecodePassword, not observed at runtime.
- U16_ShowPassword compares SeasonWins/SeasonLosses with zero-page $64 (CpuMenuTarget) where an
  immediate #100 was probably intended; as encoded, the hundreds digit depends on that RAM byte.
- Title menu (unit 12) item texts were not decoded from the tile panels; the choices are named
  from their effects (OPTIONS sub-mode 20, season vs exhibition, team select vs ENTER PASSWORD,
  SideMapMode 0/1/2).
- Raw RAM with no reader found keeps mechanical names: Var83, Var85, Var98, VarA1, VarEC,
  Var0336, Var04A5, CannedHitKind, BaseSituation, PaintScrollY/X. (Var0379/Var037A now have a
  reader, but only in the uncalled U28_ConditionRoll; see below.)
- TeamHits ($039E/$039F) and AwayErrors/HomeErrors ($03A0/$03A1) are read as the H and E of the
  scoreboard line from their update conditions only (tentative).

## Round 2 (pass-002)
- Code with no known caller (L010/L012, 25 KiB Static code overall): every such routine says
  "No known caller; reachability unproven". Several look like bypassed or older variants
  (U18_BypassedFadeExit after an unconditional jmp, the unit-20 Alt* lineup drawers, the
  unit-26 FoulWallLowMidHitCopy); that reading is tentative, not a deadness claim.
- U20_FarCallUnit18Entry4 targets unit 18 entry 4, but unit 18's entry table has 4 words; the
  word read would be $00A9 (first bytes of U18_LoadSceneRecord). It cannot work as encoded.
- U12_QueueBlankColumn stores $30 where the VramBuffer terminator (0) belongs; suspected reason
  its only caller (U12_MenuWipeOutStep) is unused.
- U28_JitterVelX/JitterVertSpeed scale by 99/99 (no effect) before adding random noise:
  probably a tuning factor left at 100%.
- U28_ClampPitchToRepertoire: the 8-byte mask table makes pitch type 8 read $AD (first opcode
  of case 0) as its mask; the cases for types 5 and 8 duplicate those of 0 and 6.
- U28_PowerRoll / U28_ConditionRoll / U28_PitchStepFromLimit/FromTable / U28_PickCannedHitByAim*:
  game meanings (home-run chance, tiring check, pitch step) are inferred from the operands only.
- U16_DrawMatchupEmblems directly follows far entry 11 (an rts); it may be that entry's former
  body. U16_LeagueMark0/1 assignment to PlayerLeague bit 0 is from the code; which league is
  which is not established.
- Generic data from L014 (about 1,000 regions, 20 KiB) carries an area note and, when within
  256 bytes, the nearest named table; formats and consumers (pointer-reached records) are not
  decoded. The first bytes of several pointer-anchored regions (U16_GlyphPtrsHead,
  U16_TeamPointerTables, U14_ScheduleHead, U28_CannedHitData) are not decoded either.
- U16_GlyphPtrs ($9B09) and U16_FielderNamePtrs ($A108) start one byte before the phase-1
  region boundary; operands use the sublabel (+1 for the high bytes).
- Var9C is read by the unused unit-20 variants and never written.

## Final editorial acceptance (round 2)
- DPCM samples: the bank-30/31 DmcSample_* regions (and the former Bank31Data_E000, now
  DmcSample_DAC0_Tail) are the 7 distinct samples of DmcSampleTable's 10 records (redirect r2-a
  decode); only the 13 bytes $FD41-$FD4D before Reset stay outside every sample (lead L011).
- Password characters: 0-9 are digits; on-screen letters 10 = R, 11 = D, 12 = B, 14 = C were
  read in the r1-a/r2-a runs. Values 13, 15 and the 16-21 league set were not observed.
- RAM reuse made explicit by scoped operand names: $1A-$1D are math scratch (Arg1A-Arg1D) in
  units 22/26/28, $042F-$0432 (PasswordBytes) are FieldPaintStep/FieldPaintTarget in unit 26
  (saved camera X/Y on entry to U26_InitRestoreFieldView), and $91.. (MenuStep) holds
  SavedBallRecord in U26_PlanBattedBall/RestoreBallRecord and U28_PitchRelease/PitchPredict.
  Which code writes the saved camera position before U26_InitRestoreFieldView is not found.
- Animation tables (U22_ObjectAnimTable/UmpireAnimTable/BatterFieldAnimTable,
  U26_CatcherAnimTable), canned-hit tables (U28_CannedSoftHits/HitsA/HitsB), field-unit
  FieldMetaDefsHigh/FieldAttrMap and the password texts are named at the exact pointer
  targets; the bytes before them in the same generic regions remain undecoded.
- Branch labels: about 1,050 generic @Skip_/@Loop_ labels were renamed. Names are either hand
  names or derived from the code (condition tested, the skipped block's first effect, the
  target's first effect, loop body); 18 mechanical condition names (@IfCarrySet, @IfBelow3 ...)
  remain in arithmetic or field-geometry code whose purpose is not established.
