# Disassembly progress

Round 01 disassembly active. Read phase, policy and workflow; configured binary is /root/romatlas/target/release/romatlas. Inspection confirms mapper 2, 131088 physical bytes (16-byte header, 128 KiB PRG, CHR RAM). Baseline established and byte-identical verified: analysis/baseline-verification.json. Classification remains initial pending vector/static/runtime inspection. No toolkit modifications or cleanup performed. Local socket transport is prohibited by sandbox; use persistent subprocess stdio instead (environment limitation, not toolkit defect).

Interim checkpoint: identified The Blues Brothers locally (copyright/title); researched NES manual and skip/RAM hypotheses in analysis/gameplay.md. Chapter1 active run/jump reached and captured (chapter1-action.png). Clean durable chapter1-ready.json and pre-patch.json snapshots saved. Two-controller skip attempt did not change $0750. Artificial stage1 patch with PC $F587 enters an interstitial; successful later-level visit still unverified. Live Python PTY session36167 retains RPC subprocess; replay log analysis/round01/rpc-live.jsonl, scenario milestone state analysis/scenarios.json. Structural annotations not yet applied; observed fetches plus bounded static discovery will be consolidated after these scenario runs. No ROM writes. No toolkit issues found so far.

First structural batch saved: 10123 Code bytes in 451 regions; map.validate passes. Observed instruction roots plus bounded static branch continuation, preserving effective contexts. RAM execution roots rejected as ROM and left separate. Chapter2 active warehouse reached under artificial-state loader recipe and action input; chapter2-final.png proves distinct environment. Later milestones in progress.

Second structural batch: 61180 / 131088 bytes classified (46.67%). Added parsed header/vectors, five partial repeated tile/attribute matrix intervals and uniform non-code runs; Data inspection evidence analysis/round01/data-inspection.json. Map validation passes; gaps remain Unknown. Artificial chapter3 prison visit completed (stage2-action.png), source chapter numbering=RAM stage+1. Chapter4 sewer test running.
Interim structural checkpoint round01-interim verified byte-identical (analysis/round01-interim-verification.json). Header + PRG classified 61180/131088 = 46.67%; this is incomplete. Broad gameplay visits continue. Current state is durable even if runtime session ends.

Progression update: artificial chapter3 prison and chapter4 sewer visited with distinct scenes/action; chapter5 construction attempt shows rendering artifacts and remains unverified pending settling check. Ending transition code inspected at $F50A: chapter5 selector enters $F512 concert branch; executing bounded $37=$0750=5 / PC=$F50A from saved clean state. Raw edits are not claimed as natural reachability.

Ending shortcut reaches WELCOME TO THE BAND and staff-name credits with terminal text; scroll artifacts retained. Treat accepted artificial-state fetches as execution only, not natural reachability or full rendering fidelity. Additional Start/settling check running.

Construction settling test: restore stage4-visited, release both, RAM energy $042A=3/invincibility hypothesis $03C2=2, right+B40 frames; active scrolling and HUD changes but artifacted display persists. Not accepted as clean stage visit. Ending extra Start yields player/color animation behind credits; retain partial terminal evidence. Straightforward bounded stage tests complete; further PPU initialization contract belongs in redirect rather than blind repetition. Final observed/direct-callee static consolidation running.

## Round 01 disassembly completed — incomplete ROM classification

ROM classification: 48.71% (63847 / 131088 bytes classified; 67241 bytes unknown).

Code 12750 bytes; Data 51097 bytes (including 16-byte iNES header). No explicit Unknown annotations; 155 unknown complement intervals remain. Counts derive from unique physical intervals in internal store, not raw fallback or execution coverage.

| 8KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 2109 | 2109 | 6083 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 1686 | 1686 | 6506 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 1412 | 1412 | 6780 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 1484 | 1484 | 6708 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 63 | 2852 | 2915 | 5277 |
| 10 | 544 | 181 | 725 | 7467 |
| 11 | 1787 | 97 | 1884 | 6308 |
| 12 | 401 | 34 | 435 | 7757 |
| 13 | 442 | 43 | 485 | 7707 |
| 14 | 3064 | 143 | 3207 | 4985 |
| 15 | 6449 | 80 | 6529 | 1663 |


