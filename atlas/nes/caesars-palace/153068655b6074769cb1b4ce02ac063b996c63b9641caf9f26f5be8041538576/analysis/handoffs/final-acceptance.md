# Final acceptance (editorial audit) handoff

Status: PASSED. `commentary.py audit` and `commentary.py verify` both succeeded (byte-identical). Every completion criterion
in goal.md is met for the round-3 map. ROM classification: 100.00% (131088/131088 bytes; 0 unknown).

## Checkpoint
- maps/round3-disasm.json = maps/current.json = analysis/state.json = commentary/source/mapping.json
  (sha256 24099db1a8311da17f7abf3fe5598c5af5380e2c4c0e9fb1cbb6c13210afbed3). There are 1021 regions: Code 26,374,
  Data 92,964 (including the header) and Padding 11,750 bytes. The regions are contiguous with no gaps, and every region
  has a non-empty evidence string.
- `romatlas map validate maps/current.json --rom input/game.nes` returned ok. `romatlas store validate ... --store
  analysis/state.json` returned ok.
- input/game.nes is unchanged (sha256 1530686...). There were no toolkit edits and no new toolkit issues
  (analysis/toolkit-issues.md is unchanged).

## Checks run (targeted; round-4 review credit reused, no whole-source reread)
Checks over commentary/source/game.asm:
- Address-style labels: 0.
- Numbered branch labels: 4 (MergeRowAttrs_High/LowLoop1/2). These were fixed (see below).
- Numeric absolute, zero-page or indirect operands in instructions: 0.
- `.word` data with literal addresses: 0.
- TODO, FIXME or XXX markers: 0. None in uncertainties.md or progress.md either.
- Data regions with the generic fallback note: 0.
- Unnamed routines: 0.
- The remaining mechanical forms are 51 `Routine_Loop` and 66 `Routine_Return` names (single loop or shared RTS). They are
  documented in the source header and accepted.
- Raw-address symbol names (Unknown00A0, Unknown04B3, Unknown04E9, Unknown0615, Temp*) all have no-reader or scratch
  explanations. They are allowed for unknown roles.

Stale names:
- ClockHourTable, ClockSecondsTable, HoldLabelTiles and ClockColon appear only in historical rename notes
  (uncertainties.md, progress.md) and in phase-1 evidence strings.
- Decision on ClockHourTable in source/mapping.json: not a defect. That file is the generator's copy of the frozen phase-1
  map and is byte-identical to maps/current.json. Neither verify nor the reading copy uses its labels. Changing it would
  make it disagree with the checkpoint. The commented source names the record TemperatureTable. The divergence is now
  stated in commentary/uncertainties.md.

Consistency between uncertainties.md and the source: every CamelCase symbol resolves to a source label or equate, except
the intentional old names in rename notes and the patterns SndCmdNN and "ColBase/RowBase".

review.json summaries: one concrete defect was found. Hard truncation at 400/500 characters produced misleading partial
names, for example "LayoutToScreenX" listed twice where the second was LayoutToScreenX_Off.

## Findings fixed
1. MergeRowAttrs_HighLoop1/2 and LowLoop1/2 were renamed to MergeRowAttrs_HighLeftScreen/_HighRightScreen and
   _LowLeftScreen/_LowRightScreen. Comments now say that Ptr1D = AttrShadowPtr0 (left screen) and Ptr1F = AttrShadowPtr1
   (right screen), with 10 bytes merged.
2. The FontTileCount equate comment had a stale "named in round 1" history. It now gives the established role: the
   47-tile count, copied to DstPtr and counted down once per tile (the ROM value equals the Font1bppTiles size).
3. commentary/tools/review.py (local commentary tool, not the toolkit) has two changes:
   - Summaries are now shortened at word boundaries with " ...".
   - A recorded editorial audit is preserved instead of being reset to pending.
   18 region summaries changed. The scaffold (id/offset/length/kind/phase1) is unchanged.

All edits went through tools/names.txt and tools/apply.py. A diff against the previous reading copy shows that only the
intended lines changed.

## Results
- `commentary.py audit`: ok, byte_identical=true, reviewed_regions 1021, reviewed_units 1203, editorial_audit=true.
  Output is in commentary/audit.json.
- `commentary.py verify`: byte_identical=true. Output is in commentary/verification.json. The source game.asm sha256
  begins afbfa4bc1c2b7c12.
- The reading copy commentary/reading-after.asm was refreshed by audit.
- commentary/review.json: editorial_audit status=reviewed, with notes listing the checks and fixes.
- Accepted residual: the audit cannot judge prose. Some long stretches without inline comments, such as the unrolled
  UploadTileRow and RouletteSpotFromCursor, are explained by their routine headers and were reviewed in round 4.

## Residual uncertainties
- Semantic: see commentary/uncertainties.md. Open points include:
  - Lemon identification (from a render only).
  - "Degrees F" (inferred).
  - The no-reader spans 6:$B554, 6:$85CE, 7:$D0E4/$D0F0/$F398/$F3DC/$E195.
  - The 7:$C0D3 figure.
  - Sound effects $0D/$10.
  - The screen $02 selector.
  - The unread ClockTextTiles+4/+5.
- Structural: the residual generic-data spans and their rejection review are in analysis/handoffs/round-3-redirect.md.
  That review is sound on unchanged premises.

## Completion criteria (goal.md)
- Every byte has an evidence-backed classification: yes (100.00%, 0 unknown; every region has evidence).
- Required annotations: yes. All 1021 scaffold entries and all 1203 units are reviewed, and unknown meanings have
  mechanical explanations with explicit uncertainty.
- Valid maps: yes (map validate and store validate both ok; the checkpoints agree).
- Final commentary audit: yes (commentary/audit.json).
- Byte-identical verification: yes (commentary/verification.json).

Remaining for the coordinator: archive-options.json and the terminal handoff. No cleanup or archive was run here.
