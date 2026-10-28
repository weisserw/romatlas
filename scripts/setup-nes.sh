#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# External assembler/linker; no game images are downloaded.
mkdir -p tools
if [ ! -d tools/cc65/.git ]; then
  git clone https://github.com/cc65/cc65.git tools/cc65
fi
git -C tools/cc65 checkout 555282497c3ecf8b313d87d5973093af19c35bd5
make -C tools/cc65 -j2 ca65 ld65
