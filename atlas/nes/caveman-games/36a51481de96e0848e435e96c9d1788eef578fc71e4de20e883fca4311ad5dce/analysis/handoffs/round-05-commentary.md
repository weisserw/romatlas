# Round 5 — commentary handoff

**Status: COMPLETE** (current-source readability for checkpoint pass-005). editorial_audit stays pending.
input/game.nes unchanged; toolkit unchanged; nothing published/archived; no processes left running.

## Scope / method
- pass-005 differs from the round-4 baseline (pass-004). The round-4 commentary is preserved whole at
  analysis/round-4/commentary/ (diff -r identical; round-4 disassembly files beside it untouched).
- commentary/ was re-prepared (`commentary.py prepare`, 4280 rows). notes/, tools/, uncertainties.md and progress.md
  were carried; build_source.py accepted every carried note.
- Code is identical to pass-004. The source differs from round 4 only at the 5 changed ranges in
  analysis/round-5/changes.json, plus two note refinements.

## Changes
- Bank 3 DR_CourseMap ($944B-$954A): now exactly 256 B. The header gives the proof: fixed pointer, 8-bit Y (column
  and column+$0F), and the finish look-ahead is $FF (L-022). New boundary note for the tail $954B-$9560 (22 B, out of
  window, probably an unused continuation).
- Bank 7 $D881-$D922 and bank 0 $A8CF-$A9AB: the notes now describe single Data blocks. The former Text runs
  ($D899, $A92E) are explained as tile data, not text (L-021).
- Bank 2 $ACE0: the existing notes already matched the moved boundary.
- Weak hints, as uncertain notes only:
  - bank 4 $B348-$B369 is the size of two 17-byte records (attribute $30 is not used in the set)
  - bank 2 $9420-$943F has the layout of two 16-byte pose frames, but no pointer reaches it
- uncertainties.md: round-4 "end not proven" and "$D899 stays Text" are marked resolved; new Round 5 section.
- Tool fix: commentary/tools/update_review.py now ends data units at their own block. 13 lengths were tightened,
  e.g. DR_CourseMap 278 -> 256.

## Review / verification
- 857 code + 192 data units, all reviewed. All 4280 rows reviewed. editorial_audit pending.
- `commentary.py verify` -> ok, byte_identical true (after the last edit).
- `commentary.py read` refreshed reading-after.asm.

## Unresolved (unchanged roles)
- Generic data with no consumer:
  - bank 0 $A108-$A147 and $A8CF-$A9AB
  - bank 2 $9420-$944B, $9C25-$A7E0 and $ACE0-$AFFF
  - bank 3 $954B-$9560 and $B375-$BEFF
  - bank 4 $B348-$B369
- Unused metatile records: bank 0 $ADBF-$AE7E and bank 7 $D881-$D922.
- Other open items:
  - the set used by the strip maps
  - DR_BackgroundMetatiles records 76-103
  - L-004 roots, and the L-005/L-006 semantics

## Leads
- L-021 and L-022: commentary attempts recorded (status verified).
- L-005: hint note recorded (status deferred).
- No new leads.

## Remaining gaps
None in current-source readability. The final editorial audit and archive acceptance remain separate steps.
