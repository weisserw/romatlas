# Commentary uncertainties (round 1)

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
  Var0336, Var0379/Var037A, Var04A5, CannedHitKind, BaseSituation, PaintScrollY/X.
- TeamHits ($039E/$039F) and AwayErrors/HomeErrors ($03A0/$03A1) are read as the H and E of the
  scoreboard line from their update conditions only (tentative).
