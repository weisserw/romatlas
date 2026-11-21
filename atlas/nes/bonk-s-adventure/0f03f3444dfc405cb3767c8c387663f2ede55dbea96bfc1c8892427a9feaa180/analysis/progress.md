# Disassembly progress

Workspace initialized. Baseline and classification have not run.

Round 001 disassembly active. Baseline verified byte-identical; no classification yet. Resumable driver/evidence: analysis/round-001/.

Accepted initial batch saved in analysis/state.json and maps/current.json: header 16, CHR Image 131072, vectors 6, Code 8183 bytes. ROM classification: 35.42% (139277 / 393232 bytes classified; 253955 bytes unknown). Current provisional accounting before checkpoint; source verification pending. Evidence: analysis/round-001/annotation-plan.json, contexts-current.json, discovery files; trace startup summary reports 3012 physical instruction starts. Title identifies Bonk's Adventure. Stage 1-1 reached naturally; stage1 movement/jump evidence underway. Sourced title-scroll level-select shortcut awaiting test; see analysis/gameplay.md. No terminal exhaustion claim.


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


Round 001 commentary active. Prepared independent commentary/source from verified pass-001b; preparation rebuild byte-identical. Frozen before/ and full-ROM 549-entry review scaffold preserved. Current frontier: full inventory pending, beginning with hardware/RAM references and fixed-bank services; no readability completion claimed.

Commentary coherent inventory batch applied: all 326 classified code fragments have mechanical-purpose descriptions, 993 entry/branch symbols and 219 shared RAM symbols; all 549 physical scaffold intervals have explicit source notes. Fixed-bank scheduler/PPU/bank services, sprite/hitbox routines, scrolling/collision buffers and audio interpreter reviewed. Frontier: validate byte identity, inspect symbolic resolution and factual comments, correct concrete defects, then refresh reading copy. Editorial acceptance remains pending.


Round 001 commentary completed across all current classified source. 549 physical intervals reviewed, 326 code fragments and 408 meaningful units; all CPU RAM operands symbolized, unexplained fields retain address identities. Final fresh rebuild byte-identical and reading-after refreshed. ROM/map/classification unchanged (73.59%, 103835 unknown). Final editorial/archive audit pending. Handoff: analysis/round-001-commentary-handoff.md; verification: analysis/round-001-commentary-verification.json; precise uncertainties and redirect questions: commentary/uncertainties.md. Current frontier: coordinator gate before redirect; no remaining commentary ranges at this checkpoint.

## Round 001 redirect active — r001-a
Input checkpoint pass-001b; 289397 classified, 103835 unknown. Current-source commentary complete. Research only; map/state/source unchanged. Frontier: establish stage transition RAM contract from fixed-bank scene initialization; then inline state dispatch and independent structural/table leads. Evidence under analysis/redirect/r001-a/. Productive research will resume in redirect if interrupted.

Redirect r001-a: successful artificial stage7 river and stage13 underwater3-1 visits; stage13 swimming/right+A exercised. Stage23 capture pending assessment; stage36/48 now executing bounded tests. Stage count59, type count128 and spawn origins established statically; active frontier in attempt/frontier.md. Map unchanged.

Redirect r001-a broad progression complete best-effort: stage13 underwater,23 vertical temple,36 boss,54 final crown arena exercised; credits STAFF->PROGRAM observed via supported fixed wrapperFD7E. stage48 short resume and credits fetch capture active. Structural batch includes actor/player dispatch graph (48683 initial unknown code-byte candidates), typed tables, spawn and IRQ; exact local malformed-callee rejection retained. See attempt/frontier.md; accepted map remains pass-001b.

Redirect r001-a latest structural checkpoint: reviewed graph56033 proposed unknown code bytes,74 inline tables/prefixes; all290 complete bank-call wrappers, pickup/actor-bytecode/boss/collision-response and audio dispatch continuations followed. All current-source code entries and12584 actual fetched roots compared: zero overlaps after correcting custom audio tick bank context. Spawn48streams482records exact2216bytecontainer; layout59stage constraints all pass. Active frontier is data inspection, exact proposals and lead/ledger packaging; no further arbitrary stage permutations needed. No map/state/source changes.

