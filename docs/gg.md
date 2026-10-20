# Game Gear adapter

Game Gear joins Game Boy, SMS, NES and SNES. Install tools with
`bash scripts/setup.sh` and build with `cargo build --locked --release` on Rust
1.90.0. `.gg` selects Game Gear automatically; `--platform gg` selects other
extensions, including headerless images. The implementation shares the Sega
adapter crate with SMS, with explicit hardware, map and snapshot identities.

```sh
target/release/romatlas inspect /path/to/owned.gg
target/release/romatlas map init /path/to/owned.gg --output /tmp/game.json
target/release/romatlas serve --rom /path/to/owned.gg
target/release/romatlas generate /path/to/owned.gg --map /tmp/game.json --out /tmp/gg-source
target/release/romatlas verify /path/to/owned.gg --generated /tmp/gg-source
python3 scripts/disassemble.py /path/to/owned.gg --workspace /tmp/gg-workspace --json
python3 scripts/test-gg.py
```

All shared Sega debugging, analysis and map APIs in [sms.md](sms.md) apply:
Z80 register/shadow state and edits, immutable mapped/physical memory, I/O peeks
and edits, actual fetch/memory/I/O/VDP/interrupt events, bank transitions,
bounded instruction/cycle/frame stepping, coverage, contextual successors,
streaming, deterministic reset and guarded snapshot save/load/export/import,
framebuffer and stereo WAV capture. Bus events use instruction-start cycles.
Game Gear selects jgenesis GameGear hardware, disables the SMS FM expansion,
and outputs the 160×144 LCD viewport with blending disabled. Start is polled
through I/O $00 bit 7; it does not drive SMS Pause NMI. Buttons are
up/down/left/right/button1/button2/start (1/2 aliases); only port 1 is accepted.
The nominal frame cadence is 53,693,175 / 896,040 Hz (about 59.92 Hz).

Inspection retains full-file SHA-256, optional 512-byte copier prefix, header
location, checksum, product/revision/region/size and bank layout. Header regions
5/6/7 identify Japanese/export/international Game Gear cartridges when no explicit
platform override is supplied. Sega/Codemasters mapper identification remains a
hypothesis, with the detection basis exposed. The baseline supports their common
banking/SRAM paths; unusual boards are outside scope. Raw image sizes are 8 KiB
units up to 4 MiB; execution currently requires a power-of-two payload.

Physical offsets include the copier prefix. ROM, mirrored CPU windows, RAM,
cartridge RAM, VRAM, 64-byte CRAM and I/O remain separate spaces. Effective
16 KiB banks belong in context.rom_banks, not mapper register encodings.
Unresolved windows remain unresolved. Discovery, references and little-endian
pointer hypotheses use the shared Z80 decoder/context engine; reached addresses
are not executed coverage. RAM execution does not prove a ROM upload origin.

Portable maps use `romatlas-gg-map` version 1 JSON, with the same physical regions,
execution views, annotations, provenance and transactional validation/import/export
contract as SMS. Cross-platform maps and snapshots are rejected by identity.
Generated local source uses WLA-DX `game.asm`, `linkfile`, `mapping.json` and
`image.gg`; headers/checksums are preserved rather than synthesized. Exact `.db`
fallbacks retain undocumented aliases, ignored prefixes and bank-boundary
instructions with decoded comments. All bytes are reassembled and compared;
raw reconstruction does not establish structural classification.

New Game Gear profiles cover initialization, classification, commentary, archives,
reference correction and native port preparation without rewriting old workspaces.
They preserve structural/reachability distinctions, whole-ROM classification,
full current-source readability and editorial gates. Archives contain reconstruction
recipes rather than game bytes. Preparation restores the identical reference and
creates a pending native scaffold; it does not implement the game.

The inherited core has incomplete serial/EXT link emulation; link peripherals,
BIOS boot, uncommon mappers and broad hardware/game compatibility are not claimed.
See [tool selection](gg-evaluation.md) and [native runtime](gg-native.md).
