# Disassembly progress

**Round001 disassembly finished; classification incomplete.** Commentary and redirect remain separate next phases.

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).

Rounded100.00% does not establish completion: sole unknown physical[$5725,$5726), CPU$D715, PRG bank2, identity banks[0,1,2,3]. Code17777/data15006/image32768. Every8616 region has evidence; static-only regions are Static, accepted fetched instructions Observed. Full current-source readability/editorial review has not run.

| Physical storage | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 0 | 0 |
| PRG0 | 0 | 8192 | 0 | 0 |
| PRG1 | 2016 | 6176 | 0 | 0 |
| PRG2 | 7808 | 383 | 0 | 1 |
| PRG3 | 7953 | 239 | 0 | 0 |
| CHR0 | 0 | 0 | 8192 | 0 |
| CHR1 | 0 | 0 | 8192 | 0 |
| CHR2 | 0 | 0 | 8192 | 0 |
| CHR3 | 0 | 0 | 8192 | 0 |

Durable state: [state.json](state.json); portable map [current.json](../maps/current.json); verified checkpoint [pass-001](pass-001-verification.json) / [exact map](../maps/pass-001.json) / [source](../generated/pass-001/game.asm). Both baseline and pass-001 rebuild byte-identically. Input SHA256 unchanged: 23d30a9769972369d5b280aabefac488400a31f4c938b1f99469befb6458671e. Map hashf41710f57e8666a751108587e1af0095a64a890466f1a045c55360ce82158201. map.validate succeeded.

Execution coverage separately: natural2382 ROM opcodes/4789 bytes; combined natural/artificial3852 opcodes/7875 bytes. Natural RAM117/artificial RAM111 unique opcode roots retained separately; no ROM upload-origin inference. Aggregate drops0 in saved scenario summaries; detail capacity0 intentionally retains no detail.

Gameplay: natural title/attract, difficulty/password and Stage1 combat; artificial$8C stage-index shortcuts reached3,6,7 and exercised movement/fire/bomb; $ED=1 stage7 clear plus$8B=2 reached graphical ending cinema. Lower-difficulty completion returned to Albatross selection. Boss kills/secret rooms and natural ending reachability are unverified. Exact reproducible recipes/budgets, snapshots and milestone checklist: [scenarios](scenarios.json), [gameplay](gameplay.md).

Easy gains included connected static call/branch paths, supported partial/full dispatch slices, bounded mapper/consumer contracts, data patterns and exact boundaries. Final connected candidate conflicts0. See [investigation](investigation.md) and [counts/evidence](round-001/counts.json). Intermediate progress preserved at round-001/progress-intermediate.md.

Remaining D001-RTS-D715: unreferenced extra RTS between reset JMP and separately targeted RTS. No encoded direct pointer/fetch found; this does not prove data/dead code. Keep unknown pending redirect structural reasoning review. Concrete primary evidence and competing interpretations: [orphan-rts](round-001/orphan-rts.txt). Semantic follow-ups: RAM upload encoding/origin, boss enum/state contracts. No unchanged failed experiments should be repeated.

Toolkit findings: **1 nonblocking missing feature**, NES-001: generic single-button parameters silently ignored by NES input API; corrected to buttons array. No confirmed or suspected emulation bug. [Persistent report](toolkit-issues.md). No toolkit edits.

Compact phase handoff: [round-001-disassembly](handoffs/round-001-disassembly.md).

## Coordinator phase boundary

Round001 disassembly handoff inspected; required classification metric printed to user before commentary. Current phase: round001 commentary, separate sequential worker. Input checkpoint pass-001; output pending. Consumed lead batch: initial/static/runtime discovery; produced D001-RTS-D715 and semantic questions as recorded in handoff/leads. No terminal decision. Previous goal turn made progress through saved classifications and gameplay evidence; current worker confirmed live.

## Round001 commentary phase complete

Full current-source readability completed after disassembly. Maintained local commentary/source has21 functional modules, shared symbols,287 explained routine/shared entries,952 named encoded control destinations and391 reviewed meaningful units. All8588 classified instructions and all data/image/unknown boundary notes covered;8617/8617 scaffold records reviewed with source locations. No remaining readability ranges. Classification remains65551/65552, sole unknown[$5725,$5726), unchanged map/state hashf41710f57e8666a751108587e1af0095a64a890466f1a045c55360ce82158201.

Fresh byte-identical final commentary rebuild passed; input/linked SHA256 unchanged. Reading-after refreshed. Reports: commentary/verification.json, analysis/round-001-commentary/final-verification.json and completion.json. Final editorial audit remains pending; no audit/redirect/archive/publication run. Source-level C001 lead: renderer reads four-byte offsets at physical[$6A4F,$6A53) while first byte remains Static BRK; always-taken predecessor branch and finite consumer bounds warrant independent redirect review. Full lead and limitations in analysis/round-001-commentary/redirect-leads.md and commentary/uncertainties.md. Compact handoff: analysis/handoffs/round-001-commentary.md. No toolkit or ROM changes, new emulator work or deletion.

## Coordinator redirect start

Commentary handoff and completion inventory inspected, with applied source examples (game.asm module organization and actor-support finite sprite-pair consumer). Full current-source readability gate accepted; final editorial acceptance remains pending. Current phase round001 redirect, separate worker, input pass-001/current commentary; output pending. Consumed questions D001 and C001 plus existing semantic opportunities; no classification change or terminal decision. Previous turn made progress through applied verified commentary.

## Round001 redirect active structural review

Unique attempt analysis/redirect/round-001-structural-review; primary-boundaries.json refreshes exact complement (sole unknown physical22309 / $5725, CPU$D715, bank2/view[0,1,2,3]). C001 combines exact four-byte consumer and forced BMI boundary; incoming-controls.json checks direct controls to boundary/interior and D001 success RTS. Original connected candidate captured in origin-connected-*.json. No map/source/state changes. Primary D001 review underway; no new structural justification yet. Gameplay broad review follows source-supported eight-part constructor through a bounded artificial stage-stream recipe and existing ending continuation; probe.py/rpc.jsonl retain exact operations. Active frontier.md retains retry premise and stop conditions.

## Round001 redirect complete — combined ready batch

No classification/source/toolkit/ROM changes. Input/output pass-001 map/state hash unchanged f41710f57e8666a751108587e1af0095a64a890466f1a045c55360ce82158201; exact residual remains1 byte[$5725,$5726), CPU$D715 bank2/view[0,1,2,3]. C001-SPRITE-PAIR-BOUNDARY ready: finite two Y/X-pair consumer plus forced BMI boundary supports one-byte Static Code→Data correction (net unknown gain0). C002-COMMAND-CHAIN-BIT ready: dispatcher saves original bit7, contrary to2 source low-bit claims. D715 primary scope-correct rejection review retains genuine role ambiguity; no natural caller prerequisite imposed. Loader bank-aware resource0/1/2/3/6 origins independently match1024 saved RAM bytes; resource8 runtime mutation and resource9 non-CHR suffix localized. Current toolkit rejects uploaded execution views, so no speculative Code map proposed.

Broad gameplay now includes all7 stages, artificial multipart controller constructor/launch with source-supported threshold and movement/fire, ending terminal PUSH START frame7253 and longer Start acknowledgement to SELECT LEVEL/ULTIMATE frame8036. Exact recipes/evidence in analysis/redirect/round-001-structural-review and updated scenarios.json. First synthetic06 selectedtype6; corrected03 selected desiredtype3. One traced180 request timed out; bounded trace8 windows/untraced120 calls succeeded. No new toolkit issue.

Attempt complete, all probes finished. final-identities.json/unknown-ledger.json/findings.md/frontier.md retain primary evidence and remaining dispositions. Ready batch and verified semantic constraints return to independent disassembly then commentary/redirect. No terminal acceptance, final editorial/archive/publication. Free disk3.1GiB, owned scratch4KiB; no deletion. Compact handoff analysis/handoffs/round-001-redirect.md.

## Coordinator round002 start

Round001 redirect compact handoff inspected; combined C001/C002 actionable corrections justify next cycle despite zero expected unknown gain. New sequential disassembly worker uses pass-001 and completed redirect handoff. Current phase round002 disassembly; checkpoint output pending. D001 remains unknown; terminal acceptance not considered while ready corrections exist. Previous turn made progress through durable redirect evidence and combined handoff.

## Round 002 disassembly — pass-002 verified

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).

Incomplete: physical [$5725,$5726), CPU $D715, PRG bank2/view[0,1,2,3] remains ambiguous. Independent C001 finite consumer/control boundary review corrected physical$6A4F from Static Code to Static Data and labeled complete [$6A4F,$6A53) SpritePairOffsets. C002 independently verified original bit7 chaining/low7 type; source corrections explicitly handed to commentary, source untouched. See analysis/round-002/review.md and fresh pair/command decodes. No additional classification gained; code17776/data15007/image32768. Regions8615/8615 have evidence.

Per-bank code/data/unknown: PRG0 0/8192/0; PRG1 2016/6176/0; PRG2 7808/383/1; PRG3 7952/240/0. Header16 Data; each CHR0..3 Image8192/unknown0. Exact counts analysis/round-002/counts.json. Execution coverage unchanged; saved natural/artificial provenance and broad all7 stages/multipart/terminal ending+ack reused. No unchanged replay or D715 forced entry. D715 primary scope-correct rejection reviewed, unchanged; classification cannot be guessed from opcode validity or absent execution.

map.validate succeeded; state saved/exported. maps/current.json = maps/pass-002.json SHA256 4e9a95522c5d7038a7a062d0fa8386e1acc7b938c0b07aa7ee95b83d3cde02b9. analysis/pass-002-verification.json confirms byte-identical generated/pass-002 rebuild. ROM unchanged at workspace hash. Full current-source commentary remains next; preserve previous source commentary and incorporate redirect findings plus C001/C002 corrections. No terminal/whole-ROM completion claim. Existing nonblocking NES-001 in analysis/toolkit-issues.md; no new toolkit issues found. No toolkit changes or deletion/archive/publication. Free disk approximately3.1GiB; no exploratory bulk scratch added.

