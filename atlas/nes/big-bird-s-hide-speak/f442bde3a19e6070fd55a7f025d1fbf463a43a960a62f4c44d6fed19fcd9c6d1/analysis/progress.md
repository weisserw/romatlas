# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 — active disassembly
Baseline verified byte-identical: [report](baseline-verification.json), map maps/baseline.json. Input identity unchanged. No classification annotations yet. Live driver analysis/round-001/driver.py (exec session 83989), queue analysis/round-001/queue.jsonl; each request and exact response retained sequentially. Live emulator reset context [0,1,30,31]. First 180-frame startup request timed out after partial bounded execution; frame startup.png shows Sesame Street lamp intro. Further 120-frame request running. Static reset discovery response-0008.json begins supported reset at physical 253543 / CPU $DE57, but budget-limited candidates remain unannotated pending inspection. No preexisting leads. Baseline is fidelity only.

### Mid-pass durable checkpoint
Internal map saved with 137,279 classified bytes (6,185 observed Code; 131,094 header/CHR/vector Data). Exact roots saved analysis/round-001/observed-roots.json. CHR role corrected from Image to mixed Data after speech consumer inspection: $F58F changes CHR bank and reads $2007 while outputting $4011. Natural menu reached at frame 1596, snapshot menu.snapshot saved at frame 2196. All six directly selectable modes now being exercised by analysis/round-001/play_modes.py (exec session 16966); mode 1 visited and captured. Frame skips with tracing disabled preserve natural state but add no observed coverage. Selection/response bursts use Instruction trace. Full command order and exact response budgets/errors stay in queue/numbered response artifacts. Gameplay research source and confirmation status in analysis/gameplay.md.

### Recovered session / current frontier
Original driver and mode script ended externally with exit 143. Same phase resumed with analysis/round-001/driver-resume.py, exec session 10157: map.load analysis/state.json, emulator.start, durable menu snapshot import, buttons reasserted. Completed modes 1/2 have durable frame captures/snapshots; mode 3 now captured, 4–6 pending. Exact resume commands recorded after request 553; unfinished old request 553 was not rerun. Trace history is session-local: session-1 roots retained separately and new session roots harvested for static expansion. Added title/credits text command stream, five selector-bounded speech descriptors, quantizer ramp data slice and exact uniform PRG runs after boundary/conflict review. State and export saved. No toolkit issues found. Local Python driver transport race fixed; not a toolkit finding.

### Menu route correction and supported RAM shortcut
Visual inspection corrected earlier textual 'visited mode' console messages: only mode 1 entered gameplay; intended modes 2–6 mostly remained at the choice screen because A was pressed during the menu's movement/speech gate. Longer 90-frame movement waits still did not pass that gate. These historical console messages are attempts, not proof of visits. Static menu logic establishes selected game index at RAM $01FA: increment/modulo-six in bank-3 $A690, and fixed-bank dispatcher $DFD4 reads it to index $E00B return target words. Clean-menu snapshot plus $01FA=1 and ordinary A input successfully entered building gameplay (analysis/round-001/ram-mode-2-test.png), artificial state, no ROM edits. Remaining modes now use the same supported single-byte RAM shortcut and bounded representative inputs. All artificial observations must remain distinguished from natural reachability. Second exec session also ended externally 143; third driver exec 62265 restored saved map/menu snapshot. Static annotations are now saved after each acceptance.

## Round 1 disassembly — completed single pass, incomplete ROM

ROM classification: 37.20% (146270 / 393232 bytes classified; 246962 bytes unknown).

Code: 8,885 bytes; Data/Text: 137,385 bytes. Exact bank counts and unknown complement: [counts.json](round-001/counts.json), [per-bank table](round-001/per-bank.md). All 279 structural regions have evidence annotations. Durable recorded execution: 3,809 unique physical instruction starts; this is separate from classified byte coverage and excludes trace-off steps/lost live traces. Static-only code and artificial observed code retain their provenance.

Final checkpoint: maps/round-001-disassembly-final.json; source generated/round-001-disassembly-final; report analysis/round-001-disassembly-final-verification.json. Map.validate/export passed and ca65/ld65 rebuild byte-identical, unchanged SHA-256 f442bde3a19e6070fd55a7f025d1fbf463a43a960a62f4c44d6fed19fcd9c6d1. Earlier verified checkpoint preserved. The direct bounded client completed recovery; persistent driver sessions ended externally with 143, so their uncompleted queued calls were not evidence. Directory/prefix annotations were re-applied and saved through direct supported map.annotate before final checkpoint.

