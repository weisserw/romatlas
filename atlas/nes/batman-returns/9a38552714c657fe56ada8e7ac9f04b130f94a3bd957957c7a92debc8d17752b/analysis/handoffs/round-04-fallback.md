# Round 4 fallback annotation handoff (pass-004)

- Scope: user-authorized residual fallback only (batch-only exception, <=10 residual bytes; explicit user
  authorization 2026-11-19). No new research on the bytes; no other region added, removed or altered.
- Input checkpoint pass-003 (maps/pass-003.json == maps/current.json == analysis/state.json before the edit).
  Complement re-verified via analysis.unknown before editing: exactly 3 bytes (3 <= 10, exception applies):
  - phys 0x14CCA (85194), bank 10 CPU $8CBA -> region 1907
  - phys 0x17BC6 (97222), bank 11 CPU $BBB6 -> region 1908
  - phys 0x1FD5A (130394), bank 15 CPU $FD4A -> region 1909
- Annotation: each a 1-byte region, kind Data, provenance Manual, confidence 0.3, no label/name, views [] (the
  toolkit rejects execution views on non-Code regions: "execution views belong to code regions only"; the CPU/bank
  context is listed above and in analysis/tools-fallback/fallback_annotate.py). Evidence and comment text:
  "User-directed fallback (explicit batch authorization 2026-11-19, <=10 residual bytes): generic Data, meaning
  unknown. NOT an observed or statically proven data role; code-vs-data remained materially ambiguous (lone
  unreferenced RTS-valued byte; see analysis/handoffs/round-03-redirect.md). Reachability not claimed."
- This is a user-directed classification, NOT an evidence-backed data role. Prior ambiguity is preserved by
  reference: analysis/handoffs/round-03-redirect.md, round-03-disassembly.md; evidence analysis/redirect/r02-a/,
  analysis/redirect/r03-a/, analysis/tools-round3/; lead L017 (round3_outcome.reject_reason).
- Files: analysis/tools-fallback/fallback_annotate.py (script), fallback-regions.json (records submitted),
  fallback-apply-result.json (ids, unknown before/after).
- Results: map.validate ok; map.save analysis/state.json; map.export maps/current.json (include_sym false).
  analysis.unknown after edit: total 0. Diff vs maps/pass-003.json: 1906 prior regions identical (full record),
  none removed, added 1907-1909 only; next_id 1910.
- Checkpoint pass-004: analysis/pass-004-verification.json ok true, byte_identical true,
  map_sha256 e6079a04cdeb26ab9fd99f3f588a3116bc98ba4bc35f1ca4e6d3ea6607d8201a; maps/pass-004.json == maps/current.json.
  input/game.nes sha256 unchanged (9a385527...).
- ROM classification: 100.00% (393232 / 393232 bytes classified), of which 3 bytes (regions 1907-1909) are Manual
  user-directed fallback Data with unknown meaning and are NOT evidence-backed.
- Commentary: the 3 new 1-byte Data regions (1907-1909) need source comments stating meaning unknown + user-directed
  fallback (not an observed/proven data role; byte value $60 = RTS opcode; code-vs-data ambiguous; reachability not claimed).
- No server left running. Toolkit: undocumented NES rule (views only on Code) recorded as TK-006 (doc, minor).