## Round001 redirect complete — coherent batch ready
Attempt analysis/redirect/r001-a/results.json; handoff analysis/round-001-redirect-handoff.md.95731 proposed previously unknown bytes:56343Code+39388Data; zero instruction/code-data conflicts. Accepted map unchanged at pass001b (289397classified/103835unknown); hypothetical residual8104 after full acceptance. Ready R001-R05/R06/R07; all290wrapper templates,74inline prefixes/tables, player/actor/audio/stage/spawn/layout and broad artificial gameplay/credits investigated. Current source verified unchanged; final editorial/archive pending. No new toolkit issues, two existing confirmed doc/API mismatches TI-001/TI-002 nonblocking. No publication/toolkit edit/archive/cleanup. Exact per-bank proposed/current counts, identities and residual dispositions in attempt.

## Round 002 disassembly active
Restoring pass-001b; reviewing redirect contracts independently before canonical annotations. Frontier: configured decoder boundary/context comparison, data consumer/extent evidence, then localized residual easy gains. Existing source/commentary and historical evidence preserved.

Round002 first accepted batch saved/validated: ROM classification: 99.48% (391179 / 393232 bytes classified; 2053 bytes unknown). Independently reviewed redirect95731bytes plus181 residual generic-Data slices6051bytes. Exact accepted-plan.json and validation-batch.json in analysis/round-002/. Frontier: independently decoded remaining routine contracts2006Code+35Data ready for final conflict/branch review, then save/export/fresh checkpoint. Existing commentary untouched.

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

Round002 commentary active. Entire prior commentary preserved at analysis/round-002/commentary-pass-001b-preserved before preparing the new baseline. Frontier: prepare pass002a baseline; carry physically unchanged source/commentary; review all newly classified code/data and affected dependencies; verify and refresh reading copy. No success handoff yet.

Round002 commentary context checkpoint: prior commentary preserved; new baseline verified; exact source/review carry, non-code/layout/RAM batch,488 wrapper fragments and41 fixed-bank entries saved. Fresh verification byte-identical; reading copy refreshed. INCOMPLETE:838 Code intervals/54636 bytes including25 affected old dependency comments remain. Resume same phase immediately using analysis/round-002/commentary-continuation.md and commentary-frontier.json; do not redirect or reprepare. No final commentary handoff/audit/publication.

Round002 commentary resumed through PRG pair0/1: remaining 643 Code intervals / 42247 bytes. All current Code in physical banks0 and1 now carries reviewed scene/player/control explanations; other banks remain pending. Since this continuation began,195 previously pending intervals /12389 bytes were reviewed, including concrete old dependency repairs. New findings include health/spare-life consumers, coherent snapshot fields, movement-script array aliases, terrain probe result correction, linked-player/actor coordinate behavior and scene collision-patch format. Fresh byte-identical verification passes. Exact frontier: analysis/round-002/commentary-frontier.json; phase remains active, incomplete and not ready for handoff/redirect.

Pair2/3 readability batch complete:51 additional intervals /3221 bytes, plus31 compact independent routine notes/units and14 pickup dispatch-entry identities. Health/material damage, contact support/height snapshots, actor hit processing, vector integration and pickup awards now have source-level explanations; six data containers have consumer-specific formats. All78 player-state targets and14 pickup dispatch words were checked by exact CPU/physical entry identity. Current pending frontier 592 Code intervals / 39026 bytes; first pending 33135. Fresh verification ok=true, byte_identical=true. Commentary remains incomplete.

Pair4/5 readability batch complete:56 additional intervals /3169 bytes,21 independent helper notes/units,11 exact alternate-scene actor dispatch entries and10 refined data containers. Health HUD and decimal rendering, slope/grid alignment, moving support, feedback generator, material6/special overlays and alternate-scene awards now have source explanations. All pending current Code in physical banks0..5 is reviewed. Current exact remaining scope:536 intervals /35857 bytes, starting physical49200 in banks6/7. Fresh byte-identical verify passed and reading-after.asm refreshed. This remains incomplete commentary; final full-scope verify/handoff and terminal editorial audit have not run.

