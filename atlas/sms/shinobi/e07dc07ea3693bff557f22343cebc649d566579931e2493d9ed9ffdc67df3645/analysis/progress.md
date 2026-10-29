# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly active
Authoritative inspection 2026-10-29: original SHA-256 matches workspace.json;
262656 physical bytes = 512-byte copier prefix + 16 x 16384 payload banks.
Baseline established once and verified byte-identical: analysis/baseline-verification.json.
Configured release binary used unchanged. Session bridge: analysis/scratch/rpc.py,
Unix socket analysis/scratch/rpc.sock. Runtime initial 124 frames reaches Shinobi
logo; trace aggregates 804 distinct instruction addresses, 1052 contextual roots,
no aggregate drops (bounded detail discarded). Static reset/context discovery
returns 821 candidates, no budget exhaustion; indirect target physical 799 unresolved.
No annotations yet. Current bounded experiment analysis/round-001/explore.py
is advancing logo/title and button1 start; exact calls/results append to
analysis/round-001/route.jsonl. Captures/checkpoints retained under round-001.
Next: inspect title/play output, merge context-backed code, investigate stage RAM
contract for broad progression, bounded static/data inspection and checkpoint.

Resume note: transient worker capacity interruptions terminated the live server;
no baseline recreated. Self-contained analysis/round-001/driver.py now restores
analysis/state.json and latest.snapshot.json (or title snapshot), starts trace,
and saves state/map/snapshot at every completed batch. Session-local history is
not assumed retained. Raw earlier trace summary remains historical, not current.
SMS-001 found: SMS input uses `buttons` array, not shared `button`/`pressed` shape.
Prior ineffective title tests superseded; correct down+button1 transitions away
from title (step2), step3 inspects after 60 frames. See toolkit-issues.md/gameplay.md.

Completed step2/3: correct down+button1 title input enters mission-1 Ken Oh
briefing (level-select-ready.png), so successful game start is confirmed; a level
selection menu is not yet confirmed. select-ready.snapshot.json preserves briefing.
Step4's 120-frame wait ended in black transition, not a confirmed gameplay visit
(mission1-play.png). Step5 adds movement, attack/jump input and complete contextual
root-seeded discovery. Latest completed snapshot before it is latest.snapshot.json
at step4; all driver calls append to driver.jsonl.
Source inspection found SMS discovery uses `max_instructions`, not shared `budget`,
and candidate flags are always Static/false. Complete root seeding and explicit
coverage matching will preserve observed provenance accurately; no speculative
code promotion from pointer values is being used.

Step8 after-release.png confirms ordinary mission-1 gameplay (city, player and HUD),
mode $C006=4; latest.snapshot.json now preserves this natural clean state.
Decoded dispatcher uses low nibble $C006 and table physical $0320; mode 5 enters
stage initialization at CPU $071F, selects assets/maps via stage index $C092,
then writes mode 4. Stage-parameter tables indexed by $C092 establish a useful
artificial progression recipe: restore clean snapshot, patch $C092, patch $C006=5,
run bounded frames and inspect initialized scene/mechanics. Exact domain remains
under investigation; sample indices will be within repeated 19-record table span.
Explicit contextual root seeding (contexts-gameplay.json) and max_instructions=20000
recovered 2537 unique instructions/4866 bytes; coherent decode inspection retained
in candidate-inspection.json and candidate-listing.txt. annotate1.py promotes only
nonoverlapping code candidates with observed provenance only where actual fetch
roots exist; other paths Static. Copier prefix classified separately by envelope.

Classification batch 1 complete: 5616 code bytes across 237 grouped regions, plus
512-byte copier envelope Data; map.validate passed. No candidate overlaps found.
Code provenance is matched to actual saved physical opcode roots, independent of
SMS discovery's always-Static flags. Current state/map saved by driver.
Broad progression underway from clean-gameplay.snapshot.json, exact recipe in
progression.py: restore, clear trace, buttons=[], write $C092=index, $C006=5,
run90 frames, right+button1 30, button2 15, release; capture init/action and roots.
Indices3 and6 completed: stage3 platform/enemy scene, stage6 helicopter encounter
visually confirmed, with 3830/4314 contextual roots; artificial observations only.
Remaining scheduled representative indices10,14,18; terminal/bonus investigation
will be bounded from supported dispatcher/stage contract rather than random RAM.

Static graphics inspection found four-plane RLE consumer CPU $10AB/$10B8 (physical
$12AB/$12B8): tag0 terminates plane; bit7 selects literal versus repeated byte,
low7 count; four planes output interleaved VRAM. Stage asset table bank15 CPU$B2B7
contains 19 valid bank+word descriptors; every pointed stream decodes to four equal
plane lengths and stays in bank. Streams pack contiguously across banks10/11/12,
providing independent boundary checks. Fixed stage graphics at bank2:$8258/$8395
and bank4:$9780 also pass. data-rle-inspection.json records exact endpoints and
plane lengths; unique supported stream bytes=46430. Ready for Data annotations,
without requiring every selector to occur naturally. Full table role/domain and
natural stage names remain separate claims.

Broad representative batch complete: artificial stage indices3,6,10,14,18 all
initialized and actions exercised. Action captures visually establish platform/enemy
scene3; helicopter6; mask-wall boss10; rotating/mandala boss14; final ninja encounter18.
All retain exact per-scenario contextual opcode roots and aggregate summaries with
zero aggregate drops. 4233 distinct physical opcode roots across saved scenarios.
Full static follow-through with max_instructions30000/use_observed=false yields
5336 unique candidate instructions (10937 bytes), inspected in broad-inspection.txt.
No claim that artificial stage routes demonstrate natural reachability.

