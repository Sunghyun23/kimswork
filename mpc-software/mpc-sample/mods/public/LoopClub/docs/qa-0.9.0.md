# Detailed QA and loop editing — local 0.9.0, 2026-11-23

The owner requested another detailed audit and in-place loop copy/delete/move. This is a local runtime/SD update; public0.1.0, the site and other products remain unchanged. See validation-0.9.0.json for the final hashes and exact counts.

## Changes and reproduced defects

SHIFT+LOOP opens COPY / DELETE / MOVE above the existing16pad workspace. Edit pads select locations; ordinary performance and SHIFT+pad recording are restored on leaving. Copy/move refuse occupied slots or active transport/notes/held FX and run preparation/cleanup on the worker. Copy preserves independent PCM, source events, sampler/chop state, XPM zones, sound/kit/mix/FX/routing parameters; move remaps chain and sidechain references. Delete confirms on a second press and retains sound configuration. Latest-edit Undo/Redo swaps existing track/clip buffers without realtime allocation or free; active copy/move Undo requires stopping/releasing first. Current held FX/tails are not copied. Existing owner data is preserved.

Screen inspection exposed misleading individual drum/chord names during whole-loop editing. EDIT now shows AUDIO / SYNTH / DRUM; the original performance labels remain outside EDIT. Actual final DRM captures were inspected for all three actions and the confirmation/refusal states.

A new failing regression reproduced an XPM save/load defect: SOURCE changes retained the kit, but format6 loading rejected its metadata on Audio/Synth. The fix validates all dormant zone PCM/metadata regardless of current source kind, matching the existing native-bank preservation policy. The new test failed before the fix with `Invalid XPM metadata` and passed after. Actual Sample Synth→Audio→Drum switches with save/load passed too. No source sample is dropped to work around this.

The Mac ran out of disk space during testing. Only this product's26GiB regenerable Rust incremental cache was removed; SDK/source/projects/banks/captures were retained. The Cargo helper now defaults incremental compilation off for ephemeral SDK staging, preserving explicit caller overrides.

## Native and real device coverage

Full Debug/Release regressions cover recording Replace/Overdub, count-in/handoff, MIDI/hold ownership, audio Warp/tuning/chop remapping, chain/mute, sound layers, FX/modulation, history, browser/format limits and runtime queues. Added loop-edit tests verify independent copies, empty-target refusal, two-location move Undo, chain/sidechain remapping, later-edit invalidation, edit-pad isolation and no callback allocation/free during full-track Undo/Redo. Strict host/all-target and ARM64/lib Clippy plus normal/isolated ARM64 builds passed. Final counts are in the JSON.

The main isolated actual Sample pass repeated12groups: all16track and pad targets; USB PCM input; chromatic repeat/release/MIDI/history; CHOP and mixed Overdub; Warp; project6 recall; actual generated mono/stereo WAV file traversal/load/resampling/tune and immutable-source SHA comparison; malformed WAV refusal and structural Chop Undo/Redo/recall; page/synth/FX views and hold/latch/two-insert refusal; mixed Warp/Synth/AutoPan rendering; LINE/MIC/USB codec readbacks and20synthetic owned SEQ-route presses; and LOOP copy/move/delete/Undo/Redo/occupied refusal/MUTE/CHAIN.

Supplemental passes repeated XPM file→A/B/H bank selection, pad positions, four velocity zones with actual finite USB audio, missing-sample atomic refusal, original bank-browser retention, portable copies of Audio source+performance events and multi-zone XPM kits through save/load. The final build rechecked SOURCE-kind project6 recall and whole-XPM/audio copy/move/delete/Undo/Redo after the UI/loader fixes. Original owner16tracks plus tempo/input were compared after every restore; the main pass also compared saved global fields. Normal LOOP is left stopped with no recorder. Exact generated fixture data, raw snapshots and captures stay ignored/private under build/qa-device/deep-audio-0.9.0/.

The measured4.5second mixed window with8Warp+16Synth voices and AutoPan had no additional deadline overruns and no near-zero output gaps; observed cumulative maximum was93.2832% of one callback deadline and p99upper65%. This is a bounded test window, not a worst-case/long-duration guarantee. The four-zone USB peaks were0.01722/0.05167/0.08611/0.12056 for the four test velocities.

Physical pad/button/knob presses, calibrated LINE/MIC audio, power-cycle and maximum16Warp+32Synth+allFX are not newly certified. Remote inputs exercise real device UI/audio but do not prove the physical transport hook. MPC native FX/envelopes/Warp import remains unsupported, the MIDI grid's first64event display limit remains, and one volatile latest-edit history remains. No external/commercial payload is published. Own SD publication retains and hashes all20old boot files before readback of the20new ones; no firmware or other mods are changed.

## Reproduction

Use the pinned canonical SDK through tools/run.py and tools/build.py. The device tools are opt-in: qa_complete.py --execute (full isolated matrix), qa_xpm_regression.py --execute (expects the preceding generated sample/project), qa_loop_edit.py --execute (final source-switch and edit regression). They save/restore the owner session and restart only the app through the constrained agent. They require the matching normal/QA build pair and their explicit isolated deep-audio session; do not aim them at owner storage or treat them as a generic destructive reset tool. Use a new isolated session for a fresh full matrix; previously imported kit-only slots are intentionally not erased by clip deletion.
