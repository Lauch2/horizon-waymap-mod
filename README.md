# Waymap 0.2.0

Waymap is a native, client-side Horizon mod for persistent waypoints. Waypoints
appear as 3D markers in the world and as coloured dots on the minimap, with an
optional list panel. A native screen provides a list/editor and a Deaths tab;
every death automatically records a waypoint at the death spot.

- Horizon: `6475f62f3c834b7489e1816bf47f47ea5eb2b879`
- horizon-mixins: `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`
- Package version: `0.2.0`

## Features

- 3D waypoint markers, culled when outside the camera view
- Coloured minimap dots; death waypoints are drawn as a red X
- Native list/editor screen with All, Overworld, Nether, End and Deaths filters
- Optional HUD panel with the nearest waypoints and distances
- Up to 512 waypoints per world/server context
- Automatic death waypoints (`Death 1`, `Death 2`, …)
- `/wp` command suite, retained alongside the GUI
- Per-context storage with atomic writes and v1 migration

## Installation

1. Put `waymap.hnmod` in `~/.local/share/horizon/horizon-native-mods/`.
2. Make sure the mod is enabled in `~/.local/share/horizon/horizon-mods.txt`:

   ```ini
   mod.waymap:true
   ```

3. In the Horizon Launcher, choose an instance built from the pinned Horizon
   revision, build it, and start the game.

Waymap is pinned to Horizon `6475f62f3c834b7489e1816bf47f47ea5eb2b879`.
Other Horizon revisions are not claimed to work.

## Controls

Default keys (rebindable in Options → Mods → Waymap; `M` is always accepted for
the list):

| Key | Action |
|-----|--------|
| `N` | Create a waypoint at the player's position |
| `B` or `M` | Open/close the waypoint list |
| `Esc` | Close the list or cancel the editor |
| `Enter` | Save the editor or edit the selected row |
| `Tab` / `Shift+Tab` | Next/previous editor field |
| `↑` / `↓` | Move the list selection or the editor field |
| `PgUp` / `PgDn` / `Home` / `End` | Move through the list |
| `Delete` | Clear the focused editor field; twice on the list deletes the row |

Clicking a row selects it. The list provides filter buttons, `Hide`/`Show`,
`Edit`, `Delete`, `+ New` and `Close`; the mouse wheel scrolls the list. The
editor provides name, X/Y/Z, `Use my position`, dimension cycling, a 16-colour
palette, visibility and save/cancel. Coordinates are always rounded to whole
numbers; decimal input is rounded when saved.

## Commands

Commands work in chat and persist immediately:

| Command | Description |
|---------|-------------|
| `/wp add [name]` | Add a waypoint at the current position |
| `/wp del <name>` | Delete a waypoint (`delete` and `remove` are aliases) |
| `/wp list` | List the current context's waypoints |
| `/wp rename <old> <new>` | Rename a waypoint; quoted names work |
| `/wp color <name> <color>` | Set a colour by name or `#rrggbb` (`colour` is an alias) |
| `/wp clear` | Delete all waypoints in the current context |
| `/wp edit <name>` | Show waypoint info (`set` is an alias) |
| `/wp hide <name>` | Hide a waypoint from the world and minimap |
| `/wp show <name>` | Show a hidden waypoint |
| `/wp help` | Show the command list |

Names are matched case-insensitively. Colours may be given as `red`, `green`,
`blue`, `white`, `orange`, `yellow`, `cyan`, `purple`, `pink`, `gray`, `brown`,
`lime` or as hex (`#rrggbb`).

## Storage

Data is stored per world/server in
`~/.local/share/horizon/horizon-mods/.horizon-mod-data/waymap/`:

| Context | File |
|---------|------|
| Multiplayer | `server_<host>_<port>.txt` |
| Singleplayer | `world_<folder>.txt` |

The v2 format is a `# waymap 2` header followed by one tab-separated waypoint
per line:

```text
dimension<TAB>x<TAB>y<TAB>z<TAB>RRGGBB<TAB>hidden<TAB>name<TAB>death
```

`hidden` and `death` are `0` or `1`; the trailing `death` column is optional
(older files load with `death = 0`). Names escape backslashes, tabs and
newlines. Coordinates are feet positions and whole numbers.

Writes go to a temporary file and are renamed atomically; an unreadable file is
never overwritten. A legacy `waypoints.txt` is migrated once to
`world_legacy.txt`, with the original preserved as `waypoints.txt.v1.bak`.
Legacy files stored eye level; the v1 parser converts it to feet once.

## Settings

Options → Mods → Waymap:

| Setting | Default | Meaning |
|---------|---------|---------|
| `markers` | on | Draw 3D world markers |
| `labels` | on | Show name and distance next to markers |
| `panel` | on | Show the bottom-left HUD list |

Settings are kept in the mod's data folder and read on start.

## Limitations

- Client-side only; waypoints are not shared with a server or other devices.
- No chat tab-completion for `/wp`.
- Markers and the panel are hidden while chat is open.
- No full-screen map; only world markers and minimap dots.
- The mod is pinned to the Horizon revision above.

## Development

```bash
HORIZON_SRC=/path/to/horizon/source ./check.sh
cd build/stage
cargo test --offline -p horizon-client waymap --target-dir ../target
cd ../..
./package.sh
```

`check.sh` applies the mixins to the pinned Horizon tree and runs
`cargo check -p horizon-client`. `package.sh` writes the installable
`waymap.hnmod` bundle.

## License and attribution

Waymap is MIT-licensed. Its screen and input architecture is adapted from
[horizon-waypoints](https://github.com/Nirokx/horizon-waypoints), Copyright
(c) 2026 Nirokx, MIT. The full third-party notice is in
[THIRD_PARTY_NOTICES.md](./THIRD_PARTY_NOTICES.md); the installable bundle
carries the same notice as `mixins/waymap/NOTICE.md`.
