#!/usr/bin/env bash
# Runs the source checks and, in full mode, the official Horizon native build.
set -euo pipefail
cd "$(dirname "$0")"

MODE=${1:-full}

cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
python3 tools/package.py

if [[ "$MODE" == "unit" ]]; then
  exit 0
fi
if [[ "$MODE" != "full" ]]; then
  echo "usage: $0 [unit|full]" >&2
  exit 2
fi

HORIZON_MODS_REPO=${HORIZON_MODS_REPO:?set HORIZON_MODS_REPO to a horizon-mods checkout}
WORK=$(mktemp -d "${TMPDIR:-/tmp}/waymap-native-check.XXXXXX")
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/native/waymap"
rsync -a --exclude=.git --exclude=target --exclude=build ./ "$WORK/native/waymap/"
cp -a "$HORIZON_MODS_REPO/native/tools" "$WORK/native/"
VERSION=$(python3 -c 'import tomllib; print(tomllib.load(open("mixins/waymap/horizon-mixin.toml", "rb"))["mod"]["version"])')
python3 "$WORK/native/tools/native.py" package waymap
cmp "target/waymap-$VERSION.hnmod" "$WORK/native/target/waymap-$VERSION.hnmod"
python3 "$WORK/native/tools/native.py" check waymap \
  --horizon "$(python3 -c 'import json; print(json.load(open("compatibility.json"))["horizon_revision"])')"
