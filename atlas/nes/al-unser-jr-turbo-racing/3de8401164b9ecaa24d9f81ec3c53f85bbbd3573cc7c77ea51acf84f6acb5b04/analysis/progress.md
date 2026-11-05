# Disassembly progress

## Round 1 disassembly checkpoint — pass-001

ROM classification: 60.31% (158102 / 262160 bytes classified; 104058 bytes unknown).

Incomplete. Verified baseline and pass-001 both rebuild byte-identically. State: analysis/state.json; portable map: maps/current.json; source: generated/pass-001/game.asm; verification: analysis/pass-001-verification.json.

Code 8,914 bytes; Data 18,116 bytes (header/vectors and bounded maximal uniform runs); Image 131,072 bytes (entire CHR section). No Unknown annotations count as classified.

Bank | Classified | Unknown
--- | ---: | ---:
PRG 00 | 884 | 7308
PRG 01 | 296 | 7896
PRG 02 | 0 | 8192
PRG 03 | 2859 | 5333
PRG 04 | 0 | 8192
PRG 05 | 2687 | 5505
PRG 06 | 1984 | 6208
PRG 07 | 3627 | 4565
PRG 08 | 277 | 7915
PRG 09 | 2256 | 5936
PRG 10 | 1931 | 6261
PRG 11 | 2000 | 6192
PRG 12 | 1164 | 7028
PRG 13 | 456 | 7736
PRG 14 | 3988 | 4204
PRG 15 | 2605 | 5587
Header | 16 | 0
CHR 00–15 | 131072 | 0

Evidence: round1/code-proposals.json, observed-decodes.json, discover-*.json, annotation-proposals.json/results.json and uniform-runs.json. Physical ROM roots only were accepted; RAM execution was excluded. Static paths started at validated vectors with [0,1,14,15] banks; observed successors supply effective contexts. No instruction overlaps.

Gameplay: title, mode selection, time-trial setup, Brazil car setup/music/grid and active acceleration/turbo/steering/braking visited. Later-track selection attempts retained; one 16-step wrap returned to Brazil, so no later-track claim. Longer menu release/wait intervals required. Additional targeted selector experiment completed and awaits interpretation.

No toolkit issues found; see [toolkit-issues.md](toolkit-issues.md). Trace detail capacity zero intentionally drops detail; aggregates had zero drops. Bounded timeout preserves completed execution and is not a bug.

Remaining: PRG-RAM upload origin ($6600+), course selector contract, championship qualifying/result/ending transitions, unsupported static paths behind mapper/call barriers and PRG data formats. No terminal exhaustion claim.

## Round 1 commentary checkpoint — pass-001

Current classified-source readability complete. All 335 code intervals (8,914 bytes), 28 Data intervals, one CHR Image interval and boundaries of 154 Unknown intervals have reviewed source explanations. Buildable commentary/source/game.asm adds 391 explicit mechanical comment blocks and 292 consistent exact-value symbols. Review scaffold remains intact; final editorial/archive audit pending.

Fresh commentary/verification.json: byte_identical=true. Reading copy refreshed at commentary/reading-after.asm. Source SHA256 de8cd4ad93d191c79956d9d417a28830452b8c5ae8a923650b3edba9030e9c57. Classification and map/state unchanged, input unchanged. Scope/hashes: round1/commentary/scope-result.json. Full compact handoff: round1-commentary-handoff.md. Prior disassembly records above are preserved.

Additional localized structural question for redirect: bank-10 CPU $8563 / physical 83315 is emitted as BRK but read by an indexed audio-control lookup at $8512. Commentary retains its upstream Code classification and explicitly flags the conflict. Prior RAM-upload, course-selection and championship-transition leads remain open. No toolkit issues found; no toolkit edits or gameplay replay during commentary.

## Coordinator transition — round 1 redirect active

Separate commentary worker completed. Coordinator checked current artifact hashes against round1/commentary/scope-result.json, fresh byte-identical verification, and all 518 review records marked reviewed; final editorial audit remains pending. Separate sequential redirect worker round1_redirect started from pass-001 and round1-commentary-handoff.md. Classification remains 60.31% (158102 / 262160 bytes classified; 104058 bytes unknown). Current phase is redirect: investigate saved upload, course/progression and audio-table questions before selecting the next cycle. No terminal assessment yet. Archive document contract inspected and saved in coordinator-archive-contract.md; archive-options.json awaits terminal inventory.

