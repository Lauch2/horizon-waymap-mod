# Submission status

The current registry workflow is the submit form at
https://playhorizon.dev/mods/submit. It uploads the `.hnmod` and opens a
pull request in `horizon-rust/horizon-mods`; a pull request to `horizon-mixins`
is not the submission path for a native mod.

The existing GitHub release `v0.1.0` must remain immutable. Changes to the bundle
need a new version and a new release asset. The project already has a minisign
public key; its private key is not available in this environment, so an updated
release cannot be signed here. Do not reuse the old `.minisig` for a changed
bundle.

Version `0.1.1` has been built and checked against Horizon revision
`c9d474dd9116e84c243f8992d75c11314df56f06`. The bundle is
`waymap-0.1.1.hnmod` (SHA-256
`9212f8acd583b32dfbfdb80e083729aa74e7750045272bc0f8d308eba014b83a`).
It is not ready to submit until it is signed with the existing private key and
the signature is added to the registry entry.
