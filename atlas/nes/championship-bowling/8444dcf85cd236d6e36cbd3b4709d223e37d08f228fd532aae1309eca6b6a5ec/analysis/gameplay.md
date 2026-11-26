# Gameplay research

## Sources

No external sources consulted in round 1 (local exploration was sufficient for the pass; no
manual/FAQ fetched). Everything below is locally confirmed on this ROM unless marked otherwise.

## Identity (local)

- Title screen: "CHAMPIONSHIP BOWLING", (c)1989 ROMSTAR, INC., licensed by Nintendo of America.
- Developer strings in PRG: "Another Ltd." with build date "1989/2/27 New バージョン" ($F208, $FF0A),
  katakana "ボーリングゲーム" signature at $8000. NES CNROM (mapper 3), 32 KiB PRG + 32 KiB CHR.

## Controls and flow (confirmed)

- Title: intro ignores input for a while (from snap_title presses within the first ~370 frames were
  ignored in round-1 redirect E3). Then (corrected in round-1 redirect, E3, matches TitleMenu static
  reading): Right/Left = HOW MANY PLAYERS +1/-1 with wrap (1-4; Temp1 $01 = players-1, copied to
  NumPlayersMinus1 $87), Up/Down = LANE NUMBER +1/-1 with wrap (Temp2 $02 = lane-1, copied to
  LaneNumber $90). The earlier "Down moves to LANE NUMBER, Right increments it" note was a
  misreading. A/B only bump unused counters. Start goes to selection.
- Selection ("MENU n PLAYER SELECTION"): 4 bowler portraits each with 3-letter NAME and BALL weight
  (PND). A enters/advances letters, Up/Down change values, A confirms. Player 2 is controlled from
  controller port 2 (port 1 input is ignored for P2). P3/P4 contract unresolved (lead L-007).
- Lane: A starts the approach / sets power (oscillating POWER meter) / sets control; ball rolls,
  pins fall, pinsetter animation, frame box updated. Left/Right before the throw were tried but the
  aim/timing contract is not understood (strike not obtained).
- Frame number in RAM $92+player (PlayerFrame, 1..10). After the game a full scorecard overlay is
  shown; it is left with **B** (ShowScorecard polls Left/Right for the page and B to close; Start/A
  are ignored). B during aiming also opens the scorecard. Then the results screen "HI SCORE / A PRIZE
  WINNER!" (trophies 300/275/250 PTS, ranks 1-3 with names) runs once per bowler; any button returns
  to the title (round-1 redirect E1, natural).
- 2P: lane intro shows a girl holding a "1PLAY" sign; players alternate.

## Round 1 redirect findings (local, this ROM)

- Natural strike recipe (lane 1, character 0, from analysis/snapshots/snap_lane1.json): hold Left
  60 frames (LaneBallX $065B = 208), A; press A when HookCurveIndex $D0 reads 6 (released value 7);
  press A when PowerLevel $CF reads 16. Gave strikes in frames 1-3 (StrikeStreak 1-3), and the same
  throw on a 9-pin leave made a spare. Hold Left 28 frames (LaneBallX 218), hook 4, power 16 gives a
  stable 9. Meter presses must be held >= 6 frames (the meters poll every 4 frames).
- 3P/4P: works naturally. Bowlers 1 and 3 select/bowl on controller 1, bowlers 2 and 4 on controller 2;
  each bowler presses A (portrait; Right/Down move the cursor), Up/Down + A three times (name), Up/Down
  + A (ball weight). Several bowlers may pick the same portrait.
- Hi-score entry and awards (ARTIFICIAL totals): at the final scorecard set FrameTotals word 10 of a
  bowler ($0340 for bowler 0, +22 per bowler) to >= 200 and press B. Results screen inserts the score,
  plays sound 9 and reveals the name letter by letter. Totals >= 300/275/250 run the award scenes:
  300 = convertible car "CONGRATULATIONS! <name>", 275 = "SPORTS NEWS"/"BCD TV" studio, 250 = beach.
  Evidence: analysis/redirect/r1-a1/evidence/e2_awards.png.

## Round 2 disassembly findings (local, this ROM; pass-002)

