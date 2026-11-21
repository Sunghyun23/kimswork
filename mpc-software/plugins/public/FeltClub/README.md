# Felt Club 0.3.2

A sample-free, physically informed instrument by **kim** for MPC standalone. Seven independent engines cover Tine, Wurli, Upright, Felt Piano, Clav, palm-muted Guitar and Thumb. Warm chords, tight rhythmic parts and restrained motion share one vintage interface.

- [Product and download](https://kimswork.app/plugins/felt-club/)
- [Complete user manual](https://kimswork.app/plugins/felt-club/manual/)
- [Illustrated physical-modeling guide](https://kimswork.app/learn/physical-modeling/)

## CPU notice

Known limitation: the owner reported MPC CPU peaks above 100% during playing after this optimization. The triggering engine and exact workload were not isolated. Worst-case hardware CPU is not certified; this release does not claim that overload is fixed.

## What is new

All 57 factory programs have been revoiced around the individually developed engines: Init plus eight sounds per model. A dry reference starts each model’s set; the remaining recipes cover softer/darker voicings, rhythmic response, stereo motion and controlled effects. Names, program indices, UID `KnAw`, all original 238 parameter identities and the approved interface are retained. Existing project chunks retain their stored values; deliberately selecting a factory program loads its new recipe.

Roll Dice keeps the selected model, Sync, velocity curve, keyboard/pad mode and all chord/voicing/key/slot/MIDI settings. Each model has its own bounded Tone, Body, character, Hammer, Decay, Release, Detune, Drift and motion-depth ranges. Body and Release are drawn together. It generates at most one unlocked color effect; any enabled locked color effect suppresses new unlocked color effects. Mechanical and key-off noise are not generated. A small continuous hiss is occasionally drawn; its own lock and Undo apply. Individual locks protect values even outside the generated ranges; Undo restores the previous sound and random state without changing current locks.

## Play

Load under **External Plugins**. MAIN starts immediately. Select EP / PIANO / PLUCK, then a model; use the common bottom SOUND PRESET browser or the MPC’s top PRESET selector for the factory bank. Category-first navigation is provided inside the plugin; the top preset list belongs to the MPC.

MAIN Q-Link Screen groups:

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A Character | Tone | Body | Bell/Reed/String/Bite | Noise |
| B Amp envelope | Attack | Decay | Sustain | Release |
| C Motion | Rate | Depth | Drift | Width |
| D Color / FX | Drive | Chorus | Delay | Room |

The 72px bakelite knobs have a stationary face/light and a separate moving pointer. Tap the upper-left lock to protect each sound value. UNLOCK ALL / ROLL DICE / UNDO and the sound browser stay in the common footer on every page.

Tap MAIN again for CHORD: up to16slots, key/transpose, chord type/inversion/octave, top-note smoothing, strum and overlapping chord glide. Touch chords sound while held and finish through Release. Physical pad notes are individually assignable; learn maps incoming notes without changing MPC Pad Perform settings. Direct keys, touch chords and MIDI chord triggers have separate ownership. Details are in the manual.

## Install

Requires **MPC OS3.10.1 or later** and Preferences → Enhancements (Beta) → **Enable Enhancements** and **External Plugin Support**. Unzip the SDK download and copy `kim - VST - Felt Club/` into a drive’s `VST2 Plugins/`. Save your work and restart the MPC app to scan it. Do not keep duplicate copies on multiple mounted drives.

The release includes both ARM32 and ARM64. Optional per-architecture portable packages are offered separately for Open MPC installers. Never change the UID or parameter IDs in existing projects.

## Build and test

From the repository root:

```sh
python3 mpc-software/scripts/bootstrap_sdk.py
python3 mpc-software/plugins/public/FeltClub/tools/build.py all
python3 mpc-software/plugins/public/FeltClub/tools/test.py
```

Noise and its lock are appended as parameters 238/239, for 246 total, including six retired inert action parameters. MAIN A4 is continuous, gently band-limited tape-like hiss; former Bark/Board/Mute remains on MODEL and Mechanical stays on TOUCH. Noise 0 is silent and retains the existing engine output.

The generator reads `factory-sounds.json` and validates all57names/order/ranges before generating `FactorySounds.h`. `SoundProfiles.h` contains host-level Dice bounds; the independent mechanics remain under `src/engines/`. One pinned, unmodified inMusic SDK is shared by the monorepo.

Native, ARM32 and ARM64 builds and SDK validation pass. Automated checks cover1539factory strikes at 3 rates/registers/velocities,266individual lock/Undo cases,448bounded Dice rolls, all-engine isolation, RT allocation guards, MIDI/pedal/strum/chord release, state recall and skin/Q-Link structure. The final native SDK host reports19PASS/0WARN/0FAIL. Hardware verification scope is kept separately in the manual and release notes; desktop tests do not certify MPC audio, worst-case CPU or all device models.

## Limits and credits

Reduced physical models combine original mechanical approximations with empirical voicing. They are not calibrated reproductions of a specific real instrument, commercial plugin or recording. A model’s physical vibration loses energy even with Sustain high; a long Release does not create new string energy. No third-party recorded samples, reference audio, commercial binaries, measured IRs or proprietary code are in the package.

Original plugin code/artwork are MIT, see LICENSE. Original chord helpers derive from this repository’s MIT Donut Theory project. The shared SDK is MIT; its separate notice is included as SDK-LICENSE.txt in the installed folder. The illustrated website guide links research sources and distinguishes assumptions, native tests and actual device checks.

## Current device scope

MPC XL / OS3.10.1.12, NVMe:600 files SHA-256 verified after the authorized restart. Remote checks cover all 7 pages, category/host preset display, Noise 60%, Tone/Noise locks, engine-preserving Dice and visible Undo; final CHORD contrast checked. Physical controls, hardware audio, full MPC recall and worst-case CPU remain unverified. See design/device-0.3.0/verification.md.

## Local CPU optimization candidate0.3.2

Installed on the owner MPC internal NVMe after full regression/build checks and600-file independent SHA256readback. Preserves24voices, all seven isolated physical models, UI/presets/246IDs. Native six-note musical processing time improves around25–67percent depending on model; these are Mac measurements, not MPC CPU percentages. Actual owner playing/CPU confirmation is pending. See [CPU evidence](design/validation-0.3.2/README.md). No new public release is implied.

## 0.3.2 release scope

All600 NVMe runtime files match the final build by SHA-256. Native/bothARM/SDK/full regression and host19PASS/0WARN/0FAIL pass. The owner accepted the current pad-command removal/startup checklist. Desktop CPU improvements preserve77 matched audio fixtures; actual hardware overload remains a known limit. No further CPU edits were requested after the owner chose publication as-is.
