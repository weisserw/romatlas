# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 — disassembly in progress

Verified baseline established: maps/baseline.json, generated/baseline,
analysis/baseline-verification.json. Byte-identical true. ROM identity unchanged.
Configured /root/romatlas/target/release/romatlas used. Mapper 4, 16 physical 8 KiB
PRG banks plus 128 KiB CHR. Game identified visually as Code Name: Viper (Capcom,
1989 title copyright). Boot/title and difficulty menu visited; intro transition
currently in progress. Replays and RPC evidence: analysis/pass-001/batch-log.jsonl;
durable snapshots start.snapshot.json and entered.snapshot.json in that directory.
Classification batching is ongoing; baseline raw bytes are not classification.
No terminal/exhaustion claim. Do not repeat baseline.


## Pass 001 completed — incomplete ROM; next phase commentary

ROM classification: 58.80% (154141 / 262160 bytes classified; 108019 bytes unknown).

Counts come from validated internal analysis/state.json, with unique non-overlapping
physical bytes. Code 14592; non-code 139549 (header, vectors, Text, Image,
and generic/structured Data). Evidence annotations cover all 154141
classified bytes across 636 regions. Exact unknown complement: 228
intervals in analysis/pass-001/unknown.json. Rounded completion is not asserted.

| Physical bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| header | 0 | 16 | 0 |
| PRG 00 | 300 | 999 | 6893 |
| PRG 01 | 653 | 542 | 6997 |
| PRG 02 | 979 | 316 | 6897 |
| PRG 03 | 662 | 511 | 7019 |
| PRG 04 | 0 | 317 | 7875 |
| PRG 05 | 5 | 1408 | 6779 |
| PRG 06 | 0 | 537 | 7655 |
| PRG 07 | 0 | 1341 | 6851 |
| PRG 08 | 0 | 282 | 7910 |
| PRG 09 | 371 | 0 | 7821 |
| PRG 10 | 0 | 611 | 7581 |
| PRG 11 | 0 | 317 | 7875 |
| PRG 12 | 2302 | 0 | 5890 |
| PRG 13 | 1593 | 901 | 5698 |
| PRG 14 | 4459 | 128 | 3605 |
| PRG 15 | 3268 | 251 | 4673 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

Execution is a separate metric: 5776
unique physical instruction starts; 12230 accepted
instruction bytes, including 5692 bytes from scenarios
without debugger edits. 10469 distinct context roots include
aliases and artificial states; this is not a byte classification percentage.
Full counts and per-bank offsets: analysis/pass-001/counts.json. Trace detail
buffers were intentionally capacity0; accepted context aggregates had no aggregate
drops in inspected summaries. Read/detail dropped counts therefore do not imply
missing accepted root evidence. Budget/timeouts stop runs and actual final frames
are recorded; requested counts are not silently treated as completed.

Verified checkpoint: maps/pass-001.json and generated/pass-001/game.asm;
analysis/pass-001-verification.json reports ok=true, byte_identical=true.
Map SHA-256: 2c7c765e75c29d2c26d5f67ca313efbe26f748123cac6bc751eee40fdb0ad7f6.
Input SHA-256 remains 0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Internal state and maps/current.json saved; configured release binary used.
Baseline verification remains retained separately; raw fallback is not evidence.

Gameplay: title/difficulty/briefing; tested password route into middle-stage map,
stage3 forest field and active movement/firing; late-stage map and final boss room
attempts. Current durable active state: analysis/pass-001/action-play.snapshot.json,
frame644, banks [0,1,14,15], screenshot action-play.png. Exact RAM/CPU edits and
buttons/budgets are in analysis/scenarios.json and the local RPC log/scripts.
Player and projectile observed, ammo50->49->48. Jump/climb/crouch inputs were
attempted but their separate effects are unconfirmed. Late-stage map skip timing,
boss dialogue/combat, victory and ending remain unresolved. The alleged ending
password actually selects quest1 stage0; no ending claim. Before handoff those
milestones were explicitly reviewed; practical password/patch attempts are retained.

L001 password contract verified and table slice classified; L002 map/preview wait
contract verified only for the saved stage3 route. L003 external password/ending
claims rejected narrowly for this ROM. L004 boss/ending, L005 reliable late-map
arrival and L006 residual PRG formats are concrete redirect leads. Last additional
24-frame control batch added only82 context roots and the following static closure
43 bytes, so repeated immediate movement was no longer a substantial easy gain.
Further broad progression needs specific transition/entry contracts; redirect can
investigate them. This pass does not claim terminal exhaustion.

