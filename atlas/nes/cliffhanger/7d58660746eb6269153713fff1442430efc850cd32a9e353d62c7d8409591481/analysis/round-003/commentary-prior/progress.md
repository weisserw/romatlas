# Round 001 commentary in progress

Checkpoint baseline: round-001-disassembly; classification unchanged at 169049/262160, unknown 93111. This phase is incomplete and must resume commentary, not redirect.

Completed coherent batch: explicit physical-boundary uncertainty notes for all 179 unknown gaps; CHR planar tile format note; global NES register and write-context MMC3 symbols. Sound bank6 classified intervals physical49168..53979 have per-block mechanical commentary, named music/effect entry points and handlers, named channel arrays/scratch pointers, and resolved bank-local branch/call symbols. Unclassified bytes within that envelope remain literal. Sound stream schemas, modulation countdowns, effect priority/ownership and dirty-register writes are explained without claiming original song/effect identities. Remaining numerical sound table addresses and address-based internal branch names require editorial follow-up.

Latest verification after this batch: commentary.py verify passed ok=true and byte_identical=true, ROM SHA256 7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481. Frozen before/ retained. Review scaffold is pending until physical/source locations are reconciled.

Exact substantive remaining frontier: all classified code intervals physical98320..131082 (banks12–15), 11,464 bytes; meaningful routine/control-flow interpretation and RAM/constant naming pending. Bank6 remaining editorial cleanup: branch role names, overlapping array aliases in initialization and table names. Generic data boundary notes already preserve bounded inspection rationale; header/vectors still need expanded field comments. Do not call this handoff complete.

No new toolkit issues. Existing NES-001 API mismatch remains nonblocking; analysis/toolkit-issues.md.

## Full classified-code prose batches complete

All 522 classified code fragments now have functional descriptions and descriptive entry names. Newly completed scope: banks12/13 player/actor modes, animation scripts, collision/trigger/HUD mechanics; bank14 terrain endpoint interpolation, world records, stage geometry and queued PPU strips; bank15 camera/scroll strip preparation, sprite grids, object workspace dispatcher, input, presentation selector and screen descriptors, timed Start waits and palette ramps. Structural classification remains unchanged. Unknown actor identities, terrain flag meanings, presentation token identities and unclassified handler continuations are localized explicitly. Source annotation working records: sound-notes.json, fixed-notes.json, geometry-notes.json, video-notes.json, actor-notes.json.

Latest verify after actor batch passes ok=true, byte_identical=true with unchanged input SHA256. Source game.asm SHA256 224d85747df65e87c1a7e9f74a3513eac278394ca617d3fc6908527878c89899.

Remaining frontier is now a consistency/editorial pass across ALL code, rather than unreviewed code ranges: symbolically express established RAM/workspace/field constants, resolve numeric branch/call aliases with effective physical bank context, inspect long multi-entry blocks for internal label gaps, reconcile review units/source locations, write uncertainties, then fresh verify and refresh reading copy. Address-only unknowns remain honest; this phase is not yet complete.

## Symbol consistency batch

Added workspace.inc with shared scratch-word, object workspace, player, camera, geometry, queued PPU, controller and HUD aliases. Source keeps CPU RAM separate from physical ROM. All6400 classified instructions were walked with syntax-derived lengths and checked against each original physical interval length. Effective same-bank/supported-pair and fixed-bank resolution replaced824 direct transfers with named targets; switchable calls from fixed-bank code remain unresolved unless their bank contract is independently explicit. Fresh verify passed byte-identical after the shared RAM batch and after branch resolution. Latest game.asm SHA256 929440a06f3d873091b848cc4b8cd7cf2a16723ddf1d2cf21c537c5343eb6630.

Remaining:33 direct target operands refer to unknown source or bank-dependent calls and require local uncertainty comments; actor field offsets/known table bases require final aliases. Reconcile all522 code entries plus data/image/unknown scaffold and compact functional units, then quality inspection, fresh verify/read, and final handoff. No source byte classification changed.

## Final source consistency batch

Added explicit31-byte actor field offsets, sound-array aliases, known table-base symbols (without classifying their surrounding unknown bytes), button masks, and supported sound/player bank-entry names. Renamed remaining address-only sound control-flow labels and113 generic branch aliases to their established local operations. Unresolved call destinations retain numeric CPU aliases with local bank/source uncertainty notes. Constant-data notes now state the exact uniform byte and avoid claiming padding/schema. Wrapped long prose for source reading.

