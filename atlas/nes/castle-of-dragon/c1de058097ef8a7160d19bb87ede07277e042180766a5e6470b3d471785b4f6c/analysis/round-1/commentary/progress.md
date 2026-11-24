# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 1 commentary pass (2026-11-24) - complete

Scope: all currently classified source of checkpoint pass-001b (maps/current.json,
sha256 0ee481b5...): every code region in bank 6 ($90D8-$BFE9) and bank 7
($C000-$FD99), every code-embedded table, the data/image blocks of banks 0-6, the
vectors/footer, and the single unknown byte ($C415).

Method (reproducible, local): source/ is generated once per edit batch from the
frozen before/game.asm plus the annotation files in notes/ by tools/apply.py
(labels, comments, operand symbols, constants; byte identity checked by verify).
tools/review.py fills review.json from the generated source. tools/list.py and
tools/asmdb.py are the listing helpers used while reading. Do not regenerate
source/ without these notes; edit notes/*.txt and rerun apply.py, review.py and
verify.

Result:
- 236 global routine/table labels, 1221 local branch labels (@name, all named;
  no address-only labels remain), 257 symbol definitions in source/symbols.inc (RAM,
  object-table fields, hardware registers, bank/button/state/bit-op constants).
- No raw RAM operands remain; ROM table operands use labels or bank-qualified
  constants (bank-4/5 tables, bank-6 tables used from bank 7).
- Phase-1 static-only code is marked in place ("[static-only code ...]"); the
  contiguous region headers were folded so routines read continuously.
- review.json: all 497 scaffold entries reviewed with summaries and source
  locations; 632 units (routines/tables plus continuation units). The editorial
  audit stays pending for final acceptance.

Main interpretations established (see source comments for details):
- Runtime: cooperative task scheduler, zero-page frame allocator (Fr+n,x),
  VRAM queue, PRG bank switching, palette brightness fades.
- Sound driver (fixed bank, data in bank 5): 8 tracks, command set $F0-$F9.
- Map engine: bank-4 map layout of 32x32 blocks, block definitions/attributes per
  ScreenKind, bank-5 2-bit collision map; metasprites (bank-6 frame grids, bank-4
  size tables), flickering sprite builder.
- Gameplay: player control/physics/weapons, page guardians driven by AI through
  the unused pad-2 input bytes, enemies, boss screens drawn in the background,
  items/quest flags, experience and energy bar, stage clear and game over,
  world map, opening cutscene, ending, hidden test mode (SPTEST/BGTEST).

Verification: see verification.json (byte_identical true) and reading-after.asm.
Deferred ranges: none. Unresolved meanings are listed in uncertainties.md.
