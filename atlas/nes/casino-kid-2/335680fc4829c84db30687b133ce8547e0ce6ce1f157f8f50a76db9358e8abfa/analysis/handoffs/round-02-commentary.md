# Round 2 / commentary handoff

Status: **complete** for the round-2 current-source readability gate. Classification unchanged
(131,086 / 131,088; 2 bytes unknown, bank 4 $BB30-$BB31). Editorial audit (final acceptance) not
run; review.json editorial_audit pending.

Baseline: map changed (pass-001 -> pass-002). Previous commentary preserved in full at
analysis/round-1/commentary/ (diff-identical copy), then a fresh baseline was prepared from
pass-002 (state == current.json, sha256 a228e814...0d36).

## Scope
All classified source: 1,469 code regions, all data/padding regions, the unknown pair.
review.json 2,878/2,878 entries reviewed (summary + source), 1,515 units.

## Deliverable / reproduction
- Buildable source: commentary/source/game.asm + symbols.asm (local image.nes, layout.cfg).
- Generator: analysis/tools/round02-commentary/build.sh (gen.py + ann/*.txt; annpatch.py for
  patches; review.py for review.json). round01-commentary/ left as the round-1 record.
- Reading copy: commentary/reading-after.asm (refreshed).
- Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json;
  game.asm sha256 7f9472c5...8c67, symbols.asm fcf0a14b...142d).

## Readability assessment
- Every routine entry has a descriptive name and header; no "purpose not yet identified" headers
  remain. 0 raw RAM operands (397 RAM symbols incl. 32-bit and bank-scoped names).
- Bank-4 inline message words are rendered with the decoded string (e.g. "PLEASE BET.").
- Resumed after coordinator review: every operand now uses a symbol (0 raw hex addresses in
  operands, incl. inline words). Tables without decoded format get mechanical bank-qualified labels
  (BankN_Data_XXXX / Fixed_Data_XXXX, 120 incl. Bank4_Msg_XXXX message strings) defined in
  symbols.asm with their reading routines and "layout not decoded"; within named ranges operands use
  Name+offset. New names: Bank2_GfxDescriptorPtr, B4_MoneyCapValue.
- Every data piece carries a role note (named table, structure range, consumer, phase-1 signature,
  inline-operand use). The 184 remaining blocks without a known reader carry a mechanical note:
  size, bank/CPU range, what was checked (no operand base/inline/control target, outside named
  ranges), nearest named data, and the explicit caveat that indexed or pointer access is untraced.
- Limits: immediates mostly numeric; mechanical labels are address-based by design.

Examples (game.asm line): B4_PlayerBetTurn 11828, B4_RaiseSelector_Confirm 12242,
B4_SelectHoldCards_Toggle 12782, B4_PreDealMenu 11252 (debug menu), B4_CollectAnte 10702,
B5_UpdateWheelAttributes 19247, B5_AskQuit 22431, MainLoop_GameFinished 29808,
DrawCardBlock 30706 / CardBlocks 30759, CallFarInline_Return 36075, GfxSet07 note 80,
unknown note 18258.

## Key interpretations (new)
- Poker flow decoded from inline messages: ante (B4_CollectAnte), pre-deal Select menu
  (BET ALL MONEY / FINAL HAND), bet turns (B4_PlayerBetTurn chip selector: Up/Down chips, A confirm,
  B fold; min/max checks "NOT ENOUGH"/"TOO MUCH"/"GIVE ME A BREAK!"), see/raise/fold, hold
  selector (A toggles hold, B finishes), match end codes 1 final hand, 2 player broke, 3 opponent broke.
- zpDebugFlag ($9D, never written by ROM code) unlocks a poker test menu (WIN COMPUTER/WIN PLAYER).
- Card blocks: DrawCardBlock (card = suit*13 + rank, $34 back). DrawVramStructList draws the chip panel.
- B5_OpponentIntro is the post-match dialogue (returns X: 0 -> MainLoop_AfterGame / game over path).

## Unresolved meanings
commentary/uncertainties.md: opponent decision codes, poker counters $0747/$0748/$0753, blackjack
option order, bank-6 per-header layout and stream boundaries, B6 note-value tables (period vs
length), bank-6 $A000-$BDE6 contents, 184 generic data blocks (mechanical notes), low-confidence fragments.

## Structural questions for redirect (evidence pointers)
1. R2-L021 $BB30-$BB31 (game.asm:18258): both readings documented; needs a consumer or jump.
2. Low-confidence fragments: is any reachable? B5_UnusedRoundEnd $90E5, B5_UnusedMouthFlip $A81D,
   B4_UnusedCursorReset $B17D, B6_UnusedClearChannelField $87C8, MainLoop_UnusedEndingJump $C1B3,
   FetchTextChar_UnusedMarkMap $D209, B2_UnusedReturn $BCFF. Bounded check: execution coverage
   during poker/roulette/blackjack and the ending.
3. Bank 6 sound data: walk B6_SoundHeaderPtrs ($8C8A, 28 entries) -> headers -> stream pointers to
   split B6_TrackStreams ($8DE7-$9F6A) into streams; confirm B6_InstrumentData end ($9FFF) and the
   $A000 word table (stride 12, pointers $A02C..).
4. zpDebugFlag: runtime test (set $9D nonzero in poker, press Start) would confirm the test menu.
5. R2-L022 roulette result variable (unchanged; $06D2 now named RltAttrRowHigh, not the result).

## Remaining gaps
None for the round-2 readability gate (including the coordinator's symbol/generic-note
requirements). Optional: decode roles behind mechanical labels and generic blocks (bank 5/6),
final editorial audit.