Final current map: maps/current.json, state analysis/state.json. Authoritative checkpoint maps/round01-disassembly-corrected.json; source generated/round01-disassembly-corrected/game.asm; verification analysis/round01-disassembly-corrected-verification.json confirms ok=true and byte_identical=true. Generated diagnostics empty. ROM SHA256 unchanged 06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12. Earlier round01-interim and round01-disassembly checkpoints are historical/superseded; do not resume their rejected static path.

Final audit correction: apparent BPL/JSR at physical $17FDB/$17FDD is four-byte indexed sprite-layout Data. BNE/BEQ complement proves no fallthrough; observed $BFCF TXA begins immediately after table. Withdrawn bogus call to physical $14A38 and its unsupported descendants, revalidated all Static Code from accepted roots behind barrier. Prior evidence/checkpoints retained; see audit-correction.json, pre-audit-state.json, expansion-evidence-pre-audit.json. Fixed-bank continuation uses mapper2 invariant banks14/15 with null lower slots after writes, never invented return contexts; static accepted entries come from actual branches/direct calls with inspected boundaries. No legal-decode-only claims.

Runtime summary: 8562 unique instruction identities and 12113 contextual roots across natural/artifical scenarios, 13639 contextual edges. These include RAM identities, not all classified ROM. Detail capture intentionally capacity0 at end; 10415745 detail events dropped, aggregate_dropped=0. Retained deterministic RPC input/patch history and selected durable snapshots/captures. Replay resets trace before needing forward time; snapshots restore emulator state but not trace history. Final emulator state artificial, bank slots[12,13,14,15], frame513 after stage4 settling.

Progression: natural copyright/title/chapter1 running+jumping; artificial warehouse, prison and sewer visits with distinct scenes/action. Construction attempted with extra settling/health shortcut but artifacted display persists; ending WELCOME TO THE BAND, staff-name credits and color/player animation visited with artifacts. No natural late-game reachability or clean rendering claim. See analysis/scenarios.json, analysis/gameplay.md and progression.py. Further screen-initialization investigation is a concrete redirect question; no repeated long blind input runs warranted.

No toolkit issues found; analysis/toolkit-issues.md retained. No toolkit changes, cleanup, archive or publishing performed. Disk at handoff ~3.8GiB free; round01 evidence ~102MiB, scratch12KiB. Stop this disassembly pass at supported easy-gain handoff; commentary must cover all currently classified source before redirect. This is not terminal method exhaustion: compressed graphics/resource consumers, object-state dispatch contracts and rendering initialization remain promising targeted research.

Compact phase handoff: analysis/round01-disassembly-handoff.md. Round01 disassembly phase finished; next commentary across the full corrected classified source, then redirect. No further runtime operations pending.

## Round01 commentary active

Prepared commentary/ baseline with byte-identical preparation; 2312 scaffold regions. All classified readability pending. Working in commentary/source; frozen before/ preserved. Next: complete physical-order routine review, data/unknown notes and symbols; fresh verify/read before handoff. No redirect begun.

Commentary checkpoint: full compact listing inspected; 139 connected mechanical routine descriptions and 301 RAM/hardware symbols added locally. Five-slot object arrays, 24-bit positions, three-byte sprite requests, four-byte OAM/spawn entries and fourteen-byte audio voices explained. Frontier: semantic branch/entry labels and references, Data/Unknown notes, prose correction review, scaffold review, verify/read. No phase completion or redirect; durable details commentary/progress.md and analysis/round01/commentary-units.json.

## Round01 commentary completed — current-source readability

All 63847 classified bytes (48.71% of 131088) have source readability review: 12750 Code bytes in 139 connected groups, 51097 Data bytes in 81 spans; all 155 Unknown spans (67241 bytes) have honest boundary notes. No classification changed. Preserved 2312 review scaffold entries, added 646 semantic source labels, consistent RAM/hardware/record operands, connected mechanical descriptions and explicit unresolved meanings. Corrected entry/return placement, indirect-JMP offset accounting, player frame/linked-slot alias and gravity-cap prose during review. Exact coverage: analysis/round01/commentary-coverage.json and commentary/review.json.

