# Waymap — Release-Checkliste für offizielles Repo

## Vor der Veröffentlichung (horizon-rust/horizon-mixins)

### Code-Qualität (noch offen)
- [ ] `draw_edit_panel`: `atlas` Variable entfernen (unused)
- [ ] `draw_minimap_dots`: Ohne `MINIMAP_PIXELS` aufrufen oder entfernen
- [ ] `MINIMAP_PIXELS`: In `minimap_lib.rs` behalten oder hier entfernen
- [ ] `edit_panel` State: Default korrekt (fehlt `edit_panel: false` im generierten Code)

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
- [ ] `horizon-native.json`: `hooks: 2` hinzufügen (wenn Hooks aktiv)

### Kompatibilität
- [x] `mixins`-Revision: `0d6b103...` (passt zu fps/minimap/recipes/zoom)
- [x] `horizon_revision`: `d009ebd...` (passt zu installierter Instanz)
- [ ] Mit `server_lod` kompatibel? (Nein — `a2af5c` vs `0d6b103`. Dokumentiert.)

### Legal
- [x] Lizenz: MIT (aus `horizon-mixins` Workspace)
- [ ] Autor/Feld im Manifest? (Nicht erforderlich für `.hnmod`)

## Bekannte Limite (für PR-Beschreibung)

> This native mod uses all available native hooks (`native_frame`, `native_hud`, `native_key`, `native_command`, `native_mods`, `native_settings`, `native_panel_items`). It cannot provide:
> - Fullscreen UI (no `Screen` enum extension possible)
> - Command tab-completion (no `native_commands` hook points)
> - Chat-visible markers when Chat screen is open (`draw_extras` stops drawing HUD)
> - Inventory-based GUI (only `native_hud` overlay possible)
>
> These are Horizon native architecture limitations, not mod bugs.
