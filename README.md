# Waymap — Waypoint Management Mod für Horizon

Native Mod (`.hnmod`) für Horizon `d009ebd140476b5bdc958c00f56a75e82abec691`.
Gebaut mit `horizon-mixins` 0.6.0 (`0d6b103...`).

## Features

- **World Markers**: 3D-Punkte aus `locator_camera` (Kamera, yaw, pitch, FOV)
- **Minimap Dots**: Farbige Punkte auf der Minimap (auch bei gedrehter Karte)
- **Waypoint List**: Unten links, 6 nächste mit Entfernung
- **`/wp` Commands**: `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- **Edit-Panel (M)**: HUD-Overlay zeigt alle Waypoints; Links=Info, Rechts=Löschen
- **Persistenz**: `horizon-mods/.horizon-mod-data/waymap/waypoints.txt`
- **Settings**: Options > Mods > Waymap (Marker, Labels, Liste, Edit-Panel)

## Installation

1. `waymap.hnmod` nach `horizon-native-mods/` kopieren
2. `mod.waymap:true` in `horizon-mods.txt` hinzufügen
3. Launcher → Build (10–30 min erster Build)

## Entwicklung

```bash
./check.sh     # Mixins + cargo check (Stage/Target unter build/)
./package.sh   # waymap.hnmod bauen (13 Einträge, zip -qrD)
```

## Limite (Native-Architektur)

- **Kein echtes Inventar/Fullscreen-UI**: Nur HUD-Overlay (`native_hud`)
- **Kein Tab-Complete für `/wp`**: `native_command` hat keinen Dispatcher-Hook
- **Chat öffnet → Marker unsichtbar**: HUD wird unter Chat-Overlay nicht gezeichnet
- **Panel mit Closure scheitert**: `draw_extras` hat Closure; `AFTER_ASSIGN`/`OVERWRITE` abgelehnt

## Offizielles Repo

https://github.com/horizon-rust/horizon-mixins
Struktur: `mixins/waymap/` + `fixtures/` + Tests
