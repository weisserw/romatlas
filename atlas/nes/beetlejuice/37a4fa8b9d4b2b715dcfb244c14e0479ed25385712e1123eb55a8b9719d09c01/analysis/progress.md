# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly — in progress
Configured release binary inspection and baseline succeeded; baseline byte identity is confirmed in analysis/baseline-verification.json. Image is mapper 7 AxROM, 131088 physical bytes including 16-byte header, 128 KiB PRG, CHR RAM. On-screen identification: Beetlejuice (Rare/LJN, 1990/1991). Current live server held by round1 worker; analysis/round1-rpc.jsonl preserves exact requests/results. Startup trace retained 429 contextual opcode roots; vector-driven discovery produced 449 candidate instructions and two indirect-jump frontiers. Classification inspection underway; baseline is not classification. Runtime stepping is bounded and slow; timeout preserves partial progress. No terminal determination yet.

### Startup batch saved
133 evidence-backed regions, 3565 code bytes classified; 127523 bytes currently unknown (header pending annotation). Broad startup/title/map transition accepted roots inspected in analysis/round1-startup-inspection.json. Static BRK instructions are legitimate bank-call opcodes here: IRQ handler $FFAC pulls stacked PC, reads two inline dispatch bytes, switches AxROM and calls $8000 indirect dispatcher. Do not decode inline dispatch bytes as code. Fresh verified checkpoint: maps/pass-001-startup.json, generated/pass-001-startup, analysis/pass-001-startup-verification.json. Current map validated and state saved. Normal route has reached first side-view room ($55=3), still exploring initialized gameplay and subsequent transitions. Next concrete frontier: inspect $55 room loader/transition contract to initialize representative later areas with RAM edits; all edits will be distinguished from natural traversal. Toolkit NES-001 documents required buttons-array input workaround.

### Gameplay frontier (same pass)
Corrected buttons-array API has enabled natural progression: title → showtime → world map → first side-view house room ($55=3), inventory opened/closed with Start, right+A+B produced visible player movement/jump. analysis/action2.png confirms active player. Durable analysis/active1.snapshot at frame1232, PC$8E12, stack$26, banks[12,13,14,15]. Earlier level1 snapshot captured a partially initialized/menu state and is not preferred. Currently testing room9 via restore active1, finish instruction, write RAM$55=9, set PC$8D23 and stack$26 (transition initializer supported by existing jumps; see bank-aware decode in RPC log). Inspect actual frame/fetches before claiming visit; artificial reachability only. No lead IDs existed at start. Compact later-state checklist and outcomes will be saved before phase handoff.

## Round 1 disassembly completed — incomplete ROM

ROM classification: 19.15% (25107 / 131088 bytes classified; 105981 bytes unknown).

| 8 KiB PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 2217 | 20 | 5955 |
| 1 | 2295 | 10 | 5887 |
| 2 | 1109 | 47 | 7036 |
| 3 | 189 | 1062 | 6941 |
| 4 | 1571 | 18 | 6603 |
| 5 | 0 | 143 | 8049 |
| 6 | 0 | 176 | 8016 |
| 7 | 200 | 209 | 7783 |
| 8 | 438 | 61 | 7693 |
| 9 | 139 | 44 | 8009 |
| 10 | 76 | 2 | 8114 |
| 11 | 1654 | 261 | 6277 |
| 12 | 3459 | 56 | 4677 |
| 13 | 2738 | 20 | 5434 |
| 14 | 3959 | 52 | 4181 |
| 15 | 2603 | 263 | 5326 |

Final state: 1068 regions; code22647, data2460. Header16 included above total, excluded from PRG table. Evidence/comments cover all classified regions; editorial review pending. Observed/static code bytes: 11468/11179. Full state saved; map validated; final checkpoint maps/pass-001-final.json generated and rebuilt byte-identically, report analysis/pass-001-final-verification.json. Map SHA256 b4cbd4690d6850d5155f0bc94dafcf48607ac1fab283289ff2d6c080043e9189.

Exact residuals: analysis/round1-counts.json. Handoff: analysis/round1-disassembly-handoff.md. Leads L-R1-ROOM attempted/not verified (three failed initializer variants; scheduler contract missing), L-R1-STATIC verified prefixes/open continuation contract, L-R1-GFX open compressed-payload consumer/bounds. Broad gameplay best effort reached natural first side-view system; later milestones attempted via supported-entry shortcuts but remained unvisited, explicitly recorded in scenarios.json. Do not claim gameplay exhaustion/ending visited. Separate redirect work is needed for scheduler prerequisite. Toolkit summary: one confirmed documentation/API mismatch, zero suspected bugs, zero missing features, no blockers. See analysis/toolkit-issues.md. Input unchanged; all artifacts local.

