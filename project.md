# Project: ROMAtlas — Agent-Driven ROM Mapping

## NES extension

NES is now in scope alongside Game Boy and SNES: NTSC baseline hardware and
common mappers, a controllable instrumented Rust debugger, full 6502 decoding and
byte-identical ca65/ld65 reconstruction, portable ROM-free JSON maps and independent
MIT native graphics/audio helpers. See docs/nes-evaluation.md, docs/nes.md and
docs/nes-native.md. Rare peripherals/mappers and PAL remain deferred. All existing
classification, reachability, current-source readability and editorial gates remain.

## SNES baseline extension

The agreed next platform is baseline NTSC SNES (LoROM/HiROM, standard controllers,
SRAM, SPC700/DSP audio), covering both the agent debugging/reconstruction workflow
and port-driven native runtime helpers. The initial isolated evaluation is in
[docs/snes-evaluation.md](docs/snes-evaluation.md); GPLv3 debugger adoption was approved after the isolated spike; production
baseline commands and measured limits are documented in [docs/snes.md](docs/snes.md). This extends the original
Game Boy-only phase scope below without weakening its classification, readable
source, portable mapping or byte-identical rebuild requirements. Game Boy retains
stock mgbdis .sym; SNES uses a platform-specific portable JSON mapping and Asar backend.
See [the prompt audit](docs/snes-prompt-audit.md) for preserving static-evidence
rules and preventing SNES-specific context requirements from affecting SM83.

## Mission

Build **ROMAtlas**, an extensible ROM reverse engineering toolkit for agents, with Game Boy, baseline NES and SNES adapters. Its first goal is to let an agent examine a user-supplied ROM, identify code and data, and produce a structurally annotated assembly listing that rebuilds byte-for-byte to the original ROM. Structural roles and supporting evidence are required. The agent workflow must also produce fully commented, labeled assembly suitable as the porting agent’s source input, with descriptive routine/branch names, RAM variables, object fields and constants. Explain meaningful behavior and structures; instruction-by-instruction paraphrases are unnecessary. Document unresolved meanings honestly.

The agent must be able to use a controllable, instrumented emulator, build a **portable, ROM-independent mapping file**, and generate the assembly listing from that mapping and the user's own ROM. The toolkit must expose evidence and uncertainty so the agent can check and revise its decisions. Keep its tools generic enough to support other reverse engineering tasks and later work, such as naming symbols and explaining game behavior, without adding those tasks to this phase.

**The portable mapping and the locally retained readable assembly are required deliverables.** It must be useful to someone who independently possesses the same ROM, without distributing any original ROM bytes, graphics, audio, or other copyrighted content. Structural and semantic investigation may proceed together; complete gameplay explanations are not required. Prioritize accurately distinguishing executable code, data, padding, pointers, and other structural regions, and identifying bank-aware boundaries and references. Keep role/evidence annotations in local analysis records or the internal store where supported, without inventing stock map directives. Whole-ROM agent completion requires evidence-backed classification of every physical byte, required annotations, valid maps and byte-identical verification. Raw-byte fallback remains essential for fidelity but does not establish classification completion. See the generated workspace contract in scripts/disassemble.py for single-pass easy-gain stopping, incremental commentary on incomplete source, and redirect-driven iteration. Every commentary invocation completes readability across currently classified source, catching up older gaps while preserving sound prior work; interruptions resume in the same phase before redirect. This is separate from whole-ROM editorial acceptance; final editorial cleanup and audit remain separate gates. An exhausted easy-gain pass hands off incomplete; every redirect investigates promising questions thoroughly but reasonably, allowing groundwork and intermediate understanding without premeasured substantial gains. Evidence and useful actionable next steps drive further rounds; rabbit holes, slim chances and repeated stalled work do not. Whole-ROM completion requirements remain unchanged.

This is an implementation project, not a design-only exercise. Deliver a functioning end-to-end vertical slice early, then improve coverage and accuracy with tests. Do not stop after writing a plan.

## Product behavior

Given `game.gb` or `game.gbc`, an agent must be able to:

1. Inspect cartridge metadata, bank layout, initial disassembly, and regions whose type is unknown.
2. Start a headless emulator, reset it, supply button inputs, advance by instruction/cycle/frame, and save/restore deterministic snapshots.
3. Query CPU registers, mapped and physical memory, current bank-controller state, execution coverage, memory-access traces, control-flow edges, and bank-switch events.
4. Add, edit, and remove bank-aware structural annotations in an internal analysis store; record confidence and provenance (observed, statically inferred, manually asserted), and detect conflicting annotations.
5. Ask the toolkit to analyze reachable instructions and references, combining static decoding with observed execution to propose new code/data boundaries. Unexecuted code must not automatically be classified as data.
6. Generate a bank-organized assembly project and extracted data **from the local ROM and mapping**, without needing the emulator to run.
7. Reassemble generated source and compare it byte-for-byte with the input ROM; report the first mismatch with bank/address context. A byte-identical round trip is a key acceptance criterion for supported ROMs. Where mgbdis cannot preserve fidelity, document the limitation and implement a minimal compatibility-preserving workaround rather than replacing the whole disassembler.
8. Export only the mapping, without copying any ROM bytes into it. Another user with the same ROM can import the mapping and reproduce the disassembly locally.

## Scope

### In scope

- Original Game Boy (DMG) and Game Boy Color (CGB) cartridges, with incremental hardware support.
- LR35902/SM83 CPU decoding, instruction execution, interrupts, timers, joypad, PPU sufficient for real game execution, DMA, memory mapping, and cartridge controllers.
- At minimum, ROM-only and MBC1 cartridges for the first milestone; add MBC3 and MBC5, including common variants, as subsequent milestones. Explicitly report unsupported cartridge features rather than silently misbehaving.
- Headless emulator with agent-friendly JSON commands and machine-readable results, accessible via CLI and a persistent JSON-RPC-over-stdio server (or a similarly simple documented protocol). An MCP adapter is optional and should wrap the same core API.
- Deterministic, queryable execution tracing and structural mapping.
- Use **mgbdis** as the initial disassembly generator and its `.sym` annotation syntax as the primary distributable mapping format. Pin a tested mgbdis revision and RGBDS version (`rgbasm`, `rgblink`, `rgbfix`); document dependencies and licensing.
- Maintain richer analysis state internally; export a deterministic mgbdis-compatible `.sym` file as the main shareable artifact. Do not require recipients to have the internal analysis database.
- Extensible Rust architecture that avoids assuming all future platforms have 16-bit addresses or bank-switched ROMs.

### Out of scope

- A graphical emulator frontend or human-oriented gameplay UI.
- Requiring the toolkit itself to infer game semantics automatically; investigation, naming and explanation are agent responsibilities.
- Automatically producing a clean-room behavioral specification or reimplementation.
- Reproducing original source formatting, labels, macros, or comments.
- Supporting additional consoles beyond Game Boy, baseline NES and SNES.
- Guaranteeing compatibility with every commercial ROM in the initial release.
- Publishing, bundling, or downloading commercial ROMs.

## Architecture

Use a Rust workspace with independently testable crates/modules. Suggested organization (adjust when justified):

- `romatlas-core`: platform-neutral identifiers, address-space abstractions, structural region types, trace/event envelopes, diagnostics, and shared interfaces.
- `romatlas-gb`: Game Boy platform adapter, including SM83 CPU, bus, peripherals, cartridge banking, deterministic stepping, snapshots, instrumentation, and Game Boy-specific address translation.
- `romatlas-analysis`: platform-neutral coverage, trace processing, control-flow/region discovery orchestration, annotations, provenance, conflicts, and validation; platform-specific instruction decoding and reference interpretation must be delegated to adapters.
- `romatlas-map`: internal analysis schema, platform-neutral mapping operations, and a Game Boy-specific mgbdis `.sym` import/export adapter. Do not impose `.sym` on future platforms.
- `romatlas-disasm`: platform-neutral generation/verification orchestration with a Game Boy backend invoking pinned mgbdis and RGBDS.
- `romatlas-agent`: platform-neutral persistent session manager and structured agent command API, routing platform-specific operations through the adapter.
- `romatlas-cli`: CLI entry points for inspection, analysis, mapping, generation, and verification.

A single workspace is preferable to prematurely publishing many crates. Preserve these dependency boundaries: platform-neutral crates must not depend on `romatlas-gb`, SM83 types, mgbdis, RGBDS, or Game Boy cartridge assumptions. The Game Boy adapter may depend on shared crates; shared crates must never import the Game Boy adapter.

### Platform adapter boundary

