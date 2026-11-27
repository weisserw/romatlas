# Coordinator terminal handoff — incomplete ROM

ROM classification: 99.98% (131063 / 131088 bytes classified; 25 bytes unknown).

Current phase: terminal incomplete handoff after round005 redirect rejection review and final editorial acceptance. No phase is blocked or running. Whole-ROM classification remains incomplete. Current input/output checkpoint is `maps/round-005-disassembly-final.json`; map/store validation and byte-identical rebuild pass. Final source/reading and editorial verification: `analysis/handoffs/final-editorial.md`, `commentary/audit.json`, `commentary/verification.json`. Primary stopping review: `analysis/redirect/round-005-a/rejection-review.md`; coordinator inspection: `analysis/coordinator-rejection-review.md`. Final handoff: `analysis/final-handoff.md`. Document selection and objective audit: `archive-options.json`, `analysis/archive-document-selection.json`, `analysis/completion-audit.json`. No archive or cleanup authorized/performed.

## Reconciled sequential phase ledger

Each phase used a separate worker context, with exclusive writes and compact handoff. Final acceptance reused valid review; no empty classification cycle followed the sound terminal review. All links below are workspace-relative; historical handoffs retain their original counts and pending-gate states.

| Round / phase | Input checkpoint | Output / verification | Leads and outcome |
|---|---|---|---|
| 001 disassembly | Initialized empty state; raw baseline verified once | `maps/round-001-disassembly-final.json`; `analysis/round-001-disassembly-final-verification.json`; 23000 classified/108088 unknown | R1-BOOT, R1-ATTRACT, R1-STAGE-ENTRY, R1-DISPATCH, R1-ENDING, R1-FINAL-SHARK verified; R1-STATIC-INLINE/R1-RAM-PC rejected. Gameplay through credits recorded; source readability pending. |
| 001 commentary | round001 final map | `analysis/round-001-commentary-verification.json`; full then-current scope readable | No classification acceptance; C001–C006 produced. Later evidence corrected three mechanics statements; historical source preserved. |
| 001 redirect | round001 final map and commentary | `analysis/handoffs/round-001-redirect.md`; exact decoder/identity checks | Ready R1-PPU-STREAMS, R1-INDIRECT-STATES, R1-ROW-TABLES, R1-ACTOR-RECORDS, R1-COLLISION-TABLES, R1-MOTION-TABLE, R1-AUDIO-TABLES, R1-SPRITE-DATA, R1-STAGE-PALETTES, R1-TEXT-OAM-DATA, R1-DATA-BARRIERS. Combined 93193 proposed bytes. |
| 002 disassembly | round001 final map; eleven R1 leads | `maps/round-002-disassembly-final.json`; `analysis/round-002-disassembly-final-verification.json`; 116193 classified/14895 unknown | All eleven ready R1 leads verified; 93193 accepted Static bytes. Exact outcomes `analysis/round-002/lead-outcomes.json`. |
| 002 commentary | round002 final map | `analysis/round-002-commentary-verification.json`; current readability complete | Prior tree preserved; all three graphics/literal corrections and 29 affected dependency summaries repaired. C001–C006 conclusions updated. |
| 002 redirect | round002 final map and commentary | `analysis/handoffs/round-002-redirect.md`; primary contract/decode checks | Produced R2-CODE-CONTRACTS and R2-DATA-CONSUMERS, 13531 proposed bytes; E2FB conditional state experiment localized. |
| 003 disassembly | round002 final map; two R2 leads | `maps/round-003-disassembly-final.json`; `analysis/round-003-disassembly-final-verification.json`;129712 classified/1376 unknown | Accepted 13519 bytes; R2 leads verified with four uniform claims narrowed by12 JMP bytes. Exact outcomes `analysis/round-003/lead-outcomes.json`. |
| 003 commentary | round003 final map | `analysis/round-003-commentary-verification.json`; current readability complete | Prior tree preserved; EB8B26-response/C109eight-state distinction corrected; unknown FFEF suffixes retained. |
| 003 redirect | round003 final map and commentary | `analysis/handoffs/round-003-redirect.md`;562 instructions configured-validated | Produced R3-LOCAL-STRUCTURE,1346 proposed bytes. FFEF producer and local contracts reopened prior barriers; stale RTI identity corrected. |
| 004 disassembly | round003 final map; R3-LOCAL-STRUCTURE | `maps/round-004-disassembly-final.json`; `analysis/round-004-disassembly-final-verification.json`;131058 classified/30 unknown | R3 lead verified,1346 Static bytes accepted. B63A evidence join corrected. Exact outcomes `analysis/round-004/lead-outcomes.json`. |
| 004 commentary | round004 final map | `analysis/round-004-commentary-verification.json`; current readability complete | Prior tree preserved; all59 claims and13 dependencies reviewed; RTI811/831B and B63A join corrected. |
| 004 redirect | round004 final map and commentary | `analysis/handoffs/round-004-redirect.md`; exact primary consumer/decode checks | Produced R4-SHARED-TARGET-RESET,5 Static Code bytes using known field endpoint plus real conditional frame consumer. |
| 005 disassembly | round004 final map; R4-SHARED-TARGET-RESET | `maps/round-005-disassembly-final.json`; `analysis/round-005-disassembly-final-verification.json`;131063 classified/25 unknown | R4 lead verified,5 Static Code bytes accepted. Exact outcomes `analysis/round-005/lead-outcomes.json`. |
| 005 commentary | round005 final map | `analysis/round-005-commentary-verification.json`; current readability complete | Prior tree preserved; reset prefix and stale mapper-zero reset wording corrected. No new leads. |
| 005 redirect | round005 final map and commentary | `analysis/handoffs/round-005-redirect.md`; all ten slices reviewed against primary evidence | No ready leads or proposals; terminal incomplete recommended. Coordinator inspected decisive evidence and accepted localized ambiguity reasoning. |
| Final editorial | round005 final map and readable source | `commentary/audit.json`; fresh byte-identical rebuild and reading refresh | Duplicate prose, source-location prefixes and891 missing fragment-unit records repaired.1491 entries/1568 unit records reviewed;677 prior meaningful units preserved. No classification change. |

