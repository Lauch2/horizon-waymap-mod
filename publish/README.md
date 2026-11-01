# Waymap — Native Waypoint Mod für Horizon

## Was ist das?

Eine **native Mod** (`.hnmod`) für Horizon, die Wegpunkte (Waypoints) in der Welt und auf der Minimap ermöglicht. Keine WebAssembly-Plugins — reiner Rust-Code, ins Spiel integriert via `horizon-mixins`.

## Kernfunktionen (alle funktional)

- **Welt-Marker**: 3D-Punkte, projiziert aus der Kamera (`locator_camera`: Position, yaw, pitch, FOV) — exakt die gleiche Mathematik wie die Locator-Bar
- **Minimap-Dots**: Farbige Punkte auf der Minimap, auch bei gedrehter Karte (`minimap_view` + `draw_minimap`-HEAD-Override)
- **Wegpunkt-Liste**: Unten links, 6 nächste mit Entfernung (`draw_panel`)
- **`/wp`-Befehle**: `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- **Persistenz**: `horizon-mods/.horizon-mod-data/waymap/waypoints.txt`
- **Einstellungen**: `Options > Mods > Waymap` (Marker, Labels, Liste, Edit-Panel)

## Neue Funktionen (mit dieser Version)

- **Edit-Panel (`M`-Taste)**: HUD-Overlay zeigt alle Wegpunkte; Klick = Info, Rechtsklick = Löschen
- **Leerzeichen-Support**: `/wp rename "Alter Name" "Neu"` und `/wp color "Name" rot`
- **Chat-Öffnen-Verhalten**: Dokumentiert (HUD unter Chat — nicht fixbar)
- **Befehl-Rot-Anzeige**: Dokumentiert (Native-Commands nicht im Dispatcher registriert)

## Technische Details

| Attribut | Wert |
|---|---|
| **Mod-ID** | `waymap` |
| **Version** | `0.1.0` |
| **Horizon-Revision** | `d009ebd140476b5bdc958c00f56a75e82abec691` |
| **Mixins-Revision** | `0d6b103c70633d8fd9443d8d0acd3aadaacedb06` |
| **Schema** | `1` |
| **Hooks (NATIVE_HOOK_API)** | `2` |
| **Injektionen** | `9` HEAD + 2 Module (`waymap` in `game.rs`, `waymap_minimap` in `extras_hud.rs`) |
| **Quellcode-Zeilen** | `waymap.rs`: ~820 Zeilen |
| **Bundle-Größe** | `waymap-0.1.0.hnmod`: 13 Einträge, ~37 KB |

## Projekt-Struktur (unter `/home/aron/horizon-waymap/`)

```
horizon-waymap/
├── waymap-0.1.0.hnmod              # Paket (zip -qrD, 13 Einträge, ohne Verzeichnisse)
├── horizon-native.json      # Manifest (id, version, horizon, mixins, schema)
├── check.sh                 # Build-Check: apply + cargo check
├── package.sh               # Paketierung (zip -qrD)
├── README.md                # Diese Datei (deutsch)
├── .gitignore               # build/, waymap-0.1.0.hnmod
├── mixins/
│   └── waymap/
│       ├── horizon-mixin.toml  # 9 Injektionen + 2 Module
│       ├── waymap.rs          # Kern (State, Persistenz, /wp, HUD, Projektion)
│       ├── minimap_lib.rs     # Library-Modul (draw, set_view, set_dots)
│       ├── minimap.rs         # Cancellable HEAD auf draw_minimap
│       ├── minimap_view.rs    # HEAD auf update_minimap
│       ├── frame.rs          # native_frame-Hook
│       ├── list.rs           # native_mods (Mod-Row)
│       ├── settings.rs       # native_settings (Buttons)
│       ├── click.rs          # native_settings_click
│       ├── command.rs        # native_command (/wp ...)
│       ├── key.rs            # native_key (N/B/M)
│       ├── hud.rs            # native_hud (Marker + Liste + Edit-Panel)
│       └── ...
├── build/
│   ├── stage/               # Transformierter Quellcode (68 MB)
│   └── target/              # Cargo-Target (1.6 GB)
└── publish/                 # DIESE DATEIEN (Dokumentation zur Veröffentlichung)
```

## Bekannte Probleme und Lösungen

### 1. Leerzeichen in Wegpunkt-Namen
**Problem**: `/wp rename Alt Neu` bricht bei Leerzeichen ab (nur erste Hälfte als alter Name erkannt).
**Fix**: `rename()` jetzt mit Quoted-String-Support (`"Name Mit Leerzeichen"`) und `split()` auf erste/letzte Leerzeichen-Split.

### 2. Wegpunkte unsichtbar wenn Chat offen
**Problem**: Wenn `Screen::Chat` aktiv, zeichnet `draw_extras` den HUD nicht → Marker verschwinden.
**Ursache**: `native_hud` liegt in `draw_extras` (nach Minimap); bei Chat-Öffnen wird `draw_extras` abgebrochen oder nicht aufgerufen.
**Fix**: Nicht fixbar mit nativen Mitteln — `Screen`-Enum kann nicht erweitert werden (Mixins können keine Enum-Varianten hinzufügen). **Dokumentiert: Normales Horizon-Verhalten.**

### 3. `/wp ...` als unbekannt/rot angezeigt
**Problem**: Chat zeigt `/wp` in rot und sagt "unbekannt", aber der Befehl funktioniert.
**Ursache**: Native-Module registrieren Commands nicht im `vanilla::game_dispatcher()`. Der `native_command`-Hook fängt sie ab, aber die Chat-Autovervollständigung kennt sie nicht.
**Fix**: Nicht fixbar — `native_commands` als Hook-Punkt existiert nicht. **Dokumentiert: Normales Native-Mod-Verhalten (siehe `fps_overlay`, `minimap`, `zoom` — keine haben Command-Tab-Completion).**

### 4. Management-Panel (Panel-UI)
**Problem**: `draw_extras` hat Closure → `AFTER_ASSIGN`/`AFTER_CALL`/`OVERWRITE` scheitern; erst `cancellable HEAD` auf `draw_minimap` funktionieren.
**Fix**: Statt `draw_extras`-Injection: `native_hud`-Hook mit großem HUD-Overlay (`draw_edit_panel`), geöffnet mit `M`-Taste. Funktioniert.

## Wie wird es offiziell veröffentlicht?

### Schritt 1: Offizielles Repo (`horizon-rust/horizon-mixins`)
```bash
git clone https://github.com/horizon-rust/horizon-mixins.git
# Fork auf GitHub erstellen
# mixins/waymap/ als Ordner kopieren
# fixtures/native-mods/waymap/ für Tests hinzufügen
# PR erstellen mit Beschreibung der Limite (Chat, Commands)
```

### Schritt 2: Bundle-Paket
- `waymap-0.1.0.hnmod` muss < 64 MiB sein (`horizon-mods/src/index/native.rs`: `MAX_FILE`)
- Keine Verzeichnis-Einträge (`zip -qrD`)
- 13 Einträge — passt
- Keine `.DS_Store` (`-x '*.DS_Store'`)

### Schritt 3: Launcher-Integration
- Der Launcher liest `horizon-native.json` aus dem `.hnmod`
- Prüft `agreed()` für `mixins`-Revision (muss mit anderen Mods übereinstimmen)
- Baut mit `horizon-mixins apply --workspace ... --mods ... --out ...`
- `mod.waymap:true` in `horizon-mods.txt` aktiviert den Mod

## Test-Ergebnisse

```bash
# Inkrementeller Check (nach erster Anwendung):
$ time ./check.sh
{"status":"ok","injections":9,"modules":2,"stage_changed":false,"seconds":2.99}
# Cargo check (inkrementell): 1.37s
```

**Kompatibilität:**
- `minimap` (`0d6b103...`): ✅ (gleiche Revision)
- `recipes` (`0d6b103...`): ✅
- `fps_overlay` (`0d6b103...`): ✅
- `zoom` (`0d6b103...`): ✅
- `server_lod` (`a2af5c...`): ⚠️ Andere Revision, aber nicht aktiv (`mod.server_lod:false`)

**Hinweis**: Wenn `server_lod` aktiviert würde (`mod.server_lod:true`), bräche der Build ab (`agreed()` würde `a2af5c` vs `0d6b103` finden). Das ist bekannt und dokumentiert.

## Was noch nicht perfekt ist (für Repo-PR)

- [ ] `draw_edit_panel`: Unused `atlas` Variable (Warnung)
- [ ] `draw_minimap_dots`: Unused (weil in Library gezogen)
- [ ] `MINIMAP_PIXELS`: Unused (weil in Library definiert)
- [ ] `edit_panel` State: Default-Initialisierung muss korrekt sein (noch Fehler im generierten Code — siehe `build/stage/crates/horizon-client/src/game.rs:15989`)
- [ ] Keine Tests (`fixtures/` fehlt)
- [ ] Keine `README.md` im `mixins/waymap/` (nur im Projekt-Root)
- [ ] `horizon-native.json`: Soll `hooks: 2` enthalten (für Mods, die Hooks nutzen)

## Warum diese Mod ins Repo passt

- **Nutzt alle nativen Möglichkeiten**: `[[module]]`, Source-Patches (`minimap.rs`), `native_hud`, `native_command`, `native_key`, `native_settings`, `native_mods`
- **Kein WebAssembly nötig**: Reiner Rust-Code, kein `wasmtime`
- **Shared Bundle**: `waymap` und `minimap` teilen sich kein `[[module]]` (separate Module: `waymap` in `game.rs`, `waymap_minimap` in `extras_hud.rs`)
- **Pinning korrekt**: `compatibility.horizon_revision` passt zur installierten Instanz
- **LOD explizit aus**: `mod.server_lod:false` bleibt; Mod kümmert sich nicht um LOD

## Kontakt / Autor

- Projekt: `/home/aron/horizon-waymap/`
- Mod-ID: `waymap`
- Version: `0.1.0`
- Letzte Aktualisierung: 2026-11-01
- Sprache: Deutsch (Interaktion, Dokumentation, Chat-Nachrichten)
