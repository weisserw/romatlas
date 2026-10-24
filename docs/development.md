# Toolkit development

This guide is for changing ROMAtlas itself. Per-game procedures and acceptance
criteria live in the generated workflow files described in
[INSTRUCTIONS.md](../INSTRUCTIONS.md), not in repository-wide instructions.

## Architecture and compatibility

Read [project scope](../project.md) and [architecture](architecture.md) for the
component contracts. Shared types and orchestration must not import console
adapters or cores. Typed execution lives in romatlas-core::execution;
state/input/snapshot/config types remain adapter-owned. Shared agent logic uses
CommandAdapter; shared disassembly delegates platform work through its backend
trait. Keep the native MIT runtime independent of GPL emulator implementation.

Preserve platform-specific address and evidence contracts. CPU aliases, mapper
registers, effective physical banks, mirrored backing and RAM upload origins are
not interchangeable. Blocked/unmapped accesses, prefetches, interrupt entry,
block-transfer continuations and reached successors must not be promoted into
accepted ROM-instruction execution. Read the affected platform's documentation
before changing translation, traces, discovery or source reconstruction.

Preserve existing workspaces and supported map/source round trips. Changes to one
platform must not weaken another platform's classification, reachability,
current-source readability, editorial or byte-identical rebuilding requirements.
These are compatibility obligations when editing the toolkit, not instructions to
perform a disassembly or game audit during an unrelated development task.

## Builds and checks

Use Rust 1.90.0, Cargo.lock and the pinned tools installed by
`bash scripts/setup.sh`. Build the default CLI with:

```sh
cargo build --locked --release
```

After relevant implementation changes, run formatting, static checks, tests and
portability:

```sh
cargo fmt -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 scripts/test-portability.py
```

Use workspace formatting rather than reformatting vendored code. Run the relevant
additional checks below for the components changed. Run cross-platform regressions
when shared behavior changes. Use synthetic or pinned licensed fixtures; do not
acquire commercial ROMs to fill a test gap. Report unavailable prerequisites and
actual results instead of asserting that unrun checks passed.

| Changed component | Relevant additional checks |
|---|---|
| Session/API routing | `python3 scripts/test-session.py`, `python3 scripts/test-api.py` |
| Dependencies or adapter boundaries | `python3 scripts/test-boundaries.py` |
| Generation or verification | `python3 scripts/test-fidelity.py`, portability and affected platform tests |
| Disassembly workspace preparation | `python3 scripts/test-disassemble.py` |
| Commentary or archive workflow | `python3 scripts/test-commentary.py`, `python3 scripts/test-archive.py` as applicable |
| Native port preparation/runtime | `python3 scripts/test-port.py`; romatlas-port desktop-feature checks below |
| Optional modernization beta | `python3 scripts/test-port-modernize.py`; `python3 scripts/test-port.py` for preparation changes |
| Native QA installation/record contract | `python3 scripts/test-port-debug.py`; `python3 scripts/test-qa-harness.py`; `python3 scripts/test-scenario-client.py`; `python3 scripts/test-qa-variants.py` (including explicit sanity-suite separation) |
| Browser port support | `python3 scripts/test-port-web.py` |
| Game Boy Color | `python3 scripts/test-gbc.py` and Game Boy regressions |
| SNES | `python3 scripts/test-snes.py`, `bash scripts/test-snes-cpu.sh` |
| NES | `python3 scripts/test-nes.py` |
| Master System | `python3 scripts/test-sms.py` |
| Game Gear | `python3 scripts/test-gg.py` and shared Sega regressions |
| Genesis | `python3 scripts/test-genesis.py` |
| PC Engine | `python3 scripts/test-pce.py` |

For native runtime/port changes, compile desktop code and test headless logic:

```sh
cargo clippy --locked -p romatlas-port --all-targets --features desktop -- -D warnings
cargo test --locked -p romatlas-port --features desktop
```

For licensed Game Boy game regressions, install the pinned fixtures with
`bash scripts/setup-homebrew-tests.sh`, then run `python3 scripts/test-homebrew.py`.
See [hardware tests](hardware-tests.md) for hardware fixture setup and measured
gates. Keep generated ROMs local and preserve upstream MIT/zlib notices.

Documentation-only edits need checks appropriate to their content, such as link
and whitespace checks; they do not require executing games. Toolkit regression
checks establish tool behavior, not gameplay fidelity. Report actual check results
and limitations to the user.

## Platform and API references

Use the affected platform's current guide for image formats, map/assembler choice,
CPU/context rules, execution units, supported hardware and limits. Do not apply
Game Boy stock .sym/RGBDS rules to every platform or infer full compatibility from
a test count in a historical progress entry.

| Platform | Adapter/API/source guide | Native runtime guidance |
|---|---|---|
| Game Boy / Color | [protocol](protocol.md), [stock .sym](sym.md), [CGB](gbc.md) | [porting](porting.md), [CGB](gbc.md) |
| SNES | [SNES](snes.md), [context/prompt audit](snes-prompt-audit.md) | [SNES native](snes-native.md) |
| NES | [NES](nes.md) | [NES native](nes-native.md) |
| Master System | [SMS](sms.md) | [SMS native](sms-native.md) |
| Game Gear | [GG](gg.md) | [GG native](gg-native.md) |
| Genesis / Mega Drive | [Genesis](genesis.md) | [Genesis native](genesis-native.md) |
| PC Engine / TurboGrafx-16 | [PCE](pce.md) | [PCE native](pce-native.md) |

[Protocol](protocol.md) documents mapped versus physical reads, trace streaming,
context-aware discovery, pointer hypotheses, replay and generation deadlines.
[README](../README.md) documents CLI/store commands; [stock .sym](sym.md) covers
Game Boy map import/export and RAM labels. Keep API parameters and numeric limits
in their owning reference rather than copying them into AGENTS.md.

## Disassembly policy ownership

The coordinator, disassembly and redirect templates under
`scripts/workflow-template/` already contain the broad progression/shortcut policy.
Their evidence-policy.md and phase prompts contain structural-versus-reachability
rules and bounded inspection before generic data fallback. Maintain those owning
files when changing the policy; do not reintroduce copies in toolkit AGENTS.md.
Historical compatibility measurements belong in platform evaluation/hardware docs
with their original scope and limitations.
