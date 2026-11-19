# Disassembly progress

Round 001 disassembly in progress. Initial state inspected and empty (no classification).
`python3 run.py inspect` verified 262160-byte MMC1 ROM; input SHA-256 remains workspace identity.
Verified raw baseline established: [verification](baseline-verification.json), map `maps/baseline.json`, generated `generated/baseline`.
Persistent RPC emulator started, reset PC $C000, effective PRG slots [0,1,30,31]. Bounded initial startup requested: no buttons, 180 frames, max_operations=10000000, timeout_ms=30000, tracing enabled capacity=0. Awaiting return before accepting execution evidence.
No ready leads existed. Pass remains incomplete; classification state unchanged so far.

Startup returned a bounded execution timeout at frame72 (2,143,640 CPU cycles); accepted instruction/context aggregates retained with aggregate_dropped=0. Saved startup-state.json, startup-summary.json and startup-contexts.json under analysis/evidence/pass-001, plus same-build startup snapshot. Title identifies Battle Chess, Data East 1990. Subsequent untraced startup/Start route reached animated chess board at frame443 (board.ppm); tracing reenabled before A selection and menu exploration. 1247 contextual roots at frame505, classification batch now being inspected. No semantic reachability claims from untraced frames.

Structural checkpoint: `pass-001-structural` verified byte-identical (map SHA256 9aa6696b7e4d2d791ed219ec5c62f00f39c1f57c22a7c953fc4b949e15fb8071). Current classified 183894/262160 bytes (70.15%), unknown78266. 2689 accepted-fetch Code bytes, 959 Static branch bytes; header/vectors22, Image139264, generic Data40960. Static batches use ordinary successor paths only, with mapper/call barriers. Inspected all1276 physical fetched instructions,450 new static candidates; zero overlap/KIL/BRK conflicts. Detailed local evidence under analysis/evidence/pass-001.

Menu at frame647; A opens Move submenu (New Game/Setup Board/Take Back/Replay) at frame685. One artificial experiment writes RAM$0030=0 (known countdown loop at CPU$C3B7-$C3CC) from clean menu snapshot, then30frames; intended only to bypass an understood delay. Subsequent observations are artificial until clean restore. Bounded setup/menu continuation active; later terminal scenarios still unvisited.

Setup Board successfully visited at frame875 after Move>A, Down, A and settling. Local screenshot `evidence/pass-001/setup-ready.png` confirms the distinct 2D editing screen with blue/red piece palettes. Earlier intermediate capture was still submenu animation; no false claim of failed setup. Snapshot `setup.snapshot.json` is resumable at frame875, artificial due only to previously documented timer patch. Built-in clear/restore/done menu investigation continuing briefly; exact replay will be retained in scenarios.json.

## Round001 disassembly completed — incomplete ROM, ready for commentary/redirect

ROM classification: 71.17% (186573 / 262160 bytes classified; 75587 bytes unknown).

Final fresh checkpoint **pass-001-final**: [verification](pass-001-final-verification.json), map `maps/pass-001-final.json` SHA256`77d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59`, generated `generated/pass-001-final`. map.validate passed and generated assembly rebuild is byte-identical. Input/game.nes SHA unchanged. Current internal map `analysis/state.json`, portable `maps/current.json`,1144evidence-bearing regions; code4899,Image139264,Data42410. Unknown complement and per-bank JSON saved under evidence/pass-001. Readability/editorial not completed in this phase.

Gameplay: initial board and options visited naturally; setup editor observed after artificial known countdown shortcut. Near-mate board edits confirmed in editor, but final Done attempts remain Setup submenu. No capture/endgame visit claimed. Final artificial snapshot `evidence/pass-001/near-mate-game.snapshot.json` at frame1485,CPU$D063,context[0,1,30,31]; useful patched editor snapshot `near-mate-done.snapshot.json` frame1200. See full progression/replay checklist in scenarios.json and handoff. Broad progression remains nonterminal: redirect can resolve menu-ready/Done contract and then capture/checkmate, rather than replaying failed quick inputs. No stage/boss system applies to this chess game.

