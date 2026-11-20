# Commentary progress

Round 1 (pass-001 map) is preserved unchanged under analysis/round-1/commentary/
(its progress.md, uncertainties.md, review.json, source/ and tools/).

## Round 2 commentary (2026-11-20)

Baseline: the pass-002 map changed (81 new/changed regions; ROM classification 100%), so
the round-1 commentary directory was moved intact to analysis/round-1/commentary and a
fresh baseline was prepared (`commentary.py prepare`: before/, baseline.json = pass-002
map, verified byte-identical).

Carry-forward method: source/game.asm is regenerated from the new before/ by
tools/annotate.py with tools/spec.txt.  The spec is keyed by bank and CPU address, so all
round-1 names, headers, inline comments, operand symbols and data notes carry forward by
physical range.  Invalidated records: the 21 round-1 notes written for unknown blocks and
the bank-5 single-block note (all replaced); everything else applied unchanged (the
annotate run reports no unused spec keys).  Tool fixes: parser skips "; round-N" region
comments; source-line indices in regionlines.json were wrong after embedded newlines
(round-1 review.json source locations drifted after the first display list) and now
point at the first text line of each region; spec-local labels may repeat across scopes.

Scope covered (all classified source):
- New bank-7 code (Static, no known caller; never called dead/unused): SoftResetJmp,
  UploadPalettesCopy, DrawObjectFlagged (+ DrawObjectBody label), SignedDivideMathA,
  ClampFishVelocityTail, Inc16MathA/Dec16MathA, ParseDecimal16, Expand1bppTilesInline
  and its helpers, NmiWrapStripJob, ScrollToNametableAddr, UploadAttr2800,
  CopyPairsToOam, RotateOamCopy, SetSpriteAttrInline, StepObjectDirection,
  DrawMetasprite, WaitSecondsTimer, BusyWaitX, ReadPpuByteRequest, ReadPadDirection,
  WaitNewButtonPress, PaletteFadeOut/In + PaletteBrighten/Darken, CopyOamReversed,
  Mmc1Init/Mmc1Write*/BankSelectMmc1, three lone rts bytes.  New RAM symbols for the work
  bytes only these routines use ($1E, $35-$3A, $4D, $58/$59, $5D/$5E, $67, $98/$99,
  $0220, $02C0-$02C3) and MMC1 port constants.
- New bank-7 data: Exp1bppListInit/Fishing rendered as records with bank-5 symbols;
  OamRotationOffsets, DirectionDeltaTable, PadDirectionTable as commented rows;
  $CD filler notes.
- Bank 5 (31 regions): each region annotated; pointer tables rendered as .word
  Label+offset (FishFramePtrs, FishK*SizeFramePtrs, FishTilePtrs, LureFramePtrs,
  LureProfilePtrs, LureMotionPtrs), record tables as rows (FishPalettes,
  FishTileSetRecords, LureActionProfiles, SplashObjDefaults).  New names: FishK0-3Frames,
  FishSharedSprites, LureFrames, LureMotionTables, SplashFrames, TileMapAB35, DataB858.
- Backlog: song/effect numbers replaced by MUS_*/SFX_* constants at every PlayMusic/
  PlaySfx/PlaySfxTracked call site; ~300 generic @skip/@loop labels renamed in the 32
  routines with five or more of them; missing BTN_* masks in name/code entry and lake-map
  input; MapRowTiles/OamBuffer/PaletteShadow pointer immediates; two "unused" wordings
  replaced by "no writer found" / "not read by this consumer".

Findings: FishSharedSprites ($AC4D-$AE72, previously "consumer not identified") holds
sprite lists referenced by all four fish kinds' frame tables; the DrawObject probe branch
is reachable only through DrawObjectFlagged (nothing else sets ProbeMode); kinds 1-3
have empty tile records for size classes 4 and 0; Mmc1Init writes the CHR0 register twice
and its inc-$FFF9 reset idiom would not reset an MMC1 with this ROM's $00 byte.

Map: 16 region-start labels added from commentary names (no structural change), saved to
analysis/state.json, exported to maps/current.json, checkpoint commentary-002.

Verification: `commentary.py verify` byte_identical=true (commentary/verification.json);
reading copy refreshed with `commentary.py read`.  review.json: 767/767 entries reviewed,
1013 units; editorial_audit pending (final acceptance).

Deferred to final acceptance: see analysis/handoffs/round-002-commentary.md.

## Final acceptance (2026-11-20)

Scope: redirect findings R-01..R-14 plus the commentary-002 known editorial issues, then
the final editorial audit.  All edits were made in tools/spec.txt and annotate.py
renderers (PIECE_RENDER for DrawObject sprite lists, object records, the envelope table
and lake-map pointer tables; INLINE_TILE_BANK; NT_ADDR; joypad compare chains).
build_review.py now cuts summaries at sentence boundaries, generates display-list and text
descriptions, and no longer resets editorial_audit.  Details and residuals are in
commentary/editorial-audit.md.  uncertainties.md has a final-acceptance section.
Verification: `commentary.py verify` and `audit` byte_identical=true, with 767 entries and
1048 units reviewed and editorial_audit reviewed.  Map checkpoint final-001 (labels only).
