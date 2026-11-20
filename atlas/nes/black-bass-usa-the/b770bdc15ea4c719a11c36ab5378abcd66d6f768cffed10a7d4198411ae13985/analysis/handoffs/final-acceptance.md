# Final acceptance handoff (final editorial audit)

Status: complete.  editorial_audit = reviewed; `commentary.py audit` passed.

## Checkpoint
- Input: commentary-002 (byte-identical).
- Output: final-001: maps/final-001.json, generated/final-001 and
  analysis/final-001-verification.json (validate ok, byte_identical=true, map sha256
  1ea41d786c8ce22da17d3892acd1959f9a688c35c54087aed143120fedfd435d).
- analysis/state.json saved and maps/current.json exported (same map as final-001).
  map.validate ok.

## Scope
- Redirect readability findings R-01..R-14 (round-002-redirect.md).
- Known editorial issues from round-002-commentary.md.
- Final editorial audit and verification.
- No classification change: 767 regions with identical bounds, kind, provenance and
  confidence.  131088/131088 bytes are classified (Code 19513, Data 111575, Unknown 0).
- Only 42 region-start labels changed in the map.

## Fixes (details in commentary/editorial-audit.md)
- Ranking logic comments: the reversed meaning of PlayerBeatsRivals is fixed by renaming
  it PlayerBehindRivals.  CompareTotals now says a deficit worsens the place.
  RankPoints is now ParticipantRank, the overall place 1..200.  Other renames:
  LakeBestPlace/LakePlaceLimit, ClassPlaceBase, ImprovePlaceForRival and
  SwapBoundaryPlaces.  Ceremony prize places are explained.
- Bank 5: DataB858 is now CastAnimFrames/PowerCursorFrames, split and rendered with the
  object records' Frames fields as labels.  TileMapAB35 is now LandingBoxTilesCopy.
  Sprite entries are documented in the order attributes, dX, dY, tile.
- Renames: MUS_SCREEN_END to MUS_STOP; CastPhase5Splash to CastPhase5Bounce (with
  BounceOffsets).  The DrawGreeting order is fixed.  The weak-claim notes now cite the
  slot layout and the epilogue survey.
- New inconsistencies found and fixed: SndEnvelope1-9 are renumbered to match
  SndEnvelopeTable, and DL_LakeLabels is renamed DL_LakeMap (lake map pictures), with
  palette, attribute, terrain and picture labels.
- About 330 generic labels were renamed.  The 56 remaining @loop labels are in routines
  of at most 15 instructions.  Other readability fixes: NT_ADDR(), character literals,
  BTN masks in compare chains, symbolic CopyTilesInline sources per bank, role-based
  names for tile and name-table blocks, MUS_/SFX_ prose, 50 new routine headers, about
  70 data-table notes, and regenerated review summaries.

## Audit result
- `python3 /root/romatlas/scripts/commentary.py verify <ws>`: ok, byte_identical=true
  (source sha256 3bb9ca97...127cec).
- `python3 /root/romatlas/scripts/commentary.py audit <ws>`: ok, byte_identical=true,
  reviewed_regions 767, reviewed_units 1048, editorial_audit true.  The reading copy
  commentary/reading-after.asm is refreshed.
- The audit tool cannot judge prose quality.  The editorial review behind it is recorded
  in commentary/editorial-audit.md and in review.json editorial_audit.notes.

## Residual uncertainties
- SFX_QUIET has not been measured.
- The on-screen look of fight reactions and retrieve actions is unconfirmed.
- Per-frame extents of the fish/lure/splash frame blocks are not split, and the last
  LureMotionTables extent is unproven.
- The internal tables of lake data banks 1-4 and LakeMapDataBank0 are not split.
- The visual content of the fishing tile sources is not identified.
- A few non-paired PPU high-byte immediates and address-suffixed duplicate text labels
  remain.
- The weak structural claims are unchanged: $C040 (0.7) and the lone rts bytes
  $D76C/$E24A/$E30F (0.6).

## Other records
- L-009 closed (analysis/leads.json).
- commentary/uncertainties.md: final-acceptance section added.
- commentary/progress.md and analysis/progress.md: final-acceptance sections appended.
  The coordinator table was not edited.
- Toolkit: TI-006 added.  A label-only map.annotate edit cleared comment/evidence, and
  the parameter is `label`, not `name`.  It was worked around and verified.  No toolkit
  changes were made.
- No server is left running.  Scratch /tmp/fa was removed.  input/game.nes is unchanged
  (sha256 b770bdc1...985).  archive.py was not run.
