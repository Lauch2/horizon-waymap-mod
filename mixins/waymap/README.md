# Waymap (Native Mod)

Native `.hnmod` bundle for Horizon.
- ID: `waymap`
- Version: `0.1.0`
- Pins: horizon `d009ebd140476b5bdc958c00f56a75e82abec691`, mixins `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`
- Hooks: `2` (`native_frame`, `native_hud`, `native_key`, `native_command`, `native_mods`, `native_settings`)
- Source: https://github.com/horizon-rust/horizon-mixins (fork + PR for `mixins/waymap/`)
- License: MIT (workspace)

## Limits (documented, architecture-level)
- Chat open → HUD markers hidden (`draw_extras` stops)
- `/wp` commands work but have no Chat tab-completion
- No fullscreen / inventory GUI (only HUD overlay)