All classified regions carry evidence/provenance; Code has explicit CPU bank views. Exact per-bank totals/complement remain `analysis/round-005/classification-counts.json` and `analysis/round-005/unknown-ranges.json`. Leads preserve every prior pass/outcome in `analysis/leads.json`; none ready or blocked remains. Toolkit NES-001/NES-002 remain nonblocking and unchanged. Full historical progress as found before this reconciliation is preserved in `analysis/coordinator-before-final-progress.md`; earlier phase source/checkpoints/handoffs are retained.

## Previous progress record

# Final editorial acceptance complete

Supported final audit passed: 1491 reviewed scaffold regions, 1568 reviewed unit
records (677 prior meaningful units preserved plus 891 required code-fragment
records), fresh byte-identical rebuild and refreshed reading-after.asm. No pending
current-source readability/editorial work remains. Classification remains incomplete:
131063/131088 classified bytes (99.98%); 25 unknown bytes in ten intervals.

Resolved duplicate stride/E2FB prose, inherited source-location path prefixes and
missing per-Code-region unit records. Noncomment source and ROM/map/state/baseline
identities unchanged. Source SHA256 8a75a69947ed859e4302fca913deeea5451fc6d4b96aff5c5a62a502c1aad2f1.
Scope, actual audit rejections and fixes, limitations: commentary/editorial-audit.md;
mechanical result: commentary/audit.json; preservation/checks: analysis/final-editorial/.
Coordinator accepted terminal incomplete research under unchanged evidence in
analysis/coordinator-rejection-review.md. No archive/cleanup/publication performed.
All final-editorial writes cease at handoff; coordinator owns archive-options refresh
and final objective acceptance.

# Round004 redirect complete; independent disassembly next
Ready R4-SHARED-TARGET-RESET5StaticCode; analysis/handoffs/round-004-redirect.md. Accepted131058/30unknown remains unchanged; ifaccepted131063/25unknown. Primary combined reset/consumer/frame contract; no terminal claim. Protected identities unchanged, final editorial pending; research mutations ceased.

# Active round004 redirect
Checkpoint maps/round-004-disassembly-final.json;131058 classified/30unknown; readability current complete, final editorial pending. Worker analysis/redirect/round-004-a/progress.md. Frontier whole-consumer/prefix and G3$9EA2 pointer-context discrimination. No map/source/state/ROM/toolkit edits.

# Round004 commentary complete

Full current-source readability complete; mutations ceased for sequential redirect. ROM remains incomplete:131058/131088 classified(99.98%),48110Code/82948other,30unknown/11slices. Map/state/classification unchanged from maps/round-004-disassembly-final.json.

Reviewed all1307 Code regions/21836 instructions,173 non-code regions and11 unknown boundaries;1491 scaffold entries,676 meaningful units(593 carried+83 added). All1249 prior Code ranges retain physical identities and sound prose; all59 accepted claims/1310Code+36Data and13 affected prior contracts incorporated. Entire prior commentary hash-preserved under analysis/round-004/commentary-prior; original live tree retained separately. No current readability backlog. Final editorial audit pending.

Source commentary/source/game.asm; review commentary/review.json; refreshed commentary/reading-after.asm. Exact assessment analysis/round-004/commentary-scope.json and commentary-dependency-review.md. Fresh analysis/round-004-commentary-verification.json passes byte-identical; sourceSHA2560c6fce44625015d8a08466e71d718ea4a6c2228085b8abd4c342c6c19dd518e3. Prior preservation/map/state/ROM identities checked.

FFEF producer/return/dispatch and fallback mirrors, display/audio initializer, two PPU column uploads, immediate-selector ADD8 tail contract, patrol/recovery/cursor/attract bodies and heading/spawn lists are readable. Conditional bank/register/stack/selector domains and actual-selection uncertainty remain explicit; queue proofs do not establish global91C1/collision51 or whole-frame guarantees. Heading convergence's asymmetric unchanged deltas0..3/59..63 are explicit. B63A fourINY→B63E BEQB643 orB640 JMPB5B9, notB5B5.

Corrected current RTI premise: physical5597 is Data5575..5607; actual unknownphysical811/CPU831B pair40/70. Historical records preserved. E2FB contradiction and code-primary graphics semantics unchanged. Actor/glyph identities, grid domains/original allocations and actual entry/bank preconditions retain honest uncertainty in commentary/uncertainties.md.

Redirect exact residuals: analysis/round-004/unknown-ranges.json; competing interpretations analysis/redirect/round-003-a/candidate-review.md. Physical277:1;811:2;2801:3;3140:3;9813:5;11951:3;14251:2;25939:3;30710:1;106162:5;113405:2. Inspect strongest residual reasoning under evidence-policy.md before terminal acceptance; no terminal exhaustion claimed. Reuse later-game/boss/ending/credits and E2FB observations; avoid unchanged decode/artificial-entry replay.

Toolkit report unchanged:1confirmedNES-001,1missing-featureNES-002,0blockers/no new issue; analysis/toolkit-issues.md. No map/classification/lead/toolkit/ROM edit, archive/deletion/cleanup/publication. Source/assets/state/traces remain local. All writes cease after final phase/progress identities.

## Round004 disassembly ledger retained

All1346 Static bytes accepted in59 claims;1310Code/36Data. maps/round-004-disassembly-final.json and analysis/round-004-disassembly-final-verification.json passed byte-identical. Full disassembly handoff/review remain analysis/handoffs/round-004-disassembly.md and analysis/round-004/review.md; no observed/natural reachability change.

# Round004 commentary active

Entire prior commentary preserved and SHA256 verified under analysis/round-004/commentary-prior. Fresh baseline required: classification changed by1310Code/36Data. Full-source physical carryover and affected contracts pending; no readability completion claimed. Frontier: analysis/round-004/commentary-phase.json. Final editorial audit reserved.

# Round003 redirect complete; independent disassembly next

