# Disassembly progress

round=1 phase=disassembly, ongoing. ROM unchanged; verified raw baseline established:
[maps/baseline.json](../maps/baseline.json), [baseline verification](baseline-verification.json).
Classification not yet committed. Live frontier is local scripts/evidence in
[round-01](round-01/), startup snapshot title.snapshot; initial 180-frame startup
coverage 815 physical instruction starts, 1001 contextual roots (first 1000 saved).
All-kind startup trace request timed out; replacement ran 18 x 10 frames with
Instruction filter, capacity zero, aggregate_dropped=0. Details intentionally dropped.
Title screen identifies Conan, Mindscape 1990. Gameplay script currently rerunning
with correct NES input contract `{port:1,buttons:[...]}`; the earlier generic
`button/pressed` parameters were ignored and produced no actual button presses.
Do not reuse that prior route as gameplay evidence. Static vectors validated:
NMI $CAA6, reset $CAFB, IRQ $CAE2 in fixed physical banks 14/15.
Fixed-CFG exploratory candidate set is NOT classified: inline words after helpers
$D4D1/$D55D/$D5E6 require barriers (consumer $D611). Correct traversal before commit.
Next: collect actual gameplay windows, inspect stage RAM/dispatcher, classify
observed starts plus supported CFG; save map/state then fresh verified checkpoint.

Intermediate verified checkpoint: round-01-startup-gameplay, byte-identical.
ROM classification: 12.41% (16267 / 131088 bytes classified; 114821 bytes unknown).
10855 code bytes, 5412 data bytes. 5259 decoded evidence-supported instruction
starts; 457 regions. Structural code from accepted ROM opcode fetches and direct
CFG; static fixed-bank traversal honors two-byte inline helper arguments.
Gameplay shortcut verified: restore actual-game-start.snapshot, release inputs,
write RAM $52=2 and $04AA=1; advance 10 traced +100 untraced +10 traced frames.
Result stage=3, $04AA cleared by full loader, intact Viking ship screen.
Broad milestone exploration ongoing through index 1,2,4,5,6,7,8, using same transition.


# Round 1 disassembly pass completed; overall ROM incomplete

round=1 phase=disassembly status=pass-complete; next phase commentary.
ROM classification: 14.54% (19055 / 131088 bytes classified; 112033 bytes unknown).

Code 13531 bytes; data 5524 bytes, including 16-byte header.
639/639 classified regions have evidence; 6462 supported decoded instruction starts.
Execution is separate: at least 4129 unique fetched opcode starts in captured contextual pages;
2473 in natural startup/gameplay windows and 3580 in artificial windows (overlap included).
The startup context page omits one of 1001 roots; no unrecorded root was used for classification.
Zero aggregate drops in saved trace summaries; detail capacity 0 intentionally drops detail.

| Physical 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 128 | 8064 |
| 1 | 0 | 1537 | 6655 |
| 2 | 0 | 0 | 8192 |
| 3 | 0 | 0 | 8192 |
| 4 | 0 | 0 | 8192 |
| 5 | 0 | 513 | 7679 |
| 6 | 0 | 0 | 8192 |
| 7 | 0 | 0 | 8192 |
| 8 | 0 | 0 | 8192 |
| 9 | 0 | 0 | 8192 |
| 10 | 0 | 0 | 8192 |
| 11 | 0 | 1615 | 6577 |
| 12 | 1879 | 0 | 6313 |
| 13 | 1290 | 626 | 6276 |
| 14 | 4490 | 118 | 3584 |
| 15 | 5872 | 971 | 1349 |

