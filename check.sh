#!/usr/bin/env bash
# Applies the mixins to a pristine Horizon tree and type-checks the client.
# Stage and Cargo target stay under build/, inside this folder.
set -euo pipefail
cd "$(dirname "$0")"

SRC=${HORIZON_SRC:-/tmp/opencode/src}
# The launcher's horizon-mixins revision (0d6b103, its default and the one the
# other installed mods pin), so a local check matches the launcher's build.
MIXINS=${HORIZON_MIXINS:-$HOME/.local/share/horizon-launcher/native/tools/0d6b103c7063/bin/horizon-mixins}
CARGO_HOME=${CARGO_HOME:-$HOME/.local/share/horizon-launcher/cargo}

"$MIXINS" apply --workspace "$SRC" --mods mixins --out build/stage

cd build/stage
CARGO_HOME="$CARGO_HOME" cargo check --offline \
  -p horizon-client --target-dir ../target
