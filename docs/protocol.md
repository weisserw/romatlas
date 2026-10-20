# Agent protocol (romatlas/1)

Run `romatlas serve`, or `romatlas serve --rom game.gb --store analysis.json`. Send one JSON-RPC 2.0 object per line on stdin. Read one reply per request from stdout. Logs use stderr. The process keeps its state until EOF. It runs requests in order. It does not support batches or positional parameters.

```json
{"jsonrpc":"2.0","id":1,"method":"api.describe"}
{"jsonrpc":"2.0","id":2,"method":"rom.open","params":{"path":"game.gb"}}
{"jsonrpc":"2.0","id":3,"method":"emulator.start"}
{"jsonrpc":"2.0","id":4,"method":"emulator.input","params":{"button":"a","pressed":true}}
{"jsonrpc":"2.0","id":5,"method":"emulator.run","params":{"operations":1000,"timeout_ms":1000}}
{"jsonrpc":"2.0","id":6,"method":"emulator.snapshot.save","params":{"name":"held"}}
{"jsonrpc":"2.0","id":7,"method":"emulator.memory","params":{"address":49152,"length":16}}
{"jsonrpc":"2.0","id":8,"method":"analysis.decode","params":{"offset":256,"count":1}}
{"jsonrpc":"2.0","id":9,"method":"map.annotate","params":{"offset":256,"length":3,"kind":"code","provenance":"manual","confidence":1}}
{"jsonrpc":"2.0","id":10,"method":"map.export","params":{"path":"game.sym"}}
{"jsonrpc":"2.0","id":11,"method":"disasm.generate","params":{"out":"build/game"}}
{"jsonrpc":"2.0","id":12,"method":"disasm.verify","params":{"generated":"build/game"}}
```

The annotation above is valid only if the local ROM has a complete three-byte instruction at offset 256. Check decode results first. Manual annotations assert structure; they do not prove execution.

Success: `{"jsonrpc":"2.0","id":12,"result":{"ok":true,"byte_identical":true}}`.
Error: `{"jsonrpc":"2.0","id":12,"error":{"code":-32000,"message":"open a ROM first"}}`.

## Calls

All byte offsets, lengths, addresses, IDs, counts, and budgets are JSON integers (decimal). Paths refer to the local filesystem. Physical ROM offsets do not equal CPU-visible addresses.

| Method | Parameters and result |
| --- | --- |
| api.describe | No parameters. Platform, protocol, method list, request and response byte limits. |
| rom.open | Required path. Returns cartridge metadata. Replaces ROM/map, stops emulator, clears snapshots. |
| rom.info | Current cartridge metadata. |
| rom.read | offset=0, length=16 (max 4096). Physical address and bytes. |
| rom.regions, analysis.unknown | Pages of unknown physical ranges. |
| emulator.start | model=auto (auto, dmg, cgb). Starts a fresh emulator; clears snapshots and traces. |
| emulator.reset | Resets emulator; clears snapshots. |
| emulator.step | operations=1, timeout_ms=1000. CPU operations, including HALT/STOP/speed-switch waits and interrupt service. Also accepts unit/count; see stepping below. |
| emulator.run | operations=1000, timeout_ms=1000. Returns registers, operation/PPU-cycle counts, effective fixed/switch banks. |
| emulator.state | Current registers and counts. |
| emulator.memory | address=0, length=16 (max 4096). CPU-visible mapped bytes, no time advance or state change. |
| emulator.input | Required button and pressed boolean. Buttons: a, b, start, select, up, down, left, right. |
| emulator.snapshot.save | Required name. Saves memory, CPU, peripherals, input, mapper and counters. |
| emulator.snapshot.load | Required name. Restores snapshot and returns state. |
| analysis.decode | offset=256, count=16 (max 4096). Decoded instructions, physical banks, next_offset. Stops at bank limits with an error for truncated instructions. |
| analysis.conflicts | Current structural overlap conflicts. Invalid edits are rejected, so a valid store has none. |
| map.get | Pages of region IDs, records and metadata (null when absent), including imported sym directives and explanatory fields. |
| map.annotate | Required offset, length, kind. Optional id edits an existing region. confidence=1, provenance=manual. Optional name, role, evidence, comment are strings; null removes a field. Returns id. |
| map.remove | Required id. Removes region. |
| map.validate | Checks canonical Game Boy region constraints and ROM identity. |
| map.save | Required path. Saves internal JSON atomically. This is not the distributable map. |
| map.load | Required path. Imports internal JSON only after hash and platform validation. |
| map.export | Optional path and include_sym (boolean, default true). With include_sym=false, requires path and returns only ok/path/bytes. Otherwise returns stock-compatible sym text and optionally writes it. |
| emulator.capabilities | No parameters. Requires emulator. Returns clock, step units, address spaces and supported controls. |
| emulator.test | max_operations=10000000, timeout_ms=10000 (max 30000). Runs the local Mooneye report protocol; returns status and state. |
| emulator.physical_memory | Required space. offset=0, length=16 (max 4096). Reads immutable backing storage; does not apply CPU access locks. |
| trace.configure | Optional enabled, capacity, kinds, space, range. Updates supplied fields; returns ok. See tracing limits below. |
| trace.summary | No parameters. Counts, retained/dropped detail, physical/contextual aggregates, stream status. |
| trace.query | offset=0, limit=100 (max 1000). Pages retained events in capture order. |
| trace.clear | No parameters. Clears event detail and aggregates; returns ok. |
| trace.stream.start | Required new file path. max_bytes=16777216 (max 67108864). Starts bounded local JSONL capture; returns stream status. |
| trace.stream.stop | No parameters. Stops current stream; returns final stream status or null. |
| analysis.coverage | offset=0, limit=100 (max 1000). Pages physical executed opcode addresses and hit counts. |
| analysis.edges | offset=0, limit=100 (max 1000). Pages physical observed successors and hit counts. |
| analysis.discover | entrypoints=[256], budget=1000 (max 100000), use_observed=true, offset=0, limit=100. Returns physical candidates, references and diagnostics. |
| analysis.discover_context | Required entrypoints with offset/cpu_context (1..1000). Same budgets/pages as discovery. Propagates CPU context and observed before/after banks. |
| analysis.references | Same traversal parameters as analysis.discover. Pages reference records. |
| analysis.pointers | offset=0, count=16 (max 4096 entries), layout=word-le, optional fixed_bank/switch_bank. References page with page_offset=0, limit=100. Returns hypotheses and conflicts. |
| analysis.structure | offset=256, required cpu_context with cpu_address and optional fixed_bank/switch_bank. Returns one instruction's physical successors, references and diagnostics. |
| map.import | Required path or sym text (max 1 MiB). Validates and replaces current map atomically; returns region count. |
| disasm.generate | Required out (new folder). Uses current ROM and map, with pinned stock mgbdis. |
| disasm.verify | Required generated folder. Links with pinned RGBDS and compares every byte. |