## Round 1 redirect active — r1-contracts
Separate worker investigating RAM upload, course/ending state and audio lookup. Attempt plan/frontier: redirect/r1-contracts/plan.md. Found documented $6401/$bf course contract; prior $062e/$062f menu delta is within audio state, so do not inherit its course-selector premise. Reset static continuation and audio boundary decodes saved. Initial boot trace timed out before evidence collection; revised partial-result collection pending. Classification unchanged.

r1-contracts update: exact RAM upload source/copy sentinel contract solved (physical58110,3790 payload+6copiedFF), gameplay course latch solved and Spain race visited through ordinary inputs; audio51-command handler graph inspected as connected continuation, with structural data/code proposals pending disassembly. Primary findings and live frontier in redirect/r1-contracts/findings.md; classification unchanged. Further final-course and championship/ending shortcut work remains active.

r1-contracts: final course U.S.A. ROUND16 visited naturally; GP player/start menu and ROUND1 advice/practice/qualify menu visited naturally. Closing sequence contract located bank8$81f3 selected by result routine on$bf15; practical shortcut investigation active. Independent bank4/5 PPU assets have finite course/descriptor consumers and repeated tile/text layouts. See updated findings.md. Session90399 completed; no emulator operation from it remains live.

r1-contracts: GP qualifying result board reached via artificial terminalflag; GP race initialization gates (scrolling grid, BGM, player-ready) resolved, actual course15 city-road now captured. Current terminalflag46 test on actual race60790, exact frontier in findings.md. Complete finite source dispatcher followup adds roadside renderer643bytes+183sprite-data, timedtext boundedhandlers/tables, while HUD/race alreadyclassified. No maps/source changes.

r1-contracts: final GP resultboard/driverpoints observed, then actual losing-championship closing message through ordinary confirmations. Artificial course/finish patches explicit. Corrected pit experiment from natural active-driving snapshot reaches PITSTOP; repair exercise25880 remains. Five pit handler contracts inspected. Combined proposals currently20390 unknown bytes (0 accepted); map158102 classified/104058 unknown unchanged. Preparing batch handoff, not terminal exhaustion.

## Round 1 redirect ready handoff — r1-contracts
Ninecombinedready leads;20693uniquecurrentlyunknownbytes proposed forindependentdisassemblyreview,0accepted. Gameplay nowincludesnaturalSpain/U.S.A.trials,GPqualifying,andartificialqualifying/finalGPrace/result/driverpoints/losingending pluspit/refuel1->2. No repair/winningending/naturalchampionshipcompletion claim. Allconnected audio/setup/finite dispatch/GP/pit avenues inspected, independentPPUassets andwholecurrent-sourcecontinuations packaged. Compacthandoff: round1-redirect-handoff.md; detailedfindings,proposals,exact169bank-splitunknownledger andboundarychecks inredirect/r1-contracts/. No terminalexhaustionclaim.

Map/statefded0bf4e573c67d93860eaede594282b2640f885e4c126a298395b85bac2fda unchanged:158102/262160classified(60.31%),104058unknown. Source de8cd4ad93d191c79956d9d417a28830452b8c5ae8a923650b3edba9030e9c57 unchanged;ROMidentityunchanged. No toolkit/input/source edits,classification mutations,archive/publication/deletion. DocumentedrelocatedCode/PRGRAMdebugger-writefeaturelimits addedtoolkit-issues.md. No liveprocesses atboundary;disk9.1GiBfree/scratch96KiB.

## Coordinator transition — round 2 disassembly active

Round 1 redirect worker completed; coordinator inspected round1-redirect-handoff.md, its combined lead recipes and actual-versus-corrected observation claims. Nine ready leads propose 20,693 unique unknown bytes plus one Code-to-Data correction. Connected audio/upload/menu/GP/pit/dispatcher investigation and independent PPU asset avenue support the next pass; diminished-prospect deferrals are explicit. This is not a terminal exhaustion assessment. Separate sequential round2_disassembly worker started from pass-001 with round1-redirect-handoff.md; no competing writer. Input state remains 158102 classified / 104058 unknown (60.31%). Completed r1-contracts research remains immutable.

## Round 2 disassembly — pass-002

ROM classification: 68.20% (178795 / 262160 bytes classified; 83365 bytes unknown).

