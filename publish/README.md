# Waymap — Einreichungs- und Release-Informationen

Waymap ist eine native Horizon-Mod (`.hnmod`) für benannte Wegpunkte. Der Mod-Code
läuft mit den Berechtigungen des Spielers; die Mod-Policy und die Native-Mod-Doku
von Horizon gelten daher vollständig.

## Funktionen

- 3D-Wegpunktmarker, projiziert aus der Spielerkamera
- Farbige Wegpunkte auf der Minimap, auch bei gedrehter Karte
- HUD-Liste mit den sechs nächstgelegenen Wegpunkten
- Edit-Overlay mit `M`; Linksklick zeigt Details, Rechtsklick löscht
- `/wp`-Befehle: `add`, `del`, `list`, `rename`, `color`, `clear`, `edit`, `help`
- Persistenz unter `horizon-mods/.horizon-mod-data/waymap/waypoints.txt`
- Einstellungen unter Options > Mods > Waymap

## Build und Prüfung

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
python3 tools/package.py
HORIZON_MODS_REPO=/path/to/horizon-mods ./check.sh full
```

`check.sh full` führt den offiziellen `native/tools/native.py check`-Ablauf aus:
Horizon wird auf der gepinnten Revision vorbereitet, die Mixins werden angewendet
und `horizon-client` wird kompiliert. Der Test benötigt eine lokale Kopie von
`horizon-mods`, Rust und die Build-Abhängigkeiten von Horizon.

## Kompatibilität

| Feld | Wert |
|---|---|
| Mod-ID | `waymap` |
| Version | `0.1.1` |
| Minecraft-Version | `26.3` |
| Horizon-Testrevision | `c9d474dd9116e84c243f8992d75c11314df56f06` |
| Mindest-Hooks-API | `35` |
| horizon-mixins-Revision | `0d6b103c70633d8fd9443d8d0acd3aadaacedb06` |

Die Mod nutzt die stabilen Native-Hooks für HUD, Eingaben, Befehle und Einstellungen.
Die Minimap-Ansicht und das Zeichnen der Minimap werden zusätzlich über interne
Horizon-Funktionen eingespeist. Deshalb bleibt die Mod auf die getestete Horizon-
Revision festgelegt, auch wenn ihre Mindest-Hooks-API angegeben ist.

## Einreichung

Die Horizon-Mod-Registry nimmt native Mods über
[playhorizon.dev/mods/submit](https://playhorizon.dev/mods/submit) an. Das Formular
lädt das `.hnmod` hoch und öffnet den Registry-Pull-Request. Vor dem Einreichen:

1. `./check.sh full` muss auf der gepinnten Revision erfolgreich sein.
2. Der hochgeladene Bundle-Hash muss zum Eintrag passen; veröffentlichte Dateien
   werden nicht nachträglich ersetzt.
3. Die Quelle muss der gebauten Datei entsprechen und unter MIT stehen.
4. Die Bestätigungen der [Mod-Policy](https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md)
   müssen vom Autor/Publisher abgegeben werden.
5. Für Updates muss mit dem im Eintrag hinterlegten Minisign-Schlüssel signiert
   werden. Die lokale Umgebung enthält den privaten Schlüssel nicht; ein Release
   kann hier daher nicht neu signiert werden.

Signieren ist optional, aber empfohlen. Sobald ein Projekt einen öffentlichen
Schlüssel in seinem Registry-Eintrag hat, müssen spätere Versionen mit genau diesem
Schlüssel signiert werden.

### Aktueller Stand für Version 0.1.1

Der vollständige native Build und der offizielle Registry-Packer wurden gegen die
gepinnten Revisionen erfolgreich geprüft. Das Bundle
`waymap-0.1.1.hnmod` hat SHA-256
`9212f8acd583b32dfbfdb80e083729aa74e7750045272bc0f8d308eba014b83a`.
Die Signatur mit dem bereits hinterlegten privaten Schlüssel fehlt noch; bis dahin
nicht einreichen. Das veröffentlichte Bundle `0.1.0` und seine Signatur bleiben
unverändert.

## Bekannte Einschränkungen

- Das Verwaltungsfenster ist ein HUD-Overlay, kein eigener Mod-Screen.
- Der Chat kann HUD-Marker überdecken.
- Der Minimap-Patch hängt von internen Horizon-Funktionen ab und begrenzt daher die
  Kompatibilität auf die getestete Revision.
