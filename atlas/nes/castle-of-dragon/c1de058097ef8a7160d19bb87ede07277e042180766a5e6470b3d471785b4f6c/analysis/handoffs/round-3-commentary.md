# Round 3 commentary handoff - COMPLETE

- Input checkpoint: pass-003 (maps/current.json = analysis/state.json, sha256 fc23d9a5...).
  Changed vs the round-2 baseline (pass-002b), so the entire round-2 commentary was preserved
  at analysis/round-2/commentary/ (round 1 untouched at analysis/round-1/commentary/) and
  commentary/ was re-prepared from pass-003 (752 scaffold entries).
- Verification: commentary/verification.json - byte_identical true (game.asm 7de15cf9...,
  symbols.inc 7ae0f295...). Reading copy refreshed: commentary/reading-after.asm.
- ROM classification unchanged: 100.00% (131087/131088; 1 unknown byte $C415).
  Source readability (separate metric): all classified source covered, no deferred ranges.

## Scope and method
All code in banks 6/7, code-embedded tables, all data/image regions of banks 0-6, vectors,
footer and the unknown byte. Names/comments carried by CPU address+bank (= physical range)
through commentary/notes/*.txt -> tools/apply.py; no dropped notes, code/label lines identical
to round 2 except the L-009 renames. Static-only markers regenerated from pass-003 provenance
(2769 bytes); every execution-status statement in comments rechecked against pass-003.
review.json rebuilt by tools/review.py: 752/752 reviewed, 854 units; editorial_audit pending.

## Readability assessment
Good: every routine/data region has a purpose or format note, branch labels named, operands
symbolic. Weaker (unchanged): object types and sounds named by number; BossPatchData and
SndSequenceData described per block, not per record; some generic @Loop/@Next locals inside
short routines.

Examples this pass: SpawnLiftPlatform ($C5D4, renamed from SpawnStage7BossBody) with the
slot-search and mirrored-position rule; UpdateHudAndStageEnd step 3 (stage-clear rules per
stage, forced 5/6 boss fights never end, pit-fall game over); SpawnPageObjects/LoadStage/
BossStage5-6/BossAiStage5-6 marked as having no natural entry (forced-entry only);
StageStart route-select rules; MsprFrameData $8A1D-$8A40 reworded (probable unused frame grid);
GuardianType16 HP rule; SPTEST/BGTEST observed through the natural test-mode route (L-010).

## Unresolved meanings (commentary/uncertainties.md)
- $C415 code vs padding (L-001, unchanged).
- L-002 fragments: no caller found; reachability unresolved.
- Object type / sound / quest item identities; ObjFieldD/F.
- MsprFrameData $8A1D-$8A40 usage; frames $E6-$EF; SndSequenceData exact end.
- Stage 5/6 boss screens: cut content in practice (no natural entry found), not formally proven.
- Closed this round: S-PROG3 title drop (pit fall -> game over -> Reset).

## For redirect
No new structural questions; only $C415 remains unknown (rejection review stands).
Remaining Static code (2769 bytes) is provenance-only.

## Gaps
None in readability scope. Final editorial audit not run (by design).
Artifacts: commentary/source/, commentary/notes/, commentary/tools/, commentary/review.json,
commentary/progress.md, commentary/uncertainties.md, analysis/round-2/commentary/ (preserved).
