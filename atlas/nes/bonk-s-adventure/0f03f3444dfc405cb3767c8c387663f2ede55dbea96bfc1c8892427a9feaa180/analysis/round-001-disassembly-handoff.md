# Round 001 disassembly handoff


## Round 001 disassembly completed — incomplete ROM

ROM classification: 73.59% (289397 / 393232 bytes classified; 103835 bytes unknown).

Code 15070 bytes; CHR Image 131072; other Data 143255. Every current region carries evidence. Structural coverage is separate from execution: startup trace had 3012 physical fetched instruction starts and 3641 contextual roots; startup+stage1 window produced 6718 contextual roots. Runtime was natural; no RAM/register patches were used. Trace capacity 0 deliberately retained aggregates with dropped detail; aggregate_dropped=0. No full-game reachability claim.

Final verified checkpoint: maps/pass-001b.json; generated/pass-001b; analysis/pass-001b-verification.json (ok=true, byte_identical=true). Prior baseline/pass-001 checkpoints preserved. Internal state analysis/state.json and portable map maps/current.json validated. ROM SHA256 remains workspace identity.

Evidence: analysis/round-001/rpc/ retains exact requests/results; contexts-current.json and discovery records support bank-aware code closure. uniform-data-inspection.json bounds 91450 uniform non-code bytes; pattern-data-inspection.txt records full residual bank22–29 non-code matrix/record inspection (51783 further bytes). These are generic data where schema/consumer remains unknown, not inferred specific layouts. Header and vectors derive validated hardware format; CHR backing is PPU tile image data.

Gameplay: natural title and stage1-1 visited, moving/jumping scrolls jungle and encounters flying enemy (stage1-play.ppm.png). Stage1-play.snapshot preserves progress. Startup request 300 frames timed out after 72 frames; state inspected and then untraced progression completed. Later areas, bosses and ending deferred to redirect because two practical built-in selector attempts left the title unchanged and a supported stage-transition RAM contract is not yet known. This is an ordinary easy-gain phase handoff, not a terminal exhaustion claim.

Toolkit: two confirmed documentation/API mismatch findings TI-001/TI-002, no blocker; analysis/toolkit-issues.md. No toolkit edits. Disk remains bounded; no cleanup performed.

Next: commentary of current source; redirect identify title selector condition, stage RAM/transition contract, and consumers/bounds of remaining PRG ranges. Review static closure at indirect dispatch/data barriers; use supported artificial stage transitions once contract is known.

|Space/bank|Code|Non-code|Unknown|
|---|---:|---:|---:|
|Header 0|0|16|0|
|PRG 0|1250|0|6942|
|PRG 1|260|47|7885|
|PRG 2|1085|32|7075|
|PRG 3|537|3984|3671|
|PRG 4|618|32|7542|
|PRG 5|364|111|7717|
|PRG 6|0|32|8160|
|PRG 7|0|913|7279|
|PRG 8|223|32|7937|
|PRG 9|0|1531|6661|
|PRG 10|151|328|7713|
|PRG 11|71|7009|1112|
|PRG 12|159|3369|4664|
|PRG 13|0|8192|0|
|PRG 14|0|8192|0|
|PRG 15|0|8192|0|
|PRG 16|2885|698|4609|
|PRG 17|0|1764|6428|
|PRG 18|0|8192|0|
|PRG 19|0|8192|0|
|PRG 20|0|8192|0|
|PRG 21|0|8192|0|
|PRG 22|0|8192|0|
|PRG 23|0|8192|0|
|PRG 24|0|8192|0|
|PRG 25|0|8192|0|
|PRG 26|0|8192|0|
|PRG 27|0|8192|0|
|PRG 28|0|8192|0|
|PRG 29|0|8192|0|
|PRG 30|2717|32|5443|
|PRG 31|4750|445|2997|
|CHR 0|0|8192|0|
|CHR 1|0|8192|0|
|CHR 2|0|8192|0|
|CHR 3|0|8192|0|
|CHR 4|0|8192|0|
|CHR 5|0|8192|0|
|CHR 6|0|8192|0|
|CHR 7|0|8192|0|
|CHR 8|0|8192|0|
|CHR 9|0|8192|0|
|CHR 10|0|8192|0|
|CHR 11|0|8192|0|
|CHR 12|0|8192|0|
|CHR 13|0|8192|0|
|CHR 14|0|8192|0|
|CHR 15|0|8192|0|

Operational closeout: persistent server stopped after state/map export and checkpoint. Resume with a new configured server, map.load analysis/state.json, emulator.start, then import a selected durable snapshot and reassert buttons. Exact request/response history remains local. Evidence directory 172 MiB, disk free 3.6 GiB; no cleanup performed. Current per-bank accounting also available as analysis/round-001/per-bank-counts-final.json. Lead outcomes and progression checklist: analysis/leads.json and analysis/scenarios.json.
