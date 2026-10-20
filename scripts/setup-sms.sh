#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p tools
if [ ! -d tools/wla-dx/.git ]; then
  git clone https://github.com/vhelin/wla-dx.git tools/wla-dx
fi
git -C tools/wla-dx checkout 89a90a56be5c2b8cf19a9afa3e1b32384ddb1a97
cmake -S tools/wla-dx -B tools/wla-dx/build -DCMAKE_BUILD_TYPE=Release
cmake --build tools/wla-dx/build --target wla-z80 wlalink -j2
