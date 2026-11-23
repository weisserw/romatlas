# Commentary progress

## Round 4 commentary (2026-11-22) -- complete for current source

Baseline unchanged (maps/round3-disasm.json = maps/current.json = analysis/state.json, sha 24099db1...ed3, 1021
regions), so the round-3 commentary was resumed in place: no re-prepare, review credit kept, before/ untouched.
Applied ready lead L-011 (analysis/handoffs/round-3-redirect.md, analysis/redirect/r3-a1/findings.json):
- Clock: ClockHourTable -> TemperatureTable (record label renamed through a D directive; tools/apply.py now lets a
  D directive at a record's start rename the phase-1 record label), ClockColon -> ClockShowsTime,
  GetClockColonPhase -> GetClockDisplayPhase, DrawClockTime_AltPhase -> DrawClockTime_Temperature. Headers corrected:
  the screen alternates time/temperature every 4 in-game seconds (no colon blink), the dark palette applies outside
  7:30-19:29 (round 3 said between), ClockTextTiles layout (degree sign, colon, AM marker; no PM marker).
- Slots: symbol pictures in the symbol-tile/strip equates and the EvaluateLineTypeA/B and SpinFourReels headers with
  full pay rules; 35 labels renamed (incl. GetClockDisplayPhase, TemperatureTable) (e.g. EvaluateLineTypeB_CheckCherries/_PayTwoCherries/_CheckBarPairs,
  EvaluateLineTypeA_CheckRedSevens/_CountBars, FourReelEval_CheckGreenSevens/_FourBlackSevens/_CountSevens,
  SpinThreeReelsType*_EvalTopLine/_EvalMiddleLine, CountSymbolX_NoMatch); per-instruction comments on every symbol id
  load and payout; pay lines 0/2/4 = top/middle/bottom; FourReelPayTable rows; PayFourReelWin header.
- Sound: PlaySound header and SfxPtrHi/SfxScript13/16 notes state that ids $0D/$10 are not produced by any of the 54
  call sites (statically unreferenced, not called dead).
- 7:$C0D3: note extended with the inferred row widths and the tentative unused sprite slot-handle pull reading.
Verification: commentary.py verify byte_identical=true after the final edit; reading copy refreshed; review.json
1021/1021 reviewed, 713 units (PayFourReelWin header adds one); editorial_audit pending.

## Round 3 commentary (2026-11-22) -- complete for current source

Baseline: the map changed (round-2 baseline sha 6b48...e055 -> maps/round3-disasm.json = maps/current.json =
analysis/state.json, sha 24099db1...ed3, 1021 regions). The entire round-2 commentary directory was moved unchanged
to analysis/round-2/commentary/ (single preserved copy; round 1 stays at analysis/round-1/commentary/), then
`commentary.py prepare` built a fresh verified baseline (before/, baseline.json, phase1-state.json,
reading-before.asm, 1021-entry review scaffold). tools/ was copied from the round-2 copy and extended.

Carry-forward: the round-2 instruction stream (11,895 instructions) is unchanged at the same physical offsets; the
round-3 map adds 34 instructions (7:$CFCF-$D024). All names/headers/comments keyed by bank:CPU carry over. All
128 region notes still start regions; the 6 notes on split regions (6:$B0EB, 7:$C049, $CFB5, $D084, $D6FD, $F378)
were removed in favour of per-record comments, $DCA3 and $E195 notes rewritten. Reverified with a fresh build.

This pass:
- 77 L-007 / 13 L-010 / 5 L-008 records: every labeled record has a format/consumer comment (D directive printed
  above its label); 8 wrong round-2 sizes fixed (RouletteWheelFrames 196, SlotMessageFrames 210, ReelSymbolTilesA
  48, ReelSymbolTilesB 64, HoldCardTiles 24 (renamed from HoldLabelTiles), NametableRowAddr 60, ClockHourTable 144
  (renamed from ClockSecondsTable; probably a temperature table, mod-10 overread for hour 23), FourReelDenomText 12).
  Pointer equates renamed: FourReelStopTablePtrHi/Lo, ReelStopTablePtrHiA/LoA. Generic no-reader spans (6:$B554,
  7:$C0D3, $D0E4, $D0F0, $F398, $F3DC) have boundary/format/uncertainty notes.
- New code 7:$CFCF-$D024: AdvanceTextCursor_Unref / PrintCharAtCursor_Unref with headers, branch names and notes.
- L-009: songs $81/$82 (random floor tunes), coin sounds $04-$08 (code gives $04-$08, not $05-$09), screens
  $10-$15 entrance pictures with vehicles, screen $02 placeholder page, MusicRestartId always 0, metasprite order
  attributes-then-tiles (MetaspriteTable, DrawSprite* and WriteSpriteAttrs headers; frame strides noted),
  ShowStartupScreen unused X / possible vestigial $E195 table.
- New `I` directive in tools/apply.py writes immediate pointer halves symbolically (#<Table / #>Table): 54 operands
  (PrintNumber digit tables, CardFaceTiles, RouletteWheelFrames, coin message frames, ...). Exact table starts now win
  over offsets into an enclosing table (HandResultBlank instead of HandResultTexts+12).
- All 147 numbered Routine_LoopN labels replaced by descriptive names (e.g. SpinFourReels_PickStops/StepReels/
  CheckAllStopped, EvaluateLineTypeB_Pay100, BlackjackDealerPlay_DrawTo17, DrawRandomCard_FindNth,
  RouletteSpin_Mod38). Return1 suffixes dropped when unique. Duplicate RouletteSpot_Return_B6 -> RouletteSpotFound_Return.
- 24 previously comment-less table equates got role comments (coin marker positions, reel timing tables,
  pay-line positions, win text positions, font descriptor, ...).

Scope: all 1021 regions (769 code, 234 data, 18 padding); 1,673 labels; 0 address-only routine names, 0 numeric
RAM/IO/ROM operands, 0 data regions without a note or record comment. Remaining mechanical names: 51 Routine_Loop
(only loop of a short routine) and 66 Routine_Return (shared RTS), documented in the file header.

Verification: `commentary.py verify` byte_identical=true after the final edit (commentary/verification.json);
reading copy refreshed (commentary/reading-after.asm); review.json 1021/1021 reviewed, 712 units, editorial_audit
pending.

Deferred / unresolved: see uncertainties.md and analysis/handoffs/round-3-commentary.md.

## Final acceptance (editorial audit, 2026-11-22)

Scope: targeted checks on the round-3 map (unchanged), reusing round-4 review credit. Fixes: MergeRowAttrs_High/LowLoop1/2
-> _LeftScreen/_RightScreen (with comments), FontTileCount comment (47-tile count), tools/review.py word-boundary summary
clipping and audit preservation. editorial_audit = reviewed. `commentary.py audit`: ok, byte_identical=true, 1021 regions,
1203 units (commentary/audit.json). Details: analysis/handoffs/final-acceptance.md.