Leads R001-L01/L05 verified, R001-L02 attempted/open (supported Setup→Done/terminal shortcut),R001-L03/L04open (engine/call contracts and mixed script/data consumer bounds). No prior rejection reused. Pass stops after first connected easy-gain batch and bounded setup continuation; not terminal exhaustion. Final trace aggregate_dropped=0; detail dropped6763852 intentionally atcapacity0,2795unique accepted instruction starts. Natural versus artificial provenance retained; execution count is not classified byte count. RPC server stopped after durable saves.

No toolkit issues found; [report](toolkit-issues.md). Free disk4.9GiB at last check; scratch32KiB before final record scripts, durable evidence<40MiB. No toolkit edits, publishing, archives or destructive changes performed.

Per-bank classification (header16Data separately):

| PRG 8KiB bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 1101 | 0 | 1101 | 7091 |
| 1 | 0 | 55 | 55 | 8137 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 0 | 8192 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 0 | 8192 | 8192 | 0 |
| 24 | 0 | 0 | 0 | 8192 |
| 25 | 0 | 204 | 204 | 7988 |
| 26 | 0 | 185 | 185 | 8007 |
| 27 | 0 | 131 | 131 | 8061 |
| 28 | 619 | 0 | 619 | 7573 |
| 29 | 0 | 239 | 239 | 7953 |
| 30 | 2786 | 0 | 2786 | 5406 |
| 31 | 393 | 620 | 1013 | 7179 |

Final retention check:4.8GiB free, scratch56KiB, durable evidence52MiB (supersedes earlier size estimates). JSON parsing and byte-for-byte identity of internal state/current map/final checkpoint map confirmed. ROM SHA256 unchanged. Compact handoff saved at analysis/handoffs/round-001-disassembly.md.

## Round 001 commentary completed — ready for redirect

All currently classified source reviewed and made readable, separately from unchanged 71.17% ROM classification. Reviewed 1246 full-ROM scaffold entries, 174 meaningful code blocks and all 4899 classified code bytes across banks 0/28/30/31. Hardware, RAM/object fields, routines, branch destinations, consumer table accesses and exact formats now have descriptive symbols and light mechanical explanations. Image/data extents and unknown boundaries have honest local format/uncertainty notes. No classified readability backlog remains; final editorial audit stays pending.

Fresh byte-identical verification: commentary/verification.json. Refreshed reading copy: commentary/reading-after.asm. Exact partition/source-reference coverage: analysis/round-001-commentary-review.json. Frozen checkpoint remains pass-001-final, map SHA256 77d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59; input and maps unchanged. Commentary source/assets remain local.

Redirect should inspect Setup submenu callback/ready/Done paths before replaying input; additional consumer-based structural questions concern menu records, object script dispatch, PPU resource descriptors and audio tables. See commentary/uncertainties.md and analysis/handoffs/round-001-commentary.md. No toolkit edits, publishing, archive, cleanup/deletion or terminal-exhaustion claim.

## Round001 redirect completed — combined ready batch for disassembly

Unique research attempt [round-001-a review](redirect/round-001-a/review.md), compact [handoff](handoffs/round-001-redirect.md). All existing concrete menu/Done, engine contracts, object scripts, PPU resources, audio and composite graphics avenues investigated through useful connected continuations and independent alternate bounds. LeadsL02verified,L03/L04ready,newL06..L10ready; prior histories retained.

Research proposals:6293new accepted artificial fetch bytes(3027instructions),6441scriptbytes across8families/3045records,27callbacks/stackhelpers,4counted menu arrays,14PPU descriptor/payloads,44audio lists/37patterns,19compositecontainers with1510pointer/width/terminatedstream checks. Composite independent origins give inferred counts; full parse consistency is corroboration, not encoded selector proof. Exact candidate unknown union63007, unproposed12580; residualledger totals75587 and splits physicalbank/context/neighbor boundaries. These are proposals only: **0classificationbytes accepted** during redirect.