Sequential worker context rollover: all source edits and review credit saved; no running processes. Resume the same commentary phase at physical49200 using analysis/round-002/commentary-continuation.md and exact536/35857 frontier. This checkpoint is not phase completion or a redirect handoff.

Round002 commentary active continuation: all pending pair6/7 main regions now reviewed (126 intervals /11500 bytes), with fresh byte-identical verification. Exact frontier410 intervals /24357 bytes starting65584 in banks8/9. Completing internal helper units and exact dispatch identities for pair6/7 next; same phase remains incomplete and no redirect/handoff is authorized by this checkpoint.


Sequential continuation checkpoint:159 additional intervals /12804 bytes reviewed, including all pair6/7 and the first33 pair8/9 intervals. Detailed helper inventory adds101+19 units;119 encounter state words plus20 initial bank8 words were checked against instruction starts (one explicit zero word). Contextual RAM/data symbols and honest null-selector uncertainty are in source. Fresh byte-identical verification and reading refresh pass. Exact remaining377 intervals /23053 bytes start67524;11 dependency defects remain. All writes/processes stopped for sequential context rollover. Resume SAME active commentary phase immediately; this is incomplete work, not a successful handoff or redirect.

Round002 commentary resumed active (same phase), latest verified frontier340Code intervals/18766bytes, first72531. This sequential worker has retired37intervals/4287bytes from the previous377/23053 checkpoint. Physical banks0..7 remain complete; bank8/9 continuation covers types14,21,1E,1D,2B,4A,56,1A/1B,4C,3D,3,1C,2C,15,3B,16,4D/4E and type55's no-op. Type15 state6, type1A/1B state4 and type14 states3..5 are actual zero words. Source explains child-slot/timer reuse, alternating scripts, companion acquisition, motion selectors, probes and type4E→4D transformation. Added118helper units/90exact words this worker. Fresh byte-identical verification passed and reading copy refreshed. Still active/incomplete; no phase handoff, classification edits, toolkit edits, cleanup or publishing.

Round002 same commentary phase active:333remaining Code intervals/17722bytes, first73780 in physical bank9. All current Code in physical banks0..8 is now reviewed; bank9 onward remains pending. Batch08f retired7intervals/1044bytes, added33independent helper units and22exact state words covering type1 cooldown/script/path/landing, type25 repeating script, types22/2A initialization/path phases and type49 movement/support/ceiling/landing/scripts. Type2A update's initializer destination and repeated type49 state3/4 destination are documented exactly. This worker cumulative44intervals/5331bytes,151helper units/112state words; verified byte-identical and reading refreshed. No successful commentary handoff yet.

Round002 active bank09a:15more intervals/666bytes,9helper units/10exact state words. Types40,48,41,3C and3F initializer explained. Reflected type48 spawn reads selector$20 then sentinel$94 from adjacent dispatcher code, while unreflected reads$00/$FF; retained exact mechanics and recorded bounded redirect question. Local data includes signedX offsets-32/+32. Fresh byte-identical verification/read refresh pass;318intervals/17056bytes remain, first74583. This worker59intervals/5997bytes,160helper units/122exact words. Phase still incomplete.

Round002 active09b:6Code intervals/930bytes,17helper units/14exact words retired for type45 collision lifetime,3F material6 pursuit/pause/child spawn and type4 grabbed-player interaction. Type4 state5 is null. Shared grabbing actor slot, escape interval/press gap/count and exit latch now consistently named in both player and actor consumers. Source retains low-Y-only child offset(no borrow) and the compact signedX delta stream format separate from movement scripts. Fresh byte-identical verify/read pass;312intervals/16126bytes first75582. Worker total65intervals/6927bytes,177helpers/136words; same phase active.

Round002 commentary sequential context rollover, incomplete:09c retired8intervals/607bytes with19helper units/15exact state words(types57,5A/5E,59/5F/5C and58initializer). Type59 frame-compare branches target the next instruction and always copy Y into attributes; source records actual mechanics. This worker73intervals/7534bytes,196helpers/151exact words. Authoritative remaining304intervals/15519bytes first76235; all current Code banks0..8 reviewed. Fresh byte-identical verify and reading refresh succeeded. All tools completed; no writer/server. Compact continuation prepended; resume SAME phase, not redirect or success handoff.