No toolkit issues found. Persistent report: analysis/toolkit-issues.md; zero
confirmed bugs, suspected bugs or missing features, no toolkit blocker. Some custom
transport/long exploratory batches were interrupted; their logs are preserved,
and only durable accepted context pages were used for classification. No toolkit
changes, downloads of ROM/assets, deletions, archive execution or publishing.

Current-source handoff: analysis/handoffs/round01-disassembly.md. No source-wide
readability/commentary audit has run; next worker should complete that phase across
all currently classified code/data, then redirect, preserving current evidence.

## Round01 commentary started

Prepared commentary/ from verified pass-001. Frozen baseline and before/ retained;
864 full-ROM scaffold ranges, 14592 classified code bytes and 139549 non-code
bytes are in scope. No earlier readability credit exists. All code remains pending
until mechanical descriptions and consistent names are reviewed. Unknown108019 bytes
retain structural uncertainty. Phase remains commentary until the complete
current-source gate, fresh verification and refreshed reading copy pass.

## Round01 commentary complete — current-source gate

Read and explained the complete pass-001 source: 527 code regions/14592 bytes,
109 non-code regions/139549 bytes, and 228 explicit unknown intervals/108019 bytes.
The 864-entry scaffold is intact and reviewed for readability, with 177 mechanical
units, named operands and branch refinements. Final editorial acceptance remains
pending; classification remains 58.80%, unchanged. No source ranges are deferred.

Final maintained source commentary/source/game.asm and constants.inc freshly
verified byte-identical; commentary/verification.json and refreshed
commentary/reading-after.asm are the readability checkpoint. ROM/map/internal state
hashes remain unchanged. See analysis/handoffs/round01-commentary.md for exact hashes,
covered examples and deferred semantic questions. New redirect questions are the
password $54-loop/54-decimal-table discrepancy and the fixed-ROM $c6ab condition.
Existing boss/late-map/residual-format leads remain. No toolkit issues found;
analysis/toolkit-issues.md unchanged. No final audit, toolkit edit, archive,
deletion or publication. Overall goal remains active; coordinator next runs redirect.

## Round01 redirect active

Current attempt: analysis/redirect/round01-01/frontier.md; primary bounded RPC and static decodes in that directory. Maps/source/ROM unchanged. Investigating L004 boss and scene/data graph, L005 late-stage map timing, L006 residual consumers; password overread and immutable-ROM condition have concrete bounded findings. Combined proposal and residual ledger remain in progress; this is not terminal exhaustion.

## Round01 redirect complete — combined ready batch

Investigation followed connected scene/dialogue/resource/boss/ending paths, corrected late-stage7 flight/preview timing into active mansion mechanics, and independent graphics/dispatch/reset/player-state graphs. terminal.png confirms The End through understood artificial-state route; late-progress.png shows mansion jump/enemy/ammo49. Prior failed patches/rejected premises preserved and narrowed. All32 numeric source-call anchors triaged; missing animation helpers included.

No classification changes:154141 classified/108019 unknown in228 intervals, checkpoint/internal/current SHA2c7c765e75c29d2c26d5f67ca313efbe26f748123cac6bc751eee40fdb0ad7f6 unchanged.33157 unique unknown bytes proposed, accepted new0;3285 observed contextual roots/410 unknown physical starts separately overlap. ReadyL004/L005/L006/L009/L010; password/ROM-condition questionsL007/L008 resolved. Exact proposals, residual ledger, primary decodes/trace pages and routes under analysis/redirect/round01-01/. Current source preserved byte-identical from commentary checkpoint. Handoff analysis/handoffs/round01-redirect.md. Next independent disassembly then full commentary/redirect; overall goal active, not terminal exhaustion. No toolkit issues found; no deletion/publication/archive/toolkit edit. Disk3.4GiB free.

## Round02 disassembly active

Independent review of round01 redirect batch against pass-001 SHA2562c7c765e75c29d2c26d5f67ca313efbe26f748123cac6bc751eee40fdb0ad7f6. New durable review directory analysis/pass-002/. Saved code decodes, consumer proofs, exact generic graphics containers and artificial accepted instruction roots are being reviewed before map.annotate; no baseline rerun or unchanged gameplay replay. Existing commentary remains intact for migration. Frontier: data bounds/roles, code instruction boundaries/effective contexts, then supported connected closure and fresh checkpoint.

