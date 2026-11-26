# Round 2 — commentary handoff

**Status: COMPLETE** (current-source readability for checkpoint pass-002). editorial_audit stays pending
(final acceptance only). input/game.nes unchanged; no toolkit change; nothing published or archived.

## Scope
- Input: pass-002 (maps/current.json == maps/pass-002.json, analysis/state.json), 100.00% classified,
  0 Unknown bytes. Commentary changes no classification.
- Map had changed since the round-1 baseline (pass-001b). Previous commentary preserved whole at
  analysis/round-1/commentary/ (diff -r identical before re-preparation). commentary/ re-prepared with
  `commentary.py prepare` (fresh before/, baseline.json, phase1-state.json, review scaffold of 5548 rows).

## Carry-forward method
- commentary/notes/ (names, headers, trailing comments, RAM symbols) and commentary/tools/ copied from round 1.
  Notes are keyed by 16K bank + CPU address, i.e. by physical range; build_source.py refuses any note that no
  longer falls on an instruction start, so stale notes over moved code boundaries would fail the build (none did).
- build_source.py: now skips the generator's region-comment lines in before/game.asm (new in pass-002).
- review.json rebuilt with tools/update_review.py and compared with the round-1 review:
  815 units carried unchanged, 9 re-read (extent changed by adjacent new code: IrqHandler, ClearSlotScores,
  WaitPpuBufEmpty, WaitFrames, ResetEnvelope, DR_AdvanceCourse, CL_IsGuarding, CL_Bracket6, DV_OutcomeOnActor),
  19 new. Totals: 843 code units reviewed, all 5548 scaffold rows reviewed.

## Invalidated / rewritten
- bank 7: IrqHandler header; new IrqSplitDone ($C036), IrqReturn ($C048); IrqSplitHandlers emitted as a 7-word
  RTS table naming IrqSplitNoChange / IrqSplitScrollX1 / IrqSplitScrollZero / IrqSplitScrollX2;
  SetFrameTimerWhenIdle ($C5F1), WaitFrames3 ($C61A), RenderingOn ($C62F), B7_QueueSfx ($F26A).
- bank 2: FS_MarkerXBase; FS_QueueMeterRow ($95E8: stack-argument entry, 12-cell meter, tiles $21-$29).
- bank 3: DR_SetVelocityBoth ($9179); DR_StaleForeignImage ($A60C-$AEF8) documented (relocation +$1F21,
  corruption, dev-port polling, no references); boundary note $AEF9-$B01D; bank intro and $8984 note updated.
- bank 5: CL_Bracket6Pairs (18 B) + CL_Bracket6Places (6 B), CL_Bracket6 header; CL_IsState2to4 ($8CBD).
- bank 6: DV_FailBelow30 ($AE0C); DV_OutcomeNearBar / DV_OutcomeRts made global.
- Backlog defect fixed (bank 2): FS_SparkSprite / FS_DrawBackground headers, FS_BackgroundMap (64 B) and
  FS_BackgroundMetatiles labelled, boundary note for the unreferenced 612-byte OAM-record block at $8A9D.
- All stale "phase-1 Unknown" / "classified Data but decodes as code" notes removed; uncertainties.md updated
  (round-1 statements marked superseded, Round 2 section added).

## Readability assessment
All current classified source meets the round-1 standard: no address-only labels (Bn_Sub/Entry/Data...),
no @loop_/@skip_ auto labels, no numeric work-RAM operands; every code unit has a header or is a short,
descriptively named helper. Newly classified static routines state their mechanics and that no caller is
known (reachability not claimed). Examples: commentary/source/bank7.asm IrqSplitDone/IrqSplitHandlers,
bank2.asm FS_QueueMeterRow, bank3.asm DR_StaleForeignImage, bank5.asm CL_Bracket6Places.

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <ws>` -> ok, byte_identical true
(commentary/verification.json, after the last edit); `commentary.py read` refreshed commentary/reading-after.asm.
Source is maintained via notes -> `python3 commentary/tools/build_source.py` -> `tools/update_review.py`.

## Unresolved meanings (commentary/uncertainties.md)
- Which meter FS_QueueMeterRow drew; what A holds for DV_FailBelow30; group meaning of states 2-4
  (CL_IsState2to4); IrqSplitParam/Delay odd bytes and tail (only even offsets 0-$0C read).
- Raster IRQ path is static-only (never observed). Origin of DR_StaleForeignImage is inferred.
- Generic data: bank 3 $AEF9-$B01D, bank 2 $8A9D block (probable metasprite frames, no consumer),
  bank 7 $CEDB-$CEE7 dev-port stubs. Earlier items (L-006 controls, actor identities, QuitEventFlag) unchanged.

## New leads
None. L-004 (reachability), L-005 and S15 remain open as before.

## Remaining gaps
None in current-source readability. Still separate: final editorial audit/archive acceptance (pending).
Toolkit: no new issues found.
