# Round 7 / redirect handoff (attempt r07-a1)

Status: **complete, proposal ready** (not terminal). Two ready leads:
- R7-L041 (medium): provenance plus role/meaning confirmations.
- R7-L042 (low): map evidence metadata.

Neither lead changes classification bytes or region boundaries. R1-L006 is closed as resolved.

Checkpoint: pass-007 (maps/current.json == analysis/state.json, sha256 bea35cf5...be7f), 131,088/131,088 classified,
0 unknown. These are unchanged: map, state, ROM, commentary source and toolkit.
- Research: analysis/redirect/r07-a1/ (findings.json, poker-static-observed.jsonl, static_observed.json,
  scripted_deal.json, deal40.json, showdown3.json, stale-evidence.jsonl, stale-refs.txt, frames/, tools/).
- Traces: traces/redirect-r07/ (three coverage files, deal25.snap.gz, afterdraw.snap.gz).
- Scratch /root/rascratch-r7r has been removed and my server stopped. No other process was touched.

## Scope
- I started from the commentary round-7 open items and the weak claims in the map:
  - code regions with confidence below 0.8;
  - generic or unresolved map evidence;
  - the old lead R1-L006, which had been left at "ready".
- **Main avenue:** the open item "B4_ScriptedCards was never run". There is a RAM recipe for it: the final-match
  poker snapshot f_pk0.snap already has StakeTier 0, so only PkrHandCount needs a patch.
- **Static check:** what PkrHandFlag value 2 or 3 would need.

## Findings
1. **B4_ScriptedCards is a royal flush for the player (artificial run, scenario S17).**
   - Setup: from f_pk0.snap, I wrote $0746 = $18, then pressed A five times (hold 4, 60 frames after each).
   - In the final match, B4_PickRoundMessage sets PkrScriptedDeal when PkrLeadHand = 1 and the hand count is 25 or 40.
     The trace shows $8577-$858D executed. I repeated the run with $0746 = $27, and $8594 executed.
   - B4_DealHand gives the scripted cards to the player's odd PkrVar071C slots (the zpLocal2 = 2 draws):
     - $97F4 ran 9 times, $97FA ran 5 times, and the terminator path $9802 ran once;
     - the reads at $984E-$9851 returned 0B 09 0C 0A.
   - The player gets Q, 10, K, J and A of spades (frames/deal25.png).
   - The terminating 0 is itself dealt as card 0, the ace of spades, and the deck-state check is skipped. In this run
     the King's third card was already the ace of spades, so the ace appears in both hands (frames/showdown3.png).
   - These are artificial observations only. In natural play the path needs a final-match poker game that lasts 25
     hands; that was not demonstrated.
2. **Showdown path observed.**
   - I patched King's money ($075B-$075E = 00 E1 F5 05), held all five cards and played to the showdown. The screen
     shows ROYAL FLUSH against HIGH CARD.
   - $0754 = 1, $0755 = 0 and $0756 = 10. So PkrPlayerHandRank is $0756, PkrOpponentHandRank is $0755, and
     PkrShowdownWinner 1 means the player wins. The pot was paid, followed by "PLACE AN ANTE.".
   - Pressing A on player card k sets GameVar0726[2k+1]. So the odd entries are the player's hold flags; this was
     tentative in the commentary and is now confirmed.
   - Coverage: 4,431 opcodes, none in Data or Padding, and no low-confidence fragment hit. 53 Static code regions
     were hit (571 opcodes, 1,662 B, 38 regions fully). No hit is misaligned with a region instruction start
     (tools/covalign.py).
3. **PkrHandFlag 2 and 3 have no writer.**
   - A scan of all map Code regions finds only five `sta $0757`, storing $FF, 0, 1, 1 and 0. There is also
     `sta $074F,x` with X < 4, and the fills write zero.
   - So no classified code produces 2 or 3. The branches after `cmp #$02` / `cmp #$03`
     (B4_RefundPotCapMoney / B4_InitMatchMoney with Y = $0F / $10) have no known selector.
   - This is a constraint only. They are ordinary code inside an observed routine and stay as they are. Other memory
     writes (indirect, or the debug menu) were not exhaustively excluded.
