# Round 3 - commentary phase handoff

Status: FINISHED. The current-source readability pass covers all classified source. The final editorial audit was
not run; it is reserved for final acceptance.

## Input / output identity
- Input checkpoint: maps/round03-final.json (sha256 aab2fb6c6b8120365d94d57a881bad3c0da726c7f5ccf5126a5504d8367b297b).
  maps/current.json and analysis/state.json are identical to it, and this phase did not change them.
- Previous commentary preserved: analysis/round-2/commentary/ (the whole round-2 commentary directory, with source
  2ed461a9...) and analysis/round-2/tools/ (round-2 cmt_*.py). Round 1 is still in analysis/round-1/.
- Fresh baseline: `commentary.py prepare`, byte-identical, 2293 scaffold entries.
- Output source: commentary/source/game.asm, sha256 b70bdbb2f5fe38db109efca1bed1eb3f995450188e442d3e0395d55e15e50869.
  It is built from commentary/annotations.txt (sha256 3e9f743d...) by analysis/tools/cmt_apply.py.
- Review inventory: commentary/review.json (sha256 4cb1656f...). All 2293/2293 entries are reviewed, with 951 code
  units plus data-symbol units. editorial_audit is pending.

## Verification and reading copy
- `commentary.py verify` returned ok:true and byte_identical:true (analysis/round03-commentary-verification.json).
- `commentary.py read` refreshed commentary/reading-after.asm after the final build.

## Classification (unchanged by this phase)
ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown). The 3 unknown bytes are blocked by
TK-04, so the rounded figure does not mean classification is complete.

## Scope covered
- **New round-3 regions**, each with a label and prose:
  - Bank0Entry0 (b0 $8008, a vestigial JSR slot that would return into the copyright text)
  - UploadStatusRow2360 ($8229)
  - StubJumpIdleLoop ($8A85)
  - SetStatusModeOneAndClear ($8CED)
  - PlaySfxSlot6 ($D7B9, which does not preset SndTemp)
  - StubJumpSurfUpdate ($DEBF)
  - FillTerrainTableForever ($F150, an endless fill loop, probably debug code)
  - CallBank0 ($F2BA). CallBank1 is now a BIT-operand constant, like CallBank2..7.
  - StubJumpReset ($FFF7)
- **Table tails:**
  - FrameHandlerTable $87CB-$87CF
  - BmxInitOam $BE8C-$BEA7 (new symbol)
  - PlayerNameOffsetsB $EEAF-$EEB6 (new symbol, with its three consumers)
  - BinToBcdTable entries 94-99
- **Generic data:** $9B46, b2 $87DF, $F072, $F883 and $FFDF each have a short role/uncertainty note.
- **TK-04:** the notes at bank 0 $BFFE-$BFFF and bank 7 $C000 state the exact blocker. A Code region must decode from
  physically contiguous bytes; 0x400F and 0x1C010 are not adjacent under any prg_banks; map.annotate rejects the
  instruction; and Data would contradict the observed fetch.
- **Redirect corrections:**
  - PlayerNames records are 7 bytes (6-character name + team byte = PlayerTeams).
  - The FrameHandlerTable extent note is fixed.
  - Labels added for the new tables.
- **Round-2 defect fixed:** two gap notes ($BEA3 and $BFFE) had wrong physical keys and were silently dropped. The tool
  now warns about notes like these.
- **Backlog / editorial issues from uncertainties.md:**
  - Correction: $61 (RegistrationNeeded) and $4D (TextGlyphMode) were wrongly recorded as unread.
  - 28 global RAM names: RNG entropy bytes, text-printer glyph state, menu cursors 4-6, scratch Work39/4E-51, award
    places, bank-6 scroll/course-object state, and SndCmdVectorHi.
  - Foot bag overlay: 24 names (Kicker*/Bag*) with corrected prose. FootBagPlayerMove was renamed FootBagStepPose
    because the old name misdescribed it.
  - Flying disk overlay: 5 names.
  - New data symbols: TextCursorOam, TeamNameEnds, TeamNameText, SndCommandTableHi.
  - Raw zero-page operands went from 1212 to 922.
- Remaining unreviewed ranges: none.

## Readability assessment
- **Good:** system code (boot, NMI/frame dispatch, banking, sound driver, text/menu system, results) and all round-2/3
  additions.
- **Foot bag:** now reads well.
- **Half pipe, flying disk, BMX, skating and surfing internals:** routines are named and have prose, but many overlay
  bytes remain numeric, and the prose describes them by address.
- Branch labels outside the round-2/3 code remain structural (Owner_Skip/Loop).
- Overall: adequate for porting navigation.

## Remaining editorial issues (explicit uncertainty in commentary/uncertainties.md)
- **Numeric overlay bytes:**
  - half pipe body-segment arrays ($91-$C1)
  - flying disk actors ($A1/$A2/$A5-$BB), where the thrower/catcher/runner roles conflict between routines
  - foot bag side object $DE-$E2 and counters
  - BMX $C2-$D4, skating $CA-$D6 and surfing $C0-$D8 extras
- **Tables left numeric:** text-screen tables $EE7D/$EE86. The bank 6 course-object slot fields are named only at
  the base.
- **Tentative:** AwardLayout and the medal-point reading of PresetScores.
- **Not established:** whether any of the round-3 no-caller code runs.

## Tools / toolkit
These are workspace tools only:
- cmt_apply.py: warns about unplaced P notes and no longer adds a "fill-like" note to 1-byte gaps.
- cmt_review.py: credits a code region that starts at its own label to that label.

No toolkit changes and no new toolkit issues (TK-04 is still the only blocker).

## Disk
Scratch is limited to scratch/r03c/ (small helper scripts plus a view file), about 3.0G free.