Pages use offset=0 and limit=100 (max 1000). Responses include items, total, next_offset. Map kinds: code, data, text, padding, image, unknown. Provenance: manual, static, observed. Explicit user-provided provenance is an assertion, not traced proof. Unknown gaps export as .data for fidelity, without becoming data claims internally.

## Budgets and errors

Each request line is at most 1 MiB, including newline. The server drains a larger line without growing its buffer, returns an error, then resumes with the next request. Run budgets allow at most one million CPU operations and timeout_ms from 1 to 30000. On timeout, completed operations remain in state. There are at most 16 named snapshots, each name at most 128 bytes. Named snapshots stay in memory and vanish on EOF or ROM change; use snapshot.export/import for local persistence.

Standard error codes: -32700 invalid JSON, -32600 invalid request, -32601 unsupported method/capability, -32602 params must be an object, -32000 operation or parameter error with a message. Requests without id are notifications and get no reply, even on errors. Use requests with id when results matter. The server recovers from malformed requests.

## Pending capabilities

Full peripheral timing remains incomplete; see docs/hardware-tests.md. Trace streaming is available. map.import reads stock .sym; map.load reads internal JSON. Unknown methods return explicit unsupported-capability errors. External disassembly commands now share a bounded deadline and captured-output limit. Do not use the current method list as proof that the full project brief is complete.

## Trace calls

Start the emulator, then call `trace.configure` with `enabled:true`. Optional fields: capacity (0..100000), kinds (array of Instruction, Fetch, Read, Write, ControlFlow, Interrupt, BankSwitch, Dma, Frame), space (string or null), range ([start,end) in physical offsets, or null). Filters affect event detail, not aggregate counts. Tracing starts disabled. Disabling it stops detail and aggregate capture; it does not erase prior results.

`trace.summary` returns retained/dropped events, per-kind counts, unique instruction/edge counts, and aggregate_dropped. Coverage and successor aggregates each retain at most 100000 unique keys. `trace.query` uses offset/limit pagination (limit max 1000). `trace.clear` clears detail and aggregates. `analysis.coverage` pages addresses and hit counts. A zero detail capacity still allows bounded aggregation.

