# Round 6 disassembly handoff

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input: checkpoint pass-005 (512 regions). Output: checkpoint **pass-006** (512 regions; maps/pass-006.json =
  maps/current.json = analysis/state.json, sha256 865f4d5a...; verify ok=true, byte_identical=true,
  analysis/pass-006-verification.json).
- Code: Observed 11922 / Static 1937 (was 11920 / 1939). No kind or boundary change.
- L-021 verified: region 386 phys 10555..10556 ($A92B `lda #$00`) Static -> Observed from r5-a1 E19 natural
  first-ball-miss open frame (whole region; no split). Script/log: analysis/evidence/pass-006/tools/apply.py, apply_log.json.
- Evidence text added (no type change): fade regions 405,407,335,333,337 (160 B, dormant by FadeDelay data flow);
  sound 270,272,276 (8 B, dormant by ROM sound data); carries 166,168,403 (6 B, impossible); 340 $FB8F (14 B, $FD
  jump unused by ROM sprite lists); 348 $FC23 (15 B, top-edge clip unobserved, deferred).
- Ready for commentary: L-022. Region marker for 386 changes provenance (Static -> Observed); no boundary change.
- Deferred: $FC23 provenance (no discriminating test); L-010 (editorial).
- Gameplay: none this pass. Toolkit: no new issues; TK-001..TK-005 kept (analysis/toolkit-issues.md).
- No server of this workspace left running; no scratch files. Markdown created: this file only.
