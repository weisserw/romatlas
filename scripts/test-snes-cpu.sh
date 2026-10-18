#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 scripts/fetch-snes-cpu-tests.py
ROMATLAS_SNES_CPU_TESTS="$PWD/tools/snes-eval/cpu-tests" cargo test --locked -p romatlas-snes --test cpu_vectors -- --nocapture
