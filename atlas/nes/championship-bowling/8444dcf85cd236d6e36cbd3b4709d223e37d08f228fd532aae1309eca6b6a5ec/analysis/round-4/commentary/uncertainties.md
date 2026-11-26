# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 4 commentary (checkpoint pass-004)

Earlier lists: analysis/round-3/commentary/uncertainties.md (round 3), and the round-2 /
round-1 copies under analysis/round-2/ and analysis/round-1/. Open items are carried
forward and revised with the round-3 redirect findings (L-018). None changes classification.

### Resolved since round 3

- Tile $43 is "G" (first ball with no pins, frames 1-9) and $42 is "-" (miss); $44 is the
  spare box and $45 the strike box (CHR banks 1/2, $1000 half; r3-a1 scorecard_mark_tiles.json).
  The source now uses TILE_GUTTER / TILE_MISS / TILE_SPARE / TILE_STRIKE.
- CelebrationTilesA ran in natural play (r3-a1 E14). The celebration variant depends only
  on the Random call history since power-on (RandomSeed is zeroed only by Reset; four call
  sites). Documented at Random, AdjustHookForLane and StrikeCelebration.
- 10th-frame third-ball encoding: $B0 + n is a spare of the pins left after "X n"
  (E15: "X 9 /" stored as $A9 $B1). The ScoreBall question mark is removed.
- Uncalled library code (L-001) and code islands (L-002) are unreachable from the vectors
  (closed-world census of computed transfers, r3-a1 closure.json). They are still code.
  The source says "unreachable", not "no known caller". Consequences: no pause in play
  (CheckPause); the NMI jobs gated by AttrRowPending / VramReadMode / FillPending stay idle.
- The Static `jmp @countBall` at $A84E (region 512) is a defensive path: a third ball after
  an open 10th frame cannot happen.

### Still open

- PlayerCharacter ($88): character 0 bowls mirrored from the right side; whether that
  means a left-handed bowler is not established.
- PlayerBallWeight ($8C): digits 7-15 (BallWeightDigits); "pounds" comes from the "PND"
  text in round-1 gameplay notes.
- Meter names: the CONTROL label for the spin meter comes from redirect notes and was
  not checked against the screen data.
- Celebration animation names describe mechanics only.
- Selection-screen objects A-D: which on-screen element each template draws was not
  checked. Object A's frame runs on into object D's records (33 sprites).
- Portrait pose B (PortraitTilePtrs) is probably a blink/expression change.
- SignGirlTiles: the per-bowler first tile ($84/$94/$95/$96) is probably the sign
  number; not checked against CHR.
- UnrefTilePairs_CDE7 ($CDE7-$CE0C), ScorecardData's first 16 bytes and
  UnusedBestScoreData stay unexplained.
- ClearScores `sta FrameTotals,y` while counting in X (probably a ,y/,x slip; no
  visible effect).
- A B press on the final scorecard held 4 frames was once not accepted (not investigated).
- Sound id meanings beyond those requested by the game are not established;
  SndCmdTransposeNote / SndCmdAddAttenuation look like older $F5/$F6 handlers (tentative).
- Purpose of the unreachable routines and islands is inferred from their mechanics only
  (for example FinalAwardsTest, DebugActorFrameViewer).
- Sprite records, compressed nametables and the bowler record block remain binary
  (.incbin) with labels and format notes; individual records are not decoded (L-010).
