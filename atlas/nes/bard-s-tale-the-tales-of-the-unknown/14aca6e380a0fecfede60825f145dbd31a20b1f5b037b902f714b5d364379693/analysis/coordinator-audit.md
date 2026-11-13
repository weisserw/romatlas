# Coordinator audit

## Workflow requirements

- Run separate sequential disassembly, commentary, and redirect workers; resume unfinished phases.
- Print whole-physical-ROM classification percentage after every disassembly pass and in terminal handoff.
- Require full current-classified-source readability before redirect, with fresh byte-identical verification and reading copy.
- Continue productive redirect avenues; terminal incomplete requires coordinator review of rejection reasoning and decisive primary evidence.
- Distinguish workflow terminal status from whole-ROM classification and from resumable blockers.
- Preserve input ROM and toolkit, local evidence and history. No archive execution, publication, or cleanup authorized.

## Archive document-selection contract inspected

Read `/root/romatlas/scripts/archive.py`, FINAL and archive() document-selection logic (lines 16–26, 375–404). No archive command executed.

Default Markdown selection: analysis/investigation.md, analysis/gameplay.md, analysis/progress.md, commentary/progress.md, commentary/uncertainties.md, commentary/editorial-audit.md. The script also recognizes analysis/toolkit-issues.md and commentary/{prompt,workflow,AGENTS}.md specially. Files beneath path components before or source are excluded from the unrecognized-document check. Additional final Markdown must be explicit workspace-relative paths beneath analysis/ or commentary/; toolkit-issues.md cannot be an additional document. Omitted documents cannot overlap selected final artifacts. Every other Markdown under analysis/ and commentary/ must be explicitly selected or omitted.

At terminal handoff, inventory actual current Markdown and write archive-options.json with documents and omitted_documents arrays. Preserve conclusions, phase handoffs and investigation decisions; omit only established disposable scratch. Validate path existence, disjointness and complete accounting without invoking archive().

## Current orchestration

Round 1 disassembly worker `/root/round01_disassembly` started from initialized workspace. Verified baseline report: analysis/baseline-verification.json. Phase still running; no terminal assessment.

## Round 1 disassembly gate accepted

Inspected analysis/handoff-round01-disassembly.md and fresh analysis/round01-disassembly-verification.json. Independently checked current map equals checkpoint byte for byte, input ROM SHA256 matches workspace identity, and state regions are non-overlapping/in bounds. Counts recomputed: Code 17920, Data 10356, total classified 28276 / 131088 (21.57%); unknown 102812. Required percentage printed to user before starting separate round01_commentary worker. Handoff is easy-gain incomplete, not terminal exhaustion; no rejection-review acceptance claimed. Full commentary remains pending.

## Round 1 commentary gate accepted

Inspected analysis/handoff-round01-commentary.md, coverage.json and fresh verification. All704classified regions (692Code/12Data) and207unknown gap notes reviewed in intact911entry scaffold; inspected source example for useful routine/field/context/uncertainty prose. Independently checked source SHA256 matches final verification (069c4fb726bd1a71b5e64650094d65cf3047ed792a36599fc46620e098abac78), frozen map and state equal current files, every review entry has nonempty summary/source, and reading-after refreshed. No current classified readability gaps reported. Final editorial audit remains pending. Redirect must investigate corrected event consumer, generated RAM origins, exact840byte audio copy and graphics/container leads as a combined batch; no terminal assessment yet.

## Round 1 redirect handoff accepted for another round

Inspected full analysis/handoff-round01-redirect.md and summary.json. Combined48814byte proposal (8292Code,40522Data), accepted0, current102812Unknown remains unchanged. Independent map/checkpoint equality, source hash and ROM hash checks passed. Connected and independent event/map/template/graphics/audio/encounter/ending avenues packaged together; remaining indirect pointer writer/return-bank/control-write barriers have actual prerequisites, not mere independent-scope deferrals. Actual artificial boss/ending/credits/terminal evidence preserved; no natural victory claim. Ready proposals provide concrete next step; no terminal exhaustion or rejection-review acceptance claimed. Next round02 independent disassembly must validate exact ranges/contracts/provenance before annotation.

