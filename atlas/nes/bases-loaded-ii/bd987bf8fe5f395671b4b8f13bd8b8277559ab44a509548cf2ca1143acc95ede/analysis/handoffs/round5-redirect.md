# Handoff: round 5, phase 3 (redirect), attempt r5-redirect-a

## Scope
- Checkpoint maps/round5-disassembly.json (0 unknown bytes). Map, source, ROM, toolkit unchanged.
- Research: analysis/redirect/r5-redirect-a/ (findings.json, batch-proposal.json, tools/
  datalabels.py, ptrpairs.py, selfbound.py, batch.py; listings datalabels.txt, ptrpairs.txt,
  selfbound.txt; leads backup). Static only; no runtime experiments, no milestone changes.
- One bounded sweep: L-024 bounds, then every unlabeled map Data region that holds a
  code-referenced source label (145 of 336 regions), tested for self-bounding pointer tables
  and consumer index domains. Round-3/4 exhaustion reviews reused, not redone.

## Findings (all Data stays Data; 0 classification change)
- **L-024 verified.** 12 pointers = $FB01+21t, the records end at $FBFD, and each record has
  9 distinct players (8-24) with positions exactly 1-9 and starters 0-7.
- **New: bank-2 team rosters.** D_B02_E811 = TeamRosterPtrs, 12 words = $E829+400t. The
  12 x 400-byte rosters (25 players x 16 bytes; PlayerRecordPtrA uses player*16) end exactly at
  DefaultLineupPtrs $FAE9. In all 300 records, byte $0D is 1-12 and byte $0E is 1-31 (birth date).
  This resolves the "$FA14-$FAE8 unidentified" block (the tail of TeamRoster11) and the L-014
  islands 2097-2107. Bank 2 $E811-$FF14 is now fully structured.
- **New: bank-14 close-up sprite tables** (DrawBatterSprites). $C0FA and $C5A2 each hold 2 side
  pointers (AtBatFlags bit7) -> 2 x 9 form pointers (PREC_FORM 0-8) -> 18 contiguous 64-byte frame
  tables (32 words, index (SwingTimer>>1)&$3E). They end at $C5A1 and $CA49. The map boundary at
  physical 232025 splits the last word: 1-byte boundary correction.
- **Layout tables** (bank 3): six 22-word tables at 44-byte bases, $C04E-$C155. They end at
  Observed code $C156. Re-split regions 2206/2110/518 at the table bases.
- **Named tables:** label-only for PitcherRecordPtrs, PitcherFormTiming (PREC_FORM 0-8 in all
  96 records) and BaseCoordTable. Split PitcherFormChrBank from its 2-byte prefix.
- **15 small self-bounding pointer tables** whose consumer index domains agree (banks 6/7/11/13/15).
  7 of them include uniform-stride records that end at a region end or a referenced label.

## Batch proposal: L-025 (ready; includes L-024)
analysis/redirect/r5-redirect-a/batch-proposal.json lists 57 items (8,320 B). Each item has the
exact physical [start,end), CPU address, bank context, label, type, evidence and touched region ids.
tools/batch.py re-asserts every layout from ROM bytes. Remainders of touched regions stay
unlabeled generic Data.

## Exhaustion review (evidence-policy.md)
- **Skipped, not cheaply provable:**
  - PitchBreakTable/PitchSpeedTable ends: only the start is anchored and the index arithmetic adds offsets.
  - Overlapping sub-table families (D_B07_CC14..CC4A, D_B15_FBDE..FC34, FE24/FE2A, FE65/FE6B,
    D_B13_C56F/CE75): the shared bases make the self-bound count inconsistent.
  - D_B04_D04E: self-bound fails.
  - D_B15_FDD5: a direct `and #$0F` index disagrees with the 5-word count.
  - About 110 other referenced D_B byte/coordinate tables: no self-bounding structure. Proving their
    extents would need per-routine domain tracing, with no classification gain. Generic data is
    acceptable under the goal policy.
- **Rejected/deferred items** keep their round-3/4 reasons (premises unchanged): L-005, L-009,
  L-018, natural stamina relief.
- No classification contradiction found. Every touched region is already Data.

## Recommendation
Run one final disassembly + commentary cycle that applies L-025 as a single mechanical batch.
Then proceed directly to final acceptance with no further redirect: the remaining questions are
semantics only.
