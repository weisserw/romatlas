# PC Engine integration choices

The existing approved jgenesis pin v0.11.4 predates PC Engine. Upstream
[jgenesis v0.13.0](https://github.com/jsgroth/jgenesis/tree/94bc43c8a375fc83be121658a0c520e6fd0999fb)
adds HuCard CPU, VDC/VCE, timer/interrupts, PSG and standard pad emulation without
CD-ROM. It supplies per-bus-cycle HuC6280 execution, incremental block transfers,
save-state serialization and a headless renderer/audio interface. The production
adapter vendors only that pin's needed Rust crates into vendor/jgenesis-pce,
separate from existing instrumented platform cores. All GPLv3 notices remain.
Local deterministic initialization, accepted-fetch/bus/backing probes, immutable
peeks and debugger edits adapt it to the existing toolkit. Newer std branch hints
and cfg_select were adjusted for the required Rust 1.90, without changing the
CPU execution model. No frontend/device initialization is needed.

The Rust alternative [mre/turbografx](https://github.com/mre/turbografx) documents
unimplemented audio and graphics/boot glitches. It does not presently offer a
better fit for the requested full debugger than jgenesis, so no core replacement
is proposed. This is an integration assessment, not a compatibility ranking.
Upstream jgenesis calls its initial PCE support preliminary; synthetic toolkit
checks do not establish broad commercial-game fidelity.

Both [WLA-DX](https://github.com/vhelin/wla-dx) and
[cc65](https://github.com/cc65/cc65/tree/555282497c3ecf8b313d87d5973093af19c35bd5)
have HuC6280 assemblers. Existing pinned ca65/ld65 is selected: its explicit CPU,
operand size overrides, origin directives and physical segment layout support
exact headerless HuCard reconstruction with an optional untouched copier prefix.
No assembler is implemented in Rust. No automatic checksum/header repair occurs.
Source carries local incbin data and editable code; independent recipients need
only their matching ROM, portable map and pinned tools.

[andlabs/huc6280disasm](https://github.com/andlabs/huc6280disasm) is an existing
GPLv3 Go disassembler. Its documented young whole-ROM flow-analysis interface
is less suitable for bounded decoding under caller-supplied MPR contexts and
transactional annotation than the already selected core's opcode mnemonic table.
ROMAtlas reuses jgenesis's complete HuC6280 mnemonic table and adds operand,
reference and ca65 syntax handling, rather than maintaining a duplicate opcode
table or depending on another whole-ROM analysis policy. Undefined opcodes and
unrepresentable wrapping relative transfers retain exact bytes plus readable
mnemonic comments. This fallback preserves fidelity, not classification proof.
