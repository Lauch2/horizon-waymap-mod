# Waymap (Native Mod)

Native `.hnmod` bundle for Horizon.

- ID: `waymap`
- Version: `0.1.1`
- Horizon tested revision: `c9d474dd9116e84c243f8992d75c11314df56f06`
- Minimum Native Hooks API: `35` (`native_commands` registers `/wp`)
- horizon-mixins revision: `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`
- License: MIT

The bundle uses native hooks for frame updates, mod settings, commands, keys and
HUD rendering. The minimap view and drawing are patched through internal Horizon
functions, so the exact tested Horizon revision remains pinned.

See the project README for build instructions and the documented HUD/chat limits.