Ready R3-LOCAL-STRUCTURE proposes1346Staticbytes(1310Code/36Data),59claims/562instructions. Accepted checkpoint remains129712/131088classified;1376unknown/67intervals. Ifaccepted30unknown/11slices. Handoff analysis/handoffs/round-003-redirect.md; exact roles/primary evidence/contracts in analysis/redirect/round-003-a/. Configured decode/complement/boundary/hash checks pass. Queue-tail method closes11 normal-return boundaries; no collision51 global guarantee. Correct staleRTI5597 in next commentary: actual currentRTI physical811/CPU831B. Full readability complete; final editorial pending. No terminalexhaustion, classification/toolkit/source/ROM changes. Redirect mutations ceased.

# Round003 redirect active
Input maps/round-003-disassembly-final.json;129712 classified/1376 unknown in67 intervals. Attempt analysis/redirect/round-003-a/progress.md. Frontier: FFEF continuation producer and local target contracts; residual independent routine islands. No classification edits; final editorial audit pending.

# Round003 commentary complete

Full current-source readability complete; ROM classification incomplete. Commentary mutations ceased for sequential redirect. Input maps/round-003-disassembly-final.json remains129712/131088 classified(98.95%):46800Code,82912other;1376unknown/67intervals.

All1249 Code regions/21274 instructions,172 non-code regions and67 unknown boundaries reviewed:1488 scaffold entries,593 meaningful units (423 carried,170 added). Entire previous commentary hash-preserved before fresh prep under analysis/round-003/commentary-prior and original live tree retained separately.1198 old Code ranges retain physical instruction identities and sound prose;51 new Code ranges/3144 bytes and106 Data ranges/10375 bytes receive mechanical explanations. Dependency review preserves prior corrections; no current-source readability backlog remains. Final editorial audit pending.

Readable source commentary/source/game.asm; refreshed commentary/reading-after.asm; review commentary/review.json; exact scope analysis/round-003/commentary-scope.json. Fresh analysis/round-003-commentary-verification.json confirms byte-identical rebuild; source SHA25611a46d831126d22f7d85accd5b65fbe7deba91888cbea682da4ca4700c385d42. Frozen map/state/ROM and prior preservation hashes checked.

EB8B now names26 response pointers atC121..C155; independentC109 remains8state entries. New explanations include steering/stack contracts, crossing/spawn descriptors, rectangular tile alternatives, direction quantizers, maximum-index random headers, auxiliary count offsets, high-bit text runs, score insertion/name-entry cursor and OAM, and biased collision pointer/quadrant handlers. FourFFEF three-byte JMP suffixes remainUnknown; preceding56-byte FF runs remainData. Generic blocks explain inspected boundaries and unresolved meanings without invented formats.

Redirect: commentary/uncertainties.md carries sound C001–C006 results and supersedes old counts/premises. Reuse round002 discriminating E2FB tests: exit-producedBE1 bypassesE59B throughE317, inconsistent forcedBE0 reaches contradictory Data jump inG0. Neither global infeasibility nor hidden bank switch proved. StageId was patched; observedStageSystem0 was not patched. Exact residuals analysis/round-003/unknown-ranges.json plus analysis/redirect/round-002-a/residual-obstacles.md; FFEF needs independent entry/role evidence. Preserve local register/stack/bank barriers, isolated5597RTI and code-primary graphics overlaps. Actor/glyph identities, grid domains and original nested allocation remain uncertain but mechanics are explained.

Toolkit unchanged: existingNES-001 confirmed documentation/API issue andNES-002 documented precision missing feature,0 blockers; analysis/toolkit-issues.md. No new issue, classification/lead edits, toolkit/ROM edits, archive/deletion/cleanup/publication. Source/assets/state/traces remain local. This phase does not establish whole-ROM classification or final editorial acceptance.


# Round003 commentary coherent integration verified

1198 prior Code ranges carried by physical instruction identity;51 expanded ranges/3144 bytes have explanations and functional control symbols. Specific new data formats and EB8B26-response correction applied; independentC109 eight-state table retained. Fresh byte-identical verify passed. Frontier: final dependency/scaffold consistency, review/source locations, uncertainty carry, exact scope, fresh final verify/read and compact handoff. Commentary remains active; classification unchanged.

# Round003 commentary active

Entire prior commentary hash-preserved under analysis/round-003/commentary-prior. Input maps/round-003-disassembly-final.json:129712 classified;1376 unknown. Frontier: fresh baseline, physical-identity carry, full3144 Code/10375 Data review and prior-source dependency repair (EB8B response26 entries; C109 remains8). Final editorial audit deferred.

# Round003 disassembly complete; commentary next

ROM classification: 98.95% (129712 / 131088 bytes classified; 1376 bytes unknown).

Accepted13519 new Static bytes:3144 Code/10375 Data in157 groups from R2-CODE-CONTRACTS and R2-DATA-CONSUMERS. Canonical3144 Code batch accepted fully; four59-byte uniform Data claims shortened to56-byte prefixes. Rejected12 suffix bytes at physical32767,65535,98303,131071 (three bytes each, CPUFFEF), which contain JMPs rather than uniformFF. They remainUnknown pending independent entry/role evidence. No new observed execution or natural reachability claim. One substantive pass completed; ROM incomplete, not terminal research exhaustion.

Fresh checkpoint maps/round-003-disassembly-final.json; internal analysis/state.json; portable maps/current.json; generated/round-003-disassembly-final. Fresh analysis/round-003-disassembly-final-verification.json confirms byte-identical rebuild. Classified Code46800; other82912 including16-byte header. Exact counts/bank totals and67 unknown intervals: analysis/round-003/classification-counts.json and unknown-ranges.json. Review/evidence: review.md, independent-review.json, review.py, accepted-*-proposals.json, localized-rejections.json, annotation-rpc.jsonl, map-validation.json; lead-outcomes.json and preserved analysis/leads.json histories.

