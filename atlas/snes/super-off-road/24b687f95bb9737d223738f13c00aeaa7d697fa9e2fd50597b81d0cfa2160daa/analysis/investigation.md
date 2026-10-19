# Investigation ledger — round1 disassembly

Pass completed with ROM incomplete. See [compact handoff](round01-disassembly-handoff.md), [batch history](progress.md), [ready leads](leads.json) and [exact unknown ranges](unknown-ranges.json).

ROM classification: 61.50% (322448 / 524288 bytes classified; 201840 bytes unknown).

Evidence-backed batches: reset/splash and frontend; consumer-bounded graphics/palette/rectangle/default tables; uploaded SPC entry/control flow; normal name/shop/race/physics/AI/nitro; static collision/lap/ranking branches; natural podium and Wipeout/Continue; continued shop/Huevos Grande; all-track fixed-domain loaders, maps/terrain/graphics/object/waypoint formats; intentional overlapping return tails. Each batch was inspected before annotation, map validated and saved/exported. Full consumer/source extent ledger: [track-asset-proposals](evidence/track-asset-proposals.json).

R1-L001superseded: artificial finish comparison was unnecessary because natural snapshot already had blue AIwinner/fifth-lap; restored natural snapshot and captured podium loader. Do not repeat patches on unchanged evidence. R1-L002verified: shared BIT/LDA entries are one code role with exact static branches and fixed formats; commentary equates still needed. R1-L003/004/005ready: specific producer-domain, audio-format and supported-callee-contract recipes in leads.json; none requires blind repeated driving or every static branch to execute.

Ordinary route exhausted for this pass: startup/title→Select join/name→shop→Fandango→AIlap5→podium→loss/ContinueYES→shop→HuevosGrande. Same-track samples diminished from thousands of new locations to45,4and15after long waits; second-track sampling after its productive initializer gained20then0. Remaining opportunities require targeted redirect research, with prerequisites and expected distinguishing evidence in leads.json. No per-range exhaustion or whole-game claim is made.

Structural annotation coverage100%of390classified regions; editorial readability reviewpending. Natural observed fetch union5705mainROMstarts/12163bytes plus827ARAM/32IPLstarts is separate from15348codebytes and307100databytes structurally classified. Unknowns201840bytes in173ranges; per-bankcounts below.

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|13862|3814|15092|
|01|0|0|32768|
|02|0|12288|20480|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|22326|10442|
|08|0|5280|27488|
|09|0|32592|176|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27648|5120|
|0E|1486|59|31223|
|0F|0|8|32760|

Verification: [round01-disassembly-final-verification.json](round01-disassembly-final-verification.json) reports ok=true/byte_identical=true using mapSHA25658540bdd7b4a4f39000a7b09b140988a5c1b228612bc4372fa152ec4dcf465b4. Internal state and portable current map saved; input unchanged. Toolkit: one missingfeature TI-001, no confirmed/suspected bugs, no blocker; [report](toolkit-issues.md).

## R1 redirect: truck pose producer/source domain proved (2026-10-18)

Input round01-commentary-final. Four clamp gaps physical C33/C39/C5D/C63 are M8 LDA #00/#4F (two bytes each), with direct preceding branch/fallthrough contracts. Thus both quantized tilt values are0..4. Consumer 8C7C indexes exactly25 words at8CA2 (physicalCA2..CD4,50bytes). Table is level0 plus24nonzero16-aligned poses32..400; level heading>>3 yields0..31 while tilted heading>>4 yields0..15. Resulting pose word domain is exactly0..415, with no hole. pack8BFD and DMA904C transform each word into two64-byte strips separated by512; bank1/2 comes from packed bit15. Exhaustive enumeration of the bounded *semantic* selector domain yields exact physical source union008000..015000 (53248bytes). It establishes sprite-source layout, not natural reachability of every suspension combination. Replay `python3 analysis/redirect/pose.py`; compact result `evidence/round01-redirect-pose-domain.json`. No map edits. R1-L003 ready recipe materially improved to a direct source-union proof; classification can cover the whole union because every byte has a bounded consumer role, without demanding all selectors naturally execute.

Direct classified-code target sweep found only one wholly unknown direct target, BF59; missing fallthroughs require a separate sweep. Track8 projection gap10D3..10FE independently decoded from90C2 REP20/SEP10 contract: test worldY-1D80<0800 and truckheading>=2E, set/clear attr bits3000/1000 then rejoin90FE. All43bytes are supported code; natural track8 reachability untested. BF59 is M8 second-player name-grid column-wrap path, restoring column4 then sharing row wrap/cursor positioning already classified. Exact decodes saved `evidence/round01-redirect-track8.json`, `...-name2.json`; direct-frontier inventory `...-direct-frontiers.json`.

### R1 redirect static closure batch and caveats

`analysis/redirect/fallthrough.py` derives entry contexts only from supported final nonterminal/noncall instructions of classified regions; `discover_frontiers.py` follows96fallthrough/direct frontiers with static-only20-root/1000instruction budgets. `list_closure.py` individually decodes805unknown candidate starts totaling1754instruction bytes; listing `evidence/round01-redirect-static-listing.txt`, full contexts/instruction semantics `...-static-inspected.json`, raw graph/diagnostics `...-static-closure.json`. Two discovery chunks exhausted1000instruction budgets; resulting candidates are proposals, not observed execution. Do not sum805lengths blindly as new classification: BIT operand-sharing at178F spans existing region bytes; preserve/consolidate a single physical role and alternate entries. Contracts with unknown E/M/X remain unknown; fixed-width instructions may still classify. No call-return preservation is assumed by these seeds. Next method: explicit proven summaries for RNG99C2, framewait9A64, sound8256 and rectangleC4FA to seed supported returns, particularly track0 object continuation and pause/resource path.

### R1 redirect audio intermediate constraints / natural bounded experiment

Envelope return-stack table ARAM0889..0892 comprises low5/high5 vectors to0893,08C5,0900,0901,092C; state writers/reset0, release3 and transitions0->1,1->2,3->4 prove0..4 domain. States2/4 are RET sinks. Exact code graph inspected in `evidence/round01-redirect-audio-listing.txt`; one unentered RET byte08FB stays unknown. Negative-sequence vector slab ARAMA10..A53 has34coherent handler targetsA54..C1E and shared stack helpersC2E..C57; all have consistent stream-command semantics and fixed SPC lengths. Full negative-command selector/table extent is not yet independently proven; do not classify table solely by pointer resemblance. Driver command92 writes unmasked next sequence byte as SRCN; this prevents blanket directory extent inference from instrument shape. Directory appears33entries but that alone is not evidence.

Natural final-gameplay machine at frame7112, no buttons changed, trace Read/ARAM5000..5400/capacity20000, trace.clear, bounded10frames/main/max_operations1000000/timeout10000 to7122. Completed without artificial state,9654retained/0dropped/aggregate_dropped0. DSP directory loop-byte reads at5002/3,505E/F,5062/3 establish selected SRCN0,23,24; CPU reads distinguished. Saved `evidence/round01-redirect-audio-summary.json`, `...-audio-reads.json`. Exported durable starting snapshot `...-start-snapshot.json`; disabled tracing and imported it after experiment, restoring7112. No claim of persistent named snapshots. Existing read query API lacks next_offset; local pager uses offset+=events.length (documented trace event paging, no bug claim).

BRR format/source provenance supports selected directory entries and start-to-END chains. `analysis/redirect/brr.py` confirms sample0ARAM50CD..51E4 (31blocks,loop515D),23ARAMA00B..A3E0(109blocks,loopA00B),24ARAMA3E0..AC08(232blocks,loopA3E0); all9-byte blocks, terminal END+LOOP flags, loop on in-chain boundary. Physical blocks use separate first/second upload transforms; compact proposals `evidence/round01-redirect-brr-proposals.json`. These are three selected structures, not full audio directory/BRR domain. Continuing command/sequence constraints and static return closure before handoff. Map/commentary unchanged.

### Audio sequence/BRR expansion and call-return/resource closure milestone

`analysis/redirect/sequences.py` is a bounded semantic parser derived from individually inspected SPC handlers, including fixed/one-shot note durations, jump, call/return and counted-loop stack mechanics. Initializer proves song0; main encoded APUIO2 commands0201/0301/0401/0501/0E01/0F01/1001 prove song indices1/2/3/4/13/14/15 after driver DEC X. Saved song0..4,13..15 results `evidence/round01-redirect-song*.json`. Song0 is the single80 voice-stop byte atARAM1567. Others terminate, reload song, or repeat an exact parser state; none hit unsupported opcode, missing stack contract or operation limit. This is static format proof, not ordinary execution of every stream branch. Streams already yield selected command-table entries and SRCN operands; full34-entry negative-command table extent is still not justified from byte sign alone. Only command indices actually consumed by parsed streams are safe table proposals.

Source consumers now establish21SRCN entries (0,2,3,5,7,8,10,12,13,14,15,16,20,21,22,23,24,25,29,30,31). `brr.py` expands their directory and9-byte END-bounded sample chains; every loop is on an in-chain block boundary. Sample20 crossesARAM8200; its ROM storage MUST split at first upload physical77E04 and second upload physical78004, excluding upload headers/terminators/padding. Replayable script/result now handles this split. Last selected sample31 endsFC27; tailFC27..FFFF is not thereby sample data. Unselected directory entries and songs5..12 remain unclassified until a selector/independent layout proof; no unsupported global extent claims.

`analysis/redirect/returns.py` applies individually inspected width/DB/DP summaries for RNG99C2, framewait9A64, sound8256 and rectangleC4FA, taking known-region calls as well as new candidates. Additional145unknown instruction starts/353bytes inspected in `evidence/round01-redirect-return-inspected.json`; exact roots/contracts/diagnostics in `...-return-closure.json`. Two discovery chunks reached1000instruction budget; no blanket callee-preservation assumption. This reaches pause helperD75C and descriptorD780: 20byte width5,height3/eight-remap descriptor and30bytesource CPU07DAB4 (physical03DAB4). Also reaches track8/14 rectangle cases with descriptorsC123(3x3),C137(2x2); physical sources068C00..068C12,06D412..06D41A,06D41A..06D422. Each descriptor is20bytes. Newly inspected palette-selection branch atB38C copies32bytes from09D3B0 or09D3F0; both source extents are independently count-bounded. These resources are unknown in current map and ready for bounded disassembly validation. Obvious continuation is closing the supported rectangle returns (41EC etc) and track0 object helper CF45 using established PHP/PLP endpoint502C..502D, then assess residual context barriers.