Accepted nine ready leads after independent exact structural review; +20693 unique bytes, one Code/BRK->Data correction. Static provenance distinguishes source structure from artificial/natural gameplay. Evidence: [review](round2/review.md), [annotations](round2/annotations.json), [accepted decodes](round2/accepted-decodes.json), [counts and residual ranges](round2/counts.json). Current map/state saved and validated; checkpoint maps/pass-002.json generated/pass-002 rebuilt byte-identically, verification analysis/pass-002-verification.json. Map SHA256 bbefab9cc9e3a09a05180c2ce5dd3ba3e01dbdb402c00094dd93bb3b34876f9d.

| Bank | Classified | Unknown |
|---|---:|---:|
|Header|16|0|
|PRG00|884|7308|
|PRG01|296|7896|
|PRG02|0|8192|
|PRG03|2859|5333|
|PRG04|8192|0|
|PRG05|8167|25|
|PRG06|2167|6025|
|PRG07|7417|775|
|PRG08|296|7896|
|PRG09|2256|5936|
|PRG10|2511|5681|
|PRG11|2000|6192|
|PRG12|1323|6869|
|PRG13|940|7252|
|PRG14|4921|3271|
|PRG15|3478|4714|
|CHR00|8192|0|
|CHR01|8192|0|
|CHR02|8192|0|
|CHR03|8192|0|
|CHR04|8192|0|
|CHR05|8192|0|
|CHR06|8192|0|
|CHR07|8192|0|
|CHR08|8192|0|
|CHR09|8192|0|
|CHR10|8192|0|
|CHR11|8192|0|
|CHR12|8192|0|
|CHR13|8192|0|
|CHR14|8192|0|
|CHR15|8192|0|

No repeated gameplay: saved normal Spain/final-course driving and artificial qualifying/final-results/losing-ending/pit-refuel records reused; repair and winning variation unvisited. Fresh connected traversal yielded no new unknown candidates (434 visited, no budget exhaustion). Incomplete ordinary easy-gain handoff; remaining83365 bytes need next source survey/redirect. Commentary must review full fresh source; old complete review applies only to prior map. Two documented missing features persist: uploaded execution views and PRG-RAM debugger editing; see [toolkit issues](toolkit-issues.md). No toolkit edits.

## Coordinator transition — round 2 commentary active

Separate round2 disassembly worker complete. Coordinator inspected round2-disassembly-handoff.md and current progress; pass-002 accepted nine leads and rebuilt byte-identically. ROM classification: 68.20% (178795 / 262160 bytes classified; 83365 bytes unknown). This exact percentage was explicitly printed to the user before commentary began. Separate round2_commentary worker started with full current-source readability gate, preserving prior commentary/review/symbol history and reusing valid credit. Final editorial audit remains pending; redirect will follow after readability gate. No terminal exhaustion assessment or archive execution.

## Round 2 commentary checkpoint — pass-002

Full current classified-source readability complete:420 Code intervals/11669 bytes,118 Data intervals/36054 bytes, one CHR Image interval/131072 bytes and139 Unknown boundary notes.678 review records reviewed,626 meaningful units,490 explicit code comments;482 unchanged physical intervals/kinds retain valid prior credit. Complete historical commentary preserved at round2/commentary/prior-commentary; fresh baseline is pass-002. Final editorial audit remains pending.

Source includes all51 audio command entry contracts, setup editors, PPU descriptor/grids/text formats, timed-text arrays, five renderer/component variants, trial/GP/pit/results control flow and full local upload-source/RAM-offset mechanics. Prior stale dependencies corrected. Source SHA256 c4bfe7cc843970b9ed4ffec887a83dbb429720f5abfd441466385ce83ee52b6f. Fresh commentary/verification.json byte_identical=true; reading-after.asm refreshed. Scope: round2/commentary/scope-result.json; compact handoff: round2-commentary-handoff.md. No remaining classified readability ranges. Classification68.20% (178795/262160),83365unknown, map/state/ROM unchanged. Next source-derived questions are in commentary/uncertainties.md; no exhaustion or final acceptance claim.

## Coordinator transition — round 2 redirect active

Separate round2 commentary worker completed. Coordinator inspected round2-commentary-handoff.md and scope-result.json; current ROM/map/state/source hashes match, verification is byte-identical and all 678 review records are reviewed. Full current-source readability gate accepted; final editorial audit pending. Separate round2_redirect worker started from pass-002 and compact handoff with finite table and mapper-return avenues. Classification remains 68.20% (178795 / 262160 bytes classified; 83365 bytes unknown). No terminal exhaustion assessment; no archive or cleanup.

