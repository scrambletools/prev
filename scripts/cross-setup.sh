#!/usr/bin/env bash
# Sets up an x86_64 Ubuntu (the release uses 22.04, as GitHub's runners
# do) to cross-compile prev for another Linux architecture:
#
#   scripts/cross-setup.sh riscv64
#
# Installs the GCC cross compiler and the target's libraries through
# Debian multiarch from Ubuntu's ports archive, and QEMU to run the
# result. In GitHub Actions it also sets the variables cargo, cc and
# MuPDF's makefile need; elsewhere it prints them.
set -euo pipefail

arch=${1:?usage: cross-setup.sh ARCH}
case $arch in
riscv64) triple=riscv64-linux-gnu target=riscv64gc-unknown-linux-gnu ;;
aarch64) triple=aarch64-linux-gnu target=aarch64-unknown-linux-gnu ;;
*) echo "cross-setup.sh: unknown architecture $arch" >&2; exit 1 ;;
esac
debarch=${triple%%-*}
[ "$debarch" = aarch64 ] && debarch=arm64
codename=$(. /etc/os-release && echo "$VERSION_CODENAME")

# The main archive has only x86 packages; pin the existing sources to the
# host's architecture and take the target's from the ports archive.
host=$(dpkg --print-architecture)
sudo sed -i -E "s/^deb (http|https|mirror)/deb [arch=$host] \1/" \
  /etc/apt/sources.list /etc/apt/sources.list.d/*.list 2>/dev/null || true
sudo tee /etc/apt/sources.list.d/ports-$debarch.list >/dev/null <<SOURCES
deb [arch=$debarch] http://ports.ubuntu.com/ubuntu-ports $codename main universe
deb [arch=$debarch] http://ports.ubuntu.com/ubuntu-ports $codename-updates main universe
SOURCES
sudo dpkg --add-architecture "$debarch"
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  clang libclang-dev pkg-config desktop-file-utils file qemu-user-static \
  gcc-$triple g++-$triple \
  libfontconfig1-dev:$debarch libfreetype-dev:$debarch \
  libxkbcommon-dev:$debarch libwayland-dev:$debarch

upper=$(echo "$target" | tr a-z- A-Z_)
lower=${target//-/_}
vars=(
  "CARGO_TARGET_${upper}_LINKER=$triple-gcc"
  "CC_$lower=$triple-gcc"
  "CXX_$lower=$triple-g++"
  "AR_$lower=$triple-ar"
  # MuPDF's makefile archives and links its font data with these.
  "AR=$triple-ar"
  "LD=$triple-ld"
  # Build scripts and other host code keep the host's tools.
  "AR_x86_64_unknown_linux_gnu=ar"
  "HOST_AR=ar"
  "PKG_CONFIG_ALLOW_CROSS=1"
  "PKG_CONFIG_LIBDIR=/usr/lib/$triple/pkgconfig:/usr/share/pkgconfig"
)
for var in "${vars[@]}"; do
  if [ -n "${GITHUB_ENV:-}" ]; then
    echo "$var" >>"$GITHUB_ENV"
  else
    echo "export $var"
  fi
done
