# Loop Club0.11.1 detailed audit

Local development patch for MPC Sample1.3.0.12. Public release/site remain0.9.3. Exact checks, final library identity and SD readbacks are in validation-0.11.1.json. Runtime screenshots and USB capture are device evidence; queued inputs do not certify physical pads/fader/SHIFT.

## Reproduced defects and corrections

Each defect was reproduced by a failing native regression before its correction.

| Defect | Correction |
|---|---|
| A Deck clip accepted note/CHOP Overdub, while the opposite direction was refused | Both directions reject mixed control/note Overdub and retain the completed clip; explicit Replace remains available |
| Replacing a Deck clip with MIDI let the automated Deck continue underneath/during/after the new take | Suppress its player at Replace start and clear it on non-Deck commit |
| Canceling an automated Deck take left its sound running after restoring a stopped track | Distinguish explicit manual audition from sequenced playback and follow the restored transport owner |
| GRID followed the next recording target or old clip at the wrong time | During recording follow the active capture type; otherwise follow the saved clip identity |
| Open-hat carried a false baked Swing basis; ride/bell lacked their real basis | Correct generator lane classification; preserve source ticks, restore older timing metadata and retain AS REC behavior |
| Hitting the128grab/4096event limit canceled capture but command completion cleared its error | Propagate Capacity to command/UI, retain the completed clip and permit same-tick coalescing without buffer growth |
| Starting Replace midway through a long Synth/sample MIDI clip left its previous held note sounding | Release that track's sequenced note owner once at capture start, retaining live input owners and drum per-lane replacement |

## Native validation scope

Existing regressions plus new cases cover:

- WAV file/target protection, empty source and invalid controls; source PCM immutability.
- Fader direction, stationary silence, Slip On/Off, bounded analytic seek and tiny/large source positioning.
- Cross-page and cross-track ownership; track pan/filter and master level remain downstream of scratch.
- Replace/Overdub, no-op passes, selective punched regions, capture cancellation and latest-edit Undo/Redo.
- Full event capacity and grab-region overflow without callback allocation/free.
- Synth/sample held playback ownership, Gate/One Shot, Mono/Legato and pre-existing audio/chop behavior.
- Global eighth/sixteenth Swing, bar boundaries/inverse mapping, no duplicate baked Swing and original timestamp preservation.
- Format9 recall, timing metadata repair for formats8/9, original source notes and compatibility defaults.
- Native320×240 layout bounds and existing browser/cursor/FX/navigation regressions.

The actual device pass adds active-DRM screenshots and stereo USB measurements for the complete normal scratch workflow, cross-mode refusal, count-in/MIDI recording display, Replace silence/live notes, Undo/Redo, recording cancellation, malformed WAV refusal and stereo resampling/channel identity. See the validation record for actual completed results rather than interpreting this planned scope as a physical-control claim.

## Reproduce the device pass

Run the normal/isolated ARM64 builds with the pinned SDK. Preserve normal.{so,elf,map} and qa.so under build/qa-device/deep-audit-0.11.1. The isolated build uses --qa --qa-session deep-audio-audit-20261123-r1. Then run tools/qa_turntable_swing.py with the dedicated audio-QA Python environment (NumPy, SoundDevice and Pillow). Set LOOP_CLUB_QA_SESSION to match a fresh compiled session suffix when repeating after a save test. The tool refuses a changed known owner clip/mix state, never saves into the owner's project, stops on failure and restores the normal runtime in finally. Review its actual PNGs before persisting via tools/qa_manage.py --persist.

No firmware flash, private bank replacement, public package/release or site deployment is part of this audit. Maximum16-deck CPU, physical controls, analog input and power-cycle recovery require separate acceptance.