Define a small, documented Rust interface that a future console implementation can satisfy without modifying platform-neutral analysis or agent-control logic. The adapter must provide platform identity and capabilities, ROM/image loading and layout, address-space description and translation, execution/stepping and snapshots, side-effect-free inspection, instrumentation/event production, instruction decoding and platform-specific control-flow/reference interpretation, and a mapping/disassembly backend. Explicitly support platform-specific commands and unsupported-capability errors where hardware differs.

Keep addresses typed and extensible: do not assume 16-bit CPU addresses, fixed bank sizes, bank-switched ROMs, a single CPU, or one universal memory map. Platform-neutral records should retain opaque platform-specific address context where needed; Game Boy bank resolution belongs in `romatlas-gb`. Likewise, mgbdis `.sym` is the **Game Boy export format**, not the universal internal schema.

Implement and test the Game Boy adapter first. Include a small mock platform adapter in tests to prove that generic analysis and agent APIs can operate without Game Boy-specific dependencies. Avoid elaborate plugin loading or generalized hardware emulation frameworks in this phase; the objective is a clean compile-time extension point, not support for another console yet.

### Emulator implementation strategy

Prefer evaluating an actively maintained, permissively licensed **Rust** Game Boy emulator core and adapting/forking it if its internals permit precise instrumentation and deterministic stepping. Before choosing a dependency, assess license, completeness, CPU stepping access, bus read/write interception, bank-state visibility, save states, and maintainability. Document the decision in `docs/architecture.md`. If no suitable core can be adapted without excessive friction, implement the necessary core in Rust incrementally. Avoid requiring a C/C++ runtime core in the finished toolkit. Vendored or forked code must preserve attribution and license notices.

Never assume a generic memory read can substitute for observing actual CPU bus accesses; debugger inspection must not alter emulation state. Support side-effect-free memory peeks separately from hardware reads.

### Canonical addressing

Represent distinct address spaces explicitly: cartridge ROM physical offset and bank/offset; CPU-visible addresses; WRAM/VRAM bank and offset; cartridge RAM bank; and I/O. Capture the effective ROM bank on **each instruction fetch and reference**, including fixed-bank and switchable-bank windows. Handle bank-controller aliases and nontrivial banking rules. Do not conflate CPU address `$4000` with a unique ROM location.

### Instrumentation

Provide configurable event capture for instruction fetches, reads, writes, control-flow transfers, interrupts, DMA, bank-controller writes, and frame boundaries. Events should include relevant physical/mapped addresses, values, cycle count, and CPU state where applicable. Provide filters, aggregation, bounded buffers, and optional streaming so full tracing does not overwhelm memory or an agent's context window. Prefer compact summaries (coverage, hot addresses, observed edges, region candidates) and requestable detail.

Distinguish direct observations from static deductions. Instrumentation must be disableable for speed and must not change emulated behavior.

## Game Boy mapping file: critical contract

**The Game Boy distributable mapping format is mgbdis-compatible `.sym`, not a new custom TOML schema.** SNES uses its own ROM-free JSON mapping and Asar backend as documented in docs/snes.md. Investigate and document the exact supported syntax in the pinned mgbdis version before implementation. Use the existing bank-qualified labels and region directives (`.code`, `.data`, `.text`, `.padding`, `.image`, etc.) where supported; do not assume features not actually implemented by mgbdis. Preserve standard compatibility so that an external user can supply the same ROM and the exported `.sym` file directly to stock mgbdis.

The internal analysis store can use Rust structs plus SQLite, JSON, or TOML as appropriate. It may track ROM hash, bank-aware regions, uncertainty, conflicts, coverage, confidence, provenance, candidate pointer tables, entry points, edges, and observations. **Do not leak internal-only metadata into the exported `.sym` in ways that break stock mgbdis.** Keep internal data separate from the portable output; if useful, support optional sidecar export clearly labeled nonessential.

Required behaviors:

