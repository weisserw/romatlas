# Combined proposal review recipe

Input map: maps/round-001-disassembly-handoff.json, internal analysis/state.json. Accepted146708 bytes (9323 Code,137385 Data/Text), unknown246524. Commentary full current-source readability complete; final editorial audit pending. Research does not modify map, source or ROM.

Configured toolkit only: /root/romatlas/target/release/romatlas via run.py serve. Read api.describe and preserved protocol/API artifacts; current RPC uses `map.annotate` with a `region` object. Do not run classification in redirect.

## What the packet means

proposals-active.json contains284 role claims and321 physical unknown-only spans. Sum246524 proposed (10786 Code,235738 Data); existing annotations are subtracted. Each span points to its claim_index. Code claim.context stores CPU address at claim.offset; for a subspan use CPU address +subspan.offset-claim.offset, preserving effective8KiB context. unknown-ledger.json keeps exact original114-range complement and321 evidence-linked dispositions. No raw gap export is being used as classification evidence.

These are reviewable assertions, not automatically accepted coverage. Distinguish structural roles from natural reachability and selected artificial fetches. Static annotations suffice for supported roles; use Observed only when matching actual accepted instruction fetch artifacts. Runtime debugger writes and reached PCs alone prove no code.

## Primary constraint bundles

1. Speech command directory, sample descriptors and complete packed5-bit extents: speech-contract.json, sample-complete-extents.json, speech-full.json, speech-chr.json. All90 command records terminate at next origin;212 unique sample starts have consistent segmentation/outer counts; all nine overlapping starts are shared segment boundaries. Corrected runtime results establish first normal, bank-crossing and repeated/shared segment endpoints. Initial NMI-enabled mismatch is retained as failed setup, not inherited as sample rejection.
2. Physical bank separation: bank-footer-contract.json. Twelve bank-crossing source prefixes exactly duplicate32 bytes of next bank, followed by17FF, recovery code and vectors. Do not annotate a broad sample interval through the reserved footer. Mapper reset first5 bytes, independent IRQ3 and NMI1 are separate context claims. FINAL bankFFF6 converges to reset; local postfix differs.
3. Graphics: graphics-contract.json and accepted CA70/CCD5/C403 consumers. Encoded1090-byte directory,360 fully bounded objects with exact nextpointer endpoints. LastFF sentinel is not a sprite. Title panel table is three4-byte pointer/timeout records under bank0.
4. Movement/animations/callbacks: movement-contract.json, movement-callback-writers.json, movement-callbacks.json, movement-callback8282.json, movement-ends.json, animation-contract.json.36 movement roots tile1518 bytes; all10 direct callback writes target parsed record boundaries. Seven sets/34 frame lists abut exactly. Inline table barriers are explicit.
5. Sound: sound-contract.json plus accepted sound consumers.35 BE16 start/end pairs,1004 six-byte events, independent instrument-directory endpoint,9 pointers/7 unique streams with two-byte commands and FF termination. Semantic song identity is not required.
6. Game handlers and other fixed contracts: mode-targets.json, window-record-contract.json, windows-continuation.json, mode6-clock.json, fixed-gap-audit.json/txt. All six dispatch targets and their connected routines/tables investigated together. Full fixed gap audit localizes masks/lookup/remap/translation/feedback arrays before checking exact code endpoints.

## Boundary surgery is sometimes necessary

Unknown-only spans count new bytes; they are not universally independent annotation units. In particular dual-entry-boundary.json shows B911 `LDA#1; BIT$00A9` overlaps alternate B914 `LDA#0` inside BIT operands. Current accepted region startsB914. Coalesce/split surrounding Code under independent review to keep canonical five-byte decoding, prior Observed byte evidence and numeric/symbolic alternateB914 entry. Blind annotation of the unknown three-byte prefix would cut an instruction. Preserve history and original evidence when reorganizing regions.

Read sensitive-claims.md before acceptance. It localizes the first unselected timingNOP, generic32-byte post-sample tail, standalone helper entry preconditions and mapper-reset details. A local rejection does not reject independently supported neighboring bytes. Do not invent a semantic schema to avoid generic Data where non-code is clear.

After independent annotation: map.validate, source generation and fresh byte-identical verification using configured binary. Classification acceptance and current-source commentary remain separate; new/changed source requires full readability review before redirect. Do not export or publish private state/source/assets. Only supported ROM-free portable mapping is a sharing deliverable, under coordinator's later workflow.
