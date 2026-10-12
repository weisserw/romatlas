#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Ubuntu/Debian; elevate only system installation, keeping Rust/builds user-owned.
setup_privilege=()
if (( EUID != 0 )); then setup_privilege=(sudo); fi
"${setup_privilege[@]}" apt-get update
"${setup_privilege[@]}" apt-get install -y build-essential pkg-config libpng-dev bison flex curl git python3
export PATH="$HOME/.cargo/bin:$PATH"
if ! command -v rustup >/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain 1.90.0
fi
rustup toolchain install 1.90.0 --profile minimal --component rustfmt --component clippy
mkdir -p tools
if [ ! -d tools/mgbdis/.git ]; then git clone https://github.com/mattcurrie/mgbdis.git tools/mgbdis; fi
git -C tools/mgbdis checkout 95d951a9881a0d7ff1a42a94d1be175da92b1777
if [ ! -d tools/rgbds/.git ]; then git clone https://github.com/gbdev/rgbds.git tools/rgbds; fi
git -C tools/rgbds checkout d1829ed92327a9f9210ffd45d865331f24dfa4e6
make -C tools/rgbds -j2
"${setup_privilege[@]}" make -C tools/rgbds install
cargo build --locked
