# Waymap — Submission and Release Information

Waymap is a native Horizon mod (`.hnmod`) for named waypoints. The mod code runs with the player's permissions; the Horizon mod policy and native mod documentation therefore apply fully.

## Features

- 3D waypoint markers, projected from the player's camera
- Colored waypoints on the minimap, also when the map is rotated
- HUD list with the six nearest waypoints
- Edit overlay with `M`; left-click shows details, right-click deletes
- `/wp` commands: `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- Persistence in `horizon-mods/.horizon-mod-data/waymap/waypoints.txt`
- Settings in Options > Mods > Waymap

## Build and Validation

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
python3 tools/package.py
HORIZON_MODS_REPO=/path/to/horizon-mods ./check.sh full
```

`check.sh full` runs the official `native/tools/native.py check` process:
Horizon is prepared at the pinned revision, mixins are applied,
and `horizon-client` is compiled. The test requires a local copy of
`horizon-mods`, Rust, and Horizon's build dependencies.

## Compatibility

| Field | Value |
|-------|-------|
| Mod ID | `waymap` |
| Version | `0.1.1` |
| Minecraft Version | `26.3` |
| Tested Horizon Revision | `c9d474dd9116e84c243f8992d75c11314df56f06` |
| Minimum Hooks API | `35` |
| horizon-mixins Revision | `0d6b103c70633d8fd9443d8d0acd3aadaacedb06` |

The mod uses stable native hooks for HUD, inputs, commands, and settings.
The minimap view and drawing hooks are injected over internal
Horizon functions; therefore, the mod remains pinned to the tested Horizon revision.

## Submission

Horizon's mod registry accepts native mods via
[playhorizon.dev/mods/submit](https://playhorizon.dev/mods/submit). The form
uploading the `.hnmod` file opens the registry pull request. Before submitting:

1. `./check.sh full` must succeed at the pinned revision.
2. The uploaded bundle hash must match the entry; published files are not replaced afterward.
3. The source must match the built file and be under the MIT license.
4. The confirmations of the [Mod Policy](https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md)
   must be given by the author/publisher.
5. For updates, the version must be signed with the existing registry key.
   The local environment does not contain the private key; a new release
cannot be signed here.

Signing is optional but recommended. Once a project has a public
key in its registry entry, later versions must be signed with exactly that key.

### Current Status for Version 0.1.1

The full native build and the official registry packer have been
validated against the pinned revisions. The bundle `waymap-0.1.1.hnmod` has SHA-256
`9212f8acd583b32dfbfdb80e083729aa74e7750045272bc0f8d308eba014b83a`.
The signature with the already stored private key is still missing;
until then, do not submit. The published bundle `0.1.0` and its signature remain unchanged.

## Known Limitations

- The edit window is a HUD overlay, not a dedicated mod screen.
- The chat can cover HUD markers.
- The minimap patch depends on internal Horizon functions and therefore limits
  compatibility to the tested revision.