Fresh final commentary verification ok=true, byte_identical=true; source SHA256 470f10f1379c921f74289eec50bb1ec2b5845b51fd9004f8761a636ca4abbc85; ROM SHA256 unchanged. reading-after.asm refreshed. Final editorial/archive acceptance remains pending separately; no audit/redirect run in commentary. No readability backlog remains for this frozen baseline. Unresolved resource extents, object handler identities and normal screen initialization remain existing redirect leads; commentary/uncertainties.md distinguishes them from source review gaps. No toolkit issues found; analysis/toolkit-issues.md retained. Compact handoff analysis/round01-commentary-handoff.md.

## Round01 redirect active
Attempt analysis/redirect/round01-contracts: policy/source/handoff review complete. Investigating resource endpoint contracts, connected object dispatch and omitted screen initialization. Classification/map/source preserved.

Redirect intermediate: five consumer-bounded compressed streams validate exact endpoints,15488 unknown bytes proposed (resource-streams.json).54 handler contracts inspected/341 initial CFG candidates recorded, shared speed/animation continuations under review. Improved F4A9 scene-reset route with chapter RAM37=4/5 produces coherent construction and credits; full recipes/captures in analysis/redirect/round01-contracts. Map/state/source unchanged. Same redirect continues.

## Round01 redirect completed — combined ready proposal
Combined40043 unknown-byte proposal(37276Data,2767Code) saved analysis/redirect/round01-contracts/proposal.json,zero overlap/conflict checks. Accepted63847classified/67241unknown unchanged. Five exact compressed endpoints,raw pattern count/source resources,metatile quartet bounds,eleven terminated screens,packed generic continuations and narrow consumer tables; complete coherent object handler/shared helper graph and independent player branch tails. Initial source/ROM hashes unchanged. Full scene-reset F4A9 artificial routes resolve earlier construction/ending artifacts; scenarios updated with captures/recipes. Leads ready for independent disassembly,then commentary/redirect; no terminal exhaustion. Handoff analysis/round01-redirect-handoff.md. No toolkit issues found; no map/source/toolkit edits,cleanup,archive or publishing.

Round01 redirect gate **pending**,same-phase continuation requested by coordinator. Prior deferred animation work has an actionable count/pointer method; inspecting eleven54-byte type rows and count-bounded overlapping records. Correcting overbroad player return dependency: several helpers are plainly RAM-only RTS and permit continuation now. Historical proposal preserved; no next phase authorized yet.

Round01 redirect same-phase revision2 ready,pending coordinator review. Corrected premature nested/return deferrals: descriptor stride/count animation overlaps,sprite FF streams,spawn-pointer provenance,27 contact/30 floor handlers and bank-aware normal returns investigated. proposal-v2.json union52428unknown bytes(47986Data,4442Code),zero conflicts/overlaps;14813 unproposed if all accepted. Five complementary branch concerns explicitly resolved by independent incoming branches,new-code-boundary-review.json. v1 records preserved. Initial ROM/source/map/state unchanged; no toolkit issues. Handoff analysis/round01-redirect-handoff.md; no next phase before coordinator gate.


## Round02 disassembly completed — incomplete classification

ROM classification: 97.32% (127569 / 131088 bytes classified; 3519 bytes unknown).

17,223 Code bytes;110,346 Data bytes including16-byte header. Added63,722 unique bytes:4,473 Code +59,249 Data. No explicit Unknown annotations;49 complement intervals remain,recorded with physical identities,effective contexts and frontier dispositions in analysis/round02/unknown-ledger.json. Internal store unique-byte counts are authoritative; neither raw export gaps,execution coverage nor readability count as classification.

|8KiB PRG bank|Code|Data|Classified|Unknown|
|---:|---:|---:|---:|---:|
|0|0|8192|8192|0|
|1|0|8192|8192|0|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|0|8192|8192|0|
|6|0|8192|8192|0|
|7|0|8192|8192|0|
|8|0|8192|8192|0|
|9|357|7835|8192|0|
|10|546|7615|8161|31|
|11|1787|6062|7849|343|
|12|1082|7019|8101|91|
|13|442|7701|8143|49|
|14|5100|283|5383|2809|
|15|7909|87|7996|196|