## Round 1 commentary — active
Input pass-001-final, map SHA256 b4cbd4690d6850d5155f0bc94dafcf48607ac1fab283289ff2d6c080043e9189. No prior commentary directory exists; preparing frozen baseline and full current-source review. Classification unchanged.

## Round 1 commentary completed — current-source readability gate met
All currently classified source and honest unknown-boundary notes reviewed: 1,437 inventory entries, 944 Code/124 Data/369 Unknown, 950 code-fragment units and 232 mechanism groups. No remaining naming/comment backlog. All 25,107 classified bytes covered; whole-ROM classification remains 19.15%, 105,981 bytes unknown. Map/state and pass-001-final checkpoint unchanged. Source/reading copy and compact handoff: commentary/source/game.asm, commentary/reading-after.asm, analysis/round1-commentary-handoff.md. Final fresh byte-identical verification: commentary/verification.json; source SHA256 82974701ab5a7bef74fffdc36b920ea20caaba5a41b9274d9802a53b7342ee12. Frozen identities and valid relative review locators checked; final editorial/archive audit remains pending. L-R1-ROOM/STATIC/GFX and new consumer questions retained in commentary/uncertainties.md. Toolkit unchanged (NES-001 existing confirmed documentation/API mismatch, no new issues/blockers). No live writer/server; ROM/toolkit preserved, all artifacts local.

## Round 1 redirect — active
Attempt analysis/redirect/r1-20261119-a; input pass-001-final, unchanged map SHA b4cbd4690d6850d5155f0bc94dafcf48607ac1fab283289ff2d6c080043e9189. Read redirect prompt/evidence policy and both handoffs. Current method: independent finite-output graphics decode with exact input consumption; then corrected transition continuation, return contracts and finite map/sprite consumers. Research only; no map/state edits.

## Round 1 redirect completed — combined supported batch ready

Attempt analysis/redirect/r1-20261119-a, input pass-001-final. Six ready leads in analysis/leads.json propose 57,246 currently unknown bytes across 303 exact candidates; no classification accepted in redirect. Existing classification remains 25,107/131,088 (19.15%), unknown105,981. Candidate/accepted and internal Code/Data conflict checks both zero. All369 residual ranges retain exact counts, context, neighbors and evidence-linked dispositions in the attempt ledger. Corrected room scheduler reached overhead9/18 and low1/2 with movement/action evidence; victory dialogue24 rendered/completed artificially, natural terminal/credits unproven. Full methods/corrections and remaining prerequisites: analysis/round1-redirect-handoff.md and attempt/report.md. ROM/map/state/current-source hashes freshly unchanged. All probe sessions/servers finished, no live writer. Toolkit preserved, all work local, no cleanup/archive/publication. Independent disassembly should validate this coherent batch, then full current-source commentary and redirect.

## Round 2 disassembly — active
Input pass-001-final; state/map SHA256 b4cbd4690d6850d5155f0bc94dafcf48607ac1fab283289ff2d6c080043e9189. Exactly one pass resumed without baseline initialization. Independently reviewing six ready leads against primary consumers, exact finite formats and bank/return contracts. Durable review: analysis/disassembly/r2-20261119-a. No proposal has yet been promoted. Prior source/readability and all evidence retained; graphics selector26 overlap, BRK operand barriers and narrowed metatile boundaries explicitly retained.

### Round 2 independent review batch
Fresh bank-aware decode plus independent pure-ROM validators accepted the narrowed structural scope of all303redirect candidates (57,246 unknown-byte union). Evidence: analysis/disassembly/r2-20261119-a/review.md, validation.json, validate.py, decode-review.json. Independent bit/RLE outputs, finite map runs/ID domains, sprite/display/text grammars and exact code/BRK boundaries pass. Read-view overlaps remain explicit; no original serializer or natural selection claim. No annotation yet; proceeding to bounded connected table gains then compatible unknown-only map annotations. Broad gameplay reuses actual rooms1/2/9/18 and victory dialogue; natural terminal identities/gates remain unresolved, so repetitive routes are not rerun.


## Round 2 disassembly completed — incomplete ROM

ROM classification: 62.87% (82416 / 131088 bytes classified; 48672 bytes unknown).

One pass accepted57,309newbytes:616Code and56,693Data, including63connected low-room table entries beyond the redirect batch. Total23,263Code/59,153Data,1,328regions. All new provenance Static; prior Observed bytes unaffected. Unique-byte counts and exact residual complement: analysis/round2-counts.json.

