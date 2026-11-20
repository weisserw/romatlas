# Round 2 disassembly pass 2 handoff

- Checkpoint: pass-002 — maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json (ok, byte_identical). analysis/state.json == maps/current.json.
- ROM classification: 99.35% (260469 / 262160 bytes classified; 1691 bytes unknown).
  Per-bank counts in analysis/progress.md. Status: incomplete.
- Leads: L-010..L-019 all verified (independently recomputed; see analysis/leads.json results).
  Produced L-020 (bank 6 record-like residuals after BG handlers), L-021 (ambiguous code vs.
  table fragments: b0 $E867-$E872, b4 $B8BD-$B8D0/$B979-$B986, b5 $831E-$8321, b2 $FC09),
  L-022 (other residuals incl. stub tails $FF6E-$FF97 / L-006, b5 $BB49-$BC46, $BC67-$BCD6,
  b2 $FC94-$FD33, b0 $84F0-$854F). Full list: scratch/r02_unknown.txt (99 runs).
- Uncertainty for commentary/review:
  - 12 uncalled fragments + L-019 pieces are Static code with no caller (confidence 0.8-0.9);
    $A381 has a caller ($80D3, itself uncalled). Evidence strings name each.
  - Pruned fall-throughs ($B45D, $B9CA, $BE7E, $F076, plus pass-1 ones) assume table-driven
    always-taken branches; bytes after them are tables classified by consumer refs.
  - Large data blocks are generic/role-labelled by consumer (redirect formats); internal
    record layouts are not proven.
  - New regions need commentary names/blocks (handlers from $9A71/$9C4E tables especially).
- Scenarios: game over visited (artificial RAM edit, traces/round02_gameover_contexts.json,
  all fetches on classified code). Deferred: 2-player modes, credits after ending
  (snapshot traces/r01a1_snap_ending.json).
- Toolkit: no new issues (analysis/toolkit-issues.md, TK-001..TK-003).
- Servers shut down.