- Natural perfect game: per-ball recipes found by retrying each ball from an in-session snapshot
  (natural inputs only). Kept list (Left-hold frames / hook value pressed at / power): 60/6/16 x3,
  60/5/16, 60/6/16, 60/5/16, 60/5/16, 64/4/16, 60/5/16, then 10th frame 60/6/16, 60/7/16, 60/4/16.
  Replays are deterministic; outcomes for one recipe differ between frames (RNG/state), so the per-ball
  list matters. Results then show the 300 award (car) before HI SCORE/A PRIZE WINNER (S16).
- Award thresholds confirmed naturally on the final total: 300 -> car, 299 -> SPORTS NEWS/BCD TV (275
  scene), 267 -> beach (250 scene) (S16-S18).
- The final ball of the game leaves PinsHitThisThrow ($0385) nonzero; detect the game end by
  FramesScored ($96+) >= 10 instead. A B press 300 frames after the last ball (4-frame hold) was
  not accepted; an 8-frame hold later worked.
- Title animation: with no input after power-on the bowler walks in and throws (TitleSkipped $04AB = 0);
  title inputs during the animation are ignored (S19).
- B during aiming opens the scorecard (page 1 before frame 7); Right/Left switch pages; B closes (S20).
- 2P game end: per-bowler 10th-frame announcement, two-row scorecard, results (S21).

## Routes

See analysis/scenarios.json (S1-S10) and snapshots in analysis/snapshots/.

## Open questions

Resolved in round-1 redirect: leaving the scorecard (B), strike/spare (natural recipe), 3P/4P
selection, hi-score entry and award screens (artificial totals). Still open: a natural >=200 game
(possible with the strike recipe but long), alternate lanes' hook behaviour, celebration variety.
Round 2: natural >= 200, 250/275/300 awards, lane 4 and 2P game end now visited; still open: lane 5,
hi-score insertion at ranks 2/3, CelebrationTilesA variant (lead L-014).

## Round 2 redirect findings (local, this ROM; attempt analysis/redirect/r2-a1)

- Title input from snapshots/snap_title.json is accepted about 420 frames after import (poll with a
  press until $01/$02 changes). Lane wraps 5 -> 1 on Up and 1 -> 5 on Down; players wrap 4 -> 1 on
  Right (S25).
- 3P game with all bowlers on portrait 1 (character 0) on lane 1: the lane-1 strike recipe (Left 60,
  spin at 5-7, power 16) gave strikes for almost every ball; per-ball retries from an in-session
  snapshot pick among natural inputs. Final totals 300 / 278 / 257 gave the car, SPORTS NEWS and beach
  scenes in one results sequence and inserted the scores at ranks 1, 2 and 3 (ties insert above, so
  each later bowler must score strictly less than the one before) (S23).
- Lanes 2, 4, 5: a throw with the spin meter released at index 3 runs the lane-specific
  AdjustHookForLane branch (8 pins each with Left 60 / power 16) (S24).
- Still not targeted: CelebrationTilesA (random choice among celebrations), pause (CheckPause has no
  caller). The game has no levels or bosses; its terminal screens (results, three award scenes) are
  all visited naturally.

## Round 3 redirect findings (local, this ROM; analysis/redirect/r3-a1/)

- RNG: Random ($F88D) is the only random source. RandomSeed ($05E5) is zeroed at power-on only
  and advances only inside Random, which is called in four places: once per throw when the
  bowler's ball weight differs from the character's preferred weight (PreferredBallWeight =
  2, 3, 1, 0 for characters 0-3; the default weight 0 mismatches for characters 0-2), once per
  throw at StrikeStreak >= 3, and at the double and turkey celebrations. Random outcomes in a
  power-on session therefore depend only on how many of these calls happened, not on timing.
  The offline model (tools/rng.py) matched the emulator after every throw.
- Double-strike variants: Random(10) >= 6 gives CelebrationTilesA (two cheering girls), else
  CelebrationTilesB. From power-on with character 0 and weight 0, two non-strike balls followed by
  two strikes gives TilesA (S27). With the usual strike-strike opening it is always TilesB.
- 10th-frame marks: X 9 /, 7 / X and 7 / - reached naturally (S28). On the scorecard "-" (tile $42)
  marks a miss and tile $43 is a "G" (first-ball miss in frames 1-9).
- Power pressed below 8 plays the slow rolling sound (S29).
- Pause cannot be reached: CheckPause has no path from the reset/NMI vectors (S30).
