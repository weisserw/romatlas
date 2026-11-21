# Round 1 disassembly handoff

Scope: first disassembly pass on a fresh workspace (baseline created), static + traced gameplay discovery.

- Checkpoint: **pass-001** — verification ok (byte_identical true; analysis/pass-001-verification.json; maps/pass-001.json; generated/pass-001).
- ROM classification: 99.98% (131057 / 131088 bytes classified; 31 bytes unknown).
- Code 24893 B (709 regions; 293 Observed, 416 Static), data 106164 B (264 regions). Per-bank table: analysis/progress.md.
- Status: incomplete.

## Findings
- MMC1 16K-switch convention, SelectPrgBank $C14E/$C000, FarCall $C852 (+3 inline bytes, target-1), CHR loaders $C58A/$C5AC,
  sound driver entry $D800 with data bank $0202 — details in analysis/investigation.md.
- Code found mostly statically (tools/trav.py); traced gameplay added observed contexts for dispatch/trampoline targets.
- 8 dispatch tables + one two-level record table verified (tools/tables_spec.py); 6 unreferenced routines classified structurally.
- CHR graphics chains in 16K banks 1 and 6 proven by exact compressed-chunk tiling; 16K banks 3/4 lower halves are sound-driver
  data banks; fixed $E000-$FF77 generic data (role unknown).
- RAM shortcuts: $4B world, $4C stage, $04E5 lives; ending via PC=$CA5B (analysis/gameplay.md, analysis/scenarios.json).

## Unresolved / deferred
- 31 unknown bytes (L-001); possible short routines inside generic-data gaps (L-002); further indirect dispatch (L-003);
  bank-15 data consumer (L-004); bonus stage / game over / bosses / battle arena not reached (L-005);
  reachability of Static/unreferenced code (L-006); data semantics for commentary (L-007).
- Static code reachability is not claimed; generic data meanings not claimed.

## Artifacts
- analysis/progress.md, analysis/investigation.md, analysis/leads.json, analysis/scenarios.json, analysis/gameplay.md
- analysis/toolkit-issues.md (TK-001..TK-004; no blocker)
- analysis/evidence/ (observed contexts, region build inputs, input log, frames); tools/ (local analysis scripts)
- scratch/snaps/{title,menu,truck,area11,ending0}.json (same-build snapshots, ~4 MB each)
