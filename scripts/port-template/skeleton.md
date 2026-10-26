# Stage 1: skeleton

You are the skeleton subagent. Read workflow.md, prompts/common.md and
prompts/coding-standards.md. The generated arbitrary-tile demonstration is only
starting infrastructure. Replace its assumptions with a game-specific skeleton;
do not implement gameplay, title screens or game-menu logic in this stage.
Toolkit frontend settings are infrastructure and must remain available.

Survey the source labels, dispatch tables, asset directories and archive notes
sufficiently to declare descriptive identifiers for all actors, states, levels,
sounds, music, tables and other assets. Use enums or typed IDs for closed semantic
sets and extensible named asset IDs for added content. Keep a source-linked
identifier inventory in architecture.md, including honest provisional names and
unresolved meanings. Later stages extend/correct it when tracing reveals gaps;
never invent semantics just to fill the list.

Create cohesive module boundaries and native state types, named ROM recipes and
interfaces for future rules, presentation, audio and loading. Wire matching-ROM
loading, override/extension validation, config, settings, desktop and browser
entrypoints through the toolkit APIs. Defer game-specific decoding algorithms
and behavior to the contract/implementation stages; explicit placeholders must
not fabricate assets or claim that source behavior is already implemented.
Keep pending behavior visible in ledger.json and progress.md.

Lay the groundwork for the headless debug API now: connect a game-owned observe
callback to debug::serve_with_snapshots instead of the scaffold's null summary, expose a
versioned semantic state schema, and define event types/observation boundaries
for later logic. Write debug/api.md describing field IDs, units, retention,
read-only inspection, deterministic reset and restorable replay checkpoints.
Use existing transport methods; document how scripts validate starting state and
rebuild checkpoints after changes. Do not add player-visible diagnostic controls.

Ensure tests are registered for native cargo test, including modules shared with
the browser; Wasm-only module gating must not accidentally hide behavior tests.
Run python3 run.py check for formatting, clippy, unit tests and desktop/Wasm
compilation. Test ROM identity/bounds, infrastructure and deterministic debug
reset/checkpoint round trips without pretending this validates game behavior.
Do not run a gameplay smoke test or launch SDL. Hand off architecture.md,
debug/api.md, the identifier/module inventory, check results and any concrete
blocker in progress.md. Stop after infrastructure checks pass; leave all game
behavior pending for the following stages.

Frontend contract: keep config.rs DISPLAY_ASPECT at its default 4:3 unless a
per-game enhancement explicitly changes it. Both frontends derive width from
height and expose no integer scaling setting. Preserve F1–F6 resume/pause/reset/
save-state/restore-state/settings controls. Implement Game::reset and versioned
full-state Game::snapshot_json / atomic Game::restore_snapshot_json, including
mutable game/audio state. The empty scaffold snapshot is not sufficient once
stateful gameplay is implemented. Retain separate original battery save hooks.