## Round02 disassembly complete — pass002

ROM classification: 73.72% (193274 / 262160 bytes classified; 68886 bytes unknown).

Incomplete whole-ROM classification; overall goal remains active. Accepted39133 new unique bytes in one coherent independently reviewed batch;233 unknown intervals remain. Code18409, Data43205, Text588, CHR Image131072. All636 old map entries/evidence preserved exactly. All193274 classified bytes carry evidence annotations. Exact residual complement and per-bank report: analysis/pass-002/unknown.json and classification-report.json.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 300 | 999 | 6893 |
| 01 | 1014 | 1774 | 5404 |
| 02 | 2726 | 378 | 5088 |
| 03 | 671 | 1070 | 6451 |
| 04 | 0 | 8192 | 0 |
| 05 | 257 | 6378 | 1557 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 407 | 753 | 7032 |
| 09 | 490 | 1230 | 6472 |
| 10 | 0 | 4160 | 4032 |
| 11 | 0 | 317 | 7875 |
| 12 | 2302 | 0 | 5890 |
| 13 | 1648 | 1607 | 4937 |
| 14 | 4637 | 202 | 3353 |
| 15 | 3957 | 333 | 3902 |

Header16 bytes Data; CHR banks00..15 each8192 Image bytes classified, zero code/unknown. Data column above includes Text. Physical ROM identities and effective8KiB bank contexts retained.

Independent fresh decodes narrowed L004's two-byte code overrun; a separate pause-consumer proof then classified those bytes Data. Accepted scene/boss/ending/player/reset/graphics batch, actual artificial fetch roots and connected direct control successors. Followed explicit post-mapper stage-introduction/map-flight callees through complete useful render/cleanup/wait routines and bounded screen/motion/probe data. Review rationale, corrected endpoints and provenance: analysis/pass-002/review.md; durable RPC/decodes/accepted manifests in that directory. No unchanged gameplay replay: reused active mansion/frame677, terminal The End/frame2320, and completed progression checklist.

L004/L005/L006/L009/L010 verified with narrow structural roles; L007/L008 semantic answers retained for commentary. Isolated physical58BC return and FE68 singleEA remain unknown. New redirect questionsL011/L012 cover resource selector/bank identity and bank10 checkpoint/spawn records; no terminal exhaustion claim.

Execution metric separate: reused3285 contextual accepted-fetch roots/2777 unique physical starts in artificial snapshots. Current code provenance Observed10694 bytes and Static7715 bytes; these map provenance totals are not exhaustive execution coverage or natural reachability. Static data and inferred code receive no Observed claim. Source readability separate: pass002 has972 code regions/18409 bytes,1145 annotated physical regions; maintained pass001 commentary remains unchanged. Full current-source commentary migration/review is next.

Fresh maps/pass-002.json and generated/pass-002 verified byte-identical, analysis/pass-002-verification.json. Internal/current/checkpoint map SHA256de61d9a48d3713139a2fb103019075d49b8740ce5986b1c42bc5c0a96e905dd1; ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d unchanged. No baseline rerun. No toolkit issues found: zero confirmed/suspected bugs or missing features; no blocker, see analysis/toolkit-issues.md. No toolkit edit, cleanup/deletion, archive or publishing. Handoff analysis/handoffs/round02-disassembly.md.


Round02 commentary ACTIVE: full pass001 tree preserved at analysis/round-2/commentary-pass001; exact current frontier and completed migration verification in analysis/commentary-round02/checkpoint.json. Continue this phase to the entire current-source readability gate before redirect.


Round02 commentary COMPLETE for full current pass002 source: all1378 scaffold entries reviewed,269 mechanical code units (177 carried +92 new), no remaining readability frontier. Fresh byte-identical rebuild and refreshed reading copy; see analysis/handoffs/round02-commentary.md. Overall goal active; whole-ROM classification73.72%, final editorial audit pending. L011/L012 remain concrete next-phase questions.

Round02 redirect ACTIVE: analysis/redirect/round02-01/CURRENT.md is authoritative live continuation; exact unchanged unknown complement in unknown-ledger.json (68886 bytes/233 intervals). L011/L012 consumer tracing underway; no classification/source/map mutation.

