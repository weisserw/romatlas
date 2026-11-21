# Disassembly progress

Round1 disassembly pass-001 finished; ROM incomplete. Ready for commentary, then redirect.

ROM classification: 84.05% (55096 / 65552 bytes classified; 10456 bytes unknown).

| Physical bank | Code | Non-code | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG-0 | 5132 | 46 | 3014 |
| PRG-1 | 5018 | 68 | 3106 |
| PRG-2 | 0 | 3856 | 4336 |
| PRG-3 | 0 | 8192 | 0 |
| CHR-0 | 0 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 0 |

94 evidence-bearing regions; Code10150 bytes, non-code44946 bytes.
Natural1888 and artificial2437 unique opcode starts are separate execution metrics.
No full-game compatibility/readability/editorial completion claim.

Latest checkpoint: [pass-001-complete verification](pass-001-complete-verification.json).
Maps/current.json and analysis/state.json match its exact map; generated/pass-001-complete
rebuilds byte-identically. Input hash unchanged. No processes remain after this phase.

[Detailed handoff](disassembly/pass-001/handoff.md),
[exact counts](disassembly/pass-001/counts.json),
[unknown complement](disassembly/pass-001/unknown.json),
[progression scenarios](scenarios.json), [gameplay](gameplay.md).

Lead outcomes: D1-001/D1-002 verified; D1-003..D1-006 ready for redirect,
not rejected or exhausted. Initial title, Player Start, courses0/7/15 driving
(artificial), courses1/3 intros and rescue ending initialization visited. Boss,
complete ending/restart and directed hazards require specific contracts.

No toolkit issues found; [persistent report](toolkit-issues.md).
No archive, cleanup, publication or toolkit changes.

## Round 1 commentary active

Prepared frozen commentary baseline from pass-001-complete; no map/state changes.
Entire classified code read; initial coherent source batch supplies 183 entry explanations and consistent hardware/RAM/table/control-flow names. Byte-identical commentary verify passed. Remaining same-phase data/gap notes, source inventory and accuracy checks are tracked in commentary/progress.md. Commentary is not yet complete; do not start redirect.

## Round 1 commentary complete — current classified source

Full current-source readability completed across all 10,150 code and 44,946 non-code bytes; 64 unknown gaps (10,456 bytes) explained locally without new classification. 158 reviewed scaffold entries, 183 reviewed routine units and zero classified code bytes outside unit coverage. Consistent hardware/RAM/field/control-flow names, meaningful routine explanations and symbolic main-state/vector words are in commentary/source/game.asm. Fresh byte-identical verify passed; reading-after.asm refreshed. Frozen map/state unchanged. Final editorial audit pending; ROM incomplete at 84.05%.

Handoff: [commentary round 1](commentary/round-1/handoff.md), [scope](commentary/round-1/scope.json), [verification](../commentary/verification.json), [redirect questions](../commentary/uncertainties.md). No remaining current-source readability ranges. Ready for coordinator-managed redirect; no redirect executed here. No toolkit issues found. No archive/deletion/publication/toolkit changes; 3.4 GiB free.

## Round 1 redirect active

Investigating sound command/stream graph, actor inner dispatches, ending progression and residual data under the completed commentary gate. Durable frontier and primary artifacts: [redirect round 1](redirect/round-1/progress.md). Classification unchanged at55096/65552 (10456 unknown). No terminal exhaustion or toolkit issue finding.

## Round 1 redirect complete — combined batch ready

Investigated sound command/stream graph, all concrete actor/terrain/player/sprite dispatch avenues, ending/finish/return, the independent five-state transition graph, arithmetic utilities and residual data containers. D1-003..D1-006 updated ready with exact ranges, contracts, bounded review recipes and preserved attempt history in leads.json.

Proposed 4,057 Code + 6,384 Data = 10,441 bytes; accepted here: zero. Actual classification remains 55,096/65,552 (84.05%), 10,456 unknown. Full acceptance would leave 15 tiny ambiguous bytes; exact current complement and each disposition are in redirect/round-1/unknown-ledger.json. No speculative generic sweep or terminal exhaustion claim.

Artificial progression reached the final opponent, contact response, defeat continuation, all 21 ending text entries and return to course-0 PLAYER START using documented accelerations. Airborne jump states visited. Natural defeat, ordinary-timing full ending and directed hazard permutations remain unclaimed. Runtime interruptions and a bounded timeout are preserved explicitly.

