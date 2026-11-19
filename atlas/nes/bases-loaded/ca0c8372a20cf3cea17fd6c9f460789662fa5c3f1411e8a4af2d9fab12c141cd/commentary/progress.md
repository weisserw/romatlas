# Commentary progress

Prepared verified baseline (2026-11-19, round 3, from checkpoint pass-003, map sha256 4b3c1d85...,
1,497 scaffold regions). The complete round-2 commentary directory is preserved unchanged at
analysis/round-2/commentary/ (copied and diff-verified before the old directory was removed and
`commentary.py prepare` was re-run; TK-005 workaround). Round 1 stays at analysis/round-1/commentary/.

## Round 3 commentary pass (2026-11-19)

### Carry-over method
- notes/, tools/, uncertainties.md and this file were copied from the round-2 copy. source/ is
  rendered by tools/build.py from the new frozen listing before/game.asm (pass-003) plus the
  notes (keyed by CPU address per 32 KiB bank = physical range). The first rebuild was
  byte-identical with no dropped notes.
- Map changes: only three regions differ from pass-002 (bank 3 $9CAC Code, L-028;
  AnimationStepTimes $A398+5 and DrawStepFigure $A39D+31, L-029). Notes re-targeted:
  UnknownByte9CAC -> NullHook9CAC with a new comment; DrawStepFigure label moved to $A39D;
  AnimationStepTimes got a format note (entries 2-4 unread). build.py adds a generated
  "reachability unresolved" region note for the L-028 byte as for L-024 code.
- review.py now carries review credit from analysis/round-2/commentary/review.json: an entry
  is carried only if its physical range/kind is identical AND its rendered source text is
  identical; 1,475 carried, 19 same-range entries re-reviewed because their comments changed,
  3 new ranges reviewed (1,497/1,497 reviewed, 1,887 units). editorial_audit pending.