Round02 redirect COMPLETE READY BATCH: analysis/handoffs/round02-redirect.md, analysis/redirect/round02-01/{findings,proposals,audit,unknown-ledger}.json/md. L011/L012 updatedready; L013/L014 newready. Proposed48010 unique unknown bytes(6946StaticCode+41064StaticData), accepted0; classification/map unchanged193274/262160,68886unknown233intervals. Conditional residual20876 ifallaccepted. Consumer-led checkpoint/spawn/room/resource, finite source/actor transition closure, independent animation/audio/HUD/collision data packaged;0overlaps/conflicts. Final graph34conservative stops,0uncovered physical continuations. Exact unchanged input/output hashes in audit.json. Next independent disassembly then fullcurrent-source commentary/redirect; not terminal exhaustion/finaleditorial acceptance. No runtime replay, toolkit issues or prohibited mutations.

Round03 disassembly ACTIVE: analysis/pass-003/frontier.md; independently reviewing combined round02 ready batch, preserving previous classifications and commentary.

## Round03 disassembly complete — pass003

ROM classification: 92.04% (241284 / 262160 bytes classified; 20876 bytes unknown).

Whole-ROM classification incomplete; overall goal active. Accepted48010 unique new bytes (6946 Static Code,41064 Static Data) in204 new regions; preserved all1145 prior regions/evidence exactly. 123 exact residual intervals: analysis/pass-003/unknown.json. Counts Code25355, Data84269, Text588, Image131072. All241284 classified bytes have evidence.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6601 | 110 |
| 02 | 4718 | 884 | 2590 |
| 03 | 1790 | 5111 | 1291 |
| 04 | 0 | 8192 | 0 |
| 05 | 257 | 6378 | 1557 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 1380 | 1553 | 5259 |
| 09 | 795 | 2240 | 5157 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2366 | 5817 | 9 |
| 13 | 1769 | 6268 | 155 |
| 14 | 4887 | 894 | 2411 |
| 15 | 5414 | 443 | 2335 |

Header16 Data; CHR00..15 each8192 Image, no unknown. Per-bank counts and exact residual ledger in analysis/pass-003/classification-report.json and unknown.json.

L011/L012/L013/L014 verified independently with fresh3465 CPU/bank instruction views, unchanged saved primary consumers/patterns, independent29 checkpoint streams/558 spawn records/219 terminators/16x31-byte descriptors. Aliases counted once. Critical mapper/audio/scene contracts and generic non-code fallback limits: analysis/pass-003/review.md. Prior even-selector permission remains rejected; $8000/$8001 remain barriers. No speculative actor enumeration or opcode-only code annotation. Original gameplay through late mansion/boss/ending reused; no unchanged replay/baseline rerun. All concrete connected graph continuations already packaged; remaining actor-instance0504/0506 and mixed endpoint questions need redirect research, not terminal rejection.

Execution coverage separate: no new runtime fetch observations; prior3285 contextual artificial-fetch roots/2777 physical starts retained. Code provenance Observed10694, Static14661 bytes, not exhaustive execution coverage or natural reachability. Source readability separate: maintained pass002 commentary untouched; generated/pass-003 has1123 code regions and needs full current-source commentary migration/review next. Final editorial audit pending.

Fresh maps/pass-003.json and generated/pass-003 byte-identical verified: analysis/pass-003-verification.json. Internal/current/checkpoint SHA256ed8d3f1839298b5116956b9a1290df6c932af010b6575ad227fd192497dbdbb6; ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d unchanged. No toolkit issues found; zero confirmed/suspected bugs/missing features, no blocker; analysis/toolkit-issues.md. No toolkit edits, deletion/cleanup/archive/publication or downloads. Compact handoff analysis/handoffs/round03-disassembly.md.

## Round03 disassembly final connected extension — pass003-final

ROM classification: 92.14% (241542 / 262160 bytes classified; 20618 bytes unknown).

Whole-ROM classification incomplete; overall goal active. Accepted48268 unique new bytes (6946 Static Code,41322 Static Data) in208 new regions; preserved all1145 prior regions/evidence exactly. 119 exact residual intervals: analysis/pass-003/unknown.json. Counts Code25355, Data84527, Text588, Image131072. All241542 classified bytes have evidence.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6709 | 2 |
| 02 | 4718 | 884 | 2590 |
| 03 | 1790 | 5111 | 1291 |
| 04 | 0 | 8192 | 0 |
| 05 | 257 | 6378 | 1557 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 1380 | 1553 | 5259 |
| 09 | 795 | 2240 | 5157 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2366 | 5817 | 9 |
| 13 | 1769 | 6418 | 5 |
| 14 | 4887 | 894 | 2411 |
| 15 | 5414 | 443 | 2335 |

