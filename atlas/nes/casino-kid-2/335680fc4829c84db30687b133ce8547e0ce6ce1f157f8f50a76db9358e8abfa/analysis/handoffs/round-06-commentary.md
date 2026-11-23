# Round 6 / commentary handoff

Status: **complete** for the round-6 current-source readability gate.
- Classification: 131,088 / 131,088 bytes (pass-006, 0 unknown).
- Editorial audit (final acceptance) was not run; review.json editorial_audit is still pending.

## Checkpoint and baseline
- The map changed from pass-005 to pass-006. These are identical: maps/current.json,
  analysis/state.json, maps/pass-006.json and commentary/phase1-state.json (sha256 dd3b5f7b...4a6c).
  commentary/manifest.json has the same map/state sha.
- The round-5 commentary is preserved in full at analysis/round-5/commentary/ (diff -r identical).
- A fresh baseline was prepared from pass-006 (2,636 regions: 1,614 code, 961 data, 61 padding).
- This session resumed an interrupted worker. It did not re-prepare. It confirmed that the pipeline
  reproduces the verified source exactly before it made the gate edits.

## Scope
All classified source: 1,614 code, 961 data and 61 padding regions.
- Carried from the first half of the round (commentary/progress.md "Round 6 commentary"):
  - the pass-006 changed regions (bank-2 scenes, graphics sets/nametables, DPCM);
  - the bank-6 world-map markers;
  - the fixed-bank tier tile-block loader (was the misread "scripted scene" code);
  - the sound pitch tables;
  - 435 symbolic immediates;
  - wrapping of over-long lines.
- This session's gate pass scanned the whole source. It looked for wrong or neutral routine names,
  raw addresses, numeric field offsets, mechanical data labels, stale notes and long lines. All the
  concrete gaps it found were in bank-3 blackjack, and they were fixed (next section).

## Deliverable and reproduction
- Source: commentary/source/game.asm + symbols.asm, with the local image.nes and layout.cfg.
- Generator: analysis/tools/round06-commentary/build.sh (gen.py + ann/*.txt), then review.py.
  - The gate edits are in handoff-gate-ann.diff (in that directory).
  - gen.py has two small changes: a bank-3/4 CardDeckState+N operand rule, and no generic
    "layout not decoded" line for regions that lie wholly inside named structures.
- Reading copy: commentary/reading-after.asm (refreshed).

## Verification
- `commentary.py verify`: ok, byte_identical true (commentary/verification.json).
  - game.asm sha256 17c20b11ce59c6a4d57c3d36bc88ae011acf1e43f05227360a81040741352d0c
  - symbols.asm sha256 4833fb530193cc6adb8e91af2a537b2efe2be781fbfdfa2405ac14d6e36f0392
- input/game.nes is unchanged (sha256 335680fc...abfa).
- review.json: 2,636/2,636 scaffold entries reviewed (summary + source), 1,900 units.

## Findings and examples (game.asm line)
- **B3_ShuffleShoe 2239** was misnamed B3_PortraitReaction. It fills CardDeckState with the card
  codes 0-51 and makes three random-swap passes. In blackjack CardDeckState is the shoe;
  BjVar06E1 was CardDeckState+1.
- **B3_DrawCard 3262**:
  - The header was wrong; the routine does not call RandomBelow.
  - The ace/court value rules and soft-ace demotion are documented. Status 2 = two-card 21,
    status 4 = bust.
  - Card assist: on a HIT (BjCardAssist), the next shoe card can replace one that would bust. The
    chance comes from tier byte 17 (5/200/100/10 out of 256).
- **Decisions:** the dispatch order equals the menu: B3_DecisionStand, Hit, Split 4466, Surrender
  and DoubleDown.
  - Rule checks: B3_CheckCanDouble 4381 (was B3_DealToActiveHand), B3_CheckCanSplit (was
    B3_CheckHandTotal) and B3_CheckCanSurrender.
  - Rule bytes: BjSplitBanned, BjDoubleRule and BjSurrenderAllowed (tier bytes 11-13).
- **Settlement:** B3_SettleHand 4923 maps status to handler.
  - The handlers are B3_SettleSurrender, Loss, Win, Blackjack and Push.
  - .B3_SettlePayout 5136 pays bet * factor / 2 (BjPayBlackjack 5, BjPayWin 4). The opponent
    pays out as far as its money allows. PlayerMoney is capped at B3_MoneyCap.
  - BjVar07CF was described as a "match-end flag". It is BjReactionSprite.
- **B3_FlashBlackjackWin 5835** (was B3_ShowBlackjackBanner) flashes the palette to white. The
  former generic region $9260-$926F is B3_WhitePalette's colour data.
- **Hand-record fields:** BJH_CARDS/TOTAL/CARD_COUNT/SOFT_ACES/STATUS, used in 44 operands.
- **New data names:** B3_ChipValues, B3_HandCardColumn/Row, B3_SplitMarkerPos, B3_TablePalette*.
  The B3_TierParams note now decodes bytes 8, 9, 11-13 and 17.
- **Editorial:**
  - 24 of the 25 over-long lines were fixed.
  - The B4_RefundPotCapMoney prose cites B4_MoneyCapValue instead of $8329.

## Unresolved meanings
See commentary/uncertainties.md, "Round 6" and "Round 6 handoff gate". Earlier sections still
apply.
- Which B3_CompareHands results produce statuses 3 and 5-9. The settlement names follow the money
  flow only.
- Tier bytes 10 and 14-16 (BjVar07B3, BjVar07B7-B9) are not decoded.
- What the reaction sprites $83/$84/$89 show.
- Carried open questions:
  - the bank-6 marker attribute lookup reads into Padding regions (structural question for
    redirect, no map change);
  - the visual meaning of the tier tile blocks.

## Remaining gaps (known editorial backlog for final acceptance; not blocking this gate)
- 115 undecoded *Var#### RAM names. Each is commented with its owning game/bank.
- 82 mechanical BankN_/Fixed_ Data/Msg labels and 110 generic-data notes. They carry boundary and
  consumer notes.
- 1,126 of 2,554 labels are mechanical _Next/_Loop/_Case branch labels inside explained routines.
- About 3,490 numeric immediates.
- 1 line over 96 columns: the quoted inline message at Bank4_Msg_A300.
- Bank-6 stream command bytes are partly undecoded.

## Deferred
- Final editorial acceptance: record editorial_audit, then run `commentary.py audit`.
- Archive options.
- Redirect round 6.
