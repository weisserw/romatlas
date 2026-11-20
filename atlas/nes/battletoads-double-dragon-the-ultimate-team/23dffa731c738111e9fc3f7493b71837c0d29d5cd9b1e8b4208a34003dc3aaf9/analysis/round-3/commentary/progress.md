# Commentary progress

## Round 3 (pass-003 map) - COMPLETE (current-source readability)

Round 2 commentary (pass-002) is preserved unchanged under analysis/round-2/commentary/
(moved before `commentary.py prepare` ran on pass-003); round 1 stays at
analysis/round-1/commentary/. This directory was freshly prepared from pass-003
(1,660 scaffold regions, baseline byte-identical).

### How the source is maintained
source/game.asm = `python3 tools/annot.py build` (run from commentary/) applied to the
frozen before/game.asm + notes/*.txt. Notes and tools were copied from round 2; pass-003
only added regions (no pass-002 region changed), so bank:CPU keys still name the same
physical bytes. Round-3 tool changes (tools/annot.py):
- r3_*.txt files may override earlier @ labels; a later file's ?B:AAAA note replaces an
  earlier file's note for the same key.
- `!B:AAAA text` = block comment before an unlabeled instruction (uncalled fragments,
  module copies).
- `^B:AAAA Expr` = exact operand expression for a data base address (e.g.
  `StagePlayerMode-2,y`, `OffsetTable_B4_B933-$70,y`). Map labels on .reloc data are file
  offsets, so CPU-address constants with the same names are defined in r3_consts.txt.
- tools/fill_review.py now takes data/unknown/fragment summaries from the notes.
- tools/r3_report.py lists the notes emitted for pass-003 regions and remaining gaps.
After edits: annot.py build; commentary.py verify; tools/fill_review.py; commentary.py read.

### Batch log
- B0: carried round-2 build verified byte-identical on the pass-003 baseline.
- B1: 149 obsolete round-2 gap notes (bytes now classified, or remaining gaps) removed;
  r3_consts.txt (constants + operand expressions for 15 reader bases) verified.
- B2: r3_data.txt notes for all 49 new data regions, the 7 remaining unknown runs and the
  stale TileUploadTables_B3 note; r3_code.txt comments/names for all 54 new code regions
  (module copies, jmp slots, fragments), DrawHook_* names, EveryOtherFrame comment.
  Verified byte-identical.

### Result
- Names added: WaitVblankFlag_B0/_B4, DelayLoopX_B5, StepCameraYTowardA8,
  LeftoverBankSnippets_B5, DrawHookSlots + 10 DrawHook_TypeXX, table constants
  (WarpDestAreaTable, StageA9Table, StagePlayerMode, StageDriftMask, StageObj0584Step,
  PcmRateTable, CameraPageE4Table + map table labels as CPU constants).
- Renamed: bank 6 DecodeGfxToPpu -> WriteAsciiStringsToPpu (its table is AsciiToTileTable).
- No address-only routine names (no Sub_Bn_); every routine of >= 4 instructions has a
  block comment. review.json: 1660/1660 scaffold entries reviewed, 908 units;
  editorial_audit pending (final acceptance only).
- Verification: commentary.py verify ok=true, byte_identical=true (verification.json,
  game.asm sha256 9efe76ec...); reading copy refreshed (reading-after.asm).

### Deferred / unresolved
See uncertainties.md "Round 3". No source ranges remain without readability notes.
