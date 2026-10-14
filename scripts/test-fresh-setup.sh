#!/usr/bin/env bash
# Ubuntu minbase with no host tool, source checkout or build cache reused.
set -euo pipefail
cd "$(dirname "$0")/.."
[[ $(id -u) == 0 ]] || { echo 'Run as root for debootstrap/chroot.' >&2; exit 1; }
command -v debootstrap >/dev/null || { echo 'Install debootstrap first.' >&2; exit 1; }
mkdir -p build
rm -f build/fresh-setup-result.json
fresh_setup_root=$(mktemp -d "$PWD/build/fresh-setup.XXXXXX")
printf '%s\n' "$fresh_setup_root" > build/fresh-setup-root.txt
cleanup() {
  mountpoint -q "$fresh_setup_root/proc" && umount "$fresh_setup_root/proc" || true
  mountpoint -q "$fresh_setup_root/dev" && umount "$fresh_setup_root/dev" || true
}
trap cleanup EXIT
debootstrap --variant=minbase noble "$fresh_setup_root" http://archive.ubuntu.com/ubuntu >build/fresh-bootstrap.log 2>&1
mkdir -p "$fresh_setup_root/workspace"
tar --exclude=./build --exclude=./target --exclude=./tools --exclude=./.git --exclude=__pycache__ --exclude='*.pyc' --exclude=./vendor/mizu/mizu-core/target -cf - . | tar -xf - -C "$fresh_setup_root/workspace"
cp /etc/resolv.conf "$fresh_setup_root/etc/resolv.conf"
printf '#!/bin/sh\nexit 101\n' > "$fresh_setup_root/usr/sbin/policy-rc.d"
chmod +x "$fresh_setup_root/usr/sbin/policy-rc.d"
mount --bind /dev "$fresh_setup_root/dev"
mount -t proc proc "$fresh_setup_root/proc"
chroot "$fresh_setup_root" /bin/bash -lc '
  set -euo pipefail
  cd /workspace
  unset CARGO_HOME RUSTUP_HOME
  export CARGO_BUILD_JOBS=2
  test ! -d /root/.cargo
  test ! -d tools
  test ! -d target
  bash scripts/setup.sh
  export PATH="/root/.cargo/bin:$PATH"
  cargo fmt --check
  cargo clippy --workspace --all-targets --locked -- -D warnings
  cargo test --workspace --locked
  cargo clippy -p romatlas-port --all-targets --features desktop --locked -- -D warnings
  cargo test -p romatlas-port --features desktop --locked
  cargo test --manifest-path vendor/mizu/mizu-core/Cargo.toml --locked
  python3 scripts/test-boundaries.py
  python3 scripts/test-session.py
  python3 scripts/test-api.py
  python3 scripts/test-portability.py
  python3 scripts/test-fidelity.py
  python3 scripts/test-disassemble.py
  python3 scripts/test-commentary.py
  python3 scripts/test-archive.py
  python3 scripts/test-port.py
  bash scripts/setup-hardware-tests.sh
  bash scripts/setup-homebrew-tests.sh
  cargo build --release --locked
  python3 scripts/test-hardware.py
  python3 scripts/test-hardware.py --all-cgb
  python3 scripts/test-homebrew.py --binary target/release/romatlas
' > build/fresh-setup.log 2>&1
printf '{"ok":true,"environment":"Ubuntu noble minbase","root":"%s","log":"build/fresh-setup.log"}\n' "$fresh_setup_root" > build/fresh-setup-result.json
cat build/fresh-setup-result.json