Stage-selector primary decode mode-52a.txt proves index domain0..18 via wrap at$13;
this independently validates 19-entry descriptor extent used by graphics inspection.
Counted sprite-resource lists at bank15:$AF88 similarly produce82 parsed references,
26 unique compressed streams/19363 bytes, all with equal four-plane output lengths.
Combined exact graphics/list/header/dispatch plus maximal uniform data proposals:
89458 bytes in113 nonoverlapping regions (data-proposals.json); 22759 uniform bytes
claim generic Data only. Whole object/generator semantics are not inferred.
Terminal attempt uses supported level-clear C000 bit1 after initializing stage18.
A300-frame request hit its operation cap; driver saved partial state, then terminal2.py
resumes in5 bounded60-frame chunks. This is an operation budget stop, not toolkit bug.

## Round 001 — disassembly complete, handoff to commentary
ROM classification: 44.73% (117489 / 262656 bytes classified; 145167 bytes unknown).

Internal map: Code=11904, Data=105585; unique physical bytes, including512-byte prefix. Classification is incomplete. Current checkpoint pass-001-final; map.validate passed and full-byte WLA rebuild byte-identical (analysis/pass-001-final-verification.json). Source generated/pass-001-final; current map maps/current.json and internal analysis/state.json. Earlier pass-001 remains historical.

| Physical bank | Classified | Unknown |
|---|---:|---:|
| prefix | 512 | 0 |
| 0 | 7020 | 9364 |
| 1 | 3176 | 13208 |
| 2 | 3573 | 12811 |
| 3 | 83 | 16301 |
| 4 | 3382 | 13002 |
| 5 | 208 | 16176 |
| 6 | 5397 | 10987 |
| 7 | 16384 | 0 |
| 8 | 3168 | 13216 |
| 9 | 1103 | 15281 |
| 10 | 16323 | 61 |
| 11 | 16384 | 0 |
| 12 | 14949 | 1435 |
| 13 | 3920 | 12464 |
| 14 | 13808 | 2576 |
| 15 | 8099 | 8285 |

Final terminal retry confirmed stage18/mode$44 before C000=6; after300 frames stage19/mode$47 and GAME OVER visibly captured (final-clear-stable-4.png). This establishes an artificial terminal-screen visit, not genuine ending/credits or natural victory. First clear attempt was premature during initialization; stable-mode retry is a materially changed precondition. contexts-terminal3.json preserves actual fetches.

Final localized data review classified15615 additional repeated byte/word slices in banks13/14; exact accepted slices in map-data-review.json; no whole-bank/table semantics claimed. Terminal roots added247 code bytes. All data annotations had conflict checks against accepted code; no overlaps/validation errors.

Pass stops at easy-gain handoff, not terminal research exhaustion: broad later encounters exercised; further useful work needs dispatcher/object/asset-table research (leads.json), rather than more repetitive early play. Ending/bonus contracts remain unresolved and specifically deferred to redirect. This is not a claim all practical methods are exhausted. Commentary must cover all currently classified source.

Toolkit: one confirmed bug (SMS-002 execution_coverage flags), one missing documentation/validation feature (SMS-001 controller/budget contract), no blockers; see analysis/toolkit-issues.md. ROM unchanged; toolkit unchanged. Disk5.8GiB free, scratch24KiB; cited round001 evidence retained60MiB. Exact counts/per-bank/unknown intervals in round-001/classification-final.json.

## Round 001 — commentary active
Prepared commentary independently from pass-001-final; frozen before/source identity
verified byte-identical by preparation. Review scaffold has 1002 physical entries.
Current readability frontier: no classified ranges reviewed yet; all 11904 Code
and105585 Data bytes pending, plus honest notes for145167 unknown bytes. Work is
in commentary/source/game.asm; baseline/map/state remain unchanged. Next inspect
vector/main dispatcher/RAM conventions, then all remaining classified streams.

Commentary batch milestone: mechanical prose and entry labels drafted through
physical5257 (classified code only; original gaps preserved), including stage-index
modulo19, stage descriptor/RLE consumer, frame ISR, sprite uploads, VRAM/CRAM and
palette primitives. Two rebuild batches byte-identical. All review credit remains
pending until branch/alias completion. Exact remaining scope and draft units:
commentary/progress.md and review.json. Remaining primary sweep Code>=5257 plus
all Data/Unknown boundary review; then finalize early branches/aliases. No successful
commentary handoff yet. ROM/toolkit/map/state/baseline unchanged.

Commentary batch3 milestone: drafted source mechanics through physical7484,
including object scheduler/animation/pattern queues, metatile expansion, signed
scroll deltas, circular row buffers and VDP column/window commits. Third coherent
verify passed byte-identical and covered the previous label insertion. Remaining
primary code sweep starts7484; Data/Unknown notes and branch/alias finalization
remain pending. Exact draft spans: commentary-notes-batch3.json; no handoff yet.

Commentary batches4/5 milestone: mechanics draft through physical31032,
including primary/player action paths,48-byte object fields, common collision/
projectile spawn/culling, HUD and stage event scanners plus later object states.
Numeric IX/IY offsets now use shared field constants throughout source. Latest
source90089546cf54c8c9d2f8ce85ea4ae4509eed40ab65b11155548eba0330d031ab
verified byte-identical. Primary code remainder>=31032; all Data/Unknown notes
and final branch/RAM/constant/readability checks remain pending.