Broad best-effort progression: title/menu and common building introduction, five artificial selector alternatives, job-delay shortcut through four opened character windows. Distinct minigame mechanics and ending remain unconfirmed; see scenarios.json. A source-backed timer shortcut made meaningful progress; redirect gets the open-window snapshot and missing continuation contract. No terminal exhaustion claim: concrete speech/record/progression avenues remain for redirect. Current source readability still requires commentary over all classified code/data. No toolkit issues found; [persistent report](toolkit-issues.md).

### Final handoff checkpoint (supersedes preceding pass totals)

ROM classification: 37.31% (146708 / 393232 bytes classified; 246524 bytes unknown).

Round=1; phase=disassembly; status=completed single pass, ROM incomplete. Last already-running open-window A/right/A probe added 438 accepted artificial instruction bytes, then research stopped at coordinator instruction. Final code 9,323; data 137,385; unknown 246,524; 303/303 regions evidence-annotated. Durable root union 4,045 physical instruction starts, separate from byte classification. Final current map matches internal store. Latest byte-identical verified checkpoint: maps/round-001-disassembly-handoff.json; source generated/round-001-disassembly-handoff; report analysis/round-001-disassembly-handoff-verification.json. Earlier checkpoints and counts preserved as history. Compact handoff analysis/handoffs/round-001-disassembly.md. No toolkit issues found.

## Round 1 commentary — active
Prepared commentary from the verified final disassembly map using documented commentary.py prepare; preparation verification byte-identical. Frozen before/source baseline retained. Current frontier: full inventory review and naming/commentary of 9,323 Code bytes plus 137,385 classified Data/Text bytes; no readability review credited yet. All classified ranges remain pending. Unknown 246,524 bytes require boundary/uncertainty notes only. Final editorial audit reserved for final acceptance. Same phase remains resumable; no redirect handoff yet.

### Commentary batch 1 — source edits verified
All 6,075 baseline source lines inspected. Added 216 routine/handler explanations, 351 branch/control-flow labels and consistent CPU RAM/I/O aliases in commentary/source/symbols.inc. Documented movement and animation records, resource header fields, graphics queue commands, timers/controller/text mechanics, sound events/queues/instruments, mapper/NMI/reset and speech command/descriptor limits. All classified data and unknown boundaries now have source notes. Two coherent edit batches verified byte-identical, latest source SHA256 a1e02a4b40a00e7a055268dceb83592918105d03d9136122ba998f7c81dd88fc. Remaining: semantic consistency review of comments/aliases, compact routine units and region review bookkeeping, final fresh verify and reading-copy refresh. No remaining wholesale uncommented classified bank; this is still active commentary, not a successful handoff yet.

### Round 1 commentary — completed current-source readability
All 146,708 currently classified bytes reviewed: 9,323 Code and 137,385 Data/Text. Added 216 routine/handler contracts/units, 351 control-flow labels, consistent RAM/I/O/table/field operands and structure/uncertainty explanations. All 417 physical scaffold entries retain their boundaries and now have review summaries/source locations, including the 246,524 unknown bytes with honest boundary notes. Final consistency review corrected the $C73B translation-setter versus $C740 string-printer boundary; no new structural classification. Remaining current-source readability scope: none. Remaining ROM classification: 246,524 unknown bytes. Semantic limits: commentary/uncertainties.md.

Fresh final byte-identical verification: commentary/verification.json; source/game.asm SHA256 7d5882aae5b3ffce40e3a269fdd7078eb996bf0d6f6173b4e7effb219ac85e00, symbols.inc 9dde331a43b43b4bd082b05b12ba5f551c81f08e6d98044c8f3a89757ad7cf63. reading-after.asm refreshed after final source edit. Frozen baseline and all historical evidence preserved, input unchanged. Final editorial audit remains pending for final acceptance; this commentary completion does not establish whole-ROM classification. Compact handoff: analysis/handoffs/round-001-commentary.md. No toolkit issues found. No redirect, archive, cleanup, publishing or toolkit changes performed.

## Round 1 redirect — active consumer-contract investigation
Round=1; phase=redirect; status=active. Attempt analysis/redirect/r001-consumer-contracts/. Input structural checkpoint maps/round-001-disassembly-handoff.json, 146708 classified / 246524 unknown; commentary readability complete. Research only, maps/source/ROM unchanged. Frontier: speech descriptor and packed-sample length contracts; then progression dispatcher and final-probe static continuations, graphics/movement/sound consumers. Reusing primary phase-1 decodes and exact unknown complement; no old runtime replay.

