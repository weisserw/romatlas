
## Round 1 — disassembly final handoff

ROM classification: 50.20% (65807 / 131088 bytes classified; 65281 bytes unknown).

Incomplete: 65281 bytes remain structurally unknown. Classified Code 10873,
Image 52000, Data 2934; 1359 regions all carry evidence.
Header16 bytes classified separately. This is one completed easy-gain disassembly
pass, not terminal investigation exhaustion.

| Physical PRG8K bank | Code | Image | Data | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 0 | 0 |
| 1 | 0 | 8176 | 0 | 16 |
| 2 | 0 | 8192 | 0 | 0 |
| 3 | 0 | 8000 | 0 | 192 |
| 4 | 0 | 0 | 0 | 8192 |
| 5 | 0 | 1616 | 35 | 6541 |
| 6 | 0 | 8192 | 0 | 0 |
| 7 | 0 | 8192 | 0 | 0 |
| 8 | 1343 | 0 | 0 | 6849 |
| 9 | 0 | 1440 | 181 | 6571 |
| 10 | 601 | 0 | 1968 | 5623 |
| 11 | 644 | 0 | 157 | 7391 |
| 12 | 9 | 0 | 0 | 8183 |
| 13 | 589 | 0 | 102 | 7501 |
| 14 | 4622 | 0 | 52 | 3518 |
| 15 | 3065 | 0 | 423 | 4704 |

Verified baseline and final checkpoints are byte-identical. Final map.validate and
portable map validation passed. Final checkpoint: maps/round1-disassembly-final.json,
generated/round1-disassembly-final/game.asm and
analysis/round1-disassembly-final-verification.json. Input SHA256 unchanged.
Internal state analysis/state.json; current map maps/current.json. Counts derive
from unique physical classified bytes, excluding unknown export fallback.
Execution separately: 5556 unique opcode starts,6482 contextual roots,7100 contextual
edges, aggregate_dropped0. Detail capacity0 deliberately retains no events;
11562194 dropped detail events are not aggregate loss. Runtime trace summary and
context captures preserve physical offset, CPU alias, effective bank and artificial
status. Natural reachability is claimed only for title/start/early play.

Broad progression used supported scene/state RAM contracts with replayable snapshots
and action inputs through later areas, final boss, rescue boat and terminal ending.
Megagon form was successfully displayed and Up+B exercised without claiming a laser
at zero score. Exact recipes and deferred same-zone scenes: analysis/scenarios.json.

Static discovery and upload consumer/descriptor evidence yielded connected code and
52KB of planar image sources; generic uniform runs were annotated only after
checking neighboring patterns and conflicting code. Seven unselected descriptors
were accepted on bounded pool layout, consumer semantics and tile-source consistency,
without inventing natural selection. Two conflicting static decodes were skipped.
No original caller/layout or dead-code claims were inferred.

Ordinary waits/input variation stopped after representative systems and ending
were visited; further scene permutations have low first-pass value. Residual bank4
records, state/boss pointer tables and asset gaps need connected consumer/boundary
inspection in redirect, not additional long gameplay. Concrete leads are ready in
analysis/leads.json. Readability/editorial review remains commentary work; no full
readability completion claim.

Toolkit findings: one confirmed documentation/API mismatch NES-001, zero suspected
bugs, zero missing-feature entries; no blockers. Workaround uses buttons arrays.
See analysis/toolkit-issues.md; no toolkit changes made. Disk checked at handoff:
8.9GiB available, scratch196KiB, phase evidence about133MiB before final artifacts.
No evidence/archive cleanup or publication performed.

Next phase: full current-source commentary, then redirect on the three ready leads.
