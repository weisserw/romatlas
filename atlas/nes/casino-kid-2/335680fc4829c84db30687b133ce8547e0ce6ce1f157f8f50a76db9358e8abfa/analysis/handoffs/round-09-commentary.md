# Round 9 / commentary handoff (final editorial acceptance)

Status: **complete**. The current-source readability gate and the final editorial acceptance both pass.
`commentary.py audit` is ok.
- Classification: 131,088 / 131,088 bytes (pass-009, 0 unknown). This pass did not change the map or the state.
  Classification and readability are separate claims.

## Checkpoint and baseline
- These files are identical (sha256 18763c395f6636b1f8d9c827708e98b059ebca665921b90ebc6835c154f95de1):
  maps/current.json, analysis/state.json, maps/pass-009.json and commentary/phase1-state.json. commentary/manifest.json
  records the same map/state sha.
- The round-8 commentary is preserved in full at analysis/round-8/commentary/ (diff -r identical). The round-1..7
  copies were not touched.
- `commentary.py prepare` built a fresh baseline from pass-009: 2,590 regions (1,670 code, 902 data, 18 padding),
  byte-identical. before/game.asm is identical to the round-8 one. pass-009 changed only evidence text.
- commentary/progress.md and uncertainties.md were restored from the round-8 copy and extended.

## Scope
- Carry-forward used analysis/tools/round09-commentary/, a copy of the round-8 pipeline. The edits are recorded in
  round09-ann.diff; gen.py and snd.py are unchanged.
- The first regeneration was identical to round-8 game.asm 3d312a13..., so all review credit carried:
  2,590 entries and 2,042 units.
- Editorial pass: R8-L044, the remaining address-based RAM names, routines found to be misread while naming them,
  terse headers in the same areas, and uncertainty cross-references. Per the final-acceptance rules there was no
  whole-source reread.

## Verification / audit
- `commentary.py audit`: ok, byte_identical true, reviewed_regions 2590, reviewed_units 2042, editorial_audit true
  (commentary/audit.json, verification.json). The reading copy commentary/reading-after.asm was refreshed.
  - game.asm sha256 7efbc79749707da4d3a76e181d12987a015d8b7b98f77ccc3adf32a496239651
  - symbols.asm sha256 7db4abb2c2f6cecb41d746327e5093678dca511c709575050644a0ae548c5def
- input/game.nes unchanged (sha256 335680fc...abfa).
- Gate scans: 0 raw-address operands, 0 lines over 96 columns. Every review entry and unit has status reviewed,
  a summary and a source.
- analysis/redirect/r08-a1/tools/danglingnames.py now reports only deliberate "Named X before round N" history
  notes and the B4_Msg wildcard.

## Changes (game.asm line)
- **R8-L044 (now verified in leads.json):**
  - B6_InstrumentData note -> B6_WorldMapMarkerPtrs.
  - B3_BlackjackGame 1736: the B3_SettleHand handlers move the money; there is no B3_UpdateBankroll.
  - B5_ResultPortraitAttrs: its user is B5_MatchResultScreen.
  - uncertainties.md: "Superseded" cross-references on the round-1/2 bank-6 and sound notes.
- **RAM names (address-based 7 -> 1):**
  - RltMarkerCovered, RltMarkerInView, RltHomeScrollStep;
  - BjDoubledFlag, BjCardTemp (two unrelated uses, documented);
  - RltScratch (bank 5, 32-bit).
  - PwdVar057B is kept because nothing reads it.
- **Misread bank-5 routines:**
  - B5_OffsetCoveredMarkers 21594 (was B5_HighlightNumber). It fans out markers that share a board cell by 4 px,
    using B5_ApplyCoverOffset / B5_RemoveCoverOffset and B5_Marker{Left,Right,Up,Down}4. The old "highlight"
    names were wrong.
  - B5_ScrollBoardHome 20310 (was B5_MatchEndSequence) is the board scroll back to Y 0 after the spin.
    B5_UpdateMarkerInView was B5_BallStep; B5_ShiftMarkersUp/Down were B5_Dec/IncBallState.
  - Callers' headers were fixed to match: B5_PlaceBet, B5_CancelPendingBet, B5_ResultSequence, B5_RouletteGame.
- **Headers rewritten:**
  - B3_CompareTotals 4884 now has the full status table. B3_CompareHands, B3_CheckCanSplit and B3_DecisionSplit
    (split aces) and B3_SettleHand were also rewritten.
  - B5_MatchEndDialogue.
  - The password button commands: B4_PasswordInput, then B4_PasswordStart and B4_PasswordSelect. These were
    B4_PasswordType and B4_PasswordMove, whose names did not match what they do. B4_PasswordErase was also
    rewritten.
  - B4_PasswordIntroAnim, B4_PasswordIntroStep and B4_PasswordIntroMove.

## Unresolved meanings
See commentary/uncertainties.md "Round 9"; the earlier sections still apply.
- PwdVar057B (never read).
- Writer of blackjack hand status 3.
- B5_MatchEndDialogue message texts.
- Odd mechanics kept as read in B5_ScrollBoardHome / B5_UpdateMarkerInView (seam jump, only 4 of 5 win flags
  tested, re-entry Y while the step is negative).
- Carried items: board geometry behind B5_CheckCursorMove; RltViewTop/Bottom unit; B3_DrawHandCards mode
  meanings; the unused sound $FE fade and header type 2; the instrument list end; B6_WorldMapLegSteps;
  info-block $FC/$FD marks; readerless items.

## Remaining limitations
- Accepted editorial residue (not defects):
  - mechanical _Next/_Loop/_Case branch labels inside explained routines;
  - local numeric immediates (counts, message and sound ids);
  - B6_Stream_XXXX labels for the 113 walked sound streams;
  - short but accurate headers on small helpers.
- Names and comments are static readings unless noted. No runtime check was made this round.
- The audit checks inventory and byte identity, not prose quality. The editorial review is recorded in
  review.json editorial_audit.
- No toolkit issues found (analysis/toolkit-issues.md unchanged). No server was started and no scratch directory
  was used outside /tmp build temporaries, which build.sh removes.
