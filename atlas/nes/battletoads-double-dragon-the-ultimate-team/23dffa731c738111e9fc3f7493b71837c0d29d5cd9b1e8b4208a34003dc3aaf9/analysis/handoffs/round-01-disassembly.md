# Round 1 disassembly pass 1 handoff

- Scope: baseline + first easy-gain pass (static recursive descent + traced gameplay).
- Checkpoint: pass-001 — maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json (ok, byte_identical). Internal store
  analysis/state.json == maps/current.json.
- ROM classification: 18.50% (48496 / 262160 bytes classified; 213664 bytes unknown).
  Per-bank counts: analysis/progress.md.
- Findings: analysis/investigation.md (bank-switch stub architecture, far-call
  JMP tables, stage id $10, dispatch tables). Game/route notes: analysis/gameplay.md.
  Scenarios: analysis/scenarios.json. Snapshots: traces/round01/snap_charsel.json,
  traces/round01/snap_lvset.json (same-build only). Coverage:
  traces/round01/coverage-contexts.json. Tools: analysis/tools/round01/ (local
  RPC client, recursive-descent analyzer, map generator; rerun mkmap.py to
  regenerate the map from coverage + rules).
- Unresolved questions: main-thread/indirect dispatch roots (L-001); graphics and
  compressed data extents (L-002, largest residual); RAM-code origin (L-003);
  invalid JMP slots/IRQ vectors (L-004); bank 6 text (L-005); $FF78/$FF84 tails
  (L-006); stage $01/$07, ending, game over (L-007); small gaps (L-008); 2P (L-009).
- Uncertainty: heuristic pruned fall-throughs (bank 0 $B45D, $B9CA; bank 2 $FC92)
  keep only the branch and its taken target; static JSR-return assumption relies on
  a 16-instruction callee scan for stack pops (manual review found only balanced
  pushes). Review in commentary.
- Deferred work: ending/credits route, data classification, toolkit issues TK-001..003
  (analysis/toolkit-issues.md; no blocker).
