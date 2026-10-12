# ROMAtlas fork
Source: https://github.com/Amjad50/mizu
Revision: ece2c0d40e1f2984a258410b6a6188418cd3e1c6 (2025-12-16).
MIT license: LICENSE. Copyright and license notices remain intact.
Only the Rust core and save-state crates are included. No ROMs, boot ROMs, assets, frontends, or external ROM-suite tests are copied.
Changes: omit external-ROM tests; expose instruction stepping and CPU register inspection for headless control.

Additional changes: mapper physical ROM translation; save/restore unticked debugger peek; skip logo check without boot execution; no implicit SRAM load when persistence is off; logs go to stderr.

Trace changes: bounded per-operation CPU bus records, separate opcode/operand reads, per-controller-write bank translation; telemetry fields stay outside hardware save states.

Frame/DMA changes: saved PPU VBlank counter; snapshot format version 3; per-byte OAM DMA events; fetch tagging uses read indices instead of the last event. No timing behavior changes.

Route remaining core diagnostic prints to stderr so they cannot corrupt JSON output.

STAT fix: clear stale LY=LYC coincidence at a scanline transition; compare the new LY on the following tick. A ROMAtlas bus-level regression test covers both match-clear and match-set phases.

MBC5 changes: RAM enable uses low-nibble comparison; rumble bit 3 remains saved but is masked out of RAM bank selection.

MBC3 changes: saved deterministic live/latched RTC counters instead of host-time seeds; snapshot format 4; correct latch edge and calendar carries; disabled RAM at reset, 7-bit ROM bank selection, invalid selectors return open bus. RTC oscillator ticks continue through STOP. STOP padding advances PC without an extra CPU operation. Battery saves remain disabled by the ROMAtlas adapter; fork RTC payload is not compatible with upstream battery files.

CGB changes: expose effective hardware bank/speed state; record general/HBlank DMA byte transfers; HBlank DMA uses active LCD hblank, not LCD-off/startup mode zero. CPU settling delay on KEY1 switching remains pending.

Physical tracing: immutable mapper RAM/RTC helpers; bank-aware bus locations and DMA targets; fixed ROM window and CGB RAM selection metadata; immutable raw memory reads. CPU VRAM reads/writes respect mode-3 locks; DMA writes bypass the CPU lock path.

step_operation retains elapsed cycles on errors; the adapter counts failed attempts and keeps illegal opcode reads outside execution coverage.

CGB speed-switch video access stays fixed during the CPU pause. Synthetic pixel tests cover all four starting PPU modes and resumed rendering. Snapshot version 7 includes frozen video state. Standalone core tests use the core Cargo.lock; the runtime still uses the workspace Cargo.lock.

CpuBusProvider now requires idle_cycle. The Game Boy bus clocks hardware without issuing a read. Internal CPU cycles no longer create artificial ROM-zero data events.

CGB HALT wake no longer adds an extra machine cycle. The source-verified 55-case CGB selection and default 89-case hardware gate pass; both selections run in CI.