Gameplay: supported SetupDone3Dtransition, BC1 manual-enable contract, legal near-mate queen move and negative terminalstatus, pawn-capture confrontation and distinct banner/result presentation atframe3053. Exact artificial writes, input/release budgets, observed/untraced distinction and snapshots preserved. Promotion/castling/alloutcomes not exhaustively visited; no natural terminal or full compatibility claim.

Input/game.nes SHA256 remains d92ad55cb98825af7876d1e93b9ce0fc04163d7cccf8060b4e790d9a919f4efc. Currentmap/state/frozenmap SHA256 all77d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59. Accepted counts unchanged186573classified(71.17%),75587unknown; current-source commentary readability complete atoldmap, finaleditorial pending. Existing fresh commentary byte-identical verification remains applicable; no source/map changes or newcheckpoint needed in research-only phase.

No toolkit issues found; [report](toolkit-issues.md). Configured binary unchanged. All traced summaries aggregate_dropped0; capacity0 detaildrops intentional.4.7GiBfree atlastinspection, scratch196KiB; durable research preserved. No agents, cleanup/deletion, toolkit/source/ROM/classification edits, downloads, publishing or archive. Next step independent disassembly review of coherent batch, then full current-source commentary/redirect. This is not terminal exhaustion.


## Round002 — disassembly completed, incomplete ROM

ROM classification: 97.82% (256451 / 262160 bytes classified; 5709 bytes unknown).

Accepted 69878new unique bytes:6293artificial observed code bytes,4018Static code bytes and59567Static data bytes. Current types:Code15210,Data101977,Image139264. Existing regions and prior commented source preserved. Header16lies outside PRG table.

Independent review: [structural review](evidence/pass-002/independent-review.md), [accepted fetch audit](evidence/pass-002/observed-audit.json), [data proposals](evidence/pass-002/data-proposals.json), [connected code](evidence/pass-002/connected-candidates.json), [107normal-return contracts](evidence/pass-002/return-contracts.json), [additional data review](evidence/pass-002/additional-data-review.json), [completion slices](evidence/pass-002/accepted-completion-slices.json). All6293physical accepted-fetch bytes freshly decoded;3352contextual records correspond to3027physical starts. No observed/static/data boundary conflicts.

R001-L03/L04/L06..L10 verified with retained history: counted menus; six scene configs/14upload resources; all8scriptfamilies/3045records/6441bytes plus declared handlers/callbacks; audio44lists/37patterns;19composite containers/1510paired rows with independent pointer/width origins. Specific role inference and natural reachability separated. Generic Static data after practical inspection covers audio parameters, packed nibble data, geometry/palette/status lookups and menu auxiliaries where exact schema remains unresolved. Body8888..893E was separated from surrounding data.

Ordinary static successors/direct callees followed through6287toolkit contextual instructions, then7250reviewed contexts with107balanced normal-return/bank-preserving contracts;1793new instructions/3757bytes in connected batch. Additional bounded8888andC3BEconsumer/wait routines206bytes and55declared-handler continuation bytes accepted independently. Low PRG banks in post-call fixed-handler views left unresolved rather than invented. Guarded8FFE byte00 is checked against68before a jump, so BRKhypothesis rejected locally; only signature data and independently bounded eight-direction masks accepted. No exhaustive recursive AI/legal-move implementation closure or every selector/asset use claimed.

No unchanged gameplay replay. Reused exact artificial Done/move/terminal/capture-result runtime evidence and durable snapshots from redirect. Progression checklist preserved; natural endgame/full cinematic return and specialized promotion/castling routes remain unclaimed. Existing routes still available to subsequent redirect; arbitrary further gameplay permutations would not resolve current static bank/return boundaries.

