# Redirect attempt r1-a1 (round 1)

Input checkpoint: pass-001 (maps/current.json = analysis/state.json = maps/pass-001.json,
sha256 f73ff2c8...c35c3a), commentary round 1 (commentary/source/game.asm 2a0fa2e1...).
Map, state, source and ROM were not modified. Binary: /root/romatlas/target/release/romatlas.

## Layout

- tools/: rpcd.py (socket bridge to `run.py serve`, copied from pass-001 with a broken-pipe fix,
  socket /tmp/r1a1.sock), rpc.py, lib.py (input/snapshot/screenshot helpers; appends every input
  to evidence/inputs.log), cov.py (analysis.coverage dump), newcov.py (lists Static Code regions
  hit by a coverage file, split natural/artificial), strike.py (aim + meter-timed throw),
  e1.py/e1t.py, e2.py/e2t.py, e3.py, e4.py, e5.py, e6.py (experiment scripts, exact replays),
  summarize.py (writes evidence/static_regions_observed.json).
- evidence/: inputs.log (all presses/writes/imports in order), coverage dumps (e1t, e2t, e3, e4t, e6t; E5 untraced), snapshots e1_scorecard.json (natural final scorecard), e3_4p_lane.json
  (natural 4-player game at lane start), e5_scorecard_natural.json (natural 235 game at the final scorecard), montages *.png.

## Experiments (budgets: emulator.step unit=frame, max_operations 1e8 per call; traced runs use
trace.configure enabled=true capacity=0; ~3.5 frames/s traced, see TK-004)

| ID | Start | Inputs / patches | Natural? | Result |
|---|---|---|---|---|
| E1/E1t | snapshots/snap_f10_last.json | A x3 (final throw), 400 f, **B**, 500 f, A | natural | Scorecard closes on B; results "HI SCORE / A PRIZE WINNER!" screen; A returns to the title. e1_leave_scorecard.png, e1t_coverage.json |
| E2/E2t | evidence/e1_scorecard.json | WRITE $87=2, $0340=2C 01 (300), $0356=13 01 (275), $036C=FA 00 (250), $AA..$B1 names; B; 3300 f; A | ARTIFICIAL | Three hi-score insertions with name reveal, award scenes 300/275/250. e2_awards.png, e2t_coverage.json (all context artificial=true) |
| E3 | snapshots/snap_title.json | 250 f; Left Up Down Right Left Up (port 1); Start; per bowler (P1/P3 port 1, P2/P4 port 2): cursor moves, A, Down, A x3, Up, A | natural | L-009 confirmed (Right/Left players, Up/Down lane); 4-player selection and 4P lane start. e3_title_4p.png, e3_coverage.json, e3_4p_lane.json |
| S1-S4 | snapshots/snap_lane1.json | strike.py grid (aim hold Left N frames, A, A when $D0 = hook, A when $CF = power) | natural | Strike: Left 60 f (LaneBallX 208), hook read 6 (released 7), power 16. 9 pins: Left 28 f, hook 3, power 16 |
| E4/E4t | snapshots/snap_lane1.json | strike recipe repeated (frames 1-6), then 9-pin + second ball | natural | Strikes frames 1-3 (StrikeStreak 1..3, celebrations), 9 + spare frame 4, strike frame 5, 9 + miss frame 6. e4_strikes.png, e4t_coverage.json |
| E5 | snapshots/snap_lane1.json | strike recipe every ball (untraced), B at scorecard, A | natural | Game total 235; results screen inserted 235 at rank 1 (AAA, sound 9, name reveal); title. e5_natural_game.png, e5_scorecard_natural.json |
| E6/E6t | evidence/e3_4p_lane.json | strike recipe, port 1 for bowlers 0/2, port 2 for 1/3, 7 balls | natural | 4P rotation and pad switching on lane 2; recipe gives 8/4 on lane 2. e6t_4p_throws.png, e6t_coverage.json |

Coverage summary: evidence/static_regions_observed.json (natural E1t+E3+E4t+E6t: 65 Static regions, 2809 bytes, 922 starts; artificial-only E2t: 4 regions, 526 bytes).

Inspection of lead questions (static) is summarised in analysis/handoffs/round-1-redirect.md.
