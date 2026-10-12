# Licensed game regression

Source: https://github.com/gbdev/gb-asm-tutorial
Pin: c931a929eac276156d65e0cb44cf422a7d19990b.

The repository uses MIT: Eldred Habert (2021–2022) and gb-asm-tutorial contributors (2022–2023). Galactic Armada also includes Eievui's Sprite Objects Library (2021), under the zlib license in libs/sporbs_lib.asm. Preserve all upstream notices. Source stays unchanged. Generated ROMs, symbol files, extracted assets and recipient projects stay in ignored tools folders or temporary directories.

```sh
sudo bash scripts/setup.sh
bash scripts/setup-homebrew-tests.sh
cargo build --release --locked
python3 scripts/test-homebrew.py --binary "$PWD/target/release/romatlas"
```

The setup builds Galactic Armada and Unbricked's bricks stage with RGBDS 0.9.4. The test checks the source revision, clean tracked files and expected ROM hashes.

Galactic Armada advances from title to story and into gameplay through A-button input. Right input changes the player position. Unbricked simulates ball/bricks and right input changes the paddle. Both run from a deterministic boot-skipping state with no proprietary boot ROM.

The test saves a gameplay snapshot, runs eight frames of right input, restores and repeats it. CPU/model/bank/cycle state and inspected OAM/player data match exactly. It collects bounded traces, confirms 64 observed code regions, and exports only stock-compatible .sym metadata. A separate recipient directory gets the same local ROM and .sym, runs stock mgbdis plus RGBDS, and rebuilds byte-identically without emulator state or trace input.

The first measured runs cover 254 unique instructions/267 successor edges in Armada and 130/140 in Unbricked. The report stays in build/homebrew-results.json. This proves these tested input paths and map round trips, not all game states or full PPU/CGB timing. The default 89-case hardware gate and separate 55-case CGB selection now pass. CPU/video settling tests also pass; untested hardware and native-color game paths remain limits.
