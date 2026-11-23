# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.
Round-1 copy: analysis/round-1/commentary/uncertainties.md; round-2 copy: analysis/round-2/commentary/uncertainties.md.

## Carried from round 1 (still open)
- Banks 3/4/5 inner routine names (card handling, hand analysis, roulette cursor/wheel, scene
  helpers) are working names from call/RAM summaries; headers use tentative wording.
- Blackjack decision handlers: resolved in round 6 (handler bodies match the menu order; see
  "Round 6 handoff gate" below).
- MarkOpponentResult bit meaning (01 visited?, 10 defeated when GameOutcome = 3) is inferred.
- PlayerMoney unit: new game stores 20000 while $200 is shown, so probably cents (unconfirmed).
- Sound engine command bytes $F1-$FE: lengths known (stream grammar), most meanings not decoded
  ($F3/$F4/$F6-$F9/$FB/$FD marked "not decoded" inline). Superseded: the whole $Fx set was
  decoded in round 8 (see "Round 8").

## Resolved or revised in round 2
- Roulette spin input: Start (B5_StartPressed) verified in the round-1 redirect (R1-L005).
- "TextPointerTable" was the card block table: renamed CardBlockTable (+ CardBlocks); the
  routine is DrawCardBlock (was PrintTextById). PrintTextList is DrawVramStructList (it runs
  VRAM-struct lists, the poker chip panel).
- Bank 4 segmentation: the 2-byte inline message callees (B4_PrintMessage*, B4_RunCursorMenu) are
  now treated as inline-parameter calls, so ~25 round-1 "routines" that were only continuations
  after an inline word became local labels inside their real routines. Renamed by behaviour:
  B4_BetPhase -> B4_OpponentBetTurn, B4_ShowdownPhase -> B4_PlayerBetTurn, B4_YesNoMenu ->
  B4_AskWhichCardsToHold/hold selector, B4_OpponentRaiseDialogue -> B4_CollectAnte,
  B4_ActionMenu -> B4_PreDealMenu, B4_RaiseOrCall -> B4_SelectMenuChoice, B4_InitStakes ->
  B4_RefundPotCapMoney, B4_CopyBetToPot -> B4_AnnounceRaise, B4_UpdatePot -> B4_PlayerSeeOrFold.
  Inline message strings are decoded in comments (game alphabet; $FE shown as '/').
- B5_OpponentIntro runs after every match (from MainLoop_GameFinished), so it is the post-match
  dialogue; the round-1 "introduction" wording was wrong. (Round 5: renamed B5_MatchResultScreen; see below.)
- Bank 6 "period tables" $8BDC/$8BE6 are indexed only by a note byte's high nibble; probably
  note-length tables (B6_NoteValueTableA/B). Unconfirmed. Superseded: now B6_NoteLengthsA/B
  (round 8); the real period tables are B6_PeriodTableLo/Hi (round 6).

## New in round 2
- zpDebugFlag ($9D): no ROM write found; when nonzero the poker Start menu offers FIRST / ESCAPE /
  WIN COMPUTER / WIN PLAYER (passed to B4_CheckMatchEnd) and menus accept any button. Probably a
  developer switch; not tested at runtime.
- Poker RAM names (PkrMinBet, PkrMaxBet, PkrAnte, PkrBetEntry, PkrOpponentBet, OpponentMoney,
  PkrAllInFlag, PkrFinalHandFlag, PkrHandCount; GameVar0714 = pot, GameVar0718 = player round bet,
  GameVar0726 = hold flags) are static readings supported by the decoded messages. Poker was not
  played naturally. Opponent decision codes (B4_OpponentDrawChoice / B4_OpponentBetChoice results
  0/1/2) are inferred from the messages printed.
- B5_UpdateWheelAttributes: attribute-row rewriting during vertical scroll; "highlight" meaning of
  RltHighlightBits and the attribute blocks is inferred.