### Connected closure checkpoint — track0/pause/audio

Budget-tail closure `analysis/redirect/close_budget.py` used7supported noncall successors omitted by grouped traversal, adding16instructions/36bytes at4FF8..501C: proximity test X difference+0300<0600 and Y difference+0080<01C0, starting composite animation2F; both branches restore saved X at501C and loop through502C PLP/RTS. Track0 renderer CF45 therefore has a supported PHP/PLP contract (project helper969A itself has balanced PHP/PHA/PLA/PLP; no DB/DP changes). Expanded return summaries now include969A,CF45,D75C,D62E; added21instructions/54bytes, including pause transition57CD..57E7 and dynamic-object sprite coordinates495C..4978. Evidence `...-budget-closure.json` and updated`...-return-inspected.json`. Remaining pause continuation calls B47D/B4C6; inspected source shows nested status preservation with common final PLP/RTS, so closing these returns is the last concrete continuation before proposal packaging.

Explicit main loss audio1001 additionally proves song15, parsed without unsupported opcodes (360distinct bytes including called stream). Across songs0..4,13..15, selected command union is30of34 even values80..C2; only8E/98/AC/BA lack a selected stream operand. Full table can remain split at these cells; all34 handler code contracts are independently coherent/inspected, but no natural execution claim. Selected song configurations use pointer tables1020/1040 indexed(song-1), copying exactly8D=141bytes (not143). Song0 wraps config indexFF into mutable driver state; do not attribute its config pointer reads to immutable ROM. Other selected config pointers and source extents are concrete consumer data. Parsing unselected songs5..12 or assuming SRCN32 from adjacent pointer shape would currently require an independently supported selector/layout contract; no such fact has been established. These are actual domain barriers rather than a need for ordinary gameplay reachability.

### Final connected resource triage and proposal manifest

Pause D794 now closes through57E7..57FA (extent<60 grows by8, loop, PLP/RTS). `analysis/redirect/summary_cfg.py` composes10explicit inspected call summaries while retaining strengthened alternate contexts across repeated call sites, using one-instruction toolkit decodes in1000instruction progress chunks. Finite queue closes (>6000contextual instructions); only9new starts/19bytes beyond prior unique starts. Exact full research graph `evidence/round01-redirect-summary-cfg.json`; this is static inference, not observed execution.

Followed object producers/tables rather than stopping at code: track-order64-slot wrap and C905 six-entry initializer prove pointer table4984..4A04 plus17six-record X/Y/height lists4A04..4C68 (740bytes total). The first record field is WORLD X (zero skips activation), not type: active type is written1 independently. Track-order table supplies all16IDs0..15; pickup consumer C5DA indexes24*track+6*(RNG&3), proving384bytes46F3..4873. Track3 initializer uses3X/Y pairs at4EF9..4EFF. Track0 animation0..2F (init0,set2F,decrementnonzero) proves48six-byte tile tuples502E..514E; required track0 pointer514E..5150 points515E,count4 gives515E..5178. Other7pointer cells remain unsupported.

Pickup award bases7/3 plus oldcounter0..3 yield activatedstates3..10 (ordinary spawnedstates1/2), proving C8DB..C8EF award bases+8extra tilewords and C8F1..C905 states1..10tilewords; unconsumed state0cellC8EF..C8F1 staysunknown. Dynamic type1->2->3 with no increment from3 proves type-offset entries1..3atCDC0..CDC6; unusedentry0CDbe..CDC0 staysunknown. Type1timer sets2C then decrements before nonnegative read, so CDC6..CDF2 contains44Ydelta bytes0..2B. Animation initialized0, set11/1D/22 and decremented onlynonzero proves35tilewordsCDF2..CE38.

Window helper continuation also has independent count-bounded assets: two7byte indirect HDMA descriptors B608/B60F; three bank-byte/word shape pointers B616..B61F selected by explicit writers0(frontend),1(race),2(pause). B4C6 reads128left/right pairs, CPY0100 proves256bytes each at CPU08E29A/E39A/E49A (physical04629A..04659A). These concrete unknown resources are included instead of deferred for independence.

Combined replayable proposal builder `analysis/redirect/proposals.py` writes `evidence/round01-redirect-proposals.json`. Data/code proposed union conflicts are asserted absent. Existing-code crossings are only BIT/LDA operand-sharing starts178F/17F8; disassembly must consolidate/split preserving alternate entries. Multiplecompatible code views are preserved; no competing unknown instruction starts/lengths were found in a separate byte-owner audit. Manifest remains research-only; no map/classification/commentary changed. Remaining audio holes have unsupported selector/source semantics, remaining main frontiers have specific width/DB/DP or callee contract diagnostics; no newly identified productive consumer is deferred merely for scope. These remaining facts are legitimate future research starting points, not a claim of whole-ROM exhaustion.

## Round 2 disassembly acceptance and bounded followup

R1-L003/L004/L005 verified against independent code decode, producer/consumer contracts and replayed pose/song/BRR proofs. Accepted +103485unique bytes:100616Data/2869Code. All2457instruction views/1308distinct starts independently match opcode/length/saved assembly; complete contexts retained in round02-code-independent-inspection.json. Physical new code grouped into106exact-decode fragments, data split into305nonoverlapping read-window pieces. Terrain region IDs169/332 expanded to include BIT prefixes178F/17F8, preserving old LDA internal-entry metadata and commentary. Prior388regions are unchanged; old390readability credit remains reusable, with two entry expansions needing local editorial reconciliation. Whole commentary preserved before map changes at analysis/preserved/round01-commentary-final.

Data acceptance follows bounded consumer roles:25tiltwords/pose0..415 exact source union; fixed object/pickup/animation/window/rectangle/palette formats; five envelope vectors and selected command/song/config/directory/BRR windows. Config copies141bytes; sample20physical storage splits77E04/78004 around metadata. Unselected command8E/98/AC/BAcells, songs5..12, unselectedSRCNs and BRR tail beyondARAMFC27 remain unknown. Main unresolved width/callee-return/DB/DP barriers remain recorded in prior diagnostics. Static structure does not claim natural selection or opcode-fetch execution. This batch consumes the combined ready evidence; no obvious connected consumer was dropped merely at its initial lead boundary. Further unsupported domains require new selector/layout/return contracts, appropriate for subsequent redirect after commentary.

Targeted ordinary scenario:7112Start(port1)2/release30 produced no pause-range fetches. Contexts place it in D580/B4C6 race-entry window. Independent changed prerequisite: restore7112, releaseall, settle120frames in six20-frame calls, then Start2/release30 to7264.751retained Instruction events,0dropped/0aggregate_dropped;26distinct starts562E..5666allalreadyclassified banner renderer, artificial=false. No pause-wrapper56E1+fetch, no unknown target starts. Thus no pause reachability claim or structural gain; do not repeat unchanged inputs. Both attempts restore durable7112start. Initial40-frame settle exhausted1Moperations at7134; restore then20framechunks succeeded. The local pager incorrectly assumed total field; evidence was recovered from unchanged live trace via short-page termination before restoring. Scenario ledger now uses existingsegments schema. These are local driver corrections, not toolkit findings.

ROM classification: 81.24% (425933 / 524288 bytes classified; 98355 bytes unknown).

801regions:18217Code/407716Data;149unknownranges in round02-unknown-ranges.json. Per-bank counts round02-bank-counts.md. map.validate/save/export passed. Fresh round02-disassembly-final checkpoint ok=true/byte_identical=true; generated diagnostics empty. Full check report round02-disassembly-verification.json. Toolkit: existing nonblocking TI-001 missing discovery paging; no new issue/confirmed/suspected bug. ROM/toolkit unchanged. Disassembly pass completed; ROM incomplete; coordinator owns commentary advancement.

## Round 2 redirect active — new return contracts and effect streams

Input round02-commentary-final; no map/source writes. Prior summary-CFG diagnostics filtered against accepted map leave only11unknown-byte barriers:212A,22AC,2441,2A19,31B8,3CC2,3E22,46BF,4E64,4E7E,548A. Materially new method individually inspects unsummarized callee returns rather than replaying unchanged discovery: B46F balanced PHP/PLP, C873 balanced PHP/project969A/PLP, CE85 explicit SEP30 returns M8/X8, BB0E REP30 plus status-preserving paired text helper returns M16/X16; B426/87BB preserve flags. Research script analysis/redirect2/contracts.py starts from current classified primary views and retains contextual alternatives; evidence round02-redirect-contract-cfg.json/new-listing.txt. Initial closure exposes24uniqueunknownstarts/57bytes; only six unknown diagnostics remain. Not yet a finalized proposal; inspect descendants/contracts before packaging. No opcode-fetch claim.

Independent concrete audio avenue: SPC host command02 calls effect selector, indexY, reads pointer low1060+Y/high1070+Y and priority1080+Y. Main source independently encodes effect2(nitro0202),3(impact0302),5(award0502),6(nitroaward0602),7(dynamic contact0702),8(surface contact0802). Thus previous song-only parser left an actual supported consumer domain unexamined. New bounded analysis/redirect2/effects.py uses same inspected stream semantics with effect entries, writing separate round02-redirect-effectN.json without altering prior evidence. All six selected entries reach END in9..34tokens, no missing stack/unsupported command/budget stop. Effects5/6 consume previously unselected commandAC; effect3 selects SRCN27 and7/8selectSRCN28. Other SRCNs23/14 already selected. Connected followup now directory/BRR27/28, effect priority/pointer cells, main call/width continuations and remaining independent residuals. No natural reachability claim or map edit; input/toolkit/source preserved.

### R2 connected expansion milestone

Selected effects2/3/5/6/7/8 all terminate with ordinary80 END; effect5and6share stream438C. Main callsite immediate sweep confirms no additional selected effect index among inspected reachable main instructions. New SRCN27(BF5E..C83A,252BRRblocks) and28(C83A..D6D1,415blocks) each end with END+LOOP and loop exactly at start; physical source507234..513237 contiguous (upload second segment), two directory cells at506C..5073. Script analysis/redirect2/effect_brr.py/results round02-redirect-effect-brr-proposals.json. Other effect-selectedSRCNs14/23are already accepted. No arbitrary effect/song/table padding expansion.