| 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 2217 | 20 | 2237 | 5955 |
| 1 | 2295 | 10 | 2305 | 5887 |
| 2 | 1109 | 47 | 1156 | 7036 |
| 3 | 189 | 7968 | 8157 | 35 |
| 4 | 1593 | 3700 | 5293 | 2899 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 200 | 6631 | 6831 | 1361 |
| 8 | 438 | 7180 | 7618 | 574 |
| 9 | 231 | 3041 | 3272 | 4920 |
| 10 | 316 | 7528 | 7844 | 348 |
| 11 | 1661 | 4566 | 6227 | 1965 |
| 12 | 3536 | 58 | 3594 | 4598 |
| 13 | 2846 | 501 | 3347 | 4845 |
| 14 | 4028 | 425 | 4453 | 3739 |
| 15 | 2604 | 1078 | 3682 | 4510 |

Header16Data is included in total and excluded from PRG table. Fresh map.validate/save/export completed and authoritative checkpoint **pass-002-reviewed** rebuilds byte-identically; analysis/pass-002-reviewed-verification.json. Map/state SHA256377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78. Original ROM and prior commentary source unchanged (sourceSHA82974701ab5a7bef74fffdc36b920ea20caaba5a41b9274d9802a53b7342ee12). Superseded pass-002-final holds unchanged input due atomically rejected first request; corrected API schema before accepted batch, retained history.

Independent structural evidence: analysis/disassembly/r2-20261119-a/review.md, validation.json, decode-review.json, accepted-candidates.json and annotation-batch.json. Eleven independent validators plus primary code/control/stack/bank inspection support all303narrowed redirect candidates; no blanket decode promotion.63extra bytes are9finite low-room1..7source/attribute/metatile/property/marker table slices. All six ready leads verified in leads.json; detailed outcomes in disassembly/r2-20261119-a/lead-outcomes.json. Overlapping graphics/metatile/sprite read views do not assert original serialized objects. BRK2operand barriers retained; unresolved heterogeneous external graphics tails remain unknown.

Gameplay review reused supported rooms1/2/9/18with actual movement/actions and artificial victory dialogue completion; natural terminal/credits and named later-area identities remain unproven in scenarios.json. No useful changed-condition experiment exists for those missing gate/identity contracts in this pass; no repetitive startup or arbitrary room enumeration performed. Next evidence-informed redirect can investigate actual gate/consumer/entry contracts for residuals. This is easy-gain handoff, not terminal exhaustion or whole-ROM classification completion.

Toolkit: existing NES-001documentation/API mismatch remains in analysis/toolkit-issues.md; no new issues, no edits. Data-view client request corrected without state mutation, not a toolkit defect. Prior source review credit retained; next commentary covers new ranges and corrects spriteFFprefix/80terminator and text1return/2..7Fdictionary descriptions. All servers finished; no active writer. About4.4GBfree, no cleanup/delete/archive/publication. Compact handoff: analysis/round2-disassembly-handoff.md.

## Round 2 commentary — active
Input pass-002-reviewed, map/state SHA377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78. Prior complete commentary preserved in analysis/round-2/commentary-prior before incremental rebase. Preserve reviewed unchanged source/symbols; pending all newly classified ranges and affected sprite/text descriptions. Remain in this phase until full current-source readability and fresh identity checks pass.

## Round 2 completed — current-source readability gate met

Input pass-002-reviewed, map/state SHA377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78. The full prior commentary phase (source, review, before, reading and verification/history) is preserved at analysis/round-2/commentary-prior; round1 source SHA82974701ab5a7bef74fffdc36b920ea20caaba5a41b9274d9802a53b7342ee12 is intact. Current before/map/state and reading-before now correspond to pass-002-reviewed.

All 1,680 current inventory ranges reviewed: 970 Code, 358 Data, 352 honest Unknown ranges. 976 reviewed Code units cover every Code region, with bank context and refreshed relative game.asm:line locators. Reused 1,068 unchanged classified range reviews; examined and explained all 260 new classified spans (616 Code/56,693 Data bytes), plus affected consumer descriptions. Remaining readability ranges: none; exact scope and pending[] are in analysis/commentary-round2/. Whole-ROM classification remains 82,416/131,088 = 62.87%; 48,672 unknown bytes remain.

Names and prose now explain finite graphics source/end/output constraints, overlapping read views, map/property/metatile domains, attribute expansion, display/text grammars, sprite descriptor fields and clipping, BRK operands and bank/stack return contracts, task0/task4 initialization and matched transition routing. Established table fields, scratch roles and new branch paths have symbols in operands. Corrected existing sprite FF flipped-prefix/80 termination and text1 return/unwind versus2..7F dictionary prose and affected units; removed stale extent/continuation claims. Player-null slots14/15 are scoped to the player table. Original object packing, exact actor/item/scene identities, wider room/selector domains and natural terminal gates remain honestly unresolved; see uncertainties.md. No naming/comment backlog is deferred to redirect.