- Bank 6 sound data split (B6_SoundHeaderPtrs/Headers/TrackStreams/InstrumentPtrs/Data) is from
  pointer extents; per-header layout and stream boundaries are not traced; B6_InstrumentData end
  ($9FFF) is inferred. Bank 6 $A000-$BDE6 holds game-alphabet text (country names, dialogue) mixed
  with undecoded data; left generic apart from the phase-1 text windows.
  Superseded: the stream boundaries were walked in round 3 (113 streams, 28 headers), $A000-$A1EB
  is the world-map marker sprite data (Round 6, B6_WorldMapMarkerPtrs), the world-map and
  portrait structures in $A1EC-$BDE6 were decoded in round 7, and the instrument/vibrato
  layout in round 8. The instrument list end is still not decoded.
- Low-confidence code fragments (no reference found, reachability unresolved): B2_UnusedReturn
  $BCFF, B4_UnusedCursorReset $B17D, B5_UnusedRoundEnd $90E5, B5_UnusedMouthFlip $A81D,
  B6_UnusedClearChannelField $87C8, MainLoop_UnusedEndingJump $C1B3,
  FetchTextChar_UnusedMarkMap $D209. CallFarInline_Return $E93B is code by stack contract only.
- Bank 4 $BB30-$BB31 (2 bytes) remain unclassified (R2-L021); the source note gives both readings.
- Generic data still without identified role: 184 blocks with mechanical notes (checks made,
  untraced indexed/pointer access caveat); 120 tables known only by readers carry mechanical
  bank-qualified labels (symbols.asm), layout not decoded.
- Bank-scoped RAM: $0732 is PkrMinBet+1 in poker but a separate 32-bit copy target in bank 5
  (RltVar0732); bank-scoped symbols are emitted for such reuse.

## Round 3 (pass-003)
Resolved / revised:
- Roulette outcome: RltWinningNumber ($0718 in bank 5) is picked at round start by
  B5_PickWinningNumber (was B5_OpponentBet) from per-tier candidate lists; RltWheelPos from
  B5_WheelPosByNumber. Bets are settled by B5_SettleBets (was B5_SettleMatchMoney) through
  B5_BetCellListPtrs / number lists, payout classes and multipliers. The round-2 names
  B5_ResolveBets / B5_PayoutStep / B5_PayoutMessages were wrong: they are the chip-panel bet
  builder (B5_ChipPanelResult, B5_AddChipToBet, B5_RemoveChipFromBet, B5_ConfirmBet).
- Bank-4 $A0C1 records are hand-name records: B4_DrawChipStack(s) -> B4_DrawHandName(s); hand
  ranks PkrOpponentHandRank/PkrPlayerHandRank/PkrShowdownWinner; hand helpers renamed
  (B4_CompareHands, B4_EvaluateHandRank, B4_CompareHighCards, B4_ExtractPairedRanks,
  B4_LoadHandCards (was B4_CardRank), B4_HandToSortedRanks, B4_SortFiveRanks).
- Bank-4 "extra scene" is the ending staff roll: B4_EndingCredits + B4_Credits* helpers,
  RunEndingCredits (was CallBank4Entry4); credits pointer table, staff lines, THE END banner.
- zpDebugFlag test menu confirmed at run time (round-2 redirect S14).
- Bank-6 sound data: 28 headers (13 music x 11 B, 15 effects x 10 B) and 113 stream entries
  rendered as labelled directives with the command grammar.
Still open / tentative:
- Hand orientation (hand 0 = opponent, 1 = player) follows the interleaved card layout and the
  drawing rows; static reading. B4_EvaluateHandRank's mapping of every pair count, and
  B4_CompareHighCards' X/Y exclusion inputs, are static readings. B4_HandAnalysis_9ADD /
  B4_HandAnalysis_9DAD (and its 2-byte table, which an index up to 12 would overrun) not decoded.
- RltActiveMarker/RltMarkerRow/RltMarkerCol: five bet markers inferred from the 5-entry arrays,
  the 5-bet limit and UpdateRouletteSprites; RltMaxBet source not traced; the net-win part of
  B5_SettleBets after the payout is summarised, not traced in detail.
