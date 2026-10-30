# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 disassembly — in progress
Baseline established and byte-identical: [verification](baseline-verification.json).
Configured release binary retained; ROM unchanged. MMC3 mapper 4, 16 physical
8KiB PRG banks and 128KiB CHR. Reset vector $FF00, initial effective PRG slots
[0,1,14,15]. API and NES docs inspected. Startup 300-frame trace timed out;
retrying shorter bounded frame batches, preserving explicit timeout evidence.
Working scripts and append-only RPC evidence: round01/. No classification yet.
This is an interrupted-safe intermediate record, not phase completion.

Initial classified checkpoint: [round01-initial verification](round01-initial-verification.json).
ROM classification: 50.95% (133566 / 262160 bytes classified; 128594 bytes unknown).
2478 code bytes, 16 header data bytes, 131072 hardware-defined CHR image bytes.
Static vector/observed-root closure inspected instruction by instruction; exact
candidates retained in round01/static-candidates.json; unsupported BRK/KIL retained
unknown. 76 regions. Gameplay still in progress; title delay being bypassed with
uninstrumented bounded wait, preserving snapshots and subsequent fetched contexts.

Subsequent checkpoint round01-gameplay is verified byte-identical, with 5282 code
bytes. IMPORTANT correction: early button/pressed API calls were silently ignored
by NES (NES-001); captured stage was attract demo, not player-controlled progress.
Observed fetch classification remains sound. Real menus now reached with buttons
arrays. Three distinct natural stage entries visited via stage-selector A cycling
(1/4/7 pulses), with movement, sword, jump and falcon input sequences.
See scenarios.json and round01/stages.py. Artificial stage-$50=7 shortcut running.
Banks0–7 now classified generic data after repeated grids/arrays/record inspection;
uniform higher-bank FF spans classified independently. State/map saved; current
counts supersede the initial checkpoint, fresh verification pending this batch.

## Round 01 single disassembly pass — finished, ROM incomplete
ROM classification: 86.96% (227971 / 262160 bytes classified; 34189 bytes unknown).

11629 Code, 85270 Data, 131072 Image bytes; 508 evidence-bearing regions.
Classified bytes have structural annotations; unknown complement remains explicit
in [round01/unknown-ranges.json](round01/unknown-ranges.json). Commentary/readable
naming remains a separate next phase: only three semantic region labels exist.
4293 distinct fetched physical ROM opcode offsets retained across saved scenario
contexts (lower-bound execution metric, separate from classified operand bytes).
Trace detail capacity0 intentionally drops ring events; all examined summary
aggregate_dropped=0. Accepted fetches include attract, player-input and artificial
scenarios, with no natural reachability claims for artificial paths.

Final checkpoint [round01-disassembly verification](round01-disassembly-verification.json):
map validation and ca65 rebuild passed, byte_identical=true. Frozen map
maps/round01-disassembly.json; generated/round01-disassembly/game.asm; current
analysis/state.json and maps/current.json agree. Input ROM/toolkit unchanged.

| Section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 0 | 8192 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 0 | 8192 | 0 |
| PRG 04 | 0 | 8192 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 0 | 8192 | 0 |
| PRG 08 | 1995 | 517 | 5680 |
| PRG 09 | 0 | 1128 | 7064 |
| PRG 10 | 0 | 3616 | 4576 |
| PRG 11 | 0 | 6254 | 1938 |
| PRG 12 | 0 | 6498 | 1694 |
| PRG 13 | 2499 | 0 | 5693 |
| PRG 14 | 2101 | 0 | 6091 |
| PRG 15 | 5034 | 1705 | 1453 |
| CHR | 0 | 131072 | 0 |

Broad gameplay visited natural distinct stages and artificial Ruth/later-room
layouts plus altar puzzle; no boss defeat or ending/credits claim. Exact routes,
patches, contexts, operation/frame budgets and outcomes: scenarios.json and
round01 scripts. Validator success at D582 is supported; terminal transition
remains unresolved after bounded test. Stop is ordinary first-pass easy-gain
handoff, not terminal research exhaustion: call-return bank contracts, remaining
code/data boundaries and ending transition need coherent redirect investigation.
No initial leads existed; R01 lead outcomes now persisted in leads.json.

