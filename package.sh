#!/usr/bin/env bash
# Packs the bundle as waymap.hnmod (a zip: horizon-native.json + mixins/waymap/).
set -euo pipefail
cd "$(dirname "$0")"
rm -f waymap.hnmod
zip -qrD waymap.hnmod horizon-native.json mixins/waymap -x '*.DS_Store'
unzip -l waymap.hnmod
