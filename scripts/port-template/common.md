# Shared native-port contract

Read this contract in every pipeline stage, alongside that stage's prompt.
Read workspace.json for toolkit/ROM/platform paths, architecture.md, progress.md,
ledger.json, uncertainties.json, differences.md, library-changes.md and existing
port code. Use reference/reading.asm and maintained source as primary evidence;
inspect research/current.json and its adopted source supplement when present.
Preserve work; do not reinitialize. Keep reference/, original ledger identities,
review/toolkit-before/ and unrelated changes intact.

Consult the toolkit's docs/development.md platform-reference table for the
source CPU/address context and independent native helpers selected by
workspace.json. Those references apply to skeleton, tracing and implementation,
including Game Boy Color, Genesis and PC Engine special concerns.

## Scope and architecture

Implement the whole source game as readable native Rust, including title/menu
behavior, all gameplay/progression, rendering, sound and original saving. Organize
modules around game concepts, with explicit state owners, small interfaces and
clear algorithms/update order. Avoid one growing game.rs and cosmetic file splits
that leave tangled ownership. No file-count, line-count or abstraction quotas
apply. architecture.md records design decisions and dependencies; behaviors.md
indexes source behavior, contracts and implementations. Neither replaces source
review. Update both when discoveries change the design or inventory.

Follow prompts/coding-standards.md for all authored Rust, including tests,
fixtures, debug adapters and scaffolding. The behavior-contract and implementation
prompts include that same language verbatim.

The player-facing target is the source game. Unless the user requests them, do
not add level selects, sound tests, overlays, cheats or developer menus. Preserve
source features with their original access conditions. Keep diagnostics in logs
or headless tools. Required frontend settings and asset extensions are supported;
they do not authorize extra content-selection menus. Hardware-induced lag,
slowdown and sprite flicker need not be reproduced. Preserve intentional timing
and visual effects, including deliberate slow motion and invulnerability flashing.

## Runtime, assets and frontends

Use romatlas-port and its independent platform helpers for common mechanisms;
review documented limits. Keep game rules, sequence interpretation and unusual
tricks in the game when appropriate. Simulate deterministically at console cadence
independent of host refresh. Render/inspect without mutating simulation state.

Load all original graphics, fonts, music, sound, maps and other gameplay data
from the matching user's ROM at startup through named recipes. Keep original
encodings/physical offsets in loaders or definitions. Support validated file overrides
and added named slots: original ROM-backed levels and added file-backed levels
must work together without a fixed level count. Use MemoryAssets::collection
for fetched or filesystem overrides; load only game-owned manifests on desktop.

Desktop SDL2 and browser Wasm share Game rules, Port::load_assets, decoded assets,
validation and battery-save representation. Frontends handle rendering, audio,
input and persistence; do not branch game rules by frontend. Keep dimensions,
TickRate, ROM identity and default settings in config.rs. Supply persisted
keyboard/gamepad mapping, per-player controller assignment, dead zones, height/
fullscreen and volume settings through the toolkit's usable settings menu.
Compile desktop code without opening a window on this headless server.

If the original has battery saves, implement the optional Game save hooks with
versioned game-owned data, transactional validation and dirty/acknowledgement.
Desktop files and browser localStorage use the same ROM-bound envelope.
No-battery games retain the default battery hooks. Implement full native player
snapshots separately through Game::snapshot_json / restore_snapshot_json. Headless
debug replay checkpoints remain a separate testing facility. Document original
saving, full snapshots and their native formats.

## Debug observability and source questions

All logic must work through the headless Game driver and expose the semantic
state needed by contracts and QA. Use the observe callback of
romatlas_port::debug::serve_with_snapshots for a versioned read-only summary in port.state:
mode/scene, progression, actors, timers, flags, transitions and sound/music state
as relevant. Define stable identifiers and units, avoid opaque RAM dumps, and
provide bounded semantic event observations when endpoint state cannot establish
a check. Inspection must not change simulation or drain events needed by a check.
Keep schema/field meanings and event retention rules in debug/api.md.

Use existing deterministic reset/input/advance/checkpoint methods and advertised
session-local snapshot save/load/delete for fast branch retries. The snapshot-capable
server requires Port::clone to isolate ALL mutable simulation, audio, scheduler,
input-edge and event state; share immutable assets only. Fork shared mutable event
journals and rebind callbacks so abandoned branches cannot mutate a saved state.
Validate branch/replay state, frame/audio hashes and pending transition behavior.
Snapshots are process-local and cleared on reset; durable recipes/checkpoints
remain required for later sessions and builds. A checkpoint
replays inputs and tick/audio calls; it is not arbitrary object serialization.
Validate restores with semantic preconditions as well as the transport's output
hashes. Rebuild checkpoints from their saved setup recipe after a build/asset
change, with fresh validation; never bypass identity checks. Do not invent native
memory/register writes or unsupported snapshot commands. A necessary new adapter
must have an explicit, validated contract and preserve deterministic replay.

Before QA, emulator use answers a specific unresolved source question: record
the question, needed observation and stopping condition, run a bounded experiment,
then return to the active stage. Revise the hypothesis/method before repeating
inconclusive runs. Final QA primarily tests native player-visible behavior. Direct runtime comparisons
are limited to separate initial sanity checks and specific uncertainty/defect diagnosis.
No GUI or unavailable audio devices are required. Follow DEBUGGING.md for API,
checkpoint/client limits and disk/evidence retention; keep original captures local.

## Uncertainty, library work and checks

Incomplete source classification does not require finishing disassembly first.
Assess uncertainties by consumers and port impact, preserving IDs/ranges. Read
prompts/reference-updates.md for the optional verified source-correction procedure.
Distinguish source facts from reasonable implementation assumptions, documenting
basis, alternatives and residual uncertainty. A planning claim, passing build or
unit test alone proves no original behavior. Preserving data is allowed only with
a supported ROM-backed consumer; preserving instruction bytes does not implement
native logic. Missing behavior cannot be hidden by exclusions or differences.

Implement needed shared-library changes directly in the toolkit in workspace.json.
Read its AGENTS.md and project.md, preserve unrelated work and leave edits
uncommitted. Log requirements, files, reusable interface rationale, focused tests
and limits in library-changes.md; compare review/toolkit-before.json and its
snapshot to distinguish pre-existing work. Two consumers are not required to
justify a reusable utility; keep uncertain abstractions local. Run relevant
toolkit regression checks, including portability/dependency boundaries when
applicable, and report actual results/limits. Toolkit checks add no game-fidelity
acceptance requirements.

Use Rust 1.90.0 and Cargo.lock. Stage prompts specify when checks must pass and
when intentionally failing behavior contracts are expected. Use meaningful
branch/boundary/interface tests and synthetic fixtures or local ROM data; avoid
trivial getter/arithmetic tests and assertions that mirror the implementation.
Do not add more validation simply because time or goal budget remains. QA must
still execute both required passes afresh. No check-in or publication is
authorized. A separate library review uses prompts/review.md; release packaging
and immutable dependency pinning are later work.

Frontend contract: keep config.rs DISPLAY_ASPECT at its default 4:3 unless a
per-game enhancement explicitly changes it. Both frontends derive width from
height and expose no integer scaling setting. Preserve F1–F6 resume/pause/reset/
save-state/restore-state/settings controls. Implement Game::reset and versioned
full-state Game::snapshot_json / atomic Game::restore_snapshot_json, including
mutable game/audio state. The empty scaffold snapshot is not sufficient once
stateful gameplay is implemented. Retain separate original battery save hooks.
