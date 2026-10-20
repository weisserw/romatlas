# Agent instructions

Build the toolkit with `cargo build --locked --release`; setup, workspace tools,
CLI examples and integration tests default to `target/release/romatlas`.

Read project.md and docs/progress.md before work. Keep the full scope; do not mark the goal complete after the raw-data slice. Never download or commit commercial ROMs or extracted assets. Keep generated files local.

Install tools with `bash scripts/setup.sh` on Ubuntu/Debian. Use Rust 1.90.0 and Cargo.lock. Run fmt, clippy, tests, and the portability script after relevant changes. Update docs/progress.md with tested facts.

Current workflow:

```sh
cargo build --locked --release
./target/release/romatlas inspect /path/to/owned.gb
./target/release/romatlas map init /path/to/owned.gb --output /tmp/owned.sym
./target/release/romatlas map validate /tmp/owned.sym --rom /path/to/owned.gb
./target/release/romatlas generate /path/to/owned.gb --map /tmp/owned.sym --out /tmp/owned-source
./target/release/romatlas verify /path/to/owned.gb --generated /tmp/owned-source
```

Success returns JSON `{"ok":true}`; verification adds `"byte_identical":true`. Errors return JSON on stderr and exit 1. Inspection returns metadata including hash and controller. For map cleanup, use `map normalize input.sym --output output.sym`. Use supported stock directives only. Split existing regions before adding a smaller region to avoid overlap.

The mapping/analysis agent workflow will open a ROM, run bounded input sequences, inspect bank-aware traces, propose code regions, confirm map changes through APIs, export .sym, and verify. Use `serve --rom ROM` and the JSON-RPC calls in docs/protocol.md for persistent runtime and map APIs. Use `run ROM --operations N --timeout-ms MS` for bounded DMG ROM-only/MBC1/MBC3/MBC5 execution. The Rust emulator adapter now supports reset, input, state, snapshots, and peeks. Use trace.configure, trace.summary, trace.query, trace.clear and analysis.coverage as documented in docs/protocol.md. See docs/hardware-tests.md for the measured 89-case Mooneye gate. CGB DMA events are available; do not claim full hardware compatibility. Use `decode ROM --offset DECIMAL --count N` for bounded physical-ROM decoding. The decoder does not resolve switchable-bank targets without bank context. The internal store Rust API now supports identity checks and transactional annotations. Use `store init`, `store annotate` (with `--id` for edits), `store get`, `store remove`, `store validate`, and `store export` as shown in README.md. These commands validate the local ROM identity and preserve the file on rejected edits. Use analysis.discover and analysis.references to inspect code candidates; confirm them with map.annotate. Unvisited bytes remain unknown. Use emulator.step unit cycle/frame with count, max_operations and timeout_ms for bounded advancement. Trace streaming, observed edges and cycle/frame stepping are available. See docs/completion-audit.md for completed scope and documented limits.

Run `python3 scripts/test-session.py` after session changes. The script is a full sample agent workflow using only a local synthetic ROM. Shared agent logic must use CommandAdapter and must not import the Game Boy adapter.

Use map.import for stock .sym; use map.load for internal JSON. Imported labels/options survive map.save/load. Read docs/sym.md before stock-only rebuilding of maps with RAM labels.

MBC3 RTC starts at zero and advances only with emulated ticks. Tests cover rollover, latch, halt/resume and replay. Use no host-time shortcuts. CGB basic bank/speed/DMA tests pass; speed-switch CPU/video pause checks pass; broader CGB peripheral/game paths remain documented limits.

Use emulator.physical_memory for immutable backing storage, and emulator.memory for mapped CPU reads. Trace addresses are physical; context.cpu_address keeps the CPU alias. Never turn blocked/unmapped bus accesses into physical ROM/RAM claims.

Use analysis.edges for bounded observed successor counts. Discovery merges them when use_observed=true; a next PC is not execution coverage. Keep data barriers and unresolved CPU-window diagnostics.

External generate/verify calls share timeout_ms (120000 by default, max 300000). Failed generation leaves no final output; retry with the same new path. Verification rebuilds game.o to avoid stale partial objects.

Use analysis.pointers for explicit 16-bit little-endian table hypotheses. Supply effective fixed/switch bank context where needed. Confirm candidates only after inspection; pointer targets do not prove code. Select layout explicitly for bank8/16 first or last fields; encoded banks mean effective physical banks, not mapper registers. Invalid or conflicting banks stay unresolved.

Run scripts/setup-homebrew-tests.sh then scripts/test-homebrew.py for the two pinned licensed game regressions. Preserve upstream MIT/zlib notices. Never commit generated ROMs or extracted game assets.

Shared disasm code must not import Game Boy types. Its backend trait delegates platform work to romatlas-gb::disassembly. Run scripts/test-boundaries.py after dependency changes. Typed execution lives in romatlas-core::execution; state/input/snapshot/config types remain adapter-owned.

