# r003-a — Round 3 redirect (input checkpoint pass-003)

Scope: bounded runtime/static checks of the commentary handoff's tentative meanings
(analysis/handoffs/round-003-commentary.md, commentary/uncertainties.md). No ROM/map/state change.
Server: `python3 run.py serve` via tools/daemon.py (socket /tmp/ra67s3, removed). Snapshots used:
traces/round-001/snapshots/{p1mid,game0}.snap.gz, analysis/redirect/r001-a/snapshots/gnext.snap.gz,
new analysis/redirect/r003-a/snapshots/field1.snap.gz (decompress to /tmp/ra67s3/*.snap for the scripts).

## Results
1. Scoreboard figures, CONFIRMED (ordinary state, p1mid, s1.py/s2.py): $0356=4 -> "HR04" (home runs);
   $0357=29 -> "AV.296" (batting average); $0363=14 -> "ERA 1.43"; $03DA=73 -> "73MPH" (pitch speed, mph).
   evidence/scoreboard_pitchview.png, evidence/pitch_speed_mph.png.
2. Object 2 = catcher, CONFIRMED (artificial position patch $022C=$40/$022E=$60, s4.py): the red catcher
   figure moves; the dark umpire figure behind home in the pitch view does not (not in ObjTable 0-14 there).
   evidence/obj2_catcher_moved.png.
3. Object 10 = umpire: NOT CONFIRMED. Position is re-forced to a station every frame (patching X/Y is undone);
   A/B test (state cleared vs kept, s6.py) over ~100 sampled field-view frames found no sprite difference
   (only a 1-pixel and a scroll-split artefact). Stations ($22,$20) behind home and ($3B,$20), ($2B,$2B)
   seen. Deferred: needs a camera view on a station (e.g. a force out at a base, state $50); low value.
4. L-016 batter group 2 = HIT BY PITCH, CONFIRMED (artificial, s7.py): message "HIT BATSMAN";
   late close variant (HalfInning 10, 0-0, BatterPowerStat 10) runs $22 Pose -> $23 Leave -> GameMode 2
   submode 2 = a bench-clearing brawl picture (the "team banner" screen) -> field play -> next batter.
   Control (HalfInning 2): $21 Hold -> runner on first. evidence/l016_*.png.
5. Wall ids / PlayResult 1-4 (artificial, s8.py): writing PlayResult 1..4 in field play leads to
   InitResultScoreboard: stands picture scrolling to a "HOMERUN" scoreboard; 1-4 pick different stands
   pictures. Hence a ball over wall N (WallHit carry: height >= $400 -> WallPlayResult = N) is a home run
   shown with picture N; wall 5 = foul-side walls (PlayResult >= 5 foul). Which outfield section is
   2/3/4 rests on the static line equations (2/3 are the low-Y = first-base/right-field side), not on a
   natural home run (none produced in ~4,600 frames of random swings with power 60; s5.py killed by its
   time limit, trace overhead). evidence/playresult*.png.
6. Lineup model, CONFIRMED (RAM reads in 3 snapshots + ROM tables): VisitorLineup/HomeLineup slots 0-8 =
   batting order (player index: 0-7 pitchers, 8-24 fielders), slot 9 = non-batting pitcher on DH teams or
   $FE, $0A-$12 bench fielders, $13-$1A pitcher numbers 0-7 with $FF marking the pitcher in the game.
7. InitRamTables blocks, RESOLVED STATICALLY: bank 21 tables $ADAA (players), $ADC4 (20-byte starting
   order records, misnamed RosterRowPtrs) and $ADDE (pitchers; duplicate equate TeamInfoPtrs) each have 13
   entries; entry 12 = $04C0 / $05EC / $0600 -> the EDIT team. Record = 10 (player, position) pairs,
   position 0 pitcher, 1-8 fielders, 9 DH; teams 0-5 no DH ($FE/9 tail), 6-11 DH. Team 7's last pair 05/4
   is anomalous (unexplained). -> L-018 (commentary-only).

## Unknown bytes
28:$8620, 28:$9883: no new evidence or inference; r002-a/r002-b rejection review (coordinator-assessed sound)
reused unchanged. Not retried.