## Round 2 disassembly gate

Final checkpoint maps/round02-disassembly-complete.json and analysis/round02-disassembly-complete-verification.json inspected. Independent region bounds/nonoverlap and counts:26829Code+51938Data=78767/131088 (60.09%),52321Unknown. Current map equals final checkpoint byte for byte; ROM hash unchanged. Required percentage printed before next commentary. Review correction E7E0 localized to zero dependent proposals; original704regions preserved per pass review. ROM remains incomplete, no terminal exhaustion accepted. Await compact worker handoff, then full current-source commentary must preserve previous entire commentary and valid review credit.

## Round 2 commentary preservation checked

Independently checked every per-file SHA256 in analysis/round-02/commentary-preservation.json against the preserved prior tree. Prior source hash069c4fb726bd1a71b5e64650094d65cf3047ed792a36599fc46620e098abac78 matches round1final. Worker reports exact-boundary carry of704prior classified regions, fresh byte-identical verification; new scope remains pending, no commentary gate accepted yet.

## Provisional historical Markdown selection

Updated archive-options.json to preserve phase handoffs, investigation records, prior commentary progress/uncertainties and current coordinator evidence. Redundant generated AGENTS.md/prompt.md/workflow.md copies inside preserved prior commentary trees are marked omitted_documents; they are copied instruction scaffolding, not final results. No files deleted or archive run. Re-inventory actual Markdown before terminal handoff.

## Round 2 commentary gate accepted

Inspected full handoff, coverage and fresh final verification:1325classified regions/78767bytes and164unknown gaps,1489reviewed entries,411units, no reported readability gaps. Independently checked finalsourceSHA2567ed5020bc57dd7657ebd941720ebc06f7262ad4c5473abbec917167cc1761b77, current/frozen map and state equality, all review summaries/source links nonempty, reading-after refreshed after edits. Prior preservation25file manifest already verified. Final editorial acceptance remains pending; next separate redirect investigates residual dispatch/context/data questions, no terminal claim.

## Round 2 redirect gate accepted for next round

Inspected complete analysis/handoff-round02-redirect.md, deferred-frontiers.md and summary.json. Combined33581proposed bytes (6807Code/26774Data), accepted0. Independent current map/source/ROM hashes unchanged. Connected conditional RAM/panel/startup/summary/effect paths and independent data avenues packaged; deferrals identify localized pointer/destination/selector/return preconditions, not independent-scope exclusion. Ready leads justify round03 independent disassembly. Not terminal exhaustion; no rejection review accepted or final editorial claim.

## Round 3 disassembly gate accepted

Inspected compact handoff and fresh finalcheckpointverification. Independently recomputed33636Code+78712Data=112348/131088(85.70%),18740Unknown; checked bounds/nonoverlap,currentmap/checkpointequality,ROMhash. Requiredpercentage printed before commentary. All1325priorregions and commentary preserved per worker manifest; CB6D destinationcontract resolved without extra bytes, no terminalclaim. Fullcurrent-source readability required for new165Staticregions; previous gate covers oldsource only.

## Round 3 commentary preservation verified

Checked every filehash in analysis/round-03/commentary-preservation.json and prior source7ed5020bc57dd7657ebd941720ebc06f7262ad4c5473abbec917167cc1761b77. Prior tree retained intact; worker reports1325exactrangecarry and byte-identicalcarryverification. New165classifiedregions/156unknownnotes pending, no gateaccepted.

## Round 3 commentary gate accepted

Inspected full handoff/coverage/freshverification:1490classified regions/112348bytes plus156unknownnotes,1646reviewed entries/569units, no pending readability. Independent finalsourcehashd38771dc648e7feb91e608734ae43ddf4bc67ab4f1ca6f280828d9fad400d1cb checked with frozenmap/stateequality, nonemptyreviewrecords and refreshedreading timestamp. Priorpreservationmanifest verifiedearlier. CB6D uncertainty corrected; nextredirect must continue actualscoped pointer/return/contextfrontiers. Finaleditorialacceptance stillpending, no terminalclaim.

