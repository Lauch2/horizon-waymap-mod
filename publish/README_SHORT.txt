Waymap v0.1.0 — Native Horizon Waypoint Mod
===============================================

INHALT (27 Dateien, 37 KB):

Paket:
  waymap.hnmod — Fertig gebautes Bundle (13 Einträge, zip -qrD)

Manifest:
  horizon-native.json — id=waymap, version=0.1.0, horizon=d009ebd..., mixins=0d6b103...

Quellcode (mixins/waymap/):
  horizon-mixin.toml — 9 Injektionen + 2 Module (frame, list, settings, click,
                       command, key, hud, minimap_view, minimap;
                       Module waymap + waymap_minimap)
  waymap.rs — Kern (~820 Zeilen, State, Persistenz, /wp, HUD, Projektion)
  minimap_lib.rs — Library-Modul für Minimap-Overlay (draw, set_view, set_dots)
  minimap.rs — Cancellable HEAD auf draw_minimap
  minimap_view.rs — HEAD auf update_minimap
  frame.rs, list.rs, settings.rs, click.rs, command.rs, key.rs, hud.rs

Dokumentation (publish/):
  README.md — Vollständige Mod-Doku (deutsch)
  RELEASE_CHECKLIST.md — Was für off. Repo noch fehlt
  NOTE.md — Schnelle Antworten zu Problemen

Build-Tools:
  check.sh — Apply + cargo check (offline, Stage/Target unter build/)
  package.sh — zip -qrD waymap.hnmod

Projekt-Root:
  README.md — Deutsche Projektübersicht
  .gitignore — build/ und .hnmod ausgeschlossen

BEKANNT: 2 Restfehler (edit_panel Default, draw_edit_panel atlas)
BEKANNT: Chat-Öffnen versteckt HUD (Native-Limit, nicht fixbar)
BEKANNT: /wp-Befehle als "unbekannt" angezeigt (Native-Limit, nicht fixbar)

Offizielles Repo: https://github.com/horizon-rust/horizon-mixins
Für Veröffentlichung: Fork → mixins/waymap/ + tests + README → PR
