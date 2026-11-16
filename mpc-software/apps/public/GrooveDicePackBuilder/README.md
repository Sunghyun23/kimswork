# Groove Dice Pack Builder 1.0.0

A standalone desktop companion for turning folders of WAV and MIDI files into Groove Dice `.gdexp` expansions. No Python installation is required for the packaged app. macOS and Windows are built independently; exact platform validation is recorded in VALIDATION.md.

## Workflow

1. **Open Folder**: choose a source directory. Original files remain unchanged.
2. **Review Mapping**: inspect detected roles and excluded invalid files. Uncertain WAV roles must be assigned. MIDI can be classified as Beats or Grooves.
3. Set pack name, creator, version and rights. Pack ID is generated automatically and lives under **Advanced pack settings**; it identifies the sound pack, not a user account. Keep it unchanged when updating a pack. Personal is the default. Choose Redistributable only when you have the required rights.
4. **Generate Kits**: choose 1–512 kits. Existing named `Kits/<name>/<role>` folders are respected. Edit eight parts at right, set their levels and lock parts or whole kits before regenerating. Rename kits as needed.
5. **Audition Kit**: hear a two-bar reference beat at the selected BPM/Swing. Stop with ■. This preview is a packing aid, not the MPC engine's full FX/groove processor.
6. **Save Project** stores a `.gdproject` with relative source paths, mappings, stable ID, rights, levels and locks. Source files remain external; move them with the project when transferring computers.
7. **Export Expansion** writes a separate `.gdexp`. **Manage Packs** installs/removes packs in a locally accessible Groove Dice `Content` directory. Reopen the plugin after changes. Core is retained.

Input layout:

```text
My Sessions/
  Kick/  Snare/  ClosedHat/  OpenHat/
  Clap/  Tom/  Cymbal/  Perc/
  MIDI/Beats/  MIDI/Grooves/
```

Alternatively use `Kits/<Kit Name>/<Role>/`. Nearest recognized folder wins; filename-prefix detection is a fallback. Missing instruments block generation unless **Allow silent missing roles** is explicitly enabled; then generated silence is used. The app never borrows sounds from other installed libraries. Invalid files are excluded and listed for review.

Supported WAV: uncompressed mono/stereo PCM 8/16/24/32-bit integer,8–192kHz, maximum1,920,000frames and32MB per file. Float/compressed WAV requires conversion first. Up to5000 source files,512 kits and4GB bank. These are format limits, not hardware-performance certification. Only current full-kit packs are emitted; partial sources are completed with explicitly approved silence.

MIDI: standard format0/1 PPQ files. **Auto** keeps native notes60–67 or converts General MIDI drums35–81. Explicit Native/GM choices resolve ambiguous note ranges. Mapping affects the exported bank only; source MIDI is unchanged. Unsupported note sets are rejected rather than silently turning into kicks.

## Running from source

From this app directory:

```sh
python3 -m venv .venv
.venv/bin/python -m pip install -r requirements.txt
.venv/bin/python src/main.py
```

Windows: use `.venv\Scripts\python.exe` instead. Pinned Qt/PyInstaller dependencies are bundled in platform builds. Shared authoring and installer backend: `mpc-software/shared/groove-dice-expansion/`.

```sh
python -m unittest discover -s tests -p test_authoring.py -v
python tests/test_gui.py
python tools/build.py
```

Build on the target OS; The local macOS build and the Windows CI build/test run separately. macOS builds are ad-hoc signed, not notarized. Windows binaries are not code-signed. No signing identities are included. See [third-party notices](https://github.com/Sunghyun23/kimswork/blob/main/mpc-software/apps/public/GrooveDicePackBuilder/THIRD_PARTY.md) and [validation](https://github.com/Sunghyun23/kimswork/blob/main/mpc-software/apps/public/GrooveDicePackBuilder/VALIDATION.md).

The approved image in design/ is a concept. Current app captures and the shipped binaries are separate artifacts. No proprietary drum library is bundled.
