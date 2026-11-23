# Commentary progress

## Round 8 commentary (2026-11-23)

Baseline: the map changed from pass-007 to pass-008 (15 bank-4 code regions split at
instruction starts, 53 regions Static->Observed, 51 data evidence notes refreshed; no kind or
byte-set change). maps/current.json == analysis/state.json == maps/pass-008.json ==
commentary/phase1-state.json (sha256 569ab31c...bbdb). The whole round-7 commentary/ was copied
to analysis/round-7/commentary/ (diff -r identical; the round-1..6 copies are untouched), then
`commentary.py prepare` built the fresh baseline: 2,590 regions (1,670 code, 902 data, 18
padding), byte-identical. progress.md and uncertainties.md were restored from the round-7 copy.

Method: the round-7 pipeline was copied to analysis/tools/round08-commentary/. The first
regeneration reproduced the round-7 text exactly except the region header lines (provenance
Static->Observed and the new split boundaries), so no carried record needed invalidation beyond
the split regions, whose labels and comments are carried by CPU address. Edits are recorded in
round08-ann.diff, round08-gen.diff and round08-snd.diff in that directory.

Carried meaning updates (round-7 redirect, analysis/handoffs/round-07-redirect.md):
- B4_ScriptedCards: royal flush in spades for the player; terminator dealt as the ace without the
  used check (B4_DrawCard header).
- PkrPlayerHandRank / PkrOpponentHandRank / PkrShowdownWinner confirmed (B4_CompareHands).
- GameVar0726 split by bank: PkrHoldFlags (bank 4; odd = player, confirmed) and
  RltOpponentBroke (bank 5, decoded from B5_SettleBets).
- PkrHandFlag 2/3: never stored; the payout branches are explained in B4_PokerGameEntry.
  B4_InitMatchMoney / B4_RefundPotCapMoney renamed B4_OpponentTakesPot / B4_PlayerTakesPot.

Backlog reduced:
- Address-based RAM names 59 -> 9. Sound engine (24 names: note nibble, current music, effect
  priority, noise/triangle parameters, fade state, track flags, rest state, envelopes, loop
  counters, custom lengths, vibrato state); SndVar05D0/05D2/0650/0658 are now rendered as
  SndTrackActive+4/+6 and SndStreamPtrHi/Lo+4. Roulette view/motion state, chip cursor, round
  start money; blackjack bet limits, reshuffle point, all-in rules, round count, last bet,
  amount scratch, final-hand flag, split statuses; poker pot (PkrPot), player bet (PkrPlayerBet),
  hand cards (PkrHandCards). GameVar0714/0718 became bank-scoped (PkrPot/BjHandBet,
  PkrPlayerBet/BjMoneyBeforeBet; bank 5 shows RltAmountScratch+1/+2 and RltChipCursorSave).
- Sound engine headers rewritten: command dispatch, every $Fx handler named
  (B6_SndCmdF1_TriLinear ... B6_SndCmdFE_Fade), B6_TrackNote, B6_UpdateChannel,
  B6_WriteChannelRegisters (vibrato, register writes, fade). Misread routines renamed:
  B6_Return -> B6_ChannelRegOffset, B6_SilenceTriangle -> B6_StopMusicTracks, B6_StartSound
  (per-track) -> B6_StopMusicTrack and the real entry B6_StartSound, B6_ResetChannel ->
  B6_StartFade, B6_PitchSlideUp/Down -> B6_VibratoPeriodAdd/Sub, B6_SetupLoopCounter ->
  B6_LoopJump, B6_SndCmdF3Group -> B6_SampleLoopJump; note tables B6_NoteLengthsA/B. The
  stream comments now name the commands.
- Roulette cursor code re-read: B5_WheelAnimation -> B5_BuildMarkerSprites, the "wheel"
  scroll/step routines are the board scroll and marker motion (B5_StepCursorMotion,
  B5_CheckCursorMove, B5_CheckScrollDown/Up, B5_ScrollBoardDown/Up, B5_MoveMarker*,
  B5_CursorDown/Up/Left/Right), B5_PlaceBet, B5_CancelPendingBet, B5_ChipSelector (was
  B5_BetCursorLoop), B5_RoundLoopStep, B5_StartPressed header corrected.