| PRG 8 KiB bank | Code | Data/text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 6143 | 2016 | 8159 | 33 |
| 1 | 6805 | 927 | 7732 | 460 |
| 2 | 7046 | 1049 | 8095 | 97 |
| 3 | 6288 | 1302 | 7590 | 602 |
| 4 | 1434 | 6758 | 8192 | 0 |
| 5 | 3449 | 4723 | 8172 | 20 |
| 6 | 1797 | 6391 | 8188 | 4 |
| 7 | 369 | 7820 | 8189 | 3 |
| 8 | 967 | 7225 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 34 | 8158 | 8192 | 0 |
| 11 | 325 | 7864 | 8189 | 3 |
| 12 | 3722 | 4448 | 8170 | 22 |
| 13 | 6556 | 1570 | 8126 | 66 |
| 14 | 1399 | 6730 | 8129 | 63 |
| 15 | 466 | 7723 | 8189 | 3 |


Broad gameplay through later areas/boss/ending/credits and saved discriminating probes reused; no redundant run. PreservedAD05 operand/AD06 data, constant-takenB500/B502 data, BE16 five-byte exclusion, E2FB Data and unresolved global state contradiction, code-primary graphics reads, isolatedRTI and local mapper/stack barriers. Conditional static routine preconditions are explicit; unresolved actual selection/return facts remain separate.

Next commentary must cover all currently classified source while preserving entire prior commentary. Correct EB8B consumer to26-entry response tableC121..C155; independent C109 remains eight-state table. Current commentary/source/game.asm untouched; final editorial gate pending. Then redirect examines exact residuals plus newFFEF rejection; strongest prior islands need independent entry/register/stack/bank evidence per analysis/redirect/round-002-a/residual-obstacles.md. No terminal exhaustion claimed.

Toolkit: existing confirmed NES-001 documentation/API issue and NES-002 precision missing feature, zero blockers, no new issues; [toolkit-issues.md](analysis/toolkit-issues.md) preserved. ROM, toolkit and commentary source unchanged; no archive/deletion/cleanup/publication.

# Active: round003 disassembly review

Checkpoint maps/round-002-disassembly-final.json and analysis/state.json:116193/131088 classified(88.64%);14895 unknown/199 intervals. Independently reviewing R2-CODE-CONTRACTS(3144 bytes) and R2-DATA-CONSUMERS(10387 bytes); proposals not yet accepted. Exact baseline per-bank counts/complement and current phase ledger in analysis/round-003/{baseline-counts.json,baseline-unknown-ranges.json,phase-status.json}. Conditional roots, protected-$00 services and exact consumer boundaries under review. Prior commentary and historical evidence retained.

# Round002 redirect complete; independent disassembly next

Ready batch13531 Static bytes(3144 Code/10387 Data), two leads R2-CODE-CONTRACTS/R2-DATA-CONSUMERS. Current accepted116193/131088(88.64%);14895unknown/199intervals unchanged. Ifaccepted1364unknown/63intervals. Fullhandoff analysis/handoffs/round-002-redirect.md. Validation andcompact candidate review complete; no terminalexhaustion. Finaleditorialaudit pending; map/state/source/ROM/binary preserved. Redirect mutationsceased.

# Active: round002 redirect

Checkpoint maps/round-002-disassembly-final.json; 116193/131088 classified;14895 unknown in199 intervals. Current-source readability complete; final editorial audit pending. Attempt analysis/redirect/round-002-a/progress.md. Investigating local return/bank contracts, E2FB contradiction and residual consumer/pattern boundaries. No new ready lead yet; prior R1 leads verified. Map/state/source/ROM unchanged.

# Current phase — round 002 commentary complete

Full current-source readability complete at `maps/round-002-disassembly-final.json`; redirect next. ROM classification unchanged116193/131088 (88.64%),14895 unknown/199 intervals. All1198 code regions,66 non-code regions and199 unknown boundaries reviewed;1463 scaffold entries/423 meaningful units. Fresh byte-identical `analysis/round-002-commentary-verification.json`; refreshed `commentary/reading-after.asm`; compact handoff `analysis/handoffs/round-002-commentary.md`; exact scope `analysis/round-002/commentary-scope.json`. Entire prior commentary preserved and hash-checked under round-002. Twenty-nine old dependency summaries corrected (supersedes active estimate34). No readability backlog; final editorial audit pending. No map/state/ROM/toolkit edits. Commentary mutations ceased; prior progress/history follows unchanged.

# Round 002 disassembly complete; commentary next

ROM classification: 88.64% (116193 / 131088 bytes classified; 14895 bytes unknown).

ROM remains incomplete; this is one completed substantive pass, not terminal research exhaustion. Accepted93193 new Static bytes (23072 Code/70121 Data) in482 groups from11 ready leads. Full independently supported connected batch accepted; no canonical proposal rejected. Preserved exclusions: five controller masks remain Data, contradictory stage table25355..25365 remains Data despite conditional JMP, isolated RTI5597 remains Unknown, graphics reads overlapping instructions keep Code primary. No new runtime/natural reachability claim. Reused existing broad artificial later-game/ending/credits routes and saved packed-consumer probes; no redundant replay.

Checkpoint `maps/round-002-disassembly-final.json`; internal `analysis/state.json`; portable `maps/current.json`; fresh verification `analysis/round-002-disassembly-final-verification.json` confirms byte-identical assembly. Code43656; other classified72537 (including16-byte header). Exact counts/residuals: `analysis/round-002/classification-counts.json`, `unknown-ranges.json` (199 intervals). All newly accepted annotations carry Static provenance and evidence. Review: `analysis/round-002/review.md`, independent checks `independent-review.json`, reproducible `review.py`; annotation transport `annotation-rpc.jsonl`. Lead outcomes: `analysis/round-002/lead-outcomes.json` and preserved history in `analysis/leads.json`.

| PRG 8 KiB bank | Code | Data/text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 5558 | 9 | 5567 | 2625 |
| 1 | 6728 | 127 | 6855 | 1337 |
| 2 | 6469 | 282 | 6751 | 1441 |
| 3 | 6128 | 384 | 6512 | 1680 |
| 4 | 1434 | 6742 | 8176 | 16 |
| 5 | 3412 | 4190 | 7602 | 590 |
| 6 | 1797 | 6307 | 8104 | 88 |
| 7 | 354 | 7739 | 8093 | 99 |
| 8 | 967 | 7035 | 8002 | 190 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 34 | 8158 | 8192 | 0 |
| 11 | 316 | 7759 | 8075 | 117 |
| 12 | 3697 | 3694 | 7391 | 801 |
| 13 | 5106 | 560 | 5666 | 2526 |
| 14 | 1205 | 4458 | 5663 | 2529 |
| 15 | 451 | 6885 | 7336 | 856 |


