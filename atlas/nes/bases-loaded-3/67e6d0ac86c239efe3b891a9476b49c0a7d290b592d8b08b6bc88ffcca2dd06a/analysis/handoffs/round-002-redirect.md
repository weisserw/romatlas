# Round 2 — redirect phase handoff (attempt r002-a)

Status: **complete; no ready classification leads.** Recommendation: **terminal incomplete**
(classification 393229/393232; the 3 residual bytes are a genuine code-vs-data ambiguity under
evidence-policy.md). Readability/editorial work remains for whoever runs final editorial audit.

- Input checkpoint: pass-002 (byte_identical; maps/current.json sha256 9a0cd81b…7069). Unchanged:
  ROM, map, state, source, binary. Research only: analysis/redirect/r002-a/ (notes.md, tools/, evidence/,
  leads.before.json), analysis/leads.json (+L-015, L-016, pass record redirect-r002-a).
- Proposed bytes: 0. Accepted bytes: n/a. Residual: physical 193554 (23:$B402), 230960 (28:$8620),
  235667 (28:$9883), each 1 byte $60.

## Findings
1. **Three RTS bytes (L-015, rejected).** Exhaustive static reference search on pass-002 found no
   branch/JMP/JSR target, no data word (target or target-1), no immediate pointer pair, no abs/indexed
   read spanning them, no RTS/RTI-trick dispatcher anywhere in classified code, no duplicate copy of the
   surrounding routine elsewhere, no observed fetch. Idiom census: every terminator+RTS in classified code
   has a branch or jump-table reference; these three are the only unreferenced ones. Details:
   analysis/redirect/r002-a/notes.md, evidence/rts_refscan.txt, rts_idiom_census.txt, rtstrick.txt.
2. **PitchOutcome 2 / team banner (L-016, commentary-only).** Batter group 2 is entered only from
   BatContact @skip3 (28:$9670) when the batter is not swinging and the ball meets the batter (1/8 RNG);
   tentative hit-by-pitch. Its late-inning close-score variant (HalfInning>=10, Visitor-Home in -4..3,
   Var_0356>=10, LoopCounter&$70=0) skips the Hold step and ends with PitchOutcome 2 -> team banner.
   Game meaning (e.g. brawl/ejection scene) unconfirmed; runtime recipe in L-016.
3. **L-013 spot-check:** scan of all PRG Data for code signatures (JSR/JMP to fixed-bank entries,
   PPU/APU/MMC3 register accesses with clean preceding decode) found nothing conflicting
   (evidence/data_codesig_spotcheck.txt). The round-2 disassembly checks (2,511 control-flow targets,
   all observed starts on Code) stand.

## Rejection review (evidence-policy.md)
- Candidate claim: each byte (1 B, exact bounds; ctx [22,23,30,31] / [28,29,30,31]) is code (a dead RTS).
- Decisive evidence: rts_refscan.txt + rtstrick.txt (no entry of any kind); rts_idiom_census.txt (no
  precedent); coverage (no fetch). What it rules out: any known static or observed entry or data reader;
  it does not rule out either role.
- Independent constraints combined: exact 1-byte bounds; code neighbours on both sides; the always-taken
  `bcc +1` / `bne +2` (23:$B400, 28:$9880) and `rts` (28:$861F) before each byte; no reader; no entry. The
  bounds and neighbours fit both roles. The missing reader and missing entry count equally against both.
- Strongest competing interpretation: a 1-byte non-code constant or leftover byte embedded between
  routines (no reader needed for a leftover byte). "Dead duplicate RTS from the original source" is the
  more likely story, but it rests only on the byte value. The policy's own example ("an unreferenced
  return opcode ... lacks sufficient structure on its own") covers this case.
- Result: no supported proposal and no discriminating test left. Only an observed fetch or read would
  change this, and the static search shows nothing that could produce one. Running more scenarios would
  be an unfocused state sweep. Keep unknown unless a reviewer/policy accepts idiom-based classification;
  if one does, all 3 bytes would become Code (Static) and whole-ROM classification would be 100%.

## Deferred avenues (reasons)
- Runtime read/fetch watch on the 3 offsets: no static path or data reader could produce a hit, so a
  negative result would change nothing, and lack of execution does not prove data.
- Commentary questions 2/3/5 (wall ids, lineup/pitcher numbering, InitRamTables blocks, scoreboard/clock):
  semantic only, no unknown bytes depend on them; recipes stay in commentary/uncertainties.md for the
  final editorial pass.
- L-006 (EDIT/2P functions, still "ready" from round 1): those banks have no unknown bytes left; it would
  only add Observed provenance. Optional.

## Recommendation
**terminal incomplete** (3 bytes, reasons above). No disassembly round 3 is needed for classification.
If the coordinator/reviewer accepts the dead-RTS idiom, classify the 3 bytes as Code and go to final
acceptance; L-016 and the commentary depth gaps feed the final editorial audit either way.
Scratch: none created outside r002-a; no serve processes started.
