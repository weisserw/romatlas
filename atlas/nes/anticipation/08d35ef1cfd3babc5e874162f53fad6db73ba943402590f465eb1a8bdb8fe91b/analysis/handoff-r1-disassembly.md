# Round 1 disassembly handoff

Incomplete ROM; single disassembly phase complete.

ROM classification: 71.92% (47143 / 65552 bytes classified; 18409 bytes unknown).

| Physical section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 8 KiB bank 0 | 5158 | 1401 | 1633 |
| PRG 8 KiB bank 1 | 3071 | 922 | 4199 |
| PRG 8 KiB bank 2 | 3085 | 428 | 4679 |
| PRG 8 KiB bank 3 | 252 | 42 | 7898 |
| CHR 32 KiB | 0 | 32768 | 0 |

Code 11566, Data 2809, Image 32768; 5469 regions with evidence. Actual accepted fetch roots5253, contextual edges5818, aggregate drops0; these are execution metrics. Observed code bytes7643; remaining code Static.

Final internal state: analysis/state.json; portable map: maps/current.json. Checkpoint [r1-disassembly-final verification](r1-disassembly-final-verification.json): valid, byte-identical generated source. Exact checkpoint map maps/r1-disassembly-final.json; source generated/r1-disassembly-final/game.asm. Baseline/r1-intermediate/r1-disassembly retained. Input hash remains workspace identity.

[Progress](progress.md), [investigation](investigation.md), [gameplay](gameplay.md), [scenarios](scenarios.json), [lead ledger](leads.json), [exact unknown complement](r1/handoff-unknown.json), [metrics](r1/final-metrics.json).

Confirmed startup and setup; level1 board/drawing; artificial stage3 and4 distinct layouts/puzzle selection; WINNER terminal scene via inspected terminal-check block. Stage2 not separately visited; later drawing, feature/drop-out square activation, and any additional credits remain deferred with explicit reasons in checklist. Exact restore/input/patch budgets in r1/session-log.jsonl, stage4.py, snapshots. Effective PRG context consistently[0,1,2,3]. Snapshots are local ROM-bearing internal evidence.

Static code graph + bounded direct-JSR closure reviewed; only residual KIL0x2aa7 rejected as impossible DEY/BNE-loop fallthrough. Do not extend this rejection to neighboring supported bytes. Generic Data0x1a97–0x23aa accepted after full pattern and consumer review; no individual table schema claimed. Physical0x5c63–0x5e0f mixed screen/tile data; uniform FF0x7f6c–0x7f90 generic data. Header and CHR have section-backed bounds.

Next phase commentary must complete readability across currently classified source. Redirect should investigate R1-001..R1-004: large command streams and nearby consumers; state dispatcher install sites; residual small code/data boundaries; practical feature transitions. These require role/boundary/entry research, rather than more unchanged early gameplay. No whole-ROM exhaustion or editorial-completion claim.

No toolkit issues found; [toolkit report](toolkit-issues.md). No toolkit modifications.
