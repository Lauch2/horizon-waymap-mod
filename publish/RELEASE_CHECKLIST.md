# Waymap — Release-Checkliste für offizielles Repo

## Vor der Veröffentlichung (horizon-rust/horizon-mixins)

### Code-Qualität (noch offen)
- [x] `draw_edit_panel`: `atlas` Variable bereinigt
- [x] `draw_minimap_dots`: `MINIMAP_PIXELS` in lib behalten, aus waymap.rs entfernt
- [x] `MINIMAP_PIXELS`: Nur in `minimap_lib.rs`
- [x] `edit_panel` State: Default `false` gesetzt, Initialisierung korrigiert

### Dokumentation
- [x] Projektdoku (`publish/README.md`)
- [x] `README.md` im Projekt-Root (deutsch)
- [ ] `mixins/waymap/README.md` (für Repo-Doku, kurz)
- [ ] `docs/` für Mod-Nutzer

### Tests (fehlen komplett)
- [ ] `fixtures/native-mods/waymap/` erstellen
- [ ] `horizon-mixin.toml` mit Minimal-Beispiel
- [ ] `waymap.rs` mit Grundfunktionen testen
- [ ] `check.sh` als Test-Referenz dokumentieren

### Bundle
- [x] `waymap.hnmod` (13 Einträge, <64MiB)
- [x] `zip -qrD` (keine Verzeichnis-Einträge)
- [x] Kein `.DS_Store`
- [x] `horizon-native.json`: `hooks: 1` hinzugefügt

### Kompatibilität
- [x] `mixins`-Revision: `0d6b103...` (passt zu fps/minimap/recipes/zoom)
- [x] `horizon_revision`: `d009ebd...` (passt zu installierter Instanz)
- [ ] Mit `server_lod` kompatibel? (Nein — `a2af5c` vs `0d6b103`. Dokumentiert.)

### Legal
- [x] Lizenz: MIT (aus `horizon-mixins` Workspace)
- [ ] Autor/Feld im Manifest? (Nicht erforderlich für `.hnmod`)

## Bekannte Limite (für PR-Beschreibung)

> This native mod uses all available native hooks (`native_frame`, `native_hud`, `native_key`, `native_command`, `native_mods`, `native_settings`). It cannot provide:
> - Fullscreen UI (no `Screen` enum extension possible)
> - Command tab-completion (no `native_commands` hook points)
> - Chat-visible markers when Chat screen is open (`draw_extras` stops drawing HUD)
> - Inventory-based GUI (only `native_hud` overlay possible)
>
> These are Horizon native architecture limitations, not mod bugs.
