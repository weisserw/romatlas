# Round 2 commentary — in progress

Previous commentary preserved in full at analysis/round-2/commentary-round01-preserved. Supported round2 preparation and exact physical-span/kind carry retained 380 reviewed code/data spans, 222 constants and generated-label aliases. See carry-report.json. Frozen baseline is round02-disassembly-reviewed-final, maphash 6b3b9c949a34093a1c23d94fcb4ba421c10049f3b5d8bf93946862397b5aa39f.

Completed batches: commentary-lower.py (28 added lower-bank regions), commentary-actor1.py (actor dispatch, upload records, initialization/phase/interaction family through physical $018B3B), commentary-actor2.py (persistent effects, composite scene, camera transition, projectiles and ballistic motion through physical $019819). Each batch passed fresh byte-identical verification. New functional names and purpose comments sit in maintained source, and shared RAM/hardware operands use carried constants. BE20 actor-frame code is corrected locally; broader Data neighbors remain Data. Old evidence/review credit remains preserved.

Exact active frontier: analysis/round-2/commentary-frontier.json lists every pending scaffold interval. Actor code starting $019858 and later, all added fixed-bank code, pending data/unknown explanations and carried dependency/source-location checks remain. Completed code has further branch/alias and naming consistency review pending; review status records the substantive range read, not final editorial acceptance. No readability-complete handoff yet. editorial_audit remains pending; no final audit invoked.

Actor3 and actor4a/4b applied and byte-identically verified; substantive actor review now through physical $01B2E6. Earlier progress frontier was superseded; no active tool/session. Pending scaffold: 236 total / 88 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor5a applied through physical $01B577; selector reservation and triplet sampler reviewed. Pending scaffold: 232 total / 84 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor5b applied through physical $01B827; shared direction, target and fixed-point screen-motion math reviewed. Pending scaffold: 231 total / 83 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor6 applied through physical $01BB6B; motion scripts and contact/hit responses reviewed. Pending scaffold: 230 total / 82 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor7a applied through physical $01BE04; contact windows and camera-edge placement reviewed; two missing actor6 entry labels corrected. Pending scaffold: 229 total / 81 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor bank substantive code complete through physical $01C00B; terrain placement searches and distance/contact helpers reviewed. Pending scaffold: 228 total / 80 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed1 applied: task delay/state APIs and title companion lifecycle reviewed. Pending scaffold: 223 total / 75 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed2 applied: audio bank restoration, queue clearing, stream handlers and bank6 service task reviewed. Pending scaffold: 219 total / 71 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed3 applied: stage metatile replacement, triggered object records and attribute-cache updates reviewed. Pending scaffold: 216 total / 68 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed4 applied: object and edge/current terrain-cache lookups reviewed. Pending scaffold: 214 total / 66 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed5 applied: layout lookup, flag clearing, byte negation and attribute-mode entries reviewed. Pending scaffold: 209 total / 61 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed6 applied: short mode setters/getters and split-entry boundaries reviewed. Pending scaffold: 204 total / 56 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed7 applied: reference helpers and auxiliary record-family selection reviewed. Pending scaffold: 202 total / 54 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed8 applied: terrain contact rules, layout classes, row offsets and flagged motion reversal reviewed. Pending scaffold: 201 total / 53 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed9 applied: upper terrain contact, primary/secondary stage lists and vertical cell-crossing tests reviewed. Pending scaffold: 197 total / 49 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed10 applied: shape-selected terrain probes, side/upper contact resolution and saved-position helpers reviewed. Pending scaffold: 195 total / 47 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed11 applied: contact-shape offset table consumer, terrain modification dispatch and stage threshold comparison reviewed. Pending scaffold: 194 total / 46 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed12 applied: packed-cache mutation, one-shot protection, edge snaps and direction offsets reviewed. Pending scaffold: 193 total / 45 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed13 applied: vertical crossing resolution, state40 phase reversal and upper terrain/layout snaps reviewed. Pending scaffold: 192 total / 44 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed14 applied: vertical delta, packed terrain-half selection and auxiliary records/frame sequences reviewed. Pending scaffold: 188 total / 40 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed15 applied: pause lifecycle, decimal score addition, reward thresholds and best-score update reviewed. Pending scaffold: 187 total / 39 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed16 applied: camera-page bounds, alternate coordinate subtraction entries and flag0 getter reviewed. Pending scaffold: 182 total / 34 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed17 applied: individual object flag APIs and auxiliary-object task dispatch reviewed. Pending scaffold: 177 total / 29 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed18 applied: player-attached offset streams and moving auxiliary sprite variants reviewed. Pending scaffold: 176 total / 28 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed19 applied: moving auxiliaries, terminal screen paths, fixed deltas and flashing sequence update reviewed. Pending scaffold: 174 total / 26 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed20 applied: counter exhaustion, graphics/audio wrappers, stage type remap and health dispatch reviewed. Pending scaffold: 170 total / 22 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed21 applied: scene-table counter initialization and graphics/health/palette/animation bank wrappers reviewed; two scene display wrappers retain explicit unresolved roles. Pending scaffold: 169 total / 21 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed22 applied: saved-scene reset continuation and split bank0 scene-step entries reviewed. Pending scaffold: 164 total / 16 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed23 applied: indexed scene uploads, initialization counters and stage-title display continuation reviewed. Pending scaffold: 161 total / 13 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed24 applied: full gameplay population task, grid/proximity/stream/type-bitfield spawn paths and maximum-health display reviewed. Pending scaffold: 160 total / 12 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed25 applied: camera spawn stream, population/slot allocation, grid cursor advance and saved-loadout initialization reviewed. Pending scaffold: 155 total / 7 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed26 applied: cache preload and transition tile walk, next-page reference, scene audio wait and scripted descent reviewed. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player1 source annotations applied for physical127700 CPUF2C4 through CPUF3B9; large spanning scaffold deliberately remains pending until all later blocks reviewed. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player2 source annotations applied through CPUF572; special terrain motion and jump initiation/recovery reviewed. Spanning region remains pending. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player3 source annotations applied through CPUF7D1; extra sprites and normal/special-health auxiliary attack animation families reviewed. Spanning region remains pending. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player3 duplicate internal label corrected (equipped special-health attack sequence12); fresh verification now passes. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Correction to preceding note: first label rename still failed because its branch retained the earlier duplicate name. Branch target now corrected; actual fresh byte-identical verification passed with source hash1de047579307fce1aad8a8cd7ca36567c2d9d77c2952760c9d10007408d2f179. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player4 source annotations applied through CPUFAA8; grounded/submode animation, terrain upgrade and selector-specific attack dispatch reviewed. Spanning region remains pending. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player5 applied through CPUFB95; large player-state region now substantively reviewed across all five saved source batches, including removal animation. Three added code regions remain. Pending scaffold: 151 total / 3 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Camera1 source annotations applied through CPUFD58; scene/cache transitions and scripted exit reviewed. Spanning camera region remains pending. Pending scaffold: 151 total / 3 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Camera2 applied through CPUFEC1; camera mode scroll/clamps and initial stream population reviewed. Two added code regions remain. Pending scaffold: 150 total / 2 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed27 applied: final added code reviewed through physical $01FFD2. All added Code scaffold now reviewed; full data/unknown source and naming/dependency/location checks still pending. Pending scaffold: 148 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Existing terrain backing source notes reviewed against preserved candidate samples and stage/scroll consumers; retained useful prose and explicit schema uncertainty for27 pending spans. Pending scaffold: 121 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Four audio/sprite backing containers now have readable format/consumer and overlap-boundary notes plus descriptive aliases; fresh byte-identical verification passed. Pending scaffold: 117 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