Accepted entire independently reviewed revision2 proposal:47,986Data/4,442Code. Primary packet/nested/sprite/spawn/descriptor/handler/player/auxiliary evidence retained in analysis/redirect/round01-contracts. Independent extent/context checks:analysis/round02/review.py,independent-review.json,decoded-proposal.json. Consumer/source/field/stack review and bounded graph reconstruction:graph-review.py/json;4,926 contextual nodes and1,920 proposed instructions,zero unsupported proposed nodes after impossible complementary fallthrough removal and explicit mapper/I/O write barriers. Applied exclusively through map.annotate Static;annotation-results.jsonl and apply-reviewed.py preserve edits. Code is statically structural,not newly Observed;natural selection/reachability unresolved beyond previous scenarios.

Supported follow-through beyond proposal:extra-review.json/extras.py add31Code bytes in auxiliary caller/restore sequences and4,344Data bytes (title block/attributes,generic graphic patterns,default voices,effect pointers,contact data,timer/mapper lookups). pattern-review.json/pattern-followthrough.py add6,919Data bytes:exact bank0 title192-byte copy,graphic continuation and six generic packed-audio-pattern slices. Pattern fallback followed practical reference/consumer and boundary inspection;no complete grammar/exclusive stream domain,exact generator,deadness or padding claim. Bank4 unselected music-pattern tail has248-byte immutable match with bank2BE04..BEFC. Generic17-byte sprite/10-byte spawn motifs remain selection-unresolved. Descriptor byte15,overlapping animation packing,count-zero use,null sprites and translation-view overlap retain their prior uncertainty.

Review corrections preserved alongside historical records:133 sprite slots contain129 nonzero slots but127 distinct nonzero starts (prior prose128 was inaccurate). BDB3 actual opcodeF0 is BEQ to BDBC,not BNE;independent branch entry remains valid. Historical17FDB false path remains rejected and classified Data;no new Code depends on it. Auxiliary creator structural call sites now additionally supported at physical1CB44..1CB4E,1E46C..1E47B,1E4A1..1E4A7;natural calling reachability remains unclaimed. Extra script's initial fixed-bank conversion error was rejected outside-image before save;reload/correct/apply succeeded,not a toolkit finding.

No runtime replays in this pass:reused broad representative scenarios and corrected scene-reset construction/ending captures from prior redirect. analysis/scenarios.json retains inputs,patches,budgets,snapshots and milestones;corrected construction/credits visit supersedes earlier shortcut artifacts. No new scenario claim or hardware/full-fidelity claim. R01 resource/state/player/nested leads verified and consumed;screen initialization remains verified. No repeated failed lead on unchanged premise.

Stopping at supported-batch handoff,not terminal method exhaustion. Concrete next redirect frontier:resolve unclassified object/helper bodies D286..D864 via actual state/caller/field contracts;audio indirect-command/return domains and finite period/scene arrays DBxx..DFxx;PPU helper jump tails B8DF/B972 family using shared routine return contracts;remaining mixed DMC-like fixed-bank patterns C000..C4EB and bank5/6 short data gaps via consumers/context. A legal decode or isolatedRTS is insufficient;no blanket residual import. Existing observed/static sources contain useful starting references,so no further gameplay grind is warranted before these targeted questions. Exact49 residual intervals and boundaries are in ledger;this pass does not claim a terminal rejection audit.

Final current map maps/current.json equals maps/round02-disassembly-final.json;internal store analysis/state.json and durable analysis/round02/final-state.json. Fresh authoritative checkpoint generated/round02-disassembly-final/game.asm;analysis/round02-disassembly-final-verification.json confirms ok=true,byte_identical=true;generated diagnostics empty. Earlier round02-interim and round02-disassembly files are preserved historical checkpoints. Final mapSHA256 fcc2c39c2cf13998983c1eeacc38cd048cb0db0429308c9060b1869ab1b591d5;ROM SHA256 unchanged 06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12.