Validation: map.validate passed after every accepted batch; state saved and current map exported. Fresh [checkpoint verification](pass-002-reviewed-verification.json) confirms ca65generate/rebuild byte-identical, mapSHA256 `6a65061ba6fb27617437a138ba294582f7f69728fcbb657e5eacf49de6c4c56e`. Earlier pass-002-final checkpoint remains a valid historical intermediate. Input ROM SHA256 unchanged `d92ad55cb98825af7876d1e93b9ce0fc04163d7cccf8060b4e790d9a919f4efc`. Final [summary](evidence/pass-002/final-summary.json), [exact unknown complement](evidence/pass-002/unknown-complement.json) and [handoff](handoffs/round-002-disassembly.md). No live process/writer. No cleanup/deletion, archive, publication or toolkit edits.

Remaining 5709unknown bytes are localized code/entry/return/bank or residual data-boundary uncertainty, chiefly banks0/26; refer to residual ledger and new R002-L11. This is an easy-gain phase handoff, not terminal exhaustion or whole-ROM completion. Current-source readability on the new map is pending commentary; old map commentary remains intact. No toolkit issues found; [report](toolkit-issues.md).

| PRG8KiB bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 4164 | 2355 | 6519 | 1673 |
| 1 | 0 | 8176 | 8176 | 16 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 0 | 8192 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 0 | 8192 | 8192 | 0 |
| 24 | 0 | 8192 | 8192 | 0 |
| 25 | 0 | 8192 | 8192 | 0 |
| 26 | 1566 | 4034 | 5600 | 2592 |
| 27 | 0 | 8192 | 8192 | 0 |
| 28 | 908 | 7268 | 8176 | 16 |
| 29 | 0 | 8192 | 8192 | 0 |
| 30 | 6277 | 827 | 7104 | 1088 |
| 31 | 2295 | 5573 | 7868 | 324 |

## Round002 commentary completed; redirect active

Current-source readability complete at pass-002-reviewed, all9086 scaffold entries and2098 meaningful units reviewed; fresh byte-identical commentary/verification.json and reading-after refreshed. See analysis/handoffs/round-002-commentary.md. Classification unchanged256451/262160,5709unknown.

Round002 redirect active at unique analysis/redirect/round-002-a/. Reassessing conservative return barriers using primary code and accepted107 contracts, especially bounded indirect RAM writes versus actual mapper/CPU-stack effects. No replay or classification mutations.

## Round002 redirect completed — combined ready batch

Current accepted checkpointpass-002-reviewed remains256451/262160classified(97.82%),5709unknown; zeroacceptedclassificationbytes inresearch. R002-L11ready4997Codebytes(2375instructions;562artificialObserved/4435Static),newR002-L12ready668DataBytes/40ranges. Uniqueproposal5665;44unproposedbytes in10exactambiguousshort/nonstandard-stackintervals. See [review](redirect/round-002-a/review.md), [freshproposal audit](redirect/round-002-a/final-audit.json), [exactcurrentunknownpartition](redirect/round-002-a/residual-ledger.json), [handoff](handoffs/round-002-redirect.md).

CorrectedfalseZP/indexedRAMCPUstackaliases, finite/tailcomponentcomposition andfixed-slotnull-lowerbankcontinuations; explicitvalidqueue/frame/script/menu/history/audio preconditions retained. ExactAIwrapper/cancelcallerconditionalcases localized,noglobalcommanddomainorunconditionalServiceFrameclaim. Fresh2381execution-viewdecodesandallData/physicalboundarychecks pass,noconflicts. SourceROM/current/frozenmap/internalstateandcommentary sourceSHAunchanged; previousbyte-identicalcurrent-sourceverificationcontinuesapplicable.

