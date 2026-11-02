# Waymap

Native waypoint management mod for [Horizon](https://github.com/horizon-rust/horizon), the Rust rewrite of Minecraft Java Edition.

Waymap adds world markers, minimap dots, a waypoint list and an edit panel — all drawn through Horizon's native hooks. No WebAssembly, no plugins: pure Rust, built into the game by the Horizon launcher.

## Features

- **World markers** — 3D points projected from the camera (`locator_camera`: position, yaw, pitch, FOV), same math as the locator bar.
- **Minimap dots** — colored points on the minimap, including when the map is rotated (`minimap_view` + `draw_minimap` head override).
- **Waypoint list** — bottom-left HUD, six nearest points with distance.
- **Edit panel** — press `M` for an overlay of all waypoints; left-click info, right-click delete.
- **`/wp` commands** — `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help` (quoted names with spaces supported).
- **Persistence** — `horizon-mods/.horizon-mod-data/waymap/waypoints.txt`.
- **Settings** — Options > Mods > Waymap (markers, labels, list, edit panel).

## Bundle

| Attribute | Value |
|---|---|
| Mod ID | `waymap` |
| Version | `0.1.1` |
| File | `waymap-0.1.0.hnmod` |
| Horizon revision | `c9d474dd9116e84c243f8992d75c11314df56f06` |
| Mixins revision | `0d6b103c70633d8fd9443d8d0acd3aadaacedb06` |
| Schema / hooks | `1` / `35` |
| Injections | 10 HEAD + 2 modules (`waymap` in `game.rs`, `waymap_minimap` in `extras_hud.rs`) |
| Bundle size | 13 entries, ~40 KB |

## Installation

After the registry listing is approved, install Waymap from the launcher's Mods
page and build the instance there. Native mods run with the player's account
permissions; install only mods from authors you trust.

## Usage

| Key | Action |
|---|---|
| `N` | Add waypoint at current position |
| `B` | Toggle waypoint list |
| `M` | Open edit panel |
| `/wp ...` | Manage waypoints (`add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`) |

## Development

```bash
./check.sh unit  # Format, lint, unit tests, and package
HORIZON_MODS_REPO=/path/to/horizon-mods ./check.sh full
./package.sh     # Rebuild the .hnmod bundle
```

Layout:

```
mixins/waymap/            # Source: horizon-mixin.toml + Rust helpers
compatibility.json        # Tested Horizon and Mixins revisions, minimum hooks API
unit_tests.rs             # Unit-test entry point for bundle helpers
publish/                  # Release docs, checklist, registry entry draft
waymap-0.1.0.hnmod        # Release bundle
```

## Known limitations (Horizon architecture, not bugs)

- **HUD-based edit panel** — the mod uses an overlay rather than a dedicated native screen.
- **Exact Horizon pin** — the minimap view/draw injections use internal Horizon functions, so the bundle is built only against its tested revision.
- **Chat overlay** — markers and minimap dots may be covered while chat is open.

## License

MIT — see [LICENSE](./LICENSE).