Final checkpoint: [map](../maps/round-01-disassembly.json),
[verification](round-01-disassembly-verification.json),
[current internal state](state.json), [source](../generated/round-01-disassembly/game.asm).
Map validation and byte-identical ca65 rebuild passed; ROM SHA256 unchanged.
All stage indices 0–7 visited, terminal 'Conan the King' captured, inventory menu reached;
stage 1–7/ending and inventory shortcut observations are artificial, not natural reachability.
[Scenarios](scenarios.json), [gameplay research](gameplay.md),
[phase handoff](handoffs/round-01-disassembly.md), [counts](round-01/counts.json).
No toolkit issues found; [persistent report](toolkit-issues.md), zero blockers.
Remaining unknown 112033 bytes require format/dispatcher investigation; no terminal
exhaustion claim. No cleanup/archive/publication performed.

## Round 1 commentary in progress
round=1 phase=commentary status=in-progress. Prepared frozen commentary baseline
from the verified disassembly map; preparation rebuilt byte-identically. Read the
entire compact 607-block / 6462-instruction classified code inventory and all 32
data-region evidence records. Curated functional explanations and entry names in
analysis/round-01/commentary-units.txt cover all currently classified source.
Next: apply source-only names/comments and coherent hardware/RAM symbols; inspect
and correct complex block-level explanations, verify, complete review inventory and
refresh reading-after. Exact remaining scope is all classified source in physical
0x18037..0x1FC28 (fragmented code; exact blocks in frozen review.json) and all 32
data blocks, plus honest notes for each unknown scaffold interval. No structural
map edits. Inherited stage-table names require correction to actual consumers.
Full readability is pending; no redirect handoff yet. Disk 3.5 GiB free.

## Round 1 commentary completed
round=1 phase=commentary status=current-source-readability-complete; next=redirect.
All current classified source reviewed: 607 code regions / 6462 decoded starts,
32 data regions, plus honest notes for all 123 unknown intervals. 223 functional
units explain routines/fragments, with 136 block-level notes and consistent actual
entry/branch, RAM/hardware, record/table symbols. 762/762 scaffold records reviewed;
652 physical-fragment unit attachments. No classified-source readability gaps remain.
Final editorial/archive acceptance remains pending, not invoked.

Corrected 14 inherited stage-table names to actual consumers, without map edits.
Resolved NMI prose: $CAB7 balances reentry gate and RTIs, $CABA callback dispatch,
$CABD default body. Nested NMIs skip callback work. Fully explained stack inline
arguments, 18-slot scheduler, object coordinates, CHR cache, mutable 5x5 maps,
row/column PPU buffering, animation records, player/inventory and music flow.
Unknown encounter identities and backing table endpoints are explicitly localized.

New redirect question: seven-byte stage-object records at physical
$1BB9B..$1BCC1, CPU $BB8B..$BCB1 in bank pair 12/13. Loop's cursor contract yields
42 iterations (last start $011F); endpoint coincides with loader entry and all
stage bytes are 0..7. Bounded parse saved at
round-01/commentary-spawn-record-observation.json; not structurally annotated.
Continue connected asset roots/handler domains plus this bounded record lead.

Fresh final commentary verify ok=true, byte_identical=true; report
round-01-commentary-verification.json includes exact source hashes. Reading copy
commentary/reading-after.asm refreshed after final edits. Maps/current.json equals
maps/round-01-disassembly.json; analysis/state.json equals frozen phase1-state;
map validation passed; input SHA256 unchanged. Classification unchanged/incomplete:
19055 classified (13531 code, 5524 data), 112033 unknown, 14.54%.
No toolkit issues found; toolkit-issues.md has zero findings/blockers for this phase.
No toolkit changes, publication, archive, deletion, cleanup or commit. Disk ~3.4 GiB free.
Scope/prose assessment: round-01/commentary-readability-assessment.md.
Handoff: handoffs/round-01-commentary.md. Exact remaining commentary scope: none
for current source; unresolved structural/game-meaning research transfers to redirect.

