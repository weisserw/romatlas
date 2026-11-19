# Disassembly progress

Round 1, disassembly single pass complete; ROM incomplete. Next phase: commentary then redirect.

ROM classification: 70.37% (276715 / 393232 bytes classified; 116517 bytes unknown).

| Physical bank/section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| PRG 00 | 0 | 122 | 8070 |
| PRG 01 | 0 | 342 | 7850 |
| PRG 02 | 0 | 135 | 8057 |
| PRG 03 | 0 | 1305 | 6887 |
| PRG 04 | 0 | 325 | 7867 |
| PRG 05 | 0 | 75 | 8117 |
| PRG 06 | 0 | 407 | 7785 |
| PRG 07 | 0 | 2195 | 5997 |
| PRG 08 | 0 | 363 | 7829 |
| PRG 09 | 0 | 3666 | 4526 |
| PRG 10 | 0 | 2544 | 5648 |
| PRG 11 | 0 | 190 | 8002 |
| PRG 12 | 0 | 844 | 7348 |
| PRG 13 | 0 | 168 | 8024 |
| PRG 14 | 22 | 0 | 8170 |
| PRG 15 | 395 | 1457 | 6340 |
| Header | 0 | 16 | 0 |
| CHR 0–31 (8KiB each) | 0 | 262144 | 0 |

All 26 regions carry Static provenance and evidence. Code: 417 bytes. Execution coverage: unavailable, zero observed ROM bytes. Structural counts are independent of execution coverage. Header/CHR boundaries follow validated iNES geometry; CHR is pattern-tile storage, not CPU code. Long exact uniform PRG runs classified generic data after checking vector-connected code, nearby transitions and references; specific purpose remains unresolved.

Baseline verified once at generated/baseline. Final checkpoint: [pass-004](../maps/pass-004.json), [generated source](../generated/pass-004/game.asm), byte-identical verification true, ROM SHA256 24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75, map SHA256 13e76833ad4c8350ca4fffcdbc6ac693aa5a100b19bcc07180a8df40db5c5e81. map.validate passed. pass-001 and pass-002 preserve intermediate empty-map checkpoints after guarded annotation retries; pass-003 is the earlier classified checkpoint. No checkpoints replaced.

Evidence: [fixed code review](round-01/fixed-walk.txt), [decoded instruction records](round-01/fixed-walk.json), [initial capability failure](round-01/initial.json), [uniform ranges and grouped code](round-01/classification-evidence.json), [bank14 reviewed prefixes](round-01/bank14-reviewed.json). State: analysis/state.json; portable map: maps/current.json. Input SHA256 unchanged.

Toolkit: one documented missing feature, mapper69 debugger admission blocks every gameplay scenario and artificial RAM shortcut; see [toolkit issues](toolkit-issues.md). No toolkit modifications. Static workaround preserves partial progress.

Concrete frontier: reset-established C117 RAM-clear routine at physical $1C127 stops at indirect store, and D394 at physical $1D3A4 calls C229. Need inspect complete callee contracts and bank writes before extending switchable-bank control flow. Reset loop E335-E345 selects banks 12/13/14 for 8000/A000/C000 after masking selector values 1C/1D/1E to physical bank indices. NMI calls C084/C1DB require interrupted effective context; do not assume reset banks universally. IRQ E2BD/E2C2 consumes table E2CA with selector $D3; selector domain and switchable target banks unresolved. No terminal exhaustion claim; this is an easy-gain handoff with actionable static leads.

## Round 1 commentary

Full current-source readability complete for pass-004: all 276715 classified bytes (417 code) and explanatory unknown-boundary notes reviewed, 49 inventory entries and eight routine/structure units. Source: commentary/source/game.asm; reading copy commentary/reading-after.asm. Fresh commentary/verification.json confirms byte_identical=true after final source edits (source SHA256 57e194928bdd7c0dc11095c3ca4109b769b187adcd4840fa8a086d19e6d14596). Map/state/input unchanged. Final editorial audit pending; whole-ROM classification still incomplete. Detailed scope/deferred questions: analysis/handoffs/round-01-commentary.md. TK-001 remains the one missing feature and gameplay blocker; no toolkit changes.

