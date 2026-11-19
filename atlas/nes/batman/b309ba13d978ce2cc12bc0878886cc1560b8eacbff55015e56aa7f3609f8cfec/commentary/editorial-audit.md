# Final editorial audit (round 5, checkpoint pass-005)

## Scope
All commented source in commentary/source/ (game.asm, bank00-15.asm, chr.asm, symbols.asm),
built from the round-4 notes carried 1:1 onto the fresh pass-005 baseline. The review.json
inventory has 2,195 scaffold entries, all reviewed. Reviewed units went from 608 to 720 because
the newly commented data tables now get units. There are 1,322 unit records in total, because code
regions that continue a routine get clipped records. Valid prior review credit was reused. There was
no whole-source rewrite: the audit used targeted searches and then read the hits in context.

## Method (searches over source/ and notes/)
1. Leftover auto labels (`Sub_`, `L_`, `Dat_`, `region_`) as definitions or operands: 0.
   Mechanical branch labels (tools/remaining.py): 0.
2. TODO/FIXME/`???` markers: 0. The one hit, "$Dxxx" in the bank-10 head, is intentional.
3. Raw RAM/hardware operands that have a symbol: 0. Only `$0103/$0104,x`, the stack-page reads
   in QueuePpuRow, remain raw, and they carry an explanation. Raw ROM operands: 4 cross-bank
   cutscene/vector accesses, each with an inline note naming the target.
4. Comment identifiers that do not resolve to a label or symbol: only shorthand
   (`.../DirFlags`, `ScriptX...`, `DeltaX`) and address text. Expanded where it was unclear.
5. Comment addresses `$8000-$FFFF` that equal a labelled address but do not name it (stale
   numeric aliases): 206 hits were reviewed. Most are region headers, vector-table descriptions or
   window names, which are fine. 14 real stale aliases were fixed (list below).
6. Labelled data tables with no comment anywhere and no mention in any comment: 111 found.
   Each now has a one-line role/consumer note (scratch/r5/data_consumers.py lists them;
   scratch/r5/data_notes.py adds the notes). The check now finds 0.
7. Routine headers without a purpose line: 0 of 459. No duplicate adjacent comment lines.
8. Certainty wording: "confirmed", "proven", "always", "never executed" and "unused" were
   checked against the evidence type (artificial-only observations, effectively-zero
   variables, static no-reference findings).
9. Review summaries: replaced 1,126 generic "Data continuing ..." summaries with the
   neighbouring structure name and bank context. Gave the Manual region its own summary.

## Findings and fixes
- **Wrong target (concrete defect):** GameFrameLoop's game-over call at $DE14 (`jsr $8003` right
  after `lda #$18 / SetPrgBanks`, which maps bank 8) was commented as "Area_Scroll (bank 4)". It
  calls AreaSvc_GameOver, the game-over/continue screen; the following GameStartFlags bit6 test
  matches GameOverScreen's continue flag. annotate.py now resolves this site statically
  (STATIC_CALL_BANK), so the operand is `AreaSvc_GameOver` and the routine has a caller. The
  GameFrameLoop header was rewritten from its actual call sequence. Misnamed branch labels were
  renamed: `.playerUpdate` -> `.areaHook` (it calls AreaSvc_Hook) and `.noAreaHook` ->
  `.skipSpawn` (it skips ObjSvc_Spawn in boss areas).
- **Wrong address:** the bank-12 region note after ObjKind4StepsB said the region "ends at code
  $9BB9". $9BB9 is the file-offset tail, not the CPU address; the CPU address is $9BA9 (ObjKind3).
  Fixed, and the table is now named instead of `$9B99,y`.
- **Contradictory names:** BossBlastSprites holds object-slot masks ORed into ObjActiveMask.
  It is renamed BossBlastSlotBits. Bank-2 orphan CsJoyStrobeRead/CsJoyMergeBits used $20/$21,
  which were shown with the gameplay names ObjDirFlags/ObjSlotX. They are now named by the
  cutscene override CsJoyExpBits1/2.
- **Stale numeric aliases:** these were replaced by symbol names. BossSpriteSetup ($DB85/$DA0B/
  bank 5 $90E0), CommitBatmanPosition ($9E08), ObjKind19 ($8261/$826D/$8279), ObjKind11
  ($8FEC/$8FF7), ObjKind1_retire (bank 13 $A03A), DrawHud (bank 8 $800C), LoadPalette
  ($C96B/$C971), NmiHandler cutscene path ($8100/$8103), CsSkipToTitle (bank 3 $AF1C), the bank-3 B3REC
  note ($B57B), DpcmSample0 (bank 0 $89DD), AreaCleared ($88=$24) and the GameFrameLoop header
  (bank 12 $80A2, bank 7 $8003).
- **Missing structures:** 111 data tables had descriptive names but no note. They now have short
  role/consumer notes, mostly per-step sprite/offset/direction tables of object kinds, boss
  set-up tables, death animation, area title position, terrain thresholds and DPCM samples
  1/2. The iNES header has a description. All notes describe what the code does mechanically;
  they add no identifications.
- **Misleading certainty:** AreaExitCheck said "Confirmed in RAM-patched experiments". It now
  says it was seen working in RAM-patched experiments that started from artificially set state.
  CsJoyStrobeBase "always 0" now reads "effectively 0", matching the uncertainty record.
  "never executed"/"(unused)" for the duplicate bank tails and the bank-2 fill now say "no
  reference/never traced".
- **Manual fallback region (new in pass-005):** $C59E-$C59F (file $1C5AE-$1C5AF). The source,
  uncertainties.md and the review entry all describe it the same way: generic data of unknown
  meaning, a user-authorized manual fallback, not observed and not statically proven. It has
  no label and no format.

## Not changed (explained, still open)
These are listed in uncertainties.md. Object-kind identities. Area-map byte formats. Sound-command
semantics. Orphan code with no known caller. Data with weak role evidence. The kind-42 spawn-list
placement. The SetupBossSlotsArea17 caller. The role of the $C59E bytes. Bank-level data regions
that phase 1 split into many pieces keep a region summary that names the neighbouring structure
and bank context; their byte formats are not decoded. The audit tool cannot judge prose quality.
This record is the agent's editorial judgement.

## Result
`commentary.py audit` passed: ok=true, byte_identical=true, reviewed_regions 2195,
reviewed_units 1322, editorial_audit true. The reading copy was refreshed.
