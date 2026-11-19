# Round 1 — commentary phase handoff (2026-11-19)

## Scope and checkpoint
- Input checkpoint: pass-001 (maps/pass-001.json, analysis/pass-001-verification.json).
  Commentary prepared with `scripts/commentary.py prepare` (frozen baseline commentary/before,
  commentary/baseline.json, phase1-state.json; map sha256 6a0d90d1...).
- ROM classification unchanged by this phase: 95.56% (313139 / 327696 bytes classified;
  14557 unknown). Classification and readability are reported separately.
- Covered: all classified source in all eight 32 KiB PRG banks, header, vectors, CHR, and
  boundary notes for every data/padding/unknown range.

## Verification and artifacts
- Fresh `commentary.py verify`: ok, byte_identical true (commentary/verification.json).
- Reading copy refreshed: commentary/reading-after.asm (36k lines).
- Buildable source: commentary/source/game.asm + hardware.inc, ram.inc, prg32_0..7.asm.
- Editable inputs: commentary/notes/*.txt (labels/comments per bank, ram.txt symbols and
  record layouts); renderer commentary/tools/build.py; `commentary/tools/check.sh` builds,
  verifies and lists any remaining automatic labels; tools/review.py fills review.json.
- review.json: 1,465/1,465 scaffold entries reviewed with summaries and source locations;
  1,596 units. editorial_audit left pending (final acceptance only; audit not run).
- Phase notes: commentary/progress.md, commentary/uncertainties.md.

## Readability assessment
- Complete for current source: no address-only routine labels remain (Sub_/Lxxxx all
  replaced); RAM/hardware/MMC1 operands symbolic (about 300 RAM symbols; Runner, Fielder and
  Marker record fields); jump tables rendered as `.word` label lists; data references labelled.
- Deep, hand-written explanations: bank 0 (reset, main loop, NMI handlers, bank switching and
  NMI recovery of interrupted MMC1 writes, VRAM queue, sprites, ball physics, bounces,
  boundary checks, camera and strip streaming, fielding assignment/AI, runners, tags/force
  outs, at-bat state machine, pitch/bat/swing control, scoring), bank 2 (field map builders,
  sound driver), banks 4-7 (voice engine, DPCM player).
- Shorter but complete coverage: bank 1 (pitching view + computer opponent states + demo) and
  bank 3 (scene engine, menus, password chain, music starters): every routine has a name and
  either a hand comment, a family comment or a generated mechanical summary.
- Branch labels: generated condition/loop names (`@ifBallFlagsZero`, `@loopRunnerFlags`,
  `@waitPPUSTATUS`); about 90 hand-named in bank 0.
- Remaining ranges: none unreviewed. Generic data keeps short boundary notes only (allowed):
  bank 0 $C567-$E33E, bank 1 $947C-$D31B, bank 2 $8400-$91BD and $ADCA-$FEFF, bank 3 data
  islands, bank 4-7 $83A2-$A022.

## Examples of named routines
Reset/ModeRestart/MainLoop, Nmi/NmiPlayFrame/NmiPitchFrame, FarCall, SetPrgBank,
NmiRecoverMapperWrite, DrawMetasprite, FlushVramQueue, QueueVramString, MoveBall, BallBounce,
CheckBoundary, LaunchBall, ComputeHitFlight, FindNearestFielder, CheckCatch, CheckTagOrForce,
RecordOut, RunAtBatState, AtBatWaitForPitch, CallPitch, CheckBatContact, UpdateBatterControl,
ReadPitchSteering, AddRunsToScore, CheckWalkOff, PitchingModeInit, DrawScoreboard,
CpuInputHook, InlineDispatch, CpuBatTrackPitch, SoundDriverTick, ParseSoundScript,
StartSound, BuildFieldRow, SceneFrame/SceneInit, PasswordDecode, PlayDpcmSample.

## Unresolved meanings (details in commentary/uncertainties.md)
Jump-table extents; handlers in unknown bytes; DPCM voice identities; password encoding;
PennantCounterA/B, SideCounterA/B, RunsTally, GameStats; whether TeamA (controller 1) is
always the home team; $7000 writes; exact computer-opponent decision tables.

## Structural questions for redirect (analysis/leads.json)
- New: L-009 ($8173 EnterPitchingMode stub is unknown but decodes as code), L-010 (bank-3
  $FF00-$FF27 vectors called from bank 0 lie in unknown bytes), L-011 (AtBatStateTable and
  scene handler tables: unresolved entries/islands), L-012 (password encoding), L-013 (DPCM
  request values -> samples, refines L-008), L-014 ($7000 writes).
- Refined: L-005 (InlineDispatch indices: CpuState & $1F etc.), L-007 (module copies in
  banks 3-7 are at the same addresses with stubbed JSRs, not relocated).

## Deferred work
- Final editorial pass: rename important branches by game meaning, deepen bank 1/bank 3
  prose, decode generic data tables when their consumers are confirmed.
- On later rounds: if maps/current.json changes, preserve commentary/ under analysis/round-1/
  before preparing a new baseline; carry notes by physical range (notes are keyed by CPU
  address per 32 KiB bank, so check moved/reclassified ranges before reuse).
- Toolkit: no new toolkit issues found in this phase (analysis/toolkit-issues.md unchanged).
