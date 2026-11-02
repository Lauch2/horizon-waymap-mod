# Waymap

<p align="center">
  <img src="https://img.shields.io/badge/Horizon%20Mod-Waymap-blue?style=for-the-badge&logo=minecraft&logoColor=white" alt="Waymap Mod">
  <img src="https://img.shields.io/badge/Version-0.1.2-green?style=for-the-badge" alt="Version 0.1.2">
  <img src="https://img.shields.io/badge/Horizon-d009ebd-blue?style=for-the-badge" alt="Horizon Revision">
  <img src="https://img.shields.io/badge/Mixins-0d6b103-orange?style=for-the-badge" alt="Mixins Revision">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge" alt="MIT License">
  <img src="https://img.shields.io/badge/Build-Passing-brightgreen?style=for-the-badge" alt="Build Status">
</p>

<p align="center">
  <strong>Native waypoint management for Horizon — Rust rewrite of Minecraft Java Edition</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#commands">Commands</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#limitations">Limitations</a> •
  <a href="#development">Development</a> •
  <a href="#license">License</a>
</p>

---

## Overview

**Waymap** is a native Horizon mod that adds persistent, client-side waypoints to Minecraft. Waypoints appear as 3D markers in the world, dots on the minimap, and an optional list panel — all rendered through Horizon's native hook system without WebAssembly or plugins.

**Key Philosophy:** Minimal, stable, and privacy-respecting. No server communication, no external dependencies, pure Rust code built into the game by the Horizon launcher.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| **World Markers** | 3D waypoint markers projected from the game camera (`locator_camera`), aligned with the world using the same math as the locator bar |
| **Minimap Dots** | Colored dots on the minimap at correct spatial positions, supports map rotation |
| **Waypoint List** | Toggleable panel (bottom-left) showing the 6 nearest waypoints with distances |
| **Per-Context Storage** | Waypoints stored separately per server (`server_<host_port>.txt`) and singleplayer world (`world_<folder>.txt`) |
| **Atomic Persistence** | Safe file writes via temp file + atomic rename; survives crashes and power loss |
| **Legacy Migration** | Auto-migrates old `waypoints.txt` to new per-context files on first run |
| **Full Command Suite** | `/wp add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help` |
| **Key Bindings** | `N` = add waypoint, `B` = toggle list panel (rebindable in Controls) |
| **Settings UI** | Options → Mods → Waymap (markers, labels, list panel toggles) |

---

## 📸 Screenshots

> *Screenshots coming soon — run the mod to see waypoints in action!*

| World Markers | Minimap Dots | Waypoint List |
|:---:|:---:|:---:|
| *3D markers in world* | *Colored dots on minimap* | *Bottom-left panel* |

---

## 📦 Installation

### Prerequisites
- Horizon Launcher installed
- Horizon instance running game version **26.3** (commit `d009ebd`)
- Minecraft: Java Edition owned on the Microsoft account

### Quick Install

