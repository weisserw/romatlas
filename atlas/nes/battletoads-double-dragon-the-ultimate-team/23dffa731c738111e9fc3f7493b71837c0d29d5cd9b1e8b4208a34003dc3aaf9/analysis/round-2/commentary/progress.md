# Commentary progress

## Round 2 (pass-002 map) - COMPLETE (current-source readability)

Round 1 commentary (pass-001 map) is preserved unchanged under
analysis/round-1/commentary/ (moved before `commentary.py prepare` ran on pass-002).
This directory was freshly prepared from pass-002 (baseline byte-identical).

### How the source is maintained (same approach as round 1)
source/game.asm = `python3 tools/annot.py build` (run from commentary/) applied to the
frozen before/game.asm + notes/*.txt. Notes carried from round 1 unchanged (all 1269
pass-001 regions are preserved in pass-002, so bank:CPU keys = same physical bytes).
Round-2 tool changes: annot.py also follows ObjTypeHandlers $9A71 / BgObjHandlers $9C4E
as routine roots; `?B:AAAA` notes now also annotate classified Data regions starting
at AAAA (fallback = phase-1 evidence text); stale `?` notes are reported on build;
map data labels that duplicate a notes constant become comments.
After edits: annot.py build; commentary.py verify; tools/fill_review.py.

### Batch log
- B0: carried build verified byte-identical (before any new notes).
- B1: bank 0 new code (pass-002 handlers via ObjTypeHandlers, fragments, split entries) named + commented in notes/r2_b0a.txt; round-1 misnames E842/EA26 corrected. Verified.
- B2: bank 6 actors/fragments (notes/r2_b6.txt) + banks 1/2/3/5/7 fragments (notes/r2_misc.txt). Verified.
- B3: data-region notes: notes/r2_data.txt (custom) + automatic consumer/evidence notes for the rest (annot.py data_note). Stale round-1 gap notes removed/rewritten. Verified.
- B4: unknown-run notes (notes/r2_gaps.txt); found that most bank 6 L-020 residuals are screen-actor motion scripts (pointer immediates -> StepBgObjScript, 4-byte records, 0-terminated, ending exactly at run ends). Verified.

### Result
- 977 routine entries in classified code, all with descriptive names (no Sub_Bn_ left);
  901 have block comments (every routine of >= 4 instructions; the rest are 1-3
  instruction helpers/entries with self-describing names). New this round: ~280 names
  in notes/r2_b0a.txt (bank 0: all 92 ObjTypeHandlers targets, split entries, uncalled
  fragments), notes/r2_b6.txt (41 screen-actor handlers + helpers), notes/r2_misc.txt.
- Renamed (contradicted by pass-002 structure): E842 ScriptedEnemyA_Update ->
  SparkEvery4thFrame; EA26 ScriptedEnemyB_Update -> ApplyGravityFastAndDrawOfs (body is
  ObjType33_Boss); bank 6 BuildColumnData -> StepBgObjScript, NextColumnIndex ->
  SignedNibbleTo16, ScrollLayerA/B -> ActorYVelMinus2/ActorAddXVel; removed the stale
  ObjType_Flyer constant ($A8F9, now ObjType41/42 at $A907/$A8FA). StageId comment fixed
  ($08 is level 5, not title/attract).
- Data: all 163 new data regions carry notes (109 custom in r2_data.txt incl. the big
  graphics/sound/anim/map blocks; others an automatic consumer/evidence note naming the
  reading routines). 31 table constants added (r2_consts.txt) so operands are symbolic.
- Unknown: 99 runs (1,691 bytes) each have a boundary/uncertainty note (r2_gaps.txt,
  stubgaps.txt or the automatic note).
- review.json: 1649/1649 scaffold entries reviewed, 901 units (tools/fill_review.py).
  editorial_audit pending (final acceptance only).
- Verification: commentary.py verify ok=true, byte_identical=true (verification.json,
  game.asm sha256 881e6a2f...); reading copy refreshed (reading-after.asm).

### Deferred / unresolved
See uncertainties.md "Round 2". No source ranges remain without readability notes.
Redirect question: bank 6 L-020 residuals are mostly screen-actor motion scripts
(see analysis/handoffs/round-02-commentary.md).