Instruction events come from actual opcode bus reads; Fetch covers operand/CB reads. Read and Write describe real CPU data accesses. ROM reads carry physical offset/bank and CPU address. Controller writes use cartridge-controller space, not ROM-write claims. BankSwitch records each controller write, even when its effective switch bank stays the same; context bank_offsets and target show before/after switch-window translation. fixed_bank_offsets records fixed-window translation before/after each controller write. Non-ROM accesses now carry physical bank-aware identities; cpu_address retains the bus alias.

ControlFlow records observed operation successors, including ordinary fallthrough. Context includes edge_kind (fallthrough, jump, call, return, interrupt-return, restart, indirect-jump, halt, stop, invalid or unknown), conditional, taken, encoded_cpu_target, fallthrough_cpu and traversable. Actual fetched bytes and pre-operation flags determine the kind and taken status. HALT/STOP/invalid/unknown edges are not traversable. Relative targets account for repeated fetch addresses from the HALT bug. Interrupt events have no opcode fetch. Frame and OAM DMA byte-transfer events are available; CGB general/HBlank DMA byte events are also available. No debugger inspection enters the event stream. Event context holds pre-operation registers, and cycle marks the bus access before the next machine-cycle tick. Trace results are local runtime data, not shareable mappings.

Snapshots restore emulator state and cycle counters, not the trace history. Thus replay adds events with earlier cycles at the end of the capture order. Clear traces before replay if you need a single forward timeline. Reset or emulator.start clears the trace and disables capture.

## Structural discovery

`analysis.discover` takes entrypoints (physical offset array, default [256]), budget (1..100000 instructions, default 1000), use_observed (default true), offset, and limit. When enabled, it adds captured ROM opcode addresses as roots and marks those starts Observed. Other starts carry Static provenance. It returns candidates, references, diagnostics, totals, visited count, and budget_exhausted. It does not edit the map. Diagnostics/reference detail uses limit; candidate detail uses offset/limit. `analysis.references` uses the same traversal options and pages all reference records.

Confirm candidates with map.annotate, passing physical offset, decoded length, kind code, confidence, and provenance. Candidates are per instruction; adjacent instructions can be kept separate. Unknown bytes stay unknown internally. The traversal stops at invalid/truncated encodings, overlap with another instruction, non-code annotations, unclear bank targets, possible controller writes, indirect jumps, and HALT/STOP continuations. A budget stop does not classify remaining bytes.

For explanatory annotations, pass strings such as `name="CopyTiles"`, `role="VRAM copy routine"`, `evidence="observed reads and VRAM writes"` and `comment="reviewed"`. These live in internal region metadata and survive map.save/load. map.get exposes the complete metadata object. Names export as stock symbols at region starts; duplicate names or label locations and invalid symbols are rejected transactionally. Role/evidence/comment stay internal because stock .sym has no round-trip schema for them. Omitted fields preserve metadata on edits with unchanged offset/length/kind; null clears an individual field. Changing offset, length or kind discards prior metadata (including imported directive options) before applying supplied fields, so evidence for the old extent is not silently reused. Rejected edits preserve the entire map.

Bank rules support conventional ROM-only, MBC1, MBC3 and MBC5 windows. MBC3/MBC5 lower-window targets resolve to physical bank zero. A switchable-window instruction can resolve same-bank targets; fixed-window instructions need bank context or observed edges to select an upper-window target. MBC1 fixed-window alias banks require explicit CPU context and are rejected conservatively. Conventional MBC3/MBC5 windows also have regression tests. Physical bank 0 roots use the fixed CPU window; other roots use the switchable window. Default discovery keeps ambiguous mapper changes unresolved. Context-aware traversal handles explicit and observed bank state. Use analysis.discover_context for effective bank context and observed before/after state. Observed coverage adds roots and successor aggregates add traversal targets. Use analysis.pointers for explicit pointer-table hypotheses. A bare 16-bit value never selects a bank on its own.

## Importing community maps

`map.import` accepts path or sym (text) and replaces the current map only after all checks pass. Imports have Manual provenance. It preserves label names, directive aliases, data widths, text arguments, and image options in internal platform metadata. `.sym` export never copies the internal hash, confidence, or other sidecar metadata. Import does not alter emulator state or snapshots.

Save imported maps with map.save, then restore them with map.load. A rejected import leaves the previous map unchanged. Structural edits (offset, length, kind) replace imported directive options with defaults; confidence/provenance edits retain them. Region removal does not remove independent labels. External charmaps remain separate resources and CLI generation does not yet load them.

Stock mgbdis substitutes RAM/I/O label references but does not define those labels. ROMAtlas adds a small romatlas-symbols.inc with EQU definitions from `.sym` metadata. For stock-only reproduction, pass undefined RAM/I/O labels to rgbasm with `-D 'wBuffer=$c000'`, as shown in docs/sym.md. Local RAM labels need global names. RGBDS linker checks still compare all input bytes.

