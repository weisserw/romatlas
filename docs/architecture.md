# Architecture and limits

ROMAtlas keeps shared records and control logic separate from the Game Boy implementation. Addresses contain a named space, a u64 offset and opaque context. CPU-visible aliases do not replace physical ROM identity.

## Workspace

| Crate | Role |
|---|---|
| romatlas-core | Typed addresses, regions, references, events and adapter contracts |
| romatlas-gb | Cartridge parsing, SM83 decoding/execution, bus/peripherals, banking, Game Boy discovery and mgbdis/RGBDS backend |
| romatlas-analysis | Trace summaries, filters, bounded buffers/streams and shared traversal |
| romatlas-map | Shared internal Store; Game Boy stock .sym parser/export type |
| romatlas-disasm | Shared backend contract, process deadlines and byte comparison |
| romatlas-agent | Shared JSON-RPC session manager and request/response limits |
| romatlas-cli | CLI and Game Boy application setup |

ImageAdapter and DecodeAdapter provide image layout and structural decoding. ExecutionAdapter owns typed step requests and capabilities; state, inputs, snapshots and configuration remain adapter-owned associated types. CommandAdapter routes commands. DisassemblyBackend owns its mapping type and generator/verifier. Wide-address and mock-backend tests check these contracts without Game Boy imports. The dependency check also runs in CI.

## Core choice and maintenance

The headless core is an MIT Mizu 1.3.0 fork, based on revision ece2c0d40e1f2984a258410b6a6188418cd3e1c6 (December 2025). Only core/save-state source and notices are vendored. No boot ROM, commercial ROM, external ROM fixture or frontend is included. Emulation uses Rust, not a C/C++ runtime core. Zstd compresses snapshots.

Mizu supplies instruction stepping through a timed bus, explicit interrupt/HALT state, mapper state and save-state derives for CPU/bus/peripherals. Delayed IME and HALT mode enter snapshots. Its bus access methods permit actual access tracing. The small maintainer base makes local tests and a fixed source revision necessary; continuing upstream maintenance is not assumed.

The earlier assessment compared Boytacean 0.13.2 (Apache-2.0), whose inspected timer used interval counters, and Ceres (MIT), whose public API focused on frames and battery saves. Mizu required fewer changes for precise stepping and complete snapshots. The rboy assessment did not establish its save-state/license fit.

The fork adds register/storage inspection, physical address translation, bus hooks, deterministic RTC, frame/DMA tracking and timing fixes. Internal CPU cycles clock hardware without creating data reads. Logs use stderr. Battery persistence is off. Peeks do not tick hardware or add trace evidence; replay tests check unchanged state. Physical reads bypass mapped CPU access locks.

## Mapping and analysis

The internal JSON Store records image hash, typed regions, stable IDs, confidence, provenance and opaque platform metadata. Rejected edits/imports leave the prior store intact. Game Boy validation checks physical ranges, banks, instruction boundaries, directives, padding, image sizes and ROM contents.

Portable export contains only stock .sym labels/directives/options. It excludes ROM bytes, runtime state, trace values and internal confidence/hash metadata. Unknown gaps use lossless .data output while remaining unknown internally. Stock-compatible aliases and options survive import/export. See docs/sym.md for syntax, RAM/I/O labels and resource limits.

Shared discovery combines decoded successors and observed edges under work limits. CPU-window traversal records fixed/switch bank state before and after changes, retains unresolved cases and respects data barriers. A reached target is not opcode coverage. Pointer analysis requires an explicit layout and adequate bank context; it returns hypotheses and does not edit the map or claim executed code.

## Execution and traces

ROM-only, MBC1, MBC3 and MBC5 common variants have runtime tests. MBC5 includes the ninth ROM-bank bit, RAM banks and rumble-bit masking; no motor is attached. MBC3 RTC starts at zero and follows emulated oscillator ticks, not host time or battery files. Latches, halt/carry bits and fractional time enter snapshots. Oversized MBC3/MBC30 and other unsupported controllers produce errors.