Native caller closure improved by B61F explicit M8/X16 result (clear four money records), B63F PHP/PLP defaults,9C0D bounded fill PHP/PLP, B1B5 newly inspected PHP/DMA/fills/PLP, BADC text PHP/PLP,9A42fadeM8,8F70bannerPHP/PLP and A3CEwindowM8. This provides supported Efalse context along actual static call paths rather than asserting global native mode. New A293..A345 window geometry/scroll/growth loop reaches all normal M8 returns; A248 normal returns remain M8/X8, AC38 Start can nonlocally unwind and is not a return path. A2441resource continuation now calls previously unknown B91E; following it through C0DB font-upload PHP/PLP is active work. Do not stop this phase yet.

New B1B5..B256 DMA initializer independently bounds sources CPU088000+4D00,08CD00+0600,08D340+0100: physical40000..44D00,44D00..45300,45340..45440. Source setup/trigger instructions individually decoded with Efalse/M8or16/X16; data roles are VRAM graphics/tilemap/CGRAM palette respectively. Unknown-byte intersection19712+1536+256=21504before other proposals. Existing accepted source windows must stay as one physical role, no overlapping annotations. New descriptor consumer A2441uses CPU09CDA0 and B3C2; exact rows/columns proof pending inspection with connected B91E graph.

Independent residual check: track-zero composite initializer explicitly branches on selected_track_id!=0 before the indexed pointer read, so only pointer514E is supported by current consumer; remaining seven cells have no independent extent proof. Dynamic type and pickup-zero cells likewise excluded by independently proved active-state domains; neither adjacency nor zero shape changes that barrier. No unchanged driving/Start route was repeated. Read-only machine inspection confirms durable7112 artificial=false; initial machine state saved round02-redirect-initial-machine.json. All current source/map/input untouched.

### R2 high-score and countdown resource closure

B91E..BA10 is a complete high-score display routine, preserving entry status: fontC0DB/textBADC helpers, ten score rows with six digits and leading-zero handling, then ten three-character names; balanced PHY/PLY and finalPLP/RTS. New caller A2441..A49C draws the existing heading rectangle, invokes B91E, scrolls BG3 until cap01D6 while polling Start/500frame timer, then contracts window. Exact listing in round02-redirect-new-listing.txt. Its unknown data: first-FF headingBA11..BA1B and ten word destination arraysBA1B..BA2F/BA2F..BA43. No more whollyunknown directcallees remain in supported graph. Three diagnostic addresses22AC/2441/31B8persist only under weaker alternative contexts; all now have separately supported native instruction views, not blanket assumed native state.

Connected countdown format: D429init18, D548Acompare34/reset18; D54B8/D54DD equivalent guards; D518init0 and readyloop threshold17. With stride3 D62E consumer and exactly adjacent20byte D6CD descriptor, table5667..56CD is independently bounded34record layout, not pointer resemblance or claim every index naturally occurs. Descriptor width8007/height7 means7x7map (highbit bypass palette remap),98bytes per supported source. Declared layout includes duplicate pointers. Track0..15domain proves16byte centersD5EE/D5FE and16wordVRAMdestinationsD60E..D62E. Main attract selector AND03 additionally proves4bytes20DE..20E2. All read windows inspected from local ROM, saved resources.py/data-windows.json. Existing accepted heading map/descriptor already cover A2441 rectangle; no duplicate role proposed.

Combined current research manifest round02-redirect-proposals.json contains785newCodebytes (352distinctstarts,362views),26668newData bytes in20unknown-complement intervals,27453unique newunionbytes. Physical code-byte ownership audit found no competing starts/lengths, no crossing current roles and no code/data conflict. This is a research proposal, not accepted classification. Current map remains425933classified/98355unknown. Productive natural pause test is still active: new initializer/ready source shows growth+17*6framecountdown+contraction exceeds prior120frames;240frame bounded settle in20framecalls thenStart tests the actual completed transition prerequisite. Already observed state0,0,2,5,8,12,15,18 at20..160frames; unchanged old negative results remain valid. Final trace/drop/natural result pending; source/map remain untouched.

### R2 natural pause result and residual assessment

Completed7112->7352 via12x20frames, Start2/release20 ->7374, artificial=false throughout. Actual aggregate opcode-fetch contexts prove96physicalpause/transition starts/127processorviews in56E1..57FA. Retained detail20kInstruction events are tight new-press wait-loop5746/5749/574B, all alreadyclassified. Detail44891events dropped; aggregate_dropped0. Thus positive natural pause entry and transition, incomplete detail history, no new unknown pause starts, and no claim every resume path executed. Finally trace disabled and durable7112snapshot imported successfully. No repeat warranted. Scenario/gameplay updated. Read-only context pager initially treated terminal next_offset=total as nullable; interrupted local query and fixed short/nonadvancing/total termination. No emulator change or toolkit defect; aggregate evidence preserved.

Other concrete avenues assessed: songs5..12 and effects not in2/3/5/6/7/8 still lack a host-selector or independent declared-layout extent; command8E/98/BA still have no selected token (ACnowproved). New effects introduce onlySRCNs27/28; no operand selects residual SRCNs or finalARAMFC27..FFFF. Format appearance alone still cannot assign that tail. Auxiliary98handler is now readable but no selected98operand supplies its pointer, so arbitrary auxiliary parsing is not justified. Typezero and extra trackzero pointer cells remain explicitly excluded by active/track guards; no new independent consumer. The main connected unknown graph now closes all eleven former width barriers in supported views, its newly discovered B91Ecallee and obvious associated data; weak alternatecontextdiagnostics retained. No whollyunknown directcallee remains; further unsummarized callees have not furnished a new unsupported byte boundary under current graph. Unentered gaps need an independent entry/format contract, not an arbitrary sweep. These are actual missing facts/reducedprospect reasons; no productive continuation intentionally deferred for independence or a byte quota. Readybatch plus exact residuals should pass to next separate disassembly phase; coordinator owns advancement.

## Round 3 disassembly acceptance — verified combined R2 batch

R2-L001/R2-L002verified and classified +27453unique bytes (785Code/26668Data),77new physical regions (12Code/65Data). All801previous physical roles unchanged. Every362proposed view/352start independently decoded;581producer/consumer context views independently checked. Exact main roles include return-width/frontier continuations, B91Ehighscore display/heading/ten-row arrays, B1B5bank08DMA sources, attract-mask3slots,16trackcenter/destination arrays,34stride3countdown records/common20byte7x7descriptor and98byte tile sources. All effect2/3/5/6/7/8semantic results replay exactly, reachingEND; selected ACvector/priority/pointers/directory/BRR windows accepted without padding or domain expansion. Complete evidence round03-independent-validation.json, round03-consumer-inspection.json, round03-effect-proof-validation.json, round03-accepted-plan.json and round03-acceptance-log.json. No new static observation is called opcode-fetch coverage.

Correction: SRCN27/28END headers areB1/C1, with LOOP clear. Directory loop words equal aligned start but are inactive. END extents remainBF5E..C83A(252blocks)/C83A..D6D1(415blocks), preserved upload transformations. Original redirect evidence retained; acceptance metadata and handoff supersede its END+LOOP prose. Local initial producer inspection draft crossed calls with unsuitable linear contexts; replaced with per-instruction supported context views before acceptance. No invalid draft was annotated.

Useful ordinary alternative after countdown prerequisite:7112release+240settle to7352, four20frame steps with B+Y+Right/B+Right/B+Left/B to7432. Natural/artificial=false;3275ROMviews/2415ROMstarts,629ARAMstarts; zero unknown ROMcontexts. Aggregate drops0, detailcapacity0 intentionally dropped1128452events. No new structural gain; restored original7112and disabled trace. Prior natural pause positive evidence reused; no unchanged earlyStart retry. Exact calls/results in round03-second-track-drive-route/result/contexts.json and scenarios ledger. Repetitive short driving now has reduced prospect; this pass does not claim gameplay exhaustion.

Ready evidence and its supported connected continuations consumed. Existing actual residual barriers persist: songs5..12/unselectedeffectIDs lack independent selector/declared-extent evidence; command8E/98/BAlackselectedtokens,98auxpointerabsent; remainingSRCNs/ARAMFC27..FFFFlack consumer/bounds. Explicit active/track guards exclude typezero/pickupzero/extra trackzero cells. Remaining isolated main/SPC gaps need independent entry/context/format proofs; weak diagnostic alternatives remain in original contractCFG, not silently overwritten by native contexts. No whollyunknown connected directcallee remains in supported closure, and no supported resource was deferred merely at lead boundary. Further research belongs to coordinator-selected redirect after current-source commentary, not speculative domain expansion.

ROM classification: 86.48% (453386 / 524288 bytes classified; 70902 bytes unknown).

878regions:19002Code/434384Data;144unknownranges in round03-unknown-ranges.json, exact perbank counts round03-bank-counts.md. All878regions have labels/comments/evidence. Full42file commentary tree preserved at analysis/preserved/round02-commentary-final/commentary; all hashes match live. Old801region editorial credit remains;77newregions need commentary, final archive audit pending. Current map/internal state SHA2562a2cdc090c6e3405b6dfc21592ad6b5f1050c6fa1871e4b852d8ae7bc91fe031. map.validate/save/export and fresh round03-disassembly-final checkpoint passed, byte_identical=true, generateddiagnostics[]. Verification round03-disassembly-verification.json. Toolkit existing nonblocking TI-001 missing paging, no new/confirmed/suspectedissue; report toolkit-issues.md. Input/toolkit unchanged. Pass complete; ROM incomplete. Threshold ineligible70902>16384 (ratio0.13523483276367188).

Round3 exact physical-bank counts:

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|16862|5793|10113|
|01|0|32768|0|
|02|0|32768|0|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|24818|7950|
|08|0|27552|5216|
|09|0|32656|112|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27682|5086|
|0E|2140|15983|14645|
|0F|0|31279|1489|


## Round 3 redirect active — explicit transfer-source audit
Input round03-commentary-final. New method: compare **all identifiable commented source transfer setups**, not only new instructions reached by a root-based return CFG, against the exact residual complement. This differs from unchanged audio selector/gameplay retries. Concrete first result: source `transfer_race_entry_map` (physical26CB..26E9, CPUA6CB..A6E9) explicitly writes DMA1 source bank07, lowwordDAD2 and length0DC0, triggers MDMAEN02; ROM physical03DAD2..03E892 is exactly3520unknownbytes. Need independently decode setup and validate increment mode/no bank wrap before ready proposal. Connected continuation is post-race count/window assets; inspect other existing source DMA/count consumers for omitted supported resources. Complete-layout audio review confirms SPC host song selector is an unrestricted port byte with DEC before config table indexing; no mask/count proves songs5..12 from loader alone. Source arrangement/pointer appearance is insufficient; no speculative whole-table expansion.