Header16 Data; CHR00..15 each8192 Image, no unknown. Per-bank counts and exact residual ledger in analysis/pass-003/classification-report.json and unknown.json.

L011/L012/L013/L014 verified independently with fresh3465 CPU/bank instruction views, unchanged saved primary consumers/patterns, independent29 checkpoint streams/558 spawn records/219 terminators/16x31-byte descriptors. Aliases counted once. Critical mapper/audio/scene contracts and generic non-code fallback limits: analysis/pass-003/review.md. Prior even-selector permission remains rejected; $8000/$8001 remain barriers. No speculative actor enumeration or opcode-only code annotation. Original gameplay through late mansion/boss/ending reused; no unchanged replay/baseline rerun. All concrete connected graph continuations already packaged; remaining actor-instance0504/0506 and mixed endpoint questions need redirect research, not terminal rejection.

Execution coverage separate: no new runtime fetch observations; prior3285 contextual artificial-fetch roots/2777 physical starts retained. Code provenance Observed10694, Static14661 bytes, not exhaustive execution coverage or natural reachability. Source readability separate: maintained pass002 commentary untouched; generated/pass-003-final has1123 code regions and needs full current-source commentary migration/review next. Final editorial audit pending.

Fresh maps/pass-003-final.json and generated/pass-003-final byte-identical verified: analysis/pass-003-final-verification.json. Internal/current/checkpoint SHA2567b1032bc75e57ecef9741788fd7f1127b26c87fb506b6fba76d03cded25ad646; ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d unchanged. No toolkit issues found; zero confirmed/suspected bugs/missing features, no blocker; analysis/toolkit-issues.md. No toolkit edits, deletion/cleanup/archive/publication or downloads. Compact handoff analysis/handoffs/round03-disassembly.md.

# Round03 commentary active

Full pass002 preserved at analysis/round-3/commentary-pass002. Current verified baseline prepared;1145 exact classified ranges carried by physical identity and phase1 dependencies. All151 new Code fragments inspected; purpose notes in progress. Remaining:151 Code fragments (6946 bytes),57 Data entries (41322 bytes),119 Unknown intervals (20618 bytes), affected prior L011-L014/bank contracts, final verification and reading refresh. Exact remaining scaffold ranges are checkpoint.json remaining_ranges. No partial completion or redirect handoff. Final editorial audit reserved.

# Round03 commentary finalizing

All1472 scaffold entries reviewed;1353 classified regions and119 explained unknown intervals. Carried1145 exact prior regions; applied208 new units. Affected room/checkpoint/bank/audio contracts reconciled. Full physical inventory check passed. Fresh byte-identical verification passed; reading-after.asm refreshed. No deferred readability ranges. Final progress/handoff records and a light new-data prose check remain. Final editorial audit pending.

# Round03 commentary — full current-source readability complete

Authoritative current map/state SHA256:
7b1032bc75e57ecef9741788fd7f1127b26c87fb506b6fba76d03cded25ad646.
Entire pass002 commentary preserved before preparation at
analysis/round-3/commentary-pass002. Fresh current baseline verified;1145 exact
classified physical ranges and their valid names/prose/review carried only where
bounds, kind and phase1 dependencies matched. No stale whole assembly copied.

Reviewed all151 new Code fragments (6946 bytes) and57 new Data slices (41322 bytes).
Added208 reviewed physical units; current inventory contains500 distinct unit names,
1472 reviewed scaffold entries and1353 classified regions. All25355 Code bytes,
84527 Data bytes,588 Text bytes and131072 Image bytes have current readability
coverage. All119 unknown intervals (20618 bytes) have honest raw-boundary notes.
No current-source readability range remains pending. Full physical inventory and
source marker checks: analysis/commentary-round03/inventory-check.json. Final
editorial audit remains pending for final acceptance; it was not invoked.

Source additions explain spawn initialization/palettes, continue/results/ending
flows, terrain motion, player contact damage, finite actor behavior/animation paths,
checkpoint matching, title setup, audio command tails, clipped scene strips and
fixed bank services.495 new bank-resolved control references use descriptive labels;
existing RAM/object/hardware constants are reused with exact addressing widths.
Internal entries distinguish decimal formatting, palette/attribute copies and
one/two/three-column rendering. Generic resource containers retain bounded-inspection
notes and uncertainty where the inner schema remains unknown.