Fresh verify after this coherent batch: ok=true, byte_identical=true; game.asm SHA256 a84a7260b72e2eb3d2b3e58ba6cd36ed298b1104e37917be4249fabba185701f. A transient undefined internal-entry alias during the field pass was corrected by adding labels at already classified starts; subsequent verification passed. No map/ROM changes.

Remaining final gate: source-location/scaffold/unit reconciliation, concise uncertainty record, representative consistency checks and final verify/read. Full current-code scope is explained; final editorial_audit stays pending as instructed.

## Round001 current-source readability complete

Completed all795 preserved scaffold entries, including522 code fragments totaling13,031 bytes,30 functional code groups, all classified generic data/header/vectors/CHR, and179 honest unknown-range explanations. No current-source readability range remains deferred. Source-local functional prose explains algorithms, state changes, entry/output contracts and bank constraints; unknown game meanings retain mechanical names and explicit limitations. review.json reconciles physical slices/source locations and functional units; current-scope.json and readability-checks.json record the scope checks.

Final fresh commentary.py verify: ok=true, byte_identical=true, unchanged ROM SHA256 7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481. Verification saved also in ../analysis/round-001-commentary-verification.json. commentary.py read refreshed reading-after.asm from maintained source. Maps/current.json and analysis/state.json remain JSON-identical to frozen baseline.json/phase1-state.json; checkpoint mapping copied to ../maps/round-001-commentary.json. Zero pending-semantics placeholders and zero address-only region/sound labels.

Current-source readability completion is separate from whole-ROM classification (169049 classified,93111 unknown) and final editorial/archive acceptance. editorial_audit remains pending and no final audit was invoked. Open meanings/questions are localized in uncertainties.md and compact handoff ../analysis/handoffs/round-001-commentary.md. No new toolkit findings; existing confirmed NES-001 remains nonblocking. Disk3.6GiB free at handoff; no deletion/archive/publication/toolkit edit.


## Round002 full current-source readability

Rebased from checkpoint `round-002-disassembly` after preserving the entire prior commentary tree at `analysis/round-002/commentary-prior/`, including its frozen before baseline, source, reading copies and reviews. Prepared the new baseline only after that preservation. Carried unchanged physical blocks and aliases; all13,031 prior code bytes remain verbatim. Existing useful prose and review credit remain intact.

Current scope:258,766 classified bytes,23,329 code bytes;3,394 unknown bytes retain explicit literal boundary notes. All89,717 newly classified bytes reviewed, including10,298 code bytes. Added209 functional/branch explanations,349 named local branch continuations and289 named data-record starts. All5,736 scaffold entries have refreshed source references; meaningful new units are clipped to their scaffold slices. No current-source readability ranges deferred. Whole-ROM classification and final editorial acceptance remain incomplete/separate.

New explanations cover all new domains: player phase/script/input/region and auxiliary-actor modes; terrain/proximity/attack/recoil/terminal actor states; four-slot child allocation and its returned record+14 pointer; special setup; random recurrence; world-list copying and tracking/cull continuations; centered text, packed score digits, synchronized presentation and the balanced PPU fill loop. Consistent RAM, record-field, hardware and routine operands preserve exact bytes. Data notes describe linked geometry, endpoint words, animation records, screen descriptors/text, palettes, world lists/profiles, dispatch domains and bounded generic storage without inventing game identities.

Concrete corrections during review: distinguish actor slot0 X from the saved player's first record; A304 scans from the type field, so its biased offsets refer to animation frame and Y-low;0651 is the preceding record's behavior field, not the fixed child at0652's flags. Preserve the unusual double Y-low store in the short recoil routine and the one's-complement distance arithmetic in the launch calculation. These are explanations of actual code, not ROM repairs.

Fresh verification after final operands passed `ok=true`, `byte_identical=true`; reading-after.asm refreshed. `analysis/round-002-commentary-verification.json` preserves the handoff verification, and `readability-checks.json` checks scaffold identity, source/unit locations, retained code and reading-copy freshness without invoking the final editorial audit. Final `editorial_audit` stays pending as required.

Unresolved meanings and structural barriers: dynamic callback9324; incoming pointer991D; D80F tracking-record matching prerequisite; exact encounter/enemy identities, some delay/table subformats and natural selection of static handlers; stage14's82 encoded terrain columns versus nominal96. No runtime replay, new reachability claim, speculative classification, toolkit change, archive or publishing was performed.