### Redirect consumer-contract checkpoint — active
Speech directory parsed all90 entries with exact adjacency/terminal bits; last record extends accepted prefix9 bytes. Packed sample contract derived212 unique starts and shared segment suffixes, nominal continuous209935-byte span $8AA1..$3BEB0. Must split12 skipped physical64-byte bank tails before proposals; duplicate32-byte tail prefixes independently match next-bank data. Initial artificial runtime tests failed because imported menu NMI restored wrong PRG bank; decisive instruction trace preserved, corrected wrapper NMI-disable prerequisite now tested. No false agreement claimed from PC-stop flag. Independent accepted-root static closure reveals60 candidate instructions/124 bytes, still inspecting connected continuations. Current frontier and primary artifacts: analysis/redirect/r001-consumer-contracts/notes.md. Maps/classification/source unchanged; phase remains redirect.

### Redirect speech test and independent directory checkpoint
Corrected runtime contract passes three exact endpoint checks after executing wrapper NMI-disable instruction (including cross-bank and class4 repeated/shared cases); primary results sample-runtime-corrected-results.json. Four additional selector-bounded fixed CHR speech directories parsed (2/4/3/3 commands) and fixed prefix bytes independently match CHR source prefixes exactly:448 candidate non-code bytes with consumer/format evidence. Current research notes updated; still active redirect, next actual progression transition contract plus graphics/movement/sound consumers and static code continuation. No map/source changes.

### Redirect independent-consumer batch — active
Graphics independently bounded: bank0 encoded1090-byte directory,360 graphicsobjects exactly tile$452..$3C29 using consumer header/count/offset schemas;361stFFsentinel localized. Movement graph36 roots covers1518-byte contiguous streams and reveals1032bytes of callbackcode plus bounded inline tables. Sound35 start/end pairs and1004 six-byte events end EXACTLY at independent19-byte instrumentdirectory;7 unique instrumentstreams abut and terminate at$6413, where directcalled A403 introcode starts. Primaryscripts/results in analysis/redirect/r001-consumer-contracts/ and detailednotes.md updated. Progression sixentrytargets andmode6 timer0253/job4 contract recovered; currentprobe stack confirms actualmode6response. Slot0-only menu shortcut stalls graphicswait for modes2..6, preserved negativeevidence; next source-backed cleaninputboundary/handler-entry method andtimedsummary shortcut. Stillredirect; no classifications/source changes, no phase-completionclaim.

### Redirect complete structural proposal, gameplay research active
Combined proposals now cover exact246524-byte unknown complement (10786 Code/235738 Data), with254 evidence-linked claims and291 unknown-only bank-split spans in proposals-active.json. Accepted classification unchanged37.31%; independent disassembly must review, no full-completion claim. Followed source-backed intro/menu code after sound, all7 animation sets/34 frame lists,28 word records, four counted window records and remaining fixed routines/tables with exact barriers. Detailed notes updated. Current live operations: initialized-entry probes1..5 from actual mode6 E618 cleanboundary; dedicated0253/job4 shortcut reached actualE70D timed-summary entry in566operations and continues towardcleanupE768. Mode4 target correctedE387 before its batch. Next: review actual snapshots/frames, scenarios, proposals and exactledger/lead outcomes; phase remainsredirect.

### Redirect runtime method refined; active handle48296
Catch-and-save60-frame tests preserved partial progression for all6 modes but each exceeded10s with instruction aggregates, so they do not establish exhausted routes. Corrected sampled C037/stalled inference: actor/stream RAM and740-frame progression show useful continuation. Current gameplay_short_bursts.py/session48296 resumes saved partials with10-frame calls (1 traced+9 untraced for input-loop modes), stack-confirmed0201=4 introduction-only shortcut, stable loop fetch checks and real timed-summary continuation. Structural proposal remains246524 bytes, now284 claims/321 spans because bank recovery prefixes/IRQ jump/RTI are separate entry/context claims. Accepted map/source/ROM unchanged.

### Redirect actual timed-round closure; corrected intro shortcut active
Mode6 actualE768 cleanup fetched after22 error-free ten-frame bursts (gameplay-short-results.json, short-mode-6-roots.json and batch21capture), following source-backed0253/job4 expiry. Separate intro shortcut error localized: five choices initialized, FOUR actors displayed (A9C0 INC thenCMP4); oldindex4 overshot and produced invalid RAM-PC states, preserved with no visit claims. Correctedindex3 from clean pre-edit snapshots now testing1..5 under session27540; exact new log/results separate. Structural proposals/ledger/leads remainresearch-only and unaccepted.

### Redirect corrected mode1–4 loop evidence; mode5 resumed after interruption
Actual stable loops E062/E174/E27E/E387 fetched under correctedindex3 route; ordinarydirection/A bursts retained, directional branch observed, A/B answeracceptanceunconfirmed. Mode3frameclean,1/2partialnametableartifacts andmode1RAMPCsamplequalified. Worker27540 externally143 duringproductive mode5setup, so97439/finish_mode5.py resumes lastcleancheckpoint only, using actualF124 boundary and known0F command pointer toskipnarration throughnormalterminalrestore path. Mode6actualsummaryalreadycomplete/no replay. Detailed notes/scenariohistory/primaryartifacts retained; structuralproposal/321-dispositionledgerandreadyleads saved. Nextmode5result+compactcombinedhandoff, no structuralacceptanceclaim.

