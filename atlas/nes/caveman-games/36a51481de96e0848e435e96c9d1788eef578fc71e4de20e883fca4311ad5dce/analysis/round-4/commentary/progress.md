# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 2 commentary (2026-11-25) -- status: COMPLETE (current-source readability)

Input: checkpoint pass-002 (maps/current.json == maps/pass-002.json; analysis/state.json), 100.00% classified.
The map had changed since the round-1 baseline (pass-001b), so the whole previous commentary directory was
preserved at analysis/round-1/commentary/ (byte-identical copy, checked with diff -r) and this directory was
re-prepared with `commentary.py prepare` (fresh before/, baseline.json, phase1-state.json, review scaffold).

### Carry-forward method
- notes/ and tools/ copied from round 1. Notes are keyed by 16K bank + CPU address, so names, headers and
  trailing comments carry by physical range; build_source.py rejects notes that no longer sit on an
  instruction start, which is the check for stale notes over changed code boundaries (none failed).
- build_source.py: one change, it now skips generator region-comment lines ("; ...") inside before/game.asm
  (new in pass-002 for DR_StaleForeignImage).
- review.json: rebuilt with tools/update_review.py (scaffold untouched) and compared with the round-1
  review: 815 code units carried unchanged; 9 units changed only in extent because a neighbouring range
  became code (IrqHandler, ClearSlotScores, WaitPpuBufEmpty, WaitFrames, ResetEnvelope, DR_AdvanceCourse,
  CL_IsGuarding, CL_Bracket6, DV_OutcomeOnActor) and were re-read; 19 new units written in this pass.
  843 code units, all reviewed; all 5548 scaffold rows reviewed; editorial_audit pending.

### Rewritten / new commentary (the changed ranges)
- bank 7: IrqHandler header; IrqSplitDone ($C036), IrqReturn ($C048); IrqSplitHandlers now .word RTS
  table naming IrqSplitNoChange/IrqSplitScrollX1/IrqSplitScrollZero/IrqSplitScrollX2; SetFrameTimerWhenIdle
  ($C5F1), WaitFrames3 ($C61A), RenderingOn ($C62F), B7_QueueSfx ($F26A).
- bank 2: FS_MarkerXBase note, FS_QueueMeterRow ($95E8, stack-argument contract, cell encoding).
- bank 3: DR_SetVelocityBoth ($9179); DR_StaleForeignImage ($A60C-$AEF8) explained; boundary note for
  $AEF9-$B01D; bank intro and $8984 note updated.
- bank 5: CL_Bracket6Pairs (18 B) and CL_Bracket6Places (6 B); CL_Bracket6 header; CL_IsState2to4 ($8CBD).
- bank 6: DV_FailBelow30 ($AE0C); DV_OutcomeNearBar/DV_OutcomeRts made global (now shared across it).
- Removed stale "phase-1 Unknown"/"classified Data" notes for these ranges; uncertainties.md updated.
- Backlog defect fixed while inspecting the inventory (bank 2): FS_SparkSprite and FS_DrawBackground headers;
  FS_BackgroundMap (64 B) / FS_BackgroundMetatiles labelled from FS_DrawBackground's pointers; boundary note
  for the unreferenced 612-byte OAM-record block at $8A9D.

### Verification
- commentary.py verify -> ok, byte_identical true (commentary/verification.json, after the last edit);
  commentary.py read refreshed reading-after.asm.

## Round 3 commentary (2026-11-25) -- status: COMPLETE (current-source readability)

Input: pass-003 (maps/current.json == maps/pass-003.json), 100.00% classified. The map changed since the
round-2 baseline (pass-002): the round-2 commentary is preserved whole at analysis/round-2/commentary/
(diff -r identical) and this directory was re-prepared with `commentary.py prepare` (5551 scaffold rows).
notes/, tools/ and uncertainties.md carried from round 2; build_source.py accepted every carried note
(instruction-start check), so no note sat over a changed code boundary.

Changed ranges commented:
- bank 0 $811A/$817A/$81C3: AttractTitle_UnusedExpire, AttractTitle_UnusedExpire2, AttractAltScreen_UnusedExpire
  (unreferenced, branch-skipped timer-expiry fragments); the loops they split got descriptive globals
  (AttractTitle_TitleOuter/TitleFrame/TitleTimeout/PictureOuter/PictureFrame/PictureTimeout,
  AttractAltScreen_OuterWait/ScrollWait/Timeout) instead of auto names B0_Entry811A_*.
- bank 0 $8547-$85B3: FameTitleOffsets (6 B, entry 4 dummy), FameTitles (5 x 20 B text), FameUnusedSelfJump
  (3 B JMP-to-self data). CavesOfFame header: index = EventId-1, skips Clubbing (4), 6 restarts (resolves a
  round-1 uncertainty).
