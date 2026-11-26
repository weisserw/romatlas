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