## Round 1 redirect — active static attempt

Attempt `analysis/redirect/round-01-static/`, input checkpoint pass-004, state unchanged. C117 loop and C229 mapper contract resolved; productive connected/independent investigation includes bank14 startup/services, bank10 sound, banks2/4/5 dispatch/gameplay, bank13 level setup, bank8 options and bank12 title. Primary decoded JSON and readable listings retained in attempt directory. Inline C0C6 carry-dependent return consumption corrected in later `*-reviewed`/`*-dispatch` records; earlier raw walks remain exploratory evidence only. IRQ secondary handler contracts and table boundaries being audited. Current frontier: audit bank4 84A0 BRK path against dispatch/data barriers, new D03C common helper, and stage/IRQ record consumers; prepare exact intervals and consolidated ready leads. No map annotations, no observed execution, no toolkit/input edits. Runtime remains unavailable TK-001.

### Round 1 redirect handoff ready

Completed `analysis/redirect/round-01-static/` coherent connected/independent static batch. Proposed23715 new code bytes +2100 data bytes (25815total), acceptance pending; accepted state unchanged276715classified/116517unknown. Independent disassembly should use review-code.json/review-data.json, contracts/findings.md and exact unknown-ledger.json. Persistent leads preserve histories and now include R01-LOOKUPS. Full handoff: analysis/handoffs/round-01-redirect.md. All final graph queues empty; indirect actor/type bank domains, sound stream grammar and RAM upload provenance remain explicit unresolved prerequisites, not terminal exhaustion. TK-001 runtime missing feature remains the sole toolkit finding and gameplay blocker. Input/state hashes unchanged; prior source/checkpoints preserved.


## Round 2 disassembly — one pass complete, incomplete ROM

ROM classification: 76.93% (302530 / 393232 bytes classified; 90702 bytes unknown).

| Physical bank/section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| PRG 00 | 0 | 122 | 8070 |
| PRG 01 | 0 | 342 | 7850 |
| PRG 02 | 5156 | 135 | 2901 |
| PRG 03 | 0 | 1305 | 6887 |
| PRG 04 | 5918 | 349 | 1925 |
| PRG 05 | 4013 | 89 | 4090 |
| PRG 06 | 0 | 407 | 7785 |
| PRG 07 | 0 | 2195 | 5997 |
| PRG 08 | 391 | 363 | 7438 |
| PRG 09 | 0 | 3666 | 4526 |
| PRG 10 | 1649 | 2560 | 3983 |
| PRG 11 | 0 | 190 | 8002 |
| PRG 12 | 435 | 846 | 6911 |
| PRG 13 | 2149 | 170 | 5873 |
| PRG 14 | 3586 | 1990 | 2616 |
| PRG 15 | 835 | 1509 | 5848 |
| Header | 0 | 16 | 0 |
| CHR 0–31 (8KiB each) | 0 | 262144 | 0 |

Independent review accepted23715 new code bytes and2100 new data bytes (25815 total). Total code24132bytes; all new evidence Static, zero Observed. All old regions preserved exactly. Fresh contextual decodes verified11036 proposal instructions over86 spans; six apparent direct-edge gaps independently resolve to four carry-clear literal words and two constant-branch barriers. Physical bank identities, mapper writes, inline literals, bounded handlers/records, palette values and arithmetic patterns checked in [review](round-02/review.md), [exact audit](round-02/decode-audit.json), [actual annotation plan](round-02/annotation-plan.json). Each annotation includes structural evidence and context; generic data retained only where precise schema remains unresolved after consumer inspection. Exact new unknown complement: [ledger](round-02/unknown-ledger.json).

Fresh final checkpoint [pass-006](../maps/pass-006.json), source [game.asm](../generated/pass-006/game.asm), [verification](pass-006-verification.json): map.validate passed and byte_identical=true. ROM SHA256 24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75; map SHA256 a991defbff43935b2bf0cce66e2d066199cb5cf5e04d4682f2f4d222fb241d66. State analysis/state.json and portable maps/current.json saved. Earlier pass-005 is an unchanged pass-004 duplicate preserved after an annotation request incorrectly put execution views on data; that rejected in-memory attempt did not save. Corrected requests use code-only views and describe data bank/CPU aliases in evidence. No toolkit defect inferred from this caller error. No checkpoint replaced.

