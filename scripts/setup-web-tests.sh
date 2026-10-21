#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if ! python3 -c 'import ensurepip' 2>/dev/null; then
  setup_privilege=()
  if (( EUID != 0 )); then setup_privilege=(sudo); fi
  "${setup_privilege[@]}" apt-get update
  "${setup_privilege[@]}" env DEBIAN_FRONTEND=noninteractive NEEDRESTART_MODE=l apt-get install -y python3-venv
fi
python3 -m venv tools/web-tests
tools/web-tests/bin/pip install 'playwright==1.56.0'
DEBIAN_FRONTEND=noninteractive NEEDRESTART_MODE=l tools/web-tests/bin/python -m playwright install --with-deps chromium