Optional trace.stream.start/stop writes bounded local JSONL files independent of the memory buffer. Do not mix raw trace values into portable mappings. Snapshot replay can rewind cycle values; file sequence remains monotonic.

Use analysis.discover_context for CPU-window-aware traversal. Supply entrypoints with offset and cpu_context. It merges recorded before/after bank states by default; use_observed=false gives static-only paths. A reached target is not execution coverage. Context roots/edges survive zero detail capacity and remain separate from portable maps.

Use scripts/port.py prepare ARCHIVE --rom ROM --out OUTPUT for separate native
Rust port workspaces. Read docs/porting.md and the generated prompts. Preparation
restores a byte-identical reference and initializes pending behavior accounting;
it is not a ported game. Original assets load from the ROM at startup, with file
replacements/additional slots. The port agent may implement needed romatlas-port
library changes directly here, uncommitted, with a change log and preservation of
the initial repository state for separate review. Native-port completion uses
explained whole-game best-effort implementations, a source-informed architecture
plan/coherent code, focused unit tests and required format/static/build checks.
Runtime fidelity validation is outside the port goal: no native-versus-ROM frame,
audio, state or scenario comparisons. Debugger use is limited to a specific
unresolved source question with a bounded experiment/stopping condition.
Implemented documented assumptions and unverified fidelity do not block completion;
missing required code does. After the administrative ledger/build audit passes,
stop. Fidelity/device auditing requires a separate user request. Phase-one
classification completion is not a native-port gate. Keep the original mapping goal's stricter
classification contract intact. Do not write tests needing unavailable GUI/devices
or launch SDL graphics on the headless server. Run scripts/test-port.py and the
romatlas-port desktop-feature clippy/tests after relevant changes; these tests
exercise headless logic and compile desktop code without initializing a window.


SNES is a separate baseline adapter using the approved GPLv3 jgenesis fork and
pinned external Asar. The stock .sym/RGBDS/Mooneye instructions above describe
Game Boy; they remain unchanged for that platform. Read docs/snes.md for SNES
JSON maps, CPU/context/address rules, APIs, snapshots, source rebuilding and limits.
SNES CLI selection is automatic for .sfc/.smc; use --platform snes otherwise.
Run python3 scripts/test-snes.py and bash scripts/test-snes-cpu.sh after relevant
SNES changes, alongside workspace checks, Game Boy regressions and portability.
Shared crates must not import either console adapter/core. The native port library
remains independent MIT code; do not copy GPL emulator implementation into it.
SNES-only instruction-width guidance must not add evidence requirements to Game
Boy. Static context may justify decoding; unresolved context stays local to the
affected instruction/view. Preserve the existing structural versus reachability,
whole-ROM classification, full current-source readability and editorial gates.
New SNES profiles select JSON/Asar; never rewrite existing game workspaces as a
side effect. Trace RAM execution does not automatically identify immutable ROM
upload origins or lifetimes. No full hardware/game compatibility claim is supported.

Game Gear shares the Sega adapter/Z80 backend with SMS and uses the pinned
jgenesis GameGear hardware. Read docs/gg.md, docs/gg-native.md and
docs/gg-evaluation.md. Select .gg or --platform gg; use romatlas-gg-map JSON and
WLA-DX, 160x144 LCD, 64-byte CRAM, Start input and stereo PSG without SMS FM.
Run python3 scripts/test-gg.py after relevant changes alongside SMS/workspace/
port/session/portability regressions. Existing workspaces and classification,
reachability, current-source readability and editorial gates remain unchanged.
Do not copy GPL emulator implementation into the MIT native runtime.

Genesis/Mega Drive uses romatlas-genesis and the same pinned GPLv3 jgenesis fork.
Read docs/genesis.md, docs/genesis-evaluation.md and docs/genesis-native.md.
Select .md/.gen/.smd/.bin or --platform genesis. Use romatlas-genesis-map JSON,
GNU binutils 2.42 for 68000 and pinned DeZ80/WLA-DX for Z80. Canonical ROM backing
is big-endian with the original copier prefix; original-file SHA-256 and reversible
encoding metadata preserve word-swapped/SMD identity. Keep original file positions,
CPU aliases, SRAM lanes, SSF 512 KiB windows and Z80 32 KiB windows distinct.
Cycle stepping uses master clocks; operation stepping selects an accepted main or
Z80 instruction while the complete console advances. Never turn prefetches,
blocked bus accesses, RAM uploads or reached targets into ROM execution claims.
Run python3 scripts/test-genesis.py, workspace checks and portability, alongside
existing platform/session/port regressions after relevant changes. Native VDP,
YM2612 and PSG helpers remain independent MIT code; never copy GPL implementation
into them. Preserve classification, reachability, current-source readability and
editorial gates. SVP, Sega CD/32X, exotic peripherals and uncommon boards are outside
the cartridge baseline; no full hardware/game compatibility claim is supported.

