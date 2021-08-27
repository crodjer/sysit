#!/usr/bin/env bash

RELEASE="$(curl -s https://api.github.com/repos/crodjer/sysit/releases/latest)"
RELEASE_PRETTY="$(echo $RELEASE | jq)"
TAG=$(echo $RELEASE | jq .tag_name -r)

get_sha256 () {
    curl -sL $(echo "$RELEASE_PRETTY" | grep -oE "https://.+$1.+sha256") | cut -f 1 -d ' '
}

echo "pkgver=${TAG/v/}"
echo "sha256sums_x86_64=('$(get_sha256 x86_64-unknown)')"
echo "sha256sums_aarch64=('$(get_sha256 aarch64)')"
echo "sha256sums_armv7=('$(get_sha256 armv7)')"
