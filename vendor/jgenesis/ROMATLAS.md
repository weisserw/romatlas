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

Game Gear extension (same revision): SMS/GG adapter now selects GameGear hardware.
VDP probes report both committed CRAM bytes on an odd-address write; even writes
remain latch-only and are retained in core snapshots. Peripheral timing is unchanged.
See docs/gg.md for hardware selection, Start, stereo PSG and deferred link support.

Genesis extension (same pinned revision): backend/genesis-core,
config/genesis-config and cpu/m68000-emu. Local hooks record accepted 68000
instructions after STOP/interrupt handling and retain the opcode prefetch's
backing identity/context. CPU buses, Z80, committed VDP backing writes, DMA,
interrupts and effective mapper states use optional bounded probes. Debugger
inspection/edits and detached I/O reads are exposed without live read side effects.
Snapshot changes include instrumentation-independent prefetch metadata and elapsed
master clocks. Native runtime helpers are separately authored MIT code; no GPL
implementation was copied. See docs/genesis.md and docs/genesis-evaluation.md.
