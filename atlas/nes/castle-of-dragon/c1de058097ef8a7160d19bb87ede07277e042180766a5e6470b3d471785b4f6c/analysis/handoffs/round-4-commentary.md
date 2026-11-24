# Round 4 commentary handoff - COMPLETE

- Input checkpoint: pass-005 (maps/current.json = analysis/state.json; pass-005 verification
  byte-identical). Only change vs round-3 baseline pass-003: region-1245 at file $1C425 /
  CPU $C415 (bank 7), 1 byte, kind Data, provenance Manual, no label = explicit USER-DIRECTED
  generic Data fallback with unknown meaning (not an observed or statically proven role).
- Preservation: round-3 commentary/ copied intact to analysis/round-3/commentary/ (diff -r clean);
  rounds 1/2 untouched. commentary/ re-prepared from pass-005 (752 scaffold entries).
- Verification: commentary/verification.json byte_identical true (game.asm bec35a06...,
  symbols.inc 7ae0f295...). Reading copy refreshed: commentary/reading-after.asm.
- Review: 752/752 entries reviewed, 855 units (all reviewed); editorial_audit pending.
- ROM classification: 100.00% (131088/131088), of which 1 byte ($C415) is the Manual
  user-directed fallback. Source readability (separate metric): all classified source covered.

## Scope and method
Names/comments/review carried by physical range via commentary/notes/*.txt -> tools/apply.py
(no dropped notes/labels; code/label lines identical to round 3 before edits). All prior review
credit carried (other 751 entries: summaries/units identical apart from source line numbers).

## Edits
- $C415 source note, review entry (region-1245) and uncertainties.md: generic Data by explicit
  user-directed Manual fallback; meaning unknown; no role or label claimed; L-001 ambiguity
  (orphan RTS vs stray/padding byte) preserved. ClearSpritesAndObjects unit clipped to $C414.
- ObjTypeE818Tbl/E846Tbl -> ObjTypeExpTbl/ObjTypeHpTbl; UpdateGuardianPhysics header uses
  table symbols instead of raw addresses; 14 generic @Next* branch labels given role names;
  short notes for RewardTypeTbl, GuardianType15, BossNone, BossHandlerLo/HiTbl.

## Readability assessment
Good: every routine and data region has a purpose/format note, branch labels named, operands
symbolic, no address-named routines. Weaker (unchanged): object types and sounds named by number;
BossPatchData/SndSequenceData described per block; a few @Loop locals in short routines.

## Unresolved meanings (commentary/uncertainties.md)
$C415 meaning (L-001, now Manual Data fallback); L-002 fragment reachability; object type /
sound / quest item identities; ObjFieldD/F; MsprFrameData $8A1D-$8A40; SndSequenceData exact
end; stage 5/6 boss screens forced-entry only.

## Gaps
None in readability scope. Final editorial audit not run (by design, final acceptance only).