- Import existing mgbdis `.sym` files, including community-produced mappings, preserving supported annotations through an import/export round trip.
- Export deterministic `.sym` files containing only metadata needed for disassembly: bank-aware labels and supported region directives. Do not include ROM bytes, extracted text, image data, or other original assets.
- Auto-generate stable, syntactically valid, nonsemantic labels as needed; these are intermediate generator fallbacks, not the final commentary naming standard.
- Track unknown and ambiguous regions internally; do not invent unsupported `.sym` directives to represent them. Use mgbdis defaults or safe explicit annotations only where justified.
- Detect overlaps, unsupported constructs, unresolved bank identities, and directives that mgbdis cannot faithfully express. Surface actionable diagnostics instead of silently losing information.
- Treat ROM identity (e.g. SHA-256) as optional *internal or sidecar* validation metadata, since stock `.sym` may not support it. Do not make a custom hash header mandatory in the distributable `.sym`.
- **Portability test:** export the `.sym`, discard all generated assembly and runtime analysis state, and regenerate using only an independently supplied identical ROM, the `.sym`, and pinned stock mgbdis/RGBDS. Verify that output is deterministic and reassembles byte-identically for supported test ROMs.

**Legal/data hygiene:** do not include original ROM bytes or extracted assets in shareable mappings or fixtures sourced from commercial games. Document that mapping metadata may still raise legal questions; do not claim automatic legal clearance.

## Disassembly and round-trip rules

Structural classification and reachability are separate claims. Require evidence
for the claimed structural role, exact boundaries and effective bank context;
require reachability evidence only when claiming reachability. A live caller,
observed instruction fetch, naturally produced selector or ordinary gameplay route
is useful evidence, not a universal prerequisite for classifying code or data.
Apply this distinction from the first disassembly pass and throughout redirect
investigation. Independently supported routine contracts, control/data flow and
bounded record/table formats can justify static classification while natural use
remains unresolved. Record structural evidence and reachability status separately;
do not label statically inferred bytes Observed or claim they are unused/dead
merely because no caller is known.

Caller/selector tracing remains useful for resolving roles, bounds and bank context.
When selected indices are the only evidence for a table's extent, retain unsupported
entries as unknown until an independent extent proof exists. Valid decoding, pointer
values, adjacent records, alignment and pattern resemblance alone do not establish
structure. Artificial execution can test consumer mechanics but does not establish
intended code, complete table extent or natural reachability by itself. Keep these
evidence safeguards and all classification, fidelity and editorial completion gates.

Apply the generated workspace's evidence-policy.md when making or rejecting static
inferences. Independent constraints may jointly establish a role and exact extent;
no particular observation or encoded declaration is a universal prerequisite.
Localize counterexamples to the claim they actually falsify, distinguish semantic
representation from exact recovery of the original generator, and reconsider flawed
rejections on existing evidence. Before terminal exhaustion, workers and coordinator
must challenge the decisive rejection premises against primary artifacts. Preserve
genuine negative results and stop unchanged retries; this review neither weakens
whole-ROM classification nor requires speculative progress. See
[the prompt inference audit](docs/agent-inference-audit.md).

Structural investigation is continuous redirect work at every classification
percentage. Supported proposals go through disassembly and full current-source
commentary; useful intermediate evidence or justified next methods continue redirect
without empty phase cycles. No separate final-investigation phase, completion
threshold or fixed pass quota controls research. Final editorial acceptance and
byte-identical verification remain distinct completion gates.

Native port preparation may use reviewed, byte-identical archives with incomplete
classification. Track residual ranges and semantic uncertainties by consumer and
port impact and preserve original ROM-backed data when justified. Implement required
gameplay behavior before claiming a finished best-effort port. Reasonable source-informed implementation assumptions may remain documented;
original-game equivalence need not be proven. Discoveries may correct a local
working disassembly/commentary through evidence-linked, map-validated and exact-rebuild
verified revisions while the archived baseline and original accounting IDs remain
frozen. This does not relax the original mapping goal's every-byte contract.

- Separate code from non-code based on validated instruction boundaries, known entry points, static analysis, and observed execution; ambiguous areas default to lossless raw bytes.
- Prefer stock mgbdis output for deterministic filenames, labels, and RGBDS bank/section declarations; do not independently recreate its assembly emitter without demonstrated need.
- Preserve every byte in ROM order, including headers, checksums, padding, inaccessible/unused banks, and unknown regions.
- Resolve code references to labels only when the target is unambiguous and assembler semantics preserve the original encoding. Otherwise emit numeric operands or raw bytes.
- Handle relative branches, RST vectors, jump tables, inline data, and instructions that cross region boundaries conservatively.
- For pointer tables, distinguish raw little-endian values from confirmed bank-aware pointers; never invent bank targets from 16-bit values alone.
- Support `generate` and `verify` commands. `generate` exports/uses the `.sym` and invokes pinned mgbdis; `verify` rebuilds with RGBDS and performs exact binary comparison with a detailed mismatch report.
- If an annotation would break round-trip fidelity, reject it or fall back to raw-byte emission with a diagnostic.

