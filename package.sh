#!/usr/bin/env bash
# Packs the bundle as target/<id>-<version>.hnmod (reproducible, fixed timestamps).
set -euo pipefail
cd "$(dirname "$0")"
python3 tools/package.py
VERSION=$(python3 -c 'import tomllib; print(tomllib.load(open("mixins/waymap/horizon-mixin.toml", "rb"))["mod"]["version"])')
cp "target/waymap-$VERSION.hnmod" "waymap-$VERSION.hnmod"
