# Commentary progress

Prepared verified baseline (pass-002, map sha256 1476b029...). All semantic review entries started pending.

## Round 2 commentary (checkpoint pass-002)

Previous commentary (round 1, pass-001) preserved unchanged under
analysis/round-1/commentary/. This directory was re-prepared with commentary.py prepare
on pass-002 (407 regions; frozen before/, baseline.json, phase1-state.json).

Carry-forward:
- source/game.asm = round-1 game.asm with region markers rewritten to pass-002 by
  analysis/tools/commentary-round2/carry.py: 98 markers changed (87 Static->Observed, 11
  split regions' first pieces) and 35 markers inserted at the split points. No assembly
  line changed; verify byte_identical=true right after the carry.
- review.json regenerated from the source with the round-1 method (cm.py review, copied to
  analysis/tools/commentary-round2/). 353 entries carry identical round-1 text, 8 changed
  because their comments were revised, 46 are new split pieces (reviewed: provenance-only
  continuations of AdjustHookForLane, ScoreBall, ScoreFrames, WriteTenthFrameMarks,
  TitleMenu, StrikeCelebration, UpdateHighScores/RevealHighScoreName, FadeInSpriteGroups).
  407/407 reviewed, 909 units; editorial_audit pending (final acceptance only).

Edits (L-013, L-015 and gaps):
- SpinMeterUnused -> SpinMarkerY/Tile/Attr/X (OAM sprite 23); HideSprite23 -> HideSpinMarker
  (`ldy #SpinMarkerY-OamBuffer`); SetSpinMeterValues -> PlaceSpinMarker; SpinMeterTable ->
  SpinMeterArc ((X, Y) arc positions); LaneBall* marked as sprite 22.
- Runtime confirmations in comments: CheckScoreAward, AwardPerfectGame/Award275/Award250
  (scene descriptions), FinalResultsScreen, UpdateHighScores, ShowScorecard (paging, B),
  TitleMenu (wrap, runtime-confirmed), TitleBowlerThrow (unskipped animation), NextPlayer;
  HighScores/HighScoreNames no longer tentative; LaneOverviewTiles shared byte (L-011).
- Header explains Observed/Static markers; ShowPlayersDigit got its missing comment.
- uncertainties.md revised (resolved vs open).

Result: current-source readability complete for all classified source; deferred ranges: none.
Verification at handoff: see analysis/handoffs/round-2-commentary.md.
