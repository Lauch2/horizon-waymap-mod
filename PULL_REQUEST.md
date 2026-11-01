**Project:** waymap — new native mod (0.1.0)

- [x] I made it, or I am allowed to publish it, under the licence the entry names (MIT).
- [x] If it ports or translates another mod, the original's licence allows it (or its authors agreed), and my page
      credits and links the original. _(Not a port — original native mod.)_
- [x] It has no code, assets or data from Minecraft, Mojang or Microsoft, and nothing that harms players.
- [x] I read and accept the [mod policy](https://github.com/horizon-rust/horizon-mods/blob/main/MOD-POLICY.md): I am this
      project's author or publisher and responsible for it; the Horizon project is not.
- [x] The `source` link has the code this file was built from: https://github.com/Lauch2/horizon-waymap-mod
- [x] I ran `cargo horizon-mod check <file>` and it passed.

<!-- submitted-by: Lauch2 -->

## Waymap 0.1.0 — native waypoint management mod

Native mod (`native-mod`, `.hnmod`) for Horizon 26.3, built with horizon-mixins 0.6.0.

- World markers projected from the camera (`locator_camera`)
- Colored dots on the minimap (`minimap_view` + `draw_minimap` head override)
- Waypoint list in the HUD (six nearest, with distance)
- Edit panel (`M`): left-click info, right-click delete
- `/wp` commands: add, del, list, rename, color, clear, edit, help
- Persistence: `horizon-mods/.horizon-mod-data/waymap/waypoints.txt`
- Settings: Options > Mods > Waymap

Pins: Horizon `d009ebd140476b5bdc958c00f56a75e82abec691`, Mixins `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`, `hooks: 1` (uses only native hook points, `NATIVE_HOOKS_API` 1).

### Known limitations (Horizon native architecture, not bugs)

- No fullscreen / inventory UI — native mods can only draw HUD overlays (`native_hud`); the `Screen` enum cannot be extended.
- No tab completion for `/wp` — native commands are not registered in the chat dispatcher; they still work.
- Markers hidden while the chat screen is open — `draw_extras` stops drawing the HUD when `Screen::Chat` is active.
- Panel with closure fails — `draw_extras` has a closure, so `AFTER_ASSIGN`/`OVERWRITE` injections are rejected; the edit panel uses a cancellable head on `draw_minimap` instead.