### Redirect mode5 actual comparison observed; final menu-return check active
Mode5-final-results.json reaches actualE4A1 byframe5252 after59error-free bursts; ordinarydirection then delayedA entersE846 andmode5type/letter-comparison body. Answerresultstillinfeedback; partialright-edge renderingqualified. All6 handlers now have boundedrepresentative artificialobservations; no narration-pointerpatch applied. Currentlastcheck99310/menu_return.py ordinaryStart transition frommode5checkpoint verifies sharedDFE2 exit/menu andcaptures scene; then packagecombinedproposalhandoff. Structuralpacket remainsunaccepted246524bytes, integrity/source/map/ROMhashes passed.

## Round 1 redirect — complete with combined ready proposal
Round=1; phase=redirect; status=complete; next=separate independent disassembly. Frozen analysis/redirect/r001-consumer-contracts/proposals.json:284 claims/321 physical unknown-only spans,246524 proposed bytes (10786 Code,235738 Data),0 newly accepted. Input/current map remains maps/round-001-disassembly-handoff.json and analysis/state.json:146708/393232 classified37.31%,246524 unknown. Exact ledger, all neighbors/bank contexts/dispositions,7 ready leads and primary constraints persisted. Current frontier is independent proposal review, including B911/B914 shared-instruction boundary; no unproposed interval. No terminal classification/compatibility claim.

Gameplay bounded batch: actual artificial input loops1..5; direction1..4; mode5 A/B/type/letter comparison; actual game6 timed-summary E768 cleanup; ordinary Start sharedexitDFE2 and menu introduction. Earlier index4 frozen states rejected; index3 corrected. Natural routes, all answers and full rendering remain qualified (mode1 later RAMPC and partialrightedge1/2/5). No extra gameplay permutations needed for this closed structural batch. Selectedframes/source/readable evidence linked in scenarios and handoff.

Integrity checks passed: exact proposed unknown union, physical Code views, unchanged ROM/state/checkpoint/commentarysource. Existing verification historical; next accepted map needs fresh rebuild/verification and fullcurrent-sourcecommentary; finaleditorial remains pending. Handoff analysis/handoffs/round-001-redirect.md. Evidence~1.2GiB,free~3.1GiB; scratchminimal. No map/source/ROM/toolkit edits, archive, cleanup/deletion or publication performed.

## Round 2 disassembly — active independent review
Round=2; phase=disassembly; status=active. Input handoff analysis/handoffs/round-001-redirect.md; input/current checkpoint maps/round-001-disassembly-handoff.json and analysis/state.json (146708 classified,246524 unknown). Frozen proposal analysis/redirect/r001-consumer-contracts/proposals.json is unaccepted. Frontier: independently review all consumer constraints, exact bank-aware Code boundaries and finite Data extents; localize F52E timing-prefix ambiguity; repair B911/B914 shared instruction representation preserving prior evidence. Ready leads R1-L003..009. Existing baseline verified; commentary/source remains unchanged and final editorial audit pending. No new output checkpoint yet.

### Round 2 independent review complete; acceptance frontier
All284 frozen claims reviewed by role/contract and exact physical/contextual bounds;189 full Code claims independently decoded with exact endpoints (5768 unique starts), six finite parsers rederived and compared every frozen record. Primary decisions analysis/round-002/review.md and decisions.json; consumer/vector/array checks independent-data-checks.json. Local correction: bank16=4 prefix31/32match, finalFF genericData rather than false exact-copy claim. Exactly physical$3F53E/CPUF52E remains unresolvedUnknown; calledF52F onward accepted separately. B911/B914 requires canonical region154 expansion preserving originalStatic region/evidence and actual priorB914 observed roots. LeadIDsR1-L003..009 reviewed, broad gameplay outcomes reused with visual/natural-route limitations. Frontier=apply supported annotations, save/validate/export, fresh unique checkpoint+byte-identical rebuild. Commentary/source unchanged; no output checkpoint yet.

## Round 2 disassembly — completed single pass, ROM incomplete

ROM classification: 100.00% (393231 / 393232 bytes classified; 1 bytes unknown).

