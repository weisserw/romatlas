# Redirect attempt r001-a (round 1, input checkpoint pass-001, maps/current.json unchanged)

Status: COMPLETE (research only; no map/source/ROM change).

## Files
- roots_spawn.json - 34 bank-8 + 23 bank-13 task entry words (MapSpawnTable/MapSpawnTable13, LevelInitTasks/LevelInitTasks13).
- roots_tables.json - bank 3 SpecialObjectHandlers (8), bank 6 EventObjectHandlers (26 words, 16 distinct), $F1E6, $F3D9.
- roots_all.json - union (input of the combined traversal).
- runs/trav_r001a_all.json.gz - combined traversal output (gunzip -> TRAV_OUT format of tools_local/trav.py).
- ledger.json - exact residual unknown complement after the proposed code (100 rows) with dispositions data/text/code-candidate/ambiguous.
- runs/obs_roots_r001a.json - observed contextual roots of this attempt (L5 CP, L3 CP, level-clear screen, credits; artificial routes).
- runs/l1cp_tasks.json, l3cp_tasks.json, l5cp_tasks.json - live task (TaskBank:resume PC) samples.
- runs/l3cp_start.json, l5cp_start.json - same-build snapshots at the start of CP gameplay (levels 3 and 5).
- leads.before.json - leads.json before this attempt.
- tools/: trav2.py (parameterised copy of tools_local/trav.py), cmp.py (overlap + per-gap gain), residual.py, resrefs.py (consumer per residual run),
  sprchain.py (sprite-definition chain parser), show.py/rdis.py (listing), opref.py, samebank_check.py (L005), tasksample.py, starttask.py (artificial task start), getctx2.py, ledger.py.

## Numbers (static traversal + ledger, not yet accepted)
- Combined traversal: 48651 code bytes (pass-001 36564) -> +12087 code bytes in unknown + 273 inline-word bytes (cmp.py: "total new code bytes in unknown" 12330 incl. inline);
  per bank code: b3 6667, b6 5644, b8 5898, b13 6090, b15 5950.
- Instruction overlaps with existing Data: 2 regions (30 bytes) = corrections (L004).
- Residual unknown after code: 6958 bytes -> data 6634 (70 rows), text 10, code-candidate 279 (12 rows), ambiguous 35 (8 rows).
