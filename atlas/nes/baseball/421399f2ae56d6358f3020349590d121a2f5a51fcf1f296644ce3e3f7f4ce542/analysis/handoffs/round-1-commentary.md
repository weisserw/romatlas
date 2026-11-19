# Round 1 — commentary handoff (2026-11-18)

Status: COMPLETE for current classified source (editorial audit reserved for final acceptance).

## Scope
- Prepared commentary with the toolkit (`scripts/commentary.py prepare`) from checkpoint
  pass-001 (maps/current.json == maps/pass-001.json). Whole ROM: 456 review entries,
  all marked reviewed with source locations; 270 routine/data units.
- Buildable source: commentary/source/game.asm (rendered from frozen before/ + notes by
  analysis/tools-commentary-r1/apply.py; notes in analysis/tools-commentary-r1/notes/).
- Reading copy: commentary/reading-after.asm.

## Readability assessment
- 0 address-only code labels remain (748 call/branch targets + 21 task entries named).
- RAM/hardware operands symbolized (~190 symbols); constants for wait lists, object/task
  slots, umpire-call codes, buttons; inline arguments shown as .byte/.word with task names.
- Every routine/task/data table has a block comment (purpose, inputs/outputs, formats);
  uncertain meanings are worded tentatively. Branch blocks have descriptive labels; inline
  comments are concentrated in NMI, scheduler, inline-arg helpers and the game loop.
- Weaker areas (mechanical labels, few inline comments): sound driver effect handlers
  (F9D0-FE3F), runner/fielder AI decision branches (D885-E296), CORDIC helpers.

## Verification
- `commentary.py verify`: ok, byte_identical true; game.asm sha256 d3e2436a1b01b352...
  (commentary/verification.json). input/game.nes unchanged.

## Examples
- Scheduler: TaskCreate/TaskSleep/TaskWaitFrame/TaskWaitNmi/TaskWaitList/TaskKill/TaskEnd.
- Game flow: StartMatch -> HalfInningStart -> NextBatter -> NextPitch (waits WAIT_PLAY_END).
- Play tasks: PitchTask, BatterTask, PitchResultTask, CatcherTask, RunnerTask, FielderTask,
  BallWatchTask, BallLandingPredictorTask, MessageTask (umpire calls FOUL/STRIKE/...).
- Field model: object slots (SLOT_UMPIRE..SLOT_UMP_LEFT), BasePoints, FieldStartTable.

## Corrections to phase-1 notes
- $62 strikes, $63 balls, $64 outs (not $64 balls).
- $0598-$059B = ObjRole of runner slots 6/8/A/C (RunnerRole), not a separate base table.

## Structural questions for redirect (evidence in source comments)
- SQ-1 $EAF8-$EAFE are inline args of WritePpuInline ($EA05, GetInlineArgs family, single
  caller $EAF5): count $06 + "CALLED"; $EAFF is the RTS the routine returns to. Map has
  them as one Code region decoded as asl/ora/asl $600D. Suggest re-annotating as Data
  (inline args) + Code RTS. Source already emits them as bytes (byte-identical).
- SQ-2 Unknown $DC91-$DCB9: coherent routine at $DCA2 creating tasks (entry $DC97 via
  TaskCreateForSlotX inline word) + helper $E4B8 (also Unknown); no caller found.
- SQ-3 Consumer-backed table bytes inside Unknown ranges: $DBF6-$DC05 (RunFrameFirst/
  Last, read by AnimateRunnerStep $DBE3/$DBE8), start of $DFEB (read at $DFEA+role by
  $DFDA), $E01D-$E02A (read at $E111, $E0D5, $E0D8, $E0E0, $E0ED, $DB6A). Bounds of the
  read extents need checking before splitting them out as Data.
- SQ-4 Coherent unreferenced stubs: $C959 (duplicate ClearBallVelocity shape), $EB3C and
  $EBB4 (frame-aligned sleep entries falling into TaskSleep), $E822 (alt entry of
  ScaleAlongVector), $E2A5/$E7BC/$E8B7/$E9C2/$FAAE (complete routines without callers),
  $E5F4 lone RTS. Only an entry-evidence decision is missing.

## Unresolved meanings
- See commentary/uncertainties.md (tentative names list, CORDIC scaling, sound mapping).

## Remaining gaps
- None for current-source readability. Whole-ROM classification still 98.20% (443 unknown
  bytes, unchanged). Final editorial audit not run (reserved).
