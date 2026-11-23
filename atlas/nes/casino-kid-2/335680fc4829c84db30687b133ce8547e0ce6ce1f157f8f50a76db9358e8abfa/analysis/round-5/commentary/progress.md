# Commentary progress

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
