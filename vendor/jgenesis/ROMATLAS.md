# ROMAtlas core fork

Upstream: https://github.com/jsgroth/jgenesis
Revision: 7b3cc06196f9cb9ac5cb067ffb3cdbc8c4efe282 (v0.11.4).
License: GPLv3; see LICENSE. Preserve all upstream attribution.

Only core/config dependencies are included. No frontend, commercial game ROM,
external coprocessor firmware, extracted game assets or binary fixture is included.
Unsupported enhancement cartridges are rejected by ROMAtlas before core creation.
The minimal workspace manifest pins the Rust 1.90 compatible time dependency.

Local changes: deterministic initial WRAM, typed optional bounded instrumentation,
actual opcode hooks, immutable debugger inspection and state access. Instrumentation
is outside hardware snapshots. The adapter owns identity/versioning and transactional
restore. See docs/snes.md for capabilities and measured limits.


NES extension (same upstream revision): backend/nes-core, backend/ym-opll,
config/nes-config and cpu/mos6502-emu. Local changes initialize CPU RAM to zero,
observe actual opcode/bus/backing accesses with optional bounded probes, expose
immutable baseline mapper inspection, detached PPU peeks, backing storage and
CPU register access. The adapter owns cycle/frame counts, ROM identity,
transactional snapshots, effective bank views and output limits. See docs/nes.md.
No GPL implementation was copied into the independent MIT romatlas-port helpers.

SMS extension (same pinned revision): backend/smsgg-core, config/smsgg-config,
cpu/z80-emu. Local hooks expose architectural/shadow CPU registers, deterministic
backing inspection/edits, actual first-fetch/memory/I/O/interrupt probes and T-cycle
progress. SMS adapter owns contexts, snapshots and output budgets. Upstream
instruction-driven timing remains unchanged; no cycle-exact hardware claim.