## Round 1 redirect active — r01-connected-20261127
Input round-01-commentary / maps/round-01-disassembly.json; no map/state edits.
Read redirect, evidence-policy and workflow; durable attempt analysis/redirect/r01-connected-20261127.
37 distinct initializers from bounded 42x7 stage records traversed with inline-word barriers;
handler-cfg.json holds 7,856 starts, 4,457 newly unknown starts awaiting independent
bank/stack/boundary audit. Mapper-call frontier is cfg-stops.json; lower-to-fixed
calls require audited return-bank6 contracts; fixed-to-lower edges deliberately not inferred.
Asset roots and exact rectangular layouts recovered. All eight metatile containers
independently tile layout -> 26-byte definitions -> pointer table -> next root;
all pointers aligned, every definition referenced, palette byte 0..2. Consumer reads
25 tiles plus palette offset25, so prior provisional 25-byte slices are superseded
by 26-byte contract in stage-container-audit.json. Eight counted CHR streams terminate
at output counts, saved token walks in stage-chr-streams.json. Decoder commentary has
repeat/literal classes reversed: $CDCD repeats same next byte; $CDEB increments source
per literal; correction must carry into next commentary. Productive continuation:
audit handler bank contracts; animation/image domains, graphic bank2 remaining roots,
raw CHR extent, palette and fixed dispatcher domains. Resume same redirect if interrupted.

## Round 1 redirect completed — combined-ready-batch
Handoff analysis/handoffs/round-01-redirect.md; attempt r01-connected-20261127.
All identified actionable connected and independent avenues investigated.37stage
initializers plus immediate producers; complete rooted asset containers; residual
coherent fixed API contracts. Proposed112032unknown bytes (13318code,98714data),
zero proposal/accepted kind conflicts, hypothetical residual1byteCAE1/physical1CAF1.
Exact unknown complement with contexts/neighbors/dispositions saved in attempt
unknown-ledger.json (accepted unknown112033, not hypothetical residual). No map,
state, source, ROM or toolkit changes. SHA/checkpoint identity verified; no new
execution coverage. LeadsQ01/Q02 ready,L04added; histories preserved. Decoder
repeat/literal correction and CE9F..CEA1 inlineRAM pointer05A7 are explicit gates
for next commentary/independent review. Next phase independent disassembly of
combined proposals; not terminal exhaustion. No toolkit issues found, no live
process. Disk remains~3.4GiB free; no cleanup/archive/publication/commit.

## Round 2 disassembly active
round=2 phase=disassembly status=in-progress. Restoring round1 accepted state;
independent review of combined redirect proposals. Durable frontier:
analysis/round-02/review.json and review.md (created during this pass).
No new execution/reachability claims; prior broad progression reused.
Inline physical1CEAF..1CEB1 is RAM pointer05A7; lone1CAF1 remains unknown.


# Round 2 disassembly completed; ROM incomplete

round=2 phase=disassembly status=pass-complete; next=commentary.
ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).
Rounded percentage does not establish completion. Code26849, data104238 including
16-byte header. Gain112032 (13318code,98714data). 906/906 classified regions have
evidence. 5964 proposed instruction records independently decoded;
all accepted structural proposals Static, no new execution/reachability claim.

| Physical 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 7646 | 546 | 0 |
| 13 | 6833 | 1359 | 0 |
| 14 | 5409 | 2782 | 1 |
| 15 | 6961 | 1231 | 0 |

Accepted R1-Q01/Q02/L04 as the combined connected structural batch after independent
consumer, exact-boundary, bank/stack and data-pattern checks. No proposal slices
rejected; unproposed lone1CAF1/CAE1 remains unknown. Existing accepted regions all
preserved exactly. [Review](round-02/review.md), [primitive audits](round-02/review.json),
[counts](round-02/counts.json), [lead history](../leads.json). Historical failed
methods remain preserved; no unchanged gameplay replays. Prior scenarios visited
stages0..7, inventory and ending with artificial/natural provenance kept separate.