Round=2; phase=disassembly; status=completed; next=fullcurrent-sourcecommentary thenredirect. Input maps/round-001-disassembly-handoff.json; output maps/round-002-disassembly-final.json equals maps/current.json, analysis/state.json saved. New246523classified:10785Code/235738Data. Totals20108Code/373123DataText/1Unknown;624/624regions evidenceannotated. Exact banktable analysis/round-002/per-bank.md and counts.json; sole complement physical$3F53E CPUF52E in unknown.json. R1-L003..009 verified; R2-L001localizedunresolved. Current frontier=sourcecarry/readability, thenindependent residualreasoning; no terminalwholeROMclaim.

Independent review all284claims, full189Code endpoints, six finite parsers and source/vector/dispatch/mechanicalcontracts documented analysis/round-002/review.md/decisions.json. Corrected bank4copy31/32matches;32byte tailgenericData; B911 canonicalregion154expanded with originalevidence and observedB914roots preserved. Initial accounting span overlapped newcanonicalprefix and was rejected by documented boundary guard; resumed fromsavedbatch afterclipping knownbytes, no toolkitissue. No newruntime, completed qualified broad sixhandler/mode5answer/mode6summary/Startmenu evidence reused; historical4045observedstarts separate fromclassification.

Fresh configuredrelease map.validate/generate/ca65/ld65 verification passed byteidentical: analysis/round-002-disassembly-final-verification.json; generated/round-002-disassembly-final. ROMunchanged; oldcommentary/source hashesunchanged, integrity.json. Newsource readability unreviewed; fullcurrent-sourcecommentary and finaleditorialaudit remainrequired. Handoff analysis/handoffs/round-002-disassembly.md. No toolkit issues found; persistent report analysis/toolkit-issues.md. No archive/cleanup/publication/toolkit/source/ROMedits.

## Round 2 commentary — active
Input maps/round-002-disassembly-final.json; generated/round-002-disassembly-final, byte-identical report preserved. Entire prior commentary moved intact to analysis/round-002/commentary-before before preparation. Scope: all 393231 classified bytes and single Unknown at physical $3F53E. Carry by physical identity, reverify changed boundaries; new-source frontier pending. Final editorial audit remains pending.

### Round 2 commentary carry and first naming batch
Prior source carried by exact physical instruction identity: 4476 of4477 old starts (B914 alternative represented by equate at original CPU/physical identity inside canonical BIT). Fresh rebuild byte-identical. Added235 routine/block contracts,274 mechanical control-flow labels and80 state aliases; verified byte-identical again. Covered new movement callbacks/three-track effect, intro/menu choreography, four-window and letter helpers, six game loops, fixed text/job/sprite controls, speech packed/CHR-buffered decoder and all15 copied recovery contexts. Remaining frontier: data-format prose across new ranges, correction of stale old unknown claims and inline category bytes within broad Code annotation, full inventory consistency/review carry and final fresh verify/read. Final editorial audit pending.

### Round 2 commentary data/consistency batch
Added finite graphics/movement/sound/animation/window/alphabet/summary/CHR-prefix format prose across all newly classified Data ranges; carried valid existing descriptions. Corrected stale round1 unknown-loop claims and mapper-register operands independently from ROM table CPU aliases. Source represents inline letter-category17bytes physical720A..721B and actual B20B shuffle entry with exact bytes; byte-identical verification passed. Structural discrepancy logged R2-L002 with exact consumer and direct-call evidence; map untouched for next redirect/disassembly correction. Remaining: inventory/review-credit reconciliation, concise current uncertainties and full-scope assessment, final fresh verify and read. Final audit pending.

## Round 2 commentary — completed current-source readability
All20,108 mappedCode+373,123Data/Text and sole1byteUnknown boundary reviewed; no remaining classified-source frontier.625scaffoldentries reviewed,550meaningfulunits,216priorunits and302exact prior ranges carried with source/dependency rechecks. Fresh byte-identical verification and refreshed reading-after complete; reports commentary/verification.json and analysis/round-002-commentary-verification.json. Full prior commentary preserved analysis/round-002/commentary-before. Finaleditorial_audit pending, not invoked. R2-L002 exposes17inlineData bytes physical720A..721B and actual B20B shuffle entry inside oldlinear decode; source corrected exactly, map unchanged for next separate redirect/disassembly. R2-L001 single EA remainsUnknown. Thus readability complete, structural/finalacceptance incomplete. Compact handoff analysis/handoffs/round-002-commentary.md contains exact evidence, correcteddependencies and nextwork. No toolkit/ROM changes, archive, deletion, publication or runtime replay.


## Round 2 redirect — active independent boundary research
Input checkpoint maps/round-002-disassembly-final.json equals maps/current.json; analysis/state.json retained. Readability gate analysis/handoffs/round-002-commentary.md complete; final editorial pending. Research directory analysis/redirect/r002-inline-boundaries. Classification remains393231/393232, one Unknown physical3F53E. Frontier: independently verify R2-L002 raw consumers/callers/exact category bounds and connected response helper contracts, reassess R2-L001 primary constraints, triage other concrete source uncertainties. No classification/source/map changes or unchanged runtime replay.