## Round 2 redirect active — r2-structures
Input pass-002/readability complete, map/state unchanged178795classified/83365unknown. Unique durable frontier/findings in redirect/r2-structures/. Connected roadside/rival projection reveals bank6 coordinate64-byte vectors/four34-byte pointer planes, two fixed256-byte projection planes, rival dispatch/renderers and finite sprite components. Accepted geometry prefixes yield exact curve-add200/elevation-subtract62-byte continuations. Explicit menu caller$87ac and bounded preset/label data under investigation next. No classification/source/toolkit edits or gameplay replay; this is active research, not a terminal assessment.

r2-structures coherent ready batch: six Static leads propose33284uniqueunknownbytes (11142Code/22142Data),0accepted. Connected rival/coordinate/renderers, full menu/GP/car-edit/pit/fuel, reset/road-init mapper continuations and independent course PPU compression/labels inspected.17course/demo streams each expand960tiles with independent matching attr/palette endpoints. Exact candidate boundaries/views, audits,152bank-split currentunknownledger and localized deferrals saved under redirect/r2-structures/. Classification remains178795/262160=68.20%,83365unknown; no map/source changes or terminal-exhaustion claim. Preparing compact ready handoff; existing representative gameplay reused.

## Round 2 redirect ready handoff — r2-structures
Six ready leads persisted,33284uniqueunknownbytes proposed (11142Code/22142Data),0accepted. Detailed primary evidence, exact candidate views/ranges, physical/branch-boundary audits and152bank-split unknownledger are in redirect/r2-structures/. Compact handoff: round2-redirect-handoff.md. All connected menu/GP/car/pit/fuel/animation, rival/road geometry and reset/road-init mapper continuations inspected; independent course compression, labels/presets and clear data fallback included. No unsupported direct targets, no overlaps; ROM/map/state/source identities unchanged. Classification68.20%,83365unknown. No terminal-exhaustion/editorial acceptance claim; proceed independent disassembly review, full current-source commentary, then redirect. Existing gameplay reused; two toolkit feature limitations unchanged, no new issues. No live processes retained; disk9.0GiBfree, attempt9.2MiB/scratch52KiB.

## Coordinator transition — round 3 disassembly active

Round2 redirect complete. Coordinator inspected round2-redirect-handoff.md, proposal-summary, scope/primary-artifact links and localized deferral reasoning. Six ready leads combine connected initialization/menu/pit/rendering and independent course-graphics/data avenues; 33284 unique unknown bytes proposed, 0 accepted. Concrete recipes and explicit mapper-view splits warrant next cycle. Separate round3_disassembly worker started from pass-002; completed r2-structures remains immutable, no competing writer. Classification remains68.20% (178795 /262160 bytes classified;83365 unknown). This is productive continuation, not terminal exhaustion; final audit/document selection remain pending.

## Round 3 disassembly — pass-003

ROM classification: 80.90% (212079 / 262160 bytes classified; 50081 bytes unknown).

Accepted six ready leads: +33284 unique unknown bytes (11142 Code,22142 Data). Independent review includes5167 fresh decodes,86 context-preserving code groups, explicit mapper-return views and17 compressed-course endpoint proofs. Prior classifications/provenance/views all preserved exactly. Structural claims are Static; no new observed execution. Evidence: [review](round3/review.md), [annotations](round3/annotations.json), [decodes](round3/accepted-decodes.json), [course audit](round3/course-audit.json), [exact counts/residuals](round3/counts.json).

|Bank|Code|Data|Image|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|Header|0|16|0|16|0|
|PRG00|0|884|0|884|7308|
|PRG01|0|296|0|296|7896|
|PRG02|0|8192|0|8192|0|
|PRG03|0|4721|0|4721|3471|
|PRG04|0|8192|0|8192|0|
|PRG05|0|8167|0|8167|25|
|PRG06|0|6936|0|6936|1256|
|PRG07|0|7417|0|7417|775|
|PRG08|296|0|0|296|7896|
|PRG09|0|2256|0|2256|5936|
|PRG10|2176|413|0|2589|5603|
|PRG11|0|2000|0|2000|6192|
|PRG12|6016|1981|0|7997|195|
|PRG13|4011|4050|0|8061|131|
|PRG14|6353|52|0|6405|1787|
|PRG15|3959|2623|0|6582|1610|
|CHR00|0|0|8192|8192|0|
|CHR01|0|0|8192|8192|0|
|CHR02|0|0|8192|8192|0|
|CHR03|0|0|8192|8192|0|
|CHR04|0|0|8192|8192|0|
|CHR05|0|0|8192|8192|0|
|CHR06|0|0|8192|8192|0|
|CHR07|0|0|8192|8192|0|
|CHR08|0|0|8192|8192|0|
|CHR09|0|0|8192|8192|0|
|CHR10|0|0|8192|8192|0|
|CHR11|0|0|8192|8192|0|
|CHR12|0|0|8192|8192|0|
|CHR13|0|0|8192|8192|0|
|CHR14|0|0|8192|8192|0|
|CHR15|0|0|8192|8192|0|