4. **Map evidence lags the commentary (R7-L042).**
   - 52 Data regions (2,544 B) still carry "Generic data" or "role unresolved" evidence (some at confidence 0.6-0.7),
     although the current source names their structures. Examples: 1429 Bank2_GfxDescriptorPtr, 2000
     GfxDescriptorTable2, 2178 PaletteSets, 2427 CardBlockTable.
   - 48 of them have a code operand inside the region.
   - The other 4 have different reasons: 3268 is unpointed by design, 2071 and 2446 are read through indexed bases,
     and 2192 has no reader.
5. **R1-L006 is resolved.** Every JSR/JMP call site of the bank-4 inline-parameter routines ($8B21, $9695/$96AC,
   $96E4, $971C/$9720, $975A/$9771) is followed by an inline Data region.

## Ready leads
| ID | Target | Recipe | Change |
|---|---|---|---|
| R7-L041 | 53 bank-4 Code regions (poker-static-observed.jsonl); Data 2187 $984E-$9852 | covalign.py on the three traces/redirect-r07 cov files; Observed in place or split (as R2-L026/R4-L033) with the S17 artificial caveat; commentary meaning updates | provenance 1,662 B; 0 class/boundary |
| R7-L042 | 52 Data regions (stale-evidence.jsonl) | stale.py + stale-refs.txt; refresh the evidence text and optional label; raise confidence only where the consumer is confirmed | metadata only |

Commentary updates that follow from R7-L041:
- **B4_ScriptedCards:** a royal flush in spades for the player; the terminator is dealt as the ace with no used check,
  so a duplicate is possible.
- **Settled orientations:** PkrPlayerHandRank, PkrOpponentHandRank and PkrShowdownWinner are no longer tentative, and
  the odd GameVar0726 entries are the player's hold flags.
- **PkrHandFlag:** 2 and 3 are never stored, as noted above.

## Rejected / kept (no new evidence)
- **Low-confidence Static fragments** (2761-2765, 2876, 2877, 3221, 1876, 3192): these are coherent unreferenced
  fragments that join established code. No new caller, conflict or execution was found; the poker runs hit none of
  them.
- **The 18 Padding regions and b2 $BB1F-$BB4A:** the r06-a1 review still applies, and its premises are unchanged.

## Deferred (reasons)
- **Semantic only, no structural effect:** B6_WorldMapLegSteps step bytes, the info-block $FC/$FD marks and the
  B5_WheelStrip number order. Each needs CHR tile/number identification or a travel-scene runtime; the r06 attempt
  did not reach the travel scene.
- **Readerless items** (B4_UnreadRankValues, the SpareMenuStrings tail, "MONEY" at b4 $A00D, BlankTileRun,
  RomTitleFooter): already Data. Operand scans (r03/r05/r07) find no reader, and a further search has no structural
  payoff.
- **S15 (full credits scroll):** unchanged; no structural question depends on it.
- **Full-ROM Instruction tracing** runs at about 1 frame/s. That is why I enabled coverage only after the draw. The
  deal phase is covered by range traces (scripted_deal.json, cov_deal25_ranges) and by the deal-40 coverage.

## Recommendation
Run **disassembly round 8** to apply R7-L041 (a provenance batch like R4-L033). Apply R7-L042 there too, or fold it
into final editorial acceptance. Then refresh the commentary for the touched regions and the meanings above, then run
final editorial acceptance.

Remaining open questions are semantic, with no identified structural consequence. A later redirect should not reopen
the items above without new evidence.

## Toolkit
No new toolkit issues found. The r05 socket-bridge workaround was reused, and full-ROM Instruction tracing is slow
(see above); that is a performance note, not a bug.