First five residual unknown ranges now have local boundary/competing-role notes based on preserved independent structural assessment; classification unchanged. Pending scaffold: 112 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

All nine residual unknown intervals now explained locally without decoding/reclassification; fresh byte-identical verification passed. Pending scaffold: 108 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Context-pressure interruption after complete added-code substantive review,31 data-container/terrain reviews and all9 unknown notes. Exact continuation saved in analysis/round-2/commentary-continuation-code-complete.md;108 Data plus full consistency/location gate remain. Not a handoff. Pending scaffold: 108 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Data batch A: banner/attribute/health consumers and unresolved three-byte boundary inspected. Pending scaffold: 104. Naming, dependencies and locations remain pending; same phase continues.

Data batch B: alternate health artwork and lower triplet backing inspected against neighbors and population consumers. Pending scaffold: 99. Naming, dependencies and locations remain pending; same phase continues.

Data batch C: five lower sparse/dense triplet containers reviewed; indirect consumer limits documented. Pending scaffold: 94. Naming, dependencies and locations remain pending; same phase continues.

Data batch D: remaining lower triplets, four-frame animation pointers and coordinate-like tail inspected. Pending scaffold: 89. Naming, dependencies and locations remain pending; same phase continues.

Data batch E: scene graphics, rewards, audio requests and HUD source consumers reviewed. Pending scaffold: 85. Naming, dependencies and locations remain pending; same phase continues.

Data batch F: palette, display scripts, graphics banks and overlapping lookup envelopes reviewed. Pending scaffold: 81. Naming, dependencies and locations remain pending; same phase continues.

Data batch G: actor dispatch bounds and explicit graphics record consumers confirmed. Pending scaffold: 76. Naming, dependencies and locations remain pending; same phase continues.

Data batch H: final explicit upload records and first phase-selected sprite/actor tables reviewed. Pending scaffold: 71. Naming, dependencies and locations remain pending; same phase continues.

Data batch I: three/four-phase selector bounds checked; projected-effect attribute interpretation corrected. Pending scaffold: 67. Naming, dependencies and locations remain pending; same phase continues.