Map/state validated and saved; [pass-003 verification](pass-003-verification.json) confirms byte-identical rebuild in generated/pass-003. Map SHA25660b360155365d7207d52ddb82c33b8d5058fdd8f4f61b172cc28648326f77f35. All241 new regions have evidence/comment annotations;678 historical readability-review records/source/comments remain untouched and need rebase plus full fresh-source review. Current commentary credit is for pass-002 only; final editorial gate remains pending.

Fresh connected traversal visited757 candidates, no newly unknown candidates, no budget exhaustion. Two indirect diagnostics122487/122843 correspond to the reviewed finite rival state/render tables; native traversal does not enumerate these automatically. Saved representative natural/artificial gameplay and repair attempt reused; no repeated broad replay, new execution or winning/natural-completion claim. Remaining50081bytes and localized entry-less fragments remain unknown. Next step is full current-source commentary, then source-derived redirect; no terminal exhaustion claim. Two documented missing toolkit features persist (uploaded execution views and PRG-RAM editing), no new issues; [report](toolkit-issues.md). No toolkit/ROM/source edits, cleanup or publication.

## Coordinator transition — round 3 commentary active

Round3 disassembly worker complete. Coordinator inspected round3-disassembly-handoff.md; pass-003 accepted all six leads, +33284 bytes, all prior539regions preserved. ROM classification:80.90% (212079 /262160 bytes classified;50081 unknown), explicitly printed to user before commentary. Fresh checkpoint byte-identical; bounded connected check found no new unknown candidates. Separate round3_commentary worker started with full current-source readability gate and prior-history preservation/review-credit reuse. No terminal exhaustion assessment; final editorial audit and document selection remain pending.

## Round 3 commentary checkpoint — pass-003

Full current classified-source readability complete.893 full-ROM records reviewed:506 Code intervals/22811 bytes,273 Data/58196 bytes, one CHR Image/131072 bytes and113 Unknown boundary notes/50081 bytes.935 meaningful units;178 new explicit mechanical code notes.626 exact-range/kind records preserve valid credit, including all539 prior classified intervals. All241 additions reviewed. Complete prior commentary/history preserved at round3/commentary/prior-commentary/. Final editorial audit remains pending.

New source covers reset/save/audio, menu/player/name/color/result editors, car budget, pit/24-state service/clock/OAM, explicit mapper PPU transfers, curve/elevation, all rival state/render families and finite coordinate/projection/course/menu/text/sprite structures. Nine stale dependencies corrected, misleading rival names repaired, old uniform coordinate ranges explained by consumers. Uploaded source remains Static Data with its local RAM algorithms preserved. Unknown semantics retain local uncertainty.

Fresh verification byte_identical=true; reading-after.asm refreshed. Source SHA256109a35b9e1ab91006b99d2fe5336b3fcf936a700b60ce5bb5f8508093e8586a9. Scope round3/commentary/scope-result.json; compact handoff round3-commentary-handoff.md. No classified readability gaps. Classification80.90% (212079/262160),50081 Unknown; map/state/ROM unchanged. No final audit, terminal exhaustion, toolkit edits, redirect/disassembly actions, archive/cleanup/publication or gameplay replay.

## Coordinator transition — round 3 redirect active

Round3 commentary complete. Coordinator inspected compact handoff/scope, checked current ROM/map/state/source hashes, byte-identical verification and all893 reviewed records; no classified readability gaps remain. Final editorial audit pending. Separate round3_redirect worker started from pass-003 with current113 unknown intervals and source-derived results/ending/audio/road questions. Classification80.90% (212079 /262160 bytes classified;50081 unknown) unchanged. Completed earlier research remains immutable; no terminal exhaustion assessment/archive/cleanup.

## Round3 redirect research checkpoint — r3-residuals active

