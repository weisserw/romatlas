# Round 3 — redirect handoff (attempt r3-a1)

**Recommendation:** classification is **terminal incomplete by 2 B** (L-018). There is one low-value ready lead, **L-020** (provenance and commentary only, 0 B). The coordinator can either:
- run one short disassembly + commentary cycle for L-020, or
- fold L-020 into final acceptance.

Neither changes the residual.

## Checkpoint and scope
- Input is **pass-003** (analysis/pass-003-verification.json byte_identical). Classification is 262,158 / 262,160.
- This attempt changed no map, state, source, ROM or toolkit. Research is in `analysis/redirect/r3-a1/`:
  - results.txt and rejection-review.json
  - observed-merge.json/.txt and observed/contexts.json
  - tools/: cont15.py, clr15f80b.py, orphref.py, newcov.py, rx.py
  - frames/
- The server was shut down. About 4.3 GB is free. Scratch is 2.8 MB (unchanged); the frames were moved into the attempt dir.

## Proposed vs accepted bytes
- **0 B proposed.** The residual is file 0x1C5AE-0x1C5AF (bank 14 $C59E, L-018), unchanged.

## Findings
1. **SkipArea16Flag ($76) bit7 is resolved.**
   - Static chain: the ObjKind33 defeat writes $80. Kind 42 and its defeat keep bit7. Only TitleAndNewGame clears $76, and continue leaves it alone.
   - GameFrameLoop_continueGame restarts area 17 at $88=$1E (area 15). AreaCleared, when leaving area 15 with bit7 set, jumps to $22 (area 17).
   - So the bit means **"area-16 boss beaten"**: after a continue, clearing area 15 skips the boss area already won. This explains why the bit has no effect when it is set inside area 16.
   - Artificial discriminating test:
     - With $76=$80 the area-15 clear goes $1E→$22. A Joker cutscene plays, then the kind-42 arena.
     - The control with $76=0 goes $1E→$20, and the area-16 boss intro plays.
     - The continue restart from area 17 to $1E was also observed.
2. The AreaCleared skip path does **not** use SetupBossSlotsArea17, because 0x113F0 was not fetched. The commentary guess linking them is contradicted, and its caller is still unknown.
3. **ObjKind42Defeated's non-indexed X read:** SetupBossSlots/BossSlotTemplate0 puts kind 42 in slot 0. It was observed in slot 0 in two separate loads. So the read is the boss's own X under that set-up. A placement by an area spawn list is not excluded.
4. **$38/$3E:**
   - The only references are in two orphan bank-2 routines.
   - The only classified writer is the reset zero-page clear. Runtime spot reads were 0.
   - So the values are constant 0: CsQueuePokeAt pokes at offset 0, and CsJoyStrobeRead does the ordinary 1-then-0 strobe.
   - Limit: (zp),y writes were not resolved exhaustively.
5. **Orphan callers via computed dispatch:** the word, RTS-1 word and immediate-pair scans (orphref.py) found nothing. All hits are coincidental, in Data or at non-operand positions.
6. **Coverage:** 72 opcode starts fall in 4 Static Code regions (list in observed-merge.json):
   - the continue restart at 0x1DE45
   - the skip test at 0x1DE9C
   - 0x1DEFE
   - the bank-8 area-16 boss intro script at 0x11717

   There were 0 fetches in Data, Text or unknown bytes, and 0x1C5AE was not fetched.

## Rejection review (compact; full: analysis/redirect/r3-a1/rejection-review.json)
- **L-018, code vs data at 0x1C5AE-0x1C5AF:**
  - The decisive artifact is r2-a1/l018-review.json. It found no reference, no fetch, and a loaded A that is clobbered before use. It rules out reachability and a meaningful entry, but not a dead remnant vs filler.
  - This round's traces (continue, area 15/16/17 transitions) add no fetch, and there is no new reference. The premises are unchanged, so the review is reused.
  - The bytes meet neither the code criteria nor the generic-data criteria, so they stay unknown. No new prerequisite was introduced.
- **Orphan reachability:** classification is already accepted. Encoded dispatch forms are negative, and the only remaining method is exhaustive state search, so this avenue stops.
- **Weak-role data blocks / area-map byte formats:** these are already classified. The question is semantic only, and pointer scans cannot discriminate. They are deferred, possibly as future commentary through the bank-4 consumers.
- No open question remains that could change classification. The remaining items are commentary niceties.

## Leads and scenarios
- leads.json pass r3-a1: added **L-020** (ready, low). L-018 got a history entry and is not reopened.
- scenarios.json: S6 was updated, and **S8-continue-skip-area16** was added (visited, artificial).
- Commentary corrections are recorded in L-020; source and commentary were not edited.

## Toolkit
No toolkit issues found (analysis/toolkit-issues.md unchanged).
