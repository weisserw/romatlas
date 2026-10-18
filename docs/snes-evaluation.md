# SNES evaluation and integration gate

Follow-up: GPLv3 adoption was approved after this evaluation. Production baseline
integration and current measured limits are documented in [snes.md](snes.md). The
spike findings below describe the earlier selection gate, not the current CLI.

2026-10-18. This is a measured feasibility spike and a recommendation for review,
not a production SNES adapter or a completed compatibility gate. The existing
Game Boy workflow, prompts, CLI, Cargo.lock and native-port library are unchanged.

## Agreed target

NTSC baseline console, standard LoROM/HiROM, SRAM and ordinary controllers,
including SPC700/DSP sound. Enhancement chips, unusual revisions and rare
peripherals are deferred, with explicit unsupported-feature errors. Debugging
must cover both CPUs, actual accesses/fetches, immutable inspection, deterministic
input/reset/snapshots and bounded execution. Uploaded audio code needs its ROM
storage identity, ARAM execution location and upload/overwrite provenance.

The complete feature also requires portable ROM-free mappings, readable commented
assembly, decoding, assembly and exact rebuilding. Evidence-backed whole-ROM
classification and editorial gates remain intact; raw fallback does not complete
them. Native ports remain behavioral Rust implementations with ROM-backed assets.
Runtime helpers grow from demonstrated port needs, independently of emulator
selection. WAV capture precedes live playback through the desktop runtime.

## Recommendation for review

Continue with **jgenesis as the leading technical candidate**, subject to GPLv3
review, completing instrumentation and representative-game testing. Evaluate
**Asar 1.91 as an external assembler** and build a Rust decoding/export backend
where existing disassembly tools cannot meet the harness and fidelity contracts.
A non-Rust emulator port is not currently warranted by the evidence.