## Cycle and frame control

emulator.step and emulator.run accept unit (`instruction`, `operation`, `cycle`, `frame`). Operation mode uses count instead of operations. Instruction mode keeps the operations parameter. Cycle/frame mode uses count=1, max_operations=1000000, and timeout_ms=1000; do not also pass operations. Cycle count is at most 10000000; frame count is at most 600. Invalid budgets fail before execution. Exhausted budgets retain completed state.

Cycles mean PPU clock ticks (DMG T cycles). Execution completes whole CPU operations, so a cycle target can overshoot by one operation. Frame stepping stops after real PPU VBlank entries, not a guessed 70224-cycle span. State includes frames and ppu_cycles. With the LCD off, no frames occur; the operation/time budget stops the request. Frame snapshot replay reproduces state and frame count.

Dma events describe each OAM DMA source byte and destination CPU OAM address. Frame events mark actual VBlank boundaries. Fetch tagging uses the recorded read index, so peripheral events from the same machine cycle cannot become false opcode events. Tests check all 160 OAM transfers and equal final hardware state with tracing on/off.

`emulator.test` runs the Mooneye report protocol in the current emulator. max_operations defaults to 10000000 (max 10000000), timeout_ms defaults to 10000 (max 30000). It returns passed/failed/timeout/operation_limit and state; no timeout counts as a pass. This call serves licensed local test ROMs, and leaves state available for memory inspection.

DMG execution now supports ROM-only, MBC1, MBC3 (types 0f–13), and MBC5 (types 19–1e). MBC5 switchable bank zero and the ninth bank bit are supported. CGB execution now uses auto/dmg/cgb model selection. Conventional MBC3/MBC5 static windows have regression coverage; aliases and mapper changes require context-aware traversal.

## Deterministic MBC3 clock

The RTC starts at day 0, 00:00:00. It advances from emulated time, including STOP waits; host time and time between API calls do not change it. Game code selects/writes RTC registers through the normal bus. A 0-to-1 latch write captures a stable copy while the live clock runs. Halting freezes the clock; clearing halt resumes it. Day carry stays set until game code clears it. Invalid RAM/RTC selections return open-bus ff and ignore writes instead of crashing.

Snapshots include the live clock, latch, halt/carry bits and fractional-second phase. Version 4 added the RTC payload; the current fork snapshot format is 8, which also saves held joypad buttons. Earlier snapshots (including format 7, which omitted held buttons) are rejected; recreate them with the current binary. Internal map JSON and portable .sym formats are unchanged. MBC30/oversized MBC3 images remain explicit unsupported cases. No MBC3 case is included in the public Mooneye gate; its coverage currently comes from local CPU-bus tests.

## CGB model and DMA

emulator.start accepts model (`auto`, `dmg`, `cgb`; default auto). Auto uses the CGB cartridge flag. CGB-only cartridges reject DMG mode. Reset keeps the selected model. State adds model, wram_bank, vram_bank and double_speed; ppu_cycles always counts PPU ticks, including double speed.

CGB tests cover effective WRAM bank 0 aliasing to 1, echo RAM, VRAM banks, KEY1 switching, 16-byte general/HBlank DMA blocks, LCD-off HBlank waits and snapshot replay. Dma events give each source/destination byte. Non-ROM event addresses use physical spaces and retain CPU aliases in context. Model selection does not claim full peripheral accuracy: speed-switch settling delay is not yet modeled, and broader CGB game tests remain pending.

## Physical memory and access identities

`emulator.physical_memory` reads space, offset=0 and length=16 (max 4096). Supported spaces: cartridge-rom, cartridge-ram, wram, vram, oam, hram. Offsets index backing storage, independent of the current CPU bank and read locks. Requests outside real storage fail; DMG WRAM/VRAM each expose 8192 bytes. These reads are immutable and create no execution evidence. Use emulator.memory for CPU-mapped I/O and RTC reads.

Bus events use the same physical spaces. bank context gives the active bank. WRAM/VRAM/cartridge-RAM offsets index the whole storage; OAM/HRAM offsets start at zero. Echo RAM resolves to the same WRAM identity. I/O trace offset 0 denotes ff00 and ff denotes ffff; cartridge-rtc offsets 0..4 denote the selected clock register. I/O uses mapped-register reads; cartridge-rtc now exposes five immutable live/latched register values through physical_memory when present.

Event context retains cpu_address, cpu_target for DMA, mapped, and blocked. Blocked reads/writes retain CPU space and never claim a physical byte. Disabled RAM and invalid RTC selections stay unmapped. CPU mode-3 VRAM reads return ff and writes do not alter storage; DMA uses its own destination access path. Tests poll STAT and verify these blocked operations plus unmodified backing data.

