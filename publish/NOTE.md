# Waymap — What You Need to Know

## Quick Answers to Your Questions

**Q: Why are my `/wp` commands shown in red / not recognized?**
A: Native mods don't register commands in the chat dispatcher. The command works (`/wp add` etc.), but autocomplete doesn't know about it. This applies to ALL native mods (fps, minimap, zoom, etc.).

**Q: Why are waypoints invisible when chat is open?**
A: `native_hud` is called from `draw_extras` — when chat is open, the game doesn't draw the HUD. This is Horizon behavior, not a mod bug.

**Q: Can I create a real inventory/fullscreen menu?**
A: No. Native mods can only draw HUD overlays (`corners.place`). No `Screen` enum extension, no inventory access, no real fullscreen UI.

**Q: How do I manage waypoints?**
A: `N` = add, `B` = toggle list panel, `/wp edit <name>` = info, Settings button = toggle.

**Q: How do I get it into the official repo?**
A: Fork `horizon-rust/horizon-mixins`, add `mixins/waymap/` + tests + README + PR. Fix 2 small issues (`atlas`, `edit_panel`).

## Files in Publish Folder
- `README.md` — Everything about the mod (features, installation, limits)
- `RELEASE_CHECKLIST.md` — What's missing for official repo submission
- `build/` — Build artifacts (not for repo, local only)
- `waymap.hnmod` — Ready-to-install package