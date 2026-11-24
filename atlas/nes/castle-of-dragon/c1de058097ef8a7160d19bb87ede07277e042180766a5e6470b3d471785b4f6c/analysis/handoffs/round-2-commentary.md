# Round 2 commentary handoff - COMPLETE

- Input checkpoint: pass-002b (maps/current.json = analysis/state.json, sha256 e30a89ef...).
  Changed vs the round-1 baseline, so the entire round-1 commentary was preserved at
  analysis/round-1/commentary/ and commentary/ was re-prepared from pass-002b (644 entries).
- Verification: commentary/verification.json - byte_identical true (game.asm 32bd7468...,
  symbols.inc 7ae0f295...). Reading copy refreshed: commentary/reading-after.asm.
- ROM classification unchanged: 100.00% (131087/131088; 1 unknown byte $C415).
  Source readability (separate metric): all classified source covered, no deferred ranges.

## Scope and method
All code in banks 6/7, all code-embedded tables, all data/image regions of banks 0-6
(including the 21 new typed bank-4/5 regions, MsprFramePtrTbl, MsprFrameData), vectors,
footer, unknown byte. Names/comments carried from round 1 by CPU address+bank through
commentary/notes/*.txt -> tools/apply.py (byte-identical on first rebuild, no dropped
notes); review.json rebuilt by tools/review.py: 644/644 reviewed, 766 units;
editorial_audit pending (final acceptance only). Details: commentary/progress.md.

## Readability assessment
Good: every routine and data region has a purpose/format note; branch labels named;
operands symbolic (RAM, object fields, constants, and now bank 4/5/6 data labels instead
of address constants). Weaker: guardian/enemy types named by number, sound numbers raw,
BossPatchData and SndSequenceData are described as blocks rather than per record.

Examples of this pass: bank-4/5 data notes (Bank4MapLayout, Bank4BlockAttrSet0-3,
Bank4BlockDefSet0-3, Bank5CollisionMap, Snd*), StaffRollFrames + UpdateEndingScene2,
SetupStage stage-7 rule, test-mode STAGE/WEAPON rows, Start weapon cycling, L-002 renames
(GuardianType0F_/EnemyType19_/EnemyPadMove_/BossPadMove_Unreferenced).

## Unresolved meanings (commentary/uncertainties.md)
- $C415 code vs padding (L-001, unchanged).
- L-002 fragments: no caller found; reachability unresolved (not disproved).
- Object type identities, sound IDs, quest item names; ObjFieldD/F.
- Map page group -> block set mapping inferred from block-id maxima.
- MsprFrameData $8A1D-$8A40 and $8F89 referenced by no frame; frames $E6-$EF overlap
  the staff text (usage unknown). SndSequenceData exact end unproven.
- S-PROG3 drop to emblem/title screen cause undetermined.

## For redirect
No new structural questions; only $C415 remains unknown (rejection review in round-1
redirect). Optional semantic leads: boss rooms stages 3-6 (S-BOSS3-6) for BossStage3-6
behaviour; identities of object types/sounds.

## Gaps
None in readability scope. Final editorial audit not run (by design).
Artifacts: commentary/source/, commentary/notes/, commentary/tools/, commentary/review.json,
commentary/progress.md, commentary/uncertainties.md, analysis/round-1/commentary/ (preserved).