### R3 transfer audit result and expanded source-reference frontier
Post-race DMA bank07/sourceDAD2/length0DC0/mode01/VRAM18 is independently present in encoded setup and does not wrap ($DAD2+$0DC0=$E892). Ready data proposal physical03DAD2..03E892,3520bytes; no natural-fetch claim. Initial decode draft started in an operand atA6AC and was discarded; exact setup beginsA6B2, bounded decode will replace it. Full commented-source literal/symbol-reference sweep identifies additional **existing consumers whose data remained unknown**, unlike the prior new-CFG-only resource extraction: start headings868B, sprite streams8804/8823, AIturn8989, direction/sine9CC7/9DC7, palette9EC7, direction tables9438/9458, wheel offsets996A, trackcountdown8F38, frontend signatureA1AE, result palette offsetsAA3D and their08EC1A sources, text/cursor/grid/upgrade/money/placement tables. These have direct meaningful source consumers and concrete domain/terminator/count recipes, so same-phase investigation continues; not deferred as independent.

### R3 supported resource batch checkpoint
Replayable research-only `analysis/redirect3/resources.py` now emits292consumer/format windows and exact unknown-complement summary: **5436 proposed unknownbytes**, including3520post-race map and288result-palette bytes. Sources are current commented assembly and local immutable image; no classification edits. Includes bounded start-heading records, four AIturn bytes, four quadrant bytes, sixteen track countdownwords,16directionwordpairs, fourwheel offsets, modulo256sine, selected signed-nibble directioncells, independently initialized sentinel-bounded sprite streams, fivebytefrontend signature, participation masks, podium awards/destinations, three resultpalette offsets, terminated prompt/shop/name text, ninecursorgeometry/tilewords, fivecost/cap/redraw entries, moneyoffsets,35cellnameeditorX/Y/glyphs and56placementanimationwords.
Exclusions are explicit: signedvector nibble8 row/column unsupported; sprite stream after firstFF atcursor23 remainsunselected despite adjacent bytes; odd sparseOAMmaskcells unselected; podiumawardfourthbyte notread. Palette source009EC7+0200 **overreads into existing code A037..A0C7**: existing code retained; source role is transfer read-window, not an asserted independent512byte paletteformat. This nuance requires independent disassembly review. Candidate windows/results in round03-redirect-resource-windows.json/resource-summary.json. Next: validate encodedconsumercontexts, follow upgrade/AIboundwriters and fixed-length callreturnfrontiers; remaining padding and unrestricted audioselectors still lack independent role/extent proof.

### R3 decisive fixed-length continuation — missing song producer found
New instruction-specific method: inspect the **physical fallthrough immediately after the classified normal-return call**, allowing only context needed by each instruction instead of requiring a whole caller root to survive every earlier unknown call. After current CPU817B JSRD518, source D518..D5ED has a normal RTS, balanced calls and no E/bank/direct-page changes. Physical017E..0198 decodes exactly26bytes: JSR9A64; SEP30; LDA06; BEQ0196; LDA1C; AND07; CLC; ADC06; XBA; LDA01; REP20; STA2142; REP20. All immediate lengths locally resolved by SEP30; Efalse for16bit hostcommand store inherited from accepted native817B and callee absenceofXCE. Durable exact decode round03-redirect-017e-decode.json. Staticfallthrough proves role, not observedexecution.
This is the missing main song producer: onhumanplay, (track_order_slot&7)+6 sends hostmusicselector6..13 to APUports2/3. SPC0440 decrements hostselector before recording/loading song, independently proving songs5..12. Prior unchanged selector barrier is **reopened by new encoded producer evidence**, not a renamed/repeated guess. Connected work now must parse these eight songs and follow selected commandvectors, instrument/directory/BRR and auxiliary streams as supported. Do not hand off only26bytes or first5436resourcebatch while this productive graph remains.

### R3 new-song connected result checkpoint
All64voice routes of newlyprovedsongs5..12 parse toENDor repeated semanticstates, with no unsupported opcode/stack/budget stop. New **command98 vector** selected by songs8and11; its auxiliary pointers areARAM2B15 and3167. Followed081F/readhelperC1F/releasephase contract: both exact19byte two-phase streams; 2B15markersFF/FF,3167markers80/FF. Physical source transforms retained. Newlyselected SRCNs4/6/11 have firstEND BRR chains and directoryfields; previouslyselectedSRCNs retained assharedroles. `analysis/redirect3/audio.py` emits320windows, **10079unknownbytes**; includes selectedsongpointercells, eight141byte configurationcopies, streambyte-useunions,98vector, auxiliarystreams, SRCNdirectory/note-base/scale cells, BRRwindows and16explicitvoice-mask/complement bytes. TerminalEND/LOOPbits recorded individually, never assumed paired. Existing sample20uploadcut still excludes77E04..78004metadata. Commands8E/BAstillunselected; no new98unsupportedpointerbarrier remains.
Currentresource+audio candidates total15515Data +26Code before final ownershipaudit. Continuing concrete pitchconsumer-state method (selected notes plus instrumentbase/transpose and envelopeinterval contracts) and upgrade/domain writer checks; no unchangedplayback required.

### R3 pitch-state/upgrade results and next frontier
New selected-note method `analysis/redirect3/pitch.py` tracks instrument0, transpose0, skipflag0 and envelopeinterval0 from driver clear; models92/90/C0/A8/AA updates plus existing sequence stack/timing semantics. For nonzero notes when intervalzero,092D directlycalls0984with index(note+instrumentbase+optionaltranspose)&FF. Parsed128songvoices yield15715explicitnoteproofs and68indices; low0CED+Y/high0D42+Y proposes134currentlyunknownbytes (some overlap16voice-maskbytes). Exact sparsecells only; no blanket85/256pitchtableextent. Includes highindices249/254 from modulo arithmetic, retainedasactualcomputedreads ratherthan silentlyclamped. Existing splituploadassociation retained; newsourcefields aren'tnaturalplaybackevidence. Intervalnonzero/tick-dependent pitches requireadditional envelope/timingcontract and are not guessed.
Upgradewriter review adds46resourcebytes:9speedlimits (AIparameter<=7 plus explicitspecialindex8),7accelerationwords (reset/shopcap6/AIexplicit6),7steeringrates(shopcap6),8AIroute-timeouts(parameterclamp7),two4bytehuman-relativerankthresholdarrays. Slot/itemcaps and actualwriters checked; unsupportedAI-timeouttail8..15 remainsunknown. Resourcebatchnow5482bytes. Scriptsandresults durable. Next final main-code/reference audit uses accepted currentregiondecodes to find any missed directtarget/fallthrough entry, ratherthan assuming earlier root-CFG traversed entireclassifiedregions; follows only evidenced entries. RemainingisolatedRET, padding, unselectedcommands8E/BAandSRCNs have no currentnewproducer. Need complete this concrete audit before handoff.

### R3 final current-code audit result
`analysis/redirect3/audit.py` independently decoded all447currentCodeprimaryviews (8793instructions), with exactregioncoverage/errors[]. No direct target points into an unknownbyte. Only two accepted regionends have nonterminal fallthrough into unknown:017E and3C3F. The latter follows BC3C JSRBABC; BABC's ordinary terminatedtext PHP/PLP preservesX16, giving9bytes BC3F LDY#BA9E / BC42 LDX#491B / BC45 JSRBABC before existingBC48. This also selects the two-byte BA9E..BAA0 terminatedtext previouslyunreferenced. Exactdecode round03-redirect-3c3f-decode.json; no additional entry/boundary needs inventing.
Combined `analysis/redirect3/proposals.py` manifest now **15741newbytes =35Code/15706Data**,16instructionviews, exact currentunknowncomplement; source-read overlaps with existingCode explicitly retained ratherthan relabeled. Pitchproofnowalsoincludesselectedinstrumentbase/scale reads, including defaultinstrument0; union counts accountforoverlaps. Proposedresidual55161bytes, not currentclassification. Mainreturncontracts/producer+consumers, all8new songs,98auxiliary/vectors,3newSRCNchains and obviousassociatedresources have been followed as onebatch. No map/source/emulator edit. Remaining deferred checks/reasons and hash/verification report pending before handoff.

### R3 callerless contract inspection outcome
Three finite unknownspans have independent semantic routine contracts beyond mere validdecoding/adjacency: (1)1A6F..1A78,9bytes, balancedPHP/PLP calls known status/X-preserving9A64 framewait, DEX/BNEbackedge thenRTS; count contract depends on entryXwidth/value, but **allinstructionlengthsfixed**, so neither a livecaller nor guessedM/Xneeded. (2)1BAB..1BCA,31bytes: PHA/PHX/PHP wrapper SEP30calls9BB7 thenrestoresP/X/A/RTS; internal9BB7 savesbyteA, shiftsfourtimes, calls9BC2, restoresA, AND0F and falls9BC2. That helper loadsDPcursor88, incrementsglyphA, storesD+89+X andincrementscursor. SEP30 suppliesbodyM8/X8 independentlyofE. Completebalancedwrapper/internalcallgraph establishes twohexnibble glyphwriter role; reachabilityunknown. (3)1116..1139,35bytes: SEP20 initializesDPoutputcursor2; reads04E9and04F1, XBA-separatedpairs ofJSR9BAB appendtwo-bytefieldhexforms, addingseparatorcursorpositions, thenRTS. Everyinstructionboundaryisfixed or immediateafterSEP20; noE/Mguessneededforclassification. NativewordinterpretationofREP20fieldreadsisconditionalonEfalse; **do not invent entryE/DB/DP**. Source aliases remainnumericwherebank/directpageunknown. Separateexactdecodes saved `round03-redirect-isolated-{wait,helper,helper-body,debug}-decode.json`. No intendedexecution/callers claimed; proposedStatic mechanicalrolecontracts, confidence0.95, require independentdisassemblyvalidation. These75additionalcodebytes complete this concretecontractavenue; theircalls/resources close in known9A64/9BABhelpers andRAMoutput, so no furtherROMextentexpansion.