| Candidate | Findings | Disposition |
|---|---|---|
| [jgenesis](https://github.com/jsgroth/jgenesis) | Separate cycle-driven 65816/SPC700, headless backend, DSP/PPU and serialization. Pinned v0.11.4 builds and passes the probes below. | Leading candidate; license decision and broader tests pending. |
| [Super Sabicom](https://github.com/tanakh/super-sabicom/tree/e897aefdfefb2ca645b40cc9a960607b01ac3a9e) | MIT; public context, pure reads, instruction stepping, state serialization. Rust 1.90 compile smoke passes with 36 upstream warnings. README and source identify missing Mode 6, direct color and interlace; source has panicking standard graphics paths. | Viable permissive fallback, with substantial baseline PPU work. No equivalent runtime/accuracy gate was run; do not present a measured head-to-head compatibility ranking. |
| [RustySNES](https://github.com/doublegate/RustySNES) | Advertised MIT/Apache; modular CPU/core/state design. Upstream itself records an [open provenance question](https://github.com/doublegate/RustySNES/blob/main/docs/provenance-open-question.md). | Source-level review only. Resolve provenance and independently measure behavior before considering adoption; no accusation or validated accuracy claim is made here. |

Current jgenesis master at b4f943bb5f6f8e3857a37dd9915d27bdf88ed5dd fails
Cargo 1.90 manifest loading: a frontend dependency overrides inherited default
features. The spike instead pins v0.11.4 at
7b3cc06196f9cb9ac5cb067ffb3cdbc8c4efe282. Its upstream locked `cargo check -p
snes-core` passes on Rust 1.90. This is a tested compatible baseline, not proof
that it is the newest usable release. Review later release fixes before vendoring.

### License decision

[jgenesis v0.11.4](https://github.com/jsgroth/jgenesis/blob/v0.11.4/LICENSE)
is GPLv3. Linking it into a distributed debugger changes the combined executable's
distribution terms: plan for GPLv3 distribution and corresponding source, preserving
MIT notices on ROMAtlas's existing code. Do not silently relabel the existing
workspace or native runtime. A separate SNES debugger executable is an available
packaging choice if the main MIT binary should remain separate. This choice needs
explicit user review before production incorporation.

The experiment's original harness/scripts are MIT. The locally built executable
links GPLv3 code and must be treated accordingly. Upstream source, licenses,
modifications, CPU test datasets and binaries stay under ignored local paths;
there is no production GPL dependency or committed emulator source.

[Asar's license](https://github.com/RPGHacker/asar/blob/v1.91/LICENSE) is
LGPL-3.0-or-later and explicitly disclaims copyright over its input/output.
Use a pinned replaceable executable, preserving notices and its redistribution
requirements. This is distinct from linking the emulator into the debugger.
The native port runtime has no emulator dependency, and should remain independent
of both tools; do not copy GPL emulator implementation into its helpers.

## Reproduction after production integration

The isolated spike was retired after production integration. Its original synthetic
fixture now lives in `scripts/fixtures/snes/fixture.asm`; the pinned licensed CPU
vector downloader is `scripts/fetch-snes-cpu-tests.py`. Historical measurements
below describe the retired spike. Current reproduction and supported limits are
in [the SNES guide](snes.md):

```sh
bash scripts/setup.sh
python3 scripts/test-snes.py
bash scripts/test-snes-cpu.sh
```

Generated images, audio and CPU datasets remain local and ignored.

## Measured results

- Two original 64 KiB fixtures: LoROM and HiROM. Reset into native mode; exercise
  8/16-bit immediates, long ROM/WRAM addressing, serial B input, CGRAM color and
  standard IPL upload to ARAM $0200. Check WRAM $0100 = 34 12 78, held B reaching
  WRAM $0110 and uploaded SPC instruction bytes.
- Each runs two warmup frames, snapshots with B held, then three frames. Replay
  after changing live input reproduces serialized hardware state, CPU registers,
  frame hashes, PCM and bounded trace records. Capture enabled/disabled produces
  identical hardware state and PCM. Fixed zero startup WRAM replaces host RNG in
  the local core so repeated cold resets also serialize identically.
- Each captured segment produces 2,397 stereo frames at 48 kHz, with 4,794 nonzero
  channel samples. Valid PCM16 stereo RIFF files are written. This tests the
  synthetic noise voice and output path, not music fidelity or live playback.
- Pure ROM/WRAM/ARAM inspection preserves serialized hardware state; mapped I/O
  peek requests explicitly fail rather than invoking a simulated access. CPU
  aliases $00:8000/$80:8000 resolve to the same ROM location. I/O/WRAM/SRAM are
  never guessed to be physical ROM. SRAM-less LoROM low-window aliases are
  conservatively unresolved by the spike.
- Foreign-ROM and truncated snapshot restores fail without modifying live state.
  Bounded main-CPU instruction stepping ends at an instruction boundary after ten
  actual opcode fetches; frame stepping and budget exhaustion are also tested.
  SRAM $00 survives both snapshot replay and cold reset after writes through
  each mapper's CPU SRAM window. Execution has wall-time/core-tick limits; zero
  budget fails. Trace detail
  caps at 50,000 records and reports drops (185,811 in each segment).
- 832 bounded external MIT CPU cases pass: first 64 cases in eight 65816 files
  and five SPC700 files. Compare registers, final memory and total cycle count.
  These are generated CPU vectors, not on-console test ROMs; bus signal/order
  comparison and a full CPU/hardware suite remain unmeasured.
- Asar 1.91 at 6a6dbdaed8cb773e583052ce4f66375bd896c3a1 builds locally. Its
  65816 and SPC700 opcode fixture golden streams match: 601 and 519 bytes.
  These are upstream assembler encoding expectations, not independent CPU tests.
- Two reconstruction probes regenerate deterministic source from only an identical
  locally supplied image, the exported JSON mapping and pinned tools. Exact whole
  image comparison passes, preserving checksums, both immediate widths, distinct
  encoded aliases, backward/wrapping branches and an SPC ROM/ARAM location pair.
  An unknown-M immediate stays raw with a diagnostic; a context-independent NOP
  elsewhere still decodes. The deliberately small Python decoder is a feasibility
  probe, not a proposed production decoder or full classification implementation.

CPU datasets are pinned to [65816 307c63f](https://github.com/SingleStepTests/65816/tree/307c63f5673c8c6b861c52f71ddb89212a227480)
and [SPC700 67d15f4](https://github.com/SingleStepTests/spc700/tree/67d15f492b2740964abd4efd1229e0ec9c342228).
Setup retains their MIT notices beside local downloaded data.

### What the spike does not establish

Representative locally owned games have not been supplied/tested. A flat-color
frame and synthetic noise are not evidence of broad graphics/audio correctness.
Standard graphics modes, timing/interrupt/DMA interactions and mapper/SRAM behavior
need larger measured gates. No full hardware compatibility is claimed.

The prototype implements instruction/cycle/frame execution through ExecutionAdapter;
portable TraceEvent/session conversion is pending. Main-CPU operation stepping
counts newly fetched instructions and first finishes any prior partial instruction.
Independent SPC700 instruction stepping remains to be exposed.
Bus/fetch records come from actual core operations (`main` denotes the main bus,
not a proven CPU-versus-DMA origin), but timestamps group SPC
accesses within a main tick, DMA origins are not separately tagged, and interrupt,
frame, DSP/DMA and observed-edge envelopes still need integration. Peeks cover
only safe memory subsets; side-effect-free I/O inspection needs dedicated APIs.
Snapshot state is same-build evaluation state; external format/versioning and
battery-file persistence policies are not implemented here.

The JSON map is a version-zero experiment. The emitter supports selected
instructions plus raw fallback; it does not satisfy readable whole-ROM output,
full opcode decoding, annotations/conflicts or stock interchange requirements.
No production commands currently accept SNES. No SNES helpers have been added to
the native runtime. The full project objective remains pending.

## Toolchain conclusions

Asar handles explicit operand widths, both CPUs and fixed physical placement.
Disable checksum repair with `--fix-checksum=off`, explicitly control copier
headers and avoid heuristics that collapse mirrored encoded addresses. The spike
uses `norom` physical layout plus `base` CPU/ARAM views and supported incbin ranges.
Numeric BRA/BRL operands are raw displacements; replacing them with numeric CPU
targets changed bytes during the probe. Future symbolic targets must be verified
at branch/bank edges. Generic SPC upload reconstruction must not assume Asar's
specific N-SPC `spcblock` envelope.

[WLA-DX](https://github.com/vhelin/wla-dx) is a credible alternative supporting
both CPUs (GPL assembler executable, not a linked runtime).
[ca65/ld65](https://cc65.github.io/doc/ca65.html) offers permissive licensing and
65816 width/layout controls, but needs a separate SPC700 backend. Stock da65 is
not a 65816 disassembler. [DiztinGUIsh](https://github.com/DizTools/DiztinGUIsh)
is a useful reference/import candidate (GPL, C#/WinForms), but its documented
alias-reconstruction caveat and application shape prevent treating it as an
already validated headless byte-exact generator. None of these alternatives was
compiled or subjected to this spike's reconstruction tests.

## Work after review

1. Settle GPL packaging/core choice and obtain representative owned images. Extend
   hardware gates, including SRAM, graphics modes, audio and interrupt/DMA behavior.
   If no candidate survives, suspend integration and scope a non-Rust core port.
2. Implement romatlas-snes behind existing shared contracts; add bounded per-CPU
   instruction execution, full immutable inspection and trace/coverage/edges,
   provenance and snapshot versioning. Keep shared crates free of console imports.
3. Implement complete Rust 65816/SPC700 decoding and context-aware analysis; define
   physical classification separately from execution views and E/M/X-dependent
   interpretations. Preserve operand aliases and uploaded-code lifetimes.
4. Specify a validated ROM-free portable SNES map, assembler interchange and Asar
   backend. Expand exact-rebuild gates beyond the subset probe, including copier
   headers, arbitrary image sizes, every instruction, branches and upload formats.
5. Add platform-selected workflow templates and archive/porter backends according
   to docs/snes-prompt-audit.md. Preserve Game Boy prompt output and stock .sym
   fidelity, classification and editorial gates.
6. Add SNES ROM-backed asset recipes and port-driven native rendering/audio helpers,
   headless tests and later desktop sound playback. Do not embed an emulator as
   the native-port implementation.