RepresentativeAIthinking/cancelvisitedartificiallyfromretainedDoneboard viaPC-only9B08,118070initialoperationsand13700Select-canceloperations reachedUI9B1EwithsameSPF0. Initialcontext-archivekeyerrorpreserved; lostinitialaddressesnotclaimed. Recoveryusedsavedthinkingcheckpoint only, retained599cancelacceptedviews/283unknownstarts562bytes. No unchangedinitialgameplayreplay. Specifictwo-columnqueuecapacitytestedwithunpatchedSP/RAM,minSPF3; initialwrongoperandentrypreserved/excludedandcorrected. Scenariosupdated,naturalending/fullcinematicreturnorfullcompatibilityunclaimed.

No toolkitissuesfound; no toolkit/ROM/map/state/sourceedits,agents,deletion/cleanup,publicationorarchiveactions. Configuredreleasebinaryused. Storage4.5GiBfree/sharedscratch196KiB atlastcheck; nolivewriter/server. Nextindependentdisassemblyreview,fullcurrent-sourcecommentary,thenredirect. Thisisreadybatch,noteitherwhole-ROMcompletionorterminalexhaustion.


## Round003 disassembly completed — incomplete

ROM classification: 99.98% (262116 / 262160 bytes classified; 44 bytes unknown).

Accepted5665bytes:4997Code(562 artificial Observed,4435Static) and668StaticData. Independent [structural review](evidence/pass-003/independent-review.md), [fresh2381view audit](evidence/pass-003/independent-audit.json), [annotations](evidence/pass-003/accepted.json). R002-L11/L12verified; typed queue/frame/script/menu/history/bank preconditions preserved. LocalE919normalreturn and987Acancel contracts evaluated separately, no globalAIreturn guarantee. Initial failed thinking-address collection excluded; recovered cancelfetches alone Observed. Ordinary supported successor batch included; remaining10intervals44bytes ambiguous code/data or stack-entry role, exact [complement](evidence/pass-003/unknown-complement.json). This is phase easy-gain handoff, not terminal exhaustion.

Validation passed; state saved/current exported. Fresh [checkpoint](pass-003-reviewed-verification.json) byte-identical, mapSHA256 `ec90f1d3ecbd70bfd1110706abd4b74dba98cd88f5e521ce4fd5650bd43cfcfa`; ROM and existing commentary source unchanged. New map current-source commentary pending; do not count old readability as new map completion. No unchanged gameplay replay: existing representative progression/AIcancel evidence reused; natural ending/fullcinematicreturn and specialized outcomes remain unclaimed. No toolkit issues found; [report](toolkit-issues.md). No ROM/toolkit edits, cleanup/deletion, publish or archive.

| Bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 5771 | 2421 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 0 | 8192 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 0 | 8192 | 8192 | 0 |
| 24 | 0 | 8192 | 8192 | 0 |
| 25 | 0 | 8192 | 8192 | 0 |
| 26 | 3815 | 4334 | 8149 | 43 |
| 27 | 0 | 8192 | 8192 | 0 |
| 28 | 908 | 7284 | 8192 | 0 |
| 29 | 0 | 8192 | 8192 | 0 |
| 30 | 7197 | 994 | 8191 | 1 |
| 31 | 2516 | 5676 | 8192 | 0 |

## Round003 redirect completed — localized static batch

Input pass-003-reviewed unchanged262116classified/44unknown. R003-L13ready10-byte8D21 initializationprefix(localSP/D/EB effects, no whole-returnclaim); R003-L14ready9-byte8F24 ordinary-stack masklookupsuffix excluding precedingPLAfragment. Combined19StaticCodeproposalbytes/13instructions,0acceptedbytes; allother25residualbytes explicitlytriaged. Primaryreasoning and rejection-scope corrections at analysis/redirect/round-003-a/structural-review.md; exact44-bytepartition,residual contexts, freshconfigureddecoderandidentityaudit retained. Compacthandoff analysis/handoffs/round-003-redirect.md. Current-source readabilitycomplete/finaleditorialpending. No terminalexhaustion, runtime replay or classification/source/ROM/toolkitchanges; no toolkitissuesfound. Provenprogressionchecklistreviewed/reused. Next independentdisassemblyreview bothlocalizedclaims separately.


