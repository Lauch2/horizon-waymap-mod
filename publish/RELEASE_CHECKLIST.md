# Waymap — Release Checklist for Official Repo

## Project and Policy

- [x] Independent native mod code under MIT (`LICENSE`)
- [x] No Minecraft/Mojang assets or data in the bundle
- [x] Source link points to the repository with package source code
- [x] Native mod risk (runs with player permissions) is documented
- [ ] Author/publisher confirms Horizon mod policy in submission form

## Build and Bundle

- [x] Version and hook metadata centrally in `compatibility.json`
- [x] Bundle manifest and compatibility revision are cross-checked
- [x] Format, Clippy, unit test, and package checks pass
- [x] Full build against the pinned Horizon revision passes
- [x] Official registry packer creates byte-identical `.hnmod`
- [x] Registry check with the final, uploadable `.hnmod` passes
- [x] Package is well under the registry limit of 64 MiB

## Publication

- [ ] Publish final build as new version; do not replace existing release files
- [ ] Include hash and minisign signature of the final file in the entry
- [ ] Use the existing registry key's private key to sign the new version (if available)
- [ ] Submit via `playhorizon.dev/mods/submit` and review the automatically generated registry PR
- [ ] Await registry CI and review

## Compatibility and Limits

- Minimum hooks API: `2`
- Tested Horizon commit: `d009ebd140476b5bdc958c00f56a75e82abec691`
- The minimap extension uses internal Horizon functions; therefore, the exact tested
  Horizon commit is pinned.
- The edit panel is not implemented; the mod uses only the settings page and commands.
- The chat can cover HUD markers.
- No inventory/fullscreen GUI (only HUD overlay possible).