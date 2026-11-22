# Loop Club

A16-track audio/MIDI looper for **MPC Sample**, by **kim**. Public preview0.1.0 targets firmware1.3.0.12. This is an SD preload mod, not an MPC OS VST or a firmware image.

[Manual](docs/manual.md) · [Installation](docs/public-install.txt) · [Verification](docs/release-0.1.0.md) · [Source specification](../../../docs/looper-srs.md)

- Independent bar-aligned audio, synth and drum loops; count-in; Replace/Overdub; MIDI grid.
- Two-oscillator synth, separate amp/filter ADSRs, six-shape LFO to Pitch/Filter/Volume/Pan.
- Original procedural drums, musical velocity accents and optional standalone user banks.
- Two layers per drum sound: sample selection, Volume/Tune/relative Shift, Decay/enable/REV.
- Pad-shaped track/drum mixers, pickup fader and per-track MAIN/USB stereo destinations.
- Direct4×4FXpad boards with tap ON/OFF: two simultaneous track slots, five master texture stages, Duck and two saved custom master effects.
- Sidechain from an individual drum or whole track, to other/custom tracks or the full master.

Native checks, real active-DRM screens, USB audio and remote-control tests are recorded separately from unverified physical inputs. The32voice cap is not an unlimited CPU guarantee; brief scheduling-inclusive peaks have exceeded the callback budget. USB multichannel drum stems, USB-input recording, time stretching and source-bank MIDI import are not implemented. See the manual for exact limits.

## Build

From the repository root:

```sh
python3 mpc-software/scripts/bootstrap_sample_sdk.py
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/run.py test -- --test-threads=1
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/run.py clippy --all-targets -- -D warnings
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/build.py --pack
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/package.py --public --direct
```

Use the pinned SDK and repository-local toolchain. A SHA-pinned input dispatch patch is applied only to temporary staging; the canonical SDK stays clean. See [development](../../../docs/development.md). QA upload/agent tools are opt-in development utilities, not part of the public ZIP. Runtime-only updates disappear at power-off until explicitly persisted.

Original source is [MIT](LICENSE); the SDK retains its own license. No firmware, commercial WAV/MIDI, personal expansion, owner project, or management agent is included in the public download. Existing card boot files must be backed up before installation; projects and expansions remain outside autorun.