Toolkit findings: one confirmed documentation/API bug NES-001, zero suspected
bugs, zero missing features, no blockers; see [toolkit-issues.md](toolkit-issues.md).
Disk inspected at handoff: approximately 11GiB free, round01 evidence101MiB before
final small reports. No cleanup/deletion/archive/publication performed.
Final storage check after annotation/report generation: round01 evidence129MiB,
approximately11GiB free; ROM SHA256 rechecked and matches workspace identity.

## Round01 commentary — current-source readability complete

All currently classified source reviewed: 636 scaffold intervals (478 code fragments,
29 Data, one CHR, 128 honest unknown notes), 193 mechanics units, 5,523 instructions
and 852 code-entry/branch symbols. Source now explains audio, menus/stages,
player/falcon/object behavior, collision and fixed-point movement, rendering,
inline call/dispatch contracts, NMI banking/tasks and altar validation. RAM, field,
buffer, controller and hardware names preserve exact values/encoding.

Fresh commentary verification byte_identical=true, source SHA256
`d0e1fb0cdc7d722a499e4863e8b347c616d5eeb35b13e7c2836ced65d3dcf102`;
reading copy refreshed. Structural checkpoint round01-commentary passed as well.
State/map unchanged; classification remains 227971/262160 with34189 unknown.
Readability scope reconciliation and handoff: round01-commentary/scope-check.json,
round01-commentary-handoff.md. No remaining present-source readability frontier.
Final editorial audit deliberately pending; no whole-ROM completion claim.

R01-CALLS gains explicit ordinary-return, inline-pointer-skip and tail-dispatch
contracts. F85A is resolved as the STA countdown entry: A=$3C means60 NMI
decrements. R01-ENDING must inspect actual success continuation/scene initialization
and NMI sprite-zero polling rather than assume a separate delay helper. Remaining
boss/object contracts and consumer-constrained data boundaries are redirect work.
No execution/scenario changes. No new toolkit issues; report retains one confirmed
NES-001 issue, no blockers. Approximately11GiB free. Input/toolkit untouched;
no cleanup, deletion, archive or publication.

## Round01 redirect research checkpoint
See analysis/round01-redirect-handoff.md and analysis/redirect/round01-contracts/results.md. Input checkpoint round01-commentary unchanged:227971classified/34189unknown. Combined proposals32152unique bytes (6770Static code,25048Data,32inline,302dispatch/index records), none accepted in redirect. Exact128-range unknown complement independently recomputed; no proposed Data overlaps known/new code or any recorded fetch. Ending artificial route now reaches terminal password and Start-to-Special-stage screen; prior supposed stall corrected using stack/source timing. Existing lead history preserved and ready proposals updated. Read frontier.md before phase transition: credible per-case return summaries remain; no terminal exhaustion claim. No new toolkit issues; persistent NES-001 nonblocking. Input/source/map/state/binary unchanged, no archive/cleanup/publication.

## Round01 redirect continuation complete — ready for disassembly
Prior nonterminal research was resumed in the same analysis/redirect/round01-contracts attempt. Final authoritative handoff reconciled at analysis/round01-redirect-handoff.md. Combined proposals now34184unique bytes:8164Static code3646instructions,25576Data,32inline,412dispatch/index records; zero accepted here. Actual checkpoint unchanged227971classified/34189unknown128intervals; hypothetical5remaining only if all proposals pass independent review. Original BDF0/C6A4/BC33/E0BD return frontier resolved with per-case/balanced contracts; reset-toB000 bank context established; actualIRQ SEI/RTI distinguished from reset mapper write; C0 producer and mode3 independent structural contract reviewed; helper-mediated state continuations followed. No identified promising actionable method remains deferred under reasonable redirect standard. Two tiny hypothetical residual claims retain localized competing interpretations; no terminal exhaustion claim. Ready for independent disassembly, then current-source commentary/redirect as appropriate. Source readability/sourcehash unchanged; final editorial audit pending. Ending terminal password/Start-to-Special-stage route recorded previously; no new gameplay replay. Input/map/state/source/binary unchanged, frozen-map byte equality and expected sourceSHA checked. No new toolkit issues; NES-001 remains nonblocking. No active tests, deletion/archive/publication. Final logs bounded by cached decoding; concise decisive artifacts retained.

