# Round 5 disassembly handoff

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input: checkpoint pass-004 (512 regions). Output: checkpoint **pass-005** (512 regions; maps/pass-005.json =
  maps/current.json = analysis/state.json, sha256 951ffb73...; verify ok=true, byte_identical=true,
  analysis/pass-005-verification.json).
- Code: Observed 11920 / Static 1939 (was 11917 / 1942). No kind or boundary change.
- L-019 verified: region 49 phys 2621..2623 ($8A2D `jmp $8A37`) Static -> Observed from r4-a1 E17 natural left-gutter
  coverage (whole region; no split). Script/log: analysis/evidence/pass-005/tools/apply.py, apply_log.json.
- Evidence text added (no type change): regions 292,294,296,306,308,311,357,361 (264 bytes) dormant by data flow;
  region 315 ($F656 FillNametable2/3) unreachable.
- Ready for commentary: L-020 (left gutter clamp, sign digit tiles, CONTROL label, dormant gated jobs,
  FillNametable2/3 unreachable, uncertainties.md). Region markers unchanged (no re-sync needed).
- Deferred: 472 reachable residual Static bytes (provenance only; r4 redirect dispositions); L-010 (editorial).
- Gameplay: none this pass. Toolkit: no new issues; TK-001..TK-005 kept (analysis/toolkit-issues.md).
- No server left running; scratch /tmp/p5peek.py removed. Markdown created: this file only.