Remaining1byte60 at physical1CAF1 CPUCAE1 is isolated RTS between NMI tailJMP and
IRQentryCAE2. No incoming edge/discriminating routine contract found. Keep unknown;
no dead/unused/code/data assumption. This local uncertainty does not reject coherent
neighboring engine APIs. No productive runtime/connected unknown target remains in
this pass; commentary then redirect should review this actual residual frontier.
No terminal exhaustion assertion by this disassembly pass.

Fresh map.validate ok=true; state saved analysis/state.json; export maps/current.json.
[Checkpoint](../maps/round-02-disassembly.json) byte-equal current export;
[verification](round-02-disassembly-verification.json) ok=true,byte_identical=true;
[source](../generated/round-02-disassembly/game.asm). ROM SHA256
`8124ff905ef374e1e1eed36994961bc04dc005bb0081a1ba620b2ec95ca7650b` unchanged.
Map SHA256 `171e9c2d3ffa53f33df490ade67191a3b7d8342fca8f67167e635f4ad3b7948e`. Generated source is local reconstruction,
not a sharing deliverable. Existing commentary/source, reading-after and historical
checkpoints unchanged; new source now requires full commentary review.

Next commentary: carry forward all223 functional units and symbols from previous
source into expanded source, review all newly classified ranges and current source.
Correct decoder token prose:01..3Frepeat,40..7Fzero,81..FFliteral,00/80skip.
Honor physical1CEAF..1CEB1 exclusive (CPUCE9F/CEA0) inline pointer05A7 after
JSRD360->D611; CEA1 resumes code. No structural reclassification needed for that word.
Detailed roles/reachability limits and exact contracts are in review and redirect
r01-connected-20261127 data-contracts.md/residual-contracts.md. Preserve generic-data
honesty where identities or original subtable layouts remain unresolved.

No toolkit issues found; [persistent report](../toolkit-issues.md), zero blockers.
No ROM/toolkit changes, cleanup, deletion, archive, publication or commit. No live
server remains. Disk~3.4GiB before checkpoint; bounded evidence retained locally.

## Round 2 commentary in progress

round=2 phase=commentary status=ongoing. Entire previous commentary preserved at
round-02/prior-commentary; fresh baseline has 907 scaffold entries. Carried 639
unchanged physical classified regions and all prior useful symbols/prose; fresh
carry verification byte_identical=true. All 5964 new instruction records inspected
with connected asset/API evidence. Current source now has 361 functional contracts
(223 carried, 138 new), descriptive routine/branch/field operands, corrected decoder
classes and updated formerly unknown neighboring-body statements. Source edit-batch
verification byte_identical=true. Enemy identities remain conservative mechanical
names. Review credit is carried only by physical range and relevant dependency.

Exact remaining readability scope: annotate/review all classified data across
physical $00000..$18030 plus the 27 consumer-table regions from $185BE through
$1FC05; ensure lone unknown $1CAF1..$1CAF2 and inline $1CEAF..$1CEB1 have explicit
correct notes; inspect full inventory for missed contracts/stale source and symbol
claims; refresh review.json source locations, uncertainties and reading copy; finish
with fresh byte-identical verification. No new structural changes, reachability
claims or final editorial audit. Continue commentary before redirect.

Commentary data batch now covers all 175 classified data regions. New notes explain
stage layouts and 26-byte metatiles, compressed CHR/screens, 246 animation roots,
383 sprite-image definitions, raw CHR cache groups, music/palette storage, object
records and descriptor-bounded DMC samples. Data/source batch verified byte-identical
(after resolving duplicate new layout-fragment label names; no bytes changed).
All 361 functional contracts have real source labels. Exact remaining scope is now
full inventory/source cross-check and changed-dependency prose consistency, review
source locations, uncertainties, fresh verify/read and compact commentary handoff.
No classified ranges remain awaiting first-pass explanation. Unknown physical
$1CAF1 stays explicitly unresolved. Token-$40 do-while wrap was checked statically;
none of 6554 saved stream token records uses it, so accepted bounds are unaffected.

## Round 2 commentary complete