Leads R01-CLEAR, R01-BANKHELPER, R01-IRQ and R01-LOOKUPS verified structurally for exact supported batch; natural selector/caller reachability remains separate. Remaining actor factory bank/type domains, sound secondary handlers/streams, bank4/5 dispatches and RAM upload origins require redirect. No terminal exhaustion asserted. Full current-source commentary is required for new accepted code; prior commentary covers pass-004 only.

Broad gameplay/ending/RAM shortcut checklist reviewed; all remain unavailable under unchanged TK-001 (mapper69 debugger admission). No unsupported scenario, execution, compatibility or natural reachability claim. [Toolkit issues](toolkit-issues.md) retains one missing feature; no toolkit edits. Input unchanged, no assets downloaded or published; evidence/checkpoints preserved.

## Round2 redirect — active static attempt

Attempt `analysis/redirect/round-02-static/`, unchanged input pass006 (302530 classified/90702 unknown). Readability gate complete. New endpoint/consumer method tests eight-actor and101-entry actor bank/target records, with per-record semantic graphs in physical banks0/8/11/12 and shared bank13 A000 helpers. Bank4 six charged/looking-up/ordinary factory targets and bank5 local state handlers under inspection. Sound explicit selector bounds exposed unsupported high suffixes reaching table bytes; these are excluded pending localized review. No map/source/state modifications, no unsupported runtime retry. Current exact frontier in plan.md, actor-records.json, graphs.json, continuation.json and active states.py. All findings Static/provisional; natural type/stream reachability separate. Investigation continues; no terminal/exhaustion claim.

Round2 redirect continuation: actor/state/shared-helper provisional audit yields34414 new code bytes,15194 instructions and zero instruction/self/data conflicts (audit.json). Bank3 actor tier/marker/312 twelve-byte records independently reconstructed (actor-data.json); all314 record pointers cover exactly312 records with two aliases, and producer types fit CE7F/CED5 domains (normal2..200 even, alternate2..16). Bank1/9 metasprite partitions consume exact7850/4526 unknown bytes as327/193 length-prefixed records; actual mapper-selected sprite targets independently align (sprite-partitions.json). Sound parser now covers all39 explicit sound-test commands, tonal and DMC grammar,11815 distinct consumed bytes with no unresolved parse errors. Intermediate failure artifacts preserved; corrected optional durations, first-half preset reread, single-slot return overwrite and no-return FA fallthrough (sound*.py/results). DMC sample register records identify seven fixed-bank15 sample starts E500/E900/ED00/F100/F500/F900/FB00 and exact lengths; residual alignment/tail bytes under pattern review. Uploaded scene/title origins now statically proven through NMI flush and copy counters: CHR1KiB banksF0..F7 orF8..FF ->RAM6000..7E5F (7776bytes), auxiliary F0-source1E60 ->RAM7E60..7FFF (416bytes); entries6000/6135 map to exact CHR identities, no observed execution or newCHR classification. Current frontier: package audited code/data, practical residual sound-envelope/actor lookup inspection and bounded boundary audit. No terminal assessment yet; state/map/source untouched.

### Round2 redirect combined handoff ready

Completed continuous `analysis/redirect/round-02-static/` investigation and final exact proposal audit. Accepted state remains302530classified/90702unknown, pass006; proposed38667code +52032data =90699bytes, acceptance pending. Exact proposal complement3bytes atphysical68546 length3, bank8CPU8BB2 isolatedJMP8C33; exact accepted complement90702 retained in attempt unknown-ledger.json with per-range proposal dispositions. Ready leads and histories saved. Final audit17175newinstructions, zero self/data overlaps or local direct-target gaps; input/map/source hashes unchanged. Full connected and independent results/reasons in findings.md and analysis/handoffs/round-02-redirect.md. All80 sound descriptor entries parse coherently; extraDMCF900 length1265 reachesFDF1 across acceptedFF. Upload origins metadata-only, no CHR split/code-view proposal, no observed RAM execution. One toolkit missing featureTK-001 unchanged; broad gameplay/ending unavailable. No terminal exhaustion, phase advancement, source/state/map/toolkit/input edits. Writer stopped after handoff; next disassembly is coordinator-owned.


