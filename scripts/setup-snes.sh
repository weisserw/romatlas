#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Build-time assembler only; the emulation runtime is Rust.
command -v cmake >/dev/null
mkdir -p tools
if [ ! -d tools/asar/.git ]; then
  git clone https://github.com/RPGHacker/asar.git tools/asar
fi
git -C tools/asar checkout 6a6dbdaed8cb773e583052ce4f66375bd896c3a1
cmake -S tools/asar/src -B tools/asar-build -DCMAKE_BUILD_TYPE=Release
cmake --build tools/asar-build --target asar-standalone -j2