Affected prior room/checkpoint prose now separates resource selectors from physical
PRG banks and explains31-byte descriptors,29 six-byte streams and eight-byte spawn
records. Source incorporates the narrow scene-state bank return contract and inline
audio return-address dispatch/Data barriers. Meaning remains tentative for dynamic
actor instances, detailed audio tracks, inner resource packing, some terrain flags
and original stage-seven PPU alias intent. Concrete next questions are in
commentary/uncertainties.md; no semantic research gap is deferred readability work.

Final verification: ok=true, byte_identical=true. ROM unchanged:
0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Final source hashes:
game.asm 9aa3142b2a48dd9577b27b2537faf107f5779cad049932daac6ae236944ff9cf
constants.inc 29cbf23061b12040a3a61ed209f8e29898826f6598f065d26b9e08939e2d8b2a
reading-after.asm refreshed after final verification. Whole-ROM classification is
92.14%, not complete. Overall goal remains active. No ROM/map/toolkit edits,
cleanup/deletion, publication or archive operation. Disk3.1GiB available; commentary
helpers252KiB before final records.

Round03 redirect ACTIVE: separate worker resumed from analysis/handoffs/round03-commentary.md. Input/current/pass003-final map7b1032bc75e57ecef9741788fd7f1127b26c87fb506b6fba76d03cded25ad646;20618unknown bytes/119intervals. Authoritative live research frontier analysis/redirect/round03-01/CURRENT.md and exact unknown-ledger.json; narrow repeated noncode, mixed bank2/12/fixed contracts and actor metadata under investigation. Classification changes remain reserved for independent disassembly. No terminal assessment.

## Round03 redirect ready batch

Unique attempt analysis/redirect/round03-01 completed connected and independent
structural research from119 exact residual intervals. L015/L016 ready, preserving
all14 prior lead outcomes. Proposed20570 distinct currentUnknown bytes:7431StaticCode
and13139StaticData; accepted0. Current map/state unchangedSHA7b1032bc75e57ecef9741788fd7f1127b26c87fb506b6fba76d03cded25ad646,
20618unknown/119intervals. Conditional48bytes/21fragments remain only if allaccepted.
Grouped primary body/control/stack/data contracts and scoped residual dispositions
are in analysis/redirect/round03-01/findings.md and residual-dispositions.json;
exact ledger/proposals/audit preserved. Handoff analysis/handoffs/round03-redirect.md.

Important primary correction: physical130680 CPUFE68 is04, notEA inoldnotes/source
prose; narrowgenericData suffix proposed, no newstage-selectionclaim. No instruction
or Data overlaps/CodeData conflicts; unchanged ROM/source hashes independently
checked. No runtime replay/mutation, map/state/source/toolkit edit, cleanup, archive
orpublication. No toolkit issues found. Independent disassembly review next, then
fullcurrent-source commentary; finaleditorial audit reserved, overallgoalactive.

Round04 disassembly ACTIVE; exact review frontier analysis/pass-004/frontier.md. Input pass003-final retained. FE68 actual04 correction acknowledged; primary structural review underway.

## Round04 disassembly — pass004 verified

ROM classification: 99.98% (262112 / 262160 bytes classified; 48 bytes unknown).

Whole-ROM incomplete; overall goal active. Accepted20570 unique new bytes (7431 Static Code,13139 Static Data),186 regions (75 Code,111 Data). All1353 prior annotations/evidence preserved exactly. Fresh3292 explicit CPU/bank instruction views and111 immutable data slices independently checked. Role/control/stack evidence and exact residual counterinterpretations: analysis/pass-004/review.md; accepted.json, fresh-code-review.json and data-review.json.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6711 | 0 |
| 02 | 4748 | 3435 | 9 |
| 03 | 2157 | 6031 | 4 |
| 04 | 0 | 8192 | 0 |
| 05 | 260 | 7931 | 1 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 4798 | 3385 | 9 |
| 09 | 3851 | 4333 | 8 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2373 | 5817 | 2 |
| 13 | 1769 | 6423 | 0 |
| 14 | 5126 | 3066 | 0 |
| 15 | 5725 | 2454 | 13 |