Unique attempt/frontier and113interval/50081byte contextual ledger saved. Current-source direct bank4 result calls unlock3257candidatecodebytes in1603instruction connected graph; all selected results/ending paths inspected through$90A1. Accepted-instruction/view scan adds fixed GP/race/pit/preview/text consumers (64unknown successor edges;968contextual872uniqueunknowninstructionstarts). Explicit mapper boundaries and transitive return contexts are being corrected before any proposal. Lower packed-profile/numeric, tile/text/sprite and audio descriptor/stream residuals have fresh bounded pattern artifacts; consumer/boundary review continues. No accepted annotation, runtime replay or final/exhaustion claim. See redirect/r3-residuals/frontier.md and findings.md.

Round3 redirect r3-residuals exact proposal checkpoint:6117Code+42262Data=48379uniqueunknownbytes proposed,0accepted, no code/dataoverlap/outsideunknown. Finite geometry32pages/eightsamplevectors, fourpreview960tile+64attributerecords, resultnine-recordcoursebaselines and audio110descriptors/compatibilitybitsets recovered. MapperreturnsincludingC94Cclosed; nineacceptedtransferpostcallcontextscheckedandconsistent (accepted-transfer-context-audit.json). Newstaticentrycontracts revise missingcaller-onlydeferrals withfiniteOAM/stack/helperproof; orphanPLAand2byteoverwritefragmentremainambiguous. Currentproposalresidual1702bytes56intervalsunderboundedprimaryinspection; notterminalexhaustion. Classificationandreadabilitycheckpointunchanged.

## Round3 redirect ready handoff — r3-residuals

Six ready leads persisted with all prior history preserved.50076uniqueunknownbytes proposed (6884Code/43192Data),0accepted; acceptedclassification80.90%,50081Unknownunchanged. Fresh3193instruction/view checks109groups, no decode discrepancy/codeDataoverlap/outsideunknown/unsupported directtarget. ROM/map/state/sourcehashesunchanged; exact113interval50081bytemapcomplement rechecked and evidence-linked dispositions saved. Finitegeometry/preview/results/audio/fixedinline structures plus complete connectedresults/race/pit/preview/text and independententrycontracts packaged. Narrow9D72 proposal uses exactmodulo-byte arithmetic/stackcontract with coordinatorcounterexample preserved, no generaldecimalcorrectness promise. Fivebytes remainfourrangesifaccepted; primary localized rejectionreview saved. Ordinaryreadybatch, no terminalexhaustion/finaleditorialclaim. Handoff round3-redirect-handoff.md; primary annotation-shaped proposed-code-instructions.json and proposed-data.json under redirect/r3-residuals. No liveprocess, no map/source/toolkit/ROMchanges, no gameplayreplay/cleanup/publication; disk9GiBfree.

## Coordinator transition — round 4 disassembly active

Round3 redirect complete. Coordinator inspected ready handoff/proposal summary and challenged BCD arithmetic contract:99hex+65hex then decimal correction wraps04, so exact modulo mechanics replace overbroad arithmetic claim. Correction/counterexample preserved for independent review. Six ready leads propose50076bytes (6884Code/43192Data),0accepted,5residualifaccepted. Connected/independent investigations and localized uncertainty justify next cycle, not terminal stop. Separate round4_disassembly started from pass-003; must independently inspect broad generic containers and narrow33byte stack-entry claim. Prior completed attempts immutable; classification80.90%,50081unknown unchanged. Final commentary/redirect/audit/document selection remain pending.

## Round 4 disassembly — pass-004

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

**Incomplete:** the rounded100.00% is not full classification; four ranges retain5Unknown bytes. Accepted all six independently reviewed ready R3 leads, +50076 unique bytes (6884 Code,43192 Data) in367 new regions. All earlier regions/labels/comments/evidence/provenance/views preserved exactly. No new Observed execution or natural reachability claim. Primary independent evidence: [review](round4/review.md), [fresh decode audit](round4/audit.json), [finite formats](round4/format-audit.json), [data references](round4/data-reference-audit.json), [positive pattern rows](round4/generic-chunks.txt), [accepted annotations](round4/annotations.json), [counts/residuals](round4/counts.json).

109 context-preserving groups/3193 instruction views freshly decoded. Independent finite consumers, exact stream endpoints, branch/stack/sibling contracts and broad positive Data patterns support classification. The33-byte9D72 body is the exact stack-argument modulo correction/clamp; 99+65=FE then+6 wraps04, so no general saturating-decimal-add correctness claim. Four narrow residuals are isolated00, overwrittenA51B, isolatedRTS and isolatedFF; [localized rejection review](round4/review.md) retains genuine structural ambiguity without requiring live callers for complete static routines.

