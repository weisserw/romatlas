# Round 3 disassembly handoff

- Input: pass-002b + analysis/state.json. Output checkpoint: pass-003 (byte-identical; analysis/pass-003-verification.json,
  map sha256 fc23d9a5...). No intermediate checkpoints.
- ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown). Unknown: $C415 (phys 0x1C425), L-001 blocked.
- Code Observed 21962 (+4309), Static 2769. Banks 0-5 fully classified (Image/Data); bank 6 Observed 8348 / Static 2142 / Data 5894;
  bank 7 Observed 13614 / Static 627 / Data+Text 2142 / Unknown 1. 751 regions (was 643).
- Leads: L-008 verified (3760 bytes; scenario ids per region; 108 bytes in S-BOSS5F/6F-only runs marked FORCED PC entry, not natural
  reachability). L-010 new + verified (test-mode SPTEST/BGTEST via natural cheat route, 549 bytes, scenario S-TMSUB).
  L-009 untouched (commentary). L-002 unchanged. L-001 not retried.
- For commentary: region boundaries changed where Static runs were split (bank 6 boss/guardian code $A3BE-$AEB7, $B4C2-$B541, bank 7
  $C5D4-$C879, $DE64-$DF3A, $E1AA-$E28E, $FB89-$FCD8, etc.). Carry commentary by address; drop "[static-only ...]" markers for
  newly Observed runs; apply L-009 corrections (incl. SpawnStage7BossBody rename, stage 5/6 boss cut-content notes).
- For redirect: only $C415 remains (review stands). Remaining Static code is provenance-only (list in investigation.md).
- Evidence/tools: analysis/evidence/round-3-disassembly/{tools/apply_l008.py, tools/tm_sub.py, tools/apply_tmsub.py,
  l008_upgrades.json, tmsub_upgrades.json, cov_tmsub.json.gz, frames/}. tm_sub.py expects snap_cheat.json gunzipped to /root/r3scratch.
- Toolkit: no new issues (TI-001..TI-004 unchanged; tracing slowness TI-003 hit again: 2358 traced frames took ~20 min).
- No processes left running; scratch /root/r3scratch removed.
