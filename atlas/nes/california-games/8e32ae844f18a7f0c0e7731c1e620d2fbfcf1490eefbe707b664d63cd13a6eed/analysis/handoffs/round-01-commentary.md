# Round 1 - commentary phase handoff

Status: FINISHED (current-source readability pass complete for all classified source; final
editorial audit intentionally not run - reserved for final acceptance).

## Input / output identity
- Input checkpoint: maps/round01-final.json (sha256 b4a8fa61eed48625a58da3d0af895c73cb61bce2d3ff43eae33f5868a6c437e2);
  maps/current.json and analysis/state.json unchanged by this phase (classification not touched).
- Commentary workspace: commentary/ (prepared from that map; before/ and baseline frozen).
- Output source: commentary/source/game.asm sha256 c0b3595f62bc45f15ba35841e5f2989c28c64caa6eacd986c490d94649918397
  built from commentary/annotations.txt (sha256 4166073a...) by analysis/tools/cmt_apply.py.
- Review inventory: commentary/review.json (sha256 b06fbbc0...), 2265/2265 scaffold entries reviewed, 876 code units.
- Verification: analysis/round01-commentary-verification.json -> ok:true, byte_identical:true.
- Reading copy: commentary/reading-after.asm (refreshed after final verify).

## Classification (unchanged)
ROM classification: 83.73% (109762 / 131088 bytes classified; 21326 bytes unknown).

## Scope covered
All 579 code regions / 12,057 instructions, all 902 data regions and all 784 unknown gaps.
No remaining unreviewed classified ranges.
- 458 routine/fragment entries named; 300 with hand-written prose; all with generated mechanical
  headers (entry, fall-through, exit, calls, symbolic reads/writes, hardware).
- 1,537 code labels (structural branch labels under owning routine names), 99 global RAM symbols,
  38 event-scoped overlay symbols, hardware/button constants, 27 table symbols, 8 BIT-skip entries,
  49 line comments in core routines.
- Data regions: evidence class + referencing routines. Unknown gaps: boundary notes, fill notes,
  reference notes; 14 gaps with concrete evidence explained (reset code, indirect-table targets).

## Readability assessment
Good for system code (NMI/frame dispatch, bank switching, sound driver, text/menu system,
registration/results) and for the top level of each event. Event internals (especially foot bag,
flying disk, half pipe helpers) are named and mechanically described but semantics are partial;
branch labels are structural rather than descriptive. Adequate for porting navigation; deeper
event semantics are future work.

## Findings
- Frame architecture: NmiState indexes FrameHandlerTable ($87B2, 15 entries incl. $18->$B9F1,
  $1A->$E2DA, $1C->$8A6D); IdleLoop $F27E.
- Event start routines via MenuActionTable kind 2: half pipe $8FF2, foot bag $9FDA, surfing $DBEC,
  skating $CEB9, BMX $BD58, flying disk $AEC4.
- CallBank1-7 BIT-skip chain $F2BD-$F2CF; bank 2 jump-table entries 3/4 are BIT-skip entries
  inside $87E7 (not unknown targets).
- Scores are 3 base-100 digits per player (AddScore), printed via BinToBcdTable.
- Reset code $86ED and reset vector target $FFB0 are still unclassified.

## Leads produced (analysis/leads.json)
L-R1C-01 reset code + vector target; L-R1C-02 indirect-table targets in unknown gaps (frame
handler $18, surf states 5-7, skate state 4, sound commands, bank 1/2/3 jump-table targets, menu
actions $8C15-$8C21); L-R1C-03 bank 0 bus-conflict table. Earlier L-R1-01..08 unchanged.

## Unresolved questions / deferred work
See commentary/uncertainties.md: event overlay variables beyond the scoped set, $07-$09 entropy
bytes, exact meaning of $0D/$55/$87, bank 4/6 helper roles, skating name-compare (easter egg?).
Future rounds: when maps change, preserve this commentary (annotations.txt + source) under
analysis/round-N/, re-prepare, then re-run cmt_apply.py (annotations carry by CPU address/bank;
re-check labels in changed ranges).

## Toolkit
No new toolkit issues found in this phase (analysis/toolkit-issues.md unchanged).