## Round 2 redirect — complete with ready localized correction
Input maps/round-002-disassembly-final.json unchanged equals current; accepted393231/393232,1Unknown. Research analysis/redirect/r002-inline-boundaries complete, frozen proposals.json recommends R2-L002 exact17Code→Data split physical720A..721B and canonical B20B entry, zero newly accepted/unknown gain. Connected helper/caller/consumer contracts independently checked. New important premise correction: category compares indexcount..0 inclusive, extras03/02/A2 cannot match generatedASCII41..5A; next commentary must explain overread and shuffle's own4stack values/nonnegative type precondition. Source unchanged.
Broader9435instruction/2266directedge audit found only known B914alternate entry; finite733PRG+47CHR command review has no class3selector witnesses, so wider stride question has a specific missing source prerequisite. F52E residual primary reasoning/rejection review preserves oneUnknown; no forcedPC/unchanged runtime. Exact unknown-ledger and integrity persisted; histories/scenarios retained. Handoff analysis/handoffs/round-002-redirect.md; next separate independentdisassembly→fullcurrent-sourcecommentary→redirect, no terminal/finaleditorial claim. Coordinator must inspect scoped residual rejection review before later terminal assessment. Free~3.1GiB; no map/state/source/ROM/toolkit mutation, cleanup/deletion/archive/publication.

## Round 3 disassembly — active
Input maps/round-002-disassembly-final.json and analysis/state.json. Frontier: independently review and apply R2-L002 localized17Code→Data correction and canonical B20B entry; preserve current commentary. R2-L001 remains unresolved under scoped primary rejection review. Reuse completed gameplay and connected audit without unchanged replay.

## Round 3 disassembly — completed single pass; ROM incomplete
ROM classification: 100.00% (393231 / 393232 bytes classified; 1 bytes unknown).
20,091 Code;373,140 Data/Text;0 newly classified;17 Code→Data. R2-L002 verified exact720A..721B inline data, canonical B20B. R2-L001 remains singleUnknown physical3F53E/CPUF52E [4,5,30,31]; scoped review reused with no new discriminator. Evidence analysis/round-003/review.md, rpc.jsonl and counts.json. Fresh checkpoint maps/round-003-disassembly-final.json validated/generated and byte-identical verified (analysis/round-003-disassembly-final-verification.json); commentary directory and input ROM unchanged. Broad six-mode gameplay/connected audit reused with qualifications; no useful unchanged replay. Per-bank exact accounting below. Next: full current-source commentary including inclusive03/02/A2 extra reads and own-four-values shuffle precondition; final editorial audit pending. No toolkit issues found; analysis/toolkit-issues.md retained.
| Bank | Code | Data/Text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 0 | 8192 | 8192 | 0 |
| PRG01 | 9 | 8183 | 8192 | 0 |
| PRG02 | 1032 | 7160 | 8192 | 0 |
| PRG03 | 4677 | 3515 | 8192 | 0 |
| PRG04 | 0 | 8192 | 8192 | 0 |
| PRG05 | 9 | 8183 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 9 | 8183 | 8192 | 0 |
| PRG08 | 0 | 8192 | 8192 | 0 |
| PRG09 | 9 | 8183 | 8192 | 0 |
| PRG10 | 0 | 8192 | 8192 | 0 |
| PRG11 | 9 | 8183 | 8192 | 0 |
| PRG12 | 0 | 8192 | 8192 | 0 |
| PRG13 | 9 | 8183 | 8192 | 0 |
| PRG14 | 0 | 8192 | 8192 | 0 |
| PRG15 | 9 | 8183 | 8192 | 0 |
| PRG16 | 0 | 8192 | 8192 | 0 |
| PRG17 | 9 | 8183 | 8192 | 0 |
| PRG18 | 0 | 8192 | 8192 | 0 |
| PRG19 | 9 | 8183 | 8192 | 0 |
| PRG20 | 0 | 8192 | 8192 | 0 |
| PRG21 | 9 | 8183 | 8192 | 0 |
| PRG22 | 0 | 8192 | 8192 | 0 |
| PRG23 | 9 | 8183 | 8192 | 0 |
| PRG24 | 0 | 8192 | 8192 | 0 |
| PRG25 | 9 | 8183 | 8192 | 0 |
| PRG26 | 0 | 8192 | 8192 | 0 |
| PRG27 | 9 | 8183 | 8192 | 0 |
| PRG28 | 0 | 8192 | 8192 | 0 |
| PRG29 | 9 | 8183 | 8192 | 0 |
| PRG30 | 7524 | 668 | 8192 | 0 |
| PRG31 | 6732 | 1459 | 8191 | 1 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round 3 commentary — active
Input: maps/round-003-disassembly-final.json and analysis/state.json; full prior commentary preserved at analysis/round-003/commentary-before before changed-baseline preparation. Carry valid names/prose/review by physical identity; affected frontier $7191..$725B and inclusive category/shuffle dependencies. Full current-source inventory consistency assessment, fresh verification and reading refresh pending; final editorial audit remains pending.

