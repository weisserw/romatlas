# Completion audit

This audit uses the full project.md scope. It checks the current source, delivered guides and test coverage, including the clean Ubuntu run. Hardware support remains incremental, as the brief permits. The result does not claim every commercial ROM, every hardware revision or semantic recovery.

## Product behavior

| Required behavior | Inspected evidence | Result |
|---|---|---|
| 1. Metadata, banks, initial decode, unknown regions | ROM parser/CLI, session/API defaults, bank translation tests | Met |
| 2. Headless reset, inputs, instruction/cycle/frame steps, snapshots | Execution adapter, emulator replay/frame/RTC tests, both game input workflows | Met |
| 3. Registers, mapped/physical memory, mapper state and traces | State/capabilities, immutable inspection tests, access/bank/DMA/edge tests | Met |
| 4. Add/edit/remove structural regions with evidence and conflicts | Store transactions, identity/overlap tests, session edit/import rejection cases | Met |
| 5. Static and observed discovery; uncertain/unexecuted bytes stay uncertain | Shared/context traversal tests, data barriers, unfetched-target test, explicit pointer hypotheses | Met |
| 6. Generate banked source/assets without emulator execution | CLI/backend, stock portability and fidelity tests | Met |
| 7. Exact rebuild with first mismatch bank/address | Shared comparison, GB backend diagnostics, corrupt-linker fault test, full byte comparisons | Met |
| 8. ROM-free portable map and independent rebuild | .sym export projection, clean recipient/session/game workflows | Met |

## Architecture and scope

| Requirement | Evidence | Result |
|---|---|---|
| Rust workspace with isolated GB implementation | Seven crates, dependency/source inspection, boundary script | Met |
| Small image/decode/execution/command/backend contracts | Core traits, adapter-owned types, wide mock/replay and alternate backend tests | Met |
| Extensible addresses and distinct memory spaces | Named space/u64/opaque context; wide mocks; physical bank/alias tests | Met |
| Evaluate and adapt a permissive Rust core | Architecture assessment, pinned MIT Mizu fork, retained notices | Met |
| Actual CPU access capture; peeks cause no hardware changes | Bus hooks, internal clock-only cycles, peek/stream equivalence and replay tests | Met |
| DMG and expanded CGB; ROM-only/MBC1/MBC3/MBC5 variants | Mapper/RTC/rumble/bank tests; 89 default and 55 CGB cases; native CGB bank/palette/DMA/pause tests | Met within documented hardware support |
| Explicit unsupported cartridge/hardware cases | Loader checks and unsupported-feature tests; architecture limits | Met |
| No GUI or semantic recovery; future consoles not implemented | Headless source/API and shared mock only | Met |

## Mapping, generation and API contracts

| Requirement | Evidence | Result |
|---|---|---|
| Stock .sym is the principal output, not a custom required schema | Pinned syntax guide/parser/export; stock-only recipients | Met |
| Community labels, aliases and directive options survive | Parser/store tests and imported-map rebuild | Met |
| Deterministic nonsemantic labels and metadata-only output | Sorted export, stable IDs/stock labels, format/portability tests | Met |
| Unknown regions, overlaps and unsupported/ambiguous cases produce honest results | Store gaps/conflicts, unknown-directive tests, discovery diagnostics | Met |
| Internal identity/confidence/trace data stays out of .sym | Export projection and recipient tests; optional internal Store | Met |
| Preserve header/checksum/unused-bank/raw bytes | Arbitrary four-bank golden fixture, exact linking without rgbfix | Met |
| Conservative code, branches, pointers and region boundaries | Decoder/reference/context tests; annotation validation; numeric/raw fallback guidance | Met |
| Minimal mgbdis compatibility workarounds | Metadata-only EQU include and stock RGBDS -D recipient test | Met |
| Bad annotations fail without final source output | Content/directive tests and fidelity fault suite; staged publish after exact rebuild | Met |
| CLI + persistent discoverable JSON API with required operations | CLI source/help, all 43 methods in call table, session/default/result/error tests | Met |
| Run/time/trace/input/output budgets and compact detail | Execution/process/stream/request/response tests, page-only conversion regression | Met |
| Reproducible buttons and snapshots; propose/confirm through tools | Session and game workflows; APIs retain transactional map edits | Met |
| Precise agent guide and runnable examples | AGENTS.md, README, protocol/sym guides and workflow scripts | Met |
| Legal/data hygiene without clearance claims | Licenses/pins, source artifact inspection, ignored generated files; sym guide caveat | Met |

## Tests, setup and delivery

- Decoder tests cover all base and CB lengths/timing, operands/flags, invalid bytes and truncation. The independent hand-derived base matrix supplements the pinned CC0 table. Hardware execution tests are separate evidence.
- Runtime tests cover banking/aliases, interrupts/HALT, timer edges, DMA, input, side-effect-free inspection and deterministic snapshots. MBC3 RTC includes overflow/latch/halt/resume; MBC5 includes bank 257/zero and rumble masking.
- Raw-data golden/property fixtures and text/image/code fidelity tests preserve every byte. Integration checks link traces to map annotations and then rebuild with stock tools.
- Clean recipient tests discard prior assembly/runtime state and use only an identical local ROM, .sym and pinned stock tools. Community imports, malformed maps, conflicts, ambiguous banks, hash mismatch and bounded operations have tests.
- CI declares fmt, clippy and tests, boundary/session/API/portability/fidelity checks, both hardware selections and both licensed games. Local commands passed; no remote GitHub job is claimed.
- Clean Ubuntu noble minbase setup started without Cargo/tools/target caches and installed/build-tested the pinned tools and source. Its full acceptance sequence passed. It uses the host kernel rather than a separate VM.
- Runnable Rust source, lockfiles, licenses, setup scripts, CI, AGENTS.md, compatibility guide, examples and current architecture/limits guide are present. No commercial ROM or extracted asset appears in source delivery.

## Milestone exit conditions

M1: the synthetic mapping/stock assembly/exact rebuild/clean .sym recipient workflow passes.

M2: scripted execution, input and snapshot sequences replay exactly and expose state through the API.

M3: observed code discovery is confirmed through map APIs, persisted/exported and rebuilt byte-identically.

M4: MBC3/MBC5 and expanded CGB checks pass. Galactic Armada and Unbricked both run real input-driven gameplay, expose code/edges, replay snapshots and rebuild exactly in independent stock-recipient directories. Known limits are documented.

The original requirements are met. Broader native-color game coverage, more hardware revisions, peripheral compatibility and faster peeks remain useful later work; the brief explicitly allows incremental hardware and does not require compatibility with every commercial ROM. These limits do not remove any required API, mapping, generation, test or delivery capability.

Local evidence: build/fresh-setup-result.json and build/fresh-setup.log, build/hardware-gate.json (89 passes), build/hardware-cgb-probe.json (55 passes), build/homebrew-results.json, plus current workspace/core and session/API/portability/fidelity checks. Generated evidence remains local and outside the portable mapping.

## SNES extension

The original audit above covers the Game Boy pilot. The later SNES baseline has
its own [capabilities, measured tests and remaining limits](snes.md), including
portable JSON/Asar reconstruction and independent native-port helpers. It does
not establish broad game/hardware compatibility or relax the mapping/classification
and editorial contracts. See progress.md for actual validation of this extension.
