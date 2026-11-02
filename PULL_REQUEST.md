## Waymap — native waypoint mod

Waymap adds named waypoints to Horizon, with world markers, minimap dots, a nearest-
waypoint HUD list, an edit overlay and `/wp` commands.

### Registry policy confirmations

- [ ] I made this mod, or have the right to publish it, under MIT.
- [ ] It is not a port or translation of another mod.
- [ ] It contains no Minecraft, Mojang or Microsoft code, assets or data.
- [ ] It contains nothing that harms players or secretly accesses their data/network.
- [ ] The source link contains the code used to build the uploaded `.hnmod`.
- [ ] I read and accept the [Horizon mod policy](https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md).

Source: https://github.com/Lauch2/horizon-waymap-mod

### Build validation (v0.1.1)

- `cargo fmt --check`
- `cargo clippy --all-targets --locked -- -D warnings`
- `cargo test --locked`
- `python3 tools/package.py`
- `HORIZON_MODS_REPO=/path/to/horizon-mods ./check.sh full`

### Build validation (v0.1.0 - vorhanden)

- Alle Tests bestanden für Version 0.1.0
- Registry-Eintrag bereits vorhanden mit korrektem Hash und Signature

### Compatibility (beide Versionen)

The bundle pins Horizon `c9d474dd9116e84c243f8992d75c11314df56f06`, requires Native
Hooks API `35`, and pins horizon-mixins
`0d6b103c70633d8fd9443d8d0acd3aadaacedb06`. The minimap overlay uses internal
Horizon functions, which is why the exact tested commit is pinned.

### Known implementation limits

- The edit panel is a HUD overlay, not a dedicated native screen.
- The chat may cover HUD markers.
- Waypoint persistence is local to the player's mod-data folder.
- Version 0.1.1 adds `/wp` commands and enhanced waypoint management.

### Änderungen für Version 0.1.1

**Im Vergleich zu Version 0.1.0 hinzugefügt:**
- Native Command-Integration (`commands.rs`) für `/wp`-Befehle
- Vollständige `/wp`-Befehls-Suite: `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- Unterstützung für durch Leerzeichen markierte Namen
- Erweiterte Hilfefunktion

### Lizenz

MIT — siehe [LICENSE](./LICENSE).