Fresh source verification byte-identical and reading-after refreshed; commentary/verification.json. Final source SHAaf64079029970ec793f30ffd8b7a7f452e7aba5996f4aec96bdac4a9e26e373a. Relative locators, full scaffold, coverage counts, original ROM/map/state identities and preserved historical source all checked. Final editorial acceptance audit remains pending and was not run. No new runtime/disassembly/redirect work, toolkit edits or issues; existing NES-001 remains nonblocking. No live writer/server, all artifacts local.

## Round 2 redirect active — r2-20261119-b
Input pass-002-reviewed; unchanged map/state SHA377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78. Exact residual48672bytes/352ranges. Research audio command/effect/song consumers and independent actor dispatch/local pattern subclaims. Frontier: analysis/redirect/r2-20261119-b/frontier.md. No map/source/ROM mutation.

## Round2 redirect completed — r2-20261119-b supported batch
Inputpass-002-reviewed/mapstateSHA377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78 unchanged. Acceptedclassification82416/131088=62.87%,unknown48672;566researchcandidates29394uniqueunknownbytes(17813Code/11581Data),accepted0. FourreadyIDsL-R2-AUDIO/ACTOR/PATTERN/FLOW appended withrecipes/history; prior6verifiedIDsretained. Exact352rangeledger,kind/perbankcounts,unchangedROM/sourceidentitiesandcandidatechecks inanalysis/redirect/r2-20261119-b/final-audit.jsonandunknown-ledger.json. See report.mdforpostcall/return/bank/SP/inlineproofs,typedbiased/nullselectorlimits,genericDatafallbacksandnegativepremisecorrections; graphnode/officialdecodechecksarenotacceptancealone. Fresh376Codespanboundarychecks pass;35connectedframegrammarszeroerrors. Actualartificialroom7gate->curtain/autowalk/dance->recognizedcyclecounter7->fade->actualscript24->FDC3/titleatframe1877 confirmed; exactpatches/snapshots/fetchesandsuccess/failedpose-onlytests preserved inreport/scenarios. No scriptpointer/gatePCedit; fullnaturalplay/creditsunclaimed. Otheropaqueconsumers/graphics tailsretainlocalizedprerequisites,no terminalexhaustionclaim. Currentfullsourcecommentarygatewasmetbeforephase; finaleditorialauditpending. Independentdisassemblynext,thenfullcommentary(withquarterconstantbranchanddamage-rootmechanicalcorrections). No ROM/map/state/source/toolkitedit,cleanup/deletion/archive/publication; NES-001existingonly. Allserversfinished/noactivewriter.

## Round 3 disassembly active — r3-20261119-a
Input pass-002-reviewed; map/state SHA377fd647a853708853b65861cdd40264cc36df0932078f8bd711cc3330d59b78. Same single pass; baseline retained. Frontier analysis/disassembly/r3-20261119-a/frontier.json records active phase and validation batches. Independent review combines physical ROM bytes and fresh configured-binary decode with actual source-linked roots, ordinary conditional-return versus reset/tail contracts, selected-handler domains, shared FFCD bank restore, BRK2 barriers, synthesized RAM stub and saved-SP data-stack checks. Data review independently checks finite consumer unions and endpoints; generic pitch/pattern tails retain narrower non-code claims. No map mutation yet. Remaining: acceptance report, unknown-only annotations, validate/save/export/new checkpoint/rebuild, exact counts and handoff. Current full prior commentary remains unchanged.

Round3 validation batch complete: fresh configured-binary Code boundaries376spans/8268instructions; all49source references checked against unchanged source and ROM, full102source/selected root list retained,203supportedBRKprofiles inspected with separate2byte inline barriers. Independent finite actor112x6 partition,55frame views,34effects,9445music states/8030readbytes and20pattern-record74byte union pass. Structural acceptance additionally rests on review.md contract analysis, not these mechanical checks alone. Connected inspection found316furtherData bytes: full256entry nibble-swap permutation, four8byte known mapper lookup constants and28byte literal ASCII author credit; no natural selector/display claim. Annotation still pending; exact candidates/validation in analysis/disassembly/r3-20261119-a/.


## Round 3 disassembly completed — incomplete ROM

ROM classification: 85.53% (112126 / 131088 bytes classified; 18962 bytes unknown).

One pass accepted29,710newunique bytes:17,813Code and11,897Data (29,394redirect union plus316connectedData). Internal map totals41,076Code/71,050Data across1,823regions. All new provenance Static; every previous accepted region record and prior full commentary source preserved byte-for-byte. Exact counts/complement163ranges: analysis/round3-counts.json.