Commentary batches6/7 milestone: first mechanical-comment sweep now reaches
all11904 classified Code bytes, final code physical102076. Added graphics/sprite
clipping/hitbox and bank6 PSG/FM driver prose. Corrected field-layout reuse:
audio now has its own32-byte-record aliases, distinct from48-byte game objects.
Latest source503f8fa3feba1b829a9941f7984d7827e2ea99eaea961d8fbc70fa73e8219ecf
verified byte-identical. Still pending: entry/branch/RAM correctness finalization,
all classified Data and Unknown boundaries, full reviewed inventory and handoff
verify/reading copy. No successful handoff asserted.

Commentary source-wide draft finalization: all126 Data regions now have
consumer/boundary evidence notes; every Unknown incbin slice explicitly retains
unknown classification. Corrected CPU entry boundaries, named302 relative branch
targets and174 fixed-window helper entries. Readable finite descriptor tables
retain explicit bank15 CPU aliases separate from physical WLA placement.
An initial pointer rebuild mismatch was corrected; latest fresh verify source
a05d00e2a5deb064dbb5ccfb82a432f913b21d821428aceb0eb8e48575453870
passes byte-identical. Remaining: final RAM/constants/consistency pass, complete
review inventory, uncertainties/handoff and final verify+reading copy.

## Round001 — commentary complete, awaiting coordinator review
All current classified source readable:657 Code entries (11904 bytes),126 Data
entries (105585 bytes);219 Unknown entries (145167 bytes) retain honest raw
boundary notes.1002 scaffold entries and283 meaningful units reviewed; no
remaining current-source readability ranges. Final editorial/archive audit pending.

Fresh final WLA rebuild byte-identical, source SHA
a731abfdc12f2e04011d89a7a4f587633d7e3f315c54d536ee0c3107c92c78cc;
commentary/reading-after.asm refreshed after final edits. ROM/map/state hashes
match frozen commentary manifest; checkpointpass-001-final remains unchanged.
Scope/identity details: round-001/commentary-checks.json and commentary-handoff.md.

Outcomes: source-wide routine/control/data explanations; consistent RAM, I/O and
separate gameplay/audio fields; finite descriptors rendered as records. Fixed
interior-entry boundaries, animation-timer/coordinate naming and physical-versus-CPU
pointer aliases, each covered by final verification. Leads
R1-COMMENTARY-DISPATCH/AUDIO/EVENTS ready; no redirect experiments executed.
Unresolved semantics and145167 unknown bytes are structural research, not omitted
readability. Whole-ROM classification remains44.73%; no terminal exhaustion claim.

## Round001 redirect — active r001-static-contracts
Starting from completed commentary and checkpoint pass-001-final (117489 classified,
145167 unknown). Research-only bank-aware static contract batch: object dispatch,
audio, stage events/maps and ending preconditions. Frontier and reproducible probes:
analysis/redirect/r001-static-contracts/. No classification/map/source/toolkit changes.

Redirect milestone: complete static nonzero object-type table and18 local state-table
follow-through collected (8381 decoded instructions, pending independent review).
Audio EC..FF20-command andB4..B8 five-request handlers collected. Bank4 A570
stage/section references resolve33 event streams/598 six-byte records, all adjacent
endpoints and monotonic coordinates agree, ending CPU B3DC. Rejected initial FFFF
coordinate sentinel premise; corrected using identical consumer-compatible terminal
records and independent adjacent pointer boundaries. Research remains active:
code/inline/stack audit, stage-map/audio records and final object/clear contract.

Redirect second milestone:86 valid sound headers and complete mode-aware command
stream graph (5704 bytes, no parse/operand/code/header conflicts) now durable.
Source-rooted fixed/audio closure provisionally adds19302/1218 code bytes; no internal
or accepted Data overlap. Corrected four state-table index0 aliases to JP operands;
those remain Code. Stage scroll records have12-byte stride with16-byte overlapping
copy windows, not disjoint16-byte records. Map/metatile boundary checks and final
phase-chain/bonus distinction continue before combined review handoff.

Redirect connected sprite continuation: exact79-entry CPU1445 descriptor table,
bank2 animation/frame variants89A0/8AE2/9028/91B6 resolve finite lists and counted
2+3*N sprite records. All matching variant frame IDs valid; record bodies endACC9
and55 FF bytes border known codeAD00.79 bank/flag/source/count-validated descriptors
support21184 distinct raw pattern bytes. No accepted Code conflicts. Proposal
assembly includes this connected continuation with object/audio/events/maps.

## Round001 redirect complete — combined review handoff
Research-only r001-static-contracts finished all identified promising original
avenues and productive sprite/bonus continuations. Proposed91124 distinct unknown
bytes:20451Code70673Data,240 intervals, eight ready R1-REDIRECT-* leads. Accepted
checkpoint/map/state/source unchanged:117489 classified,145167 unknown. Conditional
residual if all accepted54043; no actual gain claimed. Exact221 bank-split unknown
ledger and per-proposal CPU/bank contexts retained. Combined handoff:
analysis/redirect/r001-static-contracts/handoff.md. No new toolkit issue, no blocker;
existing SMS-001/SMS-002 remain documented. Next independent disassembly review,
then verified checkpoint/current-source commentary and redirect. No terminal
whole-ROM exhaustion claimed; genuine ending remains unproven.

