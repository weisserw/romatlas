# Commentary uncertainties (round 4, pass-004-fallback; final editorial update)

## Pass-004-fallback update (map rebase)
- The only map change since pass-003: the 2 bytes 28:$8620 (phys 230960, region 1252) and 28:$9883
  (phys 235667, region 1253) are now generic **Data by user-directed manual fallback** (provenance
  Manual, confidence 0.5, no label). They are no longer Unknown, but this is an administrative
  fallback, not a finding (analysis/handoffs/manual-fallback.md).
- Still unresolved for both bytes: meaning, and whether they are code or data. Each is a lone $60
  (would decode as RTS) right after another RTS; no static edge, reader, pointer, stack-dispatch target
  or observed fetch reaches either one. L-015 stays rejected as an evidence claim. The source notes
  say exactly this and give no label.
- Classification: 393230 bytes evidence-backed + 2 bytes manual user-directed fallback (0 unknown).
  This is not fully evidence-backed classification.
- Everything below is carried unchanged from pass-003 except where it mentioned the 2 bytes as
  "unknown"; those lines now refer to this update.


## Final editorial update (redirect r003-a folded in)
- Resolved and now stated in source: scoreboard figures (BatterHomeRuns $0356 "HR", BatterAverage $0357
  "AV.xyz", PitcherEra $0363 "ERA d.dx", PitchSpeedMph $03DA). Object 2 is the catcher. Batter group 2
  is a hit by pitch, and PitchOutcome 2 is a bench-clearing brawl scene (L-016). PlayResult 1-4 is a home
  run over wall section 1-4, and wall id 5 is the foul-side walls. The lineup slot model is
  documented at VisitorLineup. The InitRamTables blocks are the EDIT team's player records, starting
  order and pitcher records (L-018). StartingOrderPtrs ($ADC4) entries are 10 (player, position) pairs;
  teams 6-11 use a DH and teams 0-5 do not. Pitcher record byte 2 is stamina and byte 3 is ERA x 10.
  CpuFlags bit 0 requests a relief pitcher and bit 1 a pinch hitter.
- Still open:
  - Object 10 as the umpire is tentative; it has not been seen on screen.
  - Which outfield section is wall 2/3/4 comes from the static equations only.
  - The $A9-$AB label tiles probably read "MPH".
  - Team 7's last starting-order pair (05, 4) is unexplained.
  - Safe/HitsAllowed counters, clock $03E5/$03E6, BootABFlag and the write-only bytes are unchanged.
- The 2 bytes 28:$8620 and 28:$9883: see the pass-004-fallback update above (manual fallback Data, role unproven).
- The round-3 items below are kept as history. Where they say "tentative" or "open" for an item
  listed above as resolved, the update above takes precedence.


Basis: round-2 notes (analysis/round-2/commentary/uncertainties.md, carried in full where still
valid), plus round-3 static reading of every former Var_xxxx RAM user, every former Tbl_ table
consumer, the 41 data blocks without an absolute-operand consumer, and the shallow helper
routines in banks 22/23/26/27/28. No runtime work this round.

## Established mechanics (high confidence; new in round 3 marked +)
- Round-1/2 items still hold (boot/NMI/IRQ, FarCall, upload buffer, controllers, camera,
  metasprites, DMC, GameMode dispatch, object table, menu submodes, Level/RatingTotal, walls,
  fielder actions, bank 0/4/6 copies, InitRamTables survival over warm boot).
- + 23:$B402 is now code (pass-003, L-017): a statically dead RTS after the always-taken
  'bcc SetLevel0'; named NextLevelFrom4_DeadRts.
- + Base coordinates (RunnerBasePoints/CoverBasePoints/ThrowBasePoints): home ($2500,$2000),
  first ($2E00,$1700), second ($3700,$2000), third ($2E00,$2900). Field X runs from home toward
  second base; Y is lateral (first base low Y, third base high Y). FielderControlInput maps pad
  Left = first, Right = third, Down = second, Up = home. This partly answers the round-2 axis question.
- + Runner position on the path: progress/4 offset from the base point of the current leg
  (RunnerPosLegA-D); one leg = 9 position units ($24 progress high byte).
- + FielderOnBase/FielderBaseMask ($0350/$0351): bases touched by the running and the
  auto-moving fielders each frame; used for force outs (RunnerForceAt*) and CPU throws.
- + Rating counters: RatingCounters $03EF-$03FA bumped by CrowdCheerTop/Bottom (index X,
  cap 14); RatingWeights ($B635) gives weight 3 (5 for $03F9) to the 11 penalty counters and 2
  to the bonus counter $03FA; RatingValueLayout ($B5FD) places them on the rating screen.
- + Player records: 25 x 12-byte player records per team (PlayerRecordPtrs; byte 1 speed,
  bytes 6-10 name tiles), 8 x 8-byte pitcher records per team (PitcherRecordPtrs).
- + Pitch pre-simulation (PitchStateFlight) and the strike zone (PitchInStrikeZone: BallXHi < $68,
  BallZHi $0E-$1D, BallYHi $74-$83 sets PitchFlags bit2).
- + Sound driver work bytes $06E9-$06EF and its tables (note periods, channel bits/types,
  vibrato and fade tables).

## Tentative interpretations
- Round-2 tentative items still open: runner state step semantics, lineup slot model and
  pitcher-fatigue reading, wall ids 1-5, pitch type names, result-scoreboard reactions,
  InitRamTables 20/64-byte blocks, CPU fielder speed tables, clock $03E5/$03E6, BootABFlag.
- + Scoreboard figures: BatterPowerStat ($0356, probably home runs), BatterAvgStat ($0357,
  probably batting average .xyz with a derived third digit), PitcherShownStat ($0363, probably
  ERA d.dx, replaced by $53/$54 when the pitcher tires), PitchSpeedShown ($03DA, speed readout;
  computed as 1.5 x |BallVelX| + 0-7). Display formats are exact; the baseball meanings are not.
- + Object 10 is probably the umpire (four stations behind home/first/second/third chosen by the
  camera zone; out call via state $50). Object 2 in the pitch view is probably the catcher
  (round 1/2 called it the pitch marker). The batter-box object of the field view is probably
  the batter figure.
- + VisitorSafeCount/HomeSafeCount ($038F/$03A0) and VisitorHitsAllowed/HomeHitsAllowed
  ($0390/$03A1): exact increment conditions documented; meanings tentative.
- + L-016: batter state group 2 is entered only when the pitch touches a non-swinging batter
  (BatContact, 1 frame in 8); normal outcome PitchOutcome 1 (runners advance), late close-game
  outcome PitchOutcome 2 (team banner screen). Tentatively hit-by-pitch; the banner scene is
  not runtime-confirmed. The round-1 '@repitch' label is now '@teamBanner'.
- + Write-only bytes kept with neutral names: BatterLatch0335, SndInitByte, SavedCameraX/Y,
  EndingWorkA5-A7.

## Concrete open questions (for redirect)
Pass-003 items 2-6 (wall ids, lineup model, outcome-2 scene / hit by pitch, InitRamTables blocks,
scoreboard figures) were resolved by redirect r003-a (see the final editorial update above). Still open:
1. 28:$8620 and 28:$9883: code-vs-data role and meaning (now manual fallback Data; L-015 rejection
   stands). No evidence-producing check is known; policy only.
2. Object 10 as the umpire (tentative; not seen on screen).
3. Left/right assignment of wall sections 2-4 (static line equations only).
4. Team 7's last starting-order pair (05, 4); Safe/HitsAllowed counters; clock $03E5/$03E6;
   BootABFlag; write-only bytes.
