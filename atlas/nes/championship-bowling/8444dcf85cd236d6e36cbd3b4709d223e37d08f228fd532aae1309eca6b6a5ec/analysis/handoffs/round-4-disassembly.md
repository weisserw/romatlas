# Round 4 disassembly handoff

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input: checkpoint pass-003 (511 regions). Output: checkpoint **pass-004** (512 regions; maps/pass-004.json =
  maps/current.json = analysis/state.json; verify ok=true, byte_identical=true, analysis/pass-004-verification.json).
- Code: Observed 11917 / Static 1942 (was 11775 / 2084). No kind changes.
- L-014 verified: 11 regions (133 bytes) Static -> Observed from r3-a1 natural coverage; region 379 ($A845) split
  at the instruction boundary: $A845-$A84D Observed, new id 512 $A84E-$A850 (`jmp $A888`) Static.
  Script and log: analysis/evidence/pass-004/tools/apply.py, analysis/evidence/pass-004/apply_log.json.
- L-001/L-002: evidence text of the 23 regions now records "resolved-unreachable from vectors per computed-transfer
  census (r3-a1); structure still code". Classification unchanged.
- Ready for commentary: L-018 (RNG/celebration contract, tiles $42/$43, L-001/L-002 wording, uncertainties.md),
  plus re-sync one new marker at $A84E (id 512; no byte change).
- Deferred: residual 481 bytes of reachable Static branches (provenance only; no cheap natural trigger);
  L-010 (formatting, editorial).
- Gameplay: none this pass (S31 review in analysis/scenarios.json).
- Toolkit: no new issues; TK-001..TK-005 kept (analysis/toolkit-issues.md).
- No server left running by this pass; no scratch left. Markdown created: this file only.
