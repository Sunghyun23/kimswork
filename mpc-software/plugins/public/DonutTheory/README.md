# Donut Theory 1.13.14

A warm Moog-inspired MPC synthesizer for bass, keys, synths, leads and pads, with66factory sounds, two LFOs, synchronized Tremolo/Auto Pan and up to16editable chord slots.

ROLL DICE generates a full sound while individual shared locks preserve the controls you choose. UNLOCK ALL changes protection only. Chords, Key, MIDI/pad routing, master level and release-pitch-drop settings stay intact during Dice.

This version fixes the native PRESET list, continuous spurious host updates and state-snapshot side effects. Original artwork, all Q-Link maps and the kim/Krql identity remain.353append-only parameters preserve the first293declarations.

Install the complete `kim - VST - Donut Theory` folder inside `VST2 Plugins` at the root of an MPC drive. Enable Enhancements and External Plugin Support on MPC OS3.10.1+, keep one active copy and restart/rescan.

Manual: https://kimswork.app/plugins/donut-theory/manual/
Release: https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.13.14

Native/ARM32/ARM64, full engine/bridge/voicing/skin+UBSan and SDKhost19PASS/0WARN/0FAIL. Actual MPC XL3.10.1.12: native selections, Dice, MAIN/CHORD and177installed hashes verified. Physical encoder/multi-touch, hardware audio, full project recall and worst-case CPU are not newly certified. MIT source; retain bundled artwork/font notices.

Build: `python3 mpc-software/scripts/build_plugin.py DonutTheory --target all`
Test: `python3 mpc-software/scripts/test_plugin.py`
Developer decisions and evidence: AGENTS.md, design/device-1.13.14/native-preset-fix/, and mpc-software/docs/plugins/donut-theory/handoff.md.
