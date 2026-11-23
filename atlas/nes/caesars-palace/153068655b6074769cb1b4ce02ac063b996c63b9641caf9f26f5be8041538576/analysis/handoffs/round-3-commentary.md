# Round 3 commentary handoff

Status: COMPLETE for current classified source (readability gate). Not a final editorial acceptance:
editorial_audit stays pending. Whole-ROM classification is a separate metric (round-3 map: 100.00%, 131088/131088).

## Scope and checkpoint
- Input: maps/round3-disasm.json = maps/current.json = analysis/state.json (sha256 24099db1...ed3), 1021 regions.
  The map changed since round 2, so the whole round-2 commentary was moved unchanged to
  analysis/round-2/commentary/ (only copy; round 1 remains at analysis/round-1/commentary/) and a fresh baseline
  was prepared (commentary/manifest.json, baseline.json, phase1-state.json, before/, reading-before.asm).
- Carry-forward by physical range: the round-2 instruction stream is identical; 34 new instructions at
  7:$CFCF-$D024. Names/headers/comments (tools/names.txt, bank:CPU keys) carry over; notes on the 6 split regions
  replaced by per-record comments; stale sizes/names fixed (see below). Reverified with a fresh build.
- Source: commentary/source/game.asm (20,802 lines), rendered by commentary/tools/apply.py from before/ + names.txt.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify .` -> byte_identical=true after the final edit
  (commentary/verification.json).
- Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).
- review.json: 1021/1021 scaffold entries reviewed (summary + source line), 712 units; editorial_audit pending.
- input/game.nes unchanged; no toolkit changes; analysis/toolkit-issues.md unchanged (no new issues).

## Readability assessment
- Code: all 769 code regions; 0 address-only routine names; all JSR targets have headers. New 7:$CFCF-$D024 named
  AdvanceTextCursor_Unref / PrintCharAtCursor_Unref (statically unreferenced windowed text printer using
  $0308-$0312, which overlaps AttrShadow). All 147 numbered Routine_LoopN labels now descriptive
  (SpinFourReels_PickStops/StepReels/CheckAllStopped, EvaluateLineTypeB_Pay100/Pay18/Pay14/Pay10,
  BlackjackDealerPlay_DrawTo17, PlayerHandTotal_SumAceHigh/SumAceLow, DrawRandomCard_CountFree/ModFree/FindNth,
  RouletteSpin_Mod38/Mod76, Reset_BankN_WaitVBlank ...). Remaining mechanical: 51 Routine_Loop (single loop of a
  short routine) and 66 Routine_Return (shared RTS).
- Data: all 234 data regions carry a record comment or note. L-007/L-008/L-010 records documented with format,
  bounds and consumer. Eight wrong round-2 sizes fixed (RouletteWheelFrames 196, SlotMessageFrames 210,
  ReelSymbolTilesA 48 + CoinMarkerTilesA, ReelSymbolTilesB 64, HoldCardTiles 24, NametableRowAddr 60,
  ClockHourTable 144, FourReelDenomText 12). Pointer tables renamed to FourReelStopTablePtrHi/Lo and
  ReelStopTablePtrHiA/LoA. 54 immediate pointer halves now symbolic (#<CardFaceTiles, #>RouletteWheelFrames, ...).
- L-009 incorporated: songs $81/$82 random floor tunes; MusicRestartId always 0 (restart path never runs); coin
  sounds; screens $10-$15 entrance pictures with vehicles (EndingTally, ShowFlashingMessage headers); screen $02
  developer placeholder; metasprite records are attributes then tiles (MetaspriteTable, DrawSprite*,
  WriteSpriteAttrs; frame strides of 3x2/4x2/5x2 smaller than records); ShowStartupScreen unused X and possible
  vestigial sprite-0 table 7:$E195.
- Correction vs the redirect note: coin-insert sounds are $04-$08 in code (slots $04+CoinsInserted after the
  increment = $05-$07; video poker $04+PokerCoinsBet before the increment = $04-$08), not $05-$09.

## Unresolved meanings (details in commentary/uncertainties.md)
ClockHourTable as temperature (inference); purpose of no-reader spans 6:$B554, 6:$85CE, 7:$C0D3 (14-frame figure),
$D0E4, $D0F0, $F398, $F3DC, $E195; slot symbol pictures vs ids; effect ids $0D/$10; screen $02 selector (none known).

## Questions for redirect
1. Optional: one capture of the clock screen's alternate phase to confirm ClockHourTable is a temperature
   (bounded: ShowClockScreen, wait for the phase flip).
2. Optional: render 7:$C0D3-$C286 frames (artificial OAM write) to name the unused figure; render reel strips to
   name slot symbols (old question 4).
3. Effect ids $0D and $10: a write watch on X at PlaySound would identify callers (likely none literal).
No structural (map) questions arose; nothing blocks the next phase.

## Remaining gaps
None in current-source readability scope. Final editorial audit (archive acceptance) is still pending.

## Links
commentary/source/game.asm, commentary/reading-after.asm, commentary/review.json, commentary/progress.md,
commentary/uncertainties.md, commentary/tools/ (apply.py with new I directive, review.py, names.txt),
analysis/round-2/commentary/ (preserved round-2 commentary), analysis/handoffs/round-3-disassembly.md.