## Round002 — disassembly active
Input pass-001-final; reviewing combined r001-static-contracts240 proposals independently. Primary consumers, flow closure, stack exits, inline-data exclusions and counted boundaries under review. No accepted changes yet; prior commentary source preserved.

Round002 checkpoint bookkeeping: pass-002-final was created before annotation
persisted after a review-script assertion on a valid fixed0/1 boundary crossing.
It is a preserved baseline-identical duplicate (117489 classified), superseded by
the forthcoming pass-002-reviewed. No partial classification was saved or lost.

Round002 accepted batch persisted:240 Static intervals,91124 new bytes
(20451Code70673Data). Current Code32355/Data176258/Unknown54043.
ROM classification:79.42% (208613 /262656 bytes classified;54043 bytes unknown).
Map validates; pass-002-reviewed fresh rebuild byte-identical, map SHA
6d63cd02fcb98a963584075494af1be2aa5151e9588a00a2f3c0e0c06dd40d48.
Independent review in round-002/review.md and independent-review.json.
Bonus shortcut C000=86 from retained clean gameplay ran420 bounded frames;
init58C4 fetched, but bonus loop5AB3/5B51 not observed and final frame resumes
gameplay.1811 contextual roots, zero outside current classifications. Treat
as attempted mechanic pending explanation; no extra classification claimed.
Prior commentary source/frozen references unchanged.

## Round002 — disassembly complete, awaiting commentary

ROM classification: 79.81% (209632 / 262656 bytes classified; 53024 bytes unknown).
Code32438; Data177194. Input pass-001-final; output
pass-002-complete (phase name only; ROM incomplete). Total gain92143 bytes:
240 reviewed redirect proposals91124, bonus connected routine/table99 and
finite frame-selected collision records920. Eight R1-REDIRECT-* leads verified.
Independent primary review: round-002/review.md and independent-review.json.
Final exact bank-split unknown complement: round-002/classification-final.json.

Correction to interim bonus note:420 frames had not resumed gameplay; CPU58C4
initialization was still active (C620=25, stack return06AE). Retained snapshot
plus150 frames finishes animation;06B8,5AB3,5B51,5C65 fetched and first-person
bonus hands/targets displayed. Left/right/fire inputs exercised. Actual artificial
roots justify Observed bytes only; unobserved connected paths remain Static.
Exact recipe/evidence: round-002/bonus-review.md, bonus-active.png and contexts.
AFae..B00183-byte collision scan plus8-word B161 RAM-slot table accepted; complete
finite434-frame presentation IDs independently support920 bytes of selected
five-byte collision records from B469/B64E. Overlap of first-table index97 and
second-table zero cell retained. Unselected cells/whole-array extent unclaimed.

Map/state validate and pass-002-complete rebuild is fresh byte-identical; report
analysis/pass-002-complete-verification.json. Map/state SHA
e43c7cbb6bcfaa82ae3b32bc9e67bded6128495aa422238c37bf09e8234f85e2. Prior
commentary/source/game.asm remains SHA a731abfdc12f2e04011d89a7a4f587633d7e3f315c54d536ee0c3107c92c78cc;
frozen prior source, historical checkpoints and baseline preserved. New generated
source generated/pass-002-complete/game.asm needs source-wide commentary with
prior prose/labels carried forward. No current-round readability completion claim.

| Bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| prefix | 0 | 512 | 0 |
| 0 | 14696 | 382 | 1306 |
| 1 | 13925 | 1317 | 1142 |
| 2 | 877 | 13523 | 1984 |
| 3 | 0 | 16384 | 0 |
| 4 | 0 | 12768 | 3616 |
| 5 | 0 | 208 | 16176 |
| 6 | 2940 | 13387 | 57 |
| 7 | 0 | 16384 | 0 |
| 8 | 0 | 3168 | 13216 |
| 9 | 0 | 2353 | 14031 |
| 10 | 0 | 16323 | 61 |
| 11 | 0 | 16384 | 0 |
| 12 | 0 | 14949 | 1435 |
| 13 | 0 | 16384 | 0 |
| 14 | 0 | 16384 | 0 |
| 15 | 0 | 16384 | 0 |

Evidence-backed classification, execution coverage and source readability remain
separate. No terminal exhaustion claim: remaining unknown complement is the exact
53024-byte ledger; genuine ending remains unproven. Ready bounded
continuations for later redirect: residual bank2 collision-neighbor code/data
contracts, unselected collision cells and source/length contracts in banks5/8/9.
Do not repeat startup, accepted RLE, or C0006 ending replay unchanged. Bonus
mechanic is now visited artificially; ending still attempted, not established.

Toolkit: no new finding or edits. Existing SMS-001 documentation/validation and
SMS-002 discovery provenance bug remain with workarounds in
analysis/toolkit-issues.md; configured release repository HEAD
a15778b53a4b6c4131f84f5ea8bd7d4eed0767d0 unchanged. Full hash identity and
verification details in round-002/classification-final.json; compact handoff
round-002/disassembly-handoff.md.

## Round002 commentary active

