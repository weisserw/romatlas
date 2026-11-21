# Round 1 — commentary handoff (COMPLETE for current-source readability; classification unchanged)

- Input checkpoint: pass-002 (maps/current.json == maps/pass-002.json; analysis/state.json unchanged).
- Prepared with toolkit `scripts/commentary.py prepare` (commentary/: before/, source/, review.json,
  prompt/workflow, reading copies). Toolkit not modified.
- Output checkpoint: **commentary-001** = commentary/source/game.asm (sha256
  3ed7f088ab4752ae21908a17bff129521f74928a5173117bdb93609b3d194be2), reproducible from
  commentary/before/game.asm + analysis/commentary-tools/notes/*.txt via
  `python3 analysis/commentary-tools/cmt.py review` (build + review.json refresh).
- Verification: `commentary.py verify` -> byte_identical true (commentary/verification.json,
  copy analysis/commentary-001-verification.json). Reading copy refreshed:
  commentary/reading-after.asm.
- ROM classification unchanged: 98.48% (258166/262160; 3994 unknown). No map edits.

## Scope / readability assessment
- ALL currently classified source is covered: banks 0-7 code, all data blocks, padding, header, CHR
  and every unknown gap are in the commented source. No remaining ranges for this phase.
- 1880 labels; 372 documented units (header comments on every routine, JSR target, table-dispatched
  entry and named table); no Sub_/Data_ placeholder names remain; ~70 minor branch targets carry
  generic Routine_Skip/Loop/Return names. ~350 RAM symbols with sizes, per-slot aliases (BoyX,
  BlobX, ...), hardware/MMC1 names, FORM_*/BTN_* constants substituted by context.
- review.json: all 1359 scaffold entries status=reviewed with summary + source line; units recorded
  per region; editorial_audit left pending (final acceptance is separate; audit not run).
- Uncertain meanings are worded tentatively in source; list in commentary/uncertainties.md.
  Bank 6 set scripts and some state bytes are described mechanically only.
- Example: MainLoop now reads as a list of named handlers (UpdateBoyFacing, ThrowBean, LandOnFloors,
  CatchBean, RunBlobFormHandler, ...); BlobFormHandlers lists the 14 bean forms by name.

## Findings (new, with evidence in source)
- Bean flavour number = blob form (Ladder, Bridge, Coconut, Bubble, BlowTorch, Jack, Key, Umbrella,
  Trampoline, Rocket, Hummingbird, Wall, Hole, Vitablaster); SelectedBean 14-16 = vitamins A-C.
- Bank 1 = sound driver + DMC player + metasprite tables; FarCallBank1 = PlaySfx (11 effects).
- Bank 7 $E000-$FFC0 = DPCM samples (bank 1 DmcSampleTable $99ED).
- Bank 5 = busy-wait PCM player used by the ending; $80A9-$B1DB = one PCM recording (settles L-002).
- Bank 7 $C322-$C521 = RoomFloorPtrs (256 words), floor lists from $C522.
- Bank 0 $856A-$856E = SparkleSpriteTable (indexed read $8569,y).
- Bank 2 $8000-$89DB = scene tables (stream ptr, flags/bank, CHR, palette, rect streams);
  streamer op table $B900; ScreenLoadStep RTS dispatch $BBA6.
- Bank 6 = object system (11 slots: 0-2 vitamin shots, 3-10 enemies/mints), pickups ($A407/$A507),
  room link tables $A5C1-$ABC1 (above, below Earth/Blobolonia, right, left, column, row).

## Leads
- Consumed: L-002, L-005; partially consumed: L-009 (table extents documented).
- Produced (redirect, classification): L-010 RoomFloorPtrs tail, L-011 DPCM samples bank 7,
  L-012 PCM recording bank 5, L-013 SparkleSpriteTable, L-014 bank-1 vector entries/$C088,
  L-015 unknown slots inside tables, L-016 stream/music/template gaps (analysis/leads.json;
  previous file kept as analysis/leads.pre-commentary-r1.json).

## Unresolved questions
- In-game meaning of bank-6 set scripts (grid/hazard/risers), pharmacy offer tables, $81/$02FD,
  FormFlag74, ObjHover, edge markers; whether bank 1 $8000/$8009 vector entries are ever called.

## Deferred work
- Redirect: reclassify L-010..L-013 ranges (evidence already identified), bounded pointer walks for
  L-015/L-016. After any map change, preserve commentary under analysis/round-1/ and re-prepare,
  carrying notes by physical range (notes are keyed by bank + CPU address).
- Final editorial audit (commentary.py audit) at archive time.

## Toolkit
- No toolkit issues found in this phase (analysis/toolkit-issues.md unchanged; TK-001..TK-003 stand).