BankSwitch covers cartridge-controller writes and CGB WRAM/VRAM selectors. bank_offsets uses offsets in target.space; fixed_bank_offsets applies to ROM-controller writes. Selection writes can leave the active bank unchanged. Blocked controller writes create no bank-switch evidence. Actual instruction fetches and successor edges use physical identities, including execution from RAM.

## Observed successors

`analysis.edges` pages source, target, hits and Observed provenance from the bounded aggregate store. It works even with zero event-detail capacity. These are observed traversable operation successors, including fallthrough. HALT/STOP/invalid/unknown successors do not enter this aggregate. Query ControlFlow event detail for branch/call/return kinds and taken status.

When use_observed=true, discovery merges those edges with static successors. Each observed link appears as an observed-successor reference with hits/provenance in context. A target PC alone does not prove an opcode fetch: its candidate remains Static unless coverage includes that address. Existing data annotations and conflicting instruction boundaries still block traversal. Invalid or unsupported source decoding still returns diagnostics instead of forcing a code claim. use_observed=false ignores both coverage and edges. Detailed ControlFlow events retain source/target CPU addresses and ROM banks before/after the operation for later CPU-window analysis.

## External tool budgets

disasm.generate and disasm.verify accept timeout_ms (default 120000; valid range 1..300000). The CLI generate/verify commands accept --timeout-ms with the same rules. Commands in each call share a deadline. Unix process groups let a timeout stop make and its compiler descendants; stdin is closed and stdout/stderr captures each retain at most 65536 bytes while continuing to drain pipes.

Generation builds in a unique private staging directory. It validates the map against the ROM, emits source, builds with pinned RGBDS, and compares every byte. It moves the project to the requested output only after these checks pass, then removes staged inputs. A mismatch reports the first bank/address and asks the agent to revise the map or use raw data. It removes the check object and rebuilt ROM before publishing. Failure/timeout leaves no partial public output. It still refuses an existing output directory. Verification rebuilds game.o instead of trusting a cached/partial object, links without rgbfix, and compares original bytes. Tests cover noisy errors, waiting descendants, shared deadlines, rejected zero budgets and partial-object recovery.

## Pointer hypotheses

`analysis.pointers` reads a physical ROM offset and count of little-endian 16-bit words (default 16, max 4096). The range must stay in one physical bank. Optional fixed_bank/switch_bank declare effective physical ROM bank context; table location never supplies a switch bank by itself. ROM-only windows and known fixed MBC3/MBC5 bank zero can resolve without a hint. Unknown banked targets stay unresolved.

Results include pointer-candidate references, resolved/ambiguous/invalid/non-ROM counts, unique targets, diagnostics, existing conflicting region IDs, and an optional Static Data candidate with confidence 0.5. A candidate needs at least two resolved ROM references, at least 80% ROM resolution and multiple targets. This is a hypothesis, not proof. Repeated zero words do not create a candidate. Non-ROM values keep CPU-visible targets; they do not select physical RAM banks.

Supported layouts:

| layout | Entry bytes | Width |
|---|---|---|
| word-le (default) | CPU address low, high | 2 |
| bank8-word-le | Bank, CPU address low, high | 3 |
| word-le-bank8 | CPU address low, high, bank | 3 |
| bank16-word-le | Bank low, high, CPU address low, high | 4 |
| word-le-bank16 | CPU address low, high, bank low, high | 4 |

Bank fields mean effective physical ROM banks, not raw mapper-register values. A 16-bit bank field retains MBC5 bank 256 and above. Encoded banks outside the ROM, nonzero fixed banks on ROM-only/MBC3/MBC5, ROM-only upper banks other than 1, or banks that conflict with fixed_bank/switch_bank remain unresolved and count as invalid_rom. Non-ROM addresses retain CPU space; an encoded bank does not select physical RAM. Each reference includes layout, width, encoded_bank and bank_encoding. Results include entry_width; count and total count entries for every layout. Candidates cover the full entry width. Other layouts need an explicit future adapter; do not reinterpret register-bank encodings as effective banks.

Example: `{"offset":512,"count":8,"layout":"bank8-word-le"}` tests eight bank-first entries.

References page with page_offset and limit (max 1000); offset still means the table's physical start. The call does not edit maps or add execution coverage. Confirm an inspected candidate with map.annotate. Do not label its targets as code solely because pointer words name them. Set layout explicitly for banked entries; the toolkit never guesses a layout.

On a CPU execution error, state retains actual bus time. operations counts attempted CPU operations, including the failed attempt. Illegal opcode reads appear as Fetch evidence and never become Instruction coverage or successor edges. Retrying the same failed PC advances timestamps; snapshots retain that elapsed time.