| 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3745 | 925 | 4670 | 3522 |
| 1 | 4235 | 3459 | 7694 | 498 |
| 2 | 1742 | 5596 | 7338 | 854 |
| 3 | 189 | 7976 | 8165 | 27 |
| 4 | 2166 | 4045 | 6211 | 1981 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 229 | 6639 | 6868 | 1324 |
| 8 | 697 | 7436 | 8133 | 59 |
| 9 | 343 | 3041 | 3384 | 4808 |
| 10 | 316 | 7528 | 7844 | 348 |
| 11 | 2791 | 4574 | 7365 | 827 |
| 12 | 6305 | 318 | 6623 | 1569 |
| 13 | 6625 | 649 | 7274 | 918 |
| 14 | 7094 | 618 | 7712 | 480 |
| 15 | 4599 | 1846 | 6445 | 1747 |

Header16Data is included in the total and excluded from the PRG table. Fresh map.validate/save/export and checkpoint **pass-003-reviewed** succeeded; analysis/pass-003-reviewed-verification.json confirms byte-identical rebuilding. Map/stateSHAe71e545e01a1437d4ffbff9ffaef3b80ea2e5e94fe30d1ce7a39f59dbf217ae7; ROM remains37a4fa8b9d4b2b715dcfb244c14e0479ed25385712e1123eb55a8b9719d09c01. Commentary source remainsaf64079029970ec793f30ffd8b7a7f452e7aba5996f4aec96bdac4a9e26e373a; next commentary must preserve/rebase full prior review while explaining new ranges.

Independent structural review: analysis/disassembly/r3-20261119-a/review.md; validation.json, contract-validation.json, decode-review.json, annotation-batch.json and lead-outcomes.json. All four ready IDs L-R2-AUDIO/ACTOR/PATTERN/FLOW verified with exact finite/entry/return/bank/inline/stack contracts.49source references/102roots/203BRKprofiles checked against source/ROM;376Code spans/8268freshinstructions; independent112x6actor partition,55descriptor views,34effects,9445music states/8030readbytes and20pattern records. Specific versus generic Data distinctions retained, including8750tail and888Dprefix; failed pure11bit or first-quarter premises are localized. Additional256byte nibble permutation/four8byte mapper lookup tables/28byte ASCII credit classified with bounded role/context review.

Gameplay checklist reviewed: existing artificial rooms1/2/9/18action and actual room7gate->curtain/autowalk/dance->fade->script24->FDC3/title evidence reused. Named later-area/boss identity and natural full-completion/separate credits need an actual source-backed transition/identity contract; no useful changed recipe was available in this batch. No replay of resolved routes or arbitrary room sweep. This is easy-gain handoff, not terminal exhaustion. Residuals include selected-handler wider domains, saved-PC coroutine context, unselected assets and heterogeneous graphics tails; upcoming readable-source review may expose concrete consumers.

Existing NES-001documentation/API mismatch remains in analysis/toolkit-issues.md; no new toolkit issues or edits. First annotation client attempt incorrectly included execution views on Data; rejected before save/export, transient session discarded, corrected Code-only views with Data read contexts in evidence. This was not a second pass or toolkit defect. No deletion/cleanup/archive/publication. All servers finished/no active writer. Compact handoff: analysis/round3-disassembly-handoff.md.

## Round 3 commentary active
Full previous commentary preserved at analysis/round-3/commentary-prior. Input pass-003-reviewed. Rebased 1986 inventory ranges; 1328 unchanged classified reviews carried; 495 new classified ranges pending. Exact same-phase frontier: analysis/commentary-round3/frontier.json. No readability completion claimed.

## Round 3 commentary completed — current-source readability gate met
Input pass-003-reviewed; map/state SHAe71e545e01a1437d4ffbff9ffaef3b80ea2e5e94fe30d1ce7a39f59dbf217ae7 unchanged. Full prior commentary preserved at analysis/round-3/commentary-prior; prior source SHAaf64079029970ec793f30ffd8b7a7f452e7aba5996f4aec96bdac4a9e26e373a intact. All1986ranges reviewed(1322Code/501Data/163Unknown);1328unchanged classified reviews carried; all495new canonical spans(352Code/143Data) explained.1383reviewedCodeunits cover all41076Codebytes; relative source locators valid; pending[]. Whole ROM remains112126/131088=85.53%,unknown18962. Corrected quarter-copy BEQ and D6B1damage root, all41audio command grammars, field/table/control names, inner merged routines and obsolete boundary/continuation descriptions. New direct nibble-swap consumers bank2EC5B/ED22/ED8B yield a bounded source/width/property-union redirect method; see commentary/uncertainties.md. Actual artificial ending-to-title route/provenance retained without replay; natural completion/separate credits unclaimed. Fresh byte-identical verify and reading-after refresh passed, source SHA5215e633bf60f026a995bf7c10e39f8c341e16f12ee20115dd8a476be40333c4. Scope analysis/commentary-round3/scope.json; handoff analysis/round3-commentary-handoff.md. Final editorial audit pending, not invoked. Existing NES-001 only; no new toolkit issue/edit or blocker. All local; no ROM/map/state edits, cleanup/deletion/archive/publication. No active writer/server.

