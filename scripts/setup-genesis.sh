#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# GNU binutils 2.42 is GPLv3+. Prefer the tested Ubuntu/Debian cross package.
# Other distributions build the same upstream release into the local tools tree.
if command -v m68k-linux-gnu-as >/dev/null && m68k-linux-gnu-as --version | head -1 | grep -q '2\.42'; then
  exit 0
fi
mkdir -p tools/genesis
archive="$PWD/tools/genesis/binutils-2.42.tar.xz"
if [ ! -f "$archive" ]; then
  curl -L --fail https://ftp.gnu.org/gnu/binutils/binutils-2.42.tar.xz -o "$archive"
fi
printf '%s  %s\n' f6e4d41fd5fc778b06b7891457b3620da5ecea1006c6a4a41ae998109f85a800 "$archive" | sha256sum -c -
if [ ! -d tools/genesis/binutils-2.42 ]; then tar -xf "$archive" -C tools/genesis; fi
mkdir -p tools/genesis/build
cd tools/genesis/build
../binutils-2.42/configure --target=m68k-elf --prefix="$PWD/../../m68k" --disable-nls --disable-werror --disable-gdb --disable-gprofng --disable-sim --disable-gold
make -j2 all-binutils all-gas all-ld
make install-binutils install-gas install-ld
