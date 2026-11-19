# Round 3 — redirect phase handoff (attempt r003-a)

Status: complete. Input checkpoint pass-003 (byte_identical), classification 100.00% (393230/393232).
ROM, map, state, source and configured binary unchanged. Research: analysis/redirect/r003-a/ (notes.md,
tools/s1-s8.py, evidence/*.png, snapshots/field1.snap.gz). Scenarios S-013..S-015 added; leads L-016
updated (verified), L-018 added (commentary-only), pass record redirect-r003-a appended.

## Findings for commentary / final acceptance
- Scoreboard (confirmed, ordinary state): BatterPowerStat $0356 = home runs ("HR04"), BatterAvgStat $0357
  = batting average ("AV.296", third digit derived), PitcherShownStat $0363 = ERA ("ERA 1.43"),
  PitchSpeedShown $03DA = pitch speed in MPH ("73MPH").
- Object 2 in the pitch view = catcher (confirmed by position patch). The pitch-view umpire is not object 2.
- Object 10 = umpire: still unconfirmed (stations re-forced each frame; no drawn sprite seen in sampled
  field views). Keep tentative.
- L-016 (confirmed, artificial): batter group 2 = hit by pitch ("HIT BATSMAN"); outcome 1 = batter to first;
  outcome 2 "team banner" screen = bench-clearing brawl scene, then field play. Rename @teamBanner/
  UpdateTeamBanner wording accordingly (e.g. BrawlScene).
- PlayResult 1-4 = home run (result picture "HOMERUN" with 4 stands pictures); WallHit wall ids 1-4 = which
  wall the ball cleared (picture index), 5 = foul-side walls. Left/right assignment of 2/3/4 static only.
- Lineup slot model and InitRamTables 20/64-byte blocks resolved (L-018): EDIT team starting order and
  pitcher records; $ADC4 is a per-team starting-order pointer table (RosterRowPtrs/RosterRowAddr misnamed);
  TeamRosterPtrs/PlayerRecordPtrs and TeamInfoPtrs/PitcherRecordPtrs are duplicate equates.

## Classification leads
None ready. No finding touches the 2 unknown bytes.

## Rejection review (2 unknown bytes)
28:$8620 (after rts $861F) and 28:$9883 (after rts $9882, skipped by `bne +2`): reused the r002-a/r002-b
review (analysis/handoffs/round-002-redirect.md, round-002-redirect-b.md), assessed sound by the
coordinator. Premises unchanged: no static edge, reader, pointer or fetch; competing code/data readings
both lack a consumer; policy's "unreferenced return opcode" case applies. No new inference this round.

## Deferred avenues
- Object 10 identity: needs a field view framing a station (force-out call, state $50); repeated sampling
  showed nothing; readability-only, diminishing prospect.
- Natural home run / wall 2-4 side mapping: random batting too slow under trace (s5 killed at 1200 s);
  a ball-flight RAM injection would only restate the static equations. Readability-only.
- Clock $03E5/$03E6, BootABFlag, write-only bytes, Safe/HitsAllowed counters: no cheap discriminating test.

## Recommendation
terminal incomplete -> final acceptance (fold the confirmed meanings above and L-018 into the final
editorial pass; 2 bytes remain unknown under L-015).