Header16 Data; CHR00..15 each8192 Image, zero unknown. Code32786, Data97666, Text588, Image131072. Exact21 residual intervals/48 bytes: analysis/pass-004/unknown.json. All262112 classified bytes evidence-annotated. Classification report has allbank counts. L015/L016 verified; Static structural roles do not imply natural selection. No lone opcode/pointer-only Code or invented trailer schema accepted. Primary FE68 correction04 remains required in commentary; no stage10domain claim.

Map validationok; fresh maps/pass-004.json and generated/pass-004 byte-identical rebuild verified in analysis/pass-004-verification.json. Internal/current/checkpoint SHA256db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e; unchanged ROM0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d. Prior map entries preserved. No baseline or unchanged gameplay rerun; broad late/boss/ending routes reused. No new runtime observations. Execution coverage separate: Observed Code10694, Static Code22092 bytes; prior contextual/artificial fetch evidence retained, not full execution coverage.

Current-source readability separate: preserved prior commentary untouched;186 new regions need full current-source commentary migration/review using generated/pass-004. Finaleditorial audit reserved. 48 tiny bytes remain genuinely structurally ambiguous, not a terminalexhaustion decision. No obvious connected easygain deferred; supported combined actor/terrain/helper/data batch applied. Next commentary then redirect for new supported discrimination. No toolkit issues found; zero confirmed/suspected bugs/missingfeatures and no blocker; analysis/toolkit-issues.md. No toolkit/ROM change, cleanup/deletion, archive, download or publication. Handoff analysis/handoffs/round04-disassembly.md.

Round04 commentary ACTIVE: separate worker starts from analysis/handoffs/round04-disassembly.md, current pass004 db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e. Full previous commentary must be preserved before fresh baseline;1353exactprior+186new regions require complete current-source readability. PrimaryFE68actual04 correction and21unknownfragment notes in scope. No redirect/final editorial or terminal assessment yet.

Round04 redirect ACTIVE: separate worker starts from complete analysis/handoffs/round04-commentary.md and exact48bytes/21fragments. Inputpass004 db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e. Current-source1560scaffold fullyreviewed/byteidentical; finaleditorial reserved. Evaluate corrected combined dispatch/stack inference for sharedRTS plus other residuals under evidence-policy; no terminalassessmentyet.

Round05 disassembly ACTIVE: separate independentworker reviews analysis/handoffs/round04-redirect.md and32StaticCodebytesL017-L020 against pass004 db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e. Current48unknown21fragments; conditional16bytes is not acceptedclassification. No terminalassessment.

Round05 disassembly ACTIVE: exact review frontier analysis/pass-005/frontier.md; input pass004 retained. Independent primary evidence review, then fresh bounded decode/apply/checkpoint.

## Round05 disassembly — pass005 verified

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Whole-ROM incomplete; overall goal active. Independent review accepted L017-L020:32 unique Static Code bytes in12fragments. All1539 prior annotation records preserved exactly. Exact primary review analysis/pass-005/review.md; fresh-review.json contains17 configured-bank instruction decodes; accepted.json and rpc.jsonl record application.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6711 | 0 |
| 02 | 4749 | 3435 | 8 |
| 03 | 2159 | 6031 | 2 |
| 04 | 0 | 8192 | 0 |
| 05 | 260 | 7931 | 1 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 4806 | 3385 | 1 |
| 09 | 3859 | 4333 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2373 | 5817 | 2 |
| 13 | 1769 | 6423 | 0 |
| 14 | 5126 | 3066 | 0 |
| 15 | 5738 | 2454 | 0 |

Header16 Data; CHR00..15 each8192 Image, zero unknown. Code32818, Data97666, Text588, Image131072. All262144 classified bytes evidence-annotated; nine exact residuals in analysis/pass-005/unknown.json. Empty handlers require combined table/dispatcher/ordinary-stack contracts, animation wrappers require known interface/counterpart/field contracts, coordinate-copy is ONLY four-instruction fragment (no valid entry/exit or obsolete origin asserted), palette prefix has consumed Y and independently valid scene-resource contract. Natural selection unresolved; no new runtime observation. Execution provenance separately ObservedCode10694, StaticCode22124; not full coverage.