# Round 3 commentary — current-source readability complete

Input maps/round-003-disassembly-final.json equals maps/current.json; phase1-state.json is frozen from analysis/state.json. Entire prior commentary was preserved at analysis/round-003/commentary-before before fresh preparation. No prior evidence/source was discarded.

Full inventory consistency assessment covers 627 current physical regions and 550 reviewed meaningful units across all PRG/CHR/header storage. Carried 624 unchanged scaffold reviews and all sound source names/prose by exact physical identity and dependencies; reviewed three split entries and refreshed source locations. Current classified readability covers 20,091 Code and 373,140 Data/Text bytes. One Unknown byte at $3F53E retains its honest boundary/timing note; no classified readability gaps remain. Scope assessment: analysis/round-003/commentary-work/review-assessment.json. Prior consumer/format inspection and valid review credit were reused; no cosmetic whole-source rewrite was needed.

Corrected affected map-derived headers to Code $7191..$720A, Data $720A..$721B, canonical B20B code $721B..$725B. Explained inclusive count..0 searches and extra $03/$02/$A2 reads beyond 9/3/2 letters, excluded by uppercase candidate bounds. Independent preceding RTS and calls bound the 17 data bytes. Shuffle saves/restores its own four values with balanced PHA/PLA; known callers supply nonnegative 0..4 values required by its used-slot test. Preserved B914 alternate equate within B911 BIT operands, separate mapper/footer contexts, speech-tail 31/32 exception at $13FEF and all valid earlier uncertainty/role comments.

Fresh commentary.py verify passed byte-identical against unchanged original SHA256 f442bde3a19e6070fd55a7f025d1fbf463a43a960a62f4c44d6fed19fcd9c6d1; reading-after.asm refreshed after source edits. Classification remains 393231/393232: one unresolved byte prevents a whole-ROM completion claim. Final editorial audit remains pending and was not invoked. No map/classification/toolkit/ROM edits, archive, publication, cleanup or replay performed.


## Round 3 redirect — active residual/affected-source review
Input maps/round-003-disassembly-final.json equals current; readability handoff analysis/handoffs/round-003-commentary.md complete. Research analysis/redirect/r003-residual-review. Frontier: inspect accepted category/shuffle correction and callers for new actionable continuations; retrieve and assess R2-L001 primary bytes/call operands and scoped rejection. Reuse unchanged whole-source/gameplay audit; research records only, no final editorial execution. Accepted393231/393232, residual physical3F53E CPUF52E bank31.

## Round 3 redirect — completed; terminal incomplete recommendation
Input maps/round-003-disassembly-final.json equals current; analysis/state.json unchanged. Current readability complete. Research analysis/redirect/r003-residual-review contains affected-source/independent avenue findings, compact scoped rejection review, fresh exact complement and protected hash/primary-byte integrity. Accepted393231/393232,1Unknown physical3F53E CPUF52E bank31[4,5,30,31]; proposed/newlyaccepted0; no ready leads. R2-L002 accepted boundary/caller/type/ownstack contracts hold; no new connected correction. R2-L001 optionaltimingprefix versus skippedEA remains structurally ambiguous; no universal livecaller prerequisite imposed. Existing whole-source/directedge and broad sixmode/answer/timedsummary/Startmenu audits reused on unchanged premises; no useful revised test remains. Coordinator must inspect scoped rejection before accepting terminal incomplete, then perform separately pending editorial acceptance. Handoff analysis/handoffs/round-003-redirect.md. ROM/map/state/source/configuredbinary unchanged; free~3GiB; no runtime replay, cleanup/deletion, archive/publication or toolkit edit.

## Final editorial acceptance — active
Current-source readability credit retained across 627 regions/550 meaningful units. Reviewing final pending editorial acceptance, critical bank/category/shuffle/alternate-entry/residual contracts and source-location inventory; three known category payload operand aliases are being named without changing values or source line counts. Structural classification remains 393231/393232; fresh final audit/verification pending. No archive/cleanup/publication authorized or performed.