Round02 data-intermediate: 253991/262160 classified; 8169 unknown; counts={'Code': 11629, 'Data': 111290, 'Image': 131072}

Round02 disassembly: 262155/262160 classified; 5 unknown; counts={'Code': 19793, 'Data': 111290, 'Image': 131072}

## Round02 disassembly — reviewed redirect batch accepted

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

Rounded100.00% is not completion:5unknown bytes remain. Single pass independently accepted34184new bytes:8164Code(3646instructions),25576Data,32inline pointer bytes,412selected dispatch/index bytes. Final totals19793Code/111290Data/131072CHR, 881regions with evidence. See [independent review](round02-disassembly/review.md), [mechanical review](round02-disassembly/mechanical-review.json), [metrics](round02-disassembly/metrics.json), [exact residual](round02-disassembly/unknown-ranges.json). No proposal rejected; specific-schema claims narrowed to generic Data where full layouts remain unproven. Conditional return summaries and mode3 structural contract do not assert natural reachability or universal bank preservation.

| Section | Code | Data | CHR image | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 0 |
| PRG 00 | 0 | 8192 | 0 | 0 |
| PRG 01 | 0 | 8192 | 0 | 0 |
| PRG 02 | 0 | 8192 | 0 | 0 |
| PRG 03 | 0 | 8192 | 0 | 0 |
| PRG 04 | 0 | 8192 | 0 | 0 |
| PRG 05 | 0 | 8192 | 0 | 0 |
| PRG 06 | 0 | 8192 | 0 | 0 |
| PRG 07 | 0 | 8192 | 0 | 0 |
| PRG 08 | 1995 | 6197 | 0 | 0 |
| PRG 09 | 0 | 8192 | 0 | 0 |
| PRG 10 | 0 | 8192 | 0 | 0 |
| PRG 11 | 0 | 8192 | 0 | 0 |
| PRG 12 | 0 | 8192 | 0 | 0 |
| PRG 13 | 3966 | 4226 | 0 | 0 |
| PRG 14 | 7885 | 306 | 0 | 1 |
| PRG 15 | 5947 | 2241 | 0 | 4 |
| CHR | 0 | 0 | 131072 | 0 |

Data intermediate checkpoint maps/round02-data-intermediate.json:253991classified/8169unknown. Final state/map agree; frozen maps/round02-disassembly-verified.json and generated/round02-disassembly-verified/game.asm passed fresh validation and byte-identical ca65 rebuilding, report [verification](round02-disassembly-verified-verification.json). Prior commentary sourceSHA d0e1fb0cdc7d722a499e4863e8b347c616d5eeb35b13e7c2836ced65d3dcf102 preserved; expanded current-source commentary pending separate phase, final editorial gate pending.

Leads R01-CALLS/ROOMS/ENDING/DATA now verified for accepted structural slices, histories retained in leads.json. Scenario checklist reviewed: representative stages/later rooms and artificial congratulations/terminal password/Start-return already visited; natural bosses/alternate endings remain unverified. No new promising gameplay recipe targets the five residual bytes; no unchanged route replay. Retained contextual fetch-file union4499unique physical opcode offsets (includes attract/natural/artificial, not execution-byte coverage);171 newly unknown-at-input roots are separately saved in redirect. No runtime coverage claims inferred from Static annotations.

Residuals: physical116286 bank14 CPUC62E byte60, and123710..123713 bank15 CPUE32E A900855F. Literal/entry alternatives remain materially ambiguous; no raw-valid-decode shortcut. Ordinary disassembly handoff, not terminal exhaustion; commentary then redirect reviews accepted boundaries/remaining subclaims. No new toolkit issues found; persistent [toolkit report](toolkit-issues.md) retains1confirmedNES-001,0suspected,0missing features,no blockers. Input/toolkit untouched; no archive/cleanup/deletion/publication.

## Round02 commentary — full current-source readability complete

All883scaffold intervals reviewed;508exact physical/kind prior blocks and193valid prior units retained from the entire preserved analysis/round02-commentary-history/prior-commentary tree. Applied130newCode fragments/8164bytes/3646instructions and243newData fragments, adding373units. Corrected changed dependency prose and shared RAM/object/direct-call names. Zero pending readability frontier; final editorial audit pending. Fresh byte-identical verification sourceSHA3a548d6ea5af0a27469bf16b9fcab7338a7901a5166bc12d169b92100339847d; reading-after.asm refreshed. See analysis/round02-commentary-handoff.md,scope.json and integrity.json. Map/state/input unchanged. Classification still262155/262160,5unknown; no terminal exhaustion claim. Redirect should review localized C62E/E32E alternatives against expanded source and evidence-policy.md. No runtime replay/new coverage. No new toolkit issues; persistent NES-001 nonblocking. No toolkit edits,archive,cleanup,deletion or publication.

## Round02 redirect — localized rejection review, no ready proposal

Input round02-disassembly-verified262155/262160classified,5unknown; current-source883interval readability complete, final editorial pending. Recomputed exact complement116286len1 bank14C62E60 and123710len4 bank15E32EA900855F. Materially different static contract test compares horizontal/vertical$5Fconsumers: horizontal has real nibble reader, vertical does not; paired-entry symmetry therefore fails to independently discriminate a clear wrapper. Singleton RTS plus unselected pointer lacks an independent role constraint. Neither rejection demands natural/live caller universally; mode3 accepted criteria compared explicitly. Broader source/state/table/bank/progression avenues triaged, unchanged sound evidence reused. No useful revised method or ready proposal remains identified. See analysis/round02-redirect-handoff.md and analysis/redirect/round02-residuals/rejection-review.md; coordinator inspection required before terminal incomplete acceptance. Empty pass appended leads.json; prior entries/history retained. No new toolkit issues; NES-001 nonblocking. No runtime replay or map/state/source/input/toolkit edits; no archive/cleanup/deletion/publication.

## Final editorial acceptance — source accepted, ROM incomplete

Targeted final review reused883intervals/566uniqueunits, added98supported entry/state labels and resolved114raw known Code target words, corrected MachineB five-state transition prose and source-root-relative inventory locations/attachments. Supported commentary audit passed fresh byte-identical verification,editorial_audit=true,883regions/958unitattachments;reading copy refreshed. Final sourceSHA6e4d05976e3b851dc0c4750a70a05456def301326b7ed1678ebbc48f2dcc9056. See analysis/final-editorial-handoff.md,final-editorial-audit.json and final-editorial/integrity.json. Zero pending readability/editorial acceptance work. Classification unchanged262155/262160,5unknown; coordinator accepted terminal residual reasoning. No runtime replay,new toolkit finding,toolkit edit,archive,cleanup,deletion or publication.

## Terminal incomplete handoff and final editorial acceptance

Round02 redirect returned no ready proposals after localized rejection review and broader practical source triage. Coordinator inspected primary source/reference evidence and acceptance-standard consistency; assessment saved in coordinator/rejection-assessment.md. No corrected inference or credible discriminating test remains identified. Terminal incomplete is justified with262155/262160classified,5unknown (rounded100.00%,exact99.9980927677754%), not blocked or whole-ROM complete. Exact residuals and all evidence links: terminal-handoff.md. Do not restart unchanged investigation automatically.

Targeted final editorial acceptance reused valid review, resolved known-code pointer/entry symbols and stale transition prose, reconciled source locations/coverage, and passed supported audit with fresh byte-identical verification/read refresh.883regions,566uniqueunits/958attachments; final sourceSHA6e4d05976e3b851dc0c4750a70a05456def301326b7ed1678ebbc48f2dcc9056. No readability/editorial backlog. Final handoff final-editorial-handoff.md; current audit commentary/audit.json. Structural map/state remain round02-disassembly-verified. Archive document-selection contract inspected; options explicitly select final/history Markdown and omit only duplicate historical instruction templates. Final coordinator audit records current-state acceptance separately from ROM incompleteness. No archive,cleanup,deletion,publication or toolkit edits.

Final coordinator current-state audit passed: coordinator/completion-audit.json. Archive.py editorial-status contract complete, all source hashes current, fresh audit/read copy valid, exact5-byte complement and checkpoint identities verified. archive-options.json selects24additional final/history Markdown and omits3duplicated historical instruction templates; no unrecognized Markdown remains. Orchestration terminal handoff achieved; ROM classification remains incomplete. No archive execution or cleanup authorized/performed.
