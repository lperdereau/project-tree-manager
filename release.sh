#!/bin/bash
set -xeo pipefail

echo "[*] rust builds:"

BIN="project-tree-manager"

TARGETS=(
    "x86_64-linux:x86_64-unknown-linux-gnu"
    "aarch64-linux:aarch64-unknown-linux-gnu"
    "x86_64-macos:x86_64-apple-darwin"
    "x86_64-windows:x86_64-pc-windows-msvc"
    "aarch64-macos:aarch64-apple-darwin"
)

[ -d dist ] && rm -rf dist
mkdir dist

for target in "${TARGETS[@]}" ; do
    NAME=${target%%:*}
    TARGET=${target#*:}
  echo "[+]   $NAME:"
  rustup -q target add $TARGET
  echo "[-]    - build"

  [ "$TARGET" == *"windows"* ] && SUFFIX=".exe"

  RUSTFLAGS='-C target-feature=+crt-static' cargo build --target $TARGET --release --bins --locked -q

  mv target/$TARGET/release/$BIN dist/$BIN-$NAME
  if [ -z "$NOCOMPRESS" ]; then
    echo "[-]    - compress"
    if [ "$GOOS" = "windows" ]; then
      xz --keep dist/$BIN-${NAME}/${SUFFIX}
      (cd dist; zip -qm9 $BIN-${NAME}.zip $BIN-${NAME}${SUFFIX})
    else
      xz dist/$BIN-${NAME}
    fi
  fi
done
