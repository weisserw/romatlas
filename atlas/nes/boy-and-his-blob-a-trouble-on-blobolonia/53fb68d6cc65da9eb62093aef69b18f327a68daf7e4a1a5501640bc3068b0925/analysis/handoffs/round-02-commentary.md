# Round 2 — commentary handoff (COMPLETE for current-source readability; classification unchanged)

- Input checkpoint: pass-003 (maps/current.json == maps/pass-003.json == analysis/state.json, sha256 b5c79502...b7d01a).
  The map had changed since commentary-001 (built on pass-002), so the entire round-1 commentary was preserved under
  analysis/round-1/commentary/ (plus analysis/round-1/commentary-tools/) and commentary/ was re-prepared with
  `scripts/commentary.py prepare` (fresh before/, baseline.json = pass-003, 1365-entry review scaffold).
- Carry-forward mechanism: notes in analysis/commentary-tools/notes/*.txt are keyed by bank + CPU address and applied to
  the new frozen before/game.asm by `python3 analysis/commentary-tools/cmt.py review` (build + review.json). Unchanged
  ranges carried as-is; notes for every changed range (formerly unknown, Data->Code) were rewritten. Toolkit not modified.
- Output checkpoint: **commentary-002** = commentary/source/game.asm (sha256 ebbee43da800c3ccabdc8bdb4cd14a31639f984be853d090b35f638f8223e5f2).
- Verification: fresh `commentary.py verify` -> ok, byte_identical true (commentary/verification.json, copy
  analysis/commentary-002-verification.json). Reading copy refreshed (`commentary.py read` -> commentary/reading-after.asm).
- ROM classification unchanged (no map edits): 100.00% rounded, 262159/262160; 1 byte unknown (bank 0 $9AB8).

## Scope / readability assessment
- ALL currently classified source is covered, including every new pass-003 region and the older backlog: banks 0-7 code,
  all Data/Text/Padding blocks, header, CHR, and the single unknown byte (explained in source). No remaining ranges.
- 1910 labels; no Sub_/Data_ placeholders and no generic _SkipN/_LoopN/_ReturnN labels remain (69 renamed); all memory
  operands are symbolic except two RAM page-clear stores ($0500/$0600,x) and the explained BIT-skip operand $02A0.
- Split table blocks (phase-1 region history) now carry '(continues <table>; see the note at $xxxx)' pointers, so every
  Data/Text block is either labelled, commented or points to its owning table.
- review.json: 1365/1365 entries status=reviewed with summary + source; 396 units; every Code entry has a unit (spanning
  units split per entry). editorial_audit left pending (final acceptance; audit not run).

## Changes (examples)
- New code documented: SndVecRestart (b1 $8000) / ObjVecRestart (b6 $8000) -> RestartViaBank0 (b7 $C088: bank 0 + ColdStart),
  SndVecStopSfx (b1 $8009), AddAToZpWord (b1 $85E0).
- New/extended data documented: RoomFloorPtrs rooms $60-$FF, DPCM samples, PCM recording (bytes $14-$60), SparkleSpriteTable,
  PharmacyLabelText 7x16, FormExitDY 28, BoyThrowFrameTable ($8690-$86E7), HummingbirdDY 16 + HummingbirdDYPad,
  NotePeriodTable 64 words, MusicSequenceData ($86F2-$99BE, format described), metasprite layout (A/B 128 words, Alt 116,
  MetaspriteRecords $A2E8), scene/palette/status tables extents, RectTileData, ScreenStreams (B2/B3/B4), object-type
  tables (31 types), AnimScriptEntries, RoomPickupIndex/RoomRowTable 256, GridSlotX, 18 object-set templates (labelled by
  set; field layout), MintPosX/Y layout, DebugTextBoyH.
- Redirect r1 fixes applied: records start $A2E8 (MetaspritePtrsAltB/Data_A301 removed; DrawMetaspriteAlt A>=$80 operand
  now MetaspritePtrsAlt+$100 with a note); NotePeriodTable to $8671; `lda GridSlotX-3,y`; L-005 reference corrected
  (leads.json correction field; source text no longer cites lead numbers).
- Misleading names fixed: BoyLadderFrameTable -> BoyThrowFrameTable (indexed by BeanThrowTimer), SpawnXList1/2 ->
  ObjSetTemplate02_PX/0E_PX (template ObjPX fields read by GenXList1/2), UnusedWhistleTone -> WhistleToneBody,
  UnusedFlag92 -> UnwrittenFlag92; '&BridgeAtAnchor' text artefact -> '&$7F' (4 headers); raw RAM addresses in prose
  replaced by names ($0422 = ResetObjectsFlag now explained).

## Leads
- Consumed (documentation): L-010..L-022 regions now commented; L-005 correction recorded.
- Produced: L-024 (ready, role precision only): Padding-classified $FF runs inside tables (TypeBounces $A0B3-$A0BD;
  24 x 8-byte runs at template offsets $3A/$50 = ObjAnimTimer/ObjAnimOn slots 3-10). Non-code either way.
- L-023 (b0 $9AB8) unchanged; attempt note appended.

## Unresolved / outstanding editorial issues for final audit
- Mechanical-only meanings (commentary/uncertainties.md): BlobEatState/Timer, FormFlag74, FormEffectSel, ObjHover,
  UnwrittenFlag92, TorchFlag, set-script scenes, edge markers, pharmacy offer tables, debug text consumer.
- Music sequences and screen streams are documented by format only (not decoded per entry).
- Frame lists after AnimScriptEntries and RectTileData blocks interleaved with streams are described, not split byte-exact.
- Some long older headers could be tightened; '; -- observed/static from here' provenance markers are dense in places.
- editorial_audit pending; run `commentary.py audit` only at final acceptance.

## Deferred work
- Redirect: L-023 terminal review; optional L-024 kind correction (no percentage change).
- Final editorial audit at archive time.

## Toolkit
- No toolkit issues found in this phase (analysis/toolkit-issues.md unchanged; TK-001..TK-003 stand).
