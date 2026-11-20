# Round 1 disassembly handoff

Status: pass completed (easy gains taken); ROM incomplete.

## Scope
Fresh workspace: baseline, gameplay traces, static bank-aware descent, generic data/CHR/header classification. Game: Bigfoot (NES, MMC1, 8 x 16 KiB PRG, fixed $C000 = 8 KiB banks 14/15).

## Checkpoint / verification
- pass-001: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json — map validate ok, byte_identical true.
- Internal store: analysis/state.json; export maps/current.json (same as pass-001).
- ROM classification: 94.39% (247463 / 262160 bytes classified; 14697 bytes unknown).

## Findings
- Reset $EC8F (inits MMC1, selects 16K bank 0, jmp $8000); NMI $F256 = jmp ($0478) with 15-entry handler table $F238 set by SetNmiHandler $F220; IRQ $F259 = RTI.
- SelectPrgBank16K $EF02 (A = 16K bank; previous in $040B, current $040A), MMC1 serial writers $EF11/$EF25/$EF39/$EF4D.
- Far calls: FarCallIndexed $F81A (A = index into FarCallTable $F86A, 14 x {lo,hi,bank,0}, bank stack $040C,$0414) and FarCallAddress $F855 (A=bank, X/Y=addr). ReadJoypads $F7CF ($0426/$0427).
- Bank roles: PRG02 tile-row/nametable data; PRG03 all $FF; PRG08/09-low nibble sample-like waveform data; PRG10 tables used by PRG11; bank 9 holds tour/results/ending code and ASCII text; stop index $041D.
- All observed instructions from the later event/ending scenarios were already predicted by the static descent.

## Unresolved questions / deferred work
- Leads L-001..L-009 in analysis/leads.json (unreferenced routine-like islands, bank 9 text, bank 10/12 tables, indirect sites $8C4C).
- Gameplay not covered: 2-player, later cross-country races, out-of-cash game over (L-007); natural ending route (L-008).
- Data pieces are generic; semantics (tables, maps, samples) for commentary/redirect.

## Artifacts
- analysis/progress.md, analysis/investigation.md, analysis/leads.json, analysis/scenarios.json, analysis/gameplay.md, analysis/toolkit-issues.md
- Snapshots (same-build): traces/snapshots/snap-*.json (title, race1, shop1, crush1, event-stage1/2/3/8, ending-results)