`emulator.capabilities` requires an active emulator and returns platform, clock, supported step units, address-space sizes/context, and snapshot/input/instrumentation support. The Rust ExecutionAdapter owns associated state/configuration types; the JSON session converts its existing instruction/cycle/frame controls into typed requests. Operation requests also enforce max_operations before execution. Capabilities keep the loaded image size even if its source file disappears; reset still needs that source file.

Physical cartridge-rtc inspection reads the five clock-register values without changing the selected RAM/RTC bank or clock state. The typed inspector supports mapped I/O register addresses and rejects HRAM holes within that namespace. Current read-address translation returns no physical location for blocked/unmapped CPU accesses.

Capability units serialize as operation/cycle/frame. The session accepts operation with count; its legacy instruction alias still uses operations. Rejected operation budgets leave hardware state unchanged.

## Local trace stream

trace.stream.start accepts path and max_bytes (default 16777216, max 67108864). It creates a new file; existing paths and a second active stream fail. Enable trace.configure separately. Matching events stream even with memory capacity zero. Filters apply to both stream and memory detail; aggregate counts still cover all events.

Each JSONL record contains sequence and event. Sequence starts at zero and increases per complete record. trace.summary adds stream status: records, bytes, max_bytes, dropped, stopped and error. trace.stream.stop closes the file and returns its final status. The file cap stops before a complete record would exceed the limit; later matching records increment stream dropped. A 65536-byte per-record limit bounds serialization memory. I/O or serialization errors stop only the stream and appear in status; CPU execution continues. An I/O error can leave an incomplete final line: use the completed-record count and ignore an unterminated tail.

Disabling capture pauses writes. trace.clear clears in-memory detail/aggregates but leaves file position and sequence intact. Reset/reopen closes the old stream. Snapshot restore leaves the stream active: event cycle values can rewind, while sequence remains monotonic. These local traces contain actual bus values and stay separate from portable .sym exports.

## Explicit CPU-window decoding

`analysis.structure` decodes one instruction without running the emulator. Supply physical `offset` and `cpu_context` with `cpu_address`, optional `fixed_bank`, and optional `switch_bank`. Banks mean effective physical banks. The source CPU address and selected bank must match the physical offset. Controller-fixed windows reject invalid banks. Unknown target windows remain unresolved.

Example: `{"offset":524288,"cpu_context":{"cpu_address":0,"fixed_bank":32,"switch_bank":33}}` decodes an MBC1 fixed-window alias. MBC5 bank zero can also occupy the upper window when `cpu_address` is in 4000–7fff and `switch_bank` is zero. Reference context retains the source CPU address and asserted banks. The result includes length, physical successors, references and diagnostics. Controller writes still stop fallthrough. This call does not edit maps or coverage. Use analysis.discover_context to propagate explicit contexts through static successors.

`analysis.discover_context` accepts `entrypoints` (1..1000 objects with offset and cpu_context), budget (1..100000), offset and limit (max 1000) for candidate pages. It carries fixed/switch banks and successor CPU addresses through each static path. Traversal visits distinct CPU contexts separately, but candidates, coverage tests and data barriers use canonical physical ROM addresses. References retain the asserted CPU context. The result has the same candidate/reference/diagnostic summaries as analysis.discover.

Example: `{"entrypoints":[{"offset":524288,"cpu_context":{"cpu_address":0,"fixed_bank":32,"switch_bank":33}}],"budget":1000}`. Controller writes stop static traversal. An observed contextual edge can cross the write; without that evidence, supply a new root with the bank state after it. With use_observed=true (default), this call adds recorded context roots and merges contextual observed edges. Each edge retains source bank state before the operation and target bank state after it. This lets traversal cross an observed controller write without reusing the old bank state. use_observed=false disables both coverage and edge input. Context-free observed edges still belong to analysis.discover. An observed target alone never counts as executed coverage. Neither discovery call edits the map.

Context aggregates retain up to 100000 roots and 100000 edges, independent of event-detail filters and capacity. trace.summary reports unique_contextual_roots and unique_contextual_edges; aggregate_dropped includes their overflow. trace.clear clears these aggregates. ControlFlow detail includes traversal_source/traversal_target with opaque traversal context when the source/target maps to ROM. HALT/STOP retain fetched source roots but do not add traversable edges. Failed opcodes add no context roots. Snapshots can rewind execution while traces retain earlier observations; context keys keep distinct bank states separate. Context roots and edges remain local runtime data and never enter exported .sym files.

