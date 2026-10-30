# Commentary workflow

This phase starts from the current phase-1 map in an existing workspace.
before/, baseline.json and phase1-state.json are frozen comparison artifacts.
source/ is independently maintained, buildable assembly. Never regenerate over it.
The portable .json carries structural directives and labels, not the final comments.
To reproduce commentary, preserve source/ and its local assets, not just .json.
Phase-1 unknown regions may remain unknown when their ambiguity has been reviewed
and explained. Every invocation completes current-source readability; interrupted
work resumes in this phase before redirect.
Final editorial completion is a separate acceptance gate; neither declares
phase-1 classification complete. See prompt.md for the full contract.

Commands from any working directory:

```sh
python3 /root/romatlas/scripts/commentary.py instructions /root/romatlas-batch/4f6f12b05ff708ca69b3ca8740ac8c58be572bd1f460b8255fa8700b0e48074e/workspace
python3 /root/romatlas/scripts/commentary.py verify /root/romatlas-batch/4f6f12b05ff708ca69b3ca8740ac8c58be572bd1f460b8255fa8700b0e48074e/workspace
python3 /root/romatlas/scripts/commentary.py read /root/romatlas-batch/4f6f12b05ff708ca69b3ca8740ac8c58be572bd1f460b8255fa8700b0e48074e/workspace
# Final editorial cleanup / archive acceptance only:
python3 /root/romatlas/scripts/commentary.py audit /root/romatlas-batch/4f6f12b05ff708ca69b3ca8740ac8c58be572bd1f460b8255fa8700b0e48074e/workspace
```

verify rebuilds source/game.o and compares the linked ROM with the original without
automatic checksum/header changes. Do not use an assembler command with automatic checksum repair as fidelity proof.
read refreshes reading-after.asm by expanding .asm includes in source order; it
retains hardware/constants includes and binary-asset references. It is a reading
copy, not a separate build target. reading-before.asm is frozen at preparation.
audit checks full-ROM inventory, reviewed entries/units and editorial notes, then
performs a fresh verification and refreshes the reading copy. It cannot measure
whether explanations are good: the agent must perform that editorial review.

review.json entries partition the physical ROM. Do not remove or change that
scaffold. Preserve valid review credit. Pending classified-source readability
work prevents a successful commentary handoff; interruptions must resume this phase. At final acceptance each needs status=reviewed, a summary and source locations.
Add compact units for meaningful routines and data structures:
name, offset, length, status, summary and source (list of file:line strings).
Units can overlap, but must lie within the ROM. Split a spanning unit's inventory
records across scaffold entries if useful. Every discovered unit must be reviewed at final acceptance.
Keep editorial_audit pending until final cleanup; then record reviewed and findings.
Verify coherent edit batches and handoffs; refresh the reading copy at handoff.
Do not repeatedly invoke the final audit during an intentionally partial pass.
Use uncertainties.md for investigated limitations; ambiguity is not skipped work.
Keep all source and assets local. Preserve toolkit reports and upstream notices.

## NES instruction and address context

Use physical file offsets including the iNES/NES 2.0 header and optional trainer.
PRG ROM, CHR ROM, CPU RAM, cartridge RAM, PPU VRAM, OAM and palette are separate
spaces. CPU aliases are not physical identities. For banked code supply effective
physical 8 KiB PRG bank indices in context.prg_banks for CPU slots $8000/$A000/
$C000/$E000; mapper register values are not bank indices. Static supported entry
contracts may establish context. Unknown banks remain unresolved locally.
6502 instruction lengths do not depend on status flags. All official and
unofficial opcode lengths decode; ambiguous unofficial encodings use exact byte
directives with mnemonic comments for rebuilding. Pointers are hypotheses, not
proof of code. Mapper writes/calls may invalidate static bank context. Discover
from validated reset/NMI/IRQ vectors and supported entry contracts, supplementing
with observed context/edges where useful. Structural classification, reachability,
whole-ROM classification, full current-source readability and editorial gates
remain separate and unchanged. Unknown bytes stay unknown.
Portable maps use romatlas-nes-map JSON and explicit CPU execution views. Read
docs/nes.md. Never use Game Boy .sym or SNES Asar syntax. Use bounded headless
emulator.step and immutable inspection. RAM execution does not prove a ROM upload
origin. Native helpers in romatlas-port::nes are independent MIT code; do not
copy GPL core code into the native runtime. Baseline debugger targets NTSC,
standard controllers and mappers 0/1/2/3/4/7. Other mapper/peripheral support is
explicitly deferred. No full hardware/game compatibility claim follows.
