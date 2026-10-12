# Public hardware tests

Current result: all 87 broad DMG cases pass. The required gate contains 89 cases, including two CGB boot checks. The selected DMG known-failure list is empty. This does not prove full hardware compatibility; CGB speed-switch PPU effects and broader peripheral checks remain open.

Mooneye source: https://github.com/Gekkio/mooneye-test-suite
Pin: 31510e12eea6286d36eea060a6adde755e1067aa (2026-07-14).
License: MIT; Joonas Javanainen, 2014–2022. Keep the upstream LICENSE.

Assembler: WLA-DX 10.6, 89a90a56be5c2b8cf19a9afa3e1b32384ddb1a97.
License: GPL-2.0-or-later. It is an external test build tool, not linked to the Rust emulator.

```sh
sudo bash scripts/setup-hardware-tests.sh
cargo build --release --locked
python3 scripts/test-hardware.py --all-dmg
```

Tests build from licensed assembly source. No commercial ROM or stock boot ROM is required. Generated test ROMs and detailed reports stay under ignored tools/build folders.

The runner steps each CPU operation and recognizes the documented LD B,B report. B/C/D/E/H/L = 3/5/8/13/21/34 means pass; six bytes of 42 hex mean fail. A timeout, illegal opcode, unsupported feature, or exhausted operation budget never counts as pass. The runner uses actual LY/SC behavior; it does not use Mooneye's optional test shortcut of forcing these reads to ff.

`romatlas test-rom test.gb --operations 10000000 --timeout-ms 10000` prints status and state as JSON. The batch script fails if any selected test does not pass. `--all-dmg` probes all model-compatible acceptance tests and MBC1 tests; it does not imply full hardware compatibility. The default gate reads tests/mooneye-gate.json. Boot tests depend on the documented boot-skipping initial state; tests for other SoCs are outside the DMG gate.

## Initial DMG/MBC1 measurement

The 79 compatible DMG/MBC1 probe cases produced 72 passes, six PPU timing failures, and one explicit unsupported MBC5 cartridge error. The initial 72-case regression gate covers CPU instruction timing, EI/DI/RETI, interrupt handling, HALT, DIV/TIMA reload/write/glitch cases, OAM DMA, and all 13 MBC1 source-built cases. It also includes the applicable boot-state tests.

Known nonpasses remain in tests/mooneye-known-failures.json. The PPU cases are hblank_ly_scx_timing-GS, intr_2_mode0_timing, intr_2_mode0_timing_sprites, intr_2_mode3_timing, lcdon_timing-GS, and lcdon_write_timing-GS. The unsupported case is oam_dma/sources-GS (MBC5 type 1b). These are incomplete work, not expected hardware behavior or proof of full compatibility. The full probe still exits nonzero.

The report includes each test's final registers, cycles, frames, and operation count. The gate JSON lists exactly which cases CI requires to pass. It does not suppress nonpasses in the full probe.

## STAT boundary fix

The LCD-start probe exposed a stale coincidence flag: LY changed at a line boundary, but STAT kept the old LY=LYC match for one machine cycle. The fix clears the match at that boundary; the next comparison tick sets it for the new LY. A focused test covers LYC=0 and LYC=1 using real NOP bus cycles and unticked inspection. This removes the first STAT mismatch in lcdon_timing-GS; later PPU timing checks still need work.

Use `romatlas serve` with emulator.start, emulator.test (max_operations and timeout_ms), then emulator.memory to inspect test result buffers after a report. Tests need not restart to expose the failed state. `scripts/test-hardware.py --all-dmg --match acceptance/ppu` isolates PPU cases for local diagnosis; it still exits nonzero on nonpasses.

## MBC5 expansion

The current probe has 87 cases: 81 pass and six PPU timing cases fail. All eight MBC5 ROM-size tests and the formerly unsupported OAM DMA-source case pass. The regression gate now includes these nine cases, for a total of 81. The known-failure list now contains only the six PPU cases. The full probe still exits nonzero until they pass.

## CGB boot-state checks

The default gate now has 83 cases: the 81 DMG/MBC1/MBC5 cases plus misc/boot_regs-cgb and misc/boot_div-cgbABCDE. Both run with `test-rom --model cgb`. Use `scripts/test-hardware.py --cgb-boot` to isolate them. The DMG broad probe remains 87 cases with six known PPU nonpasses. The CGB0 boot divider test targets a different SoC revision and is outside the current CGB boot-state gate.

Local CGB CPU-bus tests also check bank aliases, echo RAM, VRAM banks, basic KEY1 clock ratio, general/HBlank DMA data and replay. A synthetic speed-switch test also checks the 2050-M-cycle pause in both directions, DIV freeze, no added instruction coverage, and snapshot replay. These checks do not prove the PPU locking/rendering effects during speed switches or full CGB video/audio accuracy. The model keeps the core's boot-skipping initial state; no proprietary boot ROM is shipped.