Metadata file reads are bounded before parsing. CLI .sym reads and map.import path reads allow at most 1 MiB. Internal map.load/Store::load files allow at most 64 MiB. Reads also stop if a file grows past the limit after opening. Metadata inputs must be regular files. An oversized import/load leaves the current session map unchanged. These file limits are separate from the 1-MiB JSON request limit.

Internal CPU wait cycles use a clock-only bus operation. HALT/STOP waits, speed-switch pauses and internal instruction/interrupt cycles advance hardware time without adding a data read at ROM offset zero. Actual frame and DMA events can still occur during waits. Query emulator.state for elapsed time when a wait produces no trace detail.

CGB palette data accesses denied during rendering carry blocked=true and mapped=false in bus traces. A denied data write can still advance the palette index. Read the index registers through emulator.memory to inspect that effect; inspection itself does not advance it.

Coverage, edge, map, reference and unknown-range pages select records before converting them to JSON. Large offsets, empty pages and zero limits do not convert skipped records. Unknown-range queries still scan/sort structural ranges, and reference queries still perform bounded discovery. Totals and next_offset keep the same meaning.

rom.open fixes the loaded image identity for the session. Source-file operations (emulator.start/reset and disasm.generate/verify) reject changed content. Reopen the ROM to accept a new image; that clears the old map, emulator and snapshots. A rejected operation keeps the existing state. Cached ROM inspection, map export and existing-emulator snapshot replay still refer to the loaded image. The emulator also checks the cartridge hash after loading to detect changes between its two reads.

JSON response payloads have a 1-MiB byte limit (excluding the line delimiter). api.describe reports max_response_bytes. The manager measures encoded JSON, including escaped characters, without first building a second response buffer. Oversized replies return error -32001. The command may already have completed: inspect state before retrying a write. An ID too large for the error reply becomes null. Notifications still produce no reply.

For large maps, use map.export with {"path":"game.sym","include_sym":false}. This writes the complete stock map but returns only a small file summary. Smaller pages/ranges can reduce other replies. The byte limit does not replace execution, trace or metadata-input limits.

map.save and map.export reject output paths that resolve to the input ROM, including Unix hard links. CLI map init and store export use the same check. A rejected alias does not write the output or alter ROM bytes.

Debugger edits: `emulator.registers.set` accepts a nonempty `registers` object
with any of a/f/b/c/d/e/h/l (0..255), pc/sp (0..65535). F's low nibble must
be zero. Unknown names and invalid values reject the entire register edit.
`emulator.memory.write` requires `address` and a `bytes` array of 1..4096 integers
in 0..255, entirely within the 16-bit CPU address space. It performs mapped,
unticked bus writes, including I/O and mapper side effects and hardware access
locks; it does not patch ROM backing storage. Writes may be ignored by hardware.
Neither command advances time or emits execution/coverage events. Existing trace
history remains historical evidence; subsequent execution uses the edited state.

`emulator.framebuffer` returns the core's current 160x144 RGB8 framebuffer as
`pixels` (row-major RGB byte array), with frames, ppu_cycles, operations and
image_sha256. Inspection does not advance execution. This is the current render
buffer; a mid-frame capture may contain pixels from different frames.

`analysis.discover_context` and `analysis.discover` accept independent
`reference_offset` and `diagnostic_offset` (default 0). Each list uses `limit`;
responses include next_reference_offset and next_diagnostic_offset alongside
its total. Candidate offset semantics are unchanged. Repeated queries recompute
discovery; keep maps, inputs and observed traces unchanged while paging.

`emulator.snapshot.export` requires a new `path` and writes the current machine
snapshot as local JSON. `emulator.snapshot.import` requires `path`, restores it
and returns state. Files carry a format version, ROM SHA-256 and hardware model;
incompatible identities/models/formats and malformed core states are rejected.
Imports are capped at 64 MiB. Format v1 is tied to the current core save schema;
future schema changes must bump the envelope version. CPU, memory, peripherals,
mapper, input and counters persist across sessions. Trace history/configuration
is not restored; existing history remains, as with named snapshot restoration.

Contextual discovery tracks literal BC/DE/HL values along straight-line paths
through immediate pair loads, pair increments/decrements and LD post-increments.
It invalidates affected pairs on byte-register loads and forgets facts on
unsupported effects and control transfers. Arrivals with different facts remain
separate under the existing traversal budget. A proved indirect destination in
8000..ffff preserves ROM bank context; uncertain/controller writes still stop.
Derived facts are traversal state only, excluded from portable regions and
execution coverage. This is bounded literal tracking, not general interval analysis.

