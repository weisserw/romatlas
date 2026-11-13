# Round 001 commentary handoff

Status: current-source readability complete. Ready for coordinator review and redirect; ROM remains incomplete. No same-phase editing task or live process remains. Final editorial/archive audit remains pending and was not run.

Input checkpoint pass-001. Frozen map SHA256 de7e76a6020c20ccef358832942b43f6196edf82499e8ab77dcbddcd0bc83945; source baseline, current map and classification evidence preserved. Classification unchanged:17021/131088 bytes (12.98%), code10829/data6192/unknown114067. Readability is separate from classification and observed execution is separate from natural reachability.

Reviewed scope: ALL currently classified code in physical PRG banks0/12/13/14/15 and ALL classified data. Every unclassified interval also has an honest local unresolved-role note. Full5157-entry scaffold preserved with source links;235 meaningful units group routines/control/structures, with explicit classified subranges and bank context.383 descriptive branch aliases plus sound/main RAM/object-field, hardware and consumer-table symbols are consistent across the source. Physical identity and exact operand widths remain intact. Fixed CPU alias constants accompany physical *_Entry labels to satisfy per-instruction relocation sections.

Examples of substantive explanations:

- [UpdateSoundEngine](../commentary/source/game.asm:922): music/effect request dispatch and stream tick.
- [ScriptFindCollisionResponse](../commentary/source/game.asm:13087): bounds, response priorities and participant linkage.
- [DecodeResourceByte](../commentary/source/game.asm:25087): literal/repeated-run format and source cursor.
- [DrainVRAMWriteQueue](../commentary/source/game.asm:30258): bounded queue consumer, header stride bit and payload.
- [RenderScreenTextByIndex](../commentary/source/game.asm:8539): encoded text format and glyph consumer.

The source explains object VM frame/motion records and private stacks, collision/response mechanics, MMC1 serial writes and effective bank dependencies, stage setup and frame synchronization, HUD record producer/consumer format, incremental graphics staging, sprite allocation, PPU transfers, sound sequence/pattern nesting and effect-over-music channel priority. Data notes cover15-stage header/pointer/digit domains,32 text pointers, text controls/alphabet mapping, vectors/header and inspected generic uniform bytes. Generic data does not invent padding purpose. Numeric scratch registers and unknown callees remain where mechanical roles genuinely vary or paths are unclassified; source prose states those limits.

Unresolved meanings, all documented near consumers and in [uncertainties](../commentary/uncertainties.md): partial sound repeat paths, score tie/name-entry gaps, per-type health/count/coordinate semantics, specific fighting-move names, external controller-port output purpose and unknown helper contracts. Structural stream/resource extents, boss6/12 setup and natural clear/ending evidence remain reserved for redirect.

New redirect question R001-BRK-CONTEXT: physical0x1D19B / CPU D18B in fixed bank14 is an observed classified BRK and a camera-offset lookup base. Retained exactly; inspect the original accepted fetch predecessor/scenario to distinguish intended entry from artificial excursion. Indexed reads alone do not invalidate observed execution or justify reclassification. Persisted in analysis/leads.json with primary observation link. No redirect experiment or new classification performed during commentary.

Fresh [verification](../commentary/verification.json): ok=true, byte_identical=true; ROM SHA256 be2714f072338f2ca7204be750eaa03daadeb12521e1713cfe54d35b65457700. game.asm SHA256 c5b3798e82718eed19827dab9d47efd3ab9eed533b59a395f3a6ea9a3927ecc6. [Reading copy](../commentary/reading-after.asm) refreshed afterward. [Review](../commentary/review.json), [progress](../commentary/progress.md), [scope](commentary/readability-scope.json). Final editorial_audit remains pending; whole-ROM completion is not claimed.

Toolkit findings: existing one confirmed documentation/validation issue NES-001, zero suspected/missing-feature findings, no blocker. Full-array input workaround remains validated; no new toolkit issue during commentary. See [report](toolkit-issues.md). Configured binary used unchanged. Final disk6.5GiB free, scratch24KiB; no cleanup/archive/publication. All source/assets/input/evidence remain local.
