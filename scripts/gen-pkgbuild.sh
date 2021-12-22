#!/usr/bin/env bash

TAG=$(curl -s https://api.github.com/repos/crodjer/sysit/releases/latest | grep tag_name  | grep -Eo "v[0-9]+\.[0-9]+\.[0-9]+")

get_sha () {
    local DOWNLOAD_PATH="https://github.com/crodjer/sysit/releases/download/$TAG/sysit-$TAG-$1.tar.gz.sha256"
    curl -s -L $DOWNLOAD_PATH | cut -d ' ' -f 1
}

read -r -d '' PKGBUILD <<EOF
# Maintainer: Rohan Jain <crodjer [@] pm [dot] me>

pkgname=sysit-bin
pkgver=${TAG:1}
pkgrel=1
pkgdesc=' System Sit! Check on the system with a quick glance.'
arch=('x86_64' 'aarch64' 'armv7')
url='https://github.com/crodjer/sysit'
license=('GPL-3.0-or-later')
depends=()
provides=("\${pkgname%-bin}")
conflicts=("\${pkgname%-bin}")

source_x86_64=("\${url}/releases/download/v\${pkgver}/\${pkgname%-bin}-v\${pkgver}-x86_64-unknown-linux-musl.tar.gz")
source_aarch64=("\${url}/releases/download/v\${pkgver}/\${pkgname%-bin}-v\${pkgver}-aarch64-unknown-linux-gnu.tar.gz")
source_armv7=("\${url}/releases/download/v\${pkgver}/\${pkgname%-bin}-v\${pkgver}-armv7-unknown-linux-gnueabihf.tar.gz")

sha256sums_x86_64=('$(get_sha x86_64-unknown-linux-musl)')
sha256sums_aarch64=('$(get_sha aarch64-unknown-linux-gnu)')
sha256sums_armv7=('$(get_sha armv7-unknown-linux-gnueabihf)')

package() {
  install -Dm755 "\${srcdir}/sysit" "\${pkgdir}/usr/bin/sysit"
}
EOF

echo "$PKGBUILD"