- B5_RoundObjectInit/B5_RoundObjectItems field meanings (probably sprite objects/tile quartets).
- Strings without a mapped reader: B5_PanelLabelStrings ($A8B5), B4_LuckyStrings ($AD9C).
- B5_StartPressed: Start dispatch handler as verified in round 1 (R1-L005); its body only
  closes an open marker level; where the spin itself is triggered is not re-traced.
- Bank-6 stream overlaps: blocks run to the next entry; the note gives the real stop.
- Generic data without identified role: 170 notes (incl. phase-1 evidence notes) remain; 84
  mechanical BankN_Data_XXXX / Fixed_Data_XXXX / Bank4_Msg_XXXX labels (were 120).
- $BB30-$BB31 unchanged (R2-L021): unknown, both readings documented at the source note.

## Round 4 (pass-004)
Resolved / revised:
- $BB30-$BB31 is now classified Static code (R3-L027, confidence 0.5): rendered as
  B4_UnreferencedStateLoad (`lda zpLocal3`) with a note. No reference reaches it; whether it
  can execute is unresolved, and it is not claimed dead.
- $9DB2-$9DB5 is one 4-entry table (R3-L028): B4_SuitOrderTable (permutation 0,2,1,3), read by
  B4_SuitOrderLookup (was B4_HandAnalysis_9DAD) with index card / 13 = 0..3, so no overrun.
  Both callers discard the result.
- Bank-5 panel captions (R3-L029): B5_PrintMessageValue is B5_PrintPanelLabels (mask bit n ->
  record n of B5_PanelLabelRecords; $80 = YES NO); B5_PanelLabelYou; B5_PanelLabelStrings now
  have a reader. B5_PrintNumber is the matching amount printer B5_PrintPanelAmounts, and the
  former generic 16 bytes at $8803 are its B5_PanelAmountRecords (address, column, row).
  Bank5_Data_8268 is B5_PanelLayouts (5 x [B5_UpdateBetDisplay argument, label mask, amount
  mask]) used by B5_SayMessage.
- Bank 3 has the same panel mechanism: B3_PrintMessageValue -> B3_PrintPanelLabels
  (B3_PanelLabelRecords, B3_PanelLabelYou, B3_PanelLabelStrings incl. the STAND/HIT menu),
  B3_PrintNumberAt -> B3_PrintPanelAmounts (former generic $911A-$912D = B3_PanelAmountRecords);
  Bank3_Data_9270 = B3_OpponentNamePtrs + B3_OpponentNames; BjVar07C2 = BjOpponentName.
  Record bytes follow SetupTextWindow (pointer, column, row, width, height).
- B4_HandAnalysis_9ADD is the opponent's draw strategy B4_OpponentPickHolds (pairs, suit or
  run draws, random high cards; return code 0..3). Static reading.
- B4_CompareHighCards is read as a final suit tie-break.
Still open / tentative:
- B4_CompareHighCards: possible original bug (hand-1 half stores the card in zpLocal4, so its
  DivideMulti uses stale zpLocalE/zpLocalD and can divide by zero). The exact outcome and
  Api_DivideMulti's divide-by-zero behaviour were not checked at run time.
- B4_OpponentPickHolds return codes 1..3 are not traced into the betting code that may use them.
- Bank-5 panel records 0 and 1 are not selected by any B5_PanelLayouts mask seen; the $E0
  caption prefix glyph is unknown. Bank 3: the "MONEY" string at $9292 has no record; the
  phase-1 region boundaries $92B0/$92D0 cut through B3_PanelLabelStrings (structural, not
  changed in commentary); GameVar0714/BjVar079C roles in the amount records are tentative.
- B4_LuckyStrings ($AD9C-$ADC2) and bank-4 $A2DE-$A46B: no consumer found (round-3 search).
- Other round-3 open items unchanged. Counted with one rule (symbols.asm lines starting
  BankN_Data_ / Fixed_Data_ / Bank4_Msg_), mechanical labels are 104 (109 in round 3 by this
  rule; the round-3 figure 84 used a different count). "Generic data (" notes: 168 (was 170).