round=2 phase=commentary status=current-source-readability-complete.
All731code regions/12426instruction starts,175data regions and unknown1CAF1 note
reviewed;907-entry scaffold preserved,361functional contracts,841code-fragment
attachments.639unchanged classified regions carried/reverified; entire previous
commentary preserved under round-02/prior-commentary. No pending classified-source
readability ranges. Changed evidence invalidated stale unknown-body/backing prose;
mechanical names preserve unresolved actor/item identities. Metatiles26bytes,
decoder classes,NMI distinction and CE9F inline word consistent. Source explains
all meaningful routines/data formats with uncertainty, not just review records.

Fresh final byte-identical verification and reading refresh pass; map validation,
input hash and frozen map/state equality pass. See round-02-commentary-verification.json,
round-02/commentary-readability-assessment.md and handoffs/round-02-commentary.md.
Remaining classification:1byte physical1CAF1/CPUCAE1; rounded100.00% is incomplete.
Final targeted editorial/audit gate remains pending for coordinator; redirect follows.
No toolkit/ROM/structural edits, cleanup,deletion,publish,archive or commit.

## Round 2 redirect completed; incomplete pending coordinator review

Current-source readability gate met; separate final editorial audit not prerequisite.
Attempt redirect/r02-cae1-review-20261127 combines full12426 accepted instruction
references (exact code-byte coverage), callback writers and NMI/scheduler tail-call
stack contracts. No direct CAE1 ref or E1CA physical word. Negative scope limited;
computed entries not excluded. Tail JMP does not require adjacent RTS: preserved
JSR CAAF returns CAB2 through downstream callback. Redundant RTS/no-op versus
spare data remain compatible, without independent discriminating structural role.
Retain sole unknown physical[1CAF1,1CAF2),CPUCAE1,bank14,byte60. Accepted131087,
code26849,data104238;proposed0;ready0. R2-Q01 records insufficiency and history;
all prior histories preserved. No justified new runtime experiment; prior stages0..7,
ending and inventory evidence reused. See handoffs/round-02-redirect.md and compact
rejection-review.md; coordinator must inspect primary artifacts before terminal acceptance.
Frozen map/state/checkpoint and source/ROM hashes rechecked. No structural/source/
toolkit/ROM edits or cleanup/deletion/publication/archive. No toolkit issues found.


## Coordinator terminal handoff — justified incomplete
round=2 phase=terminal status=justified-incomplete; coordinator workflow finished.
This goal turn made concrete progress: initialized verified baseline, completed two
sequential three-phase rounds, preserved prior commentary, accepted independent
structural evidence, completed current-source readability and final editorial audit.
No interrupted or blocked phase is treated as terminal success.

ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).
Code26849,data104238; physical[1CAF1,1CAF2),CPUCAE1,fixed8KiBbank14 remains unknown.
Rounded100.00% is not structural completion. Coordinator inspected rejection-review,
primary ROM/reference audit, NMI tail/scheduler and callback/stack contracts. A JMP
creates no local return obligation; redundant RTS and spare data both remain
compatible. No universal caller requirement imposed and no useful discriminating
method survived. No ready lead. Reopen only on supported new entry/producer/format
or corrected inference, not unchanged gameplay or fabricated execution.

Final targeted editorial acceptance reuses sound full-source review and checks
changed dependencies: metatile26bytes, decoder tokens/$40, stale unknown-body prose,
NMI and inline05A7. All907scaffold entries/1016attachments reviewed; fresh audit
byte-identical and reading refreshed. Final source hash supersedes historical
phase hashes only for current acceptance. Map/store validation passed; ROM/binary
identity unchanged. No toolkit issues found, zero blockers. No archive, cleanup,
deletion, publication or commit. All source/assets/internal evidence remain local.

