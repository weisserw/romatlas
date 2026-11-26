# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 3 commentary (checkpoint pass-003)

Earlier lists: analysis/round-2/commentary/uncertainties.md (round 2) and
analysis/round-1/commentary/uncertainties.md (round 1). Open items below are carried
forward and revised with the round-2 redirect / round-3 disassembly findings (L-014,
L-016, L-017). None changes classification.

### Resolved since round 2

- Raw table addresses in code (L-017): the hook, ball-weight, award portrait and sign
  tile-list pointer setups now use HookCurvePtrs, BallWeightDigits,
  Award275PortraitLists, Award250PortraitLists and SignGirlTileLists; PerfectFramePtrs
  uses PerfectCharFrames0-3; the sound handler return push uses SoundCmdDispatch-1.
- The list tables behind them are split and documented (2 list pointers per
  character/bowler, Y = 0 / 2 alternate poses; 26 pairs / 22 pairs / 20 tiles).
- High-score ranks 2/3 and the lane 4/5 hook branches ran in natural play (round-2
  redirect, L-014); the source no longer calls them patched/unobserved.
- Sound data: unreferenced header UnrefSoundHeader_EFC7 and three unreferenced stream
  fragments are labelled; envelopes $1C-$28 and duration indices outside 1-27 are never
  selected by a reachable stream.
- Every frame-0 entry of the object frame tables is a single $FE (all sprites hidden).

### Still open

- PlayerCharacter ($88): character 0 bowls mirrored from the right side; whether that
  means a left-handed bowler is not established.
- PlayerBallWeight ($8C): digits 7-15 (BallWeightDigits); "pounds" comes from the "PND"
  text in round-1 gameplay notes.
- Meter names: the CONTROL label for the spin meter comes from redirect notes and was
  not checked against the screen data.
- FrameMarks tile $43 (first-ball miss) is probably "G" or "-"; not checked against CHR.
- Celebration animation names describe mechanics only; CelebrationTilesA has not been
  seen in natural play.
- Selection-screen objects A-D: which on-screen element (cursor, name marker, ball,
  bowler) each template draws was not checked. Object A's frame runs on into object D's
  records (33 sprites); this looks deliberate but was not observed.
- Portrait pose B (PortraitTilePtrs) is probably a blink/expression change.
- SignGirlTiles: the per-bowler first tile ($84/$94/$95/$96) is probably the sign
  number; not checked against CHR.
- UnrefTilePairs_CDE7 ($CDE7-$CE0C): 19 (tile, $02) pairs, no reference; resembles the
  sign-girl tiles. ScorecardData's first 16 bytes and UnusedBestScoreData also stay
  unexplained.
- ClearScores `sta FrameTotals,y` while counting in X (probably a ,y/,x slip; no
  visible effect).
- A B press on the final scorecard held 4 frames was once not accepted (not investigated).
- Sound id meanings beyond those requested by the game are not established.
- Uncalled library code (L-001) and unreferenced islands (L-002) are described by their
  mechanics; no caller exists (round-2 redirect rejection review).
- Sprite records, compressed nametables and the bowler record block remain binary
  (.incbin) with labels and format notes; individual records are not decoded (L-010).