## Round 5 (pass-005)
Resolved / revised:
- Bank 3 $9270-$9974 now follows the pass-005 regions: B3_OpponentNames ends at $9290 and
  B3_PanelLabelStrings starts at $9291 ("MONEY" first, still with no reader). B3_TierParams
  (was Bank3_Data_92EE): 4 x 18-byte per-StakeTier records; the first amount is the opponent's
  starting money (B3_LoadTierOpponentMoney, was B3_CopyBytes8295; BjVar07A9 -> BjTierStartMoney).
  B3_MessagePtrs (4 x 38 words, one voice per opponent; immediates #<B3_MessagePtrs /
  #>B3_MessagePtrs, constants B3_MESSAGES_PER_TIER / B3_TIER_PARAM_SIZE) and B3_Messages.
  The false "no code reference" notes for these ranges are gone; the generic-window wording now
  says only that no classified code operand addresses the bytes directly.
- B5_OpponentIntro is the match result screen (B5_MatchResultScreen): captions "YOU'VE QUIT /
  LOST / WON THE GAME." by GameOutcome 1/2/3, then "PLEASE GO ONTO YOUR NEXT OPPONENT.", "BUT YOU
  STILL HAVE ENOUGH MONEY AND ANOTHER CHANCE." or "... THIS IS THE END OF THE STORY." (X = 0 ->
  game over). Tables B5_ResultFrameBox, B5_ResultCaptionPtrs/Captions,
  B5_ResultClosingLists/Lines/Text, B5_ResultPortraitFrames, B5_ResultPaletteByOpponent,
  B5_ResultGfxByOpponent. GameOutcome is 1 = quit, 2 = lost, 3 = won.
- Bank 2 attract text: B2_PrologueText/Ptrs (35 lines, after the title times out) and
  B2_EpilogueText/Ptrs (15 lines, X bit 7, after the final victory). B2_AttractScenePtrs
  ($B6EF, 4 x 2 words) points to eight 20 x 6 picture blocks that tile $B6FF-$BACE exactly
  (structure B2_AttractScenePictures); B2_AttractSceneAttrPtrs ($BACF) points to four 22 x 1
  attribute rows. This resolves the round-5 disassembly's open "$B6EF-$B702 layout".
- B3_PostGameScreen is the GAME OVER screen (observed in the artificial S16 scenario).
  B3_PrintAmount was misnamed: it is B3_BetAllMoneyMenu (BET ALL MONEY / FINAL HAND offer).
- StakeTier in game is 0..3 (0 = final match, after the local increment).
Still open / tentative:
- $BB1F-$BB4A (bank 2, 44 bytes between attribute rows 3 and 4) repeats row 3 and has no
  pointer; probably unused. B3_TierParams bytes 8..17 and the second amount (BjVar07AD,
  probably a betting limit) are not decoded. B5_MatchResultScreen's money thresholds
  (100 / 1,000 / 10,000) depend on OpponentResultBits bits whose meaning is not decoded.
- B5_ResultGfxByOpponent / B5_ResultPaletteByOpponent overlap by one byte if SelectedOpponent
  reaches 9 (not checked at run time).
- B3_GameOverBanner and B4_PasswordFieldEnds tile meanings were not checked against CHR data.
- (Round 6: resolved.) Api_BuildMetasprite's zpArg2/3 is an attribute lookup base indexed by
  tile code, so padding-address bases are legitimate; they are now named *SpriteAttrLookup_*.
- (Round 6: resolved by pass-006.) The bank-2 picture/attribute blocks are now one region each.

## Round 6 (pass-006)
Resolved or revised:
- Bank 6 $A000-$A1EB is the world-map marker sprite data, read by the fixed-bank WorldMapSetup
  after MapWorldMapBank maps bank 6 (the old header said bank 2). It holds 22 pointers, 22 metasprite
  headers, the tile bytes and an attribute lookup at $A14D. The 12-byte header fields other than
  the tile pointer are not decoded.