## Round 3 redirect complete — combined ready batch
Finalcombinedmanifest **15816newunique proposedbytes:110Code/15706Data,61instructionviews**. NewreadyIDsR3-L001/L002/L003; oldverifiedR1-L004/L005historyretained with explicitnewmethod/followuprecords, R2consumedbatchunchanged. Fullpersistentleadsandpassentrysaved. Resource/currentcode/source/copy-length/complete-layout/selected-note methods all produced eitherreadyproof or explicitnegative/missingprerequisite in `round03-redirect-residual-assessment.json`. Noidentifiedpromisingactionablecontinuationintentionallydeferredforindependence/firstlead/quota; connectednewmusic/aux/BRR/pitchgraph and otherconcretecurrent-sourceconsumersfollowed. Remainingpadding/unalignedentry/unselectedoperand/sparsepitch domain barriers are specific, not a demandforeverybranchtoexecute. Current70902unknownbytes/144ranges unchanged; proposedacceptancewouldleave55086unknown, stillineligiblefor16384final-investigationschedulingthreshold. Coordinatorownsnextphase.
Verification: map.validateok; freshround03-redirect-final structuralcheckpoint validates and rebuilds byte-identically, diagnostics[]. Fresh **edited commentary/source** verifyok/byte_identicaltrue;13handoffartifacthashesallmatch. `analysis/round03-redirect-verification.json` combines preservation/statistics/decode/map/source/checkpointproof. No map/source/input/toolkit modification, no runtimeorcoverage/scenariostate change. Gameplaylocalstaticmusicfindingadded; priorroutespreserved. Finalarchiveauditstillpendingseparate. ToolkitexistingTI-001only/nonblocking; no newtoolkitissue. Workerstop-writesafterhandoff.


## Round4 disassembly independent acceptance
ROM classification: 89.48% (469143 / 524288 bytes classified; 55145 bytes unknown).
AcceptedR3-L001/L002/L003 +15757unique bytes(110Code/15647Data),363newphysicalroles;878oldrolesunchanged. Exact61instructionviewinspection and6mechanicalcontracts passed; all447oldcodeviews/8793instructions stillcoverexactly. Independentparser matches8newsongunions,128voiceboundedstops,15715noteproofs/68indices,firstENDsampleboundsandtwo19byteauxstreams. See round04/audio-validation.json/code-inspection.json/current-code-inspection.json. No staticclaimcalledObserved.

Resourcecorrection:868B/9438/9458consumerloadsareM8atevenindices. Reject64skippedoddbytesfromredirectfullwordproposal; pattern/alignmentnotformatproof. AdditionalA687..A68FbyteconsumerindexequalboundedAIparameter0..7 independentlyaddsfirst5unknownbytes; other3alreadyinacceptedrankthresholdwindow. A037..A0C7remainsCodeunderpalette-readwindow; sparsemask/nibble8/sprite-tail/awardfourthbyte/AI-timeouttail exclusionspreserved. Exactcorrectedplanround04/accepted-plan.json andcorrections.md. Remaining64oddbytesneednewindependentrecord/wordconsumerproof; no arbitraryM16executionclaim.

Freshmapvalidation/save/exportandround04-disassembly-finalcheckpointpassed/byte_identicaltrue/diagnostics[]. All38preservedcommentaryfilehashesmatchandprior editedsourcefreshbuildexact;363newrolespendingcommentary,878oldcreditreused. Residual55145bytes/248rangesinround04-unknown-ranges.json; perbankround04-bank-counts.md. Existinground03-redirect-residual-assessmentbarriersretainedexceptacceptedA687thresholdandnewoddbyteexclusions; nochangedproductivegameplayprerequisite, no unchangedearlyStart/shortdriveretry. Natural7112stateandtracedropcountersunchanged. No readynewlead; coordinatorrunscommentary/redirect. Finalinvestigationthresholdineligible. ToolkitTI-001nonblockingonly; no newtoolkitfinding.

## Round 4 redirect active
R4-A1: selected interval-nonzero pitch: derive exact update ordering and a narrow first-note/tick contract rather than infer full sparse table extent. R4-A2: complete main/SPC direct-edge and fallthrough audit with uploaded alias transforms, then inspect actual unknown targets only. R4-A3: independently reconcile upload record lengths/terminators and unknown tail roles; R4-A4: excluded odd byte tables require actual different-width consumer, not adjacency. No runtime mutation or map edit authorized in this phase.

## Round 4 redirect complete — coherent ready batch
Input round04-commentary-final; unchanged output structuralcheckpoint round04-redirect-final. Researchonly; readyR4-L001 proposes25Data bytes, noCode:19temporal songpitch cells,2selectedeffectpitchcells,2instrumentbase+2scalecells. ConnectedAC071C-before0742method follows128songroutes(1375deduplicatedwitnesses,78indices) and6selectedeffectroutes(14witnesses); all stopEND/reload/repeatedcompletepitchstate. Eightboundedartificialhelperchecks pass, originaldurablenatural7112restored. No newbuttons/naturalcoverageclaim.
Independentall486executionview/8978instructionviewaudit exact/errors[],0effectiveunknownedges/references. SoleJMPFFC0is mappedIPL, notROMtail. Encodeduploadlength/zeroterminatorcontracts leave504bytepostterminatorblocksand985byteARAMtailwithoutnewinternalroleproof. Threebytewidth/evenindexconsumersdo notsupport64oddheading/menu cells; retainedunknown. Otherconcreteavenues/reasonsin round04-redirect-residual-assessment.json: no newselectedoperands/declaredextent/entry/containerfacts; no unchangedgameplayretry or outside-scope deferral.
ROM remainsincomplete:469143/524288classified(89.48%),55145unknown/248intervals;19112Code/450031Data/1241roles. Proposedacceptancewouldleave55120unknown. Bothabove16384;finalinvestigationschedulingineligible. Current-sourcecommentarygatecomplete;finaleditorial/archiveauditpending.
Freshmapvalidationandround04-redirect-finalstructuralcheckpointpass/byte_identicaltrue/diagnostics[]. Fresh editedsourceverifyalsoexact. All15priorcommentaryhandoffartifacthashesmatch;map/state/source/inputunchanged. Combinedproofround04-redirect-verification.json. LivepredecessortraceaggregateslostinlocalcollectorOOM;durabletracesretained,newconfiguredreleaseserverrestoredmap/natural7112. Newservertraceempty/disabled;noaggregatepreservationclaim. LocalOOMnottoolkitbug.
ToolkitexistingTI-001only:1nonblockingmissingfeature,0confirmedbugs/0suspectedbugs/0newfindings;analysis/toolkit-issues.mdupdated. [Compacthandoff](round04-redirect-handoff.md),[proposals](evidence/round04-redirect-proposals.json),[residualassessment](evidence/round04-redirect-residual-assessment.json),[verification](round04-redirect-verification.json). Phasecomplete;coordinatorownsnextstep. Workerstopwritesafterhandoff;nolaterphase/workerlaunched.


## Round5 disassembly complete — ROM incomplete
ROM classification: 89.49% (469168 / 524288 bytes classified; 55120 bytes unknown).

Code19112/Data450056;1266physicalroles;244unknownranges. Exact residual ledger: analysis/round05-unknown-ranges.json.

Independent consumer replay follows all128 selected songvoices and six supported effects to END/reload/repeated complete pitchstate (134 bounded stops). Reinspected actual SPC700 opcodes at04E7/068A/071C/092D/09F0/0A54 with DP0, diagnostics[]. Sequence tokens and operands are all in prior supported stream roles. Source clear068A plus note092D and tick06DB prove first-rest index0/song1voice6; song2voice0 ARAM1BD7 descent48 toward3 by2 proves sparse intermediate cells. AC071C updates target before0742 slew. All25 manifest cells independently reproduced, no additional unknown read surfaced. Effect entry does not clear current/instrument/every flag: all six reviewed prefixes set92instrument and reach a first nonzero intervalzero note before dependent reads; effects5/6 configure AC before that note, which is safe because the immediate pitch calculation/snapping precedes the tick update. Transpose0; one-shot skip cannot affect index. Fixed0 makes first duration read independent of stale one-shot duration state, and that read clears the one-shot flag. No unsupported blanket effect-state clear claim.

Accepted R4-L001: +25 Data, no Code:19songpitch+2effectpitch+2instrumentbase+2scale. Exact one-byte roles retain unknown neighbors; confirmed first upload ROM70004..77E04 -> ARAM0400..8200/DP0/physical=ARAM+6FC04. Static structural evidence; no new natural playback, runtime inputs, patches or execution coverage. Artificial predecessor tests are mechanics evidence only. Natural durable7112/artificialfalse remains; newserver traceempty/disabled, predecessor live aggregates lost but durable traces preserved.

Prior1241roles unchanged; all38files of round04 commentary preserved and hashes matched BEFORE changedmap checkpoint generation and again afterward. Reuse1241role readability credits;25newData roles pending commentary, full-source integration still required nextphase. Fresh preserved editedsource verification exact; independent structural checkpoint round05-disassembly-final exact/diagnostics[]; map.validate/saveanalysis/state.json/exportmaps/current.json pass. No commentary baseline/prep overwritten. All1266roles have label/comment/evidence. Evidence: round05/independent-temporal-review.json, consumer-inspection.json, cell-inspection.json, acceptance-log.json; combined round05-disassembly-verification.json.

No productive unchanged gameplay retry: prior completeview/directreference audit486views/8978instructions establishes no effective unknown edge; code roles unchanged. IPLFFC0 remains mappedIPL, not ROMtail. All64oddheading/menu cells remainunknown. Residual barriers unchanged beyond accepted cells: unselectedcommands/effects/SRCNs need new selected operands or independent format; isolated gaps need entry/context/role proof; upload tails need internal boundedformat/sourcecontract; graphics tails lack container/sourcecount evidence; guarded zero cells retain negativeconsumer evidence. Connected temporal consumers and all six supported effects were reviewed through their bounded stops, not deferred at first lead. No additional ready actionable lead; coordinator owns commentary then redirect. Final-investigation scheduling ineligible55120>16384; full-ROM classification/final editorial/archive gates incomplete.

Toolkit: sole existing nonblocking missing feature TI-001(discovery paging),0newfindings/0confirmedbugs/0suspectedbugs. Local review draft required removing an unjustified ACzero assertion for effect5/6 prefixes; the actual first-note ordering resolves this, not a toolkit defect. See analysis/toolkit-issues.md. Toolkit/input unchanged; no rebuild. Worker stops writes after handoff; no laterphase/worker launched.

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|16972|7410|8386|
|01|0|32768|0|
|02|0|32768|0|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|28338|4430|
|08|0|27840|4928|
|09|0|32656|112|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27682|5086|
|0E|2140|26230|4398|
|0F|0|31279|1489|