|Bank|Code|Data|Image|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|Header|0|16|0|16|0|
|PRG00|0|8192|0|8192|0|
|PRG01|19|8173|0|8192|0|
|PRG02|0|8192|0|8192|0|
|PRG03|19|8173|0|8192|0|
|PRG04|0|8192|0|8192|0|
|PRG05|19|8173|0|8192|0|
|PRG06|0|8192|0|8192|0|
|PRG07|19|8173|0|8192|0|
|PRG08|4105|4087|0|8192|0|
|PRG09|19|8173|0|8192|0|
|PRG10|2194|5997|0|8191|1|
|PRG11|19|8173|0|8192|0|
|PRG12|6211|1981|0|8192|0|
|PRG13|4136|4056|0|8192|0|
|PRG14|7638|551|0|8189|3|
|PRG15|5297|2894|0|8191|1|
|CHR00|0|0|8192|8192|0|
|CHR01|0|0|8192|8192|0|
|CHR02|0|0|8192|8192|0|
|CHR03|0|0|8192|8192|0|
|CHR04|0|0|8192|8192|0|
|CHR05|0|0|8192|8192|0|
|CHR06|0|0|8192|8192|0|
|CHR07|0|0|8192|8192|0|
|CHR08|0|0|8192|8192|0|
|CHR09|0|0|8192|8192|0|
|CHR10|0|0|8192|8192|0|
|CHR11|0|0|8192|8192|0|
|CHR12|0|0|8192|8192|0|
|CHR13|0|0|8192|8192|0|
|CHR14|0|0|8192|8192|0|
|CHR15|0|0|8192|8192|0|

[pass-004 verification](pass-004-verification.json) confirms validated byte-identical rebuild in generated/pass-004. State/current/checkpoint map SHA2563a5de61434a4e0bb43aa7e3bc3ec2b46a4e88a2262b8c3c65ffd53c6928c7381. Connected traversal1165candidates, no newly unknown candidates, no budget exhaustion; sole diagnostic83315 is an already classified Data barrier, not a new unresolved entry.

Saved representative natural Spain/final-course and artificial qualifying/finalrace/results/losingending/pit-refuel plus repair attempt reused; scenarios progression checklist reviewed, no unchanged broad replay, new execution, winning or naturalchampionship-completion claim. The four residual byte-role questions do not gain evidence from unchanged gameplay replay.

All893 prior current-source review records and commentary/source/history preserved; their credit remains for pass-003. Full fresh pass-004 source commentary/rebase is pending for367 new structural regions plus verification of preserved context; [readability backlog](round4/readability-backlog.md). Final editorial gate remains pending. Next: commentary then redirect residual review; this is not terminal universal exhaustion. Two documented missing toolkit features persist (uploaded execution source views and PRG-RAM editing), no new issues; [report](toolkit-issues.md). No toolkit/binary/ROM changes, archive, delete, cleanup or publication.

## Coordinator transition — round 4 commentary active

Round4 disassembly complete. Coordinator inspected handoff: pass-004 validated/rebuilt byte-identically; all sixleadsaccepted,all780priorregionspreserved. ROM classification:100.00% (262155 /262160 bytes classified;5unknown). This exactroundedpercentage/count/incompletewarning explicitly printed beforecommentary. Four residualranges remain, no terminalclaim. Separate round4_commentary worker started fullcurrentreadability/rebase preserving priorhistory and narrowBCDdescription. Finaleditorialacceptance and residualredirect/rejectionreview/documentselection remain pending.

## Coordinator transition — round 4 redirect active

Round4 commentary complete. Coordinator inspected handoff/scope and verified current ROM/map/state/source hashes, reading-copy hash, byte-identical result and all1151reviewed records. Fullcurrentreadability gate accepted; finaleditorialauditpending. Separate round4_redirect worker started to reassess fourresidualranges/fivebytes under current-source/evidence-policy, with primary rejectionreview required before any terminalincompleteassessment. Rounded100.00% (262155 /262160 classified;5unknown) remains incomplete. Archiveoptions/inventoryinterim, mustrefresh afterfinaldocs. No terminalacceptance/archive/cleanup.

## Round 4 redirect — current-source residual assessment