## Round 3 redirect intermediate reasoning review

Inspected compaction-retry.md and finite-control summaries in compaction-control-cases.json/all.json:438 and1512bounded immutable-instruction model cases passed. Corrected prior zero-count rejection: MutableCount>=RemainingPassCount>=1 at scan entries; last removal cannot induce another zero-count scan. Old objection was an untested local inference, not an actual full-algorithm counterexample. Positive bounded entry counts remain required; functional ordering is a stronger separate claim. Model does not prove natural reachability/hardware fidelity. Conditional wrapper artifact v3 has8verified/0unresolved (excluded from unconditional seeds); actualcaller predicates still apply. This is intermediate review, not terminal exhaustion acceptance. Worker continues localized return prerequisites before ready-batch handoff.

## Round 3 redirect ready-batch gate accepted

Read compact handoff and full findings, exact summary and runtime scope. Proposal10501bytes (8472Code,2029Data), accepted0. Independently checked unchanged map/source/ROM hashes. Connected inventory, service, record, companion/item, save, refresh and independent data avenues packaged; conditional domains explicitly preserved, corrected9097/count-counter/save-layout premises retained. Bounded initialized-combat test reached local enemy-defeat bookkeeping;23unknownfetchroots already proposed, no fullvictory/globalreturn claim. Remaining dynamic callbacks, mapper/nonrestoring contexts, state-changing attack/defeat and mixed-boundary prerequisites localized; this justifies another independent disassembly, not terminal exhaustion. Documented PRG-RAM debugger-write missing feature has a supported workaround and does not block progress.

## Round04 disassembly gate

Accepted checkpoint independently checked: 122849 / 131088 classified bytes (93.71%), 8239 unknown; sorted physical regions disjoint and in bounds. Current map equals round04 checkpoint. Original ROM hash unchanged. Fresh verification reports byte_identical=true and map SHA256 d9c0309c8498244467257fff3e319114080b4b9bf9422f4b33373560d52fe258. Read compact handoff: full10501-byte supported batch, localized conditional contracts and prior142-region addition; no terminal exhaustion claimed. Percentage printed to user before launching separate round04 commentary worker. Full current-source readability and fresh commentary verification now pending; final editorial audit remains reserved for terminal acceptance.

Round04 commentary preservation independently checked: all25 prior-tree artifacts match recorded hashes, including prior readable source. Provisional archive document selection refreshed and read-only contract check passes; final inventory remains pending after all phases. Live commentary worker confirmed running.

## Round04 commentary gate

Read handoff and independently verified1704 reviewed entries with source/summary, current map/state frozen baseline equality, source hash58419299aa63f2f2a94b54c7eec0640696bd176c6dd62134a5d586f6ca773901 against fresh byte-identical report, and reading-copy freshness. Coverage1632 classified regions plus72 Unknown gaps,711 units; all142 new regions covered and1490 prior regions carried. Cash offset operand correction and scoped predicates recorded. Full current-source readability accepted; final editorial acceptance remains pending. Launch separate redirect from this checkpoint, not terminal.

## Round04 redirect ready-batch gate

Read completed handoff, findings and local contract arguments;8236 proposed bytes(8036 Code/3540 starts,200 Data/7 pockets), accepted0. Exact partition/conflict audit and immutable ROM/map/state/source/configured binary hashes independently checked. Scoped callback bank/footprint proof, initialized record/group writes, no-empty-finish name protocol, saved pointer ABI and count-stable defeat claims remain local; natural reachability separate. Formation directory correction857F recorded for next commentary. Residual three-RTS rejection review read, but terminal assessment deferred until independent acceptance, full commentary and redirect. Separate round05 disassembly launched after redirect final completion.

## Round05 disassembly gate