## Round5 redirect active milestone — two concrete residual candidates
Direct raw-preservation source inspection found two specific semantic contracts not individually documented in prior residual notes. R5-A1 physical141F..1438:25byte status-preserving nibble-to-paired-delta routine under nativeEfalse, explicitREP20/SEP10, reads9438/9458 andwritesDP11/13. Unlike threepreviousbyteconsumers its nativeviewreadswords; Efalse/DB0 remain unanchored so nooddcellclaim. Etruealternative hasBRKat1429. R5-A2 physical03E1..0409:40byte collision speed/heading variation endingJMP8492; localimmediateboundariesrequireM8withnolocalSEP. Knownstate/targetassociationisstrongerthanisolateddecodingbutentrycontextmustbeexamined. AllknownmaincodeE/DB/DPwritersarebootXCE1/PLD8/PLB12plusRTI1C06; noadditionaldirectedgefrompriorallviewaudit. Exactmissingfactsandartifacts: evidence/round05-redirect-candidate-milestone.json. Phaseactive, nohandoff/noclassification.

## Round5 redirect completed — no ready proposal
New specific rawgap methods were investigated beyond prior classified-code audit: R5-L001141F..1438 conditional native paired-word directionconversion and R5-L00203E1..0409 conditionalM8 collisionresponse. Exact alternative decodes, knownE/DB/DPwriter review, target-byte searches and fixedboundarytests supplied useful constraints but no independent entry context. Native Efalse/DB0 at141F and M8 at03E1 remain missing facts; current knownbootnative/DB0 invariant is not an entryproof.0403boundaryisoperandunderM16 andcannotbeindependentlypromotedfromfixeddecode. No oddcell/wholearray/code speculation. Naturalcaller absence alone does not block an independentlysupported staticcontract; these candidates lack relevant boundary/context evidence. New blockedhistories retained; ready[].
All other concrete residual avenues assessed in evidence/round05-redirect-residual-assessment.json against R3/R4 methods: currentmapdelta is25consumedonebyteDataonly, alloldroles/codeviews/producerdomainsunchanged. No credible improved wholeextent/initializer/transfer/container/gameplay method emerged oncurrentevidence. ConsumedAC/sixeffects, unchangedselectors/startup/driving and priorfullCFG/DMA audits notrepeated. Researchexhausted undercurrentpremises, not wholeROMcomplete.
Fresh map.validate/save/export and round05-redirect-final checkpoint pass/byte_identicaltrue/diagnostics[]; fresh editedcommentary/source verifyexact. All14commentaryhandoff hashesmatch; exactledgerrecomputed55120bytes/244ranges. Current-source readabilitycomplete, finaleditorial/archiveauditpending. Natural7112 imported afterstaleRPCrestart; traceempty/disabled, no newcoverage. ToolkitTI-001soleexistingnonblockingmissingfeature;0newfindings/0confirmedbugs/0suspectedbugs. Verificationround05-redirect-verification.json; compacthandoffround05-redirect-handoff.md. Phasecomplete;nootherworker/laterphase.


## Round6 disassembly COMPLETE — ROM incomplete
ROM classification: 97.80% (512774 / 524288 bytes classified; 11514 bytes unknown).

Code19239/Data450100/Padding43435;1313roles;199unknownranges. Exactpercentage97.80387878417969. Inputround05-redirect-reopened-final/outputround06-disassembly-final; editorialround05-commentary-final.

Independently accepted all six ready R5-L001..006, exact47 physicalroles/43606bytes:43435Padding+127Code+44Data. Existing1266roles byte-for-byte metadata/view unchanged; no overlap/split required because proposals occupy exact unknown gaps. All1313roles retain label/comment/evidence. Static provenance for every newrole; no observed entry, actual arrival status, DB0 or natural gameplay claim.

NineFFtails independently recomputed:00609C..007FC0,026C00..028000,037980..038000,03EEB2..040000,046D3A..048000,04FF90..050000,056D9A..058000,05DAC1..060000,06EC22..070000. For each: exactFF/hash plus independent terminating predecessor code/consumer end plus validated header/32KiB allocation edge and established next object. Repeated layout corroborates. Re-screened4106main references from unchanged453Code/486views and explicit source-window manifests: zero competinghits/overlaps. Source palette/terrain/bg/countdown pointer/descriptor operands independently rechecked against ROM. Boundednegative screen is not proof of universalunread; alternativeSBC/graphics/BRR interpretations have no established object contract here. Historical linker identity/unusedstatus unclaimed. Mixed02EB72..030000 remainsunknown; sixzeros and absent02EB78independentboundary prevent transplanting fillproof.

Native141F..1438 balancedPHP/PLP/RTS and localREP20/SEP10 justify conditionalE0 structure with incomingM/X/DB/DP otherwise unconstrained. Separate inferredDB00 association accepts32highbytes of16word tables9438/9458: boundedwordloads, samebase independently acceptedM8consumer and exactsigned directiontruth table. Remaining32oddheading cells remainunknown. M8collision03E1..0409 has closed19instructiongraph, no candidate stackeffect and known8492tail; minusstore03F8/BRA0406, plusstore0403/merge0406. DB/DP only affect namedsemantics. AlternativesEtrue/M16 retained as nonsupported structuralviews, not globally impossible arrivals.

Two25byte money siblings36E8/3701 have localSEP30, PHXoriginalwidth/PHP thenPLPbeforePLX; exact371Asibling/table/normalizerinterface supports structure. Correctedcontract: PLXoverrides restoredN/Z, andSEP30clearsYhigh ifincomingnativeX16. DirectionSEP10also clearsYhigh; do not claimYunchanged. Existingaccepted371A sibling prose also needs this reconciliation during commentary; its oldphysicalrole remainsunchanged. Native12byte3457stub usesREP10/literal16bitargs/knownBABCparser/RTS; separateinferredDB00inline12byte format includesfirstFF346E beforeknownnextcode346F. All fresh decodesdiagnostics[], exactends. Evidence round06/independent-review.json, independent-decodes.json, proposal-bytes.json, accepted-plan.json, acceptance-log.json.

Entirecurrent37file commentary preserved/hashverified BEFORE annotations/generation and again afterward at analysis/preserved/round05-commentary-final. Earlier38file count differs onlyby absentcurrentsequence-layout.json; historicalfile remains in priorpreservation and additionallyround06/historical-sequence-layout.json. No stale metadata restored. Reuse1266unchangedrole reviewcredits;47newroles and affectedconsumercontracts require nextcommentary integration. Fresh existingeditedsourceverifyok/byte_identicaltrue; current source intentionallystill oldmap until separatecommentary. No baseline/prep/source overwritten. Newstructuralcheckpoint round06-disassembly-final validates/byte_identicaltrue/diagnostics[]; map.savedanalysis/state.json/exportmaps/current.json. Combinedverification round06-disassembly-verification.json.

No emulatorstarted/newexecution/input/release/patch/snapshot/trace changes. No coveragegain claimed. Saved natural7112 scenario remains prior evidence; no newruntime route or actualentryclaim. Boundedstaticchecks settle readyclaims without artificialexecution. Existingconnectedcontinuations inspected through tables,collisiontail,moneysibling/normalizer andtextparser; no additional new selecteddomain or productive unchangedgameplay prerequisite. Residual audio/heading/upload/mixedgraphics/guardedzero questions retain explicit missingformat/producer/allocation facts in round05-redirect-reopened-residual-assessment.json and exactnew199range ledger round06-unknown-ranges.json. Do not repeat unchangedStart/drive/arbitraryselector methods.

ROM incomplete; exact11514/524288=0.021961212158203125, both final-investigation thresholds met. Coordinator owns subsequent fullcurrent-sourcecommentary thenredirect and eligiblefinal-investigation scheduling; this worker launches no laterphase. Finaleditorial/archiveacceptance pending. Toolkit: no new toolkit issues found; historicalTI-001fixed/confirmedconfiguredrelease,0activefindings/blockers. See analysis/toolkit-issues.md. Toolkit/inputunchanged; no rebuild. Worker stopswrites afterhandoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7454|7972|32525|243|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26230|0|28370|4398|
|0F|0|31279|0|31279|1489|

## Round6 redirect completed research
Five newconditionalCode roles/127bytes freshlydecoded; allreferences close into knowncollision/text/direction/moneyinterfaces, no residualedge/consumer. Exact64word1AFBformula and sole9AFBencodedhit inBRRsample supplypatternbut noformatcontract. Sixnamedtilemapendcomparisons plus C4FAsourceprogression disprove an assumedcommonpostrectanglealignmentrule: followingzeros0/2/6/24, noalignmentrule, mixed02EB72sixzerosstillunsupported. Uploadtailchunkprovenancecomparison:504bytepostterminatorareas no sampled16+externalmatch; FC27tailmatchesuniform99/66only. PagepatternsFD00/FF00=99 do notmatch DSPinit6D03/7D00/4D00 reservedpagecontract; noecho/sample/fillclaim. Fullresults/missingfacts in evidence/round06-redirect-residual-assessment.json; reusable collectors in redirect6/. No oldunchangedplayback/shortdrive/CFG audit repeated. ThreeblockednewhistoriesR6-L001..003, ready[]. Finalinvestigationeligible and coordinatorrequired; no ownlaterphase.

## Final investigation attempt01
Completed independent allocation/initialization/semantic investigation; actual11514unknown unchanged,329bytes proposed across FI01-L001..004 for separate acceptance. Exact directory/BRR partition, effect parallel arrays/new closed streams, explicit writable141-byte destination and28x7 initial envelope image, plus conditional orphan program/dependentpitchcell. Wholepitch formula and enclosing upload/graphicscontainer tests did not establish additional roles. No runtime. Full methods/comparisons, precise ledger, counterclaims and verification: [handoff](final-investigation/attempt01/handoff.md). Do not repeat unchanged methods; coordinator owns next phase. No new toolkit issues found; [report](toolkit-issues.md).


## Round7 independent acceptance
ROM classification: 97.87% (513103 / 524288 bytes classified; 11185 bytes unknown).

Independently accepted FI01-L001..004, exact30 Static Data roles/329bytes (49+60+196+24). All1313 prior roles, labels, comments, evidence and execution views unchanged; all1343 current roles annotated. No split required: claims occupy exact unknown complements. Counts and complete residual ledger recomputed from current internal map, not raw export fallback.

L001 independently reconstructs both upload segments (skip physical77E04..78004), all33 directory start/loop contracts and25 distinct terminated BRR chains tiling ARAM50C4..FC27. Joint DIR50, encoded5084/50A4, explicit silent entry32 at50C4 and independent sample0 at50CD support allocation, with no encoded33/count32 safety guard or natural missing-selector claim. Existing instrument33 reads alias beyond nominal32byte arrays; preserve physical roles and explain overflow aliases in commentary.

