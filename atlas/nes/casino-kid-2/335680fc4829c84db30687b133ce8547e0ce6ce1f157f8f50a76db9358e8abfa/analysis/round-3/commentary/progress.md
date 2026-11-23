# Commentary progress

## Round 3 commentary (2026-11-23)

Baseline: map changed pass-002 -> pass-003 (maps/current.json == analysis/state.json ==
maps/pass-003.json). The whole round-2 commentary/ was copied to analysis/round-2/commentary/
(diff -r identical) before `commentary.py prepare` built the fresh frozen baseline (before/,
reading-before.asm, baseline.json, phase1-state.json, review.json scaffold 2,977 entries).

Method: round-2 pipeline copied to analysis/tools/round03-commentary/ (round02-commentary kept as
the round-2 record); patches applied with annpatch.py are kept in patches/. Names, headers, RAM
symbols and data notes were carried by CPU address within each bank (= physical range); the
generator re-validates every name on an instruction start and the build is byte-identical.
Invalidated/changed only where pass-003 or the round-2 redirect changed evidence.

Generator changes: region starts that did not exist in pass-002 (the 63 Observed/Static code
splits) no longer imply routine entries; Bank4_Msg message labels survive inside named string
ranges; whole generic-note blocks are dropped when a structure/evidence note applies (fixes stray
generic continuation lines left in round 2); lead/provenance tails stripped from evidence notes;
bank-6 sound header pointers, 28 headers and 113 stream entries rendered as labelled directives
(snd.py: B6_SoundHeaderNN, B6_Stream_XXXX, decoded command bytes, loop/call targets as labels);
review.py adds units for named data structures and streams.

Scope: all classified source (1,603 code regions, data, padding) plus the 2 unknown bytes.
Changes this round:
- Roulette: B5_OpponentBet -> B5_PickWinningNumber, B5_SettleMatchMoney -> B5_SettleBets,
  B5_ResolveBets/B5_PayoutStep/B5_PayoutMessages -> B5_ChipPanelResult/B5_RemoveChipFromBet/
  B5_ConfirmBet, new B5_AddChipToBet; headers for B5_InitRound, B5_ResultSequence,
  B5_SelectBoardCell, B5_PrintMessage, B5_SetupRouletteTable, B5_PortraitEyes. RAM: RltWinningNumber,
  RltWheelPos, RltBetWonFlags (bank-scoped $0718-$071A), RltBetCount, RltBets, RltBetWriteOffset,
  RltPendingBet, RltAmountScratch, RltClassWonFlags, RltLastBetAmount, RltTierMoney, RltMinBet,
  RltMaxBet, RltOpponentName, RltActiveMarker, RltMarkerRow, RltMarkerCol. 18 new bank-5 tables
  named ($947D-$B634, replacing B5_BoardCellTable and 8 mechanical labels); pointer immediates
  rendered #</#> of B5_OpponentNamePtrs, B5_MessagePtrs, B5_BetCellListPtrs.
- Poker: B4_DrawChipStack(s) -> B4_DrawHandName(s); B4_CompareHands, B4_EvaluateHandRank,
  B4_CompareHighCards, B4_ExtractPairedRanks, B4_LoadHandCards (was B4_CardRank),
  B4_HandToSortedRanks, B4_SortFiveRanks; RAM PkrShowdownWinner, PkrOpponentHandRank,
  PkrPlayerHandRank; B4_PrintMessageWithValues header (tier message table). 11 bank-4 tables named
  (menu strings, hand-name records/strings, opponent name widths, message pointers, dealer/tier
  strings, lucky strings, credits pointers, staff lines, THE END banner).
- Ending: B4_ExtraScene* -> B4_EndingCredits, B4_CreditsSetup/DrawLine/Scroll/BlankLines/
  WrapRow/ClearLine; CallBank4Entry4 -> RunEndingCredits.
- Sound: B6 header/stream layout and grammar documented at B6_TrackStreams.
- Evidence refresh: FinalMatch/ending reached, roulette NMI handler, zpDebugFlag test menu now
  stated as run-time observed (round-2 redirect).

review.json: 2,977/2,977 scaffold entries reviewed (summary + source), 1,830 units;
editorial_audit pending (final acceptance only).

Remaining tentative meanings: commentary/uncertainties.md "Round 3".

Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json,
game.asm sha256 30541c91...755c, symbols.asm 1930e7c3...d21a); reading copy refreshed
(commentary/reading-after.asm). Gate scan: 0 raw-address operands, 0 placeholder headers.

## Earlier rounds
See analysis/round-2/commentary/progress.md and analysis/round-1/commentary/progress.md.