[Redirect handoff](redirect/round-1/handoff.md), [proposal](redirect/round-1/proposal.json), [findings](redirect/round-1/findings.md), [data evidence](redirect/round-1/data-evidence.md), [runtime results](redirect/round-1/runtime-results.md). Map/state/source/ROM hashes unchanged. Ready for separate independent disassembly review, then full current-source commentary; final editorial audit still pending. No toolkit issues found. No archive, deletion, publication or toolkit edits.

## Round 2 disassembly active

One pass reviewing D1-003..D1-006 from pass-001-complete. Input classification remains 55,096/65,552, 10,456 unknown. Reviewing exact consumer/stack contracts, corrected table boundaries, independent utility contracts and residual ambiguities against saved primary listings; no replay of already answered runtime questions. Proposed counts are not accepted counts. Commentary/source and frozen baseline retained for later reconciliation. Free disk 3.3 GiB; prior scratch 4 KiB. Review/output: disassembly/pass-002.

## Round 2 disassembly complete — pass-002-reviewed

ROM classification: 99.98% (65537 / 65552 bytes classified; 15 bytes unknown).

Independently accepted 4,057 new Code and 6,384 new Data bytes from D1-003..D1-006. 2,008 instruction starts/lengths freshly decoded under fixed CNROM PRG[0,1,2,3], no conflicts; original regions preserved. Total Code 14,207 and non-code 51,330 bytes in161 evidence-bearing regions. New annotations Static, with local data roles and primary contract links. Reachability remains separate.

| Physical bank | Code | Non-code | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG-0 | 7187 | 991 | 8178 | 14 |
| PRG-1 | 6944 | 1247 | 8191 | 1 |
| PRG-2 | 76 | 8116 | 8192 | 0 |
| PRG-3 | 0 | 8192 | 8192 | 0 |
| CHR-0 | 0 | 8192 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 8192 | 0 |

Exactly15 bytes remain unknown in five end-exclusive ranges:007c..0080,0238..023c,049e..04a3,1219..121a,3b8a..3b8b. [Independent review](disassembly/pass-002/review.md) evaluates split stub/return/BRK versus data claims without requiring universal live callers. No new discriminating constraint found; no arbitrary-PC or unchanged gameplay replay. Original runtime recipes cover artificial early/mid/late, final opponent/contact/defeat,21 ending text records/return and airborne jump. Landing, natural defeat, ordinary-timing full ending and hazard permutations remain unclaimed/deferred without a residual classification question.

Fresh map.validate/save/export and [pass-002-reviewed verification](pass-002-reviewed-verification.json) passed; source rebuild byte-identical. Exact map/state/current SHA2561d8769b7ce8b13320c632c6204d6d8e894866768abe1266531c67b2633a613e2; ROM unchanged. [Counts](disassembly/pass-002/counts.json), [unknown complement](disassembly/pass-002/unknown.json), [integrity](disassembly/pass-002/integrity.json), [handoff](disassembly/pass-002/handoff.md).

ROM incomplete. Next is full current-source commentary reconciliation for the enlarged classified source; preserved commentary/source and frozen baseline are unchanged. Final editorial audit pending. No commentary/redirect phase performed here. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, cleanup, archive, deletion or publication. All owned analysis servers exited;3.3GiB free.

## Round 2 commentary active

Entire prior commentary preserved in analysis/round-2/commentary before fresh preparation. Fresh baseline is pass-002-reviewed, SHA256 1d8769b7ce8b13320c632c6204d6d8e894866768abe1266531c67b2633a613e2; baseline rebuild byte-identical. Reconciliation by physical ranges and dependencies is active: preserve valid prior names/prose/review, cover 4,057 added Code and 6,384 added Data bytes and concrete dependency corrections. Fifteen unknown bytes remain unchanged. Same phase must continue until full current-source readability and fresh verification/read refresh. Final editorial audit reserved.

Round2 commentary code batch verified byte-identically (source 7f84430b790b98c78bcfc8fd254957307eef90cf0eaa229bb20a3be35f547f0c). Prior 94 classified regions retained by physical boundaries; new code explanations cover transition/ending, player/collision/terrain, actor update and sprite closures, independent utilities and audio stack contracts. Remaining same-phase data/branch/dependency/inventory work is tracked in commentary/progress.md; no handoff or audit yet.

## Round 2 commentary complete — current classified source

