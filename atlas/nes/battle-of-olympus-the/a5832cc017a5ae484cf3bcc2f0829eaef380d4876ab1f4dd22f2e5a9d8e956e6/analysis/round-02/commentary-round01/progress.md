# Commentary progress

Round 01 commentary in progress, baseline pass-001-final. Frozen before/ and original evidence preserved. Initial preparation byte-identical.

Current inventory: 3,742 scaffold slices, many single instructions, covering 9,625 code bytes and 32,229 data bytes; 89,234 unknown bytes. Review is grouped into coherent consumers/routines while preserving exact physical scaffold.

Source inspection underway: bank0 actor motion/collision and animation, bank1 actor state handlers, bank13 terminal scene rendering, fixed-bank reset/game loop/player physics and area/graphics decode. Edits and review credits pending. Remaining: finish fixed-bank inspection; apply bank-aware names, shared RAM/hardware symbols and meaningful block/format prose across all current source; verify and refresh reading copy. No completion claimed.

## Authored code review frontier

Full classified code inspected through final audio engine/reset stub. Routine prose plan retained in scratch/commentary_edit.py; it contains bank-aware names, algorithm/input/output/state notes and localized gap uncertainty for all current code groups. Source transformation pending boundary checks. Covered inspection: physical 00010..0392B (bank0/1 actors), 1B418..1B865 (bank13 scenes), 1C012..1FFEF (fixed-bank classified slices). Remaining: validate semantic entry addresses against exact instruction starts, inject names/prose, shared operand symbols, data and unknown boundary notes, review scaffold credits and final fresh verification/reading refresh. No success or redirect handoff yet.

## First complete source transformation

All classified instruction slices now carry 342 authored routine/block explanations and 794 bank-aware routine/branch aliases. Every data/unknown incbin slice has local format/boundary prose; shared RAM, actor/OAM layout, hardware and decoder constants are in source/symbols.inc. Explicit operand widths and original storage ordering retained. First coherent rebuild passed byte_identical=true (verification.json). Follow-up corrected count-result flags and following-slot actor-field wording; validating these corrections and review credit next. Remaining: review scaffold link/coverage completion, uncertainty record, concrete residual scan, final verify and reading refresh.

## Round 01 completed current-source handoff

Current-source readability complete: all 3,742 frozen scaffold entries reviewed, 342 authored routine/control-block units, 183 data structures and 284 unknown slices with local source explanations. All 9,625 code bytes / 4,849 classified instruction starts and 32,229 data bytes are covered; there are no remaining current-source readability ranges. 89,234 unknown bytes remain structurally unknown. Source includes 793 routine/branch aliases, context-specific scratch aliases, RAM/hardware names, actor fields, constants and consumer-named ROM table aliases. Four cross-bank calls resolve from their saved effective PRG views, including the rescue and actor-bank-zero calls; no fixed-bank identity is inferred for their switchable targets.

Examples: signed 4.4 player/actor integration, collision geometry and tile probes; twelve sixteen-byte actor records and staged OAM composition; inline return-address dispatch and stack-unwind exits; area descriptors, parallel entrance fields and dictionary/run decoders; PPU queue modes and NMI synchronization; title/name/map/rescue scene scripts; channel arbitration, note fetch and audio envelopes. Unknown actor/item identities and unsupported script domains are explained in ordinary tentative prose.

Fresh final verify returned ok=true / byte_identical=true, and reading-after.asm was refreshed afterward. Verification hashes match final source files. Original ROM, maps/current.json, analysis/state.json and frozen checkpoint/baseline hashes remain unchanged. Source is local commentary/source; original generated/pass-001-final and commentary/before remain preserved. No toolkit edits, archive, publish, cleanup/deletion or commit. Disk approximately 4.5 GiB free; commentary 17 MiB.

Final editorial audit remains pending for final acceptance, including a possible terminal-incomplete outcome; it is not contingent on reaching full classification. No final audit was run in this phase. Scope accounting: readability-round01.json. Questions: uncertainties.md and analysis/handoff-commentary-round01.md. Next workflow phase is redirect on the existing concrete leads; source readability is not structural completion.
