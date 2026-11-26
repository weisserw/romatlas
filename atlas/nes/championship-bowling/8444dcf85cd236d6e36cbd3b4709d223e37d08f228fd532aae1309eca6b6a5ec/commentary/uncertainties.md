# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 6 commentary (checkpoint pass-006)

Earlier list: analysis/round-5/commentary/uncertainties.md (and older round copies). Open
items are carried forward with the round-5 redirect findings (L-022). None changes
classification.

### Resolved since round 5

- $A92B (open-frame first-ball code >= 10 counted as 0) runs after a missed first ball
  (code $D); Observed since pass-006.
- Stepped fades are dormant by data flow: FadeDelay is $FF only inside AnnounceBowler, and
  both of its FadeInSpriteGroups calls take the instant path; BrightenGroups and
  TakeNextFadeGroup have only dormant or unreachable callers; FadeOutAll never runs with
  $FF; BrightenColor's cap is never reached (r5-a1 fadedelay_flow.json, fade_closures.json).
- Sound: the envelope restart, the envelope page carry and the attenuation floor are never
  needed by ROM sound data (r5-a1 clamp_sim*.json). Scope: the stream grammar as implemented.
- Page carries at $AAF7, $AB07 and $BDEB are impossible by address arithmetic.
- DrawObject's $FD list jump is not used by any sprite list in ROM (scope: ROM lists; no
  RAM-built list is known).

### Resolved at final editorial acceptance

- $FC23 (PlaceSprite's out-of-range Y path) is dormant by data flow (L-023, r6-a1
  fc23_bounds.json): the path runs whenever Y + record offset leaves 0..255 at either
  edge, and every object's Y + offset stays within 9..232. Object Y high bytes come only
  from ROM templates ($00) and the Reset clear, and every frame table comes from ROM, so
  no sprite list is built in RAM (this also closes the $FD scope note above).
- Sprite record lists (L-010): kept as one labelled .incbin per frame by editorial
  decision; the record grammar is documented once at DrawObject and each group is
  labelled at its frame starts. Per-record expansion would add raw numbers without
  new meaning.

### Still open

- PlayerCharacter ($88): character 0 bowls mirrored from the right side; whether that
  means a left-handed bowler is not established.
- PlayerBallWeight ($8C): digits 7-15 (BallWeightDigits); "pounds" comes from the "PND"
  text in round-1 gameplay notes.
- Celebration animation names describe mechanics only.
- Selection-screen objects A-D: which on-screen element each template draws was not
  checked. Object A's frame runs on into object D's records (33 sprites).
- Portrait pose B (PortraitTilePtrs) is probably a blink/expression change.
- UnrefTilePairs_CDE7 ($CDE7-$CE0C), ScorecardData's first 16 bytes and
  UnusedBestScoreData stay unexplained.
- ClearScores `sta FrameTotals,y` while counting in X (probably a ,y/,x slip; no
  visible effect).
- A B press on the final scorecard held 4 frames was once not accepted (not investigated).
- Sound id meanings beyond those requested by the game are not established;
  SndCmdTransposeNote / SndCmdAddAttenuation look like older $F5/$F6 handlers (tentative).
- Purpose of the unreachable routines and islands, and of the dormant NMI jobs, is
  inferred from their mechanics only (for example FinalAwardsTest, DebugActorFrameViewer).
- Compressed nametables, tile blocks and other formatted data remain binary (.incbin)
  with labels and format notes; their internal rows are not decoded.