Map validationok; state/current/maps/pass-005.json SHA2569ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e; generated/pass-005 rebuilt byte-identically, analysis/pass-005-verification.json. ROM unchanged0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d. Prior commentary/source protected hashes unchanged, analysis/pass-005/protected-identities.json. Full current-source commentary migration/review required for12new regions; final editorial audit reserved. No baseline or unchanged gameplay replay; completed broad later/boss/ending routes reused. No connected unknown successor or useful local expansion deferred. Residual ambiguity not missing semantics alone; no forced genericData or terminal exhaustion. Next commentary then redirect exact16bytes/9fragments.

No toolkit issues found; zero confirmed/suspected bugs/missingfeatures and no blocker, analysis/toolkit-issues.md. No toolkit/ROM/source mutation, cleanup/deletion/archive/download/publication. Handoff analysis/handoffs/round05-disassembly.md.

Round05 commentary ACTIVE: separateworker migrates fullypreservedpass004commentary to pass0059ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e. Current262144classified/16unknown9fragments99.99%;12newregions32bytes need sourceexplanations/changedunknownnotes, fullscopecheck and freshverify/read before redirect. Finaleditorial reserved.

Round05 redirect ACTIVE: separateworker from complete round05commentary; currentpass0059ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e,16unknownbytes9fragments. Current-source1560scaffold fullyreviewed/byteidentical,168losthistoricalannotations restored and checked. Evaluate consistentstructuralcriteria/newdiscriminators or primaryterminalrejectionreview before stop; finaleditorialauditreserved.

Round05 redirect completed with incomplete classification pending coordinator PRIMARY rejection review. Attempt analysis/redirect/round05-01; exact16Unknown bytes9fragments unchanged, proposed0/accepted0/ready0. All protected identities unchanged audit.json. Per-claim combined-constraint/strongest-competitor review in rejection-review.md with fresh current-source/ROM evidence; no blanket caller/exit prerequisites, no unchanged replay. Completed broad later/boss/ending artifacts reviewed. leads history preserved and empty-result pass appended. Handoff analysis/handoffs/round05-redirect.md. Final editorial audit RESERVED; overall goal active. No toolkit issues found.

Coordinator accepted round05 redirect terminal INCOMPLETE classification after direct rejection/primarysource/ROMbyte review; reasoning analysis/coordinator-rejection-review.md. Current262144/262160classified99.99%,16unknown9fragments, pass0059ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e. No ready/useful revisedmethod survives; no unchanged newcycle. OrchestrationNOTYETcomplete: finaleditorial/read/fidelity and finaldocumentselectionaudit remain.

Final editorial acceptance complete: actual-source/dependency/historical boundary review, eight stale-prose fixes and source-relative review-path correction; all1560 scaffold/727 distinct units reviewed. FINAL supported commentary audit ok=true byte_identical=true editorial_audit=true, refreshed reading copy. Source game.asm28816a872f2c3d663612900ea281ef4c89d553d0a9d1c047af0cc2409ec05293; constants unchangedc9f0335a8b9400cb223f1fcea6fd2addb987ed794b8b572bf7b57f713e826d68. Protected ROM/state/maps/frozen references unchanged. Classification terminalINCOMPLETE262144/262160,16Unknown9fragments; no new research. Handoff analysis/handoffs/final-editorial.md. No toolkit issues found. Overall goal active; coordinator archive-options and final requirement audit pending. No archive execution authorized.


## Documented terminal handoff — coordinator workflow complete

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Justified terminal INCOMPLETE classification after round05 redirect and coordinator direct primary rejection review: analysis/coordinator-rejection-review.md. No ready or useful revised method on unchanged premises; not blocked, not whole-ROM complete. Exact9fragments remain in analysis/pass-005/unknown.json. Allfive sequential disassembly/commentary/redirect rounds and separate final editorial acceptance are complete. Current checkpoint maps/pass-005.json/current/internal SHA2569ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e; input ROM SHA unchanged.

Final source/editorial/readability accepted across1560scaffold entries/727distinct units; supported audit freshly byte-identical, reading copy refreshed, exact source/physical anchors and restored168historical annotations independently verified. Handoff analysis/terminal-handoff.md; final requirement/verification/current audit and archive document-selection records linked there. archive-options.json explicitly preserves final/historical Markdown conclusions and omits only verified duplicate instruction scaffolds. No archive execution, cleanup/deletion, toolkit change or publishing. Resume classification only for new evidence, justified corrected inference/method or explicit reassessment; do not restart baseline or cycle automatically on unchanged sound terminal premises. ROM classification remains incomplete.
