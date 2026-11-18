# Donut Theory

**kim · 1.10.1 · UID `Krql` · MPC standalone instrument**

Warm bass, keys, analog-style leads and pads with an original Moog-inspired nonlinear mixer, four-stage ladder low-pass and rounded VCA. Includes 66 factory sounds, 32 chord types, 96 four-chord recipes and 4–16 playable chord slots. No external samples are required. MAIN ROLL DICE generates a complete sound with all three Sync switches ON, preserving chord settings, MIDI mapping and master Volume.

[User manual](../../../docs/plugins/donut-theory/manual.md) · [Release notes](../../../docs/plugins/donut-theory/releases/1.10.1.md) · [Product page](https://kimswork.app/plugins/donut-theory/) · [Development handoff](../../../docs/plugins/donut-theory/handoff.md)

## Performance surface

| MAIN Q-Link group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A · OSC / SOUND | OSC Pair | OSC Mix | Detune | Drift |
| B · FILTER / TONE | Filter Cutoff | Resonance | Body | Sub |
| C · LFO 1 / 2 | LFO 1 Rate | LFO 1 Depth | LFO 2 Rate | LFO 2 Depth |
| D · FX / MOTION | Drive | Glide | Mod Rate | Mod Depth |

Each LFO has independent shape/target/sync. Shared Motion selects Tremolo or Auto Pan, with its own sync. Amp and filter ADSRs stay on AMP/FILTER detail pages. Body adds low-mid density; Sub adds an octave-below sine. MAIN again opens CHORD; SCREEN Q-Links become Root/Type/Voicing/Octave for its four visible slots. Key/circle-of-fifths reference, explicit transpose, smooth voicing, overlapping chord glide and one-shot strum support chord design. Sound selection preserves chords, MIDI mappings, master Volume and performance settings.

## Factory sounds

Init 1 / Bass 12 / Keys 16 / Synth 13 / Lead 12 / Pad 12. Original 64 programs stay in place; Dusty Sunday and Velvet Melt append at 65/66. Try them at 88 BPM with their eighth-note Auto Pan. Host PRESET is a flat list; the internal SOUND PRESET browser chooses category first. Init has effects/low-cut off; other patches use per-sound low-cut and trim.

## Install / update

Requires **MPC OS 3.10.1+**, **Enable Enhancements** and **External Plugin Support** in Preferences → Enhancements (Beta). Extract the official ZIP and copy the complete `kim - VST - Donut Theory` folder into `VST2 Plugins` on the MPC drive, preferably internal storage. Restart MPC and choose External Plugins → Donut Theory. Keep only one installed copy. The native development library is not a desktop DAW installer.

Saved tracks may cache older Q-Link maps; reset to defaults or compare a fresh instance. UID and the first 287 parameter declarations are retained, with full sound selection appended as parameter 288. The old 64-choice sound alias keeps its original range. Old projects migrate; new states are not intended for older binaries. Engine changes since1.2.0 mean parameter compatibility is not identical sound recall.

## Build and checks

From repository root:

```sh
python3 mpc-software/scripts/build_plugin.py public/DonutTheory --target all
python3 mpc-software/scripts/test_plugin.py
python3 mpc-software/third_party/mpc-plugin-builder-sdk/tools/package.py mpc-software/plugins/public/DonutTheory
```

Pinned shared SDK: `febd2ca5bd9eb16c4a412837196e145566bc981b`. Native/ARM32/ARM64, SDK validator, host simulator, state/engine/UBSan/analog/bridge and skin checks are required. Actual MPC XL / OS3.10.1.12 screens and runtime hashes are recorded under `design/device-1.9.0/verification.json`.

All sixteen voices are shared. Screen pads audition while held but do not send MIDI to the MPC sequencer; record physical trigger notes instead. Full physical Q-Link coverage, simultaneous finger multi-touch, worst-case CPU, hardware audio and full MPC project round-trip certification remain outstanding. Offline demos are native engine renders, not hardware recordings.

## License and provenance

Project code and original presets are MIT licensed. Custom artwork is maintained with this project. The pinned MPC SDK retains its own license. The ladder, Body and musical tools are original project implementations; no proprietary Moog code or external sample library is bundled. [Engine design and sources](../../../docs/plugins/donut-theory/engine.md) document the design and its limits.

## 1.10.1 action layout

The original warm design now places Sound Preset/Voice/Input, ROLL DICE and chord navigation below the four knob groups. Four70px chord pads span the bottom. Header/donut/knob source art and type family stay intact. All291parameters/66programs and Q-Link assignments remain unchanged. Version1.10.1 is the official update; see the latest handoff for device evidence.
