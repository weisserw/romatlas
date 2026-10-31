# Round 1 current-source commentary — in progress

Prepared from verified maps/pass-001.json via the supported commentary.py prepare command. Frozen before/, baseline.json and phase1-state.json preserved. No structural reclassification, ROM changes, toolkit edits or final editorial audit.

Reviewed source so far: all 8,702 non-code/unknown scaffold records (4,100 Text,68 Data,1 Image,4,533 Unknown) and all939 Code fragments in physical PRG banks0,1,49,50,52,55,56,57,58,61,62,63, organized into384 connected mechanical units. Text has readable ASCII previews; data has finite format/boundary explanations reusing disassembly evidence; CHR has the two-plane tile layout; every unknown gap explicitly preserves code/data uncertainty. Generic data still has no invented consumer/schema.

Fixed-bank source now explains panel/text rendering, copy sizes for persistent/working records, PPU tile/attribute addressing and strip producers, sprite clipping, selection-mask navigation, mapper IRQ/NMI interaction, banked call gates and nonlocal stack contracts, script byte/word/reference operands, PPU update records/coalescing, palette fades, controller edges, arithmetic and audio descriptors. Stable RAM/hardware meanings are named in source/symbols.inc and fixed CPU service aliases in source/fixed-services.inc. Record A/B/C names denote distinct pointer tables/copy sizes; exact game identities remain unresolved. Source comments distinguish physical ranges, logical selectors and CPU aliases.

Latest coherent batch (after startup, party menu and action-selector review) verified byte-identical true with original ROM SHA-256 2d31a3d101fd629f8ff159de4ca15785444e4a3deb8b4fd7f114fa5cdef5ce87. See verification.json. An initial duplicate equate was corrected before this successful check; it was an editorial error, not a toolkit issue.

Exact remaining readability frontier:191 Code fragments in banks59,60. All remain pending in review.json; no current-source completion claim. Continue same commentary phase before redirect. No final audit yet. Reading copy will be refreshed at handoff.

Reusable additive source-edit scripts and unit descriptions: analysis/round1/commentary/code_units.py, apply_code_review.py and symbolize.py. Non-code annotation script was applied once; do not rerun it over edited source because it would duplicate notes. apply_code_review.py reuses current edited source, tracks bank-local CPU targets and asserts decoded byte counts; it never regenerates over source. Add remaining bank units only after reading their instructions, then apply and verify. It retains previous labels and reviews. Symbolize known RAM/hardware operands after new annotation batches.

Redirect questions retained for later phase: complete script operand/command extents and consumers; coherent later-area initialization tuple; banked callee/inline-dispatch effects. This is not authorization to run redirect before readability completion. No toolkit issues found.

## Final consistency frontier

All1,130 Code fragments now reviewed,496 meaningful units after adding49 routine boundaries internal to larger scaffold fragments. All9,832 scaffold entries reviewed; no pending native banks. Refined398 branch names, resolved caller targets when saved effective bank context selects one physical bank, and added stable area/actor/audio/VM symbols plus controller masks. Corrected the RNG explanation to state*$3D09-1 and renamed its helper MultiplyWordsLow16 after reading the shared multiplier contract.

Exact remaining work before a successful handoff: finish field/copy-length constants and prose spacing, refresh all source locations and check union coverage of unit fragments, record the bank55 primary byte/control-flow conflict with exact bounds, then fresh verification and reading-copy refresh. Current source still builds byte-identically; latest verification predates the last RNG prose/name correction. Continue this same commentary phase until these consistency steps pass. This note supersedes the older per-bank pending frontier above.