L002 independently parses all16 closed effect programs and joint1060/1070/1080 + independently supported envelope1090 boundary. New effect1 is13bytes434A..4357 and effect4 is17bytes437B..438C, with meaningful initialized instrument/volume/envelope/pitch setup and exact adjacent selected stream boundaries. Count16 allocation inference is explicit, not an encoded guard. Their connected pitch/instrument cells were already classified.

L003 independently verifies AE multiply7, fixed-width absolute readers1090..1096, selected AE27 and independent next configuration source1154. Full28x7 initial uploaded envelope image is196bytes, including defaultrecord0, exact141byte writable configuration destination1097..1124, and record21 tail..27. Copy loop Y8C..1 plus explicitY0 overwrites exactly141bytes, including record21 timer1123; initial zeros are not persistent configuration/read-before-overwrite evidence. Confirmed ROM070004..077E04 -> ARAM0400..8200 supplies upload provenance, not byte equality alone.

L004 independently accepts meaningful23byte conditional program43A3..43BA with initialized instrument12/envelope24/negative modulation/AC/note78+60ticks/envelope0/note1+1tick/END, preceding selected END43A2 and next indexed43BA. Natural caller remains unresolved; descriptor6 aliases5 at438C. Independent hand derivation from AC-before-slew, intervalzero and delay3 yields indices79,86,2, adding only highcell7099C. No orphanselector6 or new natural use claim; broad pitch recurrence and pre-directory gap remain unsupported.

All15 prior handoff hashes verified BEFORE writes; entire exact28file current editorial tree copied/hashverified in round07/commentary-input, and remains unchanged. Attempt01 artifacts unchanged/hashverified; copied collectors replayed into round07/replay and five key output objects exactly equal prior evidence. Earlier37file historical preservation trees untouched. Fresh prior edited-source verifyok/byte_identicaltrue; new structural checkpoint validates/byte_identicaltrue/diagnostics[]. Saved internal analysis/state.json and portable maps/current.json. Proof round07-disassembly-verification.json.

No emulator started/new input/release/patch/snapshot/trace/coverage. No unchanged failed gameplay/playback retry: accepted connected contracts change no missing selector/guard/graphics container prerequisite. Residual heading odd cells, guarded isolated bytes, mixedgraphics02EB72..030000, sparse pitch/unselected command/sequence gaps, pre-directory allocation and post-upload/sample tails retain missing structural facts. Full exact residual ledger round07/residual-ledger.json; no byte quota or forced acceptance.

ROM incomplete. All1313 existing source-role review credits reusable;30 new physical roles and changed allocation/initialization dependencies require separate commentary integration. Current source remains prior edited round06 tree and needs refresh before redirect. Coordinator owns commentary, redirect and eligible iterative final investigation (0<11185<=16384 and11185/524288<0.05), final editorial/archive audit pending. This worker launches no laterphase. No new toolkit issues found;TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. Toolkit/input unchanged, no rebuild. Worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7454|7972|32525|243|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26559|0|28699|4069|
|0F|0|31279|0|31279|1489|


## Round7 redirect
New independent structural methods replace old selector requirements: complete34-cell command-vector/implemented-handler partition supports proposed4bytes8E/BA; complete18x32-byte encoded song/config row serialization supports proposed289bytes reserved initial capacity, subject to explicit independent allocation-versus-fill review. All18 origins verified in fresh loader decoding. Both unselected command handlers terminate at parser06BF;8E changes duration consumption, BA changes existing auxiliary mode without new pointer. No live selector, Null guard or playable spare claim. Followed connected effects/envelope overwrite/actualinstrument33 aliases, plus independent residual main/graphics/upload triage; no downstream new source roots. Exact methods, counterclaims, missing facts: evidence/round07-redirect-residual-assessment.json. Old attempt01 remains immutable; no unchanged M1..M8 or gameplay retries.


## Round8 disassembly COMPLETE — ROM incomplete
ROM classification: 97.92% (513396 / 524288 bytes classified; 10892 bytes unknown).

Accepted R7-L0014bytes and R7-L002289bytes independently, 21 new Static Data roles; all1343 prior physical roles unchanged. Both34-cell typed vector partition and complete implemented8E/BA interfaces verified against fresh SPC700 decode. Song allocation accepted jointly:18 independently encoded indexed origins, eight low/high selected-root pairs, zero spare voice fields, field-specific paired nonzero1567 configuration tails, and independent next accepted effect origin1060. Interrow-fill alternative explicitly assessed: it explains voice zeros but does not explain paired configuration-rest initialization as directly as reserved serialized capacity. Count32 inferred, not encoded. No playable spare voices, guarded NULL, natural selector or source-copy claim. Exact supported unknown complements inserted without splits. Mutable initial upload provenance retained.

Fresh saved/exported map validated; exact checkpoint round08-disassembly-final byte-identical, diagnostics[]. Prior edited commentary source freshly rebuilt byte-identically; entire28file tree preserved unchanged, all19 handoff hashes checked before writes, immutableFI01 unchanged. No emulator/runtime/input/patch/snapshot/trace/coverage changes or unchanged failed route replay. Independent collector replay copied into round08/replay. Initial local handler assertion used ABA inside predecessor JMP; corrected to actual vector-selected ABB before annotation, not a toolkit issue.

Exact156remaining unknown ranges in round08/residual-ledger.json; per-bank counts below. Connected duration and auxiliary modes install no new stream roots. No changed graphics/container/copy/source or pitch-domain prerequisite; unchanged old methods not repeated. All1343 unchanged source-role credits reusable; 21newroles require separate current-source commentary integration. ROM incomplete; final archive audit pending. Coordinator owns commentary then redirect then eligible iterativeFI02; no laterphase launched. No new toolkit issues found; TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. No toolkit/input changes or rebuild. Worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7454|7972|32525|243|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26852|0|28992|3776|
|0F|0|31279|0|31279|1489|


## Round9 disassembly COMPLETE — ROM incomplete
ROM classification: 97.93% (513419 / 524288 bytes classified; 10869 bytes unknown).

Accepted FI02-L001 (14 bytes) and FI02-L002 (9 bytes), three new Static Data roles; all1364 prior roles unchanged. Two independently encoded coindexed animation origins8804/8823, independent following code8842, complete31cell paired layout and synchronizedFF boundaries jointly establish final six-frame suffix. Count31 inferred, not encoded; no natural cursor24 producer. Song phrase ROM07168D..071696 / ARAM1A89..1A92 accepted jointly from meaningful closed note/slur/return object, established priorRETURN1A88, next selectedphrase1A92 and inherited initialized-voice/caller-stack ABI. Explicit-duration56/54/51 notes11/11/10 ticks; fixed-duration alternative retains physical command/end boundaries. No natural caller/instrument/pitch claims. Counterclaims and connected barriers: round09/independent-review.json. No additional supported extension from this batch; generic cleanup/reentry tails, repeated placement pointers and guarded slots remain unknown with existing exact missing facts. No unchanged runtime route repeated.

Map validated/saved/exported; exact round09-disassembly-final rebuild byte-identical, diagnostics[]. Prior edited commentary source freshly verified byte-identical. Entire28file editorial tree and all19 pre-write handoff hashes checked; immutableFI01/FI02 unchanged. All1364 prior source-role credits reusable;3newroles and affected consumers need separate commentary integration. Exact residual ledger has153ranges/10869bytes; per-bank counts below. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. ROM incomplete, finalarchive auditpending. Coordinator owns commentary, redirect then eligible iterativeFI03; worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7468|7972|32539|229|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26861|0|29001|3767|
|0F|0|31279|0|31279|1489|

## Round9 redirect
R9-A1 follows new animation name-bit1 to exact conditional tile/palette footprints; all sources alreadyclassified. R9-A2 closes new phrase interfaces without new initialized source/pitch/return target. R9-A3 directly reconciles153source gapnotes and compares independent residualgroups against immutable FI01/FI02 and prior redirects. No unchanged failed retry, classification or runtime changes. No ready lead; eligible iterative FI03 belongs to coordinator, not terminal exhaustion. Assessment evidence/round09-redirect-residual-assessment.json; exactledger redirect9/residual-ledger.json; proof round09-redirect-verification.json.


## Round10 disassembly COMPLETE — ROM incomplete
ROM classification: 97.93% (513424 / 524288 bytes classified; 10864 bytes unknown).

Accepted FI03-L001, exact5-byte Static Data restart-transfer object ROM07143E..071443 / ARAM183A..183F; all1367 prior roles unchanged. Joint prior END1839 boundary, coherent transpose-zero/JMP1818 semantics, established selected sibling179F/1801 idiom and balanced existing target CALL197D continuation establish conditional ownership. Independent target traversal635tokens/259notes/302alreadyclassifiedbytes, maximum5stackbytes, instrument1D before firstnote, balanced return/loops through existing END1839. Initialized active voice/fixed0/noone-shot override/validauxiliary-volume-DSP state and5free stackbytes are conditional ABI, not inferred natural entry. Separate trailingEND183F remains unknown; no new pitch or playback claim. Counterclaims and connected barriers: round10/independent-review.json. Target supplies no new source extent/pointer/bank or pitch domain; no other connected supported extension. No changed runtime prerequisite; unchanged routes not repeated.

Map validated/saved/exported; exact round10-disassembly-final rebuild byte-identical, diagnostics[]. Prior edited commentary source freshly verified byte-identical. All19 pre-write handoff hashes checked, entire28file editorial tree copied before generation and unchanged; immutableFI01..FI03 and configured release binary unchanged. Reuse1367 prior role reviews;1newrole and affected consumers require separate commentary integration. Exact residual ledger has153ranges/10864bytes; per-bank counts below. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. ROM incomplete; finalarchive auditpending. Coordinator owns commentary, redirect then eligible iterativeFI04; worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7468|7972|32539|229|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26866|0|29006|3762|
|0F|0|31279|0|31279|1489|

## Round10 redirect
R10-A1 checks accepted five-byte restart interface without repeating target parsing; unconditional transfer supplies no adjacent END ownership or new storage domain. R10-A2 directly reconciles153 source gaps and compares11 independent residual groups with FI01..03 and all previous redirect methods. No ready lead or classification/runtime changes. Current exact ledger redirect10/residual-ledger.json; assessment evidence/round10-redirect-residual-assessment.json; proof round10-redirect-verification.json. Coordinator automatically schedules iterative FI04; not terminal exhaustion.


