# Groove Dice expansions

Target product: Groove Dice, vendor `kim`, UID `KpGy`. Dedicated `.gdexp` format `GDEXP001`.

- [Original Workshop](public/original-workshop/README.md): public authoring project, original sounds and MIDI, core pack ID `kim-original-workshop`.
- `private/splat-to-clat/`: local-only restricted source, research, import tools, working banks and optional personal `.gdexp`. Excluded from Git and public packages.

Each pack owns its metadata/version and separate payload. Add/remove packs without recompiling the engine; reopen the plugin to refresh the registry. This is not native MPC XPN registration. See [format and CLI instructions](../../plugins/public/GrooveDice/EXPANSIONS.md).