PC Engine / TurboGrafx-16 HuCard uses romatlas-pce and isolated GPLv3 jgenesis
v0.13.0 sources in vendor/jgenesis-pce; never migrate existing cores implicitly.
Read docs/pce.md, docs/pce-evaluation.md and docs/pce-native.md. Select .pce/.tg16
or --platform pce. Use romatlas-pce-map JSON, explicit eight-page MPR contexts,
physical file offsets including optional copier prefix and pinned ca65/ld65.
MPR bus pages are not immutable physical ROM banks. Keep CPU aliases, 21-bit bus,
RAM/I/O/CD/unmapped pages and mirrored HuCard backing distinct. Only accepted
instruction fetches establish coverage; block-transfer continuations/interrupt
entry do not. Run python3 scripts/test-pce.py and workspace/port/session/platform/
portability regressions after relevant changes. Native VDC/VCE/PSG helpers remain
independent MIT code; never copy GPL core implementation into them. HuCard only,
no CD/SuperGrafx or esoteric peripherals/uncommon boards. Preserve structural versus
reachability, whole-ROM classification, full current-source readability and
editorial gates. No full hardware/game fidelity claim is supported.

Before falling back to generic data, make a practical attempt to identify its role:
inspect available references and consumers, nearby code and labels, and obvious
formats or repeated record layouts. Use supported specific region types, meaningful
labels and concise format/role explanations when the evidence permits. Generic data
is a fallback after this bounded inspection, not the default shortcut. Record briefly
what was checked and what remains unresolved; reuse prior sound inspection rather
than repeating it. Follow concrete promising leads, but do not require exhaustive
semantic research or block classification when further investigation is unproductive.
Never invent a specific meaning or label just to avoid the generic fallback.

Disassembly classification primarily separates code from data. Clear non-code regions,
including repeated patterns and uniform runs, may be unlabeled generic data blocks
after boundary inspection and checking for conflicting code evidence. Record a brief
rationale and Static provenance with map.annotate; these bytes count as classified
and complete without recovering their format, consumer or meaning. Unknown data
semantics must not cause a blocked handoff. Lack of execution alone is insufficient
to prove data; retain unknown for material code-versus-data or boundary ambiguity.
Commentary needs only a short uncertainty note for generic data, without invented
labels or meanings. Preserve byte-identical rebuilding and code readability gates.

## Broad gameplay progression

Basic gameplay means broad progression through the game, not just startup, menus
or the first encounter. Make a deliberate effort to visit later levels/areas,
distinct gameplay mechanics, bosses, major transitions and the ending/credits or
other terminal screens where present. Discovering these states is useful even
before a specific unknown ROM range or measurable byte gain is identified.
Avoid exhaustive permutations of collisions, rare interactions or every possible
state; representative visits to distinct systems and progression milestones suffice.

Use emulator.memory.write proactively to bypass slow or difficult progression:
identify level/stage, position, checkpoint, health/lives, inventory/key or progression
flags from source, watches and bounded experiments, then patch suitable RAM values.
Prefer a supported stage/checkpoint transition so dependent state is initialized;
use emulator.registers.set only with an understood entry contract. Do not require a
preexisting classification lead or proof that normal button inputs can reach a state
before trying this. Keep ROM backing unchanged. Save a clean snapshot before edits,
record exact addresses/values, bank context, inputs and budgets, and restore before
trying an alternative. Inspect state/frame output and actual execution after the
patch; a written stage number alone does not demonstrate a successful level visit.
Exercise representative mechanics and transitions in each reached state with bounded
inputs rather than merely displaying it. Actual accepted instruction fetches after
patching establish observed execution in that artificial scenario, not natural
reachability; edits themselves and reached targets prove neither execution nor code.

Maintain a progression checklist in analysis/scenarios.json with milestone, route
or patch recipe, observed result, evidence links and status (visited, attempted,
unavailable or deferred with reason). Reuse successful snapshots/routes across rounds.
Before declaring gameplay exhausted or handing off terminally, review unvisited
later-game and ending milestones and attempt a practical shortcut where possible.
Early-game repetition or difficult manual play alone is not grounds to stop while
an obvious state edit or supported shortcut remains. If the necessary RAM contract
is unknown, investigate it in redirect rather than retrying long button sequences.
Do not brute-force arbitrary RAM or spend indefinitely forcing a broken state;
record specific obstacles and try another representative milestone. Aim to reach
the ending, but documented unsuccessful attempts or unavailable transitions do not
require endless experimentation. Preserve classification, fidelity and readability
gates; this is broad best-effort exploration, not an exhaustive gameplay/fidelity gate.