Full readability complete across Code 14,207 and non-code 51,330 bytes; unknown 15 in five exact intervals explained locally. Entire prior commentary retained under round-2/commentary before fresh preparation. Preserved valid names/prose/credit for 94 classified regions, reconciled new physical ranges and affected dependencies, added 176 explained entry/block roles and consistent routine/branch/RAM/field/OAM/table symbols. Inventory: 166 reviewed scaffold entries, 359 routine units, 57 data-structure units; zero uncovered code bytes and no remaining readability ranges.

Fresh commentary verification byte-identical; final source SHA256 eb8281d12a2235bc4a702b8a6e00fb5cf8de43e55cc159f48acc28dd3a3759c1, reading-after.asm refreshed. Map/state/frozen baseline remain 1d8769b7ce8b13320c632c6204d6d8e894866768abe1266531c67b2633a613e2; ROM unchanged. Concrete B9 triangle operand-width correction preserved in source and corrected local static grammar test; same 3,382 tokens/zero errors because no B9 token occurs in that graph. No classification implication, runtime replay or toolkit defect.

[Handoff](commentary/round-2/handoff.md), [scope](commentary/round-2/scope.json), [audio correction](commentary/round-2/audio-grammar-correction.md), [limitations](../commentary/uncertainties.md), [verification](../commentary/verification.json). Current-source gate complete; ROM remains 99.98% classified with 15 unknown. Final editorial/archive acceptance pending, audit not invoked. No toolkit issues found. No toolkit edits, cleanup, deletion, publication or input changes; no owned server, free disk 3.3 GiB, shared scratch 4 KiB.

## Round 2 redirect active

Current-source commentary gate met. Reasoning review of exact 15-byte residual complement against primary source/listings and prior references is active in [redirect round 2](redirect/round-2/progress.md). No classification/source/state changes; no gameplay replay or terminal conclusion.

## Round 2 redirect complete — R2-001 ready

Reasoning review found a corrected four-byte split candidate049f..04a3: BIT RAM17 computes Z from A&controller_current; BNE skips exactly default X=0 to existing starting-course store. Independent title selected/default paths and downstream course consumer agree. Static entry contract A=mask,X=validcourse; actual caller/natural selection unknown. PrecedingRTI remains separate.4Code proposed,0accepted; actual65537/65552classified,15unknown unchanged. All other residuals and obvious connected continuations reviewed; no additional actionable changed method. Not terminal exhaustion.

[Handoff](redirect/round-2/handoff.md), [findings](redirect/round-2/findings.md), [proposal](redirect/round-2/proposal.json), [fresh exact ledger/integrity](redirect/round-2/primary-check.json). Separate independent disassembly next, then full current-source commentary/redirect. No source/map/state/input/toolkit changes or replay. No toolkit issues found; no server,3.3GiB free, scratch4KiB. Final audit reserved.

## Round 3 disassembly active

Exactly one pass independently reviewing R2-001 against literal ROM, prior map, controller reader and selected/default title paths and course consumer. Candidate049f..04a3 excludes049e RTI. BIT-derived Z correction confirmed; a supported mask-A/valid-course-X entry condition is separate from unknown actual caller. No gameplay replay. Durable output: disassembly/pass-003.

## Round 3 disassembly complete — pass-003-reviewed

Exactly one substantive pass complete; ROM incomplete. Next: full current-source commentary reconciliation then redirect. Final editorial audit remains pending.

ROM classification: 99.98% (65541 / 65552 bytes classified; 11 bytes unknown).

R2-001 verified: four new Static Code bytes049f..04a3, effective PRG[0,1,2,3], CPU848f..8493. Independent field/conditional-join/downstream-consumer review supports the mask-A/valid-course-X contract, separate from unknown actual caller/natural selection. Preceding049e RTI remains unknown. [Review](review.md), [fresh decode](decode-review.json), [RPC](rpc.jsonl), [validation](validation.json). No new runtime execution; execution coverage unchanged.

Total Code14211, non-code51330;162 evidence-bearing regions. All161 prior regions preserved; new4bytes annotated Static with label/role/contract. Annotation coverage65541/65552; source readability is a separate pending phase for changed bytes/dependencies.

| Physical bank | Code | Non-code | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG-0 | 7191 | 991 | 8182 | 10 |
| PRG-1 | 6944 | 1247 | 8191 | 1 |
| PRG-2 | 76 | 8116 | 8192 | 0 |
| PRG-3 | 0 | 8192 | 8192 | 0 |
| CHR-0 | 0 | 8192 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 8192 | 0 |

