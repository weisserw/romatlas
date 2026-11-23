# Round 2 / redirect handoff (attempt r02-a1)

Status: **complete** (connected avenues closed or deferred with reasons; not terminal).
Checkpoint: pass-002 (maps/pass-002.json sha256 a228e814...0d36 == analysis/state.json).
Classification unchanged: 131,086 / 131,088 (99.9985%); unknown = bank 4 $BB30-$BB31 (R2-L021).
No map/ROM/source/toolkit changes. Research: analysis/redirect/r02-a1/ (findings.json, unknown-ledger.json,
sound-streams.json, static_observed.json, tools/, frames/); runtime evidence traces/redirect-r02/
(gz snapshots + coverage). Gameplay states are artificial (money patches); see scenarios S13/S14.

## Scope
Commentary structural questions 1-5, weakest evidence (R2-D02 generic windows, R2-D01 low-confidence
fragments, bank-6 streams, $9D, R2-L021), R2-L022 roulette contract and the ending.

## Findings
1. **R2-L022 resolved (verified).** Roulette winning number = $0718 (0..36, $FF = 00), picked at
   round INIT before betting by B5_OpponentBet $9425 (RandomBelow over the $A8F6 tier lists); $0719 =
   $947D[n+1] wheel position. Bets: $06F0 count, $06F1 5-byte [cell, amount] records. Start runs
   B5_ResultSequence $9B46 -> B5_SettleMatchMoney $9D8A, which matches $0718 against the cell lists
   reached via $9ED1+2*cell (adc #$D1/#$9E at $9DBC). Start timing cannot change the outcome; the
   round-2 $06D2 test patched the wrong byte. Patch test: $0718/$0719 = 5/9 with a 1-18 bet -> payout 1:1.
2. **Natural ending reached (S13).** Bet on the preset number (27, 35:1) broke King at roulette;
   blackjack and poker stages won after King money patches ($07A9/$075B, $075B = $1000); RunPokerGame
   returned GameOutcome 3, FinalMatch fell through $C1FB -> FinalMatch_Ending $C1FD (executed) ->
   ending cutscene. $C1B3 is not on this path.
3. **Coverage audit** (4 runs, 7,037 opcode addresses): 0 opcodes in Data/Padding/unknown (no
   conflict with R2-D02 or any generic data); 167 Static code regions observed (2,164 opcodes); none
   of the 7 low-confidence fragments executed.
4. **$9D debug flag confirmed** (S14): with $9D != 0 Start in the poker pre-deal prompt opens
   FIRST/ESCAPE/WIN COMPUTER/WIN PLAYER and ends the match via B4_CheckMatchEnd; ignored when 0.
5. **Role/extent proofs** for generic data (kind stays Data): bank-5 roulette tables (exact ends at
   code), bank-4 message/menu tables + staff credits/THE END banner, bank-6 28 headers + 113 streams
   tiling $8DE7-$9F6A exactly (command grammar derived from the reader). Correction: R1-L020 text
   windows id2725 $A218, id2726 $A96F, id2727 $A9EF are roulette number lists, not text.
6. StakeTier semantics: FinalMatch writes $FF, each game entry increments it -> in-game 0 (King)..3.

## Ready leads (analysis/leads.json; classification gain 0 B each)
| ID | Target | Recipe |
| R2-L023 | b5 $947D-$94F2, $952A-$960D, $9EB8-$A327, $A88F-$AA74 roulette tables; fix ids 2725/2726/2727 | re-annotate roles/evidence along stated boundaries; commentary renames (B5_PickWinningNumber, B5_SettleBets, B5_SpinStart, RltWinningNumber $0718, RltWheelPos $0719, RltBetCount/RltBets) |
| R2-L024 | b4 $A06D-$A248 menu strings/chip records/tier message pointers; $B7C5-$B9B3 credits + banner | re-annotate roles of R2-D02 ids 2811/2812/2813/2815 + neighbours |
| R2-L025 | b6 $8CC2-$8DE6 headers, $8DE7-$9F6A 113 streams | tools/sndwalk.py --json; per-stream roles; document grammar |
| R2-L026 | 167 Static code regions observed (artificial scenario) | tools/covcheck.py on the four cov files; provenance update |
R2-L022 verified; R2-L021 attempt appended (no new evidence).

## Deferred / rejected avenues
- Remaining R2-D02 / generic windows without anchors (bank 2 $B5A3-$BB03 attract blocks, bank 3 $9330-$9410,
  bank 5 $846D-$861A, $99DA-$9AFA, $AB2F, $BB2A, bank 6 $A201-$BC8B): immscan2.py lists 118 immediate
  anchors, many constants; only the ones above were verified. No code conflict found (coverage, control
  targets). Diminishing prospect: roles only, no classification change.
- Low-confidence fragments: no reference, no execution in any run; static evidence unchanged. Missing
  fact: a caller/pointer. Natural ending shows $C1FD is entered by fall-through, so $C1B3 stays unreferenced.
- Full credits scroll on the natural path (S15): time; already visited artificially (S12).
- Exact debug-menu choice->code mapping: cursor rendering ambiguous; not structurally needed.

## Rejection review (R2-L021, the only unknown)
- Claim: $BB30-$BB31 (A5 7F) is code (dead `lda $7F`) or data. Decisive evidence: bytes sit between
  `rts` $BB2F and JSR entry $BB32 whose first instruction overwrites A; ROM-wide word, target-1 and
  immediate-pair searches find no reference; never executed in boot/gameplay/final/ending/debug runs.
- What it rules out: a known caller, dispatch or pointer; it does not rule out an unreferenced
  leftover instruction or a 2-byte constant.
- Competing interpretation: dead load left by an edited routine vs. an orphan data pair; neither has
  a consumer, format or repeated pattern to discriminate.
- Result: inference insufficient either way; keep unknown. Concrete revised test only with new
  evidence (a consumer reading $BB30/$BB31, or a computed jump/rts-pointer to $BB30).

## Recommendation
Run **disassembly round 3** to apply R2-L023..R2-L026 (role/provenance precision; 0 classification
bytes), then commentary (renames above, sound grammar, number-list corrections). R2-L021 stays
unknown, so whole-ROM classification cannot reach 100% without new evidence; after round 3 the
coordinator can consider final editorial acceptance with the documented 2-byte residual.
