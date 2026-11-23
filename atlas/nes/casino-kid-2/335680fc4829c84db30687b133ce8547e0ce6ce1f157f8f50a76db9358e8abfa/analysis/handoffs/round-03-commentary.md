# Round 3 / commentary handoff

Status: **complete** for the round-3 current-source readability gate. Classification unchanged
(131,086 / 131,088; 2 bytes unknown, bank 4 $BB30-$BB31). Editorial audit (final acceptance) not
run; review.json editorial_audit pending.

Baseline: map changed (pass-002 -> pass-003). Previous commentary preserved in full at
analysis/round-2/commentary/ (diff -r identical; round 1 remains at analysis/round-1/), then a
fresh baseline was prepared from pass-003 (state == current.json == pass-003.json).

## Scope
All classified source: 1,603 code regions (63 now split Observed/Static), all data/padding
regions, the unknown pair. Names/comments carried by CPU address per bank; only records touched by
pass-003 region changes or the round-2 redirect findings were rewritten.
review.json 2,977/2,977 entries reviewed (summary + source), 1,830 units (routines, named data
structures, sound streams).

## Deliverable / reproduction
- Buildable source: commentary/source/game.asm + symbols.asm (local image.nes, layout.cfg).
- Generator: analysis/tools/round03-commentary/build.sh (gen.py, snd.py, ann/*.txt; patches/ hold
  this round's annpatch inputs; review.py for review.json; ins.py lists instructions).
- Reading copy: commentary/reading-after.asm (refreshed).
- Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json;
  game.asm sha256 30541c91...755c, symbols.asm 1930e7c3...d21a).

## Readability assessment
- Every routine entry has a descriptive name and header (0 placeholder headers); 0 raw-address
  operands; split code regions do not create fake routines (entries limited to pass-002 starts).
- New/changed tables have role notes; pointer immediates use #</#> of named tables
  (e.g. `lda #<(B4_StaffCreditLines+296)` for the empty credits line).
- Bank-6 sound data is now readable: `.word B6_SoundHeaderNN` pointer table, per-header
  type/priority + stream labels, and each stream as decoded bytes with command comments, loop and
  call targets as labels.
- Round-2 stray generic-note fragments next to structure notes were removed.
- Limits: immediates mostly numeric; 84 mechanical BankN_Data/Fixed_Data/Bank4_Msg labels and
  170 generic/evidence-only data notes remain where the format is not decoded.

Examples (game.asm line): B5_PickWinningNumber 20704, B5_SettleBets 21924, B5_ConfirmBet 19869,
B4_DrawHandName 11949, B4_CompareHands 13648, B4_EndingCredits 15819, B6_SoundHeader01 25603,
B6_Stream_8E42 25765, unknown note 16436.

## Key interpretations (new)
- Roulette outcome fixed at round start (B5_InitRound -> B5_PickWinningNumber: tier candidate
  lists, RltWinningNumber/RltWheelPos); bets = up to 5 records [cell, 32-bit amount] built with the
  chip panel (B5_AddChipToBet / B5_RemoveChipFromBet / B5_ConfirmBet, RltMinBet/RltMaxBet) and
  settled in B5_SettleBets via cell number lists, payout classes and multipliers 36..2.
- Poker showdown: B4_CompareHands ranks both hands (0 HIGH CARD .. 10 ROYAL FLUSH) and prints them
  with B4_DrawHandNames; tier message tables (43 per tier, message 0 = opponent name).
- Ending staff roll (B4_EndingCredits) via the credits line pointer table and THE END banner.

## Unresolved meanings
commentary/uncertainties.md "Round 3": hand orientation and tie-break details (static readings),
B4_HandAnalysis_9ADD/9DAD, marker arrays, RltMaxBet source, round object fields, unreferenced
strings (B5_PanelLabelStrings, B4_LuckyStrings), undecoded sound commands, B5_StartPressed spin
path, generic data blocks, $BB30-$BB31.

## Structural questions for redirect
1. R2-L021 $BB30-$BB31 (game.asm:16436): unchanged; needs a consumer or computed jump.
2. B4_HandAnalysis_9DAD reads a 2-byte table at $9DB2 with a rank index (0..12?) that would run
   into $9DB4 and code at $9DB6: check the index range (bounded trace of a showdown) to fix the
   table extent.
3. Readers of B5_PanelLabelStrings ($A8B5-$A8D5) and B4_LuckyStrings ($AD9C-$ADC2): search for
   computed/offset pointers (roles only, no classification change).
4. Low-confidence fragments (unchanged list in round-02-commentary.md) still unreferenced.

## Remaining gaps
None for the round-3 readability gate. Optional: decode remaining generic blocks and mechanical
labels (banks 2/5/6), undecoded sound commands, final editorial audit.
