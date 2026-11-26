# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 2 commentary (checkpoint pass-002)

Round 1 entries (pass-001) are kept in analysis/round-1/commentary/uncertainties.md.
Below they are carried forward and revised with the runtime findings of the round-1
redirect and round-2 disassembly (leads L-013, L-015). None changes classification.

### Resolved since round 1

- Title-menu controls (L-009): Left/Right = players, Up/Down = lane, both wrap; confirmed
  at runtime. TitleMenu comment updated.
- Player 3/4 controllers (L-007): bowlers 0/2 controller 1, bowlers 1/3 controller 2;
  confirmed in a natural 4-player game. NextPlayer comment updated.
- High-score table: HighScores ($C2-$C7) / HighScoreNames ($B6-$C1) confirmed; it is
  updated on the results screen after B closes the final scorecard (that is why the old
  RAM patch at the scorecard showed nothing). "Tentative" removed from the equates.
- Award scenes: thresholds 300/275/250 confirmed by natural 300/299/267 games. Scenes:
  300 convertible car "CONGRATULATIONS! <name>", 275 "SPORTS NEWS" / "BCD TV" studio,
  250 beach. Routine comments updated.
- Scorecard: B during aiming opens it; it starts on frames 1-6 before frame 7; Left/Right
  switch pages and B closes; Start/A are not polled.
- SpinMeterUnused (L-013) was wrong: $065C-$065F is OAM sprite 23, the spin-meter marker
  (now SpinMarkerY/Tile/Attr/X; positions from SpinMeterArc). $0658-$065B (LaneBall*) is
  sprite 22.
- $A26A shared RTS / LaneOverviewTiles byte: deliberate (L-011); noted in the map and source.

### Still open

- PlayerCharacter ($88): character 0 bowls mirrored from the right side; whether that
  means a left-handed bowler is not established.
- PlayerBallWeight ($8C): digits 7-15 (BallWeightDigits); "pounds" comes from the "PND"
  text in round-1 gameplay notes.
- Meter names: "spin" (HookCurveIndex, first meter, marker on an arc) and "power"
  (PowerLevel) follow the code; the CONTROL label for the spin meter comes from redirect
  notes and was not checked against the screen data.
- FrameMarks codes: $A strike, $B spare, $C second-ball miss, $D first-ball miss. Tile $43
  (first-ball miss) is probably "G" or "-"; not checked against CHR.
- Celebration animation names (CelebrationWalk/Pair/TilesA/TilesB/Strike) describe
  mechanics only. CelebrationTilesA has not been seen in natural play (L-014).
- High-score insertion at ranks 2/3 and lane-5 hook branches ran only in a RAM-patched
  test or not at all (L-014); the code is commented from static reading.
- ClearScores clears FrameTotals with `sta FrameTotals,y` while counting in X, so it
  rewrites one byte; ScoreFrames overwrites totals as frames are scored, so the effect is
  probably invisible.
- A B press on the final scorecard held 4 frames was once not accepted while an 8-frame
  hold worked (round 2 runs). Possibly the press fell between polls or during a wait;
  not investigated (low value).
- Sound: the $80-$EF trap (L-008) is `bcc *`; no reachable stream contains such a byte.
  Sound id meanings beyond those requested by the game are not established.
- Uncalled library code (L-001) and unreferenced islands (L-002): DebugActorFrameViewer,
  DrawBlock9FFB/A045, SetFrameAreaAttrs, KnockDeckPinSideways, SetBowlerColors,
  FinalAwardsTest, DrawRawScreenNt1, CHR-RAM uploaders, NmiVramRead/NmiFillRect etc. are
  described by their mechanics; no selector exists.
- Data not decoded beyond labels (L-003/L-004/L-010): sprite records after the frame
  pointer tables, tile-pair lists of selection/celebration/award screens, compressed
  nametables, the bowler record block $8D5F-$9D73, title sprite data $B59E,
  UnusedBestScoreData $A993, ScorecardData's first 16 bytes.
