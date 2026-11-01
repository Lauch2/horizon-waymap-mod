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
| Version | `0.1.0` |
| File | `waymap-0.1.0.hnmod` |
| Horizon revision | `d009ebd140476b5bdc958c00f56a75e82abec691` |
| Mixins revision | `0d6b103c70633d8fd9443d8d0acd3aadaacedb06` |
| Schema / hooks | `1` / `2` |
| Injections | 9 HEAD + 2 modules (`waymap` in `game.rs`, `waymap_minimap` in `extras_hud.rs`) |
| Bundle size | 13 entries, ~40 KB |

## Installation

1. Copy `waymap-0.1.0.hnmod` to `horizon-native-mods/`.
2. Add `mod.waymap:true` to `horizon-mods.txt`.
3. Launcher → Build (first build takes 10–30 min).

## Usage

| Key | Action |
|---|---|
| `N` | Add waypoint at current position |
| `B` | Toggle waypoint list |
| `M` | Open edit panel |
| `/wp ...` | Manage waypoints (`add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`) |

## Development

```bash
./check.sh     # Apply mixins to a pristine Horizon tree + cargo check (stage/target under build/)
./package.sh   # Rebuild waymap-0.1.0.hnmod (zip -qrD, no directory entries)
```

Layout:

```
mixins/waymap/            # Source: horizon-mixin.toml + Rust helpers
fixtures/native-mods/     # Test fixtures (minimal horizon-mixin.toml + data)
publish/                  # Release docs, checklist, registry entry draft
waymap-0.1.0.hnmod        # Ready-to-install bundle
```

## Known limitations (Horizon architecture, not bugs)

- **No fullscreen / inventory UI** — native mods can only draw HUD overlays (`native_hud`); the `Screen` enum cannot be extended.
- **No tab completion for `/wp`** — native commands are not registered in the chat dispatcher; they still work.
- **Markers hidden while chat is open** — `draw_extras` stops drawing the HUD when `Screen::Chat` is active.
- **Panel closure** — `draw_extras` has a closure, so `AFTER_ASSIGN`/`OVERWRITE` injections fail; the edit panel uses a cancellable head on `draw_minimap` instead.

## License

MIT — see the `horizon-mixins` workspace.
