# Handoff: round 1, phase 2 (commentary)

**Status**: INTERRUPTED / gate NOT fully met.  Coverage is complete (every classified region
has a symbolic name and comment, every unknown range has a note, review.json fully filled), but
depth is not: ~600 helper routines carry only mechanical reference summaries and branch targets
inside routines keep mechanical local labels (@l_xxxx / @loop_xxxx).  Resume commentary on the
remaining ranges below before redirect.
Whole-ROM classification remains INCOMPLETE (95.44%, 17,912 unknown bytes, unchanged).
Final editorial audit not run (reserved for final acceptance; editorial_audit pending).

**Checkpoint / baseline**: maps/round1-disassembly-b.json (map sha256 2b95d9dd...ccac),
unchanged; analysis/state.json not modified.  Commentary workspace: commentary/ (source/game.asm,
review.json, progress.md, uncertainties.md, tools/, specs/).

**Verification**: `commentary.py verify` ok, byte_identical true (final run after last edit);
`commentary.py read` refreshed commentary/reading-after.asm.  input/game.nes unchanged.

## Scope / readability assessment
- Covered: all 16 PRG banks' classified code (1,472 code regions, ~48 KB) and data/padding/
  text/CHR regions; 239 unknown ranges carry boundary/reference/decode notes.
  review.json: 1,966/1,966 scaffold entries reviewed with summaries + source lines; 847 units.
- Depth: bank 0 (engine, NMI, math, ball physics, fielding AI) and bank 15 $C000-$E000
  (PlayState machine, counts, scoring, ball flight) read in full and commented in prose;
  bank 15 $E000-$FFFF, banks 1-14 named from reference summaries plus targeted reads, with
  short functional/tentative comments.  ~600 helper routines carry mechanical reference
  summaries ("Mechanically: calls/writes/reads ...; Role inferred from the name; tentative").
  No address-only routine names remain (only D_/U_/C_ table-address labels).
- Symbols: ~170 RAM/hardware/MMC1 symbols; scratch, ball, runner, fielder record fields named.
- Remaining ranges for the depth pass (resume here; all physical offsets = file offsets):
  bank 2 $C696-$FF44 (file 0x86A6-0xBF54), bank 4 $C0C3-$E01F (0x100D3-0x1202F),
  bank 6 $C045-$CC80 (0x18055-0x18C90), bank 11 $C043-$D320 (0x2C053-0x2D330),
  bank 12 $C5A3-$CC40 (0x305B3-0x30C50), bank 13 $C04B-$EC20 (0x3405B-0x36C30),
  bank 15 $E014-$FBDD (0x3E024-0x3FBED), banks 8/9 field scroller internals.
  In these ranges: replace mechanical summaries with functional comments, give meaningful
  names to branch targets of non-trivial control flow, and name remaining raw RAM operands.
  Weakest areas: bank 2/4 front-end and
  cutscene scripts, bank 6/11 CPU AI, bank 12 sound driver internals, bank 15 $E000-$FBFF
  long routines (runner/fielder control) — comments there are summary-level.

## Findings
- $07A6/$07A7 are the umpire voice request/delay (bank 1 DMC player run from MainLoop), not a
  boot flag.  Bank 1 and bank 10 hold DMC voice-sample sets.
- $0354 = strikes (3 -> state $26), $0355 = balls (4 -> $28), $0356 = outs (third out ->
  PlayState $60); HalfInning $66, game end tested from half-inning $11 (feeds L-003).
- Password = 7 base-25 letters from team/day/wins + checksum (bank 4); SeasonDay >= $82 ends
  the season (ending/soft reset).
- Bank 11 entry 0 is a CPU "virtual controller" writing Joy1Held/Joy2Held.
- Bank roles summarised in the source header.

## Leads
Consumed: L-006.  Produced: L-007 (NMI task table $86F4), L-008 (PlayState table extent /
unlabeled state handlers), L-009 (runtime confirmation of named RAM; outs patch for L-003).

## Unresolved questions
Flag-bit meanings of $3B/$3D/$3E/$05E3; front-end screen numbering; physics units;
whether table-index labels beyond real table ends exist (L-002).  See commentary/uncertainties.md.

## Deferred work
Deeper prose for the summary-level areas above; final editorial audit; reclassification of
unknown ranges (redirect/disassembly).  Toolkit: TK-005 (missing feature, commentary
symbolization) added to analysis/toolkit-issues.md; no blockers.