- The fixed-bank "scripted scene" routines ($F8DC-$FB65) were misread. They copy tier-dependent
  tile blocks from bank 0 (GfxSet18 area) into CHR RAM at $0200-$04FF. They are called from
  B5_SetupRouletteTable and are now named LoadTierTileBlocks, TierTileBlocks1-5,
  CopyTileBlockToChr, UploadChrRow, CopyRowToWorkBuffer and CopyRowBytes.
  - The visual meaning of the blocks is not checked.
  - The row-end test is inclusive, so a block's last row overlaps the next block's first row,
    and tier 0's last block reads ChrRowAddrTable+32. Probably an original bug (not run).
  - CopyRowBytes copies one byte more than the row width.
- The bank-6 tables B6_EnvelopeTableA/B are semitone period tables, now B6_PeriodTableLo/Hi.
  B6_ComputeEnvelope is now B6_ComputeNotePitch: octave*12 + note + transpose, then detune.
  B6_ComputeNotePeriod is now B6_GetNoteLength.
- Named sound RAM: SndOctave, SndTranspose, SndDetune, SndTrackPeriodLo/Hi, SndPeriodLo/Hi,
  SndScratch/2 and SndTrackCtrlReg. SndTrackCtrlReg is "probably" the $4000-style value.
Still open:
- Structural question for redirect (no map change made here): BuildMetasprite reads bank 6
  $A14D + tile code. For the marker codes this reads bytes inside the Padding regions
  $A14D-$A196 and $A1A1-$A1E0. Pass-006 region edges in $A000-$A14C also cut the pointer
  table, record and tile ends ($A02B, $A133, $A14C).
- The zpBankRequest value $0F stored once by fixed-bank setup code (masked to bank 7 by SwitchBank) is kept numeric.
- Immediate symbols were derived from stores/compares with established roles (immsym.py).
  Values used through other paths (for example, loaded into A and passed to a routine) stay numeric.

### Round 6 handoff gate (blackjack readability batch)
Resolved or revised (static readings from the handler bodies and the B3_TierParams bytes):
- The decision dispatch is in menu order: B3_DecisionStand (status 5), B3_DecisionHit,
  B3_DecisionSplit, B3_DecisionSurrender (halves the bet) and B3_DecisionDoubleDown. The rule
  checks were misnamed: B3_DealToActiveHand is B3_CheckCanDouble, B3_CheckHandTotal is
  B3_CheckCanSplit and B3_ReadFlag07B6 is B3_CheckCanSurrender. Their rule bytes are tier-parameter
  bytes 11-13 (BjSplitBanned, BjDoubleRule, BjSurrenderAllowed).
- B3_PortraitReaction is B3_ShuffleShoe. In blackjack CardDeckState is the shuffled shoe of card
  codes; BjVar06E1 was CardDeckState+1, the next card.
- B3_DrawCard card assist: a HIT (BjCardAssist) may swap in the next shoe card when only the first
  would bust. The chance comes from tier byte 17 (5/200/100/10 out of 256 for tiers 0-3). The test
  uses rank+1 without the ace/court adjustment, so it is approximate (as written in the ROM).
- Hand-record fields are now BJH_* constants (cards, total, card count, soft aces, status).
- Settlement: B3_UpdateBankroll / B3_FlagMatchEnd / SettleHand_Case* are B3_SettleSurrender,
  B3_SettleLoss, B3_SettleWin, B3_SettleBlackjack and B3_SettlePush. Payout = bet * factor / 2,
  with factors BjPayBlackjack 5 and BjPayWin 4. BjVar07CF was not a match-end flag; it is
  BjReactionSprite ($83/$84/$89).
- B3_ShowBlackjackBanner is B3_FlashBlackjackWin: a palette flash to white. The generic region
  $9260-$926F is the colour data of B3_WhitePalette.
Still open:
- Which B3_CompareHands results produce statuses 3 and 5-9. The settlement names follow the money
  flow only.
- Tier bytes 10 and 14-16 (BjVar07B3, BjVar07B7-B9) are not decoded.
- Which opponent reaction sprites the ids $83/$84/$89 show was not checked visually.