Round3 redirect active: r3-20261119-c, input pass-003-reviewed; current-source commentary gate independently passed. Exact unchanged classified112126/131088=85.53%, unknown18962/163ranges. Research only; current frontier analysis/redirect/r3-20261119-c/frontier.md. Investigating generated uploader PCs/saved-SP endpoints, confirmed property reader unions and independent finite data consumers. No ready leads yet, no map/source/ROM/state edits or active server.

Round3 redirect complete — r3-20261119-c; input/output map checkpoint pass-003-reviewed unchanged. Accepted112126/131088=85.53%,18962unknown. SevenreadyIDs L-R3-UPLOAD/L-R3-PROPERTY/L-R3-DATA/L-R3-OBJECT/L-R3-SPRITE/L-R3-TABLE/L-R3-CODE;536proposedviews,18925uniqueinputunknownbytes=1107Code+17818Data,accepted0,conditionalresidual37. Physical/bank/context/kind/identityauditpassed;552freshinstructionboundarychecks/45Codeviews, structuralproofseparate. Newartificialroom29event→room7fullysettledvisit; exactpatches/fetch/state/imagesinscenariosandattempt. Source/ROM/maps/state/priorcommentary/configuredbinaryunchanged;allserversclosed,noarchive/cleanup/publication/toolkitedits. Handoffanalysis/round3-redirect-handoff.md;reportanalysis/redirect/r3-20261119-c/report.md;nextphaseindependentdisassemblyreviewthenfullcommentary. Productivebatch, notterminalexhaustionoracceptance.

## Round 4 disassembly active — r4-20261119-a
Input pass-003-reviewed (map/state e71e545e01a1437d4ffbff9ffaef3b80ea2e5e94fe30d1ce7a39f59dbf217ae7). Single pass independent review in analysis/disassembly/r4-20261119-a/review.md. All536 views accepted at their narrow structural scopes:18925 unique unknown bytes=1107Code+17818Data;37 remain unknown. Fresh45Code-view boundaries,364exact source refs,74full numeric/generic records,209sprite views,69object pointer slots,6nonnull motion lists and finite stream/stack/property contracts independently checked. Graphics-tail3bytes accepted only as local generic container storage after original r1 primary records and40/42global-end counterexamples. Annotation/checkpoint pending. Current commentary source/history remains untouched.


## Round 4 disassembly completed — incomplete ROM

ROM classification: 99.97% (131051 / 131088 bytes classified; 37 bytes unknown).

Single pass accepted18925newunique bytes=1107Code+17818Data from536research views,444unknown-only canonical annotations. Internal totals42183Code/88868Data across2267regions. All1823previous region records preserved exactly; allnew provenance Static. Exact13range/37byte complement, per-bank counts and identities: analysis/round4-counts.json.

| 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3775 | 4417 | 8192 | 0 |
| 1 | 4240 | 3952 | 8192 | 0 |
| 2 | 1742 | 6450 | 8192 | 0 |
| 3 | 212 | 7976 | 8188 | 4 |
| 4 | 2166 | 6026 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 250 | 7939 | 8189 | 3 |
| 8 | 697 | 7495 | 8192 | 0 |
| 9 | 343 | 7849 | 8192 | 0 |
| 10 | 316 | 7876 | 8192 | 0 |
| 11 | 3366 | 4822 | 8188 | 4 |
| 12 | 6573 | 1603 | 8176 | 16 |
| 13 | 6638 | 1548 | 8186 | 6 |
| 14 | 7105 | 1086 | 8191 | 1 |
| 15 | 4760 | 3429 | 8189 | 3 |

Header16Data included in total, outside PRG table. Fresh map.validate/save/export and checkpoint pass-004-reviewed verified byte-identical (analysis/pass-004-reviewed-verification.json). Map/state SHAa054cad533065eba0fb67f797975f39b4f6ada358f60518161f9d5fa16f94baa; ROM unchanged. Source/commentary/history untouched, source SHA5215e633bf60f026a995bf7c10e39f8c341e16f12ee20115dd8a476be40333c4. Current full-source readability now needs rebase/review for new classified spans; previous1986reviews/1383Codeunits preserved.

