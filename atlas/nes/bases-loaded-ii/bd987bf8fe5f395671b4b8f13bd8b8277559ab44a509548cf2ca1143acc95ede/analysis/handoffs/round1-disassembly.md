# Handoff: round 1, phase 1 (disassembly)

**Scope**: fresh workspace → verified baseline, static discovery (bank-aware), traced normal
gameplay scenarios (title, 1P/2P select, league/team select both leagues, lineup menus,
starting pitchers, game intro, pitching, batted balls, field view/manual fielding, home run,
2P "INPUT PLAY DAY" and team select), structural data classification.

**Checkpoint**: `maps/round1-disassembly-b.json` (source `generated/round1-disassembly-b`,
report `analysis/round1-disassembly-b-verification.json`). Superseded same-pass checkpoint
`maps/round1-disassembly.json` retained.

**Verification**: map.validate ok; `run.py checkpoint` ok, byte_identical true; map sha256
2b95d9dd93359eeb02fdbaaf26398dc18e4c29165fc77419a0c72eb38517ccac; input ROM unchanged.

ROM classification: 95.44% (375320 / 393232 bytes classified; 17912 bytes unknown).

Per bank (classified / unknown): header 16/0; PRG00 15362/1022; PRG01 15360/1024;
PRG02 11595/4789; PRG03 14518/1866; PRG04 14023/2361; PRG05 15836/548; PRG06 16241/143;
PRG07 16100/284; PRG08 16258/126; PRG09 16258/126; PRG10 15732/652; PRG11 14356/2028;
PRG12 16213/171; PRG13 16159/225; PRG14 15015/1369; PRG15 15206/1178; CHR 131072/0.
Code 48,229 (30,270 Observed, 17,959 Static), Data 112,707, Padding 83,298, Image 131,072,
Text 14. Execution coverage (separate metric): 14,438 observed instruction starts.
Full table: analysis/progress.md.

**Findings**: MMC1 PRG mode 2 model, FarCallBank $84BC + per-bank $C03F entry tables,
far-call trampolines, jump-table dispatchers $849F/$9AA2, bank 8/9 12×1 KiB blocks, bank
tail layout (ID byte/padding/vectors; bank 14 anomaly). Details: analysis/investigation.md;
gameplay/RAM notes: analysis/gameplay.md; routes: analysis/scenarios.json.

**Leads consumed**: none (none existed). **Produced**: L-001 (ready), L-002 (ready),
L-003 (ready), L-004, L-005, L-006 — analysis/leads.json.

**Unresolved questions**
- Which unknown islands are unreached code (L-001) and what are the extents of 14 $849F tables (L-002)?
- Outs/half-inning/game-end RAM contract ($0354 is not it) to reach batting, late innings,
  game end, season progression and ending (L-003). Password/CONTINUE flow not confirmed.
- Roles of large generic data blocks (L-004).

**Deferred work**: batting/late-game/ending milestones (scenarios.json status deferred);
labels/commentary for 1,727 regions (only ~40 labels assigned); trace-speed workaround limits
(TK-004).

**Toolkit**: analysis/toolkit-issues.md — TK-001 (missing feature: MMC1 static discovery stops
at JSR/indexed stores), TK-004 (missing feature: trace ~12x slowdown), TK-002/TK-003 (suspected
doc mismatches). No blockers.

**Status**: pass complete (easy gains taken); ROM classification INCOMPLETE.
