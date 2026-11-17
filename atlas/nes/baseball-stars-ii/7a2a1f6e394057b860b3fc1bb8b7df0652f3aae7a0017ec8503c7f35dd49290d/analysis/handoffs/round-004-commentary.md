# Round 4 commentary handoff - COMPLETE (current-source readability on the pass-004 baseline)

## Scope and checkpoint
- Checkpoint: maps/pass-004.json = maps/current.json = analysis/state.json (sha256 077d58cd...). It has 4,107
  regions and 8 unknown bytes in 4 runs (analysis/pass-004-remaining.json).
- commentary/ was re-prepared from that checkpoint (`commentary.py prepare`, byte_identical; review scaffold of
  4,111 entries).
- The previous commentary is preserved whole under analysis/round-003/commentary (pass-003 baseline, c36cb8af...).
  It was copied and diff-checked before the old directory was replaced. Rounds 1 and 2 remain under
  analysis/round-001 and analysis/round-002, and nothing prior was overwritten.
- Carry-over: notes and tools are keyed by (PRG bank, CPU address). Every round-1 to round-3 note rendered on the
  pass-004 listing. The only warnings were the three end-of-line notes on the former F01 code bytes, which were then
  removed. The carried source verified byte-identical before any round-4 edit.
- Pipeline: commentary/tools/render.py builds the source from commentary/notes/*.py. The order is gencopies,
  genscreens, render, genlocals, render, review. Round-4 notes are in notes/99e_round4.py.

## Readability assessment
All currently classified source meets the naming and comment standard. The pass-004 changes are 21 bytes:
- C01, bank 30 $C0C0: **LoadChrBankTemp**. It loads ChrBankTemp and stores it straight back, so A = ChrBankTemp.
  It is probably a stub or left-over of the CHR-set helpers. No known caller.
- C02, bank 31 $F6C2: **AddRecordStrideTemp0C**. It is meant to add 18 (the EmptySlotFlagPtrs stride) to
  Temp0C/0D, but the high-byte ADC adds the new low byte. That defect is commented on the instruction. Probably dead
  code; no known caller.
- F01, bank 2 $8C52/$8CA0: the inline words are now data regions with role notes: VRAM $2204 for InlineParamCallB,
  and text buffer $00BA for InlineParamCallA. render.py now prints REGION notes on inline-parameter blocks.
- The PitcherListDrawAlt prose was rewritten. The stale "map still classifies them as code (L025)" sentence is gone.
  The value it prints is described as "d.dd" from record bytes 4/5, probably the ERA, and the raw $053D is now
  MenuChoiceB.
- Evidence-changed regions (11 Static -> Observed, 10 partly observed, in banks 1, 2, 9, 10 and 11: relief,
  pitcher of record, league finish, standings, match select and champion screens): the names and comments were
  re-checked against the observed starts. Nothing contradicts them, and the region headers show the new provenance.
- Older defect fixed: ten address-only trampoline labels were renamed after their targets, and every reference and
  note followed. The new names are:

  | Old label | New label |
  |---|---|
  | FarCall14_8F74 | FarDrawTeamEmblemBoard |
  | CallBank02_97D4 | CallLoadShiftPositions |
  | CallBank14_9999 | CallPitcherFatigueStep |
  | CallBank14_9BA1 | CallStadiumPicker |
  | CallBank15_A29B | CallLoadPaletteSet |
  | CallBank15_A324 | CallScoreboardBackground |
  | CallBank09_B18F | CallHomeRunScoreboard |
  | CallBank15_A9D7 | CallDrawInfoScreen |
  | CallBank15_B641 | CallAnimateArrowTiles |
  | CallBank11_B60B | CallCheckRostersPlayableAlt |

  The trampoline docs name their targets.
- Unknown residue (8 bytes):
  - bank 2 $80DA-$80DB (RTS pair)
  - bank 2 $9988-$998B (dead LDA pair)
  - bank 2 $9B48 (lone RTS)
  - bank 9 $A165 (lone RTS)

  Each keeps its decode, neighbour and uncertainty note, now with a pass-004 re-check sentence: no reference, sibling
  or consumer, and 0 fetches in S1-S27.
- Gate state: 0 "Purpose not yet established" summaries, 0 @skipN/@loopN labels and no `_XXXX` address-only labels.
  The only labels containing an address are PrintTeamNameAt2448 and DrawHeaderAt2052, where the address is the
  VRAM print position. 242 @return labels remain, each marking an RTS target.

## Verification
- `commentary.py verify` gives byte_identical true. It was run fresh after the final render, and the result is in
  commentary/verification.json.
- `commentary.py read` refreshed commentary/reading-after.asm.
- review.json: 4,111 reviewed / 0 pending, 1,969 units.
  - Credit is carried from round 3 for unchanged (offset, length, kind) entries.
  - The 21 evidence-changed entries, 5 new or re-split code entries, 2 new data entries and 4 unknown entries carry
    round-4 summaries.
  - editorial_audit is left pending for final acceptance, and audit was not run.

## Findings
- C02's high-byte add is defective. That supports reading it as dead code.
- The PitcherListDrawAlt value text is "d.dd": record byte 4 low nibble, '.', then the two nibbles of byte 5. It is
  printed 4 characters wide through PrintTextBoxC, which suggests an ERA field.
- The S26-S27 observations confirm the league-end and champion code paths. No earlier interpretation changed.

## Unresolved
- Purpose of LoadChrBankTemp and AddRecordStrideTemp0C. Both are Static with no caller and no observed fetch.
- Code or data for the 8 residual bytes (L021, blocked). This is the terminal residue: classifying it needs a new
  reference, a fetch or a discriminating structure.
- Earlier open items carried unchanged (commentary/uncertainties.md):
  - What the round-3 no-caller routines are for.
  - The FormatMoneyLine source field.
  - The value-only branch names in banks 13 and 24.

## Deferred (not blocking)
- Multi-role scratch RAM keeps raw operands. Examples: $0540 in bank 2, zero page $BA/$C0/$C1/$DB, and $0660 in
  bank 31 ValidateTeamEntry.
- Data with no known consumer keeps generic-data notes.
- The final editorial audit is reserved for archive acceptance.

## Leads
- L025 was consumed by pass-004, and its prose is now reflected in source.
- L021 gets a commentary-004 attempt record with no new evidence.
- No new leads.