- Blackjack bet entry re-read: B3_ConfirmQuit -> B3_ConfirmBet, B3_PrintRoundResult ->
  B3_SpecialBetMenu, B3_CopyHandSummary -> B3_ComputeInsuranceStake, B3_AddCardValue ->
  B3_DrawHandCards, B3_DrawCardFace -> B3_PrintHandTotal; B3_TakeBet / B3_BetAdjust /
  B3_BetAdjustClamp headers rewritten.

review.json: 2,590/2,590 scaffold entries reviewed (summary + source), 2,042 units;
editorial_audit pending (final acceptance only).

Verification: `commentary.py verify` ok, byte_identical true (game.asm 3d312a13...1c34,
symbols.asm 8dfe7cac...16cd). Reading copy refreshed. Gate scan: 0 raw-address operands, 0 routine
headers without a description, 0 lines over 96 columns.

## Round 7 commentary (2026-11-23)

Baseline: the map changed from pass-006 to pass-007 (1,338 B Padding->Data in 43 regions plus edge
splits; Code regions identical). maps/current.json == analysis/state.json == maps/pass-007.json ==
commentary/phase1-state.json (sha256 bea35cf5...be7f). The whole round-6 commentary/ was copied to
analysis/round-6/commentary/ (diff -r identical; the round-1..5 copies are untouched), then
`commentary.py prepare` built the fresh frozen baseline: 2,534 regions (1,614 code, 902 data,
18 padding), byte-identical. progress.md and uncertainties.md were restored from the round-6 copy.