Prior commentary archived in analysis/round-002/commentary-prior before baseline preparation. Physical-range carry, dependency checks and all new classified readability remain pending; resume commentary, not redirect. See commentary-frontier.md.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 180}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA c103bfc155dc880b5485a7394c86c18b4bd0855076d32208d51b5291fd99228f.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 141}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 81a1b9257005dddc3702446c2293ce56e588f6adde237abde36bf9ce2c060353.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 139}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 6c8135fa7f48d897a5179cac4b5f409cd48d096dbcf6828a0cf92d24ef6e610b.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 127}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 1927d61c512e32d39b7c6406811f31d8378e9d6473a0c084acb6fea4a39b71c8.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 125}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA cdcf8ffa4e7ed7fc3b04158e48b1eae3f038294f4a30ec78979571c2a4f81b67.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 125}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 5a0f4c353c030ce939251bfb7c086c8950b6573320fee245a1cbae577a45d2f0.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 105}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 82b1747a5a689e76a701ac9fa8ad6c685260de033ebc6fff1de5eed3b02c449c.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 103}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 1c01ec73c7717f9a166b0165564c7fb491d97f017e57dfe4ec612d579c38b4f4.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary active checkpoint

Reviewed new Code through0x49C6; 103 Code ranges/10850 bytes still pending. Full readability incomplete. Exact continuation: analysis/round-002/commentary-resume.md and commentary-frontier.json. Fresh byte-identical verify and refreshed reading copy, source SHA1c01ec73c7717f9a166b0165564c7fb491d97f017e57dfe4ec612d579c38b4f4. No successful handoff or redirect readiness asserted.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 97}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 1f9ae2ef6006909c128068aa56d393a616933566b29c9b30bb933bda3dbd4a7a.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 96}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 7f585725b2005415e93601a3e77f6eefe105673b0e33ba03949f84e5f59c971a.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 88}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 3e8647ff52a27b23c703c962d5c4d0b1969d2d5aa375938deaa3be723536c747.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 72}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 6a013973cbe69799c1de894e5ea1daa41d6f4e6963d0f19bd5e3ca15c1edaab9.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 65}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA ac1f2eef37dccf963643b6696cc7e740885b6bd600dca22291502a6fad69a89b.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 53}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 68c035ebe6b763749da8f4535723ceb378eb87fb74a6e8ae1dee1862b5c5af8e.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {'Code': 31}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA b3ead4098dae404b3890828c4bc4ce63cf62b4448dee89d401afcf87d9ccb27e.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 39587d6a91c0f3597a836e4d903489bed5eb011fa49b7131a56b30f9249b7912.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 082de24f9e0678c96aa631eed0efed2c1d7acf0c56584bf91d098552ac08e5a7.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary milestone

Authoritative checkpoint pass-002-complete. Entire prior commentary preserved at
analysis/round-002/commentary-prior/. Fresh baseline verified byte-identically.
Physical-range carry preserved all 783 prior classified regions, 283 units and
816 unchanged scaffold reviews. Changed boundaries were reviewed independently.

Data pass adds 99 named boundaries and format/consumer notes across all new Data,
including sprite variants, counted records, sound headers/streams/envelopes/sample,
stage events/metatiles/maps, bonus waves/sequences and selected collision cells.
New Unknown boundaries have honest raw-byte notes. Existing valid prose retained.

Current pending review: {}; exact ranges and source locations in
analysis/round-002/commentary-frontier.json. Next: Code ranges in physical order;
then dependency corrections in carried prose, full readability assessment and verify/read.
No successful handoff, redirect readiness or whole-ROM completion claimed.
Fresh current batch verification byte-identical; source SHA 7cc3448d90e5b5a917773319776257532970aa10319ac8b880269fd77aab96d0.
Final editorial_audit remains pending. No toolkit/ROM/map/state edits.

## Round002 commentary complete

Current-source readability completed on pass-002-complete. All1129 full-ROM
scaffold records and725 meaningful units reviewed, no pending ranges.
Scope:32438 Code /177194 Data /53024 Unknown; classification79.81% remains
separate from readability. Newly covered20534 Code and71609 Data bytes; prior
valid783 classified-region reviews,283 units and names/prose were carried rather
than rewritten. Entire prior commentary remains in commentary-prior/.

All current fixed-window actor/encounter states, bonus scene/waves/commands,
bank-2 collision scan, bank-6 readers/commands/envelopes/sample and finite data
containers have purpose/mechanics/bank/format notes and useful symbols. Stale
unknown-continuation claims, context-dependent RAM aliases and scalar/address
confusions corrected. Unknown bytes retain honest boundary/consumer notes.
Preserved opcode aliases5451/type0, four state index-zero JP operands, B8/8381,
source stride12/copy16, slot-specific audio readers, event terminal00100000D0FF,
39-byte bonus tail and collision-cell overlap. No structural reclassification.

Fresh byte-identical verify passed; source SHA
7cc3448d90e5b5a917773319776257532970aa10319ac8b880269fd77aab96d0.
reading-after.asm refreshed. Map/state/baseline SHAe43c7cbb...4f85e2 and original
ROM/prior-source identities unchanged; commentary-checks.json has full identities.
Configured toolkit binary used; no toolkit changes, no publish/export.

Assessment: full inventory/dependency readability check complete, not merely a
count gate. Previous sound review was reused; new regions and affected consumers
were inspected. Final editorial_audit stays pending for acceptance; whole-ROM
classification is incomplete. No audit command was run.

Leads for later redirect: R2-COMMENTARY-FREE-MOTION-RETURN (pushed CPU2546,
Unknown2746..2796); R2-COMMENTARY-BOUNCE-PROFILES (three6-byte records at
CPU494C/physical4B4C..4B5E); R2-COMMENTARY-ENCOUNTER-OFFSETS (eight2-byte
pairs CPU7BA5/physical7DA5..7DB5). Source/uncertainties describe each bounded
consumer. Exact character/effect identities, natural selector coverage and a
genuine ending/credits route remain unresolved. No repetitive replay performed.

