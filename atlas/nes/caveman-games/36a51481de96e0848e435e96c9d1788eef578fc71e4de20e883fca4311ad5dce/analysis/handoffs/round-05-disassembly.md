# Round 5 — disassembly handoff

- Input pass-004. Output checkpoint **pass-005** (maps/pass-005.json == maps/current.json; generated/pass-005;
  analysis/pass-005-verification.json ok, byte_identical true; store analysis/state.json). input/game.nes unchanged.
- Code identical to pass-004 (1614 regions, 41657 B). Regions 4306 -> 4280. Edits: tools/r05_apply.py;
  list: analysis/round-5/changes.json; pre-edit store analysis/round-5/state.pass-004.json.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Leads
- L-021 verified (all parts incl. optional bank 2 split). L-022 verified. Rejected: none. New leads: none.

## Changed ranges (for commentary)
| Bank | CPU | File | Change |
|---|---|---|---|
| 7 | $D881-$D922 | 0x1D891-0x1D932 | 13 pieces -> 1 Data (MenuMetatiles records 41-49 + 9 zeros); Text $D899-$D8A3 gone |
| 0 | $A8CF-$A9AB | 0x28DF-0x29BB | 16 pieces -> 1 Data ($FF-separated tile-number lists); Text $A92E-$A94A gone |
| 2 | $AC4F-$ACDF / $ACE0-$AFFF | 0xAC5F-0xACEF / 0xACF0-0xB00F | boundary $ACDF -> $ACE0 (stream 17 end byte / fill) |
| 3 | $944B-$954A | 0xD45B-0xD55A | Data label DR_CourseMap (256 B, Static 0.9) |
| 3 | $954B-$9560 | 0xD55B-0xD570 | generic Data tail (22 B, 0.7) |

Commentary to do: replace DR_CourseMap "end is not proven" with the window proof; note for $954B-$9560; refresh bank 0/7
notes (no longer Text). Totals: Code 41657, Data 72235, Text 4822, Padding 12374, Image 131072.

## Open (no classification effect)
- L-004 (reachability), L-005 remainder (consumerless block roles), S16. Toolkit: no new issues found. No processes running.
