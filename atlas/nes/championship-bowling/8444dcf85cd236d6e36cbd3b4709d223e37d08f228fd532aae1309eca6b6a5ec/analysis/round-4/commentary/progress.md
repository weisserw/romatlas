# Commentary progress

## Round 4 (checkpoint pass-004, 512 regions)

- Baseline: the map changed (pass-003 -> pass-004: 11 code regions Static -> Observed,
  region 379 split at $A84E into new Static region 512, evidence text of the 23 L-001/L-002
  regions updated). The whole round-3 commentary was moved to analysis/round-3/commentary/
  and `commentary.py prepare` made a fresh verified baseline (byte-identical).
- Carry (analysis/tools/commentary-round4/carry.py, the round-3 method): round-3 game.asm
  copied, 12 region markers rewritten, 1 marker inserted at $A84E. No .incbin split needed.
  Marker list equals before/game.asm; verified byte-identical before any edit.
- Edits (analysis/tools/commentary-round4/batches/, applied with replace.py):
  - L-018: header and 23 unreachable routines/islands reworded ("unreachable from the
    vectors", closure explained once in the header); CheckPause / GameStart pause note;
    gated NMI jobs and UnusedBestScoreData wording; RNG contract at RandomSeed, Random,
    AdjustHookForLane and StrikeCelebration (Random(10) thresholds per variant);
    TILE_MISS/TILE_GUTTER/TILE_SPARE/TILE_STRIKE constants ("-", "G", spare, strike) used in
    WriteTenthFrameMarks/WriteFrameMarks (12 operands); ScoreBall third-ball encoding;
    ScoreFrames 10th-frame sum note; ReleaseBall slow roll note.
  - Region 512 (`jmp @countBall`, $A84E) commented as a defensive path.
- review.json regenerated with cm.py review: 512/512 reviewed, 988 units; 497 entries
  unchanged from round 3, 13 revised (reworded headers), 2 new (379 piece, 512 with a
  hand-written summary). editorial_audit stays pending.
- Verification: `commentary.py verify` ok, byte_identical after the carry, after each batch
  and at handoff; reading copy refreshed.
- Deferred ranges: none for readability. Final editorial audit reserved for final acceptance.
