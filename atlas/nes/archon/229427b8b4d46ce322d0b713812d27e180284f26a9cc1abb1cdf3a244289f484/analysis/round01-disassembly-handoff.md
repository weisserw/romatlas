# Round01 disassembly handoff

Status: one pass complete; ROM incomplete; ready for full current-source commentary and redirect.

ROM classification: 83.57% (109551 / 131088 bytes classified; 21537 bytes unknown).

- Code:10739 bytes; Data:98812; unknown:21537. All 5147 classified regions evidence-backed; 5116 instruction regions with CPU views. Per-bank counts/exact unknown complement: [round01-metrics.json](round01-metrics.json).
- Internal state:[state.json](state.json); portable map:[current.json](../maps/current.json); fresh verified checkpoint:[pass-001-final.json](../maps/pass-001-final.json); source:[generated/pass-001-final](../generated/pass-001-final); verification:[pass-001-final-verification.json](pass-001-final-verification.json). Byte-identical, unchanged ROM SHA256.
- Accepted: vector/observed/static code; six16KiB non-code graphics/screen-data banks; header, vectors, bank-selector constants, exact stack-inline parameters and uniform FF Data. Full local bitmap inspection:[r01-all-bitmap-banks.ppm](r01-all-bitmap-banks.ppm). Mixed generic Data roles have not been invented into specific tables.
- Fetch evidence:4270 actual instruction starts,8878 physical bytes across natural and artificial routes. [Conflict audit](round01-fetch-conflict-audit.json):0 Data conflicts. Physical1F684 is intentional interior TXA entry within1F683 BIT instruction, documented in map. Mapper contexts into graphics banks do not establish code there.
- Gameplay:[scenarios.json](scenarios.json), [gameplay.md](gameplay.md), [session log](round01-session-rpc.jsonl). Natural menu/light setup and board; artificial knight/adjacent-enemy shortcut visited/exercised arena. Ending and wizard shortcuts bounded and unsuccessful. Exact clean frontier:r01-before-shortcuts.snapshot; selected artificial:r01-knight-selected.snapshot; visited arena:r01-arena-visited.snapshot.
- Leads:[leads.json](leads.json). Verified4 lead groups; blocked ending/spell triggers need new source-derived contracts. Ready additional stack-inline helpers (concrete offsets and recipe supplied) plus graphic/screen-data consumer subdivisions. Follow connected code/data batch before cycling; no byte quota. This handoff does not assert terminal exhaustion.
- Commentary: full current source still needs useful routine/branch/variable labels and behavior explanations; classification and execution coverage do not imply editorial completeness. Preserve structural evidence and inline barriers.
- Toolkit:[toolkit-issues.md](toolkit-issues.md):1 confirmed docs/API bug NES-001, no blockers. NES input uses {port:1,buttons:[...]} replacement state; documented button/pressed silently ignored. Do not repeat old attract-only routes as gameplay. No toolkit changes authorized/performed.
- No archive, publish, cleanup or deletion performed. All ROM/state/source/assets local. Resume new process with map.load and emulator.start/import snapshot, reassert complete buttons arrays.
