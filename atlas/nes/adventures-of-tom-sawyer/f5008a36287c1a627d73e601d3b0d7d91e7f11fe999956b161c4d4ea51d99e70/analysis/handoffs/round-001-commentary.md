# Round 001 commentary handoff

**Full current-source readability complete. ROM classification remains incomplete:** 206092/262160 bytes (78.61%); 56068 bytes unknown. Final editorial/archive acceptance remains pending and was not run.

## Local source and verification

- [Maintained assembly](../../commentary/source/game.asm), [shared symbols](../../commentary/source/symbols.inc), refreshed [reading copy](../../commentary/reading-after.asm).
- Fresh [byte-identical verification](../round-001-commentary/verification.json), also current at [commentary/verification.json](../../commentary/verification.json). Input hash remains `f5008a36287c1a627d73e601d3b0d7d91e7f11fe999956b161c4d4ea51d99e70`.
- Frozen checkpoint `pass-001-final`, current map and internal state unchanged; map hash remains `9311c48c78733a3ba4333c5140b905cb26fd0c32def1e1ffb49f7c16bc69094a`. Frozen `commentary/before`, `baseline.json`, `phase1-state.json` retained. Do not regenerate over maintained source.
- [Scope assessment](../round-001-commentary/scope-assessment.json), [review inventory](../../commentary/review.json), [progress](../../commentary/progress.md), [uncertainties](../../commentary/uncertainties.md).

## Covered scope

All 804 inventory partitions reviewed: 605 Code, 43 Data, one CHR Image, 155 Unknown. All 12637 code bytes, 62383 data bytes and 131072 image bytes have functional or format/uncertainty explanations. Every unknown emission has an honest local boundary/ambiguity note. **No current-source readability backlog remains.** Review units group source explanations; their intervals are not new structural classifications or asserted original routine boundaries.

201 manually described functional units cover menus/debug selection, opening/ending, six stage initializers/frame wrappers, player control/bounds, camera and map streaming, object/collision logic, metasprites/OAM, sound sequencing and reset/NMI/MMC1. Consistent symbols identify hardware, player/session/camera fields, object arrays, sound-record fields and bank-aware entry/control locations. Explicit ca65 operand widths preserve encoded aliases and instruction sizes.

Examples: `DispatchInlineIndex` explains return-address consumption and inline pointer barriers; `DrawMetatileColumn` and queue builders explain 16-byte definitions, increment direction and 30/32-byte tile writes; `BuildObjectBoundingBoxes` explains signed offset saturation; `AdvanceObjectStreams` separates movement and animation cursors; `NmiFrame` explains interrupt order, busy guard and optional classroom split. Branch symbols name death/threshold, clipping, substream and map-boundary outcomes rather than only numeric origins.

## Unresolved meanings and next work

Generic scene resources retain supported non-code status without invented schemas. Animation/song IDs, many gameplay flags and exact result units remain unresolved but mechanically described. Scratch aliases remain intentionally neutral where shared across routines. Unresolved CPU targets preserve bank uncertainty; naming a call does not prove mapper preservation or classify its return continuation.

Continue redirect on all four concrete leads from the [disassembly handoff](round-001-disassembly.md): prove safe call/return bank contracts; follow scene handlers/resources; resolve ending graphics prerequisites; establish boss/checkpoint spawning. Commentary adds no structural rejection or terminal exhaustion claim. Existing evidence and progression scenarios remain valid, including unsuccessful ending composition.

Toolkit: **no toolkit issues found**, no blockers. [Persistent report](../toolkit-issues.md). No toolkit edits, archive execution, cleanup/deletion or publishing performed. All ROM/source/assets remain local; portable map sharing is a separate authorized action.
