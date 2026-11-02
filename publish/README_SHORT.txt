Waymap v0.1.3 — Native Horizon Waypoint Mod
===============================================

CONTENTS (27 files, ~37 KB):

Package:
  waymap.hnmod — Ready-to-install bundle (13 entries, zip -qrD)

Manifest:
  horizon-native.json — id=waymap, version=0.1.3, horizon=d009ebd..., mixins=0d6b103...

Source Code (mixins/waymap/):
  horizon-mixin.toml — 9 injections + 2 modules (frame, list, settings, click,
                       command, key, hud, minimap_view, minimap;
                       Modules waymap + waymap_minimap)
  waymap.rs — Core (~820 lines, State, Persistence, /wp, HUD, Projection)
  minimap_lib.rs — Library module for minimap overlay (draw, set_view, set_dots)
  minimap.rs — Cancellable HEAD on draw_minimap
  minimap_view.rs — HEAD on update_minimap
  frame.rs, list.rs, settings.rs, click.rs, command.rs, key.rs, hud.rs

Documentation (publish/):
  README.md — Complete mod documentation (English)
  RELEASE_CHECKLIST.md — What's still needed for official repo
  NOTE.md — Quick answers to common questions

Build Tools:
  check.sh — Apply + cargo check (offline, Stage/Target under build/)
  package.sh — zip -qrD waymap.hnmod

Project Root:
  README.md — Project overview (English)
  .gitignore — build/ and .hnmod excluded

KNOWN: 2 remaining warnings (edit_panel default, draw_edit_panel atlas)
KNOWN: Chat opening hides HUD (Native limit, not fixable)
KNOWN: /wp commands shown as "unknown" (Native limit, not fixable)

Official Repo: https://github.com/horizon-rust/horizon-mixins
For submission: Fork → mixins/waymap/ + tests + README → PR