## Coordinator round002 commentary start

Verified pass-002 handoff/report; required ROM classification metric printed before commentary. Current phase round002 commentary, separate worker; input pass-002; output pending. C001 structural correction accepted, C002 prose repair verified for carry-forward. Classification remains65551/65552 with1 unknown. Full prior readability credit will be reused with affected-range corrections. No terminal decision.

## Coordinator round002 redirect start

Round002 commentary handoff inspected: complete current readability with390 units,8616 region reviews, fresh verification/read and inventory repairs. Separate round002 redirect worker started from pass-002 and current commentary. C001/C002 resolved; D001 unknown unchanged. Await independent current-opportunity/rejection review before terminal acceptance. Previous turn progressed verified carry-forward commentary.

## Coordinator terminal acceptance

Round002 redirect handoff and decisive primary decode/control/ROM artifacts independently inspected. Scope-correct rejection accepted: one-byte D715 code/data ambiguity persists; no useful revised inference/test or ready lead. Both rounds completed sequentially, no active worker/probe frontier. Current phase: terminal incomplete handoff; checkpoint pass-002. Full readability and final editorial audit completed; fresh audit rebuild byte-identical and reading refreshed. Configured store/map validation both succeeded. ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown). Whole-ROM classification remains incomplete. Final conclusions: final-handoff.md; preservation options and completion verification prepared separately. No archive/cleanup/publication.

## User-directed residual fallback — separate disassembly phase

Exact complement verified before mutation: physical [22309,22310) = [$5725,$5726), length1, byte$60, CPU$D715, PRG bank2/view[0,1,2,3]. User expressly authorized generic Data fallback for at most10 residual bytes without further research; only this1 byte was changed through supported map.annotate. Provenance Manual, confidence0; original meaning/code-versus-data role remains unresolved. This is a user-directed assumption, NOT evidence-proven structural classification, and makes no reachability claim. Prior terminating-JMP/adjacent independently targeted RTS ambiguity and rejection evidence remain unchanged in analysis/redirect/round-002-terminal-review/review.md and historical handoffs.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown). This structural-map total includes1 explicitly user-directed Manual fallback; evidence-backed count remains65551. Code17776/Data15008/Image32768. Per-bank counts: header Data16; PRG0 Code0/Data8192/Unknown0; PRG1 Code2016/Data6176/Unknown0; PRG2 Code7808/Data384/Unknown0; PRG3 Code7952/Data240/Unknown0; CHR0..3 each Image8192/Unknown0.

State saved; map exported/validated; fresh checkpoint pass-003-user-fallback rebuild byte-identical. ROM SHA256 unchanged23d30a9769972369d5b280aabefac488400a31f4c938b1f99469befb6458671e. Map SHA2565b7ff560c30bb130cbf4dac99e0648dd6cde223468a7a13614ea6da9b5b2df28. Verification: analysis/pass-003-user-fallback-verification.json. Exact pre/post counts, supported API log and identities: analysis/user-fallback-disassembly/. Commentary and fresh final audit remain next, so this disassembly handoff does not claim whole-deliverable completion. No research, gameplay replay, toolkit edits, archive/deletion/publication. Existing nonblocking NES-001 remains in analysis/toolkit-issues.md; no new toolkit issues found. Disk3.1GiB available, no scratch batch created.

## Round003 final acceptance — user-directed fallback

Separate sequential final reviewer accepted current pass-003-user-fallback after full inventory reconciliation and targeted source/dependency review with valid prior credit. Supported commentary audit passed freshly byte-identical; strict archive.editorial_status passed complete=true; configured map/store validation passed. Coverage65552/65552,0 unknown, including exactly1 Manual/confidence0 authorization fallback at physical22309; evidence-backed65551/65552. Original role unresolved. Readability complete8616 scaffold records/390 meaningful units,8587 Code records with owning units; no pending defects/frontier. Authoritative final handoff analysis/handoffs/user-fallback-final-audit.md supersedes historical terminal-incomplete conclusions without changing them. Fresh audit analysis/completion-audit-user-fallback.json and retained evidence analysis/user-fallback-final-audit/. Archive options reconciled; no archive/delete/publication/toolkit edit/ROM change. All final requirements under the explicit user exception satisfied; coordinator independent acceptance pending.

## Coordinator final fallback acceptance

Independently inspected the fresh final handoff, completion audit, exact pre/post complement and source fallback annotation. Ran analysis/coordinator/verify-user-fallback-final.py successfully against current state: all65552 bytes covered, Manual fallback exactly physical22309/length1, original role unresolved; evidence-backed65551. Strict editorial status complete with8616 reviewed regions and8587 nested units;390 meaningful review units retained. Current source hashes, refreshed reading, map/state/frozen identities, fresh byte-identical audit, map/store validations and all Markdown selection verified. Final acceptance under explicit user exception is complete; no blockers remain. Authoritative handoff: handoffs/user-fallback-final-audit.md. Historical incomplete conclusions remain preserved. No archive, cleanup, publication, toolkit edit or ROM change.