Next: current-source commentary for all newly classified code/data, preserving existing prose/history. Correct three known prior-source statements: reverse-bit mode consumes16 source bytes/tile; packed low-counter0 adds256 output bytes; stage initialization consumes COUNT literal bytes rather than repeated constants. Then redirect reviews residual source gaps/199 exact physical intervals and63 saved missing callee/indirect bank contracts. No universal post-bankcall propagation or pointer-only code assumption. No currently ready lead remains unattempted. Full-source readability and final editorial audit remain separate gates.

Toolkit:1 confirmed documentation/API-validation issue NES-001,1 documented precision missing feature NES-002,0 blockers; [toolkit-issues.md](analysis/toolkit-issues.md) preserved. Toolkit and ROM unchanged; no archive/deletion/publication. Mutations ceased after final records/checkpoint. Handoff: `analysis/handoffs/round-002-disassembly.md`.

# Round 002 disassembly active

Input checkpoint `maps/round-001-disassembly-final.json`, state `analysis/state.json`: 23000 classified,108088 unknown. Independent review of redirect combined batch (93193 proposed bytes), supported roots/control/stack/bank contracts and consumer/boundary evidence underway. No new classifications yet. Current frontier: validate all direct aliases and handler domains; inspect safe-return stack effects, data bounds/code-primary conflicts; localize unsupported slices. Existing broad gameplay/ending evidence reused. Commentary corrections preserved in redirect handoff.

# Round 001 redirect complete; independent disassembly next

Accepted checkpoint `maps/round-001-disassembly-final.json` and state unchanged:23000 classified/131088,108088 unknown. Combined proposal93193 bytes:23072 static Code(437 groups),70121 static Data(45 segments); independent acceptance pending. If all accepted,14895 unknown would remain. Eleven ready lead IDs and history: `analysis/leads.json`. Handoff: `analysis/handoffs/round-001-redirect.md`; evidence/recipes: `analysis/redirect/round-001-a/findings.md`; exact current unknown complement/context/neighbors/dispositions: attempt `unknown-ledger.json`. Consumed R1-PPU-STREAMS/R1-INDIRECT-STATES and C001–C006. Whole connected graphics/row/state/collision/motion graphs and additional audio/sprite/palette/text consumers packaged. 10423 instructions independently match configured toolkit decode; per-instruction root/parent evidence retained. Final accepted map remains byte-identical to input checkpoint; ROM/source identities unchanged.

Three discovered commentary corrections pending next current-source phase: byte reversal consumes16 bytes/tile; low-counter0 graphics request adds256 output bytes; stage initialization runs consume literal bytes. Known data barriers/isolated RTI exclusions and63 missing local return/mapper contracts preserved. No terminal exhaustion claim; final editorial audit still pending. Toolkit:1 confirmed issue,1 documented precision missing feature,0 blockers; no toolkit edits.

# Round 001 redirect active

Input checkpoint `maps/round-001-disassembly-final.json`; accepted 23000/131088 bytes, 108088 unknown. Current-source readability gate met. Attempt `analysis/redirect/round-001-a/`. Consuming R1-PPU-STREAMS / R1-INDIRECT-STATES and C001–C006. Frontier updated: C001–C006 inspected as connected batches; graphics/list/source extents, row lookup/template data, actor/state/collision handlers, motion components and local bank-safe CFG now have reusable evidence. Corrected CFG proposes22274 code bytes; data proposal assembly and exact unknown-only union pending. First graph false-fallthrough into credits and ten-byte stage table excluded. Known C001 bit-reversal wording correction pending next commentary; decoder low-counter-zero edge documented. Toolkit NES-002 missing precision feature recorded. No map/state/source mutations.

# Round 001 commentary complete; redirect next

Phase: commentary complete for all currently classified source. Input checkpoint: `maps/round-001-disassembly-final.json`; disassembly verification: `analysis/round-001-disassembly-final-verification.json`. Commentary verification: `analysis/round-001-commentary-verification.json` (fresh byte-identical assembly); source `commentary/source/game.asm`; refreshed reading `commentary/reading-after.asm`; scope `analysis/commentary-round-001/scope.json`; handoff `analysis/handoffs/round-001-commentary.md`. Map/state unchanged. 761 code regions and 21 non-code regions readable; 429 unknown gaps explained; 1211 reviewed scaffold entries and 173 meaningful units. No current-source readability backlog. Final editorial audit remains pending. Classification leads consumed/accepted: none in commentary. Produced C001–C006 research questions (counted/packed graphics, stack row descriptors, state/actor tables, movement quantization and return bank contracts), outcomes documented in commentary/uncertainties.md. No structural annotations accepted in this phase.

# Disassembly progress

Round 001 disassembly pass completed; **ROM classification remains incomplete**. Continue the prescribed commentary/redirect cycle; this is not a terminal exhaustion assessment.

ROM classification: 17.55% (23000 / 131088 bytes classified; 108088 bytes unknown).

Current state: [state.json](state.json). Portable ROM-free map: [current.json](../maps/current.json). Final checkpoint: [round-001-disassembly-final.json](../maps/round-001-disassembly-final.json); [verification](round-001-disassembly-final-verification.json), generated source `generated/round-001-disassembly-final`. Map validation and byte-identical assembly rebuild passed. ROM SHA-256 remains `daac9d85628410002742b72690f7c30e8e498a4a9be8646f0d3458716301c45c`. Earlier baseline and intermediate a/b/checkpoint records remain intact.

Classified code: 20584 bytes; classified data/text/header/vectors: 2416 bytes. Header: 16 classified Data bytes outside PRG banks. All 782 regions carry structural evidence; all 761 code regions carry explicit CPU execution views. Portable-map labels: 3 at the disassembly checkpoint; complete current-source commentary now supplies 1293 validated source control labels and 222 shared RAM/hardware symbols (see commentary scope above). Raw emission of the remaining bytes does not classify them.

