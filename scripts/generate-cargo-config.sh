#!/usr/bin/env sh

ROOT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/.. &> /dev/null && pwd )"
cd $ROOT_DIR

TARGET=$1
LINKER=$2

if [ -z "$TARGET" ]; then
    echo "Target is required!" >&2
    exit 1
fi

if [ -z "$LINKER" ]; then
    echo "Linker is required!" >&2
    exit 1
fi

echo "[target.$TARGET]"
echo "linker = \"$LINKER\""
