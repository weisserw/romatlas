# Commentary progress

Prepared verified baseline (pass-001 map, byte-identical) on 2026-11-23.

## Method (round 1)
- source/game.asm is rendered by tools/render.py from the frozen before/game.asm plus the
  annotation database annotations/*.txt (names, routine headers, notes, data notes).
  To change commentary: edit annotations, run `python3 commentary/tools/render.py`, then
  `python3 commentary/tools/review.py` (fills review.json) and the toolkit verify/read.
  Never hand-edit source/game.asm (it is regenerated); never edit before/.
- render.py automatic idioms (exact values kept, byte-identity verified): RAM/hardware
  symbols with +offset for arrays, labels for every branch/jump target, far-call slot
  immediates (#<FarN_Target) with target comments, frame-handler immediates
  (#<Handler/#>Handler), lda#/sta pointer pairs into ROM (#<Label/#>Label), bank-select
  write notes, data-region labels with .org CPU addresses, consumer lists on data regions.
  It warns about annotations at non-instruction addresses (none at handoff).

## Round 1 scope - COMPLETE for current source
- All 55 code regions (35,085 bytes; 16,512 instructions) in banks 0-3: every routine,
  frame handler, far-call target, jump-table target and vector has a descriptive name and
  header prose; internal branches named where meaningful (117 address labels remain).
- RAM map: 258 symbols (zero page, OAM/star/bullet/object/sound tables, RAM code images).
- All 422 data regions: named where a consumer/format is established (142 names + 74
  continuation regions), consumer lists and fill/generic notes on the rest.
- All 10 unknown fragments: explanatory notes (L001), still unknown.
- review.json: 487 scaffold entries reviewed with summary/source, 401 units.
  editorial_audit remains pending (final acceptance is a separate gate).

## Batch log
- Batch 1: RAM map, header, bank 0 boot/title/attract, mission start, flight frame,
  screens, sound driver, misc (annotations/00-header-ram.txt, b0-*.txt). Verified.
- Batch 2: banks 1, 2, 3 routines (b1.txt, b2.txt, b3.txt). Verified byte-identical.
- Batch 3: data names (data.txt), unknown notes (unknown.txt), RAM reuse notes,
  review.json, uncertainties.md. Verified byte-identical; reading copy refreshed.

## Names/interpretations to revisit (see uncertainties.md)
- Phase codes in Joy2Held, docking vs landing naming, tentative RAM names.

## Deferred
- Decoding formats of large data blocks (metasprites, maps, spawn lists, music data).
- Descriptive names for the remaining 117 internal Ln_XXXX labels.

## Round 1 resume pass (coordinator gate: naming gaps) - COMPLETE
- annotations/branches.txt: names for all 117 former Ln_XXXX internal labels plus short
  notes on the less obvious branches (dissolve masks, unrolled multiplies, sprite-0 wait).
  Corrected AttractFrame_Dissolve note: it masks CHR $1000-$13FF pixel columns, not
  nametable bytes.
- annotations/data-generic.txt: notes/names for all 96 formerly generic regions and 92 fill
  runs inside identified structures; new names MusicSeqDataA/B, DockingTilesRle,
  RleTileStreams1-5, MissionTileSets, CommonTilesRle, PictureDataB1, VramScreenStreams,
  PlaneTileGfx, TerrainTileQuads, TerrainMapSections, TerrainBlockRecords, MulB3_Image,
  NmiPadB1/B2, GfxDescriptorTable/RamRleDecoderImage format notes; vectors and
  bank-select tables explained. Stream ends checked by decoding (RLE/VRAM formats).
- data.txt: extents/formats added to PlaneLayouts, SongChannelPtrs, StarTilesRle,
  LogoTilesRle, SpaceDebrisFrames, PlaneTileSrcLo, VramBlockTable, TileTablesBC,
  TileTablesA, MetatileTable, MapSectionPtrs, StationAttrPairs, TerrainHeightHi,
  WarpCellPatterns.
- RAM: 12 new symbols for the last raw operands ($32 $34 $4A $4B $54 $78 $A1 $D0 $01FE
  $0600 $0668 $06FF); HW symbol NOISE_UNUSED ($400D). No raw RAM/ROM operand remains.
- tools/render.py: region notes with prose no longer print the generic "no static
  reference" sentence; HW table gains $400D.
- Verify: byte_identical true (game.asm sha256 98e61982...); reading copy refreshed;
  review.json refreshed by tools/review.py (487 entries, 530 units, all reviewed;
  editorial_audit still pending).