Full old commentary/source/game.asm preserved byte-for-byte,SHA256 470f10f1379c921f74289eec50bb1ec2b5845b51fd9004f8761a636ca4abbc85. Current-source readability now needs next commentary carry/review for63,722 added bytes;this disassembly did not prepare/regenerate/edit commentary. Byte-identical generation alone is not editorial completion. No archive,cleanup,deletion,publishing or toolkit edits.

No toolkit issues found;analysis/toolkit-issues.md retained. Final disk free~3.64GiB. One disassembly pass complete;next phase is full current-source commentary,then targeted redirect if coordinator accepts.

Round02 commentary ACTIVE: preserved ENTIRE previous commentary at analysis/round-2/commentary-previous; prepared current baseline; carried 63847 prior classified bytes across exact physical ranges; fresh verify passed byte-identical. Full new63722-byte readability and changed-dependency review pending; see commentary/progress.md. Continue commentary, not redirect. No successful handoff yet.

Round02 commentary COMPLETE current-source readability: all17223Code/110346Data explained,49unknown boundaries explicit; all2660 scaffold regions preserved/current locations; zero uncovered bytes. Fresh final byte-identical verify and refreshed reading-after; source2c0110d4f4b7155dfef2b499fb8bd4e81a14303fbc66b970ea0bc461780a8ef9. Full old commentary preserved analysis/round-2/commentary-previous. Compact handoff analysis/round02-commentary-handoff.md; coverage analysis/round-2/commentary-coverage.json. Classification unchanged97.32%; final editorial_audit pending. Coordinator acceptance before next phase. No toolkit edits/archive/cleanup/publishing.

Round02 redirect ACTIVE, commentary gate accepted. Unique attempt analysis/redirect/round02-contracts. Frozen map fcc2c39c2cf13998983c1eeacc38cd048cb0db0429308c9060b1869ab1b591d5, source2c0110d4f4b7155dfef2b499fb8bd4e81a14303fbc66b970ea0bc461780a8ef9; accepted127569/131088,unknown3519/49intervals unchanged. Initial frontier: object D286..D864 contracts; audio command/period/scene domains; PPU helper return continuations; mixed C000..C4EB and short pattern gaps. Inspect primary static evidence and narrower claims before inheriting missing-caller barriers; reuse full reset/credits scenarios. No map/source mutation or unchanged runtime replay.

Round02 redirect intermediate: explicit30-entry tile-response dispatch now anchors D286..D864 graph; PPU B906 returns close13 jump tails. Identified BDAF ten-byte RAM copier image within bank5 graphics gap, explicit16-byte reset copy to06F0; treating graphics/code-image overlap separately. Audio split-table domains and DMC/pattern data checked; player/continue-screen independent tails under contract review. Frozen classification unchanged. Detailed current frontier analysis/redirect/round02-contracts/frontier.md.

Round02 redirect COMPLETE combined proposal ready for independent disassembly review, not terminal exhaustion. `analysis/redirect/round02-contracts/proposal.json`:3515 unknown bytes (1847Code/1668Data),802 instruction boundaries,zero overlap,zero unsupported/outgoing unresolved instruction targets,zero direct controls into proposedData. Explicit30-class object graph,16-entry audio domains,13PPU JMP tails,small dispatcher gaps,exact palette/graphics/upload images,screen/exit tables,DMC/pattern gaps and player/continue-screen contracts followed together. Primary actual return/bank/stack reasoning in `structure-evidence.md`,checks `flow-review.json`/`final-check.json`;all49initial residual dispositions. Remaining4unproposed isolatedRTS D25C,D5F5..7;no speculative closure. Accepted counts remain127569/131088(97.32%),3519unknown,17223Code/110346Data. Map,state,source,ROM byte-identical to starting checkpoint; prior fresh verify reused, no source changes requiring rebuild. No runtime replay/toolkit edits/cleanup/deletion/archive/publication. No toolkit issues found. Compact handoff `analysis/round02-redirect-handoff.md`; next phase only after coordinator acceptance.


## Round03 disassembly completed — incomplete classification

ROM classification: 100.00% (131084 / 131088 bytes classified; 4 bytes unknown).

