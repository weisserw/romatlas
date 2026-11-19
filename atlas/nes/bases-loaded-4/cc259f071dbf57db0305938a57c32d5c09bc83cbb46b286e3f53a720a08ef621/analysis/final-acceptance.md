# Final editorial acceptance - commentary on checkpoint pass-002

Date 2026-11-18. Input ROM sha256 cc259f07...ef621 (input/game.nes unchanged, re-hashed after
the pass). Toolkit untouched. Phase-1 classification is not part of this gate: it stays as
recorded by pass-002 (100% classified, 0 unknown). Audit acceptance says nothing about it.

## Result
- `python3 /root/romatlas/scripts/commentary.py audit <workspace>`: **ok**. It found
  byte_identical true, reviewed_regions 4,154, reviewed_units 1,803 and editorial_audit true
  (commentary/audit.json). The audit also re-ran verification (commentary/verification.json)
  and refreshed commentary/reading-after.asm (79,141 lines).
- review.json editorial_audit: status reviewed, with notes listing the findings below.
- Pre-pass snapshot kept for comparison in analysis/round-2-final-pre/ (source/, tools/,
  review.json as they were at the round-2 handoff).

## What was fixed
1. **DPCM prose** (redirect r2-a item). The fixed_30_31.asm DmcSample_D5C0..F9C0 headers
   called the regions "generic, no consumer". They now say which DmcSampleTable records
   use each sample, with its start and length. Bank31Data_E000 is renamed
   **DmcSample_DAC0_Tail**: it is the last 192 bytes of sample $DAC0-$E0C0. The IRQ/Reset
   "readers" are MMC3 register writes at the same CPU address.
2. **Password** (r2-a). U16_PasswordEditInput now gives the letter values that were read on
   screen: 10 = R, 11 = D, 12 = B, 14 = C. Two string headers were wrong and are corrected:
   U16_PasswordText is 26 blank tiles, and U16_InvalidText holds only "INVA". The rest of the
   strings are now U16_InvalidTextRest, with sublabels U16_TryAgainText,
   U16_GoBackToStartText, U16_EnterPasswordText and U16_PasswordMenuCursorSprite.
3. **Numeric pointer immediates.** All 20 real pointer pairs into unlabelled generic data now
   use labels at the exact target, and byte identity is unchanged:
   - FieldMetaDefsHigh and FieldAttrMap in each field unit 0/2/4/6
   - the 4 unit-16 password texts and cursor sprite
   - U22_ObjectAnimTable, U22_UmpireAnimTable and U22_BatterFieldAnimTable
   - U26_CatcherAnimTable
   - U28_CannedSoftHits, U28_CannedHitsA and U28_CannedHitsB (32/16/16 x 4 bytes, contiguous
     $B888-$B988)

   The enclosing regions got Uxx_Data_XXXX labels with header notes for the sublabels. The
   "~37" count in the handoff also included VRAM addresses, sprite X/Y pairs and RAM pointers.
   Those are correctly left as numbers (e.g. `#$80/#$A0` in U16_FieldMessageSprite is a screen
   position).
4. **Branch labels.** All 760 @Skip_XXXX and 294 @Loop_XXXX labels are gone.
   - annotate.py now derives names from the code:
     - loop heads are only labels that straight-line code can reach again (shared tails are
       not loops)
     - fall-through joins are named after the effect of the skipped block (@SkipCallPlaySound,
       @SkipSetObjByte17)
     - other targets are named after their first effect (@SetIrqSplitLineTo8,
       @ReadObjByte19), their returned value (@ReturnA1, @ReturnCarrySet), or the condition
       (@StrikesAtLeast2)
     - loops are named after their body (@CopyVramBufferLoop, @FillChrBankNibblesLoop,
       @WaitForIrqCount)
   - About 200 labels got hand names in commentary/tools/knowledge_r3.py, for example:
     - U10/U22/U24/U28 metasprite drawers: @NextPiece, @SkipPiece, @PositiveYOffset,
       @NotMirrored, @ClippedPiece
     - U14_CheckGamesPlayed: @NextDayLoop, @EmptySlot, @CountPlayerGame
     - U20_ChooseStarter: @FindRestedAwayLoop, @MarkAwayUnavailable
     - U28_VectorToAngle: @NormalizeLoop, @AngleIs80
     - fixed-bank math: @DivideBitLoop, @AddOddLoop
   - Duplicates in one scope are numbered (@SkipStoreVramBuffer2) instead of carrying address
     suffixes.
5. **RAM aliasing inconsistencies** found during the label work are now shown with scoped
   operand names (ram.inc documents them):
   - $1A-$1D are plain math scratch in units 22/26/28. They now show as Arg1A-Arg1D instead of
     MetatileTiles, and the routine prose was updated to match.
   - In unit 26, $042F-$0432 show as FieldPaintStep/FieldPaintTarget instead of PasswordBytes.
     On entry to U26_InitRestoreFieldView they hold the saved camera X/Y.
   - The ball-record snapshots in U26_PlanBattedBall/RestoreBallRecord and
     U28_PitchRelease/PitchPredict now show $91 as SavedBallRecord instead of MenuStep.

## Verification
- After each coherent batch and at the end, `commentary.py verify` rebuilt the ROM and it was
  byte-identical. The final audit run also verified. annotate.py output is deterministic: a
  re-run reproduced the round-2 source exactly before any edits were made.

## Residual gaps (stated honestly)
- **18 mechanical branch names remain**, such as @IfCarrySet, @IfBelow3 and @IfAtLeast32. They
  are in arithmetic or geometry code whose purpose is not established:
  - U26 fence-centre tests and catch-box comparisons
  - the U28 intercept-angle cases
  - U20_HomeLineupFrame and U20_ChooseStarter $FE marker
  - U22_BatterFramePtr
- **168 @Return_XXXX labels** mark plain `rts` targets and keep address suffixes.
- **Code-derived names are not hand-verified.** About 850 labels are named from the code by
  rule. They were spot-checked (about 100 samples; one wrong rule, wait-loop detection, was
  fixed). A name says what the code does next or what it skips, not necessarily the game
  purpose.
- **Some names inherit RAM-name limits.** Labels built from raw names (Var88, Var0376) or from
  shared bytes such as MenuStep in cut-scenes carry those limits.
- **Generic data stays undecoded.** The L014 and phase-1 generic data formats are not decoded
  (area notes only). This includes the heads of the newly sublabelled regions, e.g. the bytes
  before U28_CannedSoftHits in U28_Data_B403.
- **Optional provenance work not done.** The 10 regions in analysis/redirect/r2-a/s11-coverage.txt
  were not given S11 provenance notes. This is optional and does not change any classification.
- **Open questions.** Semantic uncertainties are listed in commentary/uncertainties.md, for
  example game-mode meanings, uncalled-routine purposes, and the code that writes the saved
  field camera.
