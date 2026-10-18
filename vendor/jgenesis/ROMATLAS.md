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