Speed-switch timing reference: [Pan Docs KEY1](https://gbdev.io/pandocs/CGB_Registers.html). The CPU pause uses the new speed: 4100 PPU ticks after normal-to-double and 8200 after double-to-normal. The saved countdown uses fork state version 5; older fork states fail the version check. Session snapshots use the same running binary. The PPU continues to run during this pause; its special speed-switch memory locks and rendering rules still need implementation and hardware tests.

## LCD-start access diagnosis

The LCD-start test currently reaches its OAM-access checks. At round 10 (cycle 61), it expects 00 but reads ff. Removing the eight-dot post-render OAM lock and locking at the next visible-line boundary made all its OAM reads match. It then exposed a normal-line VRAM read at the mode-2-to-3 boundary (round 12, cycle 131). The VRAM lock now covers that boundary, including physical trace translation, while preserving the special first line in mode zero.

The combined OAM/VRAM experiment passed lcdon_timing-GS but broke intr_2_oam_ok_timing. The OAM experiment was reverted. It is not a complete PPU fix: rendering, OAM unlock and STAT interrupt timing need a joint correction. The six-failure checklist remains open.

## Joint STAT and OAM timing correction

The current fix asserts mode-2 STAT throughout OAM scan, starting at the normal line boundary. The DMG frame-start exception keeps its later assertion. Mode-0 STAT now starts on the tick that ends rendering, rather than waiting for another tick. OAM locks follow rendering/OAM-scan modes and the visible line boundary. The former eight-dot post-render extension is removed.

This joint change addresses the interrupt regression found in the earlier OAM-only experiment. Focused tests pass intr_2_0_timing, intr_2_oam_ok_timing, intr_2_mode0_timing, intr_2_mode3_timing and lcdon_timing-GS. Sprite and LCD-write timing still need work.

The release broad probe now passes 84 of 87 cases. The three fixed cases join the regression gate, which now contains 86 cases including the two CGB boot checks. Known failures are hblank_ly_scx_timing-GS, intr_2_mode0_timing_sprites and lcdon_write_timing-GS. The full probe still exits nonzero while these remain.

## LCD-write boundary fix

The LCD-write test exposed four rejected OAM writes at normal-line boundaries: NOP counts 111, 131, 225 and 245. Reads and writes use distinct lock timing. OAM writes remain available at the visible-line boundary and the mode-2-to-3 handoff; VRAM writes remain available at that handoff. CPU accesses and physical trace translation now use the same read/write predicates. Both LCD-start tests pass.

The release broad probe passes 85/87 cases. LCD-write timing joins the required gate. The remaining failures are hblank_ly_scx_timing-GS and intr_2_mode0_timing_sprites.

## SCX/HBlank phase check

The SCX test first failed with SCX=1: LY still held the preceding line at the test point that should read the next line. Ending rendering on the final pixel loses one dot at the HBlank boundary. The current trial ends rendering on the next draw dot. The focused SCX test and both LCD-start tests pass with this phase. A three-dot trial shifted the unscrolled line too far and failed existing cases; it was discarded. The full gate still needs to verify the one-dot fix before its case becomes required.

The verified one-dot fix passes all prior gate cases. The full DMG probe now passes 86/87, with only intr_2_mode0_timing_sprites failing. SCX/HBlank joins the required gate, bringing it to 88 cases with the two CGB boot checks.

## Sprite fetch timing

The former renderer pushed sprite pixels without charging fetch time. Sprite fetch now pauses pixel output and background fetching. Each selected object costs six dots; background-tile alignment costs at most five additional dots and applies once per tile. The first acquisition uses a three-dot correction for this fetcher's startup phase. A fetched-object mask prevents repeated fetches while the same LCD position waits. Objects beyond the right edge do not add fetch time.

The source-built sprite timing test passes its 105 layouts. A synthetic test also saves during line-one sprite fetching and checks exact state/byte replay after the fetch completes. Fork snapshot version 6 stores the pending counter and both masks. Older external snapshots fail the version check. This does not prove all peripheral or CGB edge behavior.

## CGB speed-switch video state

VRAM/OAM access rules now stay fixed at the PPU mode at STOP entry until the CPU pause ends. The PPU keeps running. HBlank/VBlank entry blocks its video reads and renders black. OAM-scan entry permits background reads but suppresses object selection. Rendering entry permits both. Mapped inspection uses the frozen CPU access rules; physical inspection still reads backing storage.

Four core tests use synthetic white background and blue objects, advance through real PPU clocks, check the output and held locks, then check resumed object rendering. CPU-level pause/replay tests verify the counter and saved video state. Snapshot version 7 includes this state. Core tests run with `cargo test --manifest-path vendor/mizu/mizu-core/Cargo.toml --locked`; their separate Cargo.lock pins test dependencies. All eight core tests and the 89-case hardware gate pass. Broader CGB peripheral and real-game checks remain open.

## CGB palette data access

Background/object palette data reads return ff while rendering owns the port. Writes leave color data unchanged but still advance an enabled index, including wrap from 63 to 0. Index registers remain accessible. Frozen speed-switch access state also applies to palette data. A synthetic CPU test records denied writes with blocked=true and mapped=false, then reads back unchanged color bytes in HBlank. Core tests cover both palettes and index wrap.

## CGB source-verified selection

Run `python3 scripts/test-hardware.py --all-cgb`. The runner selects every acceptance source whose pass list names CGB or CGBABCDE, requires all selected builds and forces model cgb. It adds the two boot cases. Source-verified CGB0-only cases and DMG-specific expectations do not apply to this model. This is a 55-case selection, not a complete CGB compatibility claim.

Five interrupt-relative PPU tests exposed an extra HALT-wake machine cycle in the CGB CPU path. Removing that cycle passes all 55 cases; the 89-case default gate remains green. CI runs both. Native-color games and broader peripheral checks remain open.