## Round 3 disassembly — one pass complete, ROM incomplete

ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).

| Physical section | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| PRG 00 | 6548 | 1644 | 8192 | 0 |
| PRG 01 | 0 | 8192 | 8192 | 0 |
| PRG 02 | 7128 | 1064 | 8192 | 0 |
| PRG 03 | 0 | 8192 | 8192 | 0 |
| PRG 04 | 6830 | 1362 | 8192 | 0 |
| PRG 05 | 6728 | 1464 | 8192 | 0 |
| PRG 06 | 0 | 8192 | 8192 | 0 |
| PRG 07 | 0 | 8192 | 8192 | 0 |
| PRG 08 | 6788 | 1401 | 8189 | 3 |
| PRG 09 | 0 | 8192 | 8192 | 0 |
| PRG 10 | 2539 | 5653 | 8192 | 0 |
| PRG 11 | 7357 | 835 | 8192 | 0 |
| PRG 12 | 6525 | 1667 | 8192 | 0 |
| PRG 13 | 6451 | 1741 | 8192 | 0 |
| PRG 14 | 5070 | 3122 | 8192 | 0 |
| PRG 15 | 835 | 7357 | 8192 | 0 |
| Header | 0 | 16 | 16 | 0 |
| CHR 00–31, each | 0 | 8192 | 8192 | 0 |

Accepted38667 code +52032 data bytes after independent17175-instruction/145-span decode and bank-aware structural review. All old regions preserved,1852 added Static regions, zero Observed bytes. Primary sprite partition/actor extent checks independently recomputed; ten local direct fallthroughs resolve to nine C0C6 literal barriers and bank5 constant BEQ. Exact evidence and competing interpretations: analysis/round-03/review.md, decode-audit.json, primary-checks.json, annotation-plan.json. Total code62799bytes. New unknown-ledger.json retains exactly physical68546 length3 bank8 CPU8BB2, isolated4C338C. All16 PRG-bank word-reference search negative; lack of entry does not classify bytes. Rounded100.00% is incomplete with3 unknown.

Final checkpoint pass-008, maps/pass-008.json, generated/pass-008/game.asm, analysis/pass-008-verification.json confirms byte_identical=true after validation/save/export. Map SHA256 b18f67ed4a03000ed2ed9e79239b7fb166fd1ce8ba04a84eba4d365c86ff421a. ROM SHA256 unchanged24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75. pass-007 preserves an unchanged pass-006 duplicate caused by premature checkpoint sequencing while annotation was still in flight; no evidence overwritten. Final checkpoint awaited confirmed saved state. No toolkit bug inferred from caller sequencing error.

R02-ACTORS/FACTORIES/TILE-IRQ/SOUND/SPRITES verified structurally for exact reviewed batch; R02-UPLOAD-ORIGINS verified metadata-only, already classified CHR left intact. No uploaded views, RAM execution or natural selection claims. Full current-source commentary remains required for expanded code. One pass only; no commentary/redirect run. Residual is a nonterminal local ambiguity for subsequent reasoning review, no terminal exhaustion asserted.

Broad progression/ending/RAM shortcuts checklist reviewed, unavailable under unchanged TK-001 mapper69 debugger admission. No unchanged failed runtime experiment repeated. See [toolkit issues](toolkit-issues.md), one documented missing feature, no toolkit changes. Disk/evidence preserved, input unchanged, no publication/archive/deletion.

## Round3 redirect — localized review, incomplete recommendation