## Final editorial acceptance — passed
All 627 physical scaffold regions and 775 meaningful unit records reviewed; no pending editorial frontier. Reused 550 sound units, added 225 split routine-fragment records across 64 named contracts; normalized audit source paths. Minimal source repairs name category payload operands, correct window carry/title continuation descriptions and three title panels. Documented commentary.py audit passed with fresh byte-identical rebuild and refreshed reading copy. Actual reports commentary/audit.json, commentary/verification.json; findings commentary/editorial-audit.md, handoff analysis/handoffs/final-editorial.md and fresh hashes analysis/final-editorial-integrity.json. Structural outcome remains terminal incomplete, 393231/393232 classified, one Unknown physical3F53E/CPUF52E. ROM/map/state/toolkit unchanged; no archive, cleanup/deletion, publication or runtime replay. No toolkit issues found. Coordinator final inventory/handoff remains separate.


## Coordinator terminal handoff — workflow complete, ROM incomplete
Three sequential rounds and final editorial acceptance are complete. Coordinator inspected the last scoped rejection reasoning and decisive raw bytes/call operands; analysis/coordinator-rejection-review.md accepts terminal incomplete, not blocked or whole-ROM complete. No ready leads remain.

ROM classification: 100.00% (393231 / 393232 bytes classified; 1 byte unknown).

Exact residual [0x3F53E,0x3F53F), CPU F52E, bank31 [4,5,30,31]. Final map/state/checkpoint maps/round-003-disassembly-final.json match; 20091 Code/373140 DataText. Fresh final editorial audit and verification passed, reading copy refreshed:627regions/775units reviewed. Final store validate and map validate each returned {"ok":true}. No source changes followed the successful audit.

Final handoff analysis/final-handoff.md; requirement evidence analysis/coordinator-completion-audit.json; archive contract accounting analysis/archive-document-accounting.json; explicit document/omission selections archive-options.json. Additional final conclusions/history selected; only duplicate historical instruction copies omitted. Prior commentary, research and checkpoints remain preserved. No archive, cleanup/deletion, publication, toolkit or ROM modification occurred. Internal state/source/assets remain local; only supported ROM-free mapping is portable. Orchestration completion does not mark ROM classification complete. Reopen residual research only on changed evidence or justified distinct inference.

## Authorized batch fallback disassembly — complete

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

This is manual-inclusive map occupancy under the explicit user batch exception: evidence-backed structural classification remains393231/393232, plus1 Manual fallback byte, not a newly proven non-code role. Exact pre-count393231classified/1unknown; post393232/0. Independent full physical complement was ONLY offset259390 length1 [0x3F53E,0x3F53F), within the user limit of10. Supported map.annotate added generic Data, Manual provenance, unknown meaning. Optional timingNOP versus skippedEA ambiguity remains unresolved at CPUF52E bank31 [4,5,30,31]; previous evidence/rejection retained with no research replay. 20091Code/373141DataText post versus20091/373140 pre. PRG31 8191/1→8192/0; all other banks unchanged; complete per-bank ledgers analysis/batch-fallback-disassembly/{pre-counts,post-counts}.json.

State saved and current map exported; map.validate and fresh checkpoint validation passed. maps/batch-fallback-disassembly-final.json, generated/batch-fallback-disassembly-final, analysis/batch-fallback-disassembly-final-verification.json confirm byte-identical rebuild. Retained exception record analysis/batch-fallback-disassembly/record.md, frozen inputs/RPC/integrity in same directory. Existing region metadata and commentary unchanged; next separate commentary refresh required. Prior final artifacts preserved. No toolkit issues found; analysis/toolkit-issues.md retained. ROM/configured binary unchanged; no toolkit edits, deletion/archive/publication.

## Batch coordinator final acceptance — complete under explicit exception

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

Evidence-backed393231 plus exactly1 Manual generic Data fallback, physical offset259390 length1 [0x3F53E,0x3F53F), unknown meaning. Optional timingNOP versus skippedEA ambiguity and explicit user authorization retained in analysis/batch-fallback-disassembly/record.md and final-handoff.md. Independent coordinator check confirms no other annotation change and empty complement. Sequential commentary then final editorial workers passed all627regions/775units, fresh byte-identical strict audit and read-only editorial_status complete. Current map/state/checkpoint/frozen baselines match; current source/reading matches passing audit. Final store/map validation passed. Evidence analysis/batch-fallback-coordinator/final-validation.json.

Final conclusions analysis/final-handoff.md; completion audit analysis/coordinator-completion-audit.json. Archive document-selection contract inspected; archive-options.json and refreshed analysis/archive-document-accounting.json retain fallback/final records and account for all unrecognized Markdown. Prior terminal conclusions frozen under analysis/batch-fallback-coordinator/prior-terminal. No outstanding required blocker; no research replay, archive, deletion, cleanup, publication, ROM or toolkit change. This exception completes assigned map occupancy and workflow, without claiming evidence-backed classification of the fallback byte.