Archive.py document contract inspected. archive-options.json explicitly accounts
for all30Markdown files:17additional final/history documents selected,3duplicate
old instruction scaffolds omitted; automatic/special paths accounted separately.
No final conclusions omitted. Full [document accounting](archive-document-selection.md).
Final [handoff](terminal-handoff.md),[objective audit](final-completion-audit.md),
[verification](final-verification.json),[editorial audit](../commentary/audit.json),
[last checkpoint](../maps/round-02-disassembly.json),[leads](leads.json),
[residual review](redirect/r02-cae1-review-20261127/rejection-review.md).

The final verification payload is also preserved in current-audit.json, an automatic
archive selection, with complete=false/classification_complete=false and the exact
unknown interval. Coordinator_workflow_complete=true applies only to orchestration.

## Authorized manual fallback disassembly phase

User explicitly authorized a batch-only exception to the structural evidence gate:
verify the complete residual complement is <=10 bytes, then record the listed
residual as generic Data, unknown meaning, Manual provenance. Authoritative saved
state/current export agreed; the exact complement was [117489,117490), length1
(physical $1CAF1, CPU $CAE1, fixed PRG8KiBbank14). Precheck and supported
map.annotate/save/export transcript are in [fallback](fallback/authorization-and-precheck.json).
One Manual Data annotation was added; all906 prior regions remain unchanged.
This is user-directed fallback, not new structural proof. Prior redundant RTS/no-op
callback versus spare-data ambiguity, reference/stack review and history remain valid.
No redirect research or gameplay reruns were performed.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
This annotation metric includes1 user-directed Manual fallback byte. Proven structural
classification remains131087/131088 (99.999237%);1byte remains structurally ambiguous.
Code26849;Data104239 (104238proven+1manual). The fallback does not satisfy the
original evidence-backed completion gate or prove execution/reachability.
Per-bank classified/unknown: banks0..15 each8192/0. Proven/manual counts: banks0..13
and15 each8192/0;bank14=8191proven+1manual. Header16proven Data. Full per-bank
ledger: [counts](fallback/counts-and-verification.json).

Map and store validations pass. New [checkpoint](../maps/manual-fallback-disassembly.json)
exactly equals current map and state; [verification](manual-fallback-disassembly-verification.json)
confirms byte-identical rebuild. Prior map/state copies retained in analysis/fallback;
all historical checkpoints/final audit artifacts preserved. Next separate commentary
worker should carry the current readable source forward, update only the residual
note and affected review/checkpoint metadata, preserve the Manual distinction and
perform fresh source verification. No toolkit issues found; [report](toolkit-issues.md).
ROM/toolkit unchanged. No deletion, archive, publish or commit.

## Batch exception final acceptance

Sequential disassembly and commentary workers completed the explicitly authorized residual fallback. ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). This includes 131087 evidence-backed bytes and one Manual generic Data byte at physical [117489,117490), CPU $CAE1, fixed PRG bank14. Original RTS-versus-spare-data ambiguity remains unresolved; unknown now describes meaning, not region kind. The user-directed exception provides no structural or reachability proof. Exact precheck, supported annotation transcript and previous state are retained under fallback/.

All 907 source inventory regions and 1016 units are reviewed, with no pending readability range. Entire previous commentary retained under round-03/prior-commentary. Final supported audit and independent strict editorial checks pass; toolkit has no strict CLI switch. Independent final verification freshly rebuilt source byte-identically and validated the map/store, exact prior-region preservation, review/source hashes and exhaustive archive document selection. Current audit now reflects this batch result; previous audit is preserved at fallback/prior-current-audit.json.

Final checkpoint maps/manual-fallback-disassembly.json, state/current map and commentary frozen mapping agree. See batch-terminal-handoff.md, batch-completion-audit.md and batch-final-verification.json. Original evidence-backed whole-ROM completion remains false for one byte; batch exception acceptance is complete. No other blockers, no toolkit issues found. Archive-options.json preserves new fallback conclusions and all additional final/history documents; no archive, deletion, publication, toolkit or ROM modification occurred.