R1-COMMENTARY-DISPATCH/AUDIO/EVENTS outcomes are reflected in accepted round002
classification and source readability; this phase executed no redirect work.

Metadata scope correction: removed124 preceding/following disjoint Unknown-gap excerpts from unit summaries, including SampleGameplayController, and reconciled18 concrete stale summaries with current source. Actual Unknown scaffold/source notes retained. Fresh verification/read passed afterward; source SHA unchanged.

## Round002 redirect active

Attempt analysis/redirect/r002-residual-contracts; input pass-002-complete, 209632 classified /53024 Unknown. Commentary gate and byte-identical verification checked. Exact bank-split unknown complement/neighbors persisted. Research only; map/state/source/ROM/toolkit unchanged.

Round002 redirect milestone: r002-residual-contracts/contracts.md now retains114-byte player/bounce/encounter contracts; new bank8 encounter streams13216 bytes and bank9 streams6920, bank5 briefing382, selector304, bank9 bonus-copy3840 and font residual2215. Collision final static closure805 instructions/350 newly Unknown under supported direct-call/pushed-return roots; table aliases still under review. Frontier continues fixed mode entries and residual lookup tables. No map/state/source/ROM edits; no new toolkit findings.

Round002 redirect continuation: stage pointer arrays0B62/0B88 connect all five bank5 graphics/palette records and exact endpoints, resolving full16176-byte unknown bank5. New bank4 global-effect and prerecorded-input contracts resolve3616; new bank12 bonus1363 packed+72 tile-layout bytes resolve1435; bank10 tail61FF. Main-mode roots01DA/0464/04D2 additionally establish bank2 two-plane560+rectangle40 input bytes. Detailed primary evidence, failed/corrected initial bank5 root and unchanged hashes retained under attempt; remaining fixed lookup/helper/collision packaging active.

Round002 redirect proposal checkpoint:118 exact proposals53009 bytes (1237Code,51772Data), accepted0; six localized intervals15 bytes remain unproposed. Primary-evidence residual review and compact combined handoff saved under r002-residual-contracts. All identified promising connected/independent resource/table/helper avenues packaged; final bookkeeping validation/lead registration pending. Current accepted Unknown remains53024 and current commentary/source unchanged.

## Round002 redirect complete

Combined118 proposals53009 bytes (Code1237,Data51772), accepted0. Eight ready lead IDs/pass persisted; old IDs/outcomes preserved. Exact accepted53024 complement/neighbors/dispositions and conditional15-byte residual retained with localized primary rejection review. verification-checks.json validates unknown-only/nonoverlap/bank/CPU accounting and unchanged ROM/map/state/source/configured-binary identities. Handoff analysis/redirect/r002-residual-contracts/handoff.md. All identified promising connected and independent avenues investigated/packaged; no research frontier or active server remains. Next independent disassembly, followed by full current-source commentary. Existing SMS-001/002 only, no new toolkit findings/edits. No whole-ROM completion, accepted-byte gain, natural reachability or terminal exhaustion claim.

## Round003 disassembly active

Independent review of118 round002 proposals; primary contracts, helper algorithms and localized residual review read. Exact consumer/boundary/code-context review in progress. No accepted changes yet. Six thin exclusions retained; commentary/source and frozen histories preserved.

## Round003 disassembly complete, ROM incomplete

ROM classification: 99.99% (262641 / 262656 bytes classified; 15 bytes unknown).

Accepted118 proposals53009 bytes after independent review:1266Code/51743Data. Total33704Code/228937Data. Corrected29 blanket-remainder proposal bytes to Code:physicalB4E1..B4F1 andB507..B514, explicit mapper2 entries and coherent HUD/score algorithms. Original proposals/history preserved. Eight ready lead IDs verified; exact outcomes in round-003/lead-outcomes.json. Static provenance throughout; no new execution/natural reachability claim.

| Bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| prefix | 0 | 512 | 0 |
| 0 | 15195 | 1186 | 3 |
| 1 | 13925 | 2452 | 7 |
| 2 | 1592 | 14792 | 0 |
| 3 | 0 | 16384 | 0 |
| 4 | 0 | 16384 | 0 |
| 5 | 0 | 16384 | 0 |
| 6 | 2992 | 13387 | 5 |
| 7 | 0 | 16384 | 0 |
| 8 | 0 | 16384 | 0 |
| 9 | 0 | 16384 | 0 |
| 10 | 0 | 16384 | 0 |
| 11 | 0 | 16384 | 0 |
| 12 | 0 | 16384 | 0 |
| 13 | 0 | 16384 | 0 |
| 14 | 0 | 16384 | 0 |
| 15 | 0 | 16384 | 0 |

Fresh map.validate/save/export succeeded; authoritative unique checkpoint pass-003-reviewed byte-identical, map SHAff0fa7f0835dc3fe700c5d3cdb0409a91dc11408c1eadcba29f3787eca140f35. See analysis/pass-003-reviewed-verification.json. pass-003-complete is a superseded unchanged-input checkpoint created after an initial annotation request rejected missing SMS id field; no mutation had occurred, old map SHAe43c7cbb...4f85e2. It is preserved, not the accepted round003 result.