### L-031 corrections (redirect r2-a1 F3-F6)
- A button, not Start (Pad1NewPress AND #$80): SceneFrame17, SceneFrame18, and also
  SceneFrame15 (found by a button-bit scan of banks 0-3); EndingStartToTitle renamed
  EndingPressAToTitle. CpuOpenPinchMenu worded as a faked Start press (it writes $10).
- ScaleLaunchByHomeLead: caller bank-0 BeginBallInPlay $9B6C (computer game, top half);
  forces LaunchSpeed=$0400 while the home side leads; MulU8 operand mismatch only matters
  with game number 0. BeginBallInPlay mentions the call.
- Season-end stall (L-026): SceneInit17SeasonEnd and SaveGameState explain the probable game
  bug (music started, then the RAM clear zeroes DpcmRequest; scene change never performed);
  SceneFrame17 refers to it.
- PennantCounterA = game number (1 during game 1): ram.inc, NextScheduleEntry (now explained:
  entry (counter mod 33)/3 of the team schedule, then increment), DrawGameResultPanel,
  DrawNextGamePassword, DrawSeasonRecord, DebugSetSeason, password routines, bank-1
  CpuMenu5Pick/PlanCpuSubstitutions/CpuSubCountTable.

### Scope check (all classified source)
- No automatic Sub_/Lxxxx names in banks 0-7 (Data_9490 kept deliberately, explained); no
  "Mechanical summary"; nocomment.py finds no global code label without a comment in any bank;
  orphans.py lists only known RTS-split continuations (bank 0 $86E3/$86FA, bank 3
  $8397/$857D) that are explained by the routine comments. No stale "unknown byte" text.

### Verification
- commentary.py verify: ok, byte_identical true (commentary/verification.json); read refreshed
  reading-after.asm. Final editorial audit not run (reserved for final acceptance).

## Round 2 commentary pass (2026-11-19)

### Carry-over method
- source/ is still rendered by the commentary-owned tools/build.py from the frozen listing
  before/game.asm (now pass-002) plus notes/*.txt; notes and tools were copied from round 1.
  Notes are keyed by CPU address per 32 KiB bank, so they carry by physical range: the
  pass-002 map only added classifications inside previously unknown bytes, and the first
  build was byte-identical with no notes lost to the new boundaries.
- build.py now reports any note that no longer lands on an emitted item ("dropped note").
  This exposed about 20 round-1 notes (bank 0 reset/mode-restart area, propagated to banks 1-7)
  that had been silently dropped since round 1 because their addresses were off by 1-2
  bytes; they were re-targeted (ModeRestart at $8084 etc.).
- Other build.py changes: generated "reachability unresolved" note on every L-024 code
  region; offsets +3 into 4-byte tables fold into Table+3 instead of address-only labels;
  operands of instructions that host an inner entry stay literal (no misleading symbol).
- review.json: tools/review.py regenerated all 1,497 entries (1,886 units); 1,278 entries
  whose physical range/kind is identical to round 1 are marked as carried
  (review_round), 219 new or changed ranges are reviewed in round 2. editorial_audit stays
  pending (final acceptance only).

### Scope covered (all currently classified source)
- New code: all L-015, L-009, L-020, L-021, L-022 and L-024 code in banks 0-7 is named and
  commented; every global label now has a hand-written comment (no "Mechanical summary"
  remains; round 1 had about 540 of them, mainly banks 0, 1 and 3). No automatic
  Sub_/L/Data_ names remain except Data_9490 (bank 0, deliberately left address-based,
  explained) and the label UnknownByte9CAC on the single unknown byte.
- New data: AtBatStateTable, PicturePanelTiles/Attrs, ScreenImageA/B, CpuPitchPlanRows,
  sound tables, the L-023 tables and the L-024 tables are labelled with format notes.
- Older backlog fixed: stale "unclassified/unknown bytes" remarks, round-1 notes that never
  rendered, misleading names (see below).

### Names and interpretations changed
- L-025: $0418/$0419 = HomeScore/HomeInningRuns, $0426/$0427 = VisitorScore/
  VisitorInningRuns; AddHomeRuns/AddVisitorRuns and the inning-row drawers renamed. Team A
  is the home side throughout bank 3.
- $054F HitsBySide (hits) and $054D ErrorsAgainst (errors), from the R/H/E layout in
  DrawTotals; AddHit/AddError. PennantCounterA = games, PennantCounterB = wins.
  $0560 RecentStarters (pitching rotation), $074F/$0759 Home/VisitorPitcherStats,
  $0573 LastGameLost, $0788 SubCheckResult, $07FF DebugSceneIndex.
- At-bat states named: $17/$18 hit-by-pitch (scene 11), $22-$27 home run, $2A ball out of
  play at the plate end (two-base award), $30-$33 out/walk follow-up, $40 catcher's throw,
  $60-$62 third out and change of sides (with the game-end test).
- Cover routines named by base from the runner-path coordinates (CoverFirstBase ...
  BackUpHomePlate; CoverMoundPoint -> CoverLeftCutoff).
- Bank 3: "Intro*" figure routines renamed Figure*/DrawCentreFigure (they also run in the
  home-run step); ReturnToGame -> FindStarterIndex; DemoAutoPick -> RememberStarterA;
  ResetSelections -> RestRecentStartersA; Password* routines renamed by function
  (Decode*/Encode*, PasswordMoveCursor/PasswordChangeLetter; round-1 Up/Down names were
  swapped); TeamSlotTiles -> TeamLineupPositions; DrawTeamA/BBatterLine -> DrawHome/
  VisitorBatterLine (they were swapped); bank 1 CheckLeadForEnding -> ScaleLaunchByHomeLead.
- Developer shortcuts identified (all L-024 or behind lone RTS hooks): DebugSceneSelect,
  DebugLoadTeams, DebugShortcutNinth, DebugSetSeason/DebugEditSeason, DebugJumpToNinth,
  DebugSkipToLivePlay, DebugShowPassword*.

### Unresolved / questions for redirect
- L-029: bank 3 $A39C ($40) decodes as RTI at the start of L-024 code; probably the 5th
  byte of AnimationStepTimes (boundary question).
- L-030: ScaleLaunchByHomeLead passes MulU8Z-style operands to bank-0 MulU8 ($90B2).
- L-028: the single unknown byte $9CAC (explained in source).
- See uncertainties.md for the remaining semantic gaps.

### Remaining ranges
None for current-source readability. Branch labels remain condition-derived where no game
meaning is established (accepted in round 1); generic data blocks keep boundary/evidence
notes. Final editorial audit is not run (reserved for archive acceptance).

### Verification
- python3 /root/romatlas/scripts/commentary.py verify: ok, byte_identical true
  (commentary/verification.json). Reading copy refreshed with read (reading-after.asm).

## Round 1 commentary pass (historical, preserved in analysis/round-1/commentary/progress.md)

### How the source is maintained
- `source/` is the buildable commented assembly. It is rendered by the commentary-owned
  script `tools/build.py` from the frozen listing `before/game.asm` plus the editable notes
  in `notes/` (never by the toolkit generator). Edit `notes/*.txt`, then run
  `tools/check.sh [banks]` (build + byte-identical verify + list of remaining automatic
  labels). `tools/review.py` refreshes `review.json` from `tools/linemap.json`.
- Layout: `game.asm` (header, includes, CHR), `hardware.inc` (PPU/APU/MMC1 registers),
  `ram.inc` (RAM symbols and the Runner/Fielder/Marker record layouts), `prg32_0.asm` ..
  `prg32_7.asm` (one `.scope BankN` per 32 KiB PRG bank; CPU $8000-$FFFF).
- Notes syntax: `= ADDR Name` label, `; ADDR text` block comment, `> ADDR text` inline
  comment, `~ ADDR text` comment after the label, `include bankM LO-HI` reuse of identical
  copies (module copies, banks 5-7 = bank 4). `notes/bank10N.txt` cover the lower-half reset
  stub views assembled at $FFB0.
- Safety checks in build.py: labels may not shadow RAM/hardware symbols; cheap-local labels
  that would cross a global label fall back to numeric operands (reported as warnings).

### Scope covered (all classified source)
- Every 32 KiB bank 0-7, header, vectors and CHR. All automatic `Sub_xxxx`/`Lxxxx` routine and
  jump labels were replaced by descriptive names (1,054 names in bank 0, 578 bank 1, 180 bank 2,
  795 bank 3, 134 per voice bank). No address-only routine labels remain.
- RAM: ~300 named symbols plus record layouts (Runner, Fielder, Marker); all RAM operands
  used more than once are symbolic; hardware and MMC1 registers are symbolic. Jump tables are
  rendered as `.word Label` lists; data tables referenced by code carry labels (`Label+1`
  for high-byte reads).
- Hand-written explanations for the engine (reset/main loop/NMI/bank switching and mapper-
  write recovery, ball physics, fielding AI, runners, at-bat state machine, pitch/bat control,
  scoring, camera/strip streaming, VRAM queue, sprites), bank 1 (pitching view, scoreboard,
  computer opponent states, demo input), bank 2 (field map builders, sound driver), bank 3
  (scene engine, menu families, password chain, music starters) and banks 4-7 (voice engine,
  DPCM player). Routines without a dedicated comment get a generated "Mechanical summary"
  (calls, writes, reads, hardware) and sit under a family comment.
- Data, padding and unknown ranges: boundary notes with the phase-1 evidence; unknown ranges
  are marked as such ("kept as raw bytes"), with specific explanations where established
  (e.g. EnterPitchingMode at $8173 decodes as code -> L-009).
- review.json: all 1,465 scaffold entries have status reviewed, a summary and source
  locations; 1,596 units (routines/structures). editorial_audit remains pending (final
  acceptance only).

### Interpretations changed during the pass
- $2A/$2B is the fielding (pitcher) pad, $2C/$2D the batting pad (first guess was reversed).
- $F8/$F9 are the batter's box position/stance, $67 the swing timer (not pitch aim).
- Depth axis: home plate near BallX=$04E0, outfield toward 0; CheckBoundary uses $DD39 at the
  plate end and $DF59 at the outfield end.
- Module copies in banks 3-7 are at the same CPU addresses (not relocated); JSRs are stubbed.

### Remaining editorial work (not blocking readability, recorded for final acceptance)
- Branch-level labels are generated mechanically but carry their meaning: forward targets
  are named after the condition that branches there (`@ifBallFlagsZero`,
  `@afterCheckXCarrySet`, `@ifBelow` when no named state is involved), loops after the state
  they walk (`@loopRunnerFlags`, `@waitPPUSTATUS`, `@loopX`), plus `@return`/`@continue`;
  about 90 hand-chosen names in bank 0. A final editorial pass may still rename important
  branches by game meaning.
- Bank 1 computer-opponent states and bank 3 scene/menu routines have shorter, partly
  mechanical explanations; several bank-3 handlers continue into phase-1 unknown bytes.
- Generic data blocks are described only by boundary/evidence notes (allowed fallback):
  bank 0 $C567-$DC0F and $DC10-$E33E, bank 1 $947C-$A000/$A000-$D31B, bank 2 sound data
  $ADCA-$BFDC and the upper 16 KiB, bank 3 data islands.

### Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok, byte_identical true
  (commentary/verification.json). Reading copy refreshed with `read` (reading-after.asm).

## Final acceptance editorial audit (2026-11-19)

- Before any edits, the round-3 commentary/ was copied to analysis/round-3/commentary/; `diff -r` showed the copy identical.
- Edits went through notes/ and tools/build.py; tools/review.py was re-run against the round-3 copy.
  - Branch labels are now condition-accurate: 545 automatic labels renamed.
  - Process references are gone from the source.
  - Module-copy wording now says "no known entry" instead of "dead" (L-007). DeadModuleReset is renamed ModuleCopyReset.
  - Every lower-confidence region now carries a confidence/unresolved note.
  - ModeFlags and Port7000 documentation is updated.
  - The L-005 extents are documented in the source.
- review.json: 1497/1497 entries reviewed, 1889 units. editorial_audit is set to reviewed with its findings.
- `commentary.py audit` was run. Results are in commentary/audit.json and analysis/handoffs/final-acceptance.md.