| PRG 8 KiB bank | Code | Data/text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3001 | 9 | 3010 | 5182 |
| 1 | 1651 | 0 | 1651 | 6541 |
| 2 | 3371 | 48 | 3419 | 4773 |
| 3 | 1898 | 134 | 2032 | 6160 |
| 4 | 1381 | 0 | 1381 | 6811 |
| 5 | 1637 | 0 | 1637 | 6555 |
| 6 | 1079 | 0 | 1079 | 7113 |
| 7 | 321 | 730 | 1051 | 7141 |
| 8 | 886 | 0 | 886 | 7306 |
| 9 | 0 | 0 | 0 | 8192 |
| 10 | 34 | 1025 | 1059 | 7133 |
| 11 | 298 | 134 | 432 | 7760 |
| 12 | 2578 | 0 | 2578 | 5614 |
| 13 | 2011 | 5 | 2016 | 6176 |
| 14 | 50 | 0 | 50 | 8142 |
| 15 | 388 | 315 | 703 | 7489 |

Counts and exact unknown complement: [classification-counts.json](round-001/classification-counts.json), [unknown-ranges.json](round-001/unknown-ranges.json). Retained evidence contains 8366 distinct ROM opcode starts and 8964 contextual roots. This is a union/lower-bound metric from saved actual-fetch artifacts, separate from classified code and not complete execution coverage: the initial skip capture capped at5,000 roots; a process-local trace was lost before snapshot recovery; long ending/boss waits were untraced with short traced milestone samples. Retained scenario summaries report aggregate_dropped=0; detail capacity0 intentionally dropped detail. No RAM execution is inferred from a temporary mid-instruction next-PC.

Substantive results: reset/NMI/IRQ roots and normal title/attract startup; corrected silent input-schema misuse; supported attract-timer shortcut; corrected pending-NMI interruption of direct stage entry; diverse rendered race/pod/mine/hazard/people/target/waterfall states, final shark encounter with fire/steering input, and automatic ending rescue through staff credits and thanks-for-playing. Later-state visits are explicitly artificial, not natural progression. Exact patches, input arrays/releases, budgets, snapshots and outcomes: [scenarios.json](scenarios.json), [gameplay.md](gameplay.md), `round-001/rpc.jsonl`. Final-boss defeat and post-credits high-score return remain unverified.

Static batches: direct control-flow from fetched roots, validated vector contracts, 56 consecutive bank-dispatch JMP stubs and supported bodies (6/6/11/33 stubs), five-byte inline controller-mask barrier, 48-byte boot defaults, ASCII developer signature and inspected uniform non-code runs. Generic data annotations claim non-code structure only where role semantics remain unknown. Rejected false ISC fallthrough at $1A55E; no pointer alone or unobserved path was called executed. See [investigation.md](investigation.md) and [handoff](handoffs/round-001-disassembly.md).

Remaining research: indirect/banked call contracts and post-call contexts; packed PPU/CHR-RAM consumers and exact data boundaries; stage and actor records; largest unknown banks9 and14. This bounded pass does not establish their formats or deadness. Broad representative mechanics/ending routes are reusable; further repetitive title play is unnecessary. Ready static supported batches were incorporated; deeper contracts/data-format investigation belongs in redirect rather than guessing from decodable bytes.

Toolkit: **1 confirmed documentation/API-validation issue, 0 suspected bugs, 0 missing-feature findings, 0 blockers**. [toolkit-issues.md](toolkit-issues.md), NES-001: NES emulator.input requires `buttons` array; shared button/pressed example silently clears inputs. Workaround validated. Toolkit unchanged. No archive, deletion or publication performed; inputs, snapshots, traces, source and assets remain local.

## Round 002 commentary active
Checkpoint maps/round-002-disassembly-final.json (116193 classified;14895 unknown). Entire previous commentary preserved twice under round-002/commentary-prior and commentary-prior-live; hash manifest retained. Fresh preparation next; all expanded code/data and carried dependencies require readability review. Maps/state/ROM remain untouched.

Round002 commentary baseline prepared byte-identical (1463 entries:1198 Code,66 Data/Text,199 Unknown). Full expanded-code inspection staged by group; physical carry/integration script in round-002/commentary-build.py. Remaining source integration/data comments/three corrections/dependency review/verification; commentary active, no classification edits.

Round002 commentary coherent integration verified byte-identical after full437-entry expanded-code review: new per-fragment explanations and internal routine contracts supplement carried761 code ranges. Three mandatory corrections and thirty-four stale dependency summaries repaired. Symbols use explicit widths and physical/group-qualified identities; source introduces no annotations. Remaining: final full-scope consistency inventory, uncertainty/handoff records and fresh final verify/read.

Round002 commentary quality review now covers all1198 code regions,66 non-code regions and199 unknown boundaries. Expanded split blocks have tailored mechanics; large graphs have callable/internal contract explanations. Source operand widths verified; actor-slot/action-field expressions and context-local collision bounds preserve RAM storage identity. Current uncertainty record supersedes C001-C006 premises while retaining historical record; remaining frontier is199 gaps/63 local callee contracts plus contradictory Data jump atE2FB. Final scope/units and fresh final verify/read next.

## Round004 disassembly active
Input maps/round-003-disassembly-final.json;129712 classified/1376 unknown. Independently reviewing R3-LOCAL-STRUCTURE59 claims/1346 bytes against exact physical bounds, local contracts and consumers. Conditional structure and actual selection remain separate. Review phase; no new execution; saved broad progression reused. Toolkit/ROM/commentary unchanged.

## Round004 disassembly complete; commentary next

ROM classification: 99.98% (131058 / 131088 bytes classified; 30 bytes unknown).

Accepted all59 R3-LOCAL-STRUCTURE claims:1346 Static bytes=1310 Code+36 Data;562 instructions independently checked and freshly decoded by configured release binary. Local B63A prose corrected: B63E wrap exit or B640 JMP B5B9 comparator repeat, not B5B5. No new execution/natural reachability assertion. Exact independent semantic/context/physical review: analysis/round-004/review.md and independent-review.json; accepted-proposals.json and annotation-rpc.jsonl. No slices rejected. Structural classification incomplete; not terminal exhaustion.