19070 Code bytes; 112014 Data bytes including16-byte header. Added3515 unique bytes:1847 Code (802 instructions),1668 Data, all Static. Rounded100.00% is not completion:4bytes in2physical intervals remain Unknown. Internal unique physical-byte complement is authoritative; raw export fallback, source readability and observed execution remain separate.

|8KiB PRG bank|Code|Data|Classified|Unknown|
|---:|---:|---:|---:|---:|
|0|0|8192|8192|0|
|1|0|8192|8192|0|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|0|8192|8192|0|
|6|0|8192|8192|0|
|7|0|8192|8192|0|
|8|0|8192|8192|0|
|9|357|7835|8192|0|
|10|577|7615|8192|0|
|11|1832|6360|8192|0|
|12|1085|7107|8192|0|
|13|459|7733|8192|0|
|14|6655|1533|8188|4|
|15|8105|87|8192|0|

Independent actual consumer/table/branch/stack/bank review: analysis/round03/independent-review.md. All802 exact instruction decodes match configured release binary; tables independently recomputed;3515-byte union has zero overlap with accepted bytes. All four ready R02 leads verified, including full response graph, returning PPU/audio paths, orphanDE7B static helper, specific bounded data and independent session tails. Copier Data source-image10bytes distinguished from16-byte upload and6-byte graphic overlap; unsupported uploaded views not invented.

Residual ledger: analysis/round03/unknown-ledger.json. Physical1D26C CPU D25C and1D605..1D608 CPU D5F5..D5F7 retain code-stub versus return-byte-data ambiguity. Missing live caller is not a universal prerequisite or the rejection premise. Ready connected easy gains exhausted; next redirect independently assesses residual frontier. Successful broad progression/construction/credits routes reused from analysis/scenarios.json; no unchanged runtime replay.

map.annotate58 runs (39 Code,19 Data), map.validate ok, saved analysis/state.json and maps/current.json, fresh checkpoint round03-disassembly-final validates/generates/verifies byte-identical. Verification: analysis/round03-disassembly-final-verification.json. Current/state/checkpoint map SHA256 f122f1e8046caf14315839029f0238d0770ed8036142d8e5e65f329e60ce45b4; separately verified state SHA matches. ROM SHA unchanged 06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12. Entire commentary untouched; game.asm SHA 2c0110d4f4b7155dfef2b499fb8bd4e81a14303fbc66b970ea0bc461780a8ef9. Commentary describes prior source and must be carried intact into next commentary phase before reviewing new source. No commentary generation performed in disassembly.

No toolkit issues found; analysis/toolkit-issues.md preserved. No toolkit edits, cleanup/deletion, archive or publication. Pass stops incomplete; next phases controlled by coordinator.


## Round03 redirect completed — incomplete, coordinator reasoning review pending
Attempt analysis/redirect/round03-residual-review. Current map/state matches round03-disassembly-final SHA256f122f1e8046caf14315839029f0238d0770ed8036142d8e5e65f329e60ce45b4; source ab1cf5d4f2451fb47feeb6362a414f4f09b429b71f9cd50b9e9671a53dec1872. Recomputed131084/131088classified;4unknown in2intervals,physical1D26C/D25C and1D605..1D608/D5F5..7,fixedbank14. Combined localized boundary,control,stack,finite-table,address-pair and generic-data reasoning; no ready lead,0proposed/accepted newbytes. R03-ISOLATED-RTS outcome rejected as insufficient structural evidence,not externally blocked. Decisive primary links and scope of each negative result in rejection-review.md; coordinator must inspect before terminal acceptance. No map/source/state/ROM/toolkit changes or runtime replay. Full-readability/fresh byte-identical round03 commentary verification reused; final editorial audit pending. One stale E3CB prose sentence flagged for audit at source21124. No publication/archive/deletion.


## Coordinator terminal handoff — terminal incomplete

Three sequential phase rounds completed. Round03 redirect rejection rationale accepted after inspecting decisive ROM/control/table artifacts, localized competing claims and evidence standards. No ready lead or external blocker remains. Final editorial acceptance and fresh byte-identical audit passed,2671regions/2799units reviewed. Internal/stock maps validate; ROM unchanged.