Read verified handoff and independently checked sorted physical partition,131085/131088 classified,3 Unknown, current map/checkpoint equality and immutable ROM hash. Fresh byte-identical report mapSHA165f81bd4144e5ddb8712506c572218948bc6e214a3d87b968dea3d8c715a123 passes. All8236 reviewed bytes accepted,75newregions/1632old preserved; three isolatedRTS remain ambiguous. Required rounded100.00% printed to user before commentary with explicit incomplete warning. Separate full current-source commentary launched with mandatory857F correction and scoped contracts; final editorial/redirect/terminal rationale audit pending.

## Round05 commentary gate

Read full handoff; independently checked1710 reviewed entries with source/summary, current map/state frozen baseline equality, source hashc441ec249a4e203f3408b1f244949cdfc9162813ec059d2d2df9442bba14fb3d against fresh byte-identical report and reading-copy freshness.1707classifiedregions+3Unknown explained;851units, all75newregions covered, formation/character correction and source-local conditional contracts complete. Prior25-file preservation already independently checked. Separate round05redirect launched after worker final; final editorial and coordinator terminal reasoning audit remain pending.

## Terminal classification reasoning acceptance after round05 redirect

Inspected round05-r06 rejection-review.md and findings.md against decisive primary four-byte-primary-review.json, bounded residual-literal-pointer-check.json, noop-area-event-contract.json and original ROM neighborhoods. Physical00943D bytes4c849560a909 decode JMP9584 / RTS / LDA09;014E64 bytes4cc28c608db5 decode JMP8CC2 / RTS / STA03B5;01D284 bytes4cb5d560a91f decode JMPD5B5 / RTS / LDA1F. Effective contexts and exact singleton boundaries agree with current map. Unconditional predecessor transfers exclude fallthrough only, not independent entry. No direct target/encoded word in the reviewed graph or bounded adjacent immediate-pointer form anchors these three; absence is explicitly not universal unreachability or proof of Data.

Considered positive repeated JMP/RTS adjacency jointly with neighbor contracts: redundant emitted epilogue is plausible, but no independently established convention or required control/stack role selects it over a surplus separator/return byte. Treating a hypothetical ordinary return frame as a routine contract here merely restates RTS semantics. Generic Data also lacks positive local run/record/noncode evidence; three disjoint singleton60s are not one uniform extent. This is material code-versus-data ambiguity, not missing semantic names.

Checked acceptance consistency: D95B has actual alignedD0A3 cell6 word at118975 within accepted80-byte table, paired indexed loads into32/33 and computedJMP under event frame. That independent typed-consumer constraint supports static no-op code without natural selector observation; it does not extend to the other three bytes. No universal natural-caller prerequisite is being inherited. Current source/accepted neighboring contracts provide no corrected entry inference. Assessed avenues retain actual callback/bank proof, saved-pointer ABI, audio literal targets already classified, generated RAM versus ROM identities, explicit name/count-stability limits and broad artificial ending progression. No identified causal revised test addresses residual structure or material unresolved frontier; arbitrary PC/pointer sweeps or unchanged gameplay add no discriminating evidence.

Accept justified terminal INCOMPLETE classification at131085/131088 with3Unknown, rather than another unchanged research cycle. Orchestration remains active pending final editorial acceptance, fresh fidelity and archive document contract/inventory audit. Separate final editorial worker launched sequentially after redirect completed; no archive execution authorized.

## Final acceptance and completion audit

Final editorial handoff and actual targeted findings inspected; metadata repairs reuse existing source-backed contracts. Independently checked audit ok/byte_identical/editorial_audit,1710 reviewed regions/1761 reviewed localunits, all Code entries have units, every region source path/line resolves, source hash matches audit+verification and reading freshness. Configured binary store/map validation both returned ok. Frozen baseline/current state/map/checkpoint byte equality holds. Terminal rationale accepted separately;3Unknown remain. Final handoff records exact residuals, checkpoint, readability, fidelity, limitations, history and local-only/noarchive constraints. Re-read archive.py FINAL and selection/omission/unrecognized Markdown logic; final selection inventory checked after writing final documents. No required workflow phase or pending editorial work remains.
