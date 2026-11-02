# Waymap — Was du wissen musst

## Schnelle Antwort auf deine Fragen

**Q: Warum sind meine /wp-Befehle rot / nicht erkannt?**
A: Native Mods registrieren Befehle nicht im Chat-Dispatcher. Der Befehl funktioniert (`/wp add` usw.), aber die Autovervollständigung kennt ihn nicht. Das gilt für ALLE native Mods (fps, minimap, zoom, etc.).

**Q: Warum sind Wegpunkte unsichtbar wenn Chat offen?**
A: `native_hud` wird von `draw_extras` aufgerufen — wenn Chat offen ist, zeichnet das Spiel den HUD nicht. Das ist Horizon-Verhalten, nicht ein Fehler der Mod.

**Q: Kann ich ein echtes Inventar/Fullscreen-Menü erstellen?**
A: Nein. Native Mods können nur HUD-Overlays (`corners.place`) zeichnen. Kein `Screen`-Enum-Extension, kein Inventar-Access, kein echtes Fullscreen-UI.

**Q: Wie verwalte ich Wegpunkte?**
A: `N` = hinzufügen, `B` = Liste unten links, `M` = Edit-Panel (HUD), `/wp edit <Name>` = Info, Settings-Button = Toggle.

**Q: Wie geht es ins offizielle Repo?**
A: Fork `horizon-rust/horizon-mixins`, `mixins/waymap/` + tests + README + PR. 2 kleine Fehler fixen (`atlas`, `edit_panel`).

## Dateien im Publish-Ordner
- `README.md` — Alles zur Mod (Features, Installation, Limits)
- `RELEASE_CHECKLIST.md` — Was noch fehlt für Repo-Veröffentlichung
- `build/` — Build-Artefakte (nicht für Repo, nur lokal)
- `waymap.hnmod` — Fertiges Paket (bereit zur Installation)
