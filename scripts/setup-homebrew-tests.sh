#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for homebrew_tool in rgbasm rgblink rgbgfx rgbfix; do
  [[ $("$homebrew_tool" --version) == *v0.9.4* ]] || { echo "Run scripts/setup.sh for pinned RGBDS 0.9.4" >&2; exit 1; }
done
mkdir -p tools
if [ ! -d tools/gb-asm-tutorial/.git ]; then git clone https://github.com/gbdev/gb-asm-tutorial.git tools/gb-asm-tutorial; fi
git -C tools/gb-asm-tutorial checkout c931a929eac276156d65e0cb44cf422a7d19990b
make -C tools/gb-asm-tutorial/galactic-armada -j2
(
  cd tools/gb-asm-tutorial/unbricked/bricks
  rgbasm -o main.o main.asm
  rgblink -n unbricked.sym -o unbricked.gb main.o
  rgbfix -v -p 0xFF unbricked.gb
)