Round002 active09e/09f:41intervals/1936bytes retired,54independent helper units and43exact state words. Types0B/3E/6B,0F/10,76/77,63,23/24,7C/7D now explain animation/event/companion/terrain/state mechanics. Preserved allocator-failure final variant store and distinct exact Y removal tests; corrected old type10 attribute write described as timer. Fresh byte-identical verification passes; exact remaining257intervals/12786bytes, first79537. Same phase incomplete and active; bank9 final757-byte mixed scene/map helpers next.

Round002 active09g:final bank9 Code757bytes reviewed with17independent units. Type7D height/blink and terrain materialA nibble replacement/2x2 PPU patch helpers now readable; exact carry chains and unsupported-stage return behavior retained. All current Code banks0..9 reviewed. Fresh byte-identical verification/read refresh passed;256intervals/12029bytes remain, first81969(bank10).

Round002 active10a/10b:13intervals/1965bytes reviewed with44independent units/9exact words. Spawn family terrain/bounce/blink, reward selector$FF stub and separate division, contact response/callbacks, camera adjustment/history and collision-probe entries explained. Exact odd history comparison/branch and repeated original-Y activation retained as uncertainty. Fresh byte-identical checks pass;243intervals/10064bytes remain.

Round002 active10c/10d:13intervals/1826bytes,26independent units/12exact words. All current Code banks0..11 reviewed. Type53/54/5B scene setup, hit phases, motion/attacks, graphic cache, scroll double buffers and counted six-byte transfer stack explained; demo append preserves uninitialized-duration/inherited-buffer mechanics. Fresh byte-identical verify/read passed. Remaining230intervals/8238bytes first99394; title, audio and fixed services pending, phase incomplete.

Round002 active12a:all2198pending title/credits bytes in16intervals reviewed,18independent units. Stage38 option/pause display, scrolling stage37 animation/input continuation and stage3A credits/final vignette/palette tracks now explained with contextual fields. Old scrolling unknown-continuation defect repaired. Exact frame-not-equal3 shift, timeout latch ordering and four-byte palette track storage preserved. Fresh byte-identical verify passes. Remaining214intervals/6040bytes:audio16 and fixed30/31; current phase active/incomplete.

Round002 active16a/effects/16b:all3823pending audio bytes in95intervals reviewed. Added261independent units,39exact effect(initializer,update) records and detailed preset event entries. Audio stream call/return circular stack, control API, lifetime/preset/envelope continuations and every pending effect timeline explained; duplicate comparator ordering retained. Fresh byte-identical verification and reading refresh pass. Remaining119intervals/2217bytes in fixed banks30/31; two audio stale dependency defects repaired. Current source readability incomplete until fixed dependencies and whole-source assessment.

Round002 fixed a/b/c active:77intervals/1517bytes reviewed,37independent units. Fixed bank30 pending scope complete; remaining42intervals/700bytes in31. Script command dispatch/exact internal entries, split/scene dependencies, vertical spawn record scan, camera correction/CHR reference release/common actor helpers explained. Corrected old bounds result reversal: accepted=$FF,rejected=0 with negative-high near-screen assumption. Fresh byte-identical checks passed; final wrappers/geometry and full-source assessment next.

Round002 current-source commentary complete:304resumed intervals/15519bytes finished. Full inventory2061regions/2664units reviewed/source-linked;0pending. Final scope assessment and handoff saved. Fresh verify byte-identical/read refresh pass, map/state/baseline unchanged;12Unknown remain. One localized type4A direct-call/Data conflict is a bounded redirect question, not a skipped commentary block. Terminal editorial_audit remains pending.

Round002 redirect active: analysis/redirect/r002-a/frontier.md; checkpoint pass-002a,12 unknown bytes unchanged.

Round002 redirect r002-a: corrected type4A flags prove local BCS self-loop, confirmed actual artificial fetches19517/19519; old malformed downstream path not entered. Type48 reflection selects20 versus0 confirmed by bounded initialized actor calls. Full current Code32306instruction scan has0direct edges to12residual Unknown bytes; paired-PHA/specific consumer review underway. Map/state/source remain unchanged; independent disassembly proposal pending.

