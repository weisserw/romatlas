# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


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

## Round001 redirect r001-a active
Stage transition contract established and stage7 artificially visited (river/stone shore and enemy). See analysis/redirect/r001-a/frontier.md, stage07.ppm.png and exact RPC. Static 59-stage parallel tables, 128-type initializer/update domains and spawn pointer loader identified; inspecting connected/independent continuations before batch handoff. No map/source change.

Round001 redirect outcome: coherent95,731-byte proposal batch ready for independent disassembly; accepted map unchanged. Stronger stage/domain/record endpoints, complete routine contracts and generic non-code fallbacks resolved previous barriers. Broad later areas/bosses/credits visited artificially with durable routes. See handoff/results/structural-contracts/residual-review;8104 hypothetical residual is pending localized review, not terminal exhaustion.

## Round 002 disassembly completed — incomplete ROM

ROM classification: 100.00% (393220 / 393232 bytes classified; 12 bytes unknown).

The rounded100.00% does not establish completion:12 physical bytes remain unknown (ten isolated RTS bytes plus one two-byte C9 F9 interval). Code73419bytes; all non-code319801bytes including131072CHR. Gain103823bytes over pass-001b:58349Code+45474Data. Exact bank counts and final unknown intervals: analysis/round-002/per-bank-counts.json, unknown-final.json.

R001-R05 and R001-R06 verified and annotated after independent configured release decoder review of all1283 proposed Code groups, exact bank resolution/lengths, Observed-start provenance, nonoverlap, all290 selected-bank wrapper templates,48 raw sentinel spawn streams and stage record constraints. Accepted redirect proposal95731bytes (56343Code+39388Data). Data role metadata was converted to canonical comment fields; every annotation uses canonical schema and Static/Observed provenance. No unsupported pair2$8C2D callee or superseded audio bank premise admitted.

Followed connected/ordinary residual gains beyond proposal ceilings:181 independently bounded generic-Data intervals6051bytes;2006 further Code bytes under explicit slot/stack/mapper/queue/palette/BCD/demo/audio/coordinate contracts;35 connected Data bytes. Full neighborhoods, decoder output, exact plans and positive structural rationale remain under analysis/round-002/. Final control-target review reports zero targets into Data/Unknown under literal selector contexts. Two uncertain words were independently resolved as pushed predecremented return addresses by explicit high/low LDA/PHA consumers; mixed routine/data regions were split. Remaining single RTS bytes were not classified from opcode alone.

R001-R07 verified existing-Data layout semantic support:59 pointer endpoint/stride5/15/16 and metatile constraints corroborated; no new bytes and no asserted full index-list extents. Next commentary should incorporate these roles. Structural classification is separate from reachability. Recorded artificial stage7/13/23/36/48/54 and credits actual-fetch evidence reused; no unchanged gameplay route or arbitrary patch was repeated. Natural selection of every actor/player/audio entry remains unresolved.

State analysis/state.json and map maps/current.json validated and saved. Fresh checkpoint maps/pass-002a.json, generated/pass-002a and analysis/pass-002a-verification.json:ok=true,byte_identical=true; mapSHA256 dd872fd78ff1cb46968d12c08ec6b7c112bedc2516e62d6d5b0a98dbfa82fbdd. Original ROM hash unchanged. Prior checkpoints/source/commentary and all historical evidence preserved; existing commentary has not been prepared/replaced. Before next commentary preparation, preserve its entire prior copy as instructed.

Toolkit: no new issues found; two existing confirmed documentation/API mismatches TI-001/TI-002 remain nonblocking (analysis/toolkit-issues.md). No toolkit edit/rebuild, cleanup, archive or publishing. All phase-owned servers stopped.

Frontier: full newly classified-source commentary, then bounded redirect review of exact12-byte residuals (unknown-final.json), especially whether finite null-handler tables or synthesized return addresses establish individual RTS contracts. Broad routes need no unchanged replay. This is an ordinary productive phase handoff, not terminal exhaustion; full current-source readability/editorial/archive gates remain pending.

|Space/bank|Code|Non-code|Unknown|
|---|---:|---:|---:|
|Header 0|0|16|0|
|PRG 0|6432|1759|1|
|PRG 1|7255|937|0|
|PRG 2|2558|5634|0|
|PRG 3|2217|5975|0|
|PRG 4|630|7562|0|
|PRG 5|3495|4697|0|
|PRG 6|5774|2417|1|
|PRG 7|5726|2466|0|
|PRG 8|6858|1333|1|
|PRG 9|5693|2496|3|
|PRG 10|3637|4554|1|
|PRG 11|376|7816|0|
|PRG 12|2198|5994|0|
|PRG 13|0|8192|0|
|PRG 14|0|8192|0|
|PRG 15|0|8192|0|
|PRG 16|6600|1591|1|
|PRG 17|0|8192|0|
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
|PRG 30|6678|1512|2|
|PRG 31|7292|898|2|
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

