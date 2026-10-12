#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
hardware_privilege=()
if (( EUID != 0 )); then hardware_privilege=(sudo); fi
"${hardware_privilege[@]}" apt-get update
"${hardware_privilege[@]}" apt-get install -y cmake build-essential git
mkdir -p tools
if [ ! -d tools/mooneye/.git ]; then git clone https://github.com/Gekkio/mooneye-test-suite.git tools/mooneye; fi
git -C tools/mooneye checkout 31510e12eea6286d36eea060a6adde755e1067aa
if [ ! -d tools/wla-dx/.git ]; then git clone https://github.com/vhelin/wla-dx.git tools/wla-dx; fi
git -C tools/wla-dx checkout 89a90a56be5c2b8cf19a9afa3e1b32384ddb1a97
cmake -S tools/wla-dx -B tools/wla-dx/build -DCMAKE_BUILD_TYPE=Release
cmake --build tools/wla-dx/build --target wla-gb wlalink -j2
make -C tools/mooneye -j2 WLA="$PWD/tools/wla-dx/build/binaries/wla-gb" WLALINK="$PWD/tools/wla-dx/build/binaries/wlalink"
