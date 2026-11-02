# Waymap — Submission and Release Information

Waymap is a native Horizon mod (`.hnmod`) for named waypoints. The mod code runs with the player's permissions; the Horizon mod policy and native mod documentation therefore apply fully.

## Features

- 3D waypoint markers, projected from the player's camera
- Colored waypoints on the minimap, also when the map is rotated
- HUD list with the six nearest waypoints
- `/wp` commands: `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- Persistence in per-context files under `horizon-mods/.horizon-mod-data/waymap/`
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

## Compatibility (both versions)

The bundle pins Horizon `d009ebd140476b5bdc958c00f56a75e82abec691`, requires Native
Hooks API `2`, and pins horizon-mixins
`0d6b103c70633d8fd9443d8d0acd3aadaacedb06`. The minimap overlay uses internal
Horizon functions, which is why the exact tested commit is pinned.

### Known implementation limits

- The edit panel is a HUD overlay, not a dedicated native screen.
- The chat may cover HUD markers.
- Waypoint persistence is local to the player's mod-data folder.
- Version 0.1.3 adds English-only text, removed HUD overlay, fixed context separation.

### Changes for Version 0.1.3

**Compared to Version 0.1.0:**
- All text in English (no German remnants)
- Removed HUD edit panel (was non-functional)
- Fixed context separation (per-server/world storage)
- Atomic file writes (temp file + atomic rename)
- Legacy migration support
- All commands work: `/wp add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- Keys: N = add waypoint, B = toggle list panel
- Persistence in `horizon-mods/.horizon-mod-data/waymap/`
- Settings page in Options > Mods > Waymap
- German UI and documentation → English

## License

MIT — see [LICENSE](../LICENSE).