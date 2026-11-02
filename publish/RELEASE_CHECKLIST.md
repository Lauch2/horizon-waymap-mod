# Waymap — Registry-Einreichungscheckliste

## Projekt und Policy

- [x] Eigenständiger Native-Mod-Code unter MIT (`LICENSE`)
- [x] Keine Minecraft-/Mojang-Assets oder -Daten im Bundle
- [x] Source-Link zeigt auf das Repository mit dem Paketquellcode
- [x] Native-Mod-Risiko (läuft mit Spielerrechten) ist dokumentiert
- [ ] Autor/Publisher bestätigt im Submit-Formular die Horizon-Mod-Policy

## Build und Bundle

- [x] Versions- und Hook-Metadaten liegen zentral in `compatibility.json`
- [x] Bundle-Manifest und Kompatibilitätsrevision sind aufeinander geprüft
- [x] Format-, Clippy-, Unit-Test- und Package-Checks bestehen
- [x] Vollständiger Build gegen die gepinnte Horizon-Revision bestehen lassen
- [x] Offizieller Registry-Packer erzeugt byte-identisches `.hnmod`
- [x] Registry-Prüfung mit dem finalen, hochzuladenden `.hnmod` bestehen lassen
- [x] Paket ist deutlich kleiner als das Registry-Limit von 64 MiB

## Veröffentlichung v0.1.1

- [ ] Finalen Build als neue Version 0.1.1 veröffentlichen; vorhandene Release-Dateien
  nicht ersetzen
- [ ] Hash und Minisign-Signatur der finalen Datei `waymap-0.1.1.hnmod` in den Eintrag übernehmen
- [ ] Bei vorhandenem Registry-Schlüssel den zugehörigen privaten Schlüssel zum
  Signieren der neuen Version verwenden
- [ ] Über `playhorizon.dev/mods/submit` einreichen und den automatisch erzeugten
  Registry-PR prüfen
- [ ] Registry-CI und Review abwarten

## Veröffentlichung v0.1.0 (vorhanden)

- [x] Finaler Build für Version 0.1.0 bereits veröffentlicht
- [x] Hash und Minisign-Signatur der finalen Datei `waymap-0.1.0.hnmod` im Eintrag gespeichert
- [x] Vorhandener Registry-Schlüssel für die alte Version verwendet

## Kompatibilität und Limits

- Mindest-Hooks-API: `35` (Native-Command-Registrierung)
- Getesteter Horizon-Stand: `c9d474dd9116e84c243f8992d75c11314df56f06`
- Die Minimap-Erweiterung nutzt interne Funktionen; daher ist der getestete
  Horizon-Commit fest gepinnt.
- Das Edit-Panel ist ein HUD-Overlay; der Chat kann HUD-Inhalte überdecken.