# Commentary progress

## Round 4 commentary (2026-11-23)

Baseline: map changed pass-003 -> pass-004 (maps/current.json == analysis/state.json ==
maps/pass-004.json, sha256 eb7719e3...9519). The whole round-3 commentary/ was copied to
analysis/round-3/commentary/ (diff -r identical) before `commentary.py prepare` built the fresh
frozen baseline (before/, reading-before.asm, baseline.json, phase1-state.json, review.json
scaffold 2,977 entries, now with no Unknown entry).

Method: round-3 pipeline copied to analysis/tools/round04-commentary/ (round-3 patch inputs kept
in patches-r3/; this round's in patches/bank3.txt, bank4.txt, bank5.txt applied with
annpatch.py; ann/ram.txt: BjVar07C2 -> BjOpponentName). Names, headers, RAM symbols and data
notes are carried by CPU address within each bank (= physical range); the generator re-validates
every name on an instruction start. Only records touched by the pass-004 changes, their
dependencies and the same panel mechanism in bank 3 were rewritten.

Changes:
- $BB30-$BB31 (now Static code): B4_UnreferencedStateLoad with an honest note (unreferenced
  leftover `lda zpLocal3`; reachability unresolved, not claimed dead). Being its own label it
  also keeps the zpLocal3 read out of B4_TravelMovePlane's register summary.
- $9DB2-$9DB5: B4_SuitOrderTable; B4_HandAnalysis_9DAD -> B4_SuitOrderLookup (result discarded).
  B4_CompareHighCards header rewritten as a suit tie-break with the possible original bug at
  $9D5B (eol note). B4_HandAnalysis_9ADD -> B4_OpponentPickHolds (opponent draw strategy).
- Bank 5: B5_PrintMessageValue -> B5_PrintPanelLabels, B5_PrintNumber -> B5_PrintPanelAmounts;
  B5_PanelLabelYou, B5_PanelLabelRecords, B5_PanelAmountRecords (former generic $8803-$8812),
  B5_PanelLayouts (former Bank5_Data_8268); B5_SayMessage header; B5_PanelLabelStrings note.
  Pointer immediates rendered #</#> of the record tables.
- Bank 3 (same mechanism, concrete defect in reviewed material): B3_PrintMessageValue ->
  B3_PrintPanelLabels, B3_PrintNumberAt -> B3_PrintPanelAmounts; B3_PanelLabelYou,
  B3_PanelLabelRecords, B3_PanelAmountRecords (former generic $911A-$912D),
  B3_OpponentNamePtrs/B3_OpponentNames (former Bank3_Data_9270), B3_PanelLabelStrings.

review.json: 2,977/2,977 scaffold entries reviewed (summary + source), 1,840 units; every code
region has units; editorial_audit pending (final acceptance only).

Remaining tentative meanings: commentary/uncertainties.md "Round 4" (and earlier sections).

Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json,
game.asm sha256 6cb1e1ee...c387, symbols.asm 526a13d5...ad1); reading copy refreshed
(`commentary.py read`). Gate scan: 0 raw-address operands, 0 placeholder routine headers.

## Earlier rounds
See analysis/round-3/commentary/progress.md (and round-2, round-1 copies).