## Agent control surface

Provide a stable, discoverable, documented API with structured input/output and informative errors. Minimum operations:

- `rom.open`, `rom.info`, `rom.read`, `rom.regions`
- `emulator.start`, `emulator.reset`, `emulator.step`, `emulator.run`, `emulator.input`, `emulator.state`, `emulator.snapshot.save`, `emulator.snapshot.load`
- `trace.configure`, `trace.summary`, `trace.query`, `trace.clear`
- `analysis.decode`, `analysis.discover`, `analysis.references`, `analysis.coverage`, `analysis.conflicts`, `analysis.unknown`
- `map.get`, `map.annotate`, `map.remove`, `map.validate`, `map.import`, `map.export`
- `disasm.generate`, `disasm.verify`

Operation names may change, but all capabilities must be present. Expose bounded execution limits, timeouts, and trace budgets; prevent accidental infinite runs. Allow scripts to drive reproducible sequences of button presses and snapshots. Return compact JSON summaries by default, with pagination or ranges for large outputs. Ensure the agent can locate unmapped areas and propose/confirm classifications through tools, not by editing internal database files directly.

Provide an `AGENTS.md` with precise instructions for Codex or another coding agent to run the toolkit, inspect results, update a map, and verify output. Include a realistic end-to-end agent workflow and sample command transcripts.

## Implementation milestones

### M1 — End-to-end skeleton

- Workspace, tests, CLI, mgbdis `.sym` parser/exporter, ROM parser, physical-bank address model.
- Complete SM83 instruction **decoder** (execution can be incomplete initially).
- Pinned mgbdis integration, conservative `.sym` generation, and byte-identical RGBDS round trip for small test ROMs.
- `.sym` import/export and internal ROM hash validation.

**Exit condition:** a synthetic ROM can be mapped, processed by stock mgbdis, assembled, verified, and regenerated from the exported `.sym` alone.

### M2 — Controllable emulation

- Working headless CPU and bus, ROM-only and MBC1, sufficient timers/interrupts/PPU/joypad to run test ROMs.
- Instruction/frame stepping, input, register/memory inspection, snapshots, and persistent JSON command interface.
- Deterministic execution tests against known public hardware test suites where licenses permit.

**Exit condition:** scripted gameplay/test sequences can be replayed reproducibly and inspected through the agent API.

### M3 — Tracing and structural discovery

- Bank-aware coverage and control-flow tracing, memory-access observation, event filtering.
- Static recursive traversal from entry points and trace-discovered targets, with uncertainty tracking.
- Internal annotation update/validation APIs, conflict detection, code/data classification, unknown-region reporting, and mgbdis `.sym` export.
- Stable-label `.sym` generation through mgbdis with verified round-trip.

**Exit condition:** an agent can run a test ROM, discover additional code regions, persist them to a map, and regenerate a byte-identical disassembly.

### M4 — Broader Game Boy support and robustness

- MBC3/MBC5, expanded CGB support, more complete PPU/timing/DMA behavior, and edge-case cartridge handling.
- Differential/regression tests and integration tests with legally distributable homebrew ROMs.
- Performance controls, trace budgets, clear unsupported-feature diagnostics, documentation and examples.

**Exit condition:** multiple nontrivial homebrew games run, can be explored programmatically, and round-trip successfully; known limitations are documented.

## Testing and quality requirements

- Unit tests for every opcode's length, operands, timing/flags where applicable, and decoding behavior, including CB-prefixed instructions and invalid opcodes.
- Tests for bank switching, mapping aliases, interrupts, HALT behavior, timer edge cases, snapshots, and side-effect-free memory inspection.
- Property/golden tests ensuring arbitrary byte sequences remain byte-identical when emitted as raw data.
- Integration tests that run ROMs, record traces, annotate maps, generate assembly, reassemble, and compare hashes and bytes.
- Test `.sym` portability on a clean workspace with only the original test ROM, exported `.sym`, and pinned stock mgbdis/RGBDS; verify community `.sym` import compatibility.
- Tests for conflicting/overlapping regions, unknown bytes, ambiguous banked references, malformed maps, hash mismatch, and bounded run/trace operations.
- Prefer public-domain or permissively licensed synthetic/homebrew test ROMs; do not commit copyrighted commercial ROMs or their extracted assets.
- Run `cargo fmt`, `cargo clippy`, and `cargo test` in CI. Pin toolchain/dependencies and provide reproducible setup instructions for RGBDS.

