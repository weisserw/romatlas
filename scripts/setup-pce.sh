#!/usr/bin/env bash
set -euo pipefail
# Reuse the existing pinned cc65 build, including its HuC6280 backend.
bash "$(dirname "$0")/setup-nes.sh"