Fresh map.validate passed; state analysis/state.json and maps/current.json saved; maps/round-004-disassembly-final.json generated/round-004-disassembly-final freshly rebuilt byte-identical (analysis/round-004-disassembly-final-verification.json). Code48110; other82948 including16-byte header; exact30 unknown bytes/11 slices in analysis/round-004/unknown-ranges.json. Every accepted Code has explicit physical effective-bank CPU view and Static evidence. Annotation coverage complete for accepted regions; execution evidence unchanged from saved prior actual-fetch lower bound, separate from structure.

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
|0|6167|2016|8183|9|
|1|7255|927|8182|10|
|2|7143|1049|8192|0|
|3|6850|1338|8188|4|
|4|1434|6758|8192|0|
|5|3469|4723|8192|0|
|6|1801|6391|8192|0|
|7|372|7820|8192|0|
|8|967|7225|8192|0|
|9|0|8192|8192|0|
|10|34|8158|8192|0|
|11|328|7864|8192|0|
|12|3739|4448|8187|5|
|13|6620|1570|8190|2|
|14|1462|6730|8192|0|
|15|469|7723|8192|0|

FFEF saved-continuation producer/return, parameterized queue tail contract, display/PPU upload, actor heading/spawn, exact87/36F3B8 split, collision counterparts, cursor restores and tiny clear-scroll/full-record-skip/speed-stop variants accepted on independent consumer/control constraints. Conditional collision91C1/shared00/local bank preconditions remain separate from actual caller guarantees. No unsupported reachability or deadness inferred.

Broad gameplay later areas/boss/ending/credits and discriminating E2FB probes reused; no redundant replay. Preserved E2FB Data contradiction and other historical barriers. Next commentary must cover all current source, including new code/data and stale RTI correction: physical5597 is acceptedData5575..5607; actual unknown pair physical811/CPU831B=40/70. Commentary source untouched, readability scope needs refresh; final editorial audit pending. Then redirect uses exact30-byte residual and reviewed competing interpretations, without another unchanged decode/state-fabrication loop.

Toolkit: existing1 confirmed issue NES-001,1 missing feature NES-002,0 suspected/0 blockers; no new toolkit issues. [toolkit-issues.md](toolkit-issues.md) preserved. ROM SHA256 unchanged; configured toolkit unchanged. No archive/deletion/cleanup/publication.

## Recovered round004 disassembly phase ledger

# Round004 disassembly complete; full current-source commentary next

ROM classification: 99.98% (131058 / 131088 bytes classified; 30 bytes unknown).

Accepted1346 Static bytes=1310Code+36Data, all59 R3-LOCAL-STRUCTURE claims/562instructions. Independent exact physical/bank/boundary/control checks and consumer/semantic review: analysis/round-004/review.md,review.py,independent-review.json,accepted-proposals.json. Fresh configured binary decode each claim, map.validate/save/export and byte-identical checkpoint succeeded: maps/round-004-disassembly-final.json; analysis/round-004-disassembly-final-verification.json; generated/round-004-disassembly-final; internal analysis/state.json,portable maps/current.json. Code48110,other82948. Annotation evidence and CPU views complete; no new Observed/natural reachability claim; gameplay/actual-fetch coverage unchanged.

FFEF producer/return/dispatch and fallback mirrors, A66C/A6E8 display/PPU routines, parameterized ADD8 immediate tail-JMP contract and eleven caller boundaries, F375→EFF8→F39B→F3B8 graph and87Code/36Data splitF40F accepted. Stage initialization independently yields system0 IDs1/10/16,selectors0/9/14 and terminated lists endingF429/F42E/F433. Collision axis counterparts, patrol/heading/state bodies, saved renderer cursors and attract state3 accepted. Tiny CFEF/B63A/B8D4 variants have explicit whole-consumer/stride contracts. Localized prose correction: B63A fourINY feeds B63E BEQB643 or B640 JMPB5B9, not B5B5. Old redirect records preserved; accepted proposal corrected. No claim rejected.

Queue bank summaries do not establish global collision51/91C1 shared00 theorem. Conditional localG0/register/stack/selector preconditions remain explicit and actual caller/selection unresolved. D590/C060 are body endpoints, not unproven whole-frame bank summaries. Protected E2FB Data contradiction, AD05 operand/AD06 Data, B502 Data, G1BE1B Data and code-primary reads remain intact.

Exact residual30bytes/11slices: analysis/round-004/unknown-ranges.json,classification-counts.json (per-bank too). Physical277:1;811:2;2801:3;3140:3;9813:5;11951:3;14251:2;25939:3;30710:1;106162:5;113405:2. Competing role interpretations/reference checks: analysis/redirect/round-003-a/candidate-review.md. This is one substantive pass, ROM incomplete, not terminal research exhaustion. Commentary then redirect should inspect strongest residual reasoning before any terminal decision; avoid unchanged decoder/artificial-entry replay.

Next commentary MUST correct stale current “isolatedRTI5597”:5597 is acceptedData5575..5607; actual unknown RTI pair physical811/CPU831B=40/70. Preserve historical records. Also incorporate B63A corrected B5B9 join and all current newly classified source. Commentary/source/game.asm unchanged; prior readability gate needs expanded-scope refresh; final editorial audit pending.

Broad later-game/boss/ending/credits and paired E2FB experiments reused; no redundant gameplay. Existing toolkit NES-001 confirmed issue/NES-002 missing feature,0blockers/no new issue; analysis/toolkit-issues.md preserved. ROM/configured binary/source unchanged; no toolkit edit/archive/deletion/cleanup/publication. All mutations cease after final identities/progress handoff.

## Round005 disassembly active
Input maps/round-004-disassembly-final.json;131058 classified/30unknown. Independent review of R4-SHARED-TARGET-RESET exact5byte contract; phase/initial per-bank ledger in analysis/round-005/phase.json and unknown-ledger-before.json. No new runtime/reachability claim; earlier rejection and correction retained.

## Round005 disassembly complete; full current-source commentary next

