# Round 1 / commentary handoff

Status: **complete** for the current-source readability gate (round 1). Classification unchanged
(87.31%, 16,641 bytes unknown; whole-ROM classification is a separate requirement). Editorial
audit (final acceptance) not run; review.json editorial_audit stays pending.

Baseline: prepared from checkpoint pass-001 (maps/current.json == analysis/state.json, sha256
78c994c3...1dcdd). Frozen: commentary/before/, commentary/reading-before.asm, baseline.json.

## Scope covered
All classified source: 1,414 code regions / 17,713 instructions (banks 0-7), all data/padding
regions, and boundary notes for every unknown range. review.json: 2,786/2,786 scaffold entries
reviewed (summary + source line), 1,458 routine units.

## Deliverable / reproduction
- Buildable source: commentary/source/game.asm + symbols.asm (+ local image.nes, layout.cfg).
- Generated from before/ + annotations by analysis/tools/round01-commentary/build.sh
  (gen.py, ann/bank0-7.txt, ann/ram.txt; review.py fills review.json). Edit ann/ and rebuild.
- Reading copy: commentary/reading-after.asm (refreshed).
- Verification: `commentary.py verify` -> ok, byte_identical true (commentary/verification.json,
  game.asm sha256 72cfeaae...85e0).

## Readability assessment
- Every routine has a descriptive label and header (purpose text + generated Calls/Reads/Writes
  lines); no address-only routine labels remain. Branch labels are <Routine>_Loop/_Next/_Return.
- All RAM/hardware operands symbolic (402 symbols). Inline parameters and 13 dispatch tables are
  rendered as .byte/.word with symbols. Referenced data blocks list consumers; 17 fixed-bank tables
  named; unknown ranges carry UNKNOWN boundary notes.
- Depth: fixed bank read in detail (FDS-BIOS-style library: VramStructWrite, FlushVramBuffer,
  LoadTileset, BuildMetasprite, JumpEngine-style DispatchJumpTable; text printer; inline-parameter
  SaveZpBlock/RestoreZpBlock; multi-byte Add/Subtract/Multiply/Divide/Compare; NMI job system;
  world map). Banks 2 (title/attract), 6 (sound engine) and the main flows of banks 3/4/5 read at
  routine level. Inner bank 3/4/5 routines named from call/RAM summaries with tentative wording.
- Remaining limits: immediates mostly numeric; mode-local RAM uses bank-scoped address names
  (BjVar0762, PkrVar0749, RltVar06B1, SndVar05B6) where meaning is not decoded.

Examples: MainLoop (game.asm:24522), SaveZpBlock (27793), DivideMulti (28268), PrintTextWindow
(25777), NmiGameHandler (31030), B3_BlackjackGame (2155), B4_DrawCard (11090: card = suit*13+rank,
CardDeckState used flags), B5_RouletteRoundLoop (16972), B6_SoundUpdate (21481).

## Key interpretations
- OpponentGameTable $C1B6: per opponent (1..9) stake tier + game (0 blackjack bank 3, 1 poker bank 4,
  2 roulette bank 5); matches the observed map order. Opponent 0 -> FinalMatch (all three games).
- Bank roles: 2 title/attract/map data, 3 blackjack + post-game screen, 4 poker + intro + password
  + travel scene, 5 roulette + arrival/opponent intro, 6 sound engine, 0/1 graphics data.
- PlayerMoney $050E (32-bit, new game 20000, probably cents); OpponentResultBits $0526-$0528
  (2 bits/opponent); NMI job numbers via NmiJobTable $EAA3.

## Unresolved meanings
See commentary/uncertainties.md (blackjack option order, roulette spin input candidate
B5_StartPressed, result-bit meaning, sound commands $F1-$FE, scene/post-game contents).

## Structural questions for redirect (evidence in source notes "Redirect question")
1. $C197-$C1A2 (bank 7, 12 B unknown, game.asm:24612): return continuation of DispatchSkipTable at
   $C18E; decodes as jsr $C208, jsr $C213, jsr $C346, txa, bne $C16F -> code.
2. $EB6D-$EB7A (14 B unknown, game.asm:31209): word table after jsr DispatchJumpTable in
   NmiRouletteHandler; 7 NMI job handler addresses -> data (jump table).
3. Bank 4 $8DDA-$90D0 (759 B, R1-L002, game.asm:9675): starts with an 8-word button dispatch table
   ($8D99,$8D99,$8DEA,$8E28,$8D99,$8D99,$9010,$8EC9) after jsr DispatchJumpTable; handlers from $8DEA.
4. Bank 4 $913D-$91D0 (148 B, game.asm:9799): 8-word table after jsr DispatchJumpTable in
   B4_YesNoMenu; targets $914D/$9165 inside the range -> table + handler code.
5. Consumer-anchored tables still unknown: BankSelectTable $E31B-$E322 (SwitchBank, X masked 0..7),
   OpponentGameTable $C1B6-$C1C7 (9 x 2 bytes), TextListTable start $CE1C, bytes $C1B3-$C1B5.
6. Existing leads R1-L001..R1-L008 unchanged; R1-L005 now has a candidate (Start -> B5_StartPressed).

## Remaining gaps
None for the round-1 readability gate. Final editorial audit, deeper semantic naming of bank 3/4/5
internals and immediate-constant symbols are optional follow-ups. Toolkit: no new toolkit issues.
