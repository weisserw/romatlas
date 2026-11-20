# Round 1 — disassembly handoff

## Scope
Single easy-gain disassembly pass on a fresh workspace: inspect + baseline, static discovery from vectors,
broad gameplay scenarios with aggregate tracing, ROM data-read sampling, local bank-aware recursive descent,
rule-based data classification, map.annotate (internal store), validate/save/export, checkpoint.
Game: The Blue Marlin (Hot-B, USA, 1991). MMC3 in PRG mode 0: $C000=bank 14 and $E000=bank 15 are fixed;
R6->$8000, R7->$A000 (see analysis/investigation.md).

## Checkpoint and verification
- Baseline: maps/baseline.json, analysis/baseline-verification.json (byte_identical true).
- **pass-001**: maps/pass-001.json, generated/pass-001/, analysis/pass-001-verification.json —
  `ok: true, byte_identical: true`, map sha256 da6cea30ad3e113d6670195aa9a02961d8661efd08bb5a75f2dfbcbbd81adf66.
- Internal store analysis/state.json; export maps/current.json (map.validate ok).

## Classification (internal map)
ROM classification: 93.23% (244422 / 262160 bytes classified; 17738 bytes unknown).

| Kind | Bytes |
|---|---|
| Code | 24,646 (Observed 18,537; Static 6,109; 626 regions) |
| Data | 67,641 (incl. 16-byte iNES header, vectors, jump tables, inline args) |
| Text | 13,065 |
| Padding | 7,998 |
| Image (CHR ROM) | 131,072 |
| Unknown | 17,738 |

Per-bank code/data/unknown: analysis/evidence/round-01/perbank.md. Execution coverage is separate: 9,011 ROM opcode
addresses observed (analysis/evidence/round-01/observed-coverage.json.gz). Status: **incomplete**.

## Scenarios exercised (analysis/scenarios.json)
S01-S10 visited: boot/intro/title, START GAME/RESUME menu, name entry, weather, cockpit, harbor and sea navigation,
pause, COMMAND menu (information pages, line/length/lure/depth submenus), wrong password, artificial bite
(RAM $0496=1) -> fight screen with all input types and SELECT "WHAT WILL YOU DO?" options (line cut, lost fish,
return to sea), hidden SOUND TEST (hold SELECT+START at power-on; entries 0-47).
S11-S15 attempted/deferred: landing a fish, end of day/results/password output, Hawaii areas, tournament ending,
boat damage/return-to-port. Reproducible route: analysis/tools/routes.py; replay: analysis/tools/cover_replay.py.

## Findings
- Fixed-bank engine: reset $E000, NMI $E124 (OAM DMA, CHR bank refresh from $3B-$40), IRQ $E1B4, joypad $C025,
  sound far-call $E208 (banks 2/3, `jsr $8000`), main navigation loop $E37E, fishing loop $E465.
- Return-address helpers: $DD3E (inline jump table), $DDEA (7 inline bytes), $DD6E/$DD79->$DDC4 (2 inline bytes);
  script interpreter $F003 with 75-entry command table at $F013 (handlers $F0A9-$F51B).
- Banks: 0/1 gameplay/fight logic (mapped together); 2 sound driver + 3 music data; 4-7 mostly graphics/layout data
  with small renderers; 8-11 sea map/metatile data (no execution observed); 12/13 fight & results text/scripts
  (+ small code); CHR ROM tiles.
- Debug: SELECT+START held at boot enters SOUND TEST ($E315 -> $FA27).

## Unresolved questions / deferred work
- Unknown residue: code-plausible but uncalled gaps in banks 0/1/14/15 (L-004) and conservative-rule residue in data
  banks 8-13 (L-007). Script stream format (L-005) would give precise text/script boundaries.
- Fight-win contract, day clock, area index, natural bite trigger (L-001, L-002, L-003, L-008).
- RAM routines at $06E1/$06E4 upload source (L-006).
- Static assumptions to re-check if contradicted: JSR returns except listed helpers; heuristic jump-table extents;
  2 rejected overlapping decodes (investigation.md).

## Leads
Consumed: none (fresh). Produced: L-001 … L-008 in analysis/leads.json.

## Toolkit findings (analysis/toolkit-issues.md)
- Confirmed bug: TK-001 NES emulator.input ignores documented button/pressed (use `buttons` array).
- Missing features: TK-002 MMC3 static discovery stops at every JSR; TK-003 tracing ~10x slowdown even with
  capacity 0; TK-004 trace.clear also clears coverage aggregates / no Fetch events on NES. No blockers.

## Artifacts
- Tools: analysis/tools/ (rpcd.py/rpc.py relay, drive.py, routes.py, cover_replay.py, covdump.py, readtrace.py,
  m6502.py, static_rd.py, build_code_regions.py, classify_gaps.py, apply_map.py, gaps.py, montage.py).
- Evidence: analysis/evidence/round-01/. Disposable scratch: scratch/ (snapshots snap_*.bin are same-build, local).
- Gameplay notes: analysis/gameplay.md. Ledger: analysis/investigation.md.
