# Round 3 commentary — pass-003

**Complete current-source readability; ROM classification remains incomplete.** Separate redirect may now resume. This is neither final editorial/archive acceptance nor terminal research exhaustion.

Current review covers **1,856 classified regions plus all twelve unknown-gap notes (1,868 reviewed, zero pending)**. Retained valid prose/symbols/review credit for 1,206 unchanged physical sections; reviewed all 222 new Code sections (8,870 bytes), 428 new Data sections (37,716 bytes) and affected earlier dependencies. The full scaffold still partitions every physical ROM byte; 2,026 meaningful routine/structure units have current source locations and bank contexts where supported. [Scope](scope.json), [inventory consistency checks](inventory-check.json), [final frontier](final-frontier.json), [current review](../../commentary/review.json).

New source explains native actor and callback mechanics, finite entity type/state handlers, tracking/projectile/selection and line-of-sight paths, script operators and camera waits, counted simple/composite frames, palette attenuation, inline PPU modes, password bitfields and special-password records, node/map metadata, replay streams and DMC samples. Script slices have operation/operand notes for each unique parsed record; native calls use final entry names. Merged frames/descriptors include exact record offsets/counts. Native/control-flow/RAM/data symbols distinguish physical storage, bank-local CPU aliases, actor versus puzzle overlays and mapper register values. [Source](../../commentary/source/game.asm), [refreshed reading copy](../../commentary/reading-after.asm), [symbol inventory](symbols.json).

Concrete corrections include:

- Special-password matching resets Y=0F for every pointer candidate. Password accumulation retains ADC carry between bytes.
- CF0E writes ActorScriptWait[ActorScriptSlot]=FF without advancing the script cursor; the scheduler decrements the wait and reexecution reloads it. It does not deactivate a puzzle entity or change actor sprite/type.
- D77B operands are group, pointer low, pointer high. Raw FF palette input is permitted before attenuation/output clamping.
- D433/D439/D43F/D10B skip exactly two inline bytes. Mode 1 has destination,width,height,one repeated tile; mode 2 inherits destination. The 32x32 PPU2800 rectangle includes the attribute area.
- D7C8 bounded ordinary return requires start<=end<8000, end low byte !=FF and destination disjoint from working pointers, interpreter state and live CPU frames. Its literal high/low comparison does not prove universal termination.
- C119 begins with partial lower context and path-specific carry; C228 success calls bank-5 CPU8150 then SEC/RTS. Literal selector5 establishes effective PRG10/11.
- CF31 field0/1, actor0..7, operator0..2 supports bounded AND/OR/ADC tails; no parsed current command21 or natural use is asserted. Marker storage01C3..01CB avoids live frames under its finite contract.
- A536 tile-animation alias is retained. Selector71665 encodes ten but has only seven independently supported pointer words before script71681; full default-skip/domain/overlap remains unresolved. Composite placeholders do not instantiate every nominal frame slot.
- Connected callers identify the C591 helper as room-number drawing at PPU211C. The old uniform-data span129904..129937 contains the first byte of the register-bounded DMC sample at129936; its source note now localizes that dependency.

Uncertain game identities, original stream/asset names, generic patterned-data partitions and natural selection remain explicitly limited in source and [current uncertainties](../../commentary/uncertainties.md). Generic Data notes record the bounded consumer/neighbor inspection without inventing schemas. The twelve residual ranges (40 bytes) retain structural ambiguity, including D07D's same-byte low/high pointer candidate; unknown does not mean unused. These are redirect questions, not skipped readability.

Fresh configured-binary **verification passed: ok=true, byte_identical=true**, with no automatic checksum repair. [Verification](verification.json). Current source SHA256 `9cb507770898f551ac529e642d13702b02f5da1036f32a65e34fc3f4fd5bbfa7`. Reading-after refreshed after the final source changes. Manifest/baseline/state/current/pass003 identities match; map SHA256 `77e3a1d0900145aa22eeab2afdf67423bfd07b4b8585bd0fd2ba11ec86139d20`. ROM remains `59f5a7d9bd4f89875465ecb55a7ef20abd3417f4c6a72b66da50652adb021a58`.

Classification unchanged: **99.98% (262120/262160)**, Code25057/Data105991/Image131072, 40 unknown bytes in12 ranges. Source readability and fidelity do not establish whole-ROM classification, actual/natural reachability or full hardware/game compatibility. The final editorial audit remains pending and was not run.

Entire prior commentary remains frozen at [disassembly preservation](../disassembly-003/preserved-commentary/); the old active directory was moved intact to [prior-active](prior-active/) before fresh preparation. Source hash c47a0895d2e576321ce33bf333b1e18691791edc4716cdfa2e3ee3810bec506d was checked before transplant. Historical source/reviews/handoffs/evidence are retained.

Reused broad gameplay evidence through THE END; no replay or new runtime claim. Existing [NES-DOC-001](../toolkit-issues.md) remains one confirmed nonblocking documentation/API-contract issue; no new toolkit issues. No toolkit edit/rebuild, input ROM change, publication, archive or deletion/cleanup. All source/assets stay local; configured binary retained. Disk approximately8.9GiB free, scratch14MiB. No live debugger/server was started in this phase.
