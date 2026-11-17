# Round 2 commentary handoff - COMPLETE (current-source readability on the pass-002 baseline)

## Scope and checkpoint
- Checkpoint: maps/pass-002.json = maps/current.json = analysis/state.json = commentary/baseline.json (sha256
  ebcf33141ae8...). It has not changed since the fresh commentary baseline was prepared (2026-11-16 01:48). The round-1
  commentary is preserved under analysis/round-001/.
- Source is rendered from notes by commentary/tools/render.py (never hand-edit source/*.asm). The round-2 notes are
  86-99, plus 85b and 85c. Generators: tools/gencopies.py (code copies), tools/genscreens.py (screen records and
  metatile sets), tools/genlocals.py (shape-based branch names). To refresh after renames, run them in this order:
  gencopies, genscreens, genlocals, render, review.
- This handoff covers two workers. Worker A was interrupted at about 02:11 after writing notes 86-96: every
  pass-002 code run got routine names and docs. Worker B (this continuation) did everything else listed below.

## Verification
- `commentary.py verify` gives byte_identical true. It was run fresh after the final render, and
  commentary/verification.json holds the result.
- `commentary.py read` refreshed commentary/reading-after.asm.
- review.json: all 4,068 scaffold entries are reviewed, 0 are pending, and there are 1,925 units (routines plus
  documented data structures). The scaffold is unchanged, and editorial_audit stays pending until final acceptance.

## What changed for readability
- New code (180 runs, 17,932 bytes):
  - No routine carries the mechanical auto summary, and there are no Sub/Loc/Tbl address labels.
  - Every forward branch in new code has a descriptive name. The bank 5 scroller and bank 13 camera locals carry over
    to all 9 byte-identical copies.
- New data (12,446 bytes, plus the phase-1 pieces of the same structures):
  - render.py gained RANGE() role notes and role notes derived from the evidence in baseline.json. Pieces read by
    classified code now name their consumer routines. In total, 173 generic-data notes remain, all on phase-1 data
    with no known consumer.
  - Stadium pairs 2/11/13/14: FieldMap, FieldMapTail, FieldAttrMap and FieldMetatiles are labelled with their
    addressing formulas. The spare map columns $70-$7F hold the per-metatile cell properties read by LookupFieldCell.
    The at-bat metatile set and screen are labelled, along with their unread copies.
  - Screen records and metatile sets: 92 screen records are named after the routine that draws them (e.g.
    ScreenStadiumPicker, ScreenTradeConfirmScreen, ScreenChampionBanner). 23 records are reached only through
    ScreenDataTable and keep index names. 19 metatile sets are labelled.
  - Bank overviews are refreshed for banks 4-6, 11, 14-29. They had said "data only in phase 1" or described the
    mode handlers as unclassified.
- Unknown bytes: all 83 runs (1,727 bytes) have an inspection note covering their neighbours, how they decode, and a
  consumer where one was found.
- Corrections:
  - TeamBlockSrcPtrs and TeamBlockDstPtrs are six words each: $B760 + $150*n and $77B0 + $150*n.
  - Indexed $0101 accesses now read ColumnBuffer+1 instead of Page1Temp01.
  - $0180 is named StripAttrBuffer, and PpuBufferData+n replaces raw $030x addresses.
  - New RAM names for pair 12: EditWorkBuf, EditWorkByte and TradeSwapBuf.
- 177 generated @skip labels across the ROM got names from exact code shapes: @clamped, @wrapped, @no_carry,
  @sign_done, @no_<call> and @continue.

## Examples
- bank22.asm FieldMap_B22: the doc gives the row formula $8000 + (r - r/16)*128 and explains the
  LookupFieldCell columns.
- bank17.asm ScreenEditMenuScreen: a screen record whose doc lists its drawing routines. Continuation pieces say
  which record they belong to.
- bank30.asm $CF53 unknown note: it is the tail of TeamBlockDstPtrs, and the note lists the six resulting words.
- bank24.asm TradeSwapRecords: @save_first, @copy_second and @restore_first through TradeSwapBuf.

## Unresolved meanings (commentary/uncertainties.md, round 2 section)
- What the routine-like unknown islands are for. It is open whether they are dead code or reached through state
  that was never traced, e.g. bank 25 $A040/$A120/$B82B/$B8DD, bank 3 $B0B3/$BAE2, bank 15 $ADEC, bank 11 $AFCE,
  bank 10 $8F10 (L021).
- 23 ScreenRecordNN screens are unidentified. The grid rows in bank 19 $A6C0-$B9FF and bank 21 $B000-$BAC0 have no
  known entry.
- Cell-property layout in the other stadium pairs: it is shown only for pair 2, because LookupFieldCell always maps
  pair 2.
- The meaning of record byte 5 in TradeClassMarker, and the tentative names carried from round 1.

## Leads (analysis/leads.json, pass commentary-002)
- Produced L023 (open, low priority). Unknown runs whose data role the commentary settles:
  - PositionLetters entries in bank 8 at $974A/$9757.
  - PositionLettersB9 entries in bank 9 at $B362/$B36F.
  - The TeamBlockDstPtrs tail in bank 30 at $CF53.
  - Unknown runs that carry consumer labels.
  - Recipe: an extent check on each consumer's index range, then annotate as Data.
- Updated L021 to record that every remaining unknown run now has a source note.

## Remaining editorial residue (not blocking; same standard as the accepted round-1 handoff)
- Generated branch labels are still in place inside documented routines: 888 @skip, 381 @loop and 238 @return.
  They are mostly in display/menu helpers. The @skip counts per bank are:
  0:48 1:41 2:92 3:41 8:35 9:38 10:63 11:69 12:60 13:54 14:20 15:25 24:75 25:92 30:79 31:56.
  New pass-002 code has no @skip left.
- Raw absolute operands remain for multi-use scratch RAM whose role is not established (e.g. $074A-$074C,
  $048A, $0534). These are candidates for the final editorial cleanup.

## Scope/readability assessment
COMPLETE. All currently classified source is named and explained to the standard used in round 1:
- No routine carries the mechanical summary.
- There are no address-only labels.
- Every data region has a role note, a label or an honest generic note.
- Every unknown run has a boundary note and an inspection note.

Whole-ROM classification is still incomplete (99.56%, 1,727 unknown bytes; L021/L023) and is a separate
requirement. Final editorial acceptance (audit) has not been run.

## Toolkit
No new toolkit issues.
