# Groove Dice Pack Builder

Cross-platform desktop companion for creating Groove Dice expansion packs from folders of user-owned WAV and MIDI files.

Status: design and specification stage. A desktop application has not been implemented or tested on Windows. The existing plugin's command-line expansion tools are a separate working component.

Confirmed requirements:

- Independent project in the existing monorepo, alongside the MPC plugin.
- macOS and Windows executables with the required runtime bundled.
- Select a root folder; detect instrument subfolders and optionally named kit folders.
- Review automatic role mapping, edit and audition generated kits, and export independent `.gdexp` files.
- Keep installed expansion payloads separate. New audio packs must not require rebuilding the MPC engine.
- Support local pack installation/removal and stable identities for updates.
- Separate personal-only content from redistributable output.

Proposed implementation: PySide6 with a shared Python packing/validation backend. Platform builds and runtime testing must be performed separately on macOS and Windows. Do not claim cross-platform validation from a Mac-only build.

Proposed UI: pack metadata and source tree at left, instrument mapping and waveform audition in the center, generated kits at right, with import/map/generate/export actions in a fixed bottom bar. The English design uses espresso brown, warm ivory and amber.

The first design image is a concept, not a screenshot of a working app. No proprietary samples are included in this project. The standalone format/packing backend is at mpc-software/shared/groove-dice-expansion/; the plugin delegates to it and the future app will consume it. The single pinned SDK stays in mpc-software/third_party/ and is not copied into this app.

[Implementation proposal](../../../docs/apps/groove-dice-pack-builder/proposal.md).