`analysis.discover_context` accepts `call_profiles` (default [], max 1000).
Each profile requires image_sha256, offset, cpu_context (explicit fixed_bank and
switch_bank), source_bytes (exact unconditional CALL/RST instruction bytes),
inline_bytes (exact following bytes, possibly empty), evidence (nonempty durable
reference), all_arrivals=true, and returns_after_inline (boolean). Source plus
inline extent is capped at 4096 bytes within one physical bank. Duplicate profiles,
identity/byte/context mismatches and conditional/non-call sites are rejected.
Profiles suppress the presumed return continuation only for the matching source
and CPU/bank context; the dispatcher/callee remains a discovery successor.
With returns_after_inline=true an explicit continuation follows the inline extent;
otherwise the profile asserts no ordinary return. Inline extents do not annotate
the map or infer selector domains/handler execution. Inspect pointer hypotheses
separately and annotate confirmed tables as Data to protect other traversal roots.
The caller must supply evidence covering all arrivals in that exact context;
reachability alone is insufficient. Profiles are local query inputs, never stock
map directives or execution evidence. No restart number has special defaults.

## SNES platform

For the SNES method parameters, address spaces, processor context, portable JSON
mapping and measured limits, see [snes.md](snes.md). Use `serve --rom owned.sfc`
or `serve --platform snes` and inspect api.describe. Game Boy parameters above
remain unchanged. Shared transport/request/response limits apply to both adapters.

## NES adapter

Use `romatlas serve --rom owned.nes` or `--platform nes` for another extension.
The NES adapter implements the shared persistent transport through CommandAdapter;
`api.describe` gives the precise method list. CPU names are main/6502/2a03.
Clock values are NTSC CPU cycles. See [NES methods, address rules and limits](nes.md)
for stepping, actual-access traces, effective PRG bank contexts, immutable peeks,
versioned snapshots, maps, 6502 analysis, frame/WAV output and reconstruction.
NES portable maps use JSON/ca65; Game Boy .sym/RGBDS and SNES JSON/Asar stay separate.

## Sega Master System

Use `romatlas serve --rom owned.sms`, or `--platform sms` for another extension.
The same persistent CommandAdapter transport exposes the SMS method set through
`api.describe`. See [SMS protocol details](sms.md). Operation stepping advances a
core instruction/interrupt/HALT quantum; cycle stepping ends at an instruction
boundary. Context uses effective 16 KiB `rom_banks`, optional `ram_enabled`,
`ram_bank` and `cartridge_enabled`. File offsets include any copier prefix.
`emulator.translate` resolves a CPU address to its current backing identity.
`emulator.memory` supports mapped CPU, detached I/O peeks and backing inspection;
`physical_memory` requires a backing space. Explicit debugger I/O/VRAM/CRAM/RAM
writes and register edits mark subsequent evidence artificial. VDP data-port
prefetch/write hooks report actual VRAM/CRAM accesses separately from the buffered
I/O return value. Rendering reads are not a cycle-exact PPU bus trace.
Maps use `romatlas-sms-map` JSON; source uses pinned WLA-DX, separate from other
platform formats. No new structural/reachability/editorial requirements are added
to Game Boy, NES or SNES by this profile.

## Game Gear platform

Use `serve --rom owned.gg` or `--platform gg` for another extension. The shared
Sega methods above apply with GameGear hardware, one controller and Start,
64-byte CRAM, stereo PSG and a 160x144 framebuffer. Start polls I/O $00; it does
not cause SMS Pause NMI. Map format is romatlas-gg-map; source uses WLA-DX.
See [Game Gear protocol and limits](gg.md) for contexts, snapshots and workflows.

## Genesis / Mega Drive platform

Use `serve --rom owned.md` or `--platform genesis` with another extension.
Genesis provides the common CommandAdapter method set, ROM-free Genesis JSON
maps, 68000 and Z80 register/memory control, bounded traces/streams, snapshots,
framebuffer/WAV capture and GNU/WLA reconstruction. `cpu:"main"`/`"m68k"` selects
the 68000; `cpu:"z80"` selects the sound CPU. Cycle units are master clocks;
operations count accepted instructions of the selected CPU while all devices advance.
Use `z80_cpu` for mapped sound-bus peeks and physical `zram` for its backing.
Pointers are big-endian 32-bit or explicitly based signed 16-bit for main, and
little-endian 16-bit for Z80. Canonical ROM backing and original-file encoding
are distinct; do not mix SMD file positions with CPU addresses. See
[Genesis API details and limits](genesis.md). Existing platform contracts remain.

CGB hardware also exposes immutable `cgb-bg-palette` and `cgb-obj-palette` physical spaces (64 bytes each; size zero/unavailable on DMG). Reads bypass CPU palette-port locks without modifying the index or adding bus evidence. See [gbc.md](gbc.md).