## CLI examples (illustrative; finalize and document actual syntax)

```sh
romatlas inspect ./game.gb
romatlas map init ./game.gb --output game.sym
romatlas serve --rom ./game.gb --map game.sym
romatlas analyze ./game.gb --map game.sym --entrypoints
# Agent interacts with persistent session, explores, and updates map.
romatlas generate ./game.gb --map game.sym --out ./build/game
romatlas verify ./game.gb --generated ./build/game
```

## Design principles

1. **Mapping-first and interoperable:** stock mgbdis-compatible `.sym` metadata is the product; assembly is a reproducible derivative.
2. **Lossless by default:** unknown or uncertain bytes are preserved rather than guessed.
3. **Observable execution:** every structural claim can be linked to runtime evidence, static reasoning, or explicit assertion.
4. **Agent-native:** automation, determinism, machine-readable errors, bounded results, and replayability matter more than a GUI.
5. **Bank-aware correctness:** physical ROM identity must never be lost through CPU address translation.
6. **Evidence-driven analysis:** investigate semantics and name routines where useful to resolve structure; readable source with explanations and consistent symbols is a separate final completion requirement.
7. **Future portability without premature generalization:** keep console-specific logic isolated while making the Game Boy implementation complete and useful.

## Delivery requirements

Deliver runnable Rust source, documented CLI and agent protocol, mgbdis `.sym` compatibility guide and examples, pinned external tool dependencies, tests, CI configuration, `AGENTS.md`, and a concise architecture/limitations document. Keep a running milestone checklist in `docs/progress.md`, updating it as features land. Prefer working vertical slices over broad unfinished scaffolding. If the full scope cannot be completed in one pass, leave the repository in a tested, runnable state, clearly identify incomplete milestones, and do not claim unsupported games or features work.

**Definition of success:** an autonomous agent can investigate a Game Boy ROM using the toolkit, iteratively build a ROM-free structural map, and another user with the identical ROM can use the exported `.sym` with stock mgbdis to generate and byte-verify the same disassembly without needing the original agent's execution traces or any distributed ROM content.

## Additional native port workflow scope

The confirmed extension in docs/port-workflow-design.md adds a final native-port
workflow beyond the original M1–M4 mapping scope. Starting from a reviewed archive
and matching local ROM, prepare a separate Rust workspace and print whole-game
agent goals. Provide a small shared runtime library with ROM-backed extensible
assets, console cadence, selected platform effects, headless execution and an
optional SDL2 desktop frontend with player configuration.

The agent accounts for all original source behavior and implements gameplay
equivalents using modern techniques; obvious irrelevant paths may be excluded
with explanations, favoring ambiguous cases. Completion requires finished
accounting, a source-informed architecture plan and coherent maintainable code,
focused passing unit tests, required format/static/build checks and a documented
handoff. Runtime fidelity validation is outside the porting goal: no native-versus-ROM
frame/audio/state/scenario comparisons or exhaustive replays. Debugger use answers
a specific unresolved source question with a bounded experiment and stopping
condition; inconclusive results may lead to documented reasonable implementation
assumptions. Missing code blocks completion; unverified fidelity does not. After
the administrative accounting/build audit passes, stop. A fidelity/device audit
requires a separate user request. No line-by-line translation is required. The scaffold is not gameplay completion. Agents
may evolve the library directly in ROMAtlas, uncommitted, with an initial-file
snapshot and change log for separate review before user check-in. Keep original
assets ROM-backed at startup and out of distributions. Headless Linux validation
does not claim desktop/other-OS runtime compatibility. Library publication and
release packaging are later scope. See docs/porting.md for the working contract.

Mapping-phase redirect investigations follow supported connected handlers, state transitions and
related data before handoff, without byte quotas or exhaustive recursive closure.
A first ready lead or initial range boundary is not sufficient reason to stop;
record reasons for deferring obvious continuations. Static evidence may justify
code without branch-by-branch runtime execution; preserve provenance and bank context.