Independent667-instruction decode,16 direct-ROM resource reparses, old/new Code control targets versus proposed Data checked. Exact15-byte/six-interval residual ledger and sound localized rejection review retained. No useful changed gameplay contract justified replay; bonus/later encounters previously visited, genuine ending unproven. Existing scenarios remain authoritative. No terminal exhaustion claim; next full current-source commentary and redirect.

Current commentary/source SHA7cc3448d90e5b5a917773319776257532970aa10319ac8b880269fd77aab96d0 remains unchanged and complete for input pass002 only. All commentary/frozen history preserved for carry/archive. New generated/pass-003-reviewed requires full commentary, including misplaced SpawnEncounterOffsetEffectCycle at physical1D4D versus actual consumer42400+ correction. All opcode/tile-list aliases, bank views and source/length corrections retained.

Toolkit summary: no new finding or toolkit edit. Existing SMS-001 missing documentation/validation and SMS-002 discovery provenance bug remain with bounded workarounds; no blocker. See analysis/toolkit-issues.md. Configured release/ROM unchanged. Storage bounded, scratch28K; evidence/identities in round-003/classification-final.json. Compact handoff round-003/disassembly-handoff.md.

## Round003 commentary active
Entire previous commentary preserved at analysis/round-003/commentary-prior before fresh prepare from pass-003-reviewed (map/state ff0fa7f...140f35). Fresh preparation byte-identical. Carried 1035 unchanged physical classified ranges and 725 meaningful units; no blanket reread/rewrite. Carry rebuild byte-identical (source dd936d71...37f11). Six residual Unknown intervals preserved with honest boundary notes. Current frontier: 118 newly classified regions (1266 Code/51743 Data), affected consumers and targeted misplaced SpawnEncounterOffsetEffectCycle label. Exact initial ranges: round-003/commentary-frontier.json. Current-source readability incomplete; final editorial audit pending and not run. Resume same commentary phase.

### Round003 commentary complete
Full current-source readability complete on pass-003-reviewed:33704Code/228937Data plus honest15-byte/six-interval Unknown notes. All1159 scaffold records/795 meaningful units reviewed; no remaining ranges. Preserved entire prior commentary and carried1035 classified physical blocks/725 units, byte-identically reverified. All118 new regions1266Code/51743Data assessed; primary consumers/resource contracts reused. Fixed player-input1D4D mislabel and actual modulo-eight encounter entry7D4D/call/table/context; removed stale free-motion/bounce/offset Unknown claims. Added mode/VDP/math/plane/RAM/camera, bank2 collision/BCD/HUD/FIFO and bank6 helper explanations with descriptive symbols; preserved documented aliases.
Fresh final byte-identical verify source d73f6df5c749909434fbacb9a92085b17940e1d85fbdd2535b1e3d2b22c57f88; reading copy refreshed. Map/state/baseline ff0fa7f...140f35 unchanged; ROM/prior source unchanged. round-003/commentary-checks.json and commentary-handoff.md persist assessment/identities/outcomes. Six structural residuals, selector/game identities, partial generic formats, unknown helper callers and genuine ending remain explicit. R2 leads resolved; no new structural question, replay, toolkit/classification edit or audit. Current-source readability complete, whole-ROM classification99.99% incomplete, final editorial/archive acceptance pending.

## Round003 redirect complete; six-byte proposal

Unique attempt analysis/redirect/r003-prefix-contract-review jointly reexamined all15 residual bytes against primary source, exact neighbors, stack/data-flow alternatives and accepted standalone-helper standards. Corrected prior wrapper sufficiency inference: two duplicated HL-to-IY bridges, physical588A..588D and5A54..5A57, supply the complete known IY initializer immediately before HL overwrite and are independently skipped by allocator paths. Proposed6 Static Code bytes; actual caller unknown. Remaining9 bytes in4 intervals retained with localized rejection review. All identified actionable structural avenues assessed; broad gameplay/history retained, genuine ending unproven, no unchanged replay. Map/state/source/ROM/configured workspace identities unchanged; no classification/toolkit/editorial mutation. Ready independent disassembly; handoff at attempt/handoff.md.

## Round004 disassembly active
Independent primary review of two HL-to-IY bridges, physical588A..588D and5A54..5A57; current map/state pass-003-reviewed unchanged. Fresh bank-aware decoding and exact shared-body/stack/boundary validation in progress. Other9 bytes remain Unknown.

## Round004 disassembly complete, ROM incomplete

ROM classification: 100.00% (262647 / 262656 bytes classified; 9 bytes unknown).
Exact percentage 99.9966%; rounded100.00% does not mean complete.

Accepted two3-byte HL-to-IY event adapters, physical588A..588D and5A54..5A57, Static Code. Independent fresh64-instruction decode and primary source excerpts confirm balanced stack, immediate HL kill, dependent IY object-field writes, duplicate allocator adapters, exact JR-around boundaries and complete RET bodies. Actual caller/natural reachability unresolved. Total33710Code/228937Data. Prior region records preserved. Review and exact outcomes: analysis/round-004/independent-review.md, code-review.json and lead-outcomes.json.

| Bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| prefix | 0 | 512 | 0 |
| 0 | 15195 | 1186 | 3 |
| 1 | 13931 | 2452 | 1 |
| 2 | 1592 | 14792 | 0 |
| 3 | 0 | 16384 | 0 |
| 4 | 0 | 16384 | 0 |
| 5 | 0 | 16384 | 0 |
| 6 | 2992 | 13387 | 5 |
| 7 | 0 | 16384 | 0 |
| 8 | 0 | 16384 | 0 |
| 9 | 0 | 16384 | 0 |
| 10 | 0 | 16384 | 0 |
| 11 | 0 | 16384 | 0 |
| 12 | 0 | 16384 | 0 |
| 13 | 0 | 16384 | 0 |
| 14 | 0 | 16384 | 0 |
| 15 | 0 | 16384 | 0 |

