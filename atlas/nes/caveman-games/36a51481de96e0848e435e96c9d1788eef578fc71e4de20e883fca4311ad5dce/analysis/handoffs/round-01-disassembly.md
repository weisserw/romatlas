# Round 1 — disassembly handoff

- Scope: fresh workspace; baseline + one disassembly pass (static discovery, broad gameplay, map, checkpoint).
- Game: Caveman Games (Data East, NES 1990), MMC1 mode 3 (fixed $C000 = 8K banks 14/15).
- Output checkpoint: **pass-001b** — map maps/pass-001b.json, source generated/pass-001b,
  verification analysis/pass-001b-verification.json (ok, byte_identical true). pass-001 superseded (kept).
- Internal store: analysis/state.json; portable export maps/current.json.

ROM classification: 99.12% (259842 / 262160 bytes classified; 2318 bytes unknown).

## Findings
- Bank layout: 16K bank 0 menus/bios/results, 1 Mate Toss, 2 Fire Starting, 3 Dino Race, 4 Saber Race,
  5 Clubbing, 6 Dino Vault, 7 fixed engine (NMI common $C0BE, joypad $C564, dispatcher $CF24-$CFBF,
  final placings $D357, MMC1 stubs $FF00-$FF4F, reset $FF50). $FF00-$FFFF identical in every bank.
- Code 41428 bytes (30981 in Observed-provenance regions), 717 of them inferred (conf 0.8, not executed).
- Generic data 67928 (conf 0.9 where >=4-byte non-opcode runs, 0.7 otherwise), Text 5340, Padding 14074, CHR Image 131072.
- RAM: $0558/$0559 newly-pressed joypads (bit7 = A), $0562/$0563 raw; $0653 competition flag; $0659 competition
  event counter (7 -> final placings); $0657/$049D practice cursor.
- Toolkit static discovery stops at mapper writes (TK-003) — used local tracer tools/trace_static.py + tools/fulltrace.py;
  map assembled by tools/build_map.py and loaded via map.load/map.validate.

## Unresolved / deferred
- L-001 bank 3 $A60C-$AEF9 (file 0xE61C-0xEF09, 2254 B) mixed code/data, entries unknown.
- L-002 bank 2 $95E6 island (46 B); L-003 18 B residues next to inferred routines.
- L-004 confirm 11 inferred routine roots (callers / natural scenarios).
- L-005 specific roles for generic data (pointer tables, metasprites, tile maps, music streams, bios pointers at 0xACD).
- L-006 event controls: Saber Race / Dino Race not finished by mashing; 2-player mode not tried.
- Assumption to keep checking: MMC1 stays in mode 3 (all observed contexts had fixed 14/15; no control rewrite in classified code).

## Leads
- Consumed: none. Produced: L-001, L-002, L-003, L-004, L-005, L-006.

## Artifacts
- analysis/progress.md, analysis/investigation.md, analysis/leads.json, analysis/scenarios.json,
  analysis/gameplay.md, analysis/toolkit-issues.md (TK-001..TK-004, none blocking)
- Snapshots (same-build, local): analysis/snapshots/mainmenu.json, evsel.json, club_end_patched6.json
- Coverage union (local runtime data): scratch/cov_s1.json, scratch/cov_s2.json, scratch/cov.json
- Local tools: tools/bridge.py (persistent serve relay), rpc.py, drv.py, play.py, runto.py, d6502.py, trace_static.py,
  fulltrace.py, probe.py, islands.py, segment.py, build_map.py