Data batch J: remaining phase replacement tables and three-byte motion descriptors reviewed. Pending scaffold: 62. Naming, dependencies and locations remain pending; same phase continues.

Data batch K: descriptors, saved-field lookups and secondary dispatch word consumers reviewed. Pending scaffold: 58. Naming, dependencies and locations remain pending; same phase continues.

Data batch L: score indices, transform masks, spawn choices and scene-progress record backing reviewed. Pending scaffold: 53. Naming, dependencies and locations remain pending; same phase continues.

Data batch M: projectile countdowns, transition offsets, ballistic lookup and composite-pair records reviewed. Pending scaffold: 48. Naming, dependencies and locations remain pending; same phase continues.

Data batch N: companion streams, composite uploads and stage-selected positions reviewed. Pending scaffold: 43. Naming, dependencies and locations remain pending; same phase continues.

Data batch O: twin offsets, graphics record layouts and packed motion backing reviewed. Pending scaffold: 38. Naming, dependencies and locations remain pending; same phase continues.

Data batch P: low-counter/curve dispatch tables and signed curve lookup reviewed. Pending scaffold: 33. Naming, dependencies and locations remain pending; same phase continues.

Data batch Q: terrain/phase dispatch, turning matrix and selector reservation tables reviewed. Pending scaffold: 28. Naming, dependencies and locations remain pending; same phase continues.

Data batch R: selector/motion/response structures and actor-bank tail reviewed; fixed-bank reference-index ambiguity kept explicit. Pending scaffold: 23. Naming, dependencies and locations remain pending; same phase continues.

Data batch S: mapper selection constants, task entry tables, title boundary and audio dispatcher reviewed. Pending scaffold: 18. Naming, dependencies and locations remain pending; same phase continues.

Data batch T: stage descriptors, RAM cache pointers, terrain lists and signed contact offsets reviewed. Pending scaffold: 13. Naming, dependencies and locations remain pending; same phase continues.

Data batch U: terrain thresholds/edge offsets and auxiliary dispatch/attribute choices reviewed. Pending scaffold: 8. Naming, dependencies and locations remain pending; same phase continues.

Data batch V: auxiliary tables, scene graphics script, maximum-health tile and population padding reviewed. Pending scaffold: 4. Naming, dependencies and locations remain pending; same phase continues.

Data batch W: player/tail padding and both fixed graphics scripts reviewed. All added Data scaffold now substantively covered. Pending scaffold: 0. Naming, dependencies and locations remain pending; same phase continues.

Consistency milestone: all 52 numeric conditional-branch operands now use purpose labels, preserving physical identities. Shared actor type/parameter, layout/contact/cache fields, input/phase latches, saved population selectors/cursors and score thresholds now have operand symbols; context-specific terrain probe scratch avoids the misleading graphics-count name. Fresh byte-identical verification passed after the main field batch. Wrapper/alias, stale prose and location/dependency checks continue; no completion handoff yet.

Dependency correction: fixed-bank consumer context resolved player preset/attribute and special auxiliary tables; misleading generic or cross-bank associations replaced. Pending scaffold: 0. Naming, dependencies and locations remain pending; same phase continues.

Dependency enrichment: graphics-bank population bitfield/page-marker lookups and bank-two stage spawn cursor tables now explained at their actual backing containers. Pending scaffold: 0. Naming, dependencies and locations remain pending; same phase continues.

Source consistency milestone: meaningful names now replace every raw RAM/ROM operand and numeric/unresolved direct call/branch in the maintained listing. Added compact named table units and corrected biased cache lookup bases, fixed-bank data associations and stale round-one claims about newly classified continuations. The E702/E710/E722/E728 wrapper prose now correctly follows select_bank6; this corrects commentary context, not classification or execution views. Timer increment label moved to its true B7FA entry; B72F is named as the ungated decrement entry. Prior aliases/evidence remain preserved. Latest verification passed byte-identically; final locations, unit contexts and uncertainty record refresh remain underway.

Carried-data dependency check: bank-zero type remap, bank-one decimal score addends and bank-three counter lookup views explained at their actual containers. Pending scaffold: 0. Naming, dependencies and locations remain pending; same phase continues.

Round-two current-source readability complete. The remaining 108 Data intervals, branch/call/field/table naming, carried dependencies and stale continuation prose have been reconciled. All 698 scaffold regions and 473 meaningful units are reviewed; every source location is refreshed, valid physical ranges/kinds checked, and code bank contexts retained. Bank-six saved-selector wrapper and banked table corrections change commentary only; no accepted structural premise was invalidated. Exact scaffold frontier is empty. Fresh supported byte-identical verification and reading-copy refresh passed; final hashes and honest semantic limits are recorded in analysis/round02-commentary-handoff.md and analysis/round-2/commentary-consistency-final.json. No known current-source readability gap remains; nine Unknown intervals remain explained Unknown. Final editorial audit/archive acceptance remains pending with the coordinator; no final audit invoked.
