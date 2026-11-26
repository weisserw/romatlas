# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1 commentary (checkpoint pass-001)

Semantic interpretations below are static readings; none changes the phase-1
classification. Each item names the source location (label) and a bounded check.

- Controls on the title menu. Static reading of TitleMenu ($B4AF): Left/Right change the
  player count (Temp1), Up/Down change the lane (Temp2), Start returns; A/B only bump
  Temp4/Temp3, which are never read. analysis/gameplay.md describes Right/Down differently.
  Check: press Up on the title screen and watch $02 (lead L-009).
- Player 3/4 controllers (old lead L-007): NextPlayer and AnnounceSelectingPlayer give
  bowlers 0/2 controller 1 and bowlers 1/3 controller 2. Not yet confirmed at runtime.
- High score table: HighScores ($C2-$C7, preset 200) and HighScoreNames ($B6-$C1) are
  updated by UpdateHighScores after a game and entry 0 is drawn by DrawTopHighScore at
  $2065 only when the lane screen is built. Round 1 disassembly patched $C2-$C7 without a
  visible change; timing of that patch may explain it. Names are "tentative" in source.
- PlayerCharacter ($88) is the portrait index; character 0 bowls mirrored from the right
  side. Whether that means a left-handed bowler is not established.
- PlayerBallWeight ($8C): digits shown are 7, 9, 11, 13, 15 (BallWeightDigits); units
  (pounds, "PND" on screen) taken from round-1 gameplay notes.
- PowerLevel ($CF) is the power meter value and sets the ball speed via BallSpeedTable;
  HookCurveIndex is set by the spin meter. Labels "spin"/"power" follow the observed
  meter order (two A presses) but the on-screen wording was not checked.
- FrameMarks codes: $A strike, $B spare, $C second-ball miss, $D first-ball miss. Tile
  $43 (first-ball miss) is probably "G" (gutter) or "-"; not checked against CHR.
- StrikeCelebration chooses an animation from StrikeStreak; the meaning of each
  animation (CelebrationWalk/Pair/TilesA/TilesB/Strike) is mechanical naming only.
- Award thresholds 250/275/300 (CheckScoreAward) are exact; the award screens' texts were
  not inspected.
- SpinMeterUnused ($065C-$065F) is written but never read by discovered code.
- ClearScores clears FrameTotals with `sta FrameTotals,y` while counting in X, so it
  rewrites one byte; ScoreFrames overwrites totals as frames are scored, so the effect is
  probably invisible. Runtime check optional.
- Sound: the $80-$EF trap (old lead L-008) is `bcc *` (endless loop). A static parse of
  all header-reachable streams found no such byte in command position, so it is not hit by
  this data. Sound id meanings beyond those requested by the game are not established.
- AttrRowPending/NmiUploadAttrRow, NmiVramRead, NmiFillRect, CHR-RAM upload routines and
  other library code are not exercised by the game (lead L-001); descriptions are of their
  mechanics only.
- Data not decoded beyond labels: sprite records after the frame pointer tables, tile-pair
  lists of the selection/celebration/award screens, compressed nametables, the bowler
  record block $8D5F-$9D73, title sprite data $B59E, UnusedBestScoreData $A993 (contains
  letter-coded "BEST SCORE"), ScorecardData's first 16 bytes. See lead L-010.
- LaneOverviewTiles starts at $A26A, the RTS opcode of DrawLaneOverviewFromRom; the byte
  is both code and the first tile of the table (lead L-011).