ROM classification: 100.00% (131084 / 131088 bytes classified; 4 bytes unknown). Rounded100.00% is incomplete. Exact residuals119404length1 and120325length3 remain Unknown. See analysis/terminal-handoff.md, analysis/coordinator/terminal-rejection-review.json, analysis/final-editorial-handoff.md and final editorial/audit records. Final source a4b8f49e41b69c0973b468f96641f5bc27212d0a0c5db6817d9b79c9a35a4a72; final map f122f1e8046caf14315839029f0238d0770ed8036142d8e5e65f329e60ce45b4.

No toolkit edits/archive/cleanup/deletion/publishing. Archive preparation only: archive-options.json and final document-selection audit account for every Markdown document; final conclusions/history preserved. Orchestration completion is separate from ROM classification. Reopen structural work only with changed evidence, justified revised reasoning/method or explicit reassessment.

## Manual-fallback round04 disassembly COMPLETE — commentary next

Explicit batch-only user authorization permits generic Data/Manual fallback solely for physical119404 length1 and120325 length3 if the freshly recomputed exact internal-state complement is <=10bytes and matches; zero unknown skips annotations; above10/mismatch stops. Authoritative preflight confirmed exactly4bytes in those2ranges before mutation. Applied exactly2 map.annotate operations,4Data bytes,Manual provenance,unknown meaning. Isolated RTS-code versus return-valued-data ambiguity remains as documented by historical rejection review; this is user-directed workflow fallback, not new evidence-backed structural classification. No research or gameplay replay; no lead consumed/produced beyond explicit fallback authorization.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Map coverage now19070Code/112018Data. Evidence-backed classification remains131084bytes plus4Manual fallback bytes; do not claim evidence-backed whole-ROM success from map coverage. Per-bank unique counts in analysis/manual-fallback/final-counts.json: bank14=6655Code/1537Data/0Unknown,all others unchanged with0Unknown;16-byte headerData included.

Input round03-disassembly-final/state/map f122f1e8046caf14315839029f0238d0770ed8036142d8e5e65f329e60ce45b4. Output analysis/state.json,maps/current.json,maps/manual-fallback-disassembly.json all793855e9c153cd3b4cdc745d426e250bac51066c28cd5025ebe0cc117da33ee0. Internal and CLI map validation pass. Fresh checkpoint generated/manual-fallback-disassembly verifies ok=true,byte_identical=true in analysis/manual-fallback-disassembly-verification.json. ROM hash unchanged06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12; commentary source and historical terminal handoff unchanged. Evidence/logs/identities in analysis/manual-fallback; compact handoff analysis/manual-fallback-disassembly-handoff.md. Current phase commentary pending; preserve prior editorial review credit and update4Manual Data bytes plus current-source reports/reading copy/fresh verify. No commentary edits in this phase; no toolkit/archive/delete/publish. No toolkit issues found; analysis/toolkit-issues.md preserved. Disk about3.6GiB free.

## Coordinator authorized-fallback final acceptance

Sequential Manual-fallback disassembly, full current-source commentary reconciliation and separate final editorial review completed. ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Exactly4bytes remain structurally ambiguous and are generic Data solely by explicit user authorization, not evidence-backed role proof: physical119404 length1 and120325 length3. The original131084 evidence-backed bytes and prior investigations remain unchanged. No redirect research reopened.

All2671 regions/2799 units reviewed, no pending readability ranges. Strict final audit passed with fresh byte-identical verification; reading copy refreshed. Read-only editorial status complete=true,reasons=[]; internal/portable maps valid; fresh manual-fallback-disassembly checkpoint byte-identical and ROM unchanged. Current final conclusions analysis/manual-fallback-final-handoff.md supersede historical terminal counts only; older handoffs remain preserved. Coordinator independently checked exact annotation delta and physical inventory. archive-options.json selects final fallback record and new handoffs, accounts all Markdown, omits only duplicate instructions. No archive/deletion/publication/toolkit edit. No toolkit issues found. Requirement-by-requirement final audit: analysis/coordinator/fallback-completion-audit.json.