- bank 2 $8363: FS_UnusedEmphasisCycle (self-modifying PPUMASK emphasis cycler; writes plain $18 from ROM).
- bank 3: DR_ActionRiderY/X extents (83 entries each, "padding" tags annotated as table values),
  DR_DrawBothRacers ($860E); duplicate stacked pose-table comment merged (backlog defect).
- bank 7 IrqHandler header + uncertainties.md: L-015 CLI/IRQ-vector correction.
- L-016 provenance splits: no commentary claimed those ranges unexecuted; nothing else to change.

Review: tools/update_review.py; 857 code units all reviewed (839 carried identical, 4 re-read for extent/header
change: AttractTitle, AttractAltScreen, CavesOfFame, FS_FlushPendingSfxX; 14 new). 5551 rows reviewed.
editorial_audit pending. Verification: commentary.py verify ok, byte_identical true; reading copy refreshed.

## Round 4 commentary (2026-11-25) -- status: COMPLETE (current-source readability)

Input: pass-004 (maps/current.json == maps/pass-004.json == analysis/state.json), 100.00% classified; code identical
to pass-003, data kinds/splits/labels changed (analysis/round-4/changes.json). Round-3 commentary preserved whole at
analysis/round-3/commentary/ (diff -r identical); this directory re-prepared (`commentary.py prepare`, 4306 rows).
notes/, tools/, uncertainties.md carried; build_source.py accepted all carried notes.

Tool changes (commentary/tools only): build_source.py skips '; ====' propagation markers in *.auto.txt (they had been
emitted as stray headers) and rewrites `lda #lo/#hi` pairs stored to MtileMapPtr/MtileSetPtr as #<Label/#>Label
when the target is labelled. update_review.py also records data units that carry an explanatory header.

Changed ranges commented (all 73 records of changes.json checked against the rebuilt source):
- bank 0: TitleMapTop, TitleMapBottom, MenuLowerMapPlayers, NumPlayersMap, CharSelectMap, MainMenuMap, MenuLowerMap;
  sets MainMenuMetatiles, CharSelectMetatiles, TitleTopMetatiles, TitleMetatiles; notes for $A108 (unused map-like),
  $A8A1-$A9D8, $ADBF (unused records); Draw*Screens headers; misplaced bank-0 sound-driver header moved to B0_SndUpdate.
- bank 1: B1 SFX tables re-cut (streams $9AE2-$9C8A); MT_FieldMap, MT_StripMapA/B, MT_FieldMetatiles;
  MT_MsprFrames as 50-word table into MT_MsprCels ("generic data after the frame records" removed).
- bank 2: FS_FlameCels (false "no reference" removed), FS_PoseFramePtrs/FS_FlameFramePtrs as word tables (17/19),
  FS_PoseCels, $9420 note, FS_BackgroundMetatiles extent (79 records) + $9C25 note; B2 SFX tables, $ACE0 note.
- bank 3: DR_LightSfx (6 B), DR_CourseMap (extent assumed), DR_BackgroundMap, DR_BackgroundMetatiles (no longer under
  DR_MsprFrames_43), $95A1/$A60B notes; ActionRider trailing notes; B3 SFX tables, $B375 note.
- bank 4: SR_MsprFramePtrs as 45-word table into SR_MsprCels (former "not decoded" block), SR_CourseStrips,
  SR_BackgroundMap, SR_BackgroundMetatiles (no longer under B4_SfxStreamPtrs), $B348 note; B4 SFX tables.
- bank 5: B5 SFX tables (streams end $AD27), CL_ArenaMap/CL_ArenaMetatiles/CL_MsprCels, CL_MsprFramePtrs as 32-word
  table; $8074 dummy-read note.
- bank 6: DV_ActorDefaults 28 B, DV_RunMap, DV_SceneMapLeft/Right, DV_SceneMetatiles; B6 SFX tables; $AD22 note.
- bank 7: event intro maps (48-byte spacing explained), MenuMetatiles, $D881 note, CavesOfFameMap/Metatiles,
  $DBD8 note; StandingsNames/NameOffsets/Headers headers.

Review: 857 code units (829 identical; 28 had only header text/extent refreshed: Draw*Screens, SFX start routines,
B*_Reset marker removal, FS_SparkSprite and DrawMenuLowerScreen extents now end at the new data labels) + 189 data
units, all reviewed; 4306 rows reviewed; editorial_audit pending. Unresolved meanings: uncertainties.md "Round 4".
Verification: commentary.py verify ok, byte_identical true (after the last edit); commentary.py read refreshed.
