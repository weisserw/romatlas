# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Pass 001 — disassembly handoff

Baseline verified once; static discovery from physically validated vectors, runtime title/New York and artificial bonus/Tokyo scenarios. Accepted 9,181 static Code bytes; 23,073 Data; 32,768 CHR Image. Exact evidence and generic fallback inspections: disassembly/pass-001/reviewed-static.json, dispatch-accepted.json, uniform-runs.json, generic-data-review.json. Code-vs-data classified independently of natural reachability. Runtime accepted aggregates show1697 natural context roots and1796 late artificial roots, no aggregate drops and no residual fetch. Zero detail capacity intentionally drops all event detail; aggregates retained. This is not whole-game execution coverage.

Correction during review: first raw traversal decoded a constant-BEQ fallthrough into OAM template2637..2657, producing overlapping byte2674. LDX#0 at2633 establishes Z before BEQ2635, so that fallthrough is infeasible. Indexed copy at2598 explicitly consumes20bytes. Inserted Data barrier, repeated discovery: zero overlap. Return-address dispatcher physical620..641 consumes inline table21406..21418; table targets and connected routines accepted in fixed CNROM context [0,1,2,3]. Physical instruction addresses, not CPU aliases, govern counting.

Ready residual questions D001..D004 in leads.json cover530bytes total: [379,421),[1976,2325),[7648,7662),[18032,18157). These are not rejected or terminally exhausted. Targeted redirect should split code/data and inspect mapper effects, contracts and consumers. Raw decode alone was not promoted. Already-classified large data blocks have generic structural roles; unknown compression/stream schema is not unknown classification.

Progression: natural New York controls; artificial $A0=4/$63=$80 gave bonus; $5A=FF and ready timer$17=1 then gave Tokyo and controls. All CPU banks [0,1,2,3]. Other backgrounds and final stage wrap remain deferred with practical recipes, not blockers or grounds for terminal handoff. See scenarios.json/gameplay.md. Representative late state adds usable understanding despite0newbytes.

Final checkpoint maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json validates and rebuilds byte-identically. counts.json records per-bank counts. ROM classification: 99.19% (65022 / 65552 bytes classified; 530 bytes unknown).

Toolkit NES-001 records1confirmed documentation/API mismatch,0suspected bugs,0missing features; workaround available, no blocker. Do not modify toolkit. Scripts game.py/play.py earlier coverage paging loop incorrectly expected null next_offset; stopped own run, corrected subsequent scripts to next_offset>=total. No toolkit defect asserted for this worker error. No active server remains at handoff.

## Pass 002 final — structural pass finished, editorial completion pending

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Accepted D001–D004: all530 previously unknown bytes (429Code/101Data). Accepted C001/C002:13Code→Data correction. No rejected proposals. Independent reasoning and primary checks: analysis/disassembly/pass-002/review.md, independent-decodes.json and annotations.json; prior redirect evidence preserved. Total Code9597, Data23187, Image32768; every byte structurally classified.

| Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 0 |
| PRG0 | 5803 | 2389 | 0 | 0 |
| PRG1 | 1386 | 6806 | 0 | 0 |
| PRG2 | 2383 | 5809 | 0 | 0 |
| PRG3 | 25 | 8167 | 0 | 0 |
| CHR0 | 0 | 0 | 8192 | 0 |
| CHR1 | 0 | 0 | 8192 | 0 |
| CHR2 | 0 | 0 | 8192 | 0 |
| CHR3 | 0 | 0 | 8192 | 0 |


Fresh checkpoint maps/pass-002-final.json / generated/pass-002-final, analysis/pass-002-final-verification.json: ok=true, byte_identical=true. Current/frozen map SHA256 58d2d1fc77ed1a0c5ee0b835c8df8a5a734ed773152332e814ff6b5fbb3a94d8. Internal state saved and map.validate passed. Input identity verified unchanged. No ROM/toolkit modification. All servers closed.

Execution coverage unchanged: no new fetch claim or replay; representative natural/earlier artificial gameplay plus redirect's later city/stage-loop recipes remain in scenarios.json and redirect/round-001-a. Static new routines have unresolved external/natural reachability. No additional gameplay question justified rerunning them. No unsupported downstream continuation remains in this supported batch.

Readability/editorial status: prior round1 commentary tree and handoff preserved untouched; map changes require next complete current-source commentary (including title-frame shared counter correction). Final editorial audit pending; this structural100% is not a terminal completion claim. Next coordinator phases commentary then redirect/audit under their own prompts.

Toolkit report analysis/toolkit-issues.md: existing1confirmed documentation/API mismatch NES-001,0suspected bugs,0missing features,0blockers. No new toolkit issues found. Workaround honored. Disk2.6GiB free, scratch36KiB.
