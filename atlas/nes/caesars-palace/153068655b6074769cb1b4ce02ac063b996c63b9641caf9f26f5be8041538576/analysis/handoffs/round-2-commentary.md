# Round 2 commentary handoff

Status: COMPLETE for current classified source (readability gate). Not a final editorial acceptance: editorial_audit
stays pending. Whole-ROM classification is a separate metric (round-2 map: 100.00%, 131088/131088 bytes classified).

## Scope and checkpoint
- Input checkpoint: maps/round2-disasm.json = maps/current.json = analysis/state.json (sha256 6b48e02b...e055),
  928 regions. The map changed since round 1 (3249...cdcdb), so the whole round-1 commentary was preserved
  unchanged at analysis/round-1/commentary/ (only copy, moved not overwritten) and a fresh baseline was prepared
  (commentary/manifest.json, baseline.json, phase1-state.json, before/, reading-before.asm).
- Carry-forward by physical range: the 11,895-instruction stream is identical between the two baselines, so all
  round-1 names/headers/comments (tools/names.txt, keyed by bank:CPU) carry over; region notes keyed by physical
  offset were re-checked and the 12 stale ones removed or rewritten. Review inventory regenerated from source.
- Source: commentary/source/game.asm (19,965 lines), built by commentary/tools/apply.py from before/ + names.txt.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify .` -> byte_identical=true after the final edit
  (commentary/verification.json, source sha f9b435f8...).
- Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).
- review.json: 928/928 scaffold entries reviewed (summary + source line), 665 units; editorial_audit pending.
- input/game.nes unchanged; no toolkit changes; analysis/toolkit-issues.md unchanged (no new issues).

## Readability assessment
- Code: all 768 code regions; 458 routine headers carried; 1,500 code labels with 0 address-only routine names.
  All 147 mechanical Routine_SkipN labels replaced by descriptive names (e.g. PayOneCoin_CannotAfford,
  BlackjackDealerPlay_Push, BlackjackDealerPlay_PayBlackjack, RouletteFindBetSpot_DozenRow,
  EvaluatePokerHand_NotFlush/NotStraight, AwardChips_NextDenom, DrawReelA_Aligned/Queue). Return-value compares now
  name the value (BlackjackDealerPlay_IfPlayerHandTotalBelow16). Misleading names fixed (FourReelSlot_Unref2_* ->
  FourReelEval_*, CanAfford_No, RouletteFindBetSpot_TryChipBar; ChipGlyphPtr+1,y -> ChipCountsMinus1,y).
  Remaining mechanical: 198 Routine_LoopN and 63 Routine_ReturnN labels (conventional, explained in the header).
- Data: every one of the 160 data/padding regions has a role note. New typed records carry their map labels
  (with .org CPU addresses) and format notes: song record + event stream format (SongData0), effect script format
  (SfxScript04) with per-id roles, 1bpp font, floor map header/blocks/attributes/rows/CHR/palette/size, sprite set
  layout, screen chunk format and chunk-type targets (ScreenData00) with every screen's caller/use and decoded sizes.
- Corrections: FourReelPayTable 54 bytes (was 66); LoadSpriteAnimSet decodes sprite CHR to PPU $0000 (was $1000).
- No numeric RAM/IO/ROM operands remain.

## Unresolved meanings (details in commentary/uncertainties.md)
Screen $02 use; message screen texts $10-$15; songs $81/$82 entry (no literal caller); effect ids $04-$08/$0D/$10;
slot symbol/pay-line meanings; layouts of 6:$B0EB-$BAD8 blocks, metasprite definitions 7:$C049-$C286/$D084-$D12B,
7:$F3DE-$F8F7, tables 6:$85CE and $E195; 6:$8000 leader byte vs NOP prologue.

## Questions for redirect
1. How are songs $81/$82 started (computed id into SoundStart)? Bounded check: watch writes of X in SoundStart
   ($0:$8031) during the casino-floor scenario.
2. Is 7:$F3DE-$F8F7 read through a computed index (e.g. NametableRowAddr with a large Y, or AttrShadowPtr)?
   Bounded check: read watch on $F3DE-$F8F7 during floor scrolling.
3. Which screen is $02, and what do message screens $10-$15 say? One frame capture per id would settle it.
4. Slot symbol ids (EvaluateLineType*/FourReelEval_*) vs pictures: one capture of each reel strip would name them.

## Remaining gaps
None in current-source readability scope. Final editorial audit (archive acceptance) is still pending.

## Links
commentary/source/game.asm, commentary/reading-after.asm, commentary/review.json, commentary/progress.md,
commentary/uncertainties.md, commentary/tools/ (apply.py, review.py, names.txt), analysis/round-1/commentary/
(preserved round-1 commentary), analysis/handoffs/round-2-disassembly.md.
