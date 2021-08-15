#!/usr/bin/env sh

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