1. **Download** the latest `waymap-0.1.2.hnmod` from the [Releases page](https://github.com/Lauch2/horizon-waymap-mod/releases/tag/v0.1.2)
2. **Copy** the file to your Horizon native mods directory:
   ```bash
   # Linux
   cp waymap-0.1.2.hnmod ~/.local/share/horizon/horizon-native-mods/
   ```
3. **Enable** the mod in `~/.local/share/horizon/horizon-mods.txt`:
   ```ini
   mod.waymap:true
   ```
4. **Build & Play** — Open the Horizon Launcher, select your instance, click **Build**, then **Play**

> **Note:** The mod is pinned to Horizon revision `d009ebd140476b5bdc958c00f56a75e82abec691` and mixins revision `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`. It will only build against this exact Horizon version.

---

## 🎮 Usage

### Quick Start
1. Join a world (singleplayer or multiplayer)
2. Press **`N`** to add a waypoint at your current position
2. Press **`B`** to toggle the waypoint list panel (bottom-left)
3. Type `/wp help` in chat for all commands

### Key Bindings (Default)
| Key | Action |
|-----|--------|
| `N` | Add waypoint at current position |
| `B` | Toggle waypoint list panel |

> **Rebindable:** Options → Controls → Waymap

---

## 📋 Commands Reference

| Command | Description | Example |
|---------|-------------|---------|
| `/wp add [name]` | Add waypoint at current position (auto-names as `WP 1`, `WP 2`, ...) | `/wp add Home Base` |
| `/wp del <name>` | Delete waypoint by name | `/wp del "Home Base"` |
| `/wp list` | List all waypoints in current context with coordinates | `/wp list` |
| `/wp rename <old> <new>` | Rename waypoint (supports quoted names) | `/wp rename "Home Base" "Main Base"` |
| `/wp color <name> <color>` | Change color (name or `#rrggbb`) | `/wp color Home Base red` or `/wp color Home Base #ff5555` |
| `/wp clear` | Delete all waypoints in current context | `/wp clear` |
| `/wp edit <name>` | Show waypoint info and edit hints | `/wp edit Home Base` |
| `/wp help` | Show this help message | `/wp help` |

### Name Handling
- **Quoted names:** Use quotes for names with spaces: `/wp add "My Base"`
- **Case-insensitive:** Commands match waypoint names case-insensitively
- **Colors:** Named colors (`red`, `green`, `blue`, `white`, `orange`, `yellow`, `cyan`, `purple`, `pink`, `gray`, `brown`, `lime`) or hex (`#rrggbb`)

---

## ⚙️ Configuration

Access via **Options → Mods → Waymap**:

| Setting | Default | Description |
|---------|---------|-------------|
| **Markers** | `on` | Show 3D world markers |
| **Labels** | `on` | Show waypoint name + distance text |
| **List Panel** | `on` | Show bottom-left waypoint list |

Settings are persisted in `horizon-mods.txt` and apply immediately.

---

## 🗂️ Data Storage

Waypoints are stored **per context** in `~/.local/share/horizon/horizon-mods/.horizon-mod-data/waymap/`:

| Context | File Pattern | Example |
|---------|--------------|---------|
| Multiplayer | `server_<host_port>.txt` | `server_localhost_25565.txt` |
| Singleplayer | `world_<folder>.txt` | `world_New_World.txt` |
| Legacy | `waypoints.txt` | Auto-migrated on first run |

### Migration
On first run with v0.1.2+, legacy `waypoints.txt` is automatically migrated to a `world_legacy.txt` file. The original is backed up as `waypoints.txt.legacy_backup`.

### Atomic Writes
All file writes use **atomic operations** (write to `.tmp`, then atomic rename) — safe against crashes and power loss.

---

## ⚠️ Limitations

| Limitation | Reason |
|------------|--------|
| **No command tab-completion** | Native commands don't register in chat dispatcher |
| **Chat hides HUD** | Markers/panel hidden when chat is open (Horizon behavior) |
| **No fullscreen map** | Only minimap dots + HUD overlay; no dedicated map screen |
| **Pinned Horizon revision** | Must match `d009ebd` exactly; breaks on Horizon updates |
| **No cross-device sync** | Waypoints stored locally only |

These are **Horizon architecture limitations**, not bugs.

---

## 🛠 Development

### Prerequisites
- Rust toolchain (stable)
- Python 3.11+
- Horizon source at pinned revision (`d009ebd`)

### Build Commands
```bash
# Quick validation (format, lint, test, package)
./check.sh

# Full native build (requires Horizon source)
HORIZON_MODS_REPO=/path/to/horizon-mods ./check.sh full

# Rebuild bundle
./package.sh

# Inspect bundle
unzip -l waymap.hnmod
```

### Project Structure
```
waymap/
├── .gitignore
├── README.md           # This file
├── check.sh            # Build validation script
├── package.sh          # Bundle creation script
├── horizon-native.json # Registry manifest
├── waymap.hnmod        # Built bundle (gitignored)
├── build/              # Build artifacts (gitignored)
├── mixins/
│   └── waymap/
│       ├── waymap.rs        # Core logic (31 KB)
│       ├── minimap_lib.rs   # Minimap overlay (4 KB)
│       ├── minimap_view.rs  # Minimap view hook
│       ├── minimap.rs       # Minimap HEAD injection
│       ├── frame.rs         # Frame hook
│       ├── list.rs          # Mod list injection
│       ├── settings.rs      # Settings page
│       ├── click.rs         # Settings click handler
│       ├── key.rs           # Key bindings
│       ├── command.rs       # Command handler
│       ├── hud.rs           # HUD rendering
│       ├── click.rs         # Click handler
│       ├── list.rs          # Mod list entry
│       ├── settings.rs      # Settings page
│       ├── horizon-mixin.toml  # Mixin config
│       ├── minimap_lib.rs   # Minimap library
│       └── minimap_view.rs  # Minimap view hook
├── publish/            # Release documentation
└── LICENSE             # MIT License
```

### Build Validation
```bash
# Quick check (format, lint, test, package)
./check.sh

# Full native build (requires Horizon source)
HORIZON_MODS_REPO=/path/to/horizon-mods ./check.sh full
```

---

## 📦 Release & Distribution

### Versioning
| Version | Date | Changes |
|---------|------|---------|
| `0.1.2` | 2026-11-02 | English only, removed HUD overlay, fixed context separation, atomic saves |
| `0.1.1` | 2026-11-01 | Added `/wp` commands |
| `0.1.0` | 2026-11-01 | Initial release |

### Release Process
1. Update version in `horizon-native.json` and `mixins/waymap/horizon-mixin.toml`
2. Run `./package.sh` to create `.hnmod` bundle
3. Test with `./check.sh`
4. Create Git tag: `git tag v0.1.x && git push origin v0.1.x`
4. Create GitHub Release with `.hnmod` asset

### Release Assets
- `waymap-<version>.hnmod` — Installable bundle
- Source code (automatic from Git tag)

---

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Run `./check.sh` before committing
4. Submit a Pull Request

**Code Style:**
- `cargo fmt` for formatting
- `cargo clippy --all-targets --locked -- -D warnings` for linting
- English-only comments and strings

---

## 📄 License

**MIT License** — see [LICENSE](./LICENSE) for details.

```
MIT License

Copyright (c) 2024 Lauch2

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🔗 Links

| Resource | Link |
|----------|------|
| **GitHub Repository** | https://github.com/Lauch2/horizon-waymap-mod |
| **Releases** | https://github.com/Lauch2/horizon-waymap-mod/releases |
| **Horizon Website** | https://playhorizon.dev |
| **Horizon Mods Policy** | https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md |
| **Horizon Mixins** | https://github.com/horizon-rust/horizon-mixins |

---

## 🙏 Acknowledgments

- **Horizon Team** — For the excellent native modding API
- **Horizon Mixins** — For the powerful mixin framework
- **Minecraft Java Edition** — For the inspiration

---

<p align="center">
  <sub>Built with ❤️ for the Horizon community</sub>
</p>