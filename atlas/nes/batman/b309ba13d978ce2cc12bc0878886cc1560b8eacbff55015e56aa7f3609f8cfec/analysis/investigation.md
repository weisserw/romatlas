# Investigation ledger

## Round 1 (disassembly, checkpoint pass-001)

Method summary (tools in analysis/tools/):
- Coverage: bounded traces (capacity 0 aggregates) over scenarios S1-S4 (analysis/scenarios.json); merged contextual
  opcode-fetch roots in analysis/observed/contexts.json (12,662 unique offset/CPU pairs).
- Static code: sa.py bank-aware recursive descent. Fixed $C000=bank 14/$E000=bank 15 (all observed contexts).
  Modeled: inline-argument helper $C4F3 (+2 bytes via $C48D), jump engines (bank 14 $C588, bank 2 $8792 copy) with
  inline word tables (13 tables, extent = lowest same-bank handler / next observed code / unresolvable word),
  sound dispatch tables $86C0 (36) and $89CB (9), bank-head JMP vector tables (banks 0,3,4,5,7,8, and $C000-$C029),
  always-taken branches after flag-setting immediates, partner banks for cross-window calls (12<->13, 3->2, 5->2;
  all observed cross-window calls agreed). Rejected/pruned: any static path into BRK/unofficial opcodes or
  conflicting boundaries (0 remaining).
- Exclusions: bank 8 $A020-$A03B and $8001 observed only in an artificial crash trail (BRK, unofficial $5C/$6B,
  misaligned) -> not code evidence (analysis/observed/exclude.json).
- Orphan routines: orphans.py accepted 9 unreferenced routines (366 B; e.g. bank 0 $8641/$866D sound pause/resume
  ending exactly before table $86C0; bank 2 $874D/$875E MMC3 CHR-bank setters). Reachability unresolved.
- Data: header, CHR ROM (Image), DPCM samples $F000/$F100/$F300 (from DMC records $89DD), CPU vectors,
  inline args, jump/pointer tables; generic data by classify_gaps.py chain test (no byte on a clean official decode
  chain >= 12 bytes; head trimmed if a clean decode reaches RTS/JMP). Hold-out check (removing observed roots of
  bank groups and re-running) showed ~62 B false positives over ~33 KiB held-out code, all short fragments beside
  inline-argument/jump-table calls (lead L-005). Text (letters $40-$5A), periodic patterns and word tables detected
  with zero hold-out false positives. Bank-role data for banks 1, 9, 10, 11 (no fetch anywhere, observed data reads,
  mapped as data partners); bank 10 tail is a stale copy of bank 14 $DD00-$DFFF (7 absolute self-references to $Dxxx).
  Residual stretches with observed non-code data reads (analysis/observed/rom_reads.json) classified as data.

Outcomes: 98.19% classified (257,410 / 262,160); 4,750 bytes unknown in 306 stretches.
Remaining unknown (>=16 B stretches listed in analysis/handoffs/round-001-disassembly.md): mostly clean-decoding
stretches in banks 0,2,3,5,6,7,8,12,13,14,15 where code vs data is undecided (unreached cutscene routines, tables).
Leads: analysis/leads.json L-001..L-006.

## Round 2 (disassembly, checkpoint pass-002)
- Applied the r1-a1 + r1-a2 proposal batch after re-verification (analysis/tools/r2_verify.py).
  - Checks: unknown-only bytes, single bank per range, no inter-group overlap, no observed instruction byte inside data, and exact code tiling.
  - All 33 groups were accepted.
- R-06 / L-007: bank 13 $BCB4-$BD56 is now Code (Observed-artificial runs plus 2 Static runs), replacing the pass-001 data region.
- L-005: verified with no reclassification. Code-to-non-code boundaries are all inline-argument/jump-table JSRs or an always-taken BNE.
- Outcome: 262,158 / 262,160 classified. Residual 0x1C5AE-0x1C5AF ($C59E) remains unknown (L-018); the r1-a2 rejection review still holds.

## Round 3 (disassembly, checkpoint pass-003)
- L-019 provenance merge: analysis/tools/r3_build.py recomputed r2-a1 artificial-scenario fetches (analysis/redirect/r2-a1/observed/contexts.json)
  against pass-002 state: 7,480 already Observed, 142 new starts (identical set to observed-merge.json), 0 fetches in non-code or unknown bytes,
  0 misaligned/CPU/bank-slot mismatches. 17 Static Code regions split into 19 Observed (artificial; tags clr6/clr6b/clr16/clr16b/clr17) runs, 308 B,
  and 7 Static runs, 128 B (analysis/tools/r3_apply.py; edit list scratch/r3/edits.json). Code provenance now 28,544 B Observed / 12,579 B Static.
  Artificial execution only; natural reachability not claimed.
- L-018 not reopened (no new evidence). No gameplay this pass: S1-S7 all visited; further play can add coverage only.
- Outcome: 262,158 / 262,160 classified (unchanged); pass-003 byte_identical.

## Round 4 (disassembly, checkpoint pass-004)
- L-020 provenance merge: analysis/tools/r4_build.py recomputed r3-a1 artificial-scenario fetches (analysis/redirect/r3-a1/observed/contexts.json)
  against pass-003 state: 5,377 already Observed, 72 new starts (identical set to observed-merge.json), 0 fetches in non-code or unknown bytes,
  0 misaligned/CPU/bank-slot mismatches. 4 Static Code regions split into 5 Observed (artificial; tags cont15/clr15f80/clr15f00/clr15f80b) runs, 153 B,
  and 2 Static runs, 48 B (analysis/tools/r4_apply.py; edit list scratch/r4/edits.json). Code provenance now 28,697 B Observed / 12,426 B Static.
  Artificial execution only; natural reachability not claimed. Commentary part of L-020 open.
- L-018 not reopened (no new evidence). No gameplay (S1-S8 visited).
- Outcome: 262,158 / 262,160 classified (unchanged); pass-004 byte_identical.

## Round 5 (disassembly, checkpoint pass-005) — user-authorized manual fallback
- Explicit user authorization (batch-only exception, applies only when <=10 physical ROM bytes remain unclassified): mark the
  residual as generic Data of unknown meaning with Manual provenance. Verified unknown complement at pass-004: exactly file
  0x1C5AE+2 (PRG 14 $C59E-$C59F). Annotated as Data, provenance Manual, confidence 0.5 (region id 2265; analysis/tools/r5_apply.py).
  No other region changed (state diff: 0 removed, 1 added).
- This is NOT observed and NOT statically proven. L-018 evidence status stays exhausted-unknown (r2-a1/r3-a1 reviews: no
  references, no fetches, code-vs-data undecided); the Manual region records the user-directed fallback only, not a meaning.
- Outcome: unknown complement 0 B; 262,158 B evidence-backed + 2 B manual fallback; pass-005 byte_identical.
