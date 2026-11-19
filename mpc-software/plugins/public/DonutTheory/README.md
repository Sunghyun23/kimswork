# Donut Theory 1.14.2

An original Moog-inspired MPC synthesizer for bass, keys, leads, pads and rich harmony. Shared POLY/MONO editing views never change the actual voice mode; an explicit VOICE control does. Individual OSC wave/tune/fine/level and two delayed LFOs bring detailed control to the front.

- Consistent selector sizing; FREE/SYNC sits above Rate and list arrows have reserved text space.
- 66 factory sounds, 32 chord types, 96 progressions and up to 16 editable chord slots.
- Selected OSC Wave/Pitch(Tune/Fine)/Level/Drift; Filter Cutoff/Resonance/Body/Drive; selected LFO Rate/Depth/Delay/Target. POLY adds Amp/Motion; MONO adds Glide/Decay/Release/Pitch Drop.
- LFO1/LFO2 and Tremolo/Auto Pan each support15 straight/dotted/triplet Sync periods. Small knob turns accumulate; FREE Hz is remembered.
- 61 shared Dice-value locks, sound-only ROLL DICE, family target, UNDO and UNLOCK ALL. Mode, harmony/MIDI/master and both LFO Delays are preserved.
- Six native keys POLY/MONO/OSC/ENV/MOD/FX; CHORD is POLY's second subtab. Full native preset selection and category-first plugin browsing remain available.

Install the complete `kim - VST - Donut Theory` folder inside `VST2 Plugins`, enable Enhancements/External Plugin Support on MPC standalone3.10.1+, then restart/rescan. Use one active copy. The ZIP contains ARM32/ARM64 binaries and required artwork/licenses; the native library is a development test build, not a desktop DAW installer.

Vendor kim, UID Krql; 391 parameters. First 353 declarations/66 factory programs are retained, and the legacy five-step Sync ranges stay compatible. Old projects migrate without tab-induced sound changes; older engines cannot reproduce all new fields.

Build: `python3 mpc-software/scripts/build_plugin.py DonutTheory --target all`
Test: `python3 mpc-software/scripts/test_plugin.py`

See the manual and design/device-1.14.2 verification for automated and actual remote-device checks. Physical encoders/multitouch, hardware-output listening/recording, full MPC project recall and worst-case hardware CPU remain separate and are not newly certified. Source and bundled artwork/font notices must be retained. Groove Dice and Crate Press are separate products.
