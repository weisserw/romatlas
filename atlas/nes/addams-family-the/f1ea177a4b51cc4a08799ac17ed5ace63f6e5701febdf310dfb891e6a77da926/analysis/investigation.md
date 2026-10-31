# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 final checkpoint and outcome

ROM classification: 55.88% (146501 / 262160 bytes classified; 115659 bytes unknown).

One disassembly pass completed, ROM incomplete. Current phase handoff: analysis/handoffs/round-001-disassembly.md; next full current-source commentary then redirect. No live server. Earlier active server references below/above are historical and superseded. Final internal state analysis/state.json, portable maps/current.json. Checkpoint pass-001-verified: valid and byte-identical, report analysis/pass-001-verified-verification.json, generated/pass-001-verified. Map SHA256 d9865903bf1c965ce68368ef12bb243fa04dcb03c7a6afaae1577c7b30dc3c5f. ROM hash unchanged.

Code14089, Data1340, Image131072;489regions,146501evidence-annotated bytes. Execution metric separately: persisted union3685physical ROM opcode starts,1RAM start; partial saved aggregate evidence, not whole-run cumulative coverage. Detailed count/unknown complement round-001/classification-summary.json. Source readability/editorial review pending; no whole-ROM completion claim.

Gameplay: natural garden; artificial crypt/hallway/pond/river/Gomez/roof/freezer and congratulations terminal. Exact scripts, patches, snapshots and milestones in scenarios.json/gameplay.md. Broad batch10174newcodebytes, ending62, roof/freezer271; static pointer/ordinal-data follow-up101bytes. Further advancement requires mapper/indirect/data role investigation outlined in handoff, not repetition of unchanged routes. All failed/no-op and interrupted experiments preserved with limits. No prior leads existed; local startup/reload/ending leads verified. No terminal exhaustion claim.

Toolkit report: analysis/toolkit-issues.md has one confirmed documentation/validation issue NES-001, no suspected/missing-feature entries and no blockers. Workaround buttons array. Toolkit unchanged.

### Per physical 8 KiB bank

| Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
|header|0|16|0|0|
|PRG-00|817|0|0|7375|
|PRG-01|397|64|0|7731|
|PRG-02|1473|0|0|6719|
|PRG-03|926|0|0|7266|
|PRG-04|474|175|0|7543|
|PRG-05|1683|0|0|6509|
|PRG-06|0|0|0|8192|
|PRG-07|0|298|0|7894|
|PRG-08|0|64|0|8128|
|PRG-09|0|0|0|8192|
|PRG-10|0|393|0|7799|
|PRG-11|0|223|0|7969|
|PRG-12|0|0|0|8192|
|PRG-13|0|0|0|8192|
|PRG-14|4671|101|0|3420|
|PRG-15|3648|6|0|4538|
|CHR-00|0|0|8192|0|
|CHR-01|0|0|8192|0|
|CHR-02|0|0|8192|0|
|CHR-03|0|0|8192|0|
|CHR-04|0|0|8192|0|
|CHR-05|0|0|8192|0|
|CHR-06|0|0|8192|0|
|CHR-07|0|0|8192|0|
|CHR-08|0|0|8192|0|
|CHR-09|0|0|8192|0|
|CHR-10|0|0|8192|0|
|CHR-11|0|0|8192|0|
|CHR-12|0|0|8192|0|
|CHR-13|0|0|8192|0|
|CHR-14|0|0|8192|0|
|CHR-15|0|0|8192|0|

Concrete next investigations and primary artifacts: handoffs/round-001-disassembly.md. Remaining table target reachability and bank contracts are open, not rejected. Generic data decisions depend on actual non-code patterns/bounds, not nonexecution.


## Round 2 disassembly outcome

ROM classification: 88.63% (232360 / 262160 bytes classified; 29800 bytes unknown).

Incomplete. Added85859 unique Static bytes:13714Code and72145Data. Seven copied bootstrap trailing RTS bytes rejected narrowly for unresolved post-write return context. All other proposed claims accepted after independent review; bootstrap views split by instruction and local contracts. Review: round-002/review-notes.md, independent-audit.json, accepted-proposals.json and rejected-proposals.json. Exact residual complement and counts in round-002. No unchanged gameplay replay; broad milestones/ending evidence reused. No observed provenance or execution-coverage gain claimed; prior saved3685 physical ROM starts/1RAM start remains separate partial evidence.

Fresh final checkpoint pass-002-final valid and byte-identical; verification analysis/pass-002-final-verification.json, generated/pass-002-final. Earlier pass-002-verified and pass-002-reviewed are unchanged-input fidelity attempts created after failed unsaved annotation attempts; preserved, superseded, not final output. API optional empty labels corrected by omission; no toolkit defect inferred. Final map SHA 9810a02b85ad2ffa5ad2075708c642b9273cf5d042bbf6914c9ce84ca993cd7c. ROM unchanged. Prior commentary untouched; full new current-source commentary required.

Toolkit issues: existing NES-001 confirmed documentation/validation issue, workaround buttons array; no new findings, no blockers; analysis/toolkit-issues.md. No toolkit edits.

| Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
|CHR-00|0|0|8192|0|
|CHR-01|0|0|8192|0|
|CHR-02|0|0|8192|0|
|CHR-03|0|0|8192|0|
|CHR-04|0|0|8192|0|
|CHR-05|0|0|8192|0|
|CHR-06|0|0|8192|0|
|CHR-07|0|0|8192|0|
|CHR-08|0|0|8192|0|
|CHR-09|0|0|8192|0|
|CHR-10|0|0|8192|0|
|CHR-11|0|0|8192|0|
|CHR-12|0|0|8192|0|
|CHR-13|0|0|8192|0|
|CHR-14|0|0|8192|0|
|CHR-15|0|0|8192|0|
|PRG-00|1007|7171|0|14|
|PRG-01|397|3094|0|4701|
|PRG-02|6963|636|0|593|
|PRG-03|3545|1566|0|3081|
|PRG-04|2703|1943|0|3546|
|PRG-05|2151|5750|0|291|
|PRG-06|151|8040|0|1|
|PRG-07|0|8192|0|0|
|PRG-08|249|64|0|7879|
|PRG-09|0|6|0|8186|
|PRG-10|151|8040|0|1|
|PRG-11|0|8192|0|0|
|PRG-12|151|8039|0|2|
|PRG-13|0|8192|0|0|
|PRG-14|6016|923|0|1253|
|PRG-15|4319|3621|0|252|
|header|0|16|0|0|

## Round002 redirect — combined ready research batch
Input pass-002-final, accepted232360classified/29800Unknown unchanged. Whole-current-source/physical complement review localized consumer-backed parameter/HUD/presentation data, complete local routine contracts, mixed dictionaries versus relocated old fixed-window code image, and retained-high mode2 bootstrap serializer returns. No gameplay replay or toolkit edit. Final candidates29786=6669Code+23117Data; accepted0 this phase. Native3131official instructions/32physical-context runs; candidate subset/no-overlap/source identities pass. See analysis/redirect/round-002-attempt-001/findings.md, audit.json/proposals.json, rejection-review.md and exact29800-byte unknown-complement-dispositions.json. If all independently accepted,14bytes remain in7intervals; their local primary-evidence ambiguity review is preserved, not speculative classification. Leads histories appended; ready combined batch for round3 independent disassembly then full source commentary. This is a research handoff, not ROM completion or terminal exhaustion.


## Round 3 disassembly outcome

ROM classification: 99.99% (262146 / 262160 bytes classified; 14 bytes unknown).

Incomplete. Independently accepted29786 Static bytes:6669Code/23117Data; all prior classifications preserved. Primary mapper/consumer/native/physical/pattern review in analysis/round-003/review-notes.md and independent-audit.json. Exact14bytes/7intervals remain ambiguous:3995+4,4228+1,680E+3,6B95+3,80C2+1,A706+1,180A8+1. No natural/Observed claims added; saved partial execution metric3685 ROM opcode starts/1RAM start unchanged. No unchanged gameplay replay; later milestone/mechanics/ending evidence reused. No useful new discriminator from accepted partition; residual primary review handed to commentary/redirect, no terminal workflow exhaustion declaration.

Checkpoint pass-003-verified map.validate passed, fresh generated assembly byte-identical; analysis/pass-003-verified-verification.json. Map/state SHA 887095479d741da9bd20fcf6763e646dd005010cf9442803f462feafabf8b2f3. ROM unchanged. Classified evidence annotation coverage 262146/262146 bytes; source commentary/editorial gate pending for enlarged source. Prior commentary source SHA3fcceaca3f9532ac32ede3cf446ae6d726fd06001ea83b06b8a61840d1671b6a untouched. No live handles.

Toolkit: existing NES-001 confirmed documentation/input validation issue, buttons-array workaround; no blockers or new findings, no toolkit edits. Report analysis/toolkit-issues.md.

| Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
|CHR-00|0|0|8192|0|
|CHR-01|0|0|8192|0|
|CHR-02|0|0|8192|0|
|CHR-03|0|0|8192|0|
|CHR-04|0|0|8192|0|
|CHR-05|0|0|8192|0|
|CHR-06|0|0|8192|0|
|CHR-07|0|0|8192|0|
|CHR-08|0|0|8192|0|
|CHR-09|0|0|8192|0|
|CHR-10|0|0|8192|0|
|CHR-11|0|0|8192|0|
|CHR-12|0|0|8192|0|
|CHR-13|0|0|8192|0|
|CHR-14|0|0|8192|0|
|CHR-15|0|0|8192|0|
|PRG-00|1008|7184|0|0|
|PRG-01|720|7468|0|4|
|PRG-02|6987|1204|0|1|
|PRG-03|3545|4641|0|6|
|PRG-04|2704|5487|0|1|
|PRG-05|2151|6040|0|1|
|PRG-06|152|8040|0|0|
|PRG-07|0|8192|0|0|
|PRG-08|2143|6049|0|0|
|PRG-09|4321|3871|0|0|
|PRG-10|152|8040|0|0|
|PRG-11|0|8192|0|0|
|PRG-12|152|8039|0|1|
|PRG-13|0|8192|0|0|
|PRG-14|6112|2080|0|0|
|PRG-15|4325|3867|0|0|
|header|0|16|0|0|


## Round003 redirect — exact14-byte remainder
Research attempt analysis/redirect/round-003-attempt-001, input pass-003-verified, proposed0. Revisited original primary rejections and current accepted-copy standards; narrowed3995 firstRTS versus3996store/return independently. Nearby3-byte latch suffix and renderer triple motifs remain suggestive but do not discriminate structural membership. Duplicate-return bounds and180A8copy exclusion are localized, not opposite-role proofs. Combined manifests/current source expose only apparent audio indexed route; CMP06/BCS/TAY bounds exclude that route, not global references/data role. Source-wide missingcallee/dispatcher/bank/copy/container opportunities and already visited later/ending scenarios checked without unchanged experiments. Exact ledgers, source excerpts, review script/results, hashes and perbank counts persisted. No useful current revised test; coordinator inspect rejection-review.md before terminal assessment. Stale game.asm:56974 pointer-table uncertainty recorded for editorial correction, source untouched. Conditional leadR3-RESIDUAL-MEMBERSHIP appended; no ready leads, no live handles or unfinished research. See analysis/handoffs/round-003-redirect.md.