Round002 redirect complete: R002-L02/L03 ready (4 existing Data bytes proposed as dual-use Code;0unknown-byte gain; exact vector layout refinement). R002-L04 mechanics verified; R002-L01 blocked after exact12-byte review. Compact rejection review saved for coordinator inspection. Map/state/baseline hash dd872fd78ff1cb46968d12c08ec6b7c112bedc2516e62d6d5b0a98dbfa82fbdd unchanged. Readability2061regions/2664units/0pending remains separate; final editorial pending. Handoff analysis/round-002-redirect-handoff.md.


Round003 disassembly active: analysis/round-003/frontier.md. Reviewing dual-use correction and15-row vector layout; input pass-002a;12unknown unchanged.


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

## Round003 redirect start — r003-a
Input pass-003a, map/state hash f24d8835160d5de6fef4dca61b263064cc9d040d046142de5180b264f8aba567. Readability2063 regions/2665 units, pending0. Assess accepted corrections against primary residual evidence before any retry; preserve r002-a completed records. Frontier: analysis/redirect/r003-a/frontier.md.

## Round003 redirect resolved — r003-a
No new actionable lead after independently assessing accepted dual-use branch closure and bounded vector layout against primary residual evidence. Exact12Unknown/11intervals unchanged;393220classified. Current readability2063regions/2665units pending0; prior fresh byte-identical pass-003a preserved. Sound r002-a review reused with explicit changed-premise/local alternatives assessment at analysis/redirect/r003-a/rejection-review.md. No runtime replay, map/source/state/toolkit edits or cleanup. Leads pass appended; progression scenarios retained and reviewed. Terminal INCOMPLETE handoff analysis/round-003-redirect-handoff.md requires coordinator inspection; editorial audit pending. No new toolkit issues found (existing TI-001/TI-002 retained).

## Final editorial acceptance — active
Coordinator accepted the terminal r003-a research assessment; do not restart unchanged research. Frontier: targeted editorial acceptance across the actual full 2063-region/2665-unit inventory, reusing valid current-source review and scope assessment. Classification remains INCOMPLETE (393220/393232; 12 Unknown bytes); readability complete; editorial gate pending substantive review and supported audit. No archive execution, cleanup, classification or toolkit edits authorized.

## Final editorial acceptance — complete
Readability complete2063 regions/2665 units0pending; final editorial gate ACCEPTED. Concrete source-link/context metadata and uncertainty-prose defects repaired; maintained source bytes unchanged. Supported commentary.py audit passed after repairing the first audit's redundant-path rejection; fresh byte-identical rebuild and refreshed reading copy. Configured map/store validation ok=true. Exact commands/results/hashes: analysis/final-editorial-verification.json; substantive assessment commentary/editorial-audit.md; frontier/limits analysis/final-editorial-handoff.md. Classification remains terminal INCOMPLETE393220/393232,12Unknown in11intervals, checkpoint pass-003a hashf24d8835160d5de6fef4dca61b263064cc9d040d046142de5180b264f8aba567. Ready for coordinator archive preparation/requirement audit only; no archive execution, cleanup/removal or publication performed. No process remains running; no new toolkit issues found (existing TI-001/TI-002 nonblocking).


## Coordinator terminal handoff — orchestration complete, ROM INCOMPLETE
Final editorial audit passed2063regions/2665units; current-source and reading verified byte-identical, map/store validation passed. Coordinator inspected terminal rejection primary artifacts and recomputed exact12Unknown/11intervals; no ready leads or interrupted productive frontier. ROM classification: 100.00% (393220 / 393232 bytes classified; 12 bytes unknown). Rounded100.00 is not classification completion. Finalcheckpointpass-003a; terminal-handoff.md and completion-audit.md provide requirements/results. archive-options.json and archive-document-selection.json account for every Markdown path, selecting final records and omitting only9copied instruction scaffolds. No archive,cleanup,toolkit edit or publication performed. All workers/processes finished.
