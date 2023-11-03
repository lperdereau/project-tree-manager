#!/bin/bash
set -exo pipefail

echo "[*] rust builds:"

BIN=$(grep name Cargo.toml | cut -d\  -f3 | tr -d '\"')

TARGETS=(
    "x86_64-linux"
    "aarch64-linux"
    "x86_64-windows"
    "x86_64-macos"
    "aarch64-macos"
)

[ -d dist ] && rm -rf dist
mkdir dist

for TARGET in "${TARGETS[@]}" ; do
  echo "[+]   $TARGET:"
  SUFFIX=""

  [[ "$TARGET" =~ .*"windows".* ]] && SUFFIX=".exe"
  NAME=${BIN}-${TARGET}${SUFFIX}

  mv bin-${TARGET}/${BIN}${SUFFIX} dist/$NAME

  if [ -z "$NOCOMPRESS" ]; then
    echo "[-]    - compress"
    if [[ "$TARGET" =~ .*"windows".* ]]; then
      xz --keep dist/$NAME
      (cd dist; zip -qm9 $BIN-${TARGET}.zip $NAME)
    else
      xz dist/$NAME
    fi
  fi
done

echo "[*] sha256sum"
(cd dist; sha256sum *) | tee ${BIN}.sha256sum
mv ${BIN}.sha256sum dist/

echo "[*] done"
