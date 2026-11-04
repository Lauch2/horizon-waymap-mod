#!/usr/bin/env bash
# Packs the bundle as waymap.hnmod (a zip: horizon-native.json + mixins/waymap/).
# The launcher allows only mixin sources, data files and README/LICENSE/NOTICE
# text files inside mixins/<id>/; the licence notice ships as NOTICE.md.
set -euo pipefail
cd "$(dirname "$0")"
rm -f waymap.hnmod
zip -qrD waymap.hnmod horizon-native.json mixins/waymap -x '*.DS_Store'
unzip -l waymap.hnmod