Independent acceptance analysis/disassembly/r4-20261119-a/review.md plus contract-validation.json:45Code views/552fresh instructions,364exact source references,74full numeric/generic records,209sprite views,69roomobjectslots,6motion slots, finite stream/property/stack/vector/wrapper contracts. Allseven L-R3ready IDs verified with attempt histories; no proposed interval rejected. Earlier invalid sprite interpretation, backward-timer premise, global encoded-end rule, consumed-tail-payload inference and broad natural-domain claims remain rejected at their documented scope. Three graphics-tail bytes accepted narrowly as generic local container storage after primaryr1 records.

Remaining37:16heterogeneous852E..853E,sixisolatedRTS,nineoldphysicalFFEBshadowJMPbytes,twoRTI,fourBC11..BC15. Exactresidual primary reasoning remains analysis/redirect/r3-20261119-c/residual-assessment.json. Not terminal exhaustion; no forced classification. Existing broad scenarios including room29event→settledroom7andending-to-title reused; no unchanged gameplay replay/new natural-reachability claim.

Only existing NES-001 in analysis/toolkit-issues.md; no new toolkit issue/edit. No cleanup/deletion/archive/publication/sourceoverwrite. All RPCserversclosed and checkpointprocessfinished. Nextphase fullcurrent-source commentary, including inherited12vs13and20vs21copy-count prose corrections; disassembly did not edit thatsource. Compact handoff analysis/round4-disassembly-handoff.md.

## Round 4 commentary active
Full prior phase preserved at analysis/round-4/commentary-prior. Incremental rebase retains 1823 unchanged classified reviews; all 46 new Code spans have specific routine/control/stack descriptions drafted. Exact remaining Data ranges: analysis/commentary-round4/pending.json. Same phase remains active; source verification, current locators, dependency/field symbols and reading refresh due. No readability handoff claimed.

Round4 commentary coherent quality update: all 444 new spans (46 Code,398 Data) now have mechanism/format/read-view explanations. Added inner uploader/alternate-pose mechanisms, bank-aware CPU aliases, one-based motion fields and scoped stack-pointer roles. Corrected inherited twelve/twenty-byte copy counts, support-record schema, connected uploader/wrapper contracts and stale structural-gap claims. Fresh byte-identical verification source7e80abdecd3440b44894bed59df7fa9e1adf3d99283068d5e1895c115a98fe1c. Same phase active for final metadata/fullscope checks and reading refresh; no handoff yet.

## Round 4 completed — current-source readability gate met

Input pass-004-reviewed; map/state SHA a054cad533065eba0fb67f797975f39b4f6ada358f60518161f9d5fa16f94baa. The entire prior commentary/source/review/before/reading/verification/history is preserved at analysis/round-4/commentary-prior; its source SHA 5215e633bf60f026a995bf7c10e39f8c341e16f12ee20115dd8a476be40333c4 is unchanged. Current baseline/state, before, source mapping and reading-before correspond to pass 4. All 1823 old classified map records remain exact.

All 2280 full-ROM scaffold entries reviewed: 1368 Code,899 Data,13 honest Unknown ranges. Retained sound review credit for all 1823 unchanged classified ranges and explained all 444 new canonical spans (46 Code,398 Data;1107 Code/17818 Data bytes). There are 2035 reviewed meaningful units, including1439 Code units covering all42183 Code bytes. The full current-source inventory and affected old dependencies were checked; exact relative game.asm:line locators and scaffold/range/unit coverage passed. No remaining readability/naming/comment backlog; pending.json is []. Final editorial acceptance remains pending and the audit command was not run.

Source explains generated column/row upload PC arithmetic and page-one data stacks, finite saved-PC graph and restored-SP returns; bank-select/service/restore preconditions; NMI register saves, BRK operand barriers and reset mapper-store backing changes; executable null-target ROM slots versus unresolved RAM zero; property offset/record bounds versus generic map/reference storage; object/motion field layouts and exact room-list sources/counts/ends; palette selectors/colors and separate stream grammars; sprite storage domains versus natural selectors; and three narrowly bounded nonconsumed graphics-container tail bytes. Added inner uploader/alternate-pose routines, descriptive control/data CPU aliases, motion fields and scoped scratch roles.

Corrected inherited twelve-byte X=$0C..1 and twenty-byte Y=$14..1 copy prose in source and all affected reviews. The adjacent FEEA byte is a separate generic-storage unit, not part of the twenty-byte copy. F9BF's leading60is actual copied setup data. Corrected stale incomplete/unclassified dependency claims and the support-record four-byte schema; distinguished zero output runs, forward packed timers driven by INC07EE, FF-address/FE-end streams and inventory triples. Existing sound prose and historical investigation records were reused.

