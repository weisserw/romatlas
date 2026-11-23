# Round 7 / commentary handoff

Status: **complete** for the round-7 current-source readability gate.
- Classification: 131,088 / 131,088 bytes (pass-007, 0 unknown). Classification and readability are
  separate claims; this pass changed no map or state.
- Editorial audit (final acceptance) was not run; review.json editorial_audit is still pending.

## Checkpoint and baseline
- The map changed from pass-006 to pass-007. These are identical: maps/current.json, analysis/state.json,
  maps/pass-007.json and commentary/phase1-state.json (sha256 bea35cf5...be7f). commentary/manifest.json has
  the same map/state sha.
- The round-6 commentary is preserved in full at analysis/round-6/commentary/ (diff -r identical). The round-1..5
  copies were not touched.
- A fresh baseline was prepared from pass-007: 2,534 regions (1,614 code, 902 data, 18 padding), byte-identical.
  commentary/progress.md and uncertainties.md were restored from the round-6 copy and extended.

## Scope
All classified source: 1,614 code, 902 data and 18 padding regions.
- Carry-forward by CPU address per bank with the round-6 generator, copied to
  analysis/tools/round07-commentary/. Unchanged regions regenerate the round-6 text; only the pass-007 regions
  and their dependencies were invalidated and rewritten.
- Then a backlog pass over the whole source: mechanical table labels, generic-data notes, address-based RAM
  names, misread routine names, wrong immediates and long lines.

## Deliverable and reproduction
- Source: commentary/source/game.asm + symbols.asm, with the local image.nes and layout.cfg.
- Generator: analysis/tools/round07-commentary/build.sh (gen.py + ann/*.txt), then review.py.
  - Edits against round 6: round07-ann.diff and round07-gen.diff (same directory).
  - gen.py changes: former-padding notes use pass-005 and pass-006 Padding, merge adjacent runs and describe them
    by table role; inline message labels are named after their text; buffer offsets for RltSpriteHeaders,
    RltMarkerHeaders, RltMarkerCol, BjDealerHand/BjHandRecords (with BJH_* fields); over-long inline .word
    comments move to the line above.
- Reading copy: commentary/reading-after.asm (refreshed).

## Verification
- `commentary.py verify`: ok, byte_identical true (commentary/verification.json).
  - game.asm sha256 ad2beb8ad1596767e442fb878e78b33f8da6d644fe8f905104b51b1e711d6e63
  - symbols.asm sha256 a8a11730a33e8f35cf33a6e47a12d699b79f1f14d1bc1ac1c52aaa937ec33e6c
- input/game.nes unchanged (sha256 335680fc...abfa).
- review.json: 2,534/2,534 scaffold entries reviewed (summary + source), 1,974 units.
- Gate scan: 0 raw-address operands, 0 routine headers without a description, 0 lines over 96 columns.

## Examples (game.asm line)
- **Bank-6 world map:** B6_WorldMapInfoBlocks 26684 (game icon per location matches OpponentGameTable),
  B6_PortraitChrAmerica 26729 (11 portrait sets), B6_WorldMapLegSteps/LegDirs, B6_WorldMapPortraitPalettes.
  The fixed routines are re-read: WorldMapPortraitUpload 36072 (9-step CHR upload into tile buffer $B0 or $D2),
  WorldMapPortraitShow, WorldMapInfoStep 36285 and WorldMapApplyPortraitPalette. The old "Fixed_Data_* bank 7
  table" labels are gone.
- **Bank-5 panel lists:** B5_PanelListPtrs 16481 and eight lists; B5_DrawPanelList 16137 (was
  B5_UpdateBetDisplay). The list rows are offset by the scroll (B5_MessageRowAddress). CopyBoardTiles 27383 (was
  CopyBank3Record20) copies board tiles from NametablePicture1/2.
- **GAME OVER screen:** the B3 "post-game" routines animate the letters: B3_GameOverLetterStep 6237 (bounce
  physics), B3_GameOverLetterX/CellAddr/LetterTiles.
- **Poker:** B4_PokerTierRecords 13126 (opponent money, minimum bet, AI nibbles, raise limit); B4_OpponentMood
  11543 (was B4_RankCardsA); B4_OpponentChooseBet / AnswerRaise / FoldCheck (the old Draw/Bet/Bluff names were
  swapped or wrong); PkrLeadHand, PkrRaiseCount, PkrBetRound, PkrScriptedDeal and B4_ScriptedCards.
- **Roulette:** B5_WheelStrip (38 pockets x 2 tile columns + 8 wrap columns), B5_SpinToNumber 20526,
  RltMarkerHeaders/X/Y/Hidden, RltMoneyBefore/AfterSettle.
- **Sound envelope:** B6_UpdateChannel 23610: [rate, level delta, frames] segments with SndEnvPtrLo/Hi,
  SndEnvSegFrames, SndEnvAccum and SndEnvLevel.
- **Other tables:** B2_WipePalettePtrs 1403, B4_PasswordAlphabet 13426, RomTitleFooter 37533 ("CASINO KID 2").

## Unresolved meanings
See commentary/uncertainties.md, "Round 7"; earlier sections still apply. Main items:
- The two step-count bytes of B6_WorldMapLegSteps; the meaning of the 1-3 $FC/$FD marks in the info blocks.
- B4_ScriptedCards (probably a rigged final-match hand, not run); PkrHandFlag tested for 2, never stored.
- The odd GameVar0726 entries read by B4_OpponentFoldCheck (player hold flags, tentative); the
  B4_OpponentChooseBet thresholds.
- B5_WheelStrip number order; whether header 0 of B5_SpinToNumber is the ball.
- No reader found: B4_UnreadRankValues, the tail of B4_SpareMenuStrings, "MONEY" at b4 $A00D, BlankTileRun,
  RomTitleFooter; write-only PkrFinalOpponentMoney, PkrPlayerWinStreak/PkrOpponentWinStreak, PkrTierByte0.
- Carried: B3_CompareHands statuses 3 and 5-9; blackjack tier bytes 10 and 14-16; bank-6 stream commands.

## Remaining gaps (editorial backlog for final acceptance; not blocking this gate)
- 59 *Var#### RAM names remain (57 "role not decoded", mostly sound engine per-track arrays, roulette cursor/
  wheel state and blackjack bet amounts). Each is commented with its owning game/bank.
- 25 inline message labels are text-derived (B4_Msg*); 0 mechanical BankN_Data_/Fixed_Data_ labels; 0
  generic-data notes.
- 1,214 of 2,554 labels are mechanical _Next/_Loop/_Return/_Case branch labels inside explained routines.
- About 3,484 numeric immediates.
- Some older short headers (for example several bank-4 password and bank-5 cursor helpers) are accurate but
  terse.

## Deferred
- Final editorial acceptance: record editorial_audit, then `commentary.py audit`.
- Redirect round 7.

## Toolkit
No new toolkit issues found (analysis/toolkit-issues.md unchanged).