ROM classification: 99.98% (131063 / 131088 bytes classified; 25 bytes unknown).

Accepted R4-SHARED-TARGET-RESET5StaticCode at physical9813..9818/G0A645..A64A,banks[0,1,2,3]. Independent review combines known043F zero endpoint and accepted conditional no-call frame consumer; actual entry/natural selection remains unresolved. Original rejection and redirect correction preserved. Exact evidence analysis/round-005/review.md, independent-review.json,accepted-proposals.json,annotation-rpc.jsonl. Connected consumer/tail bytes already classified; ten remaining local ambiguities supply no newly discriminating contract. No unchanged gameplay/artificial-entry replay. One pass complete; ROM incomplete, no terminal exhaustion assertion.

Fresh configured decode, map.validate/save/export and byte-identical checkpoint succeeded: maps/round-005-disassembly-final.json; analysis/round-005-disassembly-final-verification.json; generated/round-005-disassembly-final. Internal analysis/state.json and portable maps/current.json saved. Code48115,other82948; exact counts/perbank and complement analysis/round-005/classification-counts.json and unknown-ranges.json. All classified Code has explicit CPU bank views/evidence; no new Observed bytes or gameplay coverage. Existing broad progression/boss/ending/credits reused.

Per-bank code/data/unknown: 0:6167/2016/9; 1:7260/927/5; 2:7143/1049/0; 3:6850/1338/4; 4:1434/6758/0; 5:3469/4723/0; 6:1801/6391/0; 7:372/7820/0; 8:967/7225/0; 9:0/8192/0; 10:34/8158/0; 11:328/7864/0; 12:3739/4448/5; 13:6620/1570/2; 14:1462/6730/0; 15:469/7723/0. Header16 classified non-code bytes separate. Residual physical277:1;811:2;2801:3;3140:3;11951:3;14251:2;25939:3;30710:1;106162:5;113405:2.

Source commentary unchanged and expanded-scope readability refresh now required; replace former unknown reset prefix with meaningful reset/frame contract. Review stale current source33832 “adjacent unknown reset continuation” wording. Final editorial remains pending. Toolkit report analysis/toolkit-issues.md retains1confirmed issue/NES-001 and1missing feature/NES-002,0suspected,0blockers,no new finding; toolkit unchanged. ROM unchanged; no archive/deletion/cleanup/publication.

## Round005 commentary active
Entire prior commentary preserved at analysis/round-005/commentary-prior with per-file hashes. Fresh current-map baseline preparation follows. Carry valid physical review; new reset9813..9818 and affected mirror130979/current residual wording require edits. Full readability/rebuild/read gate pending; final editorial audit reserved. Disassembly checkpoint maps/round-005-disassembly-final.json:131063 classified,48115Code,82948other,25unknown/10slices.

# Round005 commentary complete

Full current-source readability complete for maps/round-005-disassembly-final.json:131063/131088 classified(99.98%),48115Code/82948other,25unknown/10intervals. ROM classification remains incomplete; final editorial audit pending. Map/state/ROM unchanged.

All1491 scaffold entries,1308Code regions/21838instructions,173non-code regions,10unknown boundaries and677 meaningful units reviewed. Carried1307Code regions/21836instructions and676units by exact physical identities and valid dependencies; no source readability backlog remains. New5Code bytes physical9813..9818/G0A645 clear SharedMotionTarget and fall into A64A JMPD53E. Known zero endpoint, conditional no-call consumer and retained frame/stack requirements are explained; actual entry/natural selection unproved. Corrected stale source “adjacent unknown reset continuation” at G3FF93/physical130979 to mapper-zero G0FF96 hardware initialization. Current uncertainties exclude the accepted reset prefix; historical records preserved.

Entire prior commentary hash-preserved at analysis/round-005/commentary-prior before fresh baseline preparation. Scope/evidence: analysis/round-005/commentary-scope.json,commentary-dependency-review.md,commentary-preservation.json. Fresh byte-identical verification analysis/round-005-commentary-verification.json; source commentary/source/game.asm SHA256 fdc6762e2f0a9640869a503900292a59e08a0c2787fcab2586412bbe75daa5e7; refreshed commentary/reading-after.asm. No final audit run.

Redirect residuals analysis/round-005/unknown-ranges.json:physical277:1;811:2;2801:3;3140:3;11951:3;14251:2;25939:3;30710:1;106162:5;113405:2. Use accepted correction analysis/round-005/review.md plus localized competing interpretations analysis/redirect/round-003-a/candidate-review.md and round-004-a findings. Assess strongest remaining reasoning under evidence-policy.md before terminal acceptance; no terminal exhaustion asserted here. Reuse broad later-game/boss/ending/credits and E2FB observations; avoid unchanged decode/artificial-entry replay. Unknown meanings remain in commentary/uncertainties.md.

Toolkit report analysis/toolkit-issues.md unchanged:1confirmed NES-001,1missing feature NES-002,0blockers,no new findings. No map/classification/toolkit/ROM edits, archive/deletion/cleanup/publication. All writes cease at this handoff.

## Round005 redirect active
Unique attempt analysis/redirect/round-005-a; input maps/round-005-disassembly-final.json and completed current-source commentary. 131063 classified,25unknown/10slices. Primary rejection reasoning under review, no terminal conclusion yet. No map/state/source/toolkit/ROM edits.

## Round005 redirect complete; incomplete pending coordinator primary review
No ready leads or proposed bytes after current-source/primary consumer/control/representation review. Exact25unknown/10slices unchanged,131063classified. All candidate role types/localized competing interpretations and decisive evidence in analysis/redirect/round-005-a/rejection-review.md; fresh primary-source/unknown-ledger/survey linked. No revised discriminating method survives current evidence; coordinator must inspect reasoning before terminal acceptance. Compact handoff analysis/handoffs/round-005-redirect.md. leads.json appended empty-result pass, historical leads preserved. Broad gameplay and E2FB evidence reused. Identities verified unchanged; no classification/map/state/source/toolkit/ROM edits. Existing1confirmed/1missingfeature toolkit issues,0blockers,no new findings. Final editorial audit remains pending; ROM incomplete. All writes cease at handoff.