map.validate/save/export succeeded; fresh unique pass-004-reviewed checkpoint byte-identical. Map/state SHA91d37255c7b1b6a741a5ad615ae9f7f434186be7ff451dc0e82b513ffd684a0f; verification analysis/pass-004-reviewed-verification.json. Four residual intervals9bytes:0C65..0C68,4CE1..4CE2,182DE..182DF,18D3A..18D3E. Reused localized primary rejection review; no speculative data promotion or terminal exhaustion claim. No changed gameplay discriminator justified replay; broad earlier gameplay and genuine-ending uncertainty preserved.

Current full commentary/source d73f6df5c749909434fbacb9a92085b17940e1d85fbdd2535b1e3d2b22c57f88 preserved unchanged for carry/archive. It is complete for pass003; new pass004 needs current-source commentary, especially two former Unknown bridge notes and labels. Final editorial audit pending. No toolkit edits or new findings; existing SMS-001 documentation/validation and SMS-002 provenance bug remain nonblocking (analysis/toolkit-issues.md). Configured release and ROM unchanged; reusable scratch28K, free5.6G.

## Round004 redirect terminal incomplete

Research-only attempt analysis/redirect/r004-residual-role-review completed current-source and localized primary-artifact rejection review. No ready leads/proposals, accepted mutations, productive deferred frontier or changed runtime discriminator. Four exact residuals0C65..0C68,4CE1..4CE2,182DE..182DF,18D3A..18D3E remain9Unknown;33710Code/228937Data=262647/26265699.9966%. Map/state pass-004-reviewed91d37255c7b1b6a741a5ad615ae9f7f434186be7ff451dc0e82b513ffd684a0f unchanged. Current-source readability1159regions/797units and fresh byte-identical verification retained; source3e8b115cd431a428ad2528115a6a19c9e8bb2d2e60563206305cd1d40373e789. Final audit not run. Broad progression preserved; genuine ending remains unproven; no unchanged replay. No toolkit edit/new finding (SMS-001/002 existing). Exact neighbors, alternatives, primary links and stopping rationale in attempt/rejection-review.md; compact handoff attempt/handoff.md and round-004/redirect-handoff.md. Stop incomplete absent concrete new evidence/corrected method.

## Coordinator terminal assessment

Inspected decisive primary rejection artifacts and independently recomputed current map complement: 33710Code/228937Data/9Unknown, four intervals. Accepted sound incomplete stop; no useful corrected inference/test or productive frontier remains. Readability1159regions797units complete, fresh byte-identical source identity checked. Final editorial/archive acceptance remains pending; whole-ROM completion unproven. See round-004/coordinator-assessment.md. Do not relaunch unchanged research.

## Goal blocked after third terminal revalidation

Same nine-byte structural evidence limitation persists; zero ready leads or active workers. Source/map identities unchanged, classification99.9966%, current-source readability and byte-identical verification preserved. Whole-ROM completion/final editorial acceptance remain unproven. See round-004/coordinator-assessment.md; resume only with new evidence or a concrete corrected inference.

## Explicit user-directed generic Data classification; final audit pending

User instruction "Mark those bites as generic unclassified, DATA, and proceed with the final audit review" supersedes the prior terminal Unknown disposition, without changing its preserved evidence/rejections. Added9bytes in4physical half-open intervals0C65..0C68,4CE1..4CE2,182DE..182DF,18D3A..18D3E as Manual Data, confidence0. Original role remains UNKNOWN; no recovered non-code proof, semantic role, execution or natural reachability is claimed. All previous regions and completed historical records preserved.

Current33710Code/228946Data/0Unknown:100% map coverage, explicitly comprising262647prior evidence-backed bytes plus9user-directed Data bytes. map.validate/save/export and unique checkpoint pass-005-user-data succeeded, byte-identical. Map/state SHAc3dfce87e73e79fc955752587d27409a65de5eb37300def81d12d1b342e73bb1. ROM/toolkit/commentary source unchanged; source3e8b115cd431a428ad2528115a6a19c9e8bb2d2e60563206305cd1d40373e789. Commentary reconciliation and final audit remain pending. No new toolkit finding; existing SMS-001/002 persist. Decision, full added annotations, archived pre-state and audit handoff: analysis/final-user-data/. Previous terminal statements remain historical; new user direction changes classification disposition, not structural evidence.

## Final editorial acceptance after pass-005 user disposition

Final audit succeeded:1159 reviewed physical regions,798 unique meaningful units/1304 embedded records; fresh byte-identical rebuild and refreshed reading copy, no pending checks. Map coverage100% (33710Code228946Data0Unknown), explicitly262647 evidence-backed+9 user-directed generic Data confidence0, original role UNKNOWN. Entire prior commentary archived before fresh baseline; maintained source hash41239d34f2c4508149d106d5b24388ea7fe5f4cfcbbc014fec61913383babff9 matches audit/verification. Detailed scope/findings/identity assertions and unresolved ending/semantics limitations: analysis/final-audit/handoff.md and checks.json. Historical stopping/rejection records above describe earlier evidence states and remain unchanged.