Method: the round-6 pipeline was copied to analysis/tools/round07-commentary/ (gen.py + ann/*.txt,
review.py). Names, headers and notes are carried by CPU address per bank and re-validated by
gen.py; the first regeneration reproduced the round-6 text for every unchanged region. Edits are
recorded in round07-ann.diff and round07-gen.diff in that directory.

Carried and invalidated (pass-007 changes, analysis/handoffs/round-07-disassembly.md):
- Bank 6 $A000-$BDE6: 30 new regions named (B6_WorldMapMarkerAttrs now $A14D-$A1FC, 16 world-map
  tables, 11 B6_PortraitChr* sets, B6_WorldMapPortraitPalettes). The Fixed_Data_A1FD..BDBB
  "bank 7 table" labels and the "no operand points into it" notes are gone; 29 fixed-bank
  operands now name the bank-6 tables.
- Fixed world-map code re-read and renamed: WorldMapPortraitUpload, WorldMapPortraitShow,
  WorldMapInfoStep, WorldMapApplyPortraitPalette; WorldMapBasePalette, WorldMapPaletteString,
  WorldMapPortraitPalette; zpLocal usage on the map documented in WorldMapScreen.
- Sprite attribute tables named at their new regions: B5_SpriteAttrTableA/B,
  B5_ResultPortraitAttrs, MetaspriteAttrTable, B4_TravelSpriteAttrs (were *SpriteAttrLookup_*).
- Bank 5 $844D-$86BC: B5_PanelListPtrs and 8 lists; B5_UpdateBetDisplay -> B5_DrawPanelList,
  B5_ScrollMessageWindow -> B5_SaveBoardUnderWindow, CopyBank3Record20 -> CopyBoardTiles; the
  message-row helpers are re-described.
- B6_PeriodTableHi is 65 entries.
- Former-padding runs inside the new Data regions are described by role (attribute-0 cells, zero
  high period bytes, $FF blank-block tiles, blank/solid CHR rows); adjacent runs are merged.

Backlog reduced:
- Mechanical table labels: all 45 BankN_Data_/Fixed_Data_ labels replaced by named structures
  (B2 wipe palettes/menu options, B3 GAME OVER tables and message window, B4 poker tier records,
  frames, mood tables, logo, password alphabet/frame/intro sprites, travel palettes/sprites, B5
  message rects, chip tables, sprite sets, wheel strip, metasprite tables, arrival names, fixed
  password alphabet copy, chip amounts). The 25 Bank4_Msg_XXXX labels are named after their
  text (B4_MsgPlaceAnAnte, ...).
- Generic-data notes: from 110 to 0; the last ones were explained (B4_TableFrameStructs,
  B4_SpareMenuStrings, B5_WheelStrip, B4_UnreadRankValues, BlankTileRun, RomTitleFooter,
  MetaspriteTileListsA, ChipAmounts, B4_PokerShortStrings).
- RAM: 57 of 116 address-based names resolved (poker tier/AI/betting state, roulette markers and settle
  copies, sound envelope state, blackjack dealer record and flags, world-map palette string);
  59 *Var#### names remain (57 still "role not decoded").
- Routines re-read and renamed: B3 GAME OVER letter routines, B4 opponent AI (B4_OpponentMood,
  ChooseBet, AnswerRaise, FoldCheck), B4 password/travel/table routines, B5 wheel spin
  (B5_SpinWheelIntro, SpinToNumber, SpinFrame, SpinExit, BuildRamMetasprites,
  UpdateMarkerHeaders, LoadSpriteHeaders), B6 envelope (B6_LoadEnvSegment, UpdateChannel),
  B2_WipeLoadPalette.
- Two wrong immediates (#<VramBufLimit for a direction/count pair) reverted to numbers; long
  inline .word comments moved above their line (0 lines over 96 columns).

review.json: 2,534/2,534 scaffold entries reviewed (summary + source), 1,974 units;
editorial_audit pending (final acceptance only).

Verification: `commentary.py verify` ok, byte_identical true (game.asm ad2beb8a...6e63,
symbols.asm a8a11730...3e6c). Reading copy refreshed. Gate scan: 0 raw-address operands, 0 routine
headers without a description, 0 lines over 96 columns.

Open questions: commentary/uncertainties.md, "Round 7".

## Round 6 commentary (2026-11-23)

Baseline: the map changed from pass-005 to pass-006. maps/current.json, analysis/state.json and
maps/pass-006.json are identical (sha256 dd3b5f7b...6c). The whole round-5 commentary/ was copied
to analysis/round-5/commentary/ (diff -r identical; the round-1..4 copies are untouched). Then
`commentary.py prepare` built the fresh frozen baseline: 2,636 regions (1,614 code, 961 data,
61 padding), byte-identical.

Method: the round-5 pipeline was copied to analysis/tools/round06-commentary/.
- Names, headers, RAM symbols and data notes are carried by CPU address per bank (= physical
  range), and gen.py re-validates them.
- Invalidated and rewritten:
  - the pass-006 regions: bank-2 scene blocks, graphics sets, nametable pictures, B6 instrument
    data, chip panel structs, roulette sprite sets and DPCM samples;
  - the structure notes that described the old windows;
  - the dependencies found while checking them (world-map markers, the fixed-bank tile-block
    loader, sound pitch).

Changes:
- Bank 2:
  - one named block per region: B2_AttractScene0..3Upper/Lower and B2_AttractScene0..3AttrRow;
  - B2_UnpointedAttrCopy;
  - the "phase-1 windows do not follow" notes are removed.
- Former-padding runs inside the 22 graphics-set / nametable regions are now explained as
  empty/solid tile rows or repeated picture cells (gen.py note from pass-005 Padding).
- DPCM:
  - the DpcmSample1 note is corrected: the record-5 sample ends one byte past it, and the
    region is their union;
  - new DpcmFillFDF1.
- Evidence notes no longer show the "[Round 6: narrowed ...]" prefix or stale byte counts.
- Bank 6 $A000-$A1EB holds the world-map marker sprites: B6_WorldMapMarkerPtrs, Sprites, Tiles,
  Attrs, and B6_WORLDMAP_MARKERS.
  - WorldMapSetup operands are symbolic.
  - The MapWorldMapBank header is corrected (bank 6, not bank 2).
- Fixed $F8DC-$FB9F: the misread "scripted scene" family was renamed and explained as a
  tier-dependent CHR tile-block loader:
  - routines LoadTierTileBlocks, TierTileBlocks1-5, CopyTileBlockToChr, UploadChrRow,
    CopyRowToWorkBuffer and CopyRowBytes;
  - tables TierTileSourcePtrs and ChrRowAddrTable;
  - RAM zpTileRow*, zpTileCopySrc/Dst and zpChrRowTablePtr. zpScenePtr was wrong: it is two
    separate bytes.
- Sound:
  - B6_PeriodTableLo/Hi (were EnvelopeTableA/B), B6_ComputeNotePitch and B6_GetNoteLength;
  - 10 RAM names (SndOctave, SndTranspose, SndDetune, SndTrackPeriodLo/Hi, SndPeriodLo/Hi,
    SndScratch/2, SndTrackCtrlReg);
  - the B6_LoadVibrato header no longer cites $9FEE.
- BuildMetasprite: zpArg2/3 is documented as an attribute lookup base indexed by tile code.
  Its bases are named (*SpriteAttrLookup_*, WorkBuffer).
- Immediates: the new immsym.py writes ann/immsym.txt and ann/immconsts.txt, with 435 operands
  symbolic. They are:
  - PPUCTRL/PPUMASK/PPUSTATUS bit masks and PPU nametable/attribute/palette addresses;
  - bank numbers, NMI job numbers, zpOamBusy, zpPadMode, NmiHookEnable, GameOutcome,
    zpTextStatus, zpSoundRequest, APU/joypad register values;
  - RAM pointer pairs passed to FormatNumber, BuildVramStrings and FillMemory;
  - pushed return addresses (#>(Label-1)).
  - Numeric immediates went from 3,980 to 3,536.
- Over-long comment lines and .word lists are wrapped at 96 columns. 25 lines remain longer
  (end-of-line comments).

review.json:
- 2,636/2,636 scaffold entries are reviewed (summary + valid source), with 1,893 units.
- Every code region has units.
- editorial_audit is pending (final acceptance only).

Verification: `commentary.py verify` ok, byte_identical true (game.asm sha256 e04a7ba6...51,
symbols.asm 35c78377...b7). The reading copy is refreshed. Gate scan: 0 raw-address operands, 0
routine headers without a description.

Open questions and tentative readings: commentary/uncertainties.md, "Round 6".

### Round 6 commentary, handoff gate (resumed session, 2026-11-23)
The previous worker stopped before the handoff. The map is unchanged since its preparation:
current == state == pass-006 == phase1-state (dd3b5f7b...4a6c), and manifest.json has the same
map/state sha. The pipeline reproduced the verified source exactly (gen.py output == game.asm
e04a7ba6...), so the gate edits were made in ann/ and gen.py and then regenerated. The diff of
the ann/ inputs is in analysis/tools/round06-commentary/handoff-gate-ann.diff.

Gate scan over all source found these concrete gaps, all in bank-3 blackjack, and fixed them:
- Wrong or neutral names:
  - B3_PortraitReaction -> B3_ShuffleShoe;
  - B3_PlayerOption0/_Case1/_Case2/Option3/_Case4 -> B3_DecisionStand/Hit/Split/Surrender/
    DoubleDown (menu order confirmed by the handler bodies);
  - B3_DealToActiveHand -> B3_CheckCanDouble, B3_CheckHandTotal -> B3_CheckCanSplit,
    B3_ReadFlag07B6 -> B3_CheckCanSurrender;
  - B3_UpdateBankroll, B3_FlagMatchEnd and SettleHand_Case* -> B3_SettleSurrender/Loss/Win/
    Blackjack/Push and .B3_SettlePayout;
  - B3_ShowBlackjackBanner -> B3_FlashBlackjackWin;
  - dispatch locals .B3_TakeBet_IgnoreButton/_ResetBet and .B3_InsuranceAnswer.
  - Headers were rewritten for these routines and for B3_DrawCard (the card assist),
    B3_SettleHand (status map) and B3_TakeBet (exit codes).
- Hand-record field offsets are symbolic: BJH_CARDS/TOTAL/CARD_COUNT/SOFT_ACES/STATUS in
  ann/consts.txt, with 44 operands rendered through ann/immsym.txt.
- RAM:
  - BjVar06E1 is now CardDeckState+1, through a bank-3/4 rule in gen.py resolve();
  - b3:07D0 BjCardAssist;
  - BjSplitHands, BjSplitBet, BjSplitBanned, BjDoubleRule, BjSurrenderAllowed, BjPayBlackjack,
    BjPayWin, BjReactionSprite and BjBetAmount;
  - the CardDeckState comment now covers both card games.
- Data:
  - B3_ChipValues, B3_HandCardColumn/Row, B3_SplitMarkerPos, B3_MoneyCap;
  - B3_TablePaletteArgs/WhitePaletteArgs/TablePalette/WhitePalette. The last one explains the
    former generic region $9260-$926F.
  - The B3_TierParams note now decodes trailing bytes 8, 9, 11-13 and 17.
- gen.py no longer prints the generic "Data read by ...; layout not decoded" line for a region
  that lies wholly inside named structures.
- Long lines: 24 of 25 were fixed (shorter end-of-line notes, and a shorter inline-copy comment).
  One remains: the quoted inline message at Bank4_Msg_A300.
- B4_RefundPotCapMoney prose now cites B4_MoneyCapValue instead of $8329.

review.py was re-run: 2,636/2,636 reviewed, 1,900 units, editorial_audit pending.
`commentary.py verify`: ok, byte_identical true. game.asm sha256 17c20b11...2d0c, symbols.asm
4833fb53...92. The reading copy is refreshed. Uncertainties are in commentary/uncertainties.md,
"Round 6 handoff gate".

Known backlog, left for final acceptance (counts after this pass):
- 115 undecoded *Var#### RAM names;
- 82 mechanical BankN_/Fixed_ Data/Msg labels and 110 generic-data notes;
- 1,126 of 2,554 labels are mechanical _Next/_Loop/_Case branch labels;
- about 3,490 numeric immediates;
- 1 line over 96 columns.

## Round 5 commentary (2026-11-23)

Baseline: map changed pass-004 -> pass-005 (maps/current.json == analysis/state.json ==
maps/pass-005.json, sha256 c79c8cdf...8bc298). The whole round-4 commentary/ was copied to
analysis/round-4/commentary/ (diff -r identical; round-1..3 copies untouched) before
`commentary.py prepare` built the fresh frozen baseline (2,946 regions: 1,614 code, 1,206 data,
126 padding; byte-identical).

Method: round-4 pipeline copied to analysis/tools/round05-commentary/ (round-4 patch inputs now in
patches-r4/, this round's in patches-r5/ applied with annpatch.py). Names, headers, RAM symbols
and data notes carried by CPU address per bank (= physical range); gen.py re-validates every
name on an instruction start. Records invalidated: the replaced bank-3 $9270-$9974, bank-2
$B323-$B702 and bank-5 $BAEA-$BC21 windows, their consumers, and the provenance-only regions'
headers (36+6 code regions now Observed).

Changes:
- Bank 3: B3_OpponentNames $9278-$9290, B3_PanelLabelStrings from $9291, B3_TierParams (was
  Bank3_Data_92EE), B3_MessagePtrs, B3_Messages; `adc #<B3_MessagePtrs` / `lda #>B3_MessagePtrs`
  at $816E/$8172; constants B3_TIER_PARAM_SIZE, B3_MESSAGES_PER_TIER (new ann/consts.txt).
  B3_CopyBytes8295 -> B3_LoadTierOpponentMoney, BjVar07A9 -> BjTierStartMoney, B3_PrintAmount ->
  B3_BetAllMoneyMenu (was a wrong name), grids B3_BetAllMenuGrid/B3_YesNoGrid/B3_DecisionGrid,
  B3_PostGameScreen = GAME OVER screen (observed, artificial S16), B3_GameOverBanner,
  B3_BlankTile2x2, RAM BjHandRecords.
- Bank 5: B5_OpponentIntro -> B5_MatchResultScreen (quit/lost/won captions and the three closing
  texts; X = 0 -> game over) with branch labels; tables B5_ResultFrameBox, B5_ResultCaptionPtrs,
  B5_ResultCaptions, B5_ResultClosingLists, B5_ResultClosingLines, B5_ResultClosingText,
  B5_ResultPortraitFrames, B5_ResultPaletteByOpponent, B5_ResultGfxByOpponent; B5_YesNoGrid.
- Bank 2: B2_PrologueText/Ptrs, B2_EpilogueText/Ptrs, B2_AttractScenePtrs, structure
  B2_AttractScenePictures (8 x 20x6 blocks tiling $B6FF-$BACE), B2_AttractSceneAttrPtrs,
  structure B2_AttractSceneAttrRows, B2_TitleMetasprite (B2_MenuCursorXTable narrowed to 2 bytes).
- RAM comments: StakeTier 0..3 in game (0 = final match), GameOutcome 1 quit / 2 lost / 3 won;
  MainLoop_AfterGame / MainLoop_GameFinished headers rewritten.
- Editorial: 28 immediate pointer pairs symbolised (ann/ptrpairs.txt; NMI hook vectors, grid and
  VRAM-string pointers); ~75 raw CPU-RAM addresses in prose replaced by symbols (ramprose.py);
  generic-window wording no longer claims "no code reference"; DATA/structure notes wrapped to
  96 columns; register summaries no longer stop at local branch labels (gen.py ownership fix);
  review.json summaries clipped at sentence boundaries (600 chars) instead of a hard 300 cut.

review.json: 2,946/2,946 scaffold entries reviewed (summary + valid source), 1,872 units; every
code region has units; editorial_audit pending (final acceptance only).

Verification: `commentary.py verify` ok, byte_identical true (game.asm sha256 58a89e7c...5c,
symbols.asm 7840bf95...16); reading copy refreshed (`commentary.py read`). Gate scan: 0
raw-address operands, 0 routine headers without a description.

Remaining tentative meanings: commentary/uncertainties.md "Round 5" (and earlier sections).

## Round 4 commentary (2026-11-23)

Baseline: map changed pass-003 -> pass-004 (maps/current.json == analysis/state.json ==
maps/pass-004.json, sha256 eb7719e3...9519). The whole round-3 commentary/ was copied to
analysis/round-3/commentary/ (diff -r identical) before `commentary.py prepare` built the fresh
frozen baseline (before/, reading-before.asm, baseline.json, phase1-state.json, review.json
scaffold 2,977 entries, now with no Unknown entry).

Method: round-3 pipeline copied to analysis/tools/round04-commentary/ (round-3 patch inputs kept
in patches-r3/; this round's in patches/bank3.txt, bank4.txt, bank5.txt applied with
annpatch.py; ann/ram.txt: BjVar07C2 -> BjOpponentName). Names, headers, RAM symbols and data
notes are carried by CPU address within each bank (= physical range); the generator re-validates
every name on an instruction start. Only records touched by the pass-004 changes, their
dependencies and the same panel mechanism in bank 3 were rewritten.

Changes:
- $BB30-$BB31 (now Static code): B4_UnreferencedStateLoad with an honest note (unreferenced
  leftover `lda zpLocal3`; reachability unresolved, not claimed dead). Being its own label it
  also keeps the zpLocal3 read out of B4_TravelMovePlane's register summary.
- $9DB2-$9DB5: B4_SuitOrderTable; B4_HandAnalysis_9DAD -> B4_SuitOrderLookup (result discarded).
  B4_CompareHighCards header rewritten as a suit tie-break with the possible original bug at
  $9D5B (eol note). B4_HandAnalysis_9ADD -> B4_OpponentPickHolds (opponent draw strategy).
- Bank 5: B5_PrintMessageValue -> B5_PrintPanelLabels, B5_PrintNumber -> B5_PrintPanelAmounts;
  B5_PanelLabelYou, B5_PanelLabelRecords, B5_PanelAmountRecords (former generic $8803-$8812),
  B5_PanelLayouts (former Bank5_Data_8268); B5_SayMessage header; B5_PanelLabelStrings note.
  Pointer immediates rendered #</#> of the record tables.
- Bank 3 (same mechanism, concrete defect in reviewed material): B3_PrintMessageValue ->
  B3_PrintPanelLabels, B3_PrintNumberAt -> B3_PrintPanelAmounts; B3_PanelLabelYou,
  B3_PanelLabelRecords, B3_PanelAmountRecords (former generic $911A-$912D),
  B3_OpponentNamePtrs/B3_OpponentNames (former Bank3_Data_9270), B3_PanelLabelStrings.

review.json: 2,977/2,977 scaffold entries reviewed (summary + source), 1,840 units; every code
region has units; editorial_audit pending (final acceptance only).

Remaining tentative meanings: commentary/uncertainties.md "Round 4" (and earlier sections).

Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json,
game.asm sha256 6cb1e1ee...c387, symbols.asm 526a13d5...ad1); reading copy refreshed
(`commentary.py read`). Gate scan: 0 raw-address operands, 0 placeholder routine headers.

## Earlier rounds
See analysis/round-4/commentary/progress.md (and round-3..1 copies).
