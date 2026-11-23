# Commentary progress

## Round 2 commentary (2026-11-22) -- complete for current source

Baseline: the map changed (round-1 baseline map sha 3249...cdcdb -> round-2 maps/round2-disasm.json =
maps/current.json = analysis/state.json, sha 6b48...e055, 928 regions). The entire round-1 commentary directory was
moved unchanged to analysis/round-1/commentary/ (single preserved copy), then `commentary.py prepare` built a fresh
verified baseline (before/, baseline.json, phase1-state.json, reading-before.asm, review scaffold of 928 entries).

Carry-forward: the instruction stream is identical (11,895 instructions at the same physical offsets; checked by
parsing both before/ listings), so every round-1 label, header and inline comment keyed by bank:CPU address in
tools/names.txt applies unchanged. Region notes keyed by physical offset were re-checked: 12 notes at offsets that
are no longer region starts were removed and the bank-leader / bank 0-5 notes rewritten. review.json is regenerated
from the source by tools/review.py (all 928 entries reviewed, 665 units; editorial_audit pending).

Method unchanged: tools/apply.py renders source/game.asm from before/game.asm and tools/names.txt; then
`python3 tools/review.py`, `commentary.py verify`, `commentary.py read`.

This pass:
- New typed data records (63 from L-002/L-006 plus the L-005 table splits) carry their phase-1 labels in source,
  placed with .org so they hold the bank CPU address; colliding equates are dropped in favour of the label.
  Format/role notes written for SongData0-3 (song table + event stream format), SfxScript04-22 (effect script
  format, per-id roles), Font1bppTiles, FloorMapHeader/BlockTiles/BlockAttrs/MapRows/ChrRle/Palette/Aux/MapSize,
  SpriteSetData, ScreenData00-29 (chunk format, chunk types, decoded sizes and the screen's use), bank leader bytes,
  shortened $FF fill after the screen records, 6:$8000.
- FourReelPayTable equate 66 -> 54 bytes; its former +54..+59 operands now read FourReelColumns/FourReelStripPtrs.
- All 147 mechanical Routine_SkipN labels replaced by descriptive names (B directives), e.g. PayOneCoin_CannotAfford,
  BlackjackDealerPlay_Push/PayBlackjack, RouletteFindBetSpot_DozenRow, EvaluatePokerHand_NotFlush, DrawReelA_Aligned.
  Misleading auto names fixed: FourReelSlot_Unref2_* (really SpinFourReels' evaluation) -> FourReelEval_*,
  CanAfford_IfPlayerCardCountNot02 -> CanAfford_No, RouletteFindBetSpot_IfCursorYBelow20 -> _TryChipBar.
- Auto-namer: compares of a routine's return value now produce names like IfPlayerHandTotalBelow16 instead of
  IfNotPlayerHandTotal; indexed operands prefer an exact variable (ChipCountsMinus1,y instead of ChipGlyphPtr+1,y).
- LoadSpriteAnimSet header corrected (sprite CHR goes to PPU $0000, not $1000).
- 7:$F3DE-$F8F7 note extended after a bounded operand rescan.

Scope covered: all 928 regions (768 code, 142 data, 18 padding); 1,576 labels; no Skip labels, no address-only
routine names, no numeric RAM/IO/ROM operands. Remaining mechanical names: 198 Routine_LoopN backward targets and
63 Routine_ReturnN shared RTS labels (conventional, documented in the file header).

Verification: `commentary.py verify` byte_identical=true after the final edit (commentary/verification.json);
reading copy refreshed (commentary/reading-after.asm).

Deferred / unresolved: see uncertainties.md and analysis/handoffs/round-2-commentary.md.