Pass008 remains393229classified/3Unknown, physical68546..68549 bank8CPU8BB2. Full current-source readability input gate met. Unique attempt analysis/redirect/round-03-contract-review contains changed-inference plan, fresh bounded primary decodes, exact-map complement and terminal rejection review. No proposed/accepted bytes, no ready leads. Combined local bank/working-state/actor-tail constraints favor veneer but do not discriminate bare JMP from opaque storage; no live-caller prerequisite imposed. New independent semantic result: phase6 reads accidental E020 IRQ continuation ending RTI, incompatible with ordinary actor RTS stack; no fourth handler inferred. R03-ISOLATED-JUMP/R03-PHASE6-IRQ and pass history appended, prior outcomes preserved. Independent frontiers and full unavailable progression checklist reviewed; TK-001 documented missing mapper69 debugger feature unchanged, no new toolkit issues. Input/map/state/source untouched, hashes recorded; no runtime retry, archive, publication, deletion or phase advance. Handoff analysis/handoffs/round-03-redirect.md requires coordinator rejection-review inspection before terminal incomplete acceptance. Disk4.8GiB free; no bulk scratch.


## Coordinator terminal handoff — workflow finished, ROM incomplete

ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown). Rounded percentage is not full classification. Final checkpoint pass-008; exact residual physical[68546,68549), bank8CPU8BB2, bytes4C338C, context[8,13,14,15]. Independent union count and configured store/map validation pass (coordinator-classification-audit.json, coordinator-map-validation.json).

Round3 commentary completed all1996 inventory intervals/1103 meaningful units with no backlog. After redirect, coordinator inspected the primary rejection evidence and accepted justified terminal incomplete (coordinator-rejection-review.md). Final targeted editorial follow-up passed the supported audit: byte_identical=true, editorial_audit=true; source f68c867fe59146ff32d7282a703f32b74a22e1d9124552d902fde9edf687a958, map/state b18f67ed4a03000ed2ed9e79239b7fb166fd1ce8ba04a84eba4d365c86ff421a. Reading copy refreshed. No ready leads, useful justified next method or interrupted phase remains. Future reassessment must preserve the exact frontier and historical evidence.

Final outcome/readability/verification/runtime limitations are separate in final-handoff.md. TK-001 mapper69 admission still prevents gameplay/boss/ending/RAM scenarios; no hardware compatibility or observed execution claimed. This is a terminal incomplete workflow result, not blocked ongoing work or whole-ROM success. All workers stopped.

Archive contract inspected and archive-options.json prepared with complete current Markdown accounting; final conclusions/handoffs selected, only duplicate historical instruction templates omitted from future selection and retained locally. Coordinator completion/verification records identify evidence per requirement. No toolkit/input changes, archive execution, cleanup, deletion, publication or commit. All source/assets/internal evidence remain local.


## Batch fallback disassembly — complete scoped pass

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

This map count includes 3 bytes of explicit user-directed Manual fallback, not new evidence-backed structural proof. The prior evidence-backed count remains 393229; the physical code-versus-data ambiguity remains unresolved. No full semantic or compatibility claim follows.

Scope/authorization: the user explicitly authorized this batch exception only after independently verifying the exact physical complement is at most 10 bytes. The byte-union audit found exactly [68546,68549), length3, physical PRG bank8, CPU alias $8BB2 under effective banks [8,13,14,15]. These bytes admit a JMP $8C33 veneer or opaque storage interpretation; the competing interpretations and prior rejection reasoning remain in analysis/final-handoff.md, analysis/coordinator-rejection-review.md and analysis/redirect/round-03-contract-review/findings.md. No new research, code proof, dead-code inference or natural-reachability claim was made.

Supported map.annotate added one unlabeled Data region, Manual provenance, with unknown meaning and explicit uncertainty. All prior regions are preserved exactly. Input checkpoint pass-008; output checkpoint batch-fallback-009. Before/after exact union audits: analysis/batch-fallback/before-union-audit.json and after-union-audit.json. Before:393229 classified/3 unknown; after:393232 classified/0 unknown, zero classified overlap. Totals:62799 Code,68289 Data,262144 Image. All16 physical PRG banks have8192 classified/0 unknown; header16/0; CHR262144/0. The sole gain is3 Manual Data bytes. Runtime observed bytes remain zero.