## Round002 redirect r002-a

Corrected type4A rejected linear path using actual caller flags; four-byte dual-use branch/self-loop and exact15-row vector layout ready for independent review. Reflected type48 child selector mechanics confirmed in bounded artificial calls. Twelve residual bytes remain materially ambiguous after direct/stack/context/neighbor review. Primary rejection review, proposal, ledgers and handoff: analysis/redirect/r002-a/rejection-review.md; analysis/round-002-redirect-handoff.md. No pending productive experiment or unchanged gameplay replay.


## Round003 disassembly pass-003a

ROM classification: 100.00% (393220 / 393232 bytes classified; 12 bytes unknown).
INCOMPLETE:12unknown bytes in11intervals. R002-L02/L03 verified;4Data bytes reclassified dual-useCode,0unknown gain. Exact15-pointer/15x32 signedXY layout independently established. R002-L01 sound residual rejection reused; R002-L04 artificial mechanics unchanged. Review: analysis/round-003/review.md. Broad gameplay/credits evidence reused without unchanged replay.

Saved/validated analysis/state.json and maps/current.json; checkpoint maps/pass-003a.json, generated/pass-003a, analysis/pass-003a-verification.json:ok=true,byte_identical=true. Map SHA256 f24d8835160d5de6fef4dca61b263064cc9d040d046142de5180b264f8aba567; ROM SHA256 unchanged. Historical evidence and entire commentary/source preserved. Existing TI-001/TI-002 nonblocking; no new toolkit issues found (analysis/toolkit-issues.md). No toolkit edits, cleanup, archive or publishing.

|Space/bank|Code|Non-code|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header 0|0|16|16|0|
|PRG 0|6432|1759|8191|1|
|PRG 1|7255|937|8192|0|
|PRG 2|2562|5630|8192|0|
|PRG 3|2217|5975|8192|0|
|PRG 4|630|7562|8192|0|
|PRG 5|3495|4697|8192|0|
|PRG 6|5774|2417|8191|1|
|PRG 7|5726|2466|8192|0|
|PRG 8|6858|1333|8191|1|
|PRG 9|5693|2496|8189|3|
|PRG 10|3637|4554|8191|1|
|PRG 11|376|7816|8192|0|
|PRG 12|2198|5994|8192|0|
|PRG 13|0|8192|8192|0|
|PRG 14|0|8192|8192|0|
|PRG 15|0|8192|8192|0|
|PRG 16|6600|1591|8191|1|
|PRG 17|0|8192|8192|0|
|PRG 18|0|8192|8192|0|
|PRG 19|0|8192|8192|0|
|PRG 20|0|8192|8192|0|
|PRG 21|0|8192|8192|0|
|PRG 22|0|8192|8192|0|
|PRG 23|0|8192|8192|0|
|PRG 24|0|8192|8192|0|
|PRG 25|0|8192|8192|0|
|PRG 26|0|8192|8192|0|
|PRG 27|0|8192|8192|0|
|PRG 28|0|8192|8192|0|
|PRG 29|0|8192|8192|0|
|PRG 30|6678|1512|8190|2|
|PRG 31|7292|898|8190|2|
|CHR 0|0|8192|8192|0|
|CHR 1|0|8192|8192|0|
|CHR 2|0|8192|8192|0|
|CHR 3|0|8192|8192|0|
|CHR 4|0|8192|8192|0|
|CHR 5|0|8192|8192|0|
|CHR 6|0|8192|8192|0|
|CHR 7|0|8192|8192|0|
|CHR 8|0|8192|8192|0|
|CHR 9|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

## Round003 redirect resolved — r003-a
No new actionable lead after independently assessing accepted dual-use branch closure and bounded vector layout against primary residual evidence. Exact12Unknown/11intervals unchanged;393220classified. Current readability2063regions/2665units pending0; prior fresh byte-identical pass-003a preserved. Sound r002-a review reused with explicit changed-premise/local alternatives assessment at analysis/redirect/r003-a/rejection-review.md. No runtime replay, map/source/state/toolkit edits or cleanup. Leads pass appended; progression scenarios retained and reviewed. Terminal INCOMPLETE handoff analysis/round-003-redirect-handoff.md requires coordinator inspection; editorial audit pending. No new toolkit issues found (existing TI-001/TI-002 retained).