The model defaults from the cartridge flag. Explicit DMG/CGB choice survives reset; CGB-only images reject DMG execution. State includes effective WRAM/VRAM banks and speed. Tests cover CPU/interrupt/timer/HALT behavior, bank aliases, echo RAM, DMA and replay. Source-built tests currently pass all 89 default gate cases and the separate 55-case CGB selection. These selections do not prove compatibility with every cartridge or hardware revision.

Instruction, fetch, read, write, control-flow, interrupt, bank, DMA and frame records use bank-aware locations and CPU aliases. Denied accesses do not become physical ROM/RAM claims. Capture can be disabled. Detail buffers, aggregates, traversal and streams have limits. Streaming writes complete bounded records and stays outside hardware snapshots; replay can rewind virtual cycles while stream sequence continues.

Snapshot format is version 7. It includes pending sprite-fetch state and frozen speed-switch video access. Older external core states fail the version check. Session replay uses snapshots from the same running binary. No serial/link/printer device is attached; external-device state is outside supported replay.

CGB tests cover banks, KEY1, general/HBlank DMA, palette data access and CPU/video speed-switch pauses. The 2050-M-cycle pause freezes DIV and holds video access state. Synthetic pixel tests cover all four starting PPU modes. Palette writes denied during rendering preserve data but still advance an enabled index. Full peripheral timing, native-color game coverage and early CGB revisions remain unproven.

## Generation and verification

Rust is pinned to 1.90.0. Workspace Cargo.lock pins runtime/build dependencies; the standalone core lock pins its unit-test dependencies. mgbdis uses MIT revision 95d951a9881a0d7ff1a42a94d1be175da92b1777. RGBDS 0.9.4 uses MIT revision d1829ed92327a9f9210ffd45d865331f24dfa4e6. Setup retains upstream licenses, bundled Python PNG code and hardware notices.

The Game Boy backend uses stock mgbdis emission. Generation checks map contents, emits to a private staging directory and publishes only after a byte-identical RGBDS rebuild. External commands share a deadline, own a Unix process group and drain bounded output. Verification rebuilds the object instead of trusting a stale artifact, then reports the first mismatch with bank/address context.

Stock make runs rgbfix, which changes headers. Exact verification instead builds game.o and links without rgbfix. Clean-recipient tests use the same stock commands. Invalid headers and arbitrary unknown bytes therefore survive. Source comments retain the ROM filename; deterministic output assumes the same filename.

Stock mgbdis can reference non-ROM labels without defining them. Generation adds metadata-only EQU definitions after hardware.inc. A recipient using only stock tools can supply these values with RGBDS -D. This tested fix does not replace the assembly emitter.

## Operating limits

- Generation accepts complete 16-KiB banks, at least 32 KiB. Emulation additionally requires a valid header checksum and a title its core can parse as UTF-8.
- Invalid instructions, nonstandard STOP encoding, lossy directives and unsupported external charmaps require corrected annotations or raw data. External charmap files are not loaded by the generator.
- Default bank decoding is conservative. Supply CPU-window context for aliases and banked pointer hypotheses. A 16-bit literal alone does not establish a ROM bank.
- Metadata inputs, JSON messages, trace detail/aggregates, stream files, traversal, peeks and runs have separate documented limits. A response-budget error can follow a completed command; inspect state before retrying writes.
- Source-file operations check loaded ROM identity. Reopen a changed image to replace its map/runtime context. Map/state output aliases cannot overwrite the input ROM.
- External tool paths resolve from the source checkout. Keep it with the binary. Unix process-tree cancellation is the supported timeout model.
- Hardware support is incremental. The public gates, synthetic tests and two pinned licensed games provide measured evidence, not full commercial-ROM or peripheral compatibility.

See README.md for setup/commands, docs/protocol.md for bounds and errors, and docs/hardware-tests.md for test selection. See docs/completion-audit.md for the final requirement review and docs/requirements-review.md for its history.