Configured /root/romatlas/target/release/romatlas validated the store, saved analysis/state.json, exported maps/current.json, then generated a fresh byte-identical checkpoint: maps/batch-fallback-009.json, generated/batch-fallback-009/game.asm, analysis/batch-fallback-009-verification.json. Store validation: analysis/batch-fallback/map-validation.json. Supported RPC transcript: analysis/batch-fallback/rpc-results.json. A rejected empty-label request made no mutation; corrected omission/null label succeeded, with rejected response preserved separately. No toolkit issue inferred.

Map/state SHA256:51e3a8458cdfa5eae69bd407bba5efb0ecf1666f8e1e04631790f1babe0fc1e9. Input SHA256 unchanged:24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75. Prior internal state preserved in analysis/batch-fallback/state-before.json; pass008 and all prior evidence retained.

One disassembly phase only; no commentary or redirect performed. No leads consumed or produced; authorization replaces research only for this exact fallback. Fresh commentary/editorial acceptance and reading-copy refresh remain coordinator-owned. Prior readable source remains preserved and must explain this Manual fallback without claiming structural certainty. Existing TK-001 mapper69 admission limitation remains in analysis/toolkit-issues.md; no new toolkit issues found. Gameplay, bosses, ending and RAM scenarios remain unavailable; no unchanged experiment repeated. Disk about4.8GiB free, scratch20KiB; no cleanup required or performed. No toolkit changes, archive, deletion, publication or commit. Coordinator will print the classification percentage before commentary.

## Batch coordinator final acceptance — complete under explicit exception

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

This continuation made authoritative progress: exact before-complement verified, exactly 3 Manual generic Data bytes added through supported API, state/maps saved and validated, fresh batch-fallback-009 checkpoint byte-identical. Prior 393229 evidence-backed bytes remain separate; physical [68546,68549) remains JMP-versus-opaque ambiguous with unknown meaning. Completion uses the user's explicit at-most-10-byte exception only, not a new structural inference. No redirect research resumed. Old terminal incomplete conclusions and all primary evidence remain preserved.

Sequential separate phase workers completed disassembly then full current-source commentary and final actual editorial review. Prior commentary tree retained in analysis/batch-fallback/commentary-history/pass-008. Current baseline/state/map/checkpoint agree; review inventory matches current map including Manual fallback. All 1996 intervals and 1103 meaningful units reviewed; no readability/editorial backlog. Fresh supported commentary audit passed byte_identical=true and editorial_audit=true; strict archive.editorial_status complete=true with no reasons. Expanded reading copy matches official renderer. Source SHA256 b8de523f562ebd370e21a029194ce0a1e2cdc11d748f2328b631616aa90e3de7. Map/state/checkpoint/baseline SHA256 51e3a8458cdfa5eae69bd407bba5efb0ecf1666f8e1e04631790f1babe0fc1e9. ROM SHA256 unchanged.

Final conclusions: analysis/batch-fallback/final-conclusions.md. Phase handoffs: analysis/handoffs/batch-fallback-disassembly.md and batch-fallback-commentary.md. Requirement-by-requirement completion audit: analysis/batch-fallback/completion-audit.md. Independent current-state checks: analysis/batch-fallback/final-verification.json and verify-final.py. Current readable source: commentary/source/game.asm; fresh reading copy: commentary/reading-after.asm.

Actual scripts/archive.py document-selection contract and strict editorial status inspected without archive execution. archive-options.json includes documents and omitted_documents plus final_fallback record. All 51 relevant recursive Markdown files accounted, zero unrecognized; only duplicate historical instruction templates omitted from future selection, retained locally. Final results and handoffs selected. All phase writers stopped. TK-001 remains an unavailable mapper69 runtime capability; no successful gameplay, boss, ending, RAM shortcut, observed execution or compatibility claim. No toolkit/input changes, archive execution, deletion, publication, cleanup or commit. This concludes the requested batch exception workflow; semantic/structural certainty for the 3 Manual bytes and runtime validation are not claimed.