## Round 7
Resolved or revised by the pass-007 carry-forward and the backlog pass (static readings):
- Bank-6 world-map chain $A000-$BDE6: the former "Fixed_Data_* bank 7 table" labels are bank-6
  structures (B6_WorldMapLegSteps/LegDirs, LocationPos, PortraitLayoutA/B, Info/Name/LockedMsg/
  LocationMsg pointers and strings, PortraitChrPtrs, 11 B6_PortraitChr* sets, PortraitPalettes).
  The old "location name strip / status window" reading of WorldMapUpdateTitle/StatusFrame/
  UpdateStatus/DrawPanel was wrong: they are the portrait swap (WorldMapPortraitUpload,
  WorldMapPortraitShow), the info-panel sequencer (WorldMapInfoStep) and the portrait palette
  upload (WorldMapApplyPortraitPalette).
- Bank-5 $844D-$86BC are block lists of B5_DrawPanelList (was B5_UpdateBetDisplay); list entries
  are (block, screen row, column), with the row offset by the scroll (B5_MessageRowAddress).
  CopyBank3Record20 is CopyBoardTiles: it copies board tiles from NametablePicture1/2.
- The B3 "post-game" routines are the GAME OVER letter animation (B3_GameOverStepTimer,
  PlaceLetter, EraseCell, MoveLetters, LetterStep, PulsePalette; B3_PostGameDelay is B3_Times16).
- Poker AI: B4_RankCardsA is B4_OpponentMood; B4_OpponentDrawChoice chooses bets
  (B4_OpponentChooseBet), B4_OpponentBetChoice answers raises (B4_OpponentAnswerRaise) and
  B4_OpponentBluff is a fold check (B4_OpponentFoldCheck). Tier record B4_PokerTierRecords decoded.
- Roulette result animation: B5_ResultChips/ChipSlide/ChipFrame/ChipHide are the wheel spin
  (B5_SpinWheelIntro, B5_SpinToNumber, B5_SpinFrame, B5_SpinExit); $99A2-$9B45 is B5_WheelStrip.
Still open:
- B6_WorldMapLegSteps: which of the two step-count bytes is used for which part of the flight.
- B6_WorldMapInfoBlocks second word: one to three $FC/$FD marks; difficulty or stake rating not
  confirmed. Portrait pictures and the B2 menu option words were not checked against CHR data.
- B4_ScriptedCards: probably a rigged hand for the final opponent (hands 25/40, tier 0), not run.
- PkrHandFlag is compared with 2, a value no classified code stores.
- B4_OpponentFoldCheck counts the odd GameVar0726 entries; reading them as the player's hold
  flags is tentative. B4_OpponentChooseBet thresholds are not decoded in detail.
- B5_WheelStrip number order; header 0 of B5_SpinToNumber is probably the ball.
- No reader found: B4_UnreadRankValues ($9A1B), B4_SpareMenuStrings tail ($A03B-$A06C), "MONEY"
  at b4 $A00D, BlankTileRun ($F8C2), RomTitleFooter ($FFE0), PkrFinalOpponentMoney (write only),
  PkrPlayerWinStreak/PkrOpponentWinStreak (write only), PkrTierByte0 (write only).
- PasswordEntryChars ($052A) is written by B4_PasswordPutChar; its reader uses another base.

## Round 8 (pass-008)
Settled by the round-7 redirect (artificial scenario S17) and folded into the source:
- B4_ScriptedCards is the player's royal flush in spades (hands 25/40 of the final match);
  the terminating 0 is dealt as the ace without the used-card check, so a duplicate ace is
  possible. Natural reachability of hand 25 is still not demonstrated.
- PkrPlayerHandRank ($0756), PkrOpponentHandRank ($0755) and PkrShowdownWinner (1 = player)
  are no longer tentative; the odd PkrHoldFlags ($0726) entries are the player's holds.
- PkrHandFlag 2/3: no classified code stores them. The B4_PokerGameEntry branches for them pay
  the pot like fold outcomes ("I FOLD" for 2); probably unused, since folds are settled in the
  betting code. Indirect writes were not excluded.

Decoded statically this round (no runtime check):
- Sound engine: the whole $Fx command set, the music fade ($FE, B6_StartFade, SndFade*), the
  vibrato entries at B6_InstrumentData+111 and their phase pattern, legato mode (bit 7 of
  SndTrackActive), note lengths (B6_NoteLengthsA/B). No stream uses $FE; the round-3 stream walk
  took it as 1 byte, while the handler consumes 3 operand bytes (no effect on the current data).
  Sound-header type 2 (fade) is handled but no header uses it.
