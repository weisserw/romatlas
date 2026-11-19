# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.


## Final acceptance (2026-11-19)

- Lower-confidence classifications, stated in source as unresolved, not as live or dead:
  - 2,426 B of structure-only code with no known entry (0.75).
  - NullHook9CAC (0.6).
  - 78 pattern-based generic data regions, 74,925 B (0.75; with the other round-1 heuristic
    blocks about 74,961 B). Each carries a "data by byte patterns only" note.
  - AnimationStepTimes entries 2-4: no known code reads them; the extent is a layout inference.
- Module copies in banks 3-7 are not dead. Their bank-switch helpers run while those banks
  are mapped (traces; the bank-7 copy was not seen). Only the reset/main-loop part has no
  known entry outside bank 0.
- InlineDispatch table extents (L-005) were checked statically. The one residual is runtime
  only: carrying the field-only state $16 into the bat table would need a side change without
  a pitching-view set-up, and none is known.
- Port7000: no absolute-addressed read of $6000-$7FFF exists; its purpose is still unknown.
- These remain open (semantic only):
  - Scenes 3, 4 and 19.
  - The fourth-bounce AddError.
  - Voice sample content.
  - The CPU decision tables.
  - ScreenImageA/B.
  - RunsTally.
  - The $FFF0-$FFF9 bytes.

## Round 3 commentary (2026-11-19, baseline pass-003)

Resolved since round 2 (L-031, from redirect r2-a1):
- Bank 3 $9CAC is now Code (L-028, confidence 0.6), named NullHook9CAC; reachability
  unresolved (no caller/pointer). No unknown bytes remain in the ROM.
- Bank 3 $A39C is the 5th AnimationStepTimes entry (L-029); DrawStepFigure starts at $A39D.
  AnimationStepTimes entries 2-4 are never read by the known consumer (step index < 2); the
  five-byte extent rests on the sibling tables FigureStepTimesA/B (layout inference).
- ScaleLaunchByHomeLead (bank 1 $85BE) is called from bank-0 BeginBallInPlay ($9B6C): in a
  computer game, top half, a home lead forces LaunchSpeed = $0400 (L-030). The MulU8/MulU8Z
  operand mismatch only matters with a game number of 0 (not reached in pennant play).
- SceneFrame17, SceneFrame18, SceneFrame15 and EndingPressAToTitle (renamed from
  EndingStartToTitle) test the A button (bit 7), not Start.
- Season-end exits (scene 17) stall: music is started before SaveGameState clears
  DpcmRequest (L-026). Called a probable game bug in source; confirmed only in artificial
  runtime tests, not in a natural 132-game season.
- PennantCounterA is the game number (1 during game 1; advanced by NextScheduleEntry when
  the next opponent is picked), not games played.

Still open (lower-confidence classifications explained in source):
- L-024 static code without a known entry: about 2,426 bytes (confidence 0.75); each region
  carries a generated "reachability unresolved" note.
- Round-1 heuristic generic data: about 74,961 bytes (confidence 0.75), pattern-based; the
  source notes give boundary/evidence only, no format claim.
- The round-2 open items below (scene meanings 3/4/19, AddError on the fourth bounce,
  Port7000, voice meanings, CPU decision tables, ScreenImageA/B) are unchanged.

## Round 2 commentary (2026-11-19, baseline pass-002)

Resolved since round 1 (kept here for traceability):
- Jump-table extents: NmiModeTable, FieldTargetTable, AtBatStateTable (128 words, unused
  states -> AtBatStateNone), SceneFrameTable/SceneInitTable now end at classified code.
- EnterPitchingMode ($8173) is classified code (L-009). Bank-3 vectors $FF00-$FF2A are
  decoded and labelled (Svc*).
- Score labels were swapped (L-025): $0418 HomeScore, $0426 VisitorScore; TeamA = home side.
- Cover points are now named by base from the runner-path coordinates (home $04E0/$0200,
  first $046C/$026C, second $0400/$0200, third $046C/$0194).
- PennantCounterA = games played (corrected in round 3: game number), PennantCounterB = wins; $054F = hits, $054D = errors
  (line-score layout in DrawTotals); RecentStarters ($0560) = last two starters per team.
- Password: field layout decoded (team, opponent, 3+3 digits, checksum letter).

Open at the end of round 2 (see round 3 above for what was resolved):
- L-024 code (about 2.4 KB, mostly bank 3 developer shortcuts, disabled hooks and duplicate
  helpers) has no known entry; every such region carries a "reachability unresolved" note.
  Several sit behind a lone RTS that live code calls (disabled-hook pattern).
- Bank 3 $A39C: decoded as RTI at the start of an L-024 routine; probably the 5th byte of
  AnimationStepTimes instead (structure question for redirect).
- Bank 3 $9CAC: one unknown byte ($60) kept raw (L-028).
- Bank 1 ScaleLaunchByHomeLead ($85BE) FarCalls bank-0 $90B2 (MulU8) with MulU8Z-style
  operands; the effective computation is unclear (handicap interpretation tentative).
- Scene meanings: 0/1 home run, 2 relief-pitcher entrance, 10 pinch-hitter entrance,
  11 picture-panel scene after some hit batters (probably a bench-clearing fight),
  21 probably intentional walk; 3/4/19 only mechanically described.
- AddError's only caller is the fourth-bounce branch ($A3E6); why that is an error is open.
- Port7000 ($7000) writes: purpose unknown (L-014).
- Voice sample meanings remain inferred from context (L-013); the DPCM request $A0 at reset
  is only probably the title voice.
- Computer-opponent decision tables (CpuPitchPlanRows, CpuSwingChance*) are not decoded.
- ScreenImageA/B (bank 2 $95E3/$99E3) and the bank-2 upper data blocks: screens not identified.

## Round 1 commentary (superseded where noted above) (2026-11-19)

- Jump-table extents (phase-1 L-001/L-005): NmiModeTable ($84F2), FieldTargetTable ($87C1),
  AtBatStateTable ($B199), MarkerRoutineTable ($F43A), bank-3 SceneFrameTable ($83E8),
  SceneInitTable ($8710), StaticTextSets ($9017) are bounded heuristically; handlers reached
  only through unconfirmed entries sit in unknown bytes (L-011).
- EnterPitchingMode ($8173, banks 0-2) is unknown in phase 1 but decodes as a FarCall stub
  to PitchingModeInit; labelled and explained, classification left to redirect (L-009).
- Bank-3 vectors $FF06/$FF15/$FF18/$FF21/$FF27 are called from bank 0 but lie in unknown
  bytes; the names BankThreeTick/SceneInit/SceneFrame/OnLanding/OnContact describe the call
  sites, not decoded targets (L-010).
- Field axes: BallX (depth, plate near $04E0) and BallY (lateral, centre $0200) are inferred
  from base coordinates and screen projection; individual base/backup point names in the
  CoverPoint* routines are by bit position, not by confirmed base.
- TeamA/TeamB (bank 3, $06BF/$06C0): TeamA is the controller-1 side and uses
  HomeLineupIndex; whether that side is always the home team in pennant mode is unconfirmed.
- Voice sample meanings (DPCM request values) are inferred from the game situation in which
  they are requested; not listened to (L-013).
- Password encoding: only the routine structure is known (L-012).
- PennantCounterA/B ($0766/$0767), SideCounterA/B ($054D/$054F), RunsTally ($0459),
  GameStats ($0560): roles only partly established.
- $7000 writes at reset: purpose unknown (L-014).
- Computer-opponent state names in bank 1 follow the observed control flow; the exact
  decision tables (CpuSwingChance*, CpuPitchCountTable rows) are not decoded.
