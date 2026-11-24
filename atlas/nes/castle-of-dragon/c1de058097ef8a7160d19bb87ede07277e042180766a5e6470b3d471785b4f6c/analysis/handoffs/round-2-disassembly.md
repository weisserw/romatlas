# Round 2 disassembly handoff

- Input: pass-001b + analysis/state.json. Output checkpoint: pass-002b (byte-identical; analysis/pass-002b-verification.json).
  pass-002 = intermediate (lead edits only), verified, superseded.
- ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown). Unknown: $C415 (phys 0x1C425), L-001 blocked.
- Code Observed 17653 (+4531), Static 7078. Banks 0-5 fully classified (Image/Data), bank 6 0 unknown, bank 7 1 unknown.
- Leads: L-003, L-006 verified (provenance upgrades, partial regions split at fetched boundaries); L-004 (21 typed bank-4/5 data
  regions) and L-007 (MsprFramePtrTbl / MsprFrameData) verified; L-002 commentary-only (rename *_Unused labels in commentary;
  map evidence already neutral; $CCE6-$CD06 conf 0.85); L-001 untouched.
- New scenarios (scenarios.json): S-WPN alternate weapons (artificial ownership), S-PROG3/S-PROG4 stage progression with RAM
  assists; S-BOSS3-6 deferred (boss rooms not reached).
- Tools/evidence: analysis/evidence/round-2-disassembly/{tools/apply_r2.py, tools/apply_cov.py, tools/newfet.py, traces/*.json.gz,
  scenarios/, frames/}. Scripts expect round-1 snap_map0 (gunzip to a scratch dir).
- For commentary: region boundaries changed (split Static/Observed runs in bank 6/7 code; bank 4/5/6 data split into labelled
  regions: Bank4MapLayout..MsprHeightTbl, Bank5CollisionMap..Bank5ZeroFill_BC68, MsprFramePtrTbl, MsprFrameData). Carry existing
  commentary forward; update "consumer unresolved" staff-text note (DrawMetasprite frames $F6-$FF via UpdateEndingScene2) and
  "[static-only ...]" markers for newly observed code.
- For redirect: only $C415 remains; review exists (round-1 redirect). Optional: boss rooms stages 3-6 (no classification gain).
- Toolkit: no new issues (TI-001..003 unchanged). No processes left running; scratch /root/r2scratch removed.