- Roulette cursor: RltMoveState (was RltRoundState) and the motion/scroll helpers; the grid
  coordinates were swapped in the old names (RltMarkerRow is horizontal, now RltMarkerGridX). The
  exact board geometry behind B5_CheckCursorMove's tests and the unit of RltViewTop/Bottom
  (probably tile rows of the tall board) are not mapped.
- Blackjack bet entry: BjMinBet/BjMaxBet, BjReshufflePoint, BjAllInMinRounds/Chance (BET ALL
  MONEY always accepted after 10 rounds, since the chance byte is 1), BjLastBet, BjFinalHandFlag.
  B3_DrawHandCards' mode argument (card backs / face up / from the newest card) is a static
  reading.

Still open (address-based names, each commented with its known mechanics; all but PwdVar057B
were named in round 9, see below):
- RltVar06CB (5 entries; selects the re-entry Y of markers during a board scroll, also used by
  B5_BallStep and B5_HighlightNumber), RltVar06D5, RltVar072D (ball / match-end sequence).
- BjVar0761 (comparison flag in B3_CompareHands/B3_CompareTotals), BjVar0767 (shared by the
  hand-drawing mode and the split check).
- GameVar07D0 (bank-5 multi-use 32-bit scratch, BjCardAssist in bank 3), PwdVar057B (written,
  never read).

## Round 9 (pass-009, final editorial acceptance)
Named this round from the code (static readings, no runtime check):
- RltMarkerCovered (was RltVar06CB): per marker, index + 1 while a later bet marker shares its
  board cell. B5_OffsetCoveredMarkers (was B5_HighlightNumber) fans such markers out by 4 pixels
  diagonally (B5_ApplyCoverOffset / B5_RemoveCoverOffset, was B5_HighlightPatternA/B; the four
  B5_Highlight{On,Off}{A,B} helpers are 4-pixel moves). The old "number highlight" wording was
  wrong: the routines move marker sprites only.
- B5_ScrollBoardHome (was B5_MatchEndSequence) scrolls the board back to Y 0 after the spin;
  RltHomeScrollStep (was RltVar072D) is its +2/-2 step, RltMarkerInView (was RltVar06D5) the
  per-marker visibility kept by B5_UpdateMarkerInView (was B5_BallStep); B5_ShiftMarkersUp/Down
  were B5_DecBallState/IncBallState. Nothing there involves a ball.
  - Mechanically odd but kept as read: the seam jump goes to Y $EE of the first nametable; only
    RltBetWonFlags 0-3 are tested for sound $0D; while the step is negative a newly visible
    uncovered marker enters at Y $F8, but a covered one still at $E4/$EC.
- BjDoubledFlag (was BjVar0761): set for a double-down hand (status 6); B3_CompareTotals then
  writes status 8 for a win. The full B3_CompareTotals status table is in its header; both
  hands bust gives status 6 (push) as coded. The writer of status 3 was not identified.
- BjCardTemp (was BjVar0767): two unrelated uses (draw-mode flag; split pair rank, 0 = ace,
  which makes B3_DecisionSplit give split aces one card each). No single meaning.
- RltScratch (bank 5, was GameVar07D0): short-lived 32-bit scratch.
- Password buttons: B4_PasswordInput acts on the lowest set pad bit; Start (B4_PasswordStart, was
  B4_PasswordType) asks for validation, Select (B4_PasswordSelect, was B4_PasswordMove) leaves,
  B erases, A types. The button order follows ReadJoypadSimple's standard shift order.
- R8-L044 applied: the three dangling prose names were fixed; danglingnames.py now reports only
  deliberate "Named X before round N" history notes and the B4_Msg wildcard.
Still open:
- PwdVar057B: five bytes set to $61 by B4_PasswordIntroAnim, never read by classified code.
- B5_MatchEndDialogue message texts ($08+class, $0F, $1B, $1C) are not decoded in place.
