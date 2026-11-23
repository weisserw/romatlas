# Round 8 / commentary handoff

Status: **complete** for the round-8 current-source readability gate.
- Classification: 131,088 / 131,088 bytes (pass-008, 0 unknown). Classification and readability are
  separate claims; this pass changed no map or state.
- Editorial audit (final acceptance) was not run; review.json editorial_audit is still pending.

## Checkpoint and baseline
- The map changed from pass-007 to pass-008. These are identical: maps/current.json, analysis/state.json,
  maps/pass-008.json and commentary/phase1-state.json (sha256 569ab31c...bbdb). commentary/manifest.json has
  the same map/state sha.
- The round-7 commentary is preserved in full at analysis/round-7/commentary/ (diff -r identical). The round-1..6
  copies were not touched.
- A fresh baseline was prepared from pass-008: 2,590 regions (1,670 code, 902 data, 18 padding), byte-identical.
  commentary/progress.md and uncertainties.md were restored from the round-7 copy and extended.

## Scope
All classified source: 1,670 code, 902 data and 18 padding regions.
- Carry-forward by CPU address per bank with the round-7 generator, copied to analysis/tools/round08-commentary/.
  The first regeneration differed from the round-7 game.asm only in region header lines (provenance and the
  71 new split boundaries in bank 4), so the carried names, headers and notes needed no invalidation.
- The round-7 redirect meanings were folded in (below).
- Then a backlog pass: address-based RAM names, terse or wrong headers and misread routines in the sound engine,
  the roulette cursor code and the blackjack bet entry.

## Deliverable and reproduction
- Source: commentary/source/game.asm + symbols.asm, with the local image.nes and layout.cfg.
- Generator: analysis/tools/round08-commentary/build.sh (gen.py + ann/*.txt), then review.py.
  - Edits against round 7: round08-ann.diff, round08-gen.diff, round08-snd.diff (same directory).
  - gen.py changes: bytes inside bank-scoped 32-bit RAM variables resolve to name+k; SndTrackActive,
    SndStreamPtrHi/Lo and BjSplitHandStatus are rendered as base+offset arrays.
  - snd.py: stream command comments name the decoded commands.
- Reading copy: commentary/reading-after.asm (refreshed).

## Verification
- `commentary.py verify`: ok, byte_identical true (commentary/verification.json).
  - game.asm sha256 3d312a13460c1757d0b56656fedc4ba1dd368dafc883e3eeb33232b36ae51c34
  - symbols.asm sha256 8dfe7cac4129911aecf2e02db8f799353720c78987d53c3bdddb493cfbff16cd
- input/game.nes unchanged (sha256 335680fc...abfa).
- review.json: 2,590/2,590 scaffold entries reviewed (summary + source), 2,042 units.
- Gate scan: 0 raw-address operands, 0 routine headers without a description, 0 lines over 96 columns.

## Redirect meanings folded in (r07-a1)
- B4_ScriptedCards (data note) and B4_DrawCard 11588: royal flush in spades for the player; the terminator
  is dealt as the ace without the used check, so a duplicate ace is possible. Artificial run only.
- B4_CompareHands 12621: orientation confirmed (PkrPlayerHandRank $0756, PkrOpponentHandRank $0755,
  PkrShowdownWinner 1 = player).
- $0726 is bank-scoped: PkrHoldFlags (bank 4; odd = player's holds, confirmed) and RltOpponentBroke (bank 5).
- B4_PokerGameEntry 6680: PkrHandFlag payout branches; 2/3 never stored, probably unused fold paths.
  B4_InitMatchMoney / B4_RefundPotCapMoney -> B4_OpponentTakesPot / B4_PlayerTakesPot.

## Examples (game.asm line)
- **Sound engine:** B6_TrackControlCommand 22959 with all $Fx handlers named (B6_SndCmdF1_TriLinear ...
  B6_SndCmdFE_Fade); B6_WriteChannelRegisters 24219 (vibrato, register writes, fade); B6_StartSound 24632
  (the real entry; the old B6_StartSound was a per-track stop); B6_StartFade 24825; B6_ChannelRegOffset (was
  B6_Return, described as a do-nothing return). 26 sound RAM names decoded.
- **Roulette cursor:** B5_StepCursorMotion 19225, B5_CheckCursorMove 19346, B5_PlaceBet 18525,
  B5_ChipSelector, B5_BuildMarkerSprites (was B5_WheelAnimation). The marker grid names were swapped:
  RltMarkerRow -> RltMarkerGridX, RltMarkerCol -> RltMarkerGridY; RltRoundState -> RltMoveState.
- **Roulette settle:** B5_SettleBets 21108 (opponent-broke rule, payout cap).
- **Blackjack bet entry:** B3_ConfirmBet 2798 (was B3_ConfirmQuit), B3_SpecialBetMenu 2939 (was
  B3_PrintRoundResult), B3_DrawHandCards 3635 (was B3_AddCardValue), BjMinBet/BjMaxBet, BjReshufflePoint,
  BjAllInMinRounds/Chance.

## Unresolved meanings
See commentary/uncertainties.md, "Round 8"; earlier sections still apply. Main items:
- 9 address-based RAM names remain (was 59): RltVar06CB, RltVar06D5, RltVar072D, BjVar0761, BjVar0767,
  GameVar07D0 (multi-use scratch), PwdVar057B (never read); each commented with its known mechanics.
- Board geometry behind B5_CheckCursorMove and the unit of RltViewTop/Bottom; B3_DrawHandCards mode meanings
  (static reading).
- Sound: $FE fade and header type 2 are handled but unused by the data; instrument list end not decoded.
- Carried: B6_WorldMapLegSteps bytes, info-block $FC/$FD marks, B5_WheelStrip number order, readerless items,
  B3_CompareHands statuses, blackjack tier byte 10.

## Remaining gaps (editorial backlog for final acceptance; not blocking this gate)
- 1,160 of 2,554 labels are mechanical _Next/_Loop/_Return/_Case branch labels inside explained routines.
- Numeric immediates remain where roles are local (counts, message numbers, sound ids).
- Some older short headers outside the areas re-read this round remain terse but accurate.

## Deferred
- Final editorial acceptance: record editorial_audit, then `commentary.py audit`.
- Redirect round 8.

## Toolkit
No new toolkit issues found (analysis/toolkit-issues.md unchanged).
