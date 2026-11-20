# Round 5 commentary handoff (final acceptance)

Status: COMPLETE. Current-source readability scope (pass-005, 100.00% classified) done;
final editorial review recorded. `commentary.py audit` passed once, at the end: ok,
byte_identical true, 1,401 regions and 1,325 units reviewed, editorial_audit true,
reading copy refreshed (commentary/reading-after.asm).

## Baseline / carry-forward
- Map changed (pass-005). Whole round-4 commentary preserved untouched at
  analysis/round-4/commentary/ (round-1..3 copies not touched).
- `commentary.py prepare` built a fresh frozen baseline from pass-005. notes/, tools/,
  progress.md and uncertainties.md carried; notes re-applied by bank/CPU address.
  checknotes initially flagged 7 comment keys at removed splits (3:$892C/$894D/$8975/
  $8991, 0:$9087/$BE40/$BE6F); these were deleted or moved (0:$90B2). Now 0 problems.
- review.json: 1,384 entries keep earlier credit; 17 entries whose source text changed
  (tools/review.py diff vs round-4 source) re-reviewed as round 5.

## Changes
- 3:$88B5 MulTable16x16 (was MetatileTileIndex): a*16+b = a*b; reads in
  ExpandMetatileTiles/Attrs with half*16 | MetaCount explained inline.
- 0:$9072 MapTruckPath: 8 stops x 8 sprite tile ids copied to OAM sprites 3-10
  (stop pose, tentative); AnimateMapTruck header corrected (was "positions").
- 0:$90B2-$90EC generic: honest no-reader note; $90ED note de-duplicated.
- 0:$BE30-$BE77 spark tables: whole 24-byte tables; truck-1 SparkStartY tail unobserved.
- L-028: TruckEndBitMasks F:$C723 (SetTruckEndBit) and TileReplaceRowStarts F:$D450
  (0/6/12/18 row starts); wrong "no direct reader" notes removed.
- DebrisAttrs now 24 B; 0:$BDE8/$BE00 labelled PieceTableUnreadA/B (no reader).

## Remaining uncertainties
commentary/uncertainties.md (round-5 section). Open: 0:$8F84, $90B2-$9180,
PieceTableUnreadA/B, LegalScreenTiles reader, 2:$83D4.., 3:$AF42, 4:$A474, 5:$8388,
6:$8400/$8701, F:$E9AC/$FF2C; partly decoded record fields; MapTruckPath pose meaning.

## Structural questions for redirect
None blocking; no map change proposed.

## Artifacts
commentary/source/{game.asm,ram.inc,hardware.inc}, review.json, audit.json,
verification.json, reading-after.asm, progress.md, uncertainties.md; notes edits in
r2_b3456, r2_b012, r3_regions, r3_tables_06, r3_tables_F, b0_2, r4_labels;
tools/review.py now diffs against round 4. Toolkit: no new issues; not modified.