TERMINAL-INCOMPLETE recommended for coordinator evidence-policy inspection. Pass-004 remains262155/262160classified,5Unknown in four ranges; rounded100.00% is incomplete. Unique attempt redirect/r4-current-residuals contains exact map complement/raw neighbors/bank contexts, numbered current source excerpts and localized rejection-review.md. Retrieved primary r3 decodes, checked named audio dispatcher, terminal loop, steering branches and split/ordinary renderer contracts. Corrected historical D8F7 predecessor to JMP and audio JMP target849F. No useful revised inference/test or actionable independent continuation;0proposed/accepted bytes, empty lead pass recorded, all prior histories preserved. Representative later/final-course/losing-ending/pit coverage reviewed; no unchanged replay. Full current readability1151records/1377units and byte-identical verification remain valid; final editorial gate pending. ROM/map/state/source identities unchanged; reading-copy separate hash recorded, about9GBfree. Two existing toolkit limitations, no new issues. No toolkit/binary/ROM/map/state/source edits, archive/delete/cleanup/publication. Handoff round4-redirect-handoff.md.

## Coordinator terminal reasoning accepted — final editorial active

Round4 redirect complete with terminal-incomplete recommendation. Coordinator read full rejection-review, all four numbered current-source excerpts, neighboring accepted annotations and independent rawROM bytes; recorded candidate-by-candidate reasoning in coordinator-rejection-review.md. AudioJMPtarget and residualbytecount transcription corrections incorporated. No overbroad negative prerequisite or concrete useful revised test remains identified; fivebytes/fourranges stay materially ambiguous. Stop structuralworkflow incomplete, not blocked or wholeROMcomplete; no empty researchcycle. Separate final_editorial worker started to repair known stale source14561 dependency, target remaining naming/comment issues with validcredit reuse, complete finalreview/audit/verification/reading refresh. Final goal remains active until editorial artifacts/currentfinalhandoff and archive-document inventory/options pass finalcompletion audit. No archive/cleanup/publication authorized.

## Final editorial acceptance — pass-004

Completed targeted final acceptance with valid1151record/1377unit review credit. Fixed three stale dependency comments (terminal continuation, pulse reload/phase, rival periodic sound) and six matching summaries; unchanged line counts preserve source references. Preserved pre-final authoritative artifacts under analysis/final-editorial/pre-final-authoritative. Supported commentary audit passed, byte-identical fresh verification/read refresh complete. Source SHA256 a878f56d875d48dc904e72f7cc3c4ad01f659a4b7e5e2dea25dd14d425e81d60; reading SHA256 ac304c013faf43e3009f0c246c85e44fdac0d1d2cf71bdb3dc8f17010f681e20. Current map/state independently validate and remain3a5de61434a4e0bb43aa7e3bc3ec2b46a4e88a2262b8c3c65ffd53c6928c7381. Classification262155/262160,5Unknown/four gaps, rounded100.00% INCOMPLETE; editorial complete is separate. No new toolkit issues; two existing limitations remain. No structural research, ROM/map/state/toolkit/binary edits, archive/delete/cleanup/publication. See commentary/editorial-audit.md and analysis/final-editorial-handoff.md. Coordinator to refresh archive options/inventory; no archive performed.

## Coordinator terminal handoff — justified incomplete, editorial accepted

Sequential coordinator workflow reached documented terminal incomplete after primary-evidence rejection review. ROM classification:100.00% (262155 /262160 classified;5unknown), incomplete despite rounding. Final editorial audit passed1151records/1377units, freshbyteidenticalverification/readingcopycurrent; sourcea878f56d875d48dc904e72f7cc3c4ad01f659a4b7e5e2dea25dd14d425e81d60, map/state/pass004unchanged3a5de61434a4e0bb43aa7e3bc3ec2b46a4e88a2262b8c3c65ffd53c6928c7381. Coordinator independently re-counted uniquephysicalcoverage/fourunknownranges, checked8finalidentityhashes and only3same-linecommentedits. Gameplaychecklist linked to existingroutes/evidence. Finalhandoff analysis/final-handoff.md; completionaudit coordinator-completion-audit.json. Archivecontract re-inspected and options/inventory refreshed for everycurrentMarkdown, preserving finalconclusions/handoffs/history and omittingonlyidenticalinstructioncopies. No unfinishedproductivephase or ready lead remains; no wholeROMcompletion claim. No archive/delete/cleanup/publication/toolkit/binary/ROMedit.
