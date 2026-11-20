# Round 2 commentary handoff

Status: COMPLETE. All currently classified source (pass-002, 100.00% classified) is
at the naming/comment standard; fresh byte-identical verification passed; reading
copy refreshed. Final editorial audit is still pending (separate acceptance gate).

## Baseline / carry-forward
- The map changed (295 new regions + region 657 bank 0 $8055-$805D Data->Code), so
  the whole round-1 commentary was moved, untouched, to analysis/round-1/commentary/
  (the only prior copy), and `commentary.py prepare` built a fresh frozen baseline
  from pass-002 (commentary/before, baseline.json, phase1-state.json).
- Round-1 notes (keyed by bank/CPU address, i.e. physical ranges) were carried and
  re-applied by commentary/tools/annotate.py. Invalidated and rewritten: 82 comment
  blocks on formerly unknown ranges, the per-bank "unclassified" defaults, three bank
  overviews, and 10 prose references to auto table names whose numbering shifted
  (found by an old/new name-to-address diff). Every remaining note key was checked to
  land on an instruction, label or region start (tools/checknotes.py: 0 problems).
- Round-1 defects fixed: four cross-bank X tags on wrong instruction addresses, the
  bank-6 alias of $0687-$069E (showed race-mode names), sound-queue operands shown as
  PartIndicator+n (exact variable starts now win over array spans), bank-3 RAM shown
  with shop/text names, stale cross references.

## Scope / readability
- New regions named and explained: bank 1 EventScreenRaw + 21 RLE screen streams
  (named by screen id / user); bank 0 WarmStartMenu, PalettePtrTable/Palettes,
  ScriptPtrTable/ObjectScripts, StoryTextBlocks, StopNameStrings, DistanceScoreTable;
  bank 2 song tables, NotePeriods, title palette tables, music stream area; bank 3
  TrackBlockPtrs/Metatiles/TrackBlocks/TrackMaps and CpuPathListPtrs/Lists/Records;
  bank 4 AnnouncerSample, SfxRecords, ResultsText, FontTileMap; bank 5 complete event
  data layout and engine tables; bank 6 HillNmiStepTable + six HillNmiShift*/upload
  routines, hill terrain records/index/blocks/height profile; fixed bank
  LoadScreen_Tbl record format, SfxPtrTable, SampleDescriptor, debug hex print pair,
  short tables. Data layouts carry an explicit extent; every region inside one is
  marked "Continues <name>".
- Low-confidence code described with explicit uncertainty: 0:$B1E2 (pla/tax residue),
  6:$A61E (lone rts), 0:$8395 and F:$D827 (unreached else-arms), plus other uncalled
  static islands (copies, debug helpers, unused results helpers).
- Whole-source pass: header comments added to about 230 routines/shared tails that had
  only a name in round 1 (all 530 code labels now have a header); unnamed branch
  targets get mechanical descriptive names (@check_X, @call_X, @set_X, @loop_...);
  210 plain @skip labels remain where the target has no symbolic operand.
  Cross-bank operands resolved where the mode fixes the bank (palettes, finish line,
  scenery, frame pointers, reset/warm-restart targets, font map).
- review.json: 1385 entries reviewed (1089 carried with unchanged ranges, 296
  new/changed, field review_round), 1310 units (every code entry has a unit);
  editorial_audit pending.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: byte_identical
  true (commentary/verification.json). Reading copy: commentary/reading-after.asm.

## Unresolved meanings (details in commentary/uncertainties.md)
- Bank 6 special sequence purpose (lane shift past the nametable height) is an
  interpretation; HillSpecial 1-12 never observed.
- Roles of 0:$A000-$A844, 2:$A000-$AC37, 2:$B082+ tail, 4:$A474-$A873, 5:$8388-$8777,
  small gaps (0:$8F84, $BE6F, 6:$8400/$8701, F:$FE12); music byte format per song.
- PalettePtrTable slots $0000/$0491; SfxPtrTable entries >= 12 point into code.
- Carried: garage part identities, secret button sequences, ScriptOp1D `jsr $0014`,
  NmiHandler2 `jsr $8592` bank, NmiHandler13 $B0BF/$B0D3 bank.

## Structural questions for redirect (bounded checks)
1. HillSpecial reachability: RAM-patched hill climb with a watch on $066A / bank 6
   $8DA6 to confirm the lane-shift interpretation.
2. Readers of 0:$A000-$A844 and 2:$A000-$AC37: read watches during a cross-country
   race and the title sequence.
3. SfxPtrTable length: watch PlaySoundEffect's A for ids >= 12.
4. NmiHandler2 `$8592` and NmiHandler13 `$B0BF/$B0D3` bank at run time (PRG-bank
   watch at $F3C1 / $F737).
5. Carried: writes to $0014-$0020 before ScriptOp1D's `jsr $0014`.
No map change is proposed; all bytes are classified.

## Remaining gaps
- None in the current-source readability scope. Known weaker spots, left for final
  editorial cleanup: 210 generic @skip labels; ~170 data tables named after their
  consumer routine (X_TblN) rather than by role; field layouts inside metatile /
  path / animation records only partly described.

## Artifacts
- commentary/source/{game.asm,ram.inc,hardware.inc}, commentary/reading-after.asm,
  commentary/review.json, commentary/progress.md, commentary/uncertainties.md,
  commentary/notes/ (round-2 files r2_*.txt), commentary/tools/{annotate.py,review.py,
  checknotes.py}; round-1 archive analysis/round-1/commentary/.
- Toolkit: no new issues (TK-001..TK-006 unchanged).
