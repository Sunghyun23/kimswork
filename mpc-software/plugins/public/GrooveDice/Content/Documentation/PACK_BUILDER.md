# Make your own Groove Dice expansion

Groove Dice Pack Builder 1.0.0 is an optional, separate desktop application. The instrument already includes Original Workshop and does not require Pack Builder to play. The builder does not install on the MPC.

- Overview: https://kimswork.app/apps/groove-dice-pack-builder/
- Full manual: https://kimswork.app/apps/groove-dice-pack-builder/manual/
- Releases/checksums: https://github.com/Sunghyun23/kimswork/releases/tag/groove-dice-pack-builder/v1.0.0
- macOS Apple Silicon: https://github.com/Sunghyun23/kimswork/releases/download/groove-dice-pack-builder/v1.0.0/groove-dice-pack-builder-1.0.0-darwin-arm64.zip
- Windows x64: https://github.com/Sunghyun23/kimswork/releases/download/groove-dice-pack-builder/v1.0.0/groove-dice-pack-builder-1.0.0-windows-amd64.zip

Packaged apps include their runtime; no Python installation is required. The Mac bundle is ad-hoc signed, not notarized, and the Windows app is unsigned. Intel Mac and Windows ARM builds are not provided by this release. Keep bundled third-party notices; Pack Builder's dependencies and licensing are separate from the plugin.

## 1. Organize your own source files

```text
My Pack/
  Kick/
  Snare/
  ClosedHat/
  OpenHat/
  Clap/
  Tom/
  Cymbal/
  Perc/
  MIDI/
    Beats/
    Grooves/
```

Alternatively use `Kits/<Kit Name>/<Role>/`. WAV folders are required to supply the eight roles; MIDI folders are optional. The nearest recognized folder determines the role. Use material you own or have permission to use. The app never borrows samples from other installed libraries to fill gaps.

## 2. Review and build

1. Open Folder and review detected instrument roles and invalid/excluded files.
2. Enter pack name, creator and version. Personal is the default. Select Redistributable only when your source permissions allow it; the selection does not grant rights.
3. Generate 1–512 kits, review each part, adjust levels and lock the combinations you want to retain.
4. Audition the two-bar reference beat at your BPM/Swing. This is a packing preview, not the full MPC engine/FX.
5. Save a `.gdproject` to retain mappings, source paths, metadata, levels and locks. Keep it with its external source folder.
6. Export Expansion to create a separate `.gdexp` file. Keep the generated stable Pack ID unchanged for future updates; it is a pack identifier, not a user account.

Uncompressed mono/stereo integer PCM WAV8/16/24/32-bit at8–192kHz is supported; floating-point/compressed WAV requires conversion. MIDI format0/1 PPQ can use Groove Dice native notes60–67 or General MIDI drums; choose the mapping if Auto is ambiguous. Unsupported data is reported. Missing roles require explicit permission to insert silence.

## 3. Install on accessible MPC storage

In Manage Packs, choose the locally accessible `Content` directory of the installed `kim - VST - Groove Dice` folder under `VST2 Plugins`, then install the `.gdexp`. For example, use the actual MPC SD card or USB drive mounted on your computer. Selecting an arbitrary local folder does not transfer it to the MPC. Only use private opt-in for your own nonredistributable packs.

The installer creates `Content/Expansions/<stable-pack-id>/` and updates its registry. It does not merge samples into the core bank or rebuild the engine. Safely eject external media, return it to the MPC and close/reopen Groove Dice so it reads the updated registry. Keep one installed plugin copy across drives.

## 4. Select and play

Open the kit browser, choose the new pack, then select a kit. The MPC PRESET header also lists installed kit programs. Roll Dice can use This Pack, All Packs or selected kits; Kit Fixed preserves the current kit. Pack addition/removal is picked up when a new instance opens.

Use Manage Packs to remove an optional pack. The original core is retained. Removing a pack used by a saved project can produce MISSING SAMPLE/PATTERN/GROOVE; reinstall the same Pack ID and reopen/reload to reconnect it. Back up the `.gdexp` and important MPC projects.

This is Groove Dice's own expansion format, not the MPC XPN Expansion Browser. Exported packs keep the creator's source rights. Public Groove Dice downloads contain Original Workshop only; personal/commercial packs are not bundled.
