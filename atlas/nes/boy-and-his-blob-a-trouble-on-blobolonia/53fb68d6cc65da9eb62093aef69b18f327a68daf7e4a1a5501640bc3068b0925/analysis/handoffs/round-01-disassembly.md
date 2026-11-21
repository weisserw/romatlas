# Round 1 — disassembly handoff (COMPLETE, result: incomplete ROM classification)

- Scope: fresh workspace; inspect + baseline (maps/baseline.json, verified), broad play with tracing,
  static recursive descent, jump tables, generic data classification. Game: David Crane's A Boy and
  His Blob: Trouble on Blobolonia (USA), MMC1, 128K PRG + 128K CHR.
- Output checkpoint: **pass-002** (maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json). Earlier same-pass checkpoint pass-001 (maps/pass-001.json,
  analysis/pass-001-verification.json) is superseded (pass-002 adds two referenced table blocks).
- Verification: `byte_identical: true`, ok: true, map_sha256 33ef2e56314a39bf84c5d8a196aaa37f2854cf7966ee17c0398e49bf0f296428.
- Internal store analysis/state.json == maps/current.json (map.validate ok; 1218 regions).

ROM classification: 98.48% (258166 / 262160 bytes classified; 3994 bytes unknown).

| Space | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| iNES header | 0 | 16 | 0 | 0 |
| PRG 16K bank 0 (8K 0,1) | 11732 | 4313 | 171 | 168 |
| PRG 16K bank 1 (8K 2,3) | 1411 | 12305 | 2104 | 564 |
| PRG 16K bank 2 (8K 4,5) | 1578 | 14055 | 54 | 697 |
| PRG 16K bank 3 (8K 6,7) | 1578 | 14304 | 54 | 448 |
| PRG 16K bank 4 (8K 8,9) | 1578 | 11228 | 3575 | 3 |
| PRG 16K bank 5 (8K 10,11) | 192 | 11765 | 3555 | 872 |
| PRG 16K bank 6 (8K 12,13) | 5115 | 5930 | 5065 | 274 |
| PRG 16K bank 7 fixed (8K 14,15) | 1962 | 8367 | 5087 | 968 |
| CHR ROM | 0 | 131072 | 0 | 0 |
| **Total** | **25146** | **213355** | **19665** | **3994** |

Execution coverage (separate metric): 9704 unique instruction addresses observed; 12860 code bytes lie
in Observed-provenance regions (some only in artificial warp scenarios; marked in evidence).

## Scenarios exercised (analysis/scenarios.json)
Title/attract/credits, game start, bean selection/throws (7+ flavours), pause, Earth street to subway
entrance (natural). Artificial: register-call of unreferenced warp routine $926F (Blobolonia jungle,
caverns rooms $43/$42/$1B), RAM room warps ($5A/$5B/$5C/$16 + $A4=1) over Earth rooms $00-$F8 and
Blobolonia rooms, death -> GAME OVER. Not reached: pharmacy/vitamins, treasure pickup, Emperor/ending
(S10/S11 deferred with reasons). Snapshots: traces/snapshots/{ingame0,mainloop_top,blob1}.snap.

## Findings
- Bank model: MMC1 16K mode (control $1E), bank 7 fixed at $C000; PRG switch only through $C05A
  (A = 16K bank, mirror $0203); CHR via $C000/$C01E/$C03C (control/CHR0/CHR1). Every bank has the
  reset stub $FFBF + vectors (NMI $C86F, RESET $FFBF, IRQ $C8D2).
- Bank 0 = main game logic (main loop $808A, joypad $87CB, warp $926F/table $928B, bean text $8D22).
- Bank 1 = APU sound driver $8000-$85DF + 11-entry far-call jmp vector ($C299/$C283); rest data.
- Banks 2/3/4 share an identical self-bank-switching PPU streaming engine $B900-$BFFF (RAM stub
  $04AA `JMP ($B9xx)` into unrolled `STA $2007` runs via table $B900).
- Bank 6 = object/AI and map lookup code ($8000-$88xx, $94xx-$A0xx) + data.
- Debug/unreferenced code: $926F warp, $CB82 hex coordinate printer, $BB80 APU helper.
- Data criterion used: no >=24-byte clean linear decode at any alignment + no control-flow target
  inside (0 conflicts). Details: analysis/investigation.md.

## Unresolved questions
- 157 unknown windows (analysis/pass-001/unknown-ranges.txt): clean-decoding stretches not reached
  by code — tables/text or unreferenced code.
- Bank 5 entry points / data format; ending trigger; pharmacy contract; jump-table extents.

## Leads produced
L-001 .. L-009 (analysis/leads.json). No leads consumed (none existed).

## Deferred work
Semantic labelling of generic data blocks; extent proofs for heuristic jump tables; ending and
pharmacy scenarios; review of data blocks adjacent to code for short routines (L-007).

## Toolkit findings (analysis/toolkit-issues.md)
- Confirmed bugs: TK-001 (NES emulator.input ignores documented button/pressed; use `buttons` array),
  TK-002 part (discover `budget` documented but `max_instructions` read).
- Missing feature: TK-002 (MMC1 static discovery stops at every JSR/indexed store).
- Suspected bug/perf: TK-003 (tracing ~10x slowdown regardless of kinds filter).
- No blockers (workarounds used).

## Artifacts
analysis/state.json, maps/current.json, maps/pass-002.json, analysis/pass-002-verification.json,
analysis/pass-001/ (scripts, plan, islands, unknown ranges), traces/coverage-pass001.json.gz,
analysis/scenarios.json, analysis/gameplay.md, analysis/leads.json, analysis/investigation.md.
