# Round 1 commentary handoff

Status: COMPLETE for current classified source (not a whole-ROM or editorial-acceptance claim).

- Baseline: maps/current.json (= maps/round1-disasm-v2.json) + analysis/state.json, frozen in commentary/
  (manifest.json, baseline.json, phase1-state.json, before/, reading-before.asm).
- Source: commentary/source/game.asm (16.6k lines). Generated from before/ by commentary/tools/apply.py with
  the editable database commentary/tools/names.txt; review inventory by commentary/tools/review.py.
- Verification: `python3 /root/romatlas/scripts/commentary.py verify .` -> byte_identical=true
  (commentary/verification.json), run after the final edit.
- Reading copy: commentary/reading-after.asm (refreshed after final verify).
- Review: commentary/review.json -- 470/470 scaffold entries reviewed with summaries/source lines, 458 units;
  editorial_audit left pending (final archive acceptance only).
- Records: commentary/progress.md, commentary/uncertainties.md.

## Scope / readability assessment
- All 11,895 instructions (banks 0, 6, 7 + bank 1-5 tail copies), all data/padding regions and the unknown byte covered.
- Routines: 363/363 JSR targets plus region entries have descriptive names (no Sub_/address-only routine
  names); 458 header comments explain purpose, inputs/outputs, algorithms and data formats.
- RAM/IO: ~270 equates; no numeric RAM/IO/ROM operands remain. ~135 ROM table equates; every data region has a role note.
- Branch labels: ~225 condition-derived (Routine_IfCondition), ~186 mechanical Routine_SkipN forward targets,
  loops Routine_LoopN. Adequate but the SkipN labels are the main residual readability weakness.
- Examples: DispatchGameScreen $C438 (hotspot map), NmiHandler $F955 (VRAM job queue), ScreenDecode $E099 /
  DecodeNibbleRuns $E1A4 (nibble RLE), AwardChips 6:$BF2E / PayFromChips $D5E5 (chip economy),
  RouletteBetPayout $D370, TickPlayClock $FCA9 (clock + progressive jackpots), SoundUpdate 0:$80CB.

## Unresolved meanings (tentative in source; details in commentary/uncertainties.md)
Sound id roles; SndCmdNN instrument meaning; slot pay-table/symbol formats; blackjack surrender/insurance
payout details (LSR/ROL halving looks like an original bug); InfoScreenAtTopEdge content; message texts $10-$15;
consumers of the 32-byte table 6:$85CE and 15-byte table $E195.

## Questions for redirect
1. L-003: is 6:$8000 ever executed (bank-6 entry table starts at $8001)? Bounded check: trace any jump to $8000 with bank 6 mapped.
2. L-001 now has a documented contract: DispatchGameScreen $C438 takes Y = hotspot (0 quit, 1 clock, 3-7 poker,
   8-9 blackjack, 10-11 roulette, 12 Big Six, 13-23 slots); hotspot rectangles are HotspotX/W/Y/H ($CAA8..).
   Visiting each game would confirm sound-id roles, info screens and the ending (EndingTally $C72B via QuitDialog choice 0).
3. L-004: any indirect use of $C679, $E296 (animation builder with SprAnimPtrs), $E49F, $FB10?
4. L-002/L-005: slot pay tables (FourReelPayTable 6:$8CD3, EvaluateLineType*), ScreenTable $E015 entries and floor map format are now documented enough to drive data-format decoding.

## Remaining gaps
None in current-source readability scope. Whole-ROM classification still has 1 unknown byte; final editorial audit pending.

## Toolkit
No toolkit issues found in this phase (analysis/toolkit-issues.md unchanged).
