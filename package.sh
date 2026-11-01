#!/usr/bin/env bash
# Packs the bundle as target/<id>-<version>.hnmod (reproducible, fixed timestamps).
set -euo pipefail
cd "$(dirname "$0")"
python3 tools/package.py
cp target/waymap-0.1.0.hnmod waymap-0.1.0.hnmod
