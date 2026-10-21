#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
rustup target add wasm32-unknown-unknown --toolchain 1.90.0
if [[ ! -x tools/wasm-bindgen/bin/wasm-bindgen ]] || [[ "$(tools/wasm-bindgen/bin/wasm-bindgen --version)" != 'wasm-bindgen 0.2.129' ]]; then
  cargo +1.90.0 install wasm-bindgen-cli --version 0.2.129 --locked --root "$PWD/tools/wasm-bindgen"
fi
# Resolve/cache browser scaffold dependencies for offline preparation.
cargo fetch --locked
