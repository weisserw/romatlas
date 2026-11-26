# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 5 commentary (checkpoint pass-005)

Earlier lists: analysis/round-4/commentary/uncertainties.md (round 4) and the round-3 /
round-2 / round-1 copies. Open items are carried forward and revised with the round-4
redirect findings (L-020). None changes classification.

### Resolved since round 4

- CONTROL label: the arc gauge with the spin marker is labelled CONTROL on screen and the
  vertical bar POWER (r4-a1 E18 screenshots). RunSpinMeter says so without a hedge.
- Sign number: the sign reads "nPLAY"; SignGirlTiles' first tile is the bowler digit
  ($84/$94/$95/$96 = "1"-"4" in CHR bank 2, sprite half $0000; e18_sign2.png,
  e18_sign_tiles.txt).
- Left gutter: UpdateLaneBall's $C0 clamp ($8A2D) is reached in natural play (full-left aim
  stops at X $C4, then a left-drifting hook; E17). The old "left aim clamped above $C0"
  premise was wrong; AimAndWaitForThrow now gives the aim range as $C4-$E4.
- Gated NMI jobs (AttrRowPending, VramReadMode, FillPending, WaitSecondCount, SpriteCycleMode
  1-$EF, StripMode column mode) are dormant by data flow: no reachable direct or
  indexed/indirect store opens the gates (r3-a1 gate_writers.json, r4-a1 indexed_stores.json).
  Still code. Documented in the header and at each job.
- FillNametable2/3 ($F656) are unreachable: no reference, and FillNametable1's bne is always
  taken (r4-a1 residual_preds.json). Still code.

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
- Sprite records, compressed nametables and the bowler record block remain binary
  (.incbin) with labels and format notes; individual records are not decoded (L-010).