Exact end-exclusive residuals007c..0080(4),0238..023c(4),049e..049f(1),1219..121a(1),3b8a..3b8b(1). [Counts](counts.json), [unknown complement](unknown.json). Reuse sound localized review in redirect/round-2/findings.md; no actionable changed test or connected unknown continuation identified here. Saved broad progression evidence reused without gameplay replay.

Fresh [pass-003-reviewed verification](../../pass-003-reviewed-verification.json) passed byte-identically. Map/state/current SHA25690c4126c01fcd06e3dc687064fda1288f02ec1e022ec37049170ffe359a75c16. [Integrity](integrity.json): ROM unchanged; commentary/source preserved SHA256eb8281d12a2235bc4a702b8a6e00fb5cf8de43e55cc159f48acc28dd3a3759c1. Preserve full source and prior commentary for next reconciliation.

No toolkit issues found; [report](../../toolkit-issues.md). No toolkit edits, cleanup/archive/deletion/publication or input changes; owned server exited. No commentary/redirect performed.

## Round 3 redirect complete — incomplete, no actionable leads

Fresh exact complement remains11 bytes in five intervals;65541/65552classified(99.98%). Retrieved literal stubs, actual vectors, neighbor flow, eight-entry NMI selector domain and audio stream origin. Compact terminal [rejection review](redirect/round-3/findings.md) evaluates reset JMP and separate zero, wrapper/secondRTS, isolatedRTI and parser zero using exact claims, scoped negative evidence, combined independent constraints and concrete competitors. Accepted BIT correction remains settled. No revised inference or discriminating test survives;0proposed/0accepted, no added/updated lead IDs, empty-result pass preserved in leads.json.

[Handoff](redirect/round-3/handoff.md), [fresh ledger](redirect/round-3/unknown-ledger.json), [integrity](redirect/round-3/integrity.json). Terminal research assessment incomplete requires coordinator inspection of reasoning, not only hashes. Current-source readability complete; final editorial/acceptance audit remains pending. Broad progression and unvisited milestone triage reused with natural defeat/landing/ordinary-timing ending limits intact. No gameplay replay, protected source/map/state/input/toolkit changes, cleanup/deletion/archive/publication or toolkit issues.3.3GiB free; no scratch batch or server.

## Final editorial acceptance complete

Separate final worker reused valid current-source review and inspected complete inventory plus targeted names/aliases, uncertainty notes, controller/course and B9 dependencies and correct vector storage. Fixed concrete redundant source/ inventory-location prefixes (initial audit rejected source/game.asm:357); assembly/ranges/line numbers unchanged. Historical vector typo corrected by note in commentary/editorial-audit.md, retaining old record. Final commentary audit passed, 167 reviewed scaffold entries / 417 units, fresh byte-identical verification and reading refresh. [Handoff](final-acceptance/handoff.md), [evidence](final-acceptance/evidence.json). Protected map/state/source/input/frozen hashes unchanged.

ROM remains incomplete:65541/65552classified(99.98%),11unknown; terminal redirect review reused without reopening unchanged research. Editorial acceptance passes separately from classification. No toolkit issues found. No toolkit changes, archive, cleanup, deletion or publication; coordinator archive-options/requirements audit remains separate.3.3GiB free, no new scratch/capture batch or server.


## Coordinator terminal handoff — justified incomplete

ROM classification: 99.98% (65541 / 65552 bytes classified; 11 bytes unknown).

All three sequential phase rounds and final editorial acceptance are finished.
Coordinator inspected terminal rejection reasoning and decisive primary artifacts;
no useful corrected inference/test survives. This ends orchestration with justified
ROM incompleteness, not full classification or compatibility. Full current-source
readability and final audit passed:167 scaffold entries,417 units, byte-identical
rebuild, reading-copy refresh. Current pass-003-reviewed map/state remain unchanged.
Fresh configured-release map and store validation both returned ok=true.

[Terminal handoff](terminal-handoff.md), [coordinator review](coordinator-rejection-review.md),
[final acceptance](final-acceptance/handoff.md), [completion audit](completion-audit.json),
[document selections](../archive-options.json). Every unrecognized Markdown document
is preserved; no disposable scratch Markdown was found, so omitted_documents=[].
No toolkit issues found. No archive,cleanup,deletion,publication or toolkit changes.
