# PC Engine core fork

Upstream: https://github.com/jsgroth/jgenesis
Revision: 94bc43c8a375fc83be121658a0c520e6fd0999fb (v0.13.0 commit).
Annotated tag object: 554f5687a39c567720231978b90ed85203ce2fb1.
License: GPLv3; preserve LICENSE and upstream attribution.

Only pce-core, pce-config, huc6280-emu, dsp and common/proc-macro dependencies
are included. This isolated workspace leaves the older approved jgenesis fork
and its instrumented platforms unchanged. No frontend, CD core, ROM, extracted
asset or commercial fixture is included. Dependencies retain Rust 1.90 support;
newer cfg_select/cold_path calls are replaced by cfg blocks/omitted branch hints.

Local changes: deterministic zero initial registers/RAM/VRAM, optional bounded
actual accepted fetch/bus/interrupt/VRAM/CRAM/DMA probes, immutable debugger
inspection, backing edits, CPU/MPR state access and original HuCard bank identity.
Mapper guessing occurs before runtime ROM expansion. The 512 KiB compatibility
expansion repeats the upper chip as described by upstream's mapping comments.
Original SRAM/SF2 CRC identification is retained. The adapter owns identity,
limits, transactional guarded snapshots, trace contexts and export policies.
See docs/pce.md for tested scope and limits. Power-on zeros are a deterministic
debugger policy, not an assertion about physical power-on register/RAM contents.

No GPL implementation is used in independent MIT romatlas-port::pce helpers.