Fresh final byte-identical verification passed; source SHA 5a1c69fc6659bf7bceb2bd6956108bab3cfde15efb4ce232bb717fe05e1e8c5f. reading-after refreshed. Scope/identity/locator proof: analysis/commentary-round4/scope.json; verification: commentary/verification.json. Classification remains131051/131088=99.97%, with37structurally unknown bytes in13ranges; source readability does not establish whole-ROM completion. Current unresolved meanings/limits and primary residual evidence are in uncertainties.md; compact handoff analysis/round4-commentary-handoff.md.

No toolkit/ROM/map/state edits, deletion/cleanup, archive, publication, redirect/disassembly or research replay. Only existing NES-001 remains, with no new issue/blocker. No live ROMAtlas server, runtime session or background writer remains; all ROM-derived artifacts stay local. Natural full completion, separate credits and full hardware/game compatibility remain unclaimed.

## Round 5 disassembly completed — one pass

ROM classification: 99.98% (131067 / 131088 bytes classified; 21 bytes unknown).

Incomplete. Independently accepted L-R4-PPU:16 unique Static Data bytes, physical99646..99662, CPU852E..853E in effective banks12..15. Two bounded PPUDATA blocks at PPU starts1E00/1E50; exact immutable caller/helper/control-flow/boundaries in analysis/disassembly/r5-20261119-a/review.md and contract-validation.json. Corrected old no-consumer premise without rewriting historical records. All2267 prior region records remain exact; source SHA5a1c69fc6659bf7bceb2bd6956108bab3cfde15efb4ce232bb717fe05e1e8c5f and entire source/review history untouched.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3775 | 4417 | 8192 | 0 |
| 1 | 4240 | 3952 | 8192 | 0 |
| 2 | 1742 | 6450 | 8192 | 0 |
| 3 | 212 | 7976 | 8188 | 4 |
| 4 | 2166 | 6026 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 250 | 7939 | 8189 | 3 |
| 8 | 697 | 7495 | 8192 | 0 |
| 9 | 343 | 7849 | 8192 | 0 |
| 10 | 316 | 7876 | 8192 | 0 |
| 11 | 3366 | 4822 | 8188 | 4 |
| 12 | 6573 | 1619 | 8192 | 0 |
| 13 | 6638 | 1548 | 8186 | 6 |
| 14 | 7105 | 1086 | 8191 | 1 |
| 15 | 4760 | 3429 | 8189 | 3 |

Header16Data included outside PRG table. Total42183Code/88884Data;2268regions. Exact21unknown/12ranges in analysis/round5-counts.json. No new runtime execution evidence; r4 broad gameplay, source triage, rejection-review and residual ledger reused without unchanged route replay. Structural classification and execution/readability are separate.

Fresh map.validate/save/export, pass-005-reviewed checkpoint and byte-identical rebuild passed: analysis/pass-005-reviewed-verification.json. Map/state SHA65c2cde5d46cf3c6230bbec1704dec5c30a84b5a5c2872e1394db23887b7a8c4. ROM unchanged. Ready lead verified with append-only attempt history; other residual outcomes retained. Only existing confirmed NES-001 in analysis/toolkit-issues.md, workaround buttons arrays; no new issue/blocker. No toolkit edits/deletion/cleanup/archive/publication. No active server/writer.

Next phase full current-source commentary: retain all prior source/history/reviews, rebase onto pass005 and explicitly correct old852Eunknown/no-consumer prose including adjacent initialization-field note; name/explain two bounded PPU pattern blocks. Source readability was current at pass004 but needs refresh for this accepted span. No terminal exhaustion, forced residual classification, natural completion/boss identity or full hardware/game compatibility claim.

## Terminal coordinator handoff — orchestration complete, ROM classification incomplete

Round5redirect closed with0proposals/0readyleads after primary-linked scoped rejection review; no useful revised inference/test survives. Coordinator inspected decisive source/consumer/mapper/flag/wholehelper evidence and accepted terminal incomplete, not whole-ROM completion. Finalcheckpoint pass-005-reviewed,131067/131088classified99.98%,21unknown12ranges. Fullcurrent-source editorial acceptance completed:2280reviewedranges/2036units,1439Codeunitscover42183Codebytes; freshaudit/rebuildbyteidentical,reading-afterrefreshed. Finalsource94ddc38d54852b33965fab65d8e9e949594764a26fb7d13a8ea4fde6baebc337; map/state65c2cde5...unchanged. Onefinalstalesentence/26units corrected withpre-editfilespreserved. Finalhandoff analysis/final-handoff.md; exactresidual/reopeningconditions linked. Archive-options prepared/documentinventorychecked; noarchive/cleanup/publication/toolkitedits. Goalcompletionconcernsrequestedterminalorchestrationonly.
