# Round 4 / commentary handoff

Status: **complete** for the round-4 current-source readability gate. Classification: 131,088 /
131,088 (pass-004, 0 unknown). Editorial audit (final acceptance) not run; review.json
editorial_audit pending.

Baseline: map changed (pass-003 -> pass-004). Previous commentary preserved in full at
analysis/round-3/commentary/ (diff -r identical; round-1/round-2 copies untouched), then a fresh
baseline was prepared from pass-004 (state == current.json == pass-004.json, sha256 eb7719e3...).

## Scope
All classified source: 1,604 code regions, 1,247 data and 126 padding regions. Names/comments
carried by CPU address per bank; only records affected by pass-004 (b4 $BB30, $9DB2-$9DB5,
b5 $8798-$87CB, b5 $A8B5 evidence), their dependencies and the identical bank-3 panel mechanism
were rewritten. review.json 2,977/2,977 entries reviewed (summary + valid source), 1,840 units,
every code region has units.

## Deliverable / reproduction
- Buildable source: commentary/source/game.asm + symbols.asm (local image.nes, layout.cfg).
- Generator: analysis/tools/round04-commentary/build.sh (gen.py, snd.py, ann/*.txt; this round's
  annpatch inputs in patches/, round-3's in patches-r3/; review.py fills review.json).
- Reading copy: commentary/reading-after.asm (refreshed with `commentary.py read`).
- Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json;
  game.asm sha256 6cb1e1ee...c387, symbols.asm 526a13d5...ad1).

## Changes and examples (game.asm line)
- B4_UnreferencedStateLoad 16484: $BB30 leftover `lda zpLocal3`, unreferenced, reachability
  unresolved, explicitly not claimed dead; separate label keeps it out of B4_TravelMovePlane.
- B4_SuitOrderTable 13913 / B4_SuitOrderLookup 13906 (was B4_HandAnalysis_9DAD): index card/13,
  result discarded. B4_CompareHighCards 13776: suit tie-break, possible original bug noted at
  the `sta zpLocal4` (hand-1 half divides stale values). B4_OpponentPickHolds 13372 (was
  B4_HandAnalysis_9ADD): opponent draw strategy, return code 0..3.
- Bank 5: B5_PrintPanelLabels 17951 (was B5_PrintMessageValue), B5_PrintPanelAmounts 18052 (was
  B5_PrintNumber), B5_PanelLabelYou/B5_PanelLabelRecords 18033, B5_PanelAmountRecords 18089
  (former generic block), B5_PanelLayouts 17328 (was Bank5_Data_8268), B5_SayMessage 17242.
- Bank 3: B3_PrintPanelLabels 6209, B3_PrintPanelAmounts 6337, B3_PanelLabelRecords 6318,
  B3_PanelAmountRecords 6377 (former generic block), B3_OpponentNamePtrs/Names/
  B3_PanelLabelStrings 6683; RAM BjOpponentName.
- Pointer immediates of the four record tables rendered as #</#> of the table labels.

## Readability assessment
Gate scan: 0 raw-address operands, 0 placeholder routine headers, every routine entry named
and explained; uncertain readings in tentative prose. Mechanical data labels 104, generic-data
notes 168 (role unknown after bounded inspection; kept as explained generic blocks).

## Unresolved meanings
commentary/uncertainties.md "Round 4": B4_CompareHighCards bug outcome and DivideMulti by zero
(not run), B4_OpponentPickHolds return-code consumers, panel records never selected (b5 0/1),
$E0 caption glyph, bank-3 "MONEY" string without a record, GameVar0714/BjVar079C roles,
B4_LuckyStrings / b4 $A2DE-$A46B consumers, plus the round-3 list.

## Structural note for the final review (no lead)
Bank-3 phase-1 data boundaries $92B0 and $92D0 cut through B3_PanelLabelStrings ($9292-$92ED)
and $92D0-$932F also holds Bank3_Data_92EE; the region kinds are correct (Data), only the
boundaries do not follow string ends. Commentary documents this, it did not change the map.

## Known editorial issues for the final audit
1. 101 comment lines still cite raw RAM addresses in prose ($0747, $06B1, ...) where a symbol
   or GameVar/PkrVar name exists (e.g. B4_InitMatchMoney, B5_UpdateBetDisplay headers).
2. 94 RAM symbols remain role-undecoded GameVar/PkrVar/RltVar/BjVar/FixVar names.
3. Immediates are mostly numeric (~4,000 lda/cmp/... #$ operands); only pointer pairs symbolic.
4. 110 comment lines exceed 96 columns (DATA notes join "Name ($addr): " to the first line).
5. review.json routine-unit summaries are cut at 300 characters; data-region summaries are the
   first note line.
6. B6 sound: undecoded stream command bytes remain annotated as unknown.
7. 104 mechanical BankN_Data/Fixed_Data/Bank4_Msg labels and 168 generic data notes.

## Remaining gaps
None for the round-4 readability gate. Next: final editorial acceptance (`commentary.py audit`
after recording editorial_audit) and archive options.