## Round004 disassembly completed — incomplete

ROM classification: 99.99% (262135 / 262160 bytes classified; 25 bytes unknown).

Accepted 19 Static Code bytes / 13 instructions: R003-L13 initialization prefix (10 bytes), R003-L14 bounded complemented mask suffix (9 bytes). [Independent review](evidence/pass-004/independent-review.md), [fresh audit](evidence/pass-004/independent-audit.json), [annotations](evidence/pass-004/accepted.json). Local SP/D/EB initialization and ordinary-stack suffix contracts evaluated separately; actual entry and whole-body return unclaimed. Excluded preceding LSR/LSR/PLA and extended-mask/status bytes remain unknown. No supported connected unknown successor found; known initializer, delay and masks already classified. Exact [nine-interval complement](evidence/pass-004/unknown-complement.json) retains 24 bytes in bank26 and one in bank30. No forced generic data or terminal exhaustion claim.

State saved, map exported and validation passed. Fresh [checkpoint](pass-004-reviewed-verification.json) rebuild is byte-identical; map/state SHA256 `8a795941fc84238feed1847b172e3cdfdaa272a53230c9950e98dd3401c2d55b`. Physical classification includes header16 bytes, all PRG counts below. Classification annotations cover262135bytes; execution coverage unchanged and independently recorded, with no new Observed evidence. Existing source and symbols preserved unchanged; full current-source commentary for new map pending, final editorial audit pending. Saved representative progression evidence reused; no unchanged runtime replay or natural ending/full cinematic return claim. No toolkit issues found; [report](toolkit-issues.md). No ROM/toolkit/source edits, agents, cleanup/deletion, publish or archive.

| Bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 5771 | 2421 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 0 | 8192 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 0 | 8192 | 8192 | 0 |
| 24 | 0 | 8192 | 8192 | 0 |
| 25 | 0 | 8192 | 8192 | 0 |
| 26 | 3834 | 4334 | 8168 | 24 |
| 27 | 0 | 8192 | 8192 | 0 |
| 28 | 908 | 7284 | 8192 | 0 |
| 29 | 0 | 8192 | 8192 | 0 |
| 30 | 7197 | 994 | 8191 | 1 |
| 31 | 2516 | 5676 | 8192 | 0 |


## Round005 disassembly completed — incomplete

ROM classification: 99.99% (262138 / 262160 bytes classified; 22 bytes unknown).

Accepted R004-L15 exactly3 Static Code bytes/3instructions, physical[216881,216884), bank26 CPU$8F21 context[26,27,30,31]. [Independent review](evidence/pass-005/independent-review.md), [fresh configured decode](evidence/pass-005/independent-audit.json), [accepted annotations](evidence/pass-005/accepted.json). Two-output adapter selects incoming A bit1 into C then pops separate q into accepted carry-preserving mask suffix; q&15<8 and one argument above valid RTS frame are explicit preconditions. Returns complement mask in A, index in Y, X unchanged, netSP+3. No relation between inputs, actual caller or natural entry claimed. Combined suffix/table dependency and nonredundant carry/payload switch support localized code over coincidental stored values; partial original interface remains compatible.

Remaining22bytes/eightintervals: bank26=21, bank30=1; [exact complement](evidence/pass-005/unknown-complement.json). Other residual decisions reuse sound round-004-a primary review; no widened table, forced data, unknown connected successor or terminal exhaustion claim. Saved progression evidence reused without unchanged runtime replay; coverage unchanged, no new Observed evidence.

Validation/save/export passed. Fresh [pass-005-reviewed](pass-005-reviewed-verification.json) rebuild byte-identical. Map/state SHA256 `0b5b73e2df1a5a4d27387884f1e5323c9d899b095f1fa84a011e68423a1001f0`. Every prior region preserved exactly; ROM and commentary source/symbols/reading unchanged. Prior fully readable11385entries/2295units retained for next full current-source commentary, which must revise old prefix rejection; final editorial audit pending. No toolkit issues found; [report](toolkit-issues.md). No ROM/toolkit/commentary edits, agents, cleanup/deletion, publication/archive. Physical counts include16-byte header.

| Bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 5771 | 2421 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 0 | 8192 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 0 | 8192 | 8192 | 0 |
| 24 | 0 | 8192 | 8192 | 0 |
| 25 | 0 | 8192 | 8192 | 0 |
| 26 | 3837 | 4334 | 8171 | 21 |
| 27 | 0 | 8192 | 8192 | 0 |
| 28 | 908 | 7284 | 8192 | 0 |
| 29 | 0 | 8192 | 8192 | 0 |
| 30 | 7197 | 994 | 8191 | 1 |
| 31 | 2516 | 5676 | 8192 | 0 |


## Round005 redirect completed — terminal incomplete recommendation

Research attempt `analysis/redirect/round-005-a/`: inspected current residual source, exact immutable bytes and decoder evidence; fresh internal-map complement confirms22 Unknown bytes/eight intervals, bank26=21/bank30=1. Input pass-005-reviewed SHA256 `0b5b73e2df1a5a4d27387884f1e5323c9d899b095f1fa84a011e68423a1001f0`;262138/262160 classified. Corrected 8F21 adapter accepted previously; its independent outputs neither widen the eight-mask domain nor create residual entries. Sound prior localized constraints reused after primary review; no useful changed inference/test identified. [Rejection review](redirect/round-005-a/rejection-review.md), [exact ledger](redirect/round-005-a/residual-ledger.json), [identity audit](redirect/round-005-a/audit.json). Zero proposed/accepted bytes; empty redirect pass appended to leads with all historical outcomes preserved.

Saved editor/Done/manual move/status/capture-result/AI-cancel progression reviewed and reused without replay; ending/credits, complete cinematic return and specialized chess transitions unclaimed. Current readability complete11387entries/2296units, [review](round-005-commentary-review.json), [fresh source verification](round-005-commentary-verification.json), [checkpoint verification](pass-005-reviewed-verification.json). No final editorial audit or whole-ROM completion claim. Terminal incomplete recommendation requires coordinator inspection; not blocked. No toolkit issues found; [report](toolkit-issues.md). No ROM/map/state/source/toolkit edits, processes, cleanup/deletion, agents, archive or publication.


## Coordinator terminal handoff — orchestration complete, ROM incomplete

ROM classification: 99.99% (262138 / 262160 bytes classified; 22 bytes unknown).

Accepted terminal incomplete after direct primary-evidence review of round005 redirect. Exact eight residual intervals (21bank26+1bank30) remain code/data ambiguous; no ready lead or useful revised test remains. The earlier input-relation rejection was corrected through round004b and independently accepted in pass005. See analysis/coordinator-terminal-review.md and analysis/redirect/round-005-a/rejection-review.md. No blocked/whole-ROM-complete claim.

Final current-source editorial acceptance passed:11387 scaffold entries,2296 distinct units,8390 embedded unit references; no readability backlog. Supported commentary/audit.json rebuilt byte-identically and refreshed reading-after.asm after final wording corrections and13 inherited missing unit-link repairs. All owned processes finished. Final checkpoint pass-005-reviewed/map-state SHA2560b5b73e2df1a5a4d27387884f1e5323c9d899b095f1fa84a011e68423a1001f0; input ROM unchanged.

Final handoff analysis/final-handoff.md and requirement audit analysis/completion-audit.md link exact residuals, source/readability, fidelity, progression limits and resume rules. archive-options.json plus analysis/archive-selection-verification.json explicitly select additional final/historical Markdown and account for the duplicate scratch omission under inspected archive.py contract. No archive execution, cleanup/deletion, publication or toolkit edits. All input/source/assets/evidence remain local. No toolkit issues found.
