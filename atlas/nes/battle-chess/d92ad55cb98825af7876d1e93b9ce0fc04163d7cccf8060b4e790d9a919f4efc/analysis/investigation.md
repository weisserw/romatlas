# Investigation ledger

## Round001 — disassembly easy-gain pass completed, ROM incomplete

Policy/workflow and initial empty state inspected. Verified raw baseline established once; preserved. Physical ROM identity unchanged. Main supported claims:

- R001-L01 verified: reset vector$C000, NMI$C4EF, IRQ$C4EE; effective reset slots[0,1,30,31]. Accepted fetches decoded individually with exact bank views and non-overlapping boundaries:2689natural observed code bytes. Static ordinary branch exploration adds959bytes (450instructions), with mapper/call barriers, no KIL/BRK or overlap conflict. Setup/menu paths add1251bytes (587instructions); those observations are artificial after RAM timer edit. Static structure and natural reachability kept distinct.
- R001-L05 verified: banks2-18 contain paired planar raster patterns/continuous fragment runs, supported by local tile sheets; classified Image slices139264bytes. Bank19 shows graphics-to-nibble transition; banks20-23 have smooth/repeated nibble streams, incompatible with coherent executable structure;40960generic Data bytes. Checked available roots/references/PPU copy consumers, nearby boundaries and absence of conflicting code, without claiming logical asset extents or original format semantics. Generic data fallback used where exact audio/record meaning remains unresolved. FF runs1428bytes in mixedbanks classified separately using exact uniform bounds, raw transitions and conflict checks. Header16/vector6 validated Data.
- R001-L02 attempted/open: confirmed initial3Dboard, Move/Settings/Level menu, Move submenu and distinct2D Setup Board editing screen. RAM arrays$04BE/$04FE have standard64-square board encoding; patching both to bluekinga8/redqueenb6/redkingc6 visibly changes editor. Supported Done acceptance not confirmed: bounded input alternatives remain editor/submenu. No capture/AI/checkmate/ending claim. Exact recipes, budget stops, snapshot paths and milestone checklist are in [scenarios](scenarios.json); source findings in [gameplay](gameplay.md).
- R001-L03/L04 open: bank0engine control/return contracts, bank28setup handlers, mixed data/scripts/pointers in banks1/24-29 and fixed-bank gaps. These need targeted consumer/entry/context research, not speculative whole-bank decoding. Exact unknown complement saved, no raw fallback counted. Redirect should resolve menu-ready/Done handler before repeated play; this easy-gain handoff is not terminal exhaustion.

Evidence: `evidence/pass-001/observed-instructions.json`, `static-candidates.json`, `later-instructions.json`, `noncode-review.json`, `uniform-review.json`, `final-contexts.json`, `unknown-complement.json`, selected frames and same-build snapshots. No trace stream was emitted; detail capacity0 intentionally discards events while aggregate_dropped=0. Final trace2795unique instruction starts,2912contextual roots,3181contextual edges; execution coverage separate from classification. Final12traced frames include1new root not incorporated into map; no need to infer a structural gain without review.

Final [checkpoint verification](pass-001-final-verification.json): `maps/pass-001-final.json`, SHA25677d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59, generated/pass-001-final. map.validate passed, generated ca65 assembly byte-identical. All1144regions carry evidence; source readability remains commentary work. State saved toanalysis/state.json, portable mapmaps/current.json. No toolkit issues found; see [toolkit report](toolkit-issues.md). Expected execution time/budget stops retained as scenario outcomes, not toolkit defects.

ROM classification: 71.17% (186573 / 262160 bytes classified; 75587 bytes unknown).

Counts: Code4899, Image139264, Data42410 (includes16header,6vectors,1428uniformFF,40960stream slices); unknown75587. Header lies outside PRG bank table below.

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

## Round001 redirect A — completed ready combined proposal

See [primary review](redirect/round-001-a/review.md), [frontier/history](redirect/round-001-a/frontier.md) and [handoff](handoffs/round-001-redirect.md). Corrected overbroad script-bank/frame-domain barriers using primary consumers and independent encoded origins; investigated all8scriptfamilies/all1510composite frames rather than first-ready-lead stopping.6293new artificial observed code bytes,6441script bytes and bounded menu/PPU/audio/graphics data proposed; current map unchanged. Exact75587unknown ledger,63007candidate union and12580unproposed. Remaining code/data ambiguity and recursive engine semantics are honestly separated. No terminal rejection review required because supported ready batch exists. New/updated leads and negative method history retained in leads.json. No toolkit issues found.


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
