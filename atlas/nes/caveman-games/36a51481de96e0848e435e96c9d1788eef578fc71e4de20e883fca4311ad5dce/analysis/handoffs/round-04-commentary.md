# Round 4 — commentary handoff

**Status: COMPLETE** (current-source readability for checkpoint pass-004). editorial_audit stays pending.
input/game.nes unchanged; toolkit unchanged; nothing published/archived; no processes left running.

## Scope / method
- pass-004 differs from the round-3 baseline (pass-003). Round-3 commentary preserved whole at
  analysis/round-3/commentary/ (diff -r identical); commentary/ re-prepared (`commentary.py prepare`, 4306 rows).
- notes/, tools/, uncertainties.md carried by physical range; build_source.py accepted every carried note.
- All 73 records of analysis/round-4/changes.json checked against the rebuilt source and commented.
- Commentary tools only: build_source.py no longer emits '; ====' propagation markers as headers, and writes
  map/set pointer immediates as `#<Label`/`#>Label`. update_review.py now also records headed data units.

## L-020 commentary part (done -> L-020 verified)
- Bank 2: FS_FlameCels explained (false "no reference found" removed). FS_FlameFramePtrs (19 words) and
  FS_PoseFramePtrs (17 words) are now word tables. FS_PoseCels added, plus a note for $9420-$944B.
  FS_BackgroundMetatiles extent fixed (79 records), with a note for $9C25-$A7E0.
- Bank 1: MT_MsprFrames is a 50-word table into MT_MsprCels ("generic data after frames" removed). Added
  MT_FieldMap, MT_StripMapA/B and MT_FieldMetatiles.
- Banks 1-6 banked SFX tables: the labels propagated from bank 0 covered the data after them. Each bank is now
  re-cut from its own effect table and a stream parse: SfxChannels, SndInstruments, a SfxStreamPtrs word table
  and the streams.
- Maps and sets that the old labels swallowed now have their own labels:
  - Bank 3: DR_CourseMap, DR_BackgroundMap and DR_BackgroundMetatiles.
  - Bank 4: SR_CourseStrips, SR_BackgroundMap, SR_BackgroundMetatiles, plus SR_MsprFramePtrs (45 words) into
    SR_MsprCels.
  - Bank 6: DV_ActorDefaults (now 28 B), DV_RunMap, DV_SceneMapLeft/Right and DV_SceneMetatiles.
- Bank 5: CL_ArenaMap, CL_ArenaMetatiles and CL_MsprCels; CL_MsprFramePtrs is now a 32-word table.
- Bank 0/7 menu graphics: TitleMapTop/Bottom, NumPlayersMap, CharSelectMap, MainMenuMap, MenuLowerMap(+Players),
  their sets, MenuMetatiles, and CavesOfFameMap/Metatiles. The event intro maps' 48-byte overlap is explained.
  The bank-0 sound-driver header moved to B0_SndUpdate.
- Operand notes: bank 5 $8074 is a dummy timing read; bank 6 $AD22 is DV_OutcomeTable-12.
- L-019 kind-only changes: removed the stale "not padding/text" wording. Added StandingsNames, NameOffsets and
  Headers headers.

## Review
857 code units (829 identical to round 3, 28 with only header text or extent refreshed) plus 189 data units are
all reviewed. All 4306 rows are reviewed. editorial_audit is pending.

## Verification
`commentary.py verify` -> ok, byte_identical true (commentary/verification.json, after the last edit).
`commentary.py read` refreshed commentary/reading-after.asm.

## Unresolved (commentary/uncertainties.md, Round 4)
- Generic data with unknown role:
  - bank 0: $A108-$A147 (probably an unused title map) and $A8CF-$A9AB (lists)
  - bank 2: $9420-$944B, $9C25-$A7E0 and $ACE0-$AFFF ($FF fill)
  - bank 3: $B375-$BEFF
  - bank 4: $B348-$B369
- Probably unused metatile records: bank 0 $ADBF-$AE7E and bank 7 $D881-$D922.
- Unproven extents and set assumptions:
  - DR_CourseMap's end is not proven.
  - The strip maps' set is inferred.
  - DR_BackgroundMetatiles records 76-103 rest on stride evidence only.
- Carried: L-004 roots, L-005/L-006 semantics, IrqHandler static-only, IrqSplitParam odd bytes.

## Leads
- L-020 -> verified.
- New L-021 (low): bank 7 Text $D899-$D8A3 sits inside unused MenuMetatiles records, so it should be Data.
  Bank 0 Text $A92E-$A94A is part of a $FF-separated value list (optional).

## Remaining gaps
None in current-source readability. Final editorial audit/archive acceptance remains separate.