## Round11 disassembly COMPLETE — ROM incomplete
ROM classification: 97.93% (513455 / 524288 bytes classified; 10833 bytes unknown).

Accepted FI04-L001, exact31 Static Data bytes in16claims within ROM001CC7..001DC7 / CPU009CC7..009DC7; all1368 prior roles unchanged. Independent finite256cell test confirms255defined coordinate bearings within1unit of -atan2(y,x)*128/pi, including all31omitted cells; undefinedzero-vectorFF already classified. Independent decoded actual collision and AI consumers establish heading type and coordinate layout. Encoded9CC7 origin plus independent992A/9DC7 sine-source contract, complete nonlinear finite two-dimensional object and225prior typed cells jointly support inferred full256byte storageextent. No encoded count or exactgenerator/rounding/holdout claim. Numerical pattern, adjacency and indexcapacity alone are insufficient; counterclaims reviewed explicitly in round11/independent-review.json. Exhaustive65536input scalar model of decoded simultaneous magnitude reduction confirms produced components-7..7 and225indices: coordinate-8 storage is distinct from natural read-domain. No naturalnibble8read, unused/dead, newexecution or universalhardware claim.

Connected consumers are already classified byte-heading arithmetic; all256sinecells already classified. No new pointer/source/bank/upload/length domain. Existing other residuals retain their documented missing ownership/extent facts; no productive changed runtime prerequisite, unchanged routes not repeated. Map validated/saved/exported; exact round11-disassembly-final rebuild byte-identical, diagnostics[]. Prior edited commentary source freshly verified byte-identical. All19pre-write handoff hashes checked; entire28file editorial tree copied before generation and unchanged; FI01..04 immutable and configuredreleasebinary unchanged. Reuse1368prior role reviews;16newroles and affected grid descriptions require separate commentary integration. Exact residual ledger has137ranges/10833bytes; per-bank counts below. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historicallyfixed,0activefindings/blockers; analysis/toolkit-issues.md. ROM incomplete; finalarchiveauditpending. Coordinator owns commentary, redirect then eligible iterativeFI05. Worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7499|7972|32570|198|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26866|0|29006|3762|
|0F|0|31279|0|31279|1489|

## Round11 redirect
R11-A1 inspects newly complete bearing-grid interfaces through collision sectors, pair separation, sine/terrain probes and AI steering/waypoint continuation. They operate on byte headings and already classified sine storage; no new pointer, bank, source/count, index producer or asset. Independent start-heading odd bytes still lack a stored-word schema; the coordinate-like block still lacks a typed producer/allocation. No repeated grid fit,65536input quantizer, unchanged source/pitch/selector/parser/copy/reference/gameplay experiment. R11-A2 directly reconciles137source gaps/neighboring raw sections, zero generic physical-role labels and complete current-source review; all11independent residual groups compared with15immutableFI01..04methods and prior redirect histories. No ready lead; FI04-L001 history updated. Specific structural barriers retained separately from natural reachability.

Classification513455/524288=97.93376922607422%;10833unknownbytes/137ranges/1384roles. Exactledger analysis/redirect11/residual-ledger.json. Fresh internal/portable validation, map.save/export, separate structuralcheckpoint round11-redirect-final and edited commentary/source rebuild byte-identical; diagnostics[]. All19handoff hashes, whole28-file editorial tree,79immutableFI01..04files and configuredbinary unchanged; readingcopy fresh. Proof analysis/round11-redirect-verification.json; bankcounts analysis/redirect11/per-bank-counts.json. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. Current-source readability complete; finaleditorial/archiveaudit pending.

Coordinator automatically schedules iterativeFI05:10833<=16384 and10833/524288=0.020662307739257812<=0.05. No ready redirect lead is required. Design concrete alternative methods against exact barriers before any substantive retry; this is not terminal exhaustion. No laterphase/workers launched; worker stopswrites. No new toolkit issues found; TI-001 historicallyfixed,zeroactivefindings/blockers; analysis/toolkit-issues.md.

## Final investigation attempt05
Final investigation attempt05 COMPLETE — terminal incomplete. Input/retained checkpoint round11-redirect-final; complete current-source editorial round11-commentary-final.513455/524288=97.93376922607422%classified;10833unknown bytes/137ranges/1384roles;0proposed/accepted. All15prior FI methods and redirect histories compared before two new discriminating methods. FI05-M1 rejects complete output-bit semantics as source ownership:AA and paired-bit packing manufacture flags without consuming odd masks. FI05-M2 independently extends prior MOV audit to non-MOV writes:987preserved SPC instructions reconciled with current views,25direct-page mutations freshly decoded,108implicit stack-write sites,0absolute RMW/bit candidates reaching residual images. No dependent allocation or producer. All11residual groups independently assessed for other credible schema/container/extent/context alternatives; none remains immediately productive on unchanged evidence. This is genuine current-method/planning exhaustion, not missing facts alone, quota or universal runtime requirement. Exact137ranges, aliases,neighbors,hypotheses,missingfacts and dispositions:analysis/final-investigation/attempt05/range-ledger.json; comparisons and terminal reasons:residual-method-assessment.json. Stop unchanged automatic scheduling; resume requires new evidence, materially improved concrete discriminator or explicit user reassessment. No invented next lead.

Fresh inspect/internal and portable validation, structural and edited-source byte-identical rebuilds pass;diagnostics[];fresh reading expansion matches. All19handoff hashes,28editorial files,79immutable FI01..04files and configured binary unchanged. Exact map/state/source preserved, no save/export/new checkpoint because evidence-only final-investigation contract. No ROM/toolkit/runtime/input/patch/snapshot/trace/coverage changes or rebuild. Current-source readability complete; finaleditorial/archiveaudit pending. No whole-ROM completion. No new toolkit issues found;TI-001historicallyfixed,0activefindings/blockers;analysis/toolkit-issues.md. Compact handoff analysis/final-investigation/attempt05/handoff.md. Attempt05 immutable upon handoff;worker stopswrites,nootherworkers/laterphases launched.

## Updated-policy redirect policy-resume-01
Corrected negative scope on unchanged primary evidence: exact-floor counterexamples do not reject85word quantized pitch representation; unresolved sixzero prefix does not reject independently supported FF banktail suffix. Two ready leads PR01-L001/002 propose19+5256=5275bytes;0accepted, current10833unknown. All11group compact primary rejection review and exact split ledger retained in [handoff](redirect/policy-resume-01/handoff.md). Fresh validation/rebuild/preservation pass; independent disassembly next. No runtime or gameplay experiment; no new toolkit issues found.

## Round12 disassembly active milestone
Input round11-redirect-final / round11-commentary-final; independent policy-resume-01 PR01-L001/L002 review. All 19 current handoff hashes checked; entire 28-file commentary preserved before map/generation changes; 104 immutable legacy files and release binary inventoried. No runtime or toolkit changes. Evidence analysis/round12/preservation.json.


## Round12 disassembly COMPLETE — ROM incomplete
ROM classification: 98.94% (518730 / 524288 bytes classified; 5558 bytes unknown).

Accepted PR01-L001: 19 Static Data bytes in 19 exact cells of the split85word pitch table, ROM0708F3..07099D / uploaded ARAM0CEF..0D99. Independent raw-ROM full85word check plus fresh0984 integer/scaling decode, encoded85byte base displacement and independently accepted temporal endpoint indices2/86 jointly establish bounded storage. Every known/omitted word agrees within one integer quantum with seven-octave semitones. Known75 versus incomplete10 criterion is identical. Exact floor failures63/75/84 are already typed; they reject original uncapped floor generation only.14bit field masks;3FFF is inferred table cap, not hardware saturation. No original rounding, natural selection or newexecution claim. ThreeD9suffix bytes07099D..0709A0 remain unknown.

Accepted PR01-L002: 5256 Static Padding bytes ROM02EB78..030000 / CPU05:EB78..05:FFFF. Fresh source/rectangle consumer decode and independently exact23x17word end02EB72, maximalFFsuffix, independently rooted next-bank track source030000, nine accepted allocation-fill analogues and reconciled9228instruction/4171mainreference plus source-window counterchecks jointly support inferred bank fill. Uniformity and absence of references alone are insufficient. Sixzeros02EB72..02EB78 remain unknown; no trailer/historical linker/universal unread or saturation claim.

All1384 prior roles unchanged; reuse their review credits.20new roles and affected pitch/fill descriptions need separate full current-source commentary integration. Entire28file editorial tree copied before changed generation and remains identical; all19input handoff hashes verified beforehand,104legacy attempts/configuredrelease binary unchanged afterward. Fresh prior edited source and new exact round12-disassembly-final checkpoint rebuild byte-identically; diagnostics[]. Maps validated/saved/exported. Exact residual ledger:123ranges/5558bytes; bank totals below. No emulator creation, runtime/input/patch/snapshot/trace/coverage changes, ROM/toolkit edits or rebuild. Local draft Data execution views rejected safely and corrected per documented schema; no new toolkit issues found, TI-001 historically fixed, zero active findings/blockers; report analysis/toolkit-issues.md.

Connected continuation excludes pitchD900 neighbor cells; instrument/scaling consumers and downstream track allocation already classified. Sixzero and interior graphics gaps still lack independent ownership. New pitch/fill roles supply no new producer/source/context or changed productive gameplay prerequisite; unchanged routes not repeated. Prior11group claim-local rejection review retained at analysis/redirect/policy-resume-01/rejection-review.json. Easy gains complete for this pass; this is not terminal exhaustion or whole-ROM completion. Ordinary workflow next is commentary then redirect, with no separate final-investigation phase or percentage rule. Coordinator owns later phases; worker stops writes after handoff. Current prior-source readability verified; new-role integration and final editorial/archive acceptance remain pending.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7499|7972|32570|198|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|5256|32762|6|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26885|0|29025|3743|
|0F|0|31279|0|31279|1489|


Round26 disassembly consumed R25-L001. The exact 32 odd offsets in the finite bank-00 two-byte-stride start-heading window were accepted Static Data under a bounded storage-window claim; slot-internal meanings and reachability are unresolved. Sparse one-byte values plus independent zeros remains a competing finer semantic layout, so no high-field/padding/word claim was made. See analysis/round26/independent-review.json and handoff.md. Current status 518890/524288 classified, 5398 unknown. `round26-disassembly-final` verification was byte-identical. Commentary must integrate these annotations before redirect. No new toolkit issues found; see analysis/toolkit-issues.md.
