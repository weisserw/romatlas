# Round 1 - disassembly handoff

Status: complete-pass (easy gains taken; ROM classification incomplete).

## Scope
Fresh workspace: inspect, baseline (verified), static discovery from vectors with UxROM bank context, traced
gameplay scenarios (boot/attract, menus, tournament setup + auto break, in-game menus, Challenge/Party setup),
data-read scans of non-code banks, code/data/padding annotation, save/export/checkpoint.

## Output checkpoint
- `pass-001-final`: maps/pass-001-final.json, generated/pass-001-final,
  analysis/pass-001-final-verification.json -> ok: true, byte_identical: true.
  (Intermediate `pass-001` also verified byte_identical; superseded by pass-001-final.)
- Internal state: analysis/state.json (identical content to maps/current.json).

ROM classification: 98.30% (128863 / 131088 bytes classified; 2225 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3492 | 12755 | 134 | 3 |
| PRG 3 | 13570 | 769 | 0 | 2045 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13266 | 3045 | 41 | 32 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11479 | 4671 | 89 | 145 |

Code: 21,331 bytes Observed + 22,785 Static (998 of them exact gap-fill at confidence 0.7, reachability unknown).
Execution coverage (separate): 10,457 observed instruction starts.

## Scenarios exercised
See analysis/scenarios.json: S1 boot/attract, S2 mode menu, S3 tournament setup + AUTO break, S4 in-game
menus (main menu, game control, spin, jukebox, actions, special actions, overview), S5 shot attempts (no ball
motion achieved), S6 data-read scans, S7 Challenge + Party setup. Game: Championship Pool (Mindscape/Bitmasters
1993); research in analysis/gameplay.md.

## Findings
- Bank switching: shadow $1E; SetPrgBankFromA $E4B2 + SelectBank stubs $E4CE-$E538; bus-conflict table $FFF0.
  Fixed-bank trampolines call banked routines; FarCallViaZp04 $E559 uses `jmp ($0004)` with RAM-provided targets.
- Static walker (analysis/tools/walk.py) agrees with all observed instruction starts (0 bank mismatches).
- Banks 0, 1, 4 and bank 6 $8000-$B152 are data (graphics/tiles, nametables, compressed-looking blocks); bank 2
  $8000-$B1D2 is ASCII text/script data including ending text. Code lives in banks 2 (tail), 3, 5, 6 (tail), 7.
- Tooling scripts: analysis/tools/ (rpcd.py bridge, nes.py, walk.py, gapfill.py, annotate_*.py, counts.py,
  readscan.py, tiles.py, codelike.py, gaps.py).

## Leads produced
L-001 unreferenced trampolines; L-002 bank 3 $832B block; L-003 bank 3 $95F6 block; L-004 bank 3 $88A8/$8A51
mixed; L-005 $E3A3 routine; L-006 small dead fragments; L-007 shot input / tournament RAM progression.
(analysis/leads.json). No leads consumed (none existed).

## Unresolved questions
- Callers of the bank-3 code-like blocks (likely FarCallViaZp04 targets from RAM $0435-$0437 hooks or modes not
  visited). Exact code/data boundaries in $88A8-$8B13 and $95F6-$9880.
- Shot trigger: manual says A (phantom ball) then A (shoot); not reproduced locally.
- Exact formats/consumers of most data blocks (generic data claimed only).

## Deferred work
- Freestyle, 9-ball, Party play, later tournament rounds, World Championship, ending (L-007).
- Provenance upgrade of static code observed later; commentary/labels (commentary phase).

## Toolkit issues (analysis/toolkit-issues.md)
- Confirmed bug: TK-003 trace Read events include opcode/operand fetches (contradicts docs).
- Suspected bug: TK-002 ~12x slowdown with tracing at zero capacity.
- Missing feature: TK-001 analysis.discover stops at every JSR on UxROM (fixed-window calls included).
None blocking; workarounds recorded.
