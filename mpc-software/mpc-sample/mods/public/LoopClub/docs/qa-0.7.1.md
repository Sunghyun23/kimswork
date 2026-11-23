# Detailed QA — local0.7.1

The owner requested a complete fine-grained audit. Public0.1.0 is unchanged. Source fixes are local0.7.1, with270Debug and270Release tests, strict host/ARM64 Clippy and normal/isolated ARM64 builds passed.

## Defects reproduced and corrected

- Chop deletion, splitting and regeneration could leave recorded slice IDs outside the new slice count, making later project load fail. Events now remap by source start, and Chop+MIDI history restores both identities and regions. Parameter-only Chop history stays cheap.
- Selecting an empty sampler target during count-in could cancel the current take before target validation. Targets now validate before retarget/cancel or queue changes.
- Chromatic performance capture could be rejected solely because an imported WAV's bar padding exceeded the PCM take buffer. MIDI capture no longer inherits that irrelevant PCM-length guard.
- Raw Audio loop Tune did nothing with Warp off. Sample-accurate raw-loop playback now supports continuous varispeed, including negative Tune across multiple bars.
- Warp unity could discard/repeat a WAV's padding and alter its source. Raw playback now uses the whole loop and sample-accurate position, preserving the unity source and its silence.
- SOUND was blocked during Audio MIDI capture although16LEVELS could open it. SOUND now opens sample settings for CHROMATIC/CHOP capture; unfinished PCM capture remains protected.
- Unsupported INPUT Overdub of an existing performance or tempo-changed PCM could clear performance data or truncate the new take. It now refuses with RESAMPLE BEFORE INPUT OVERDUB; ordinary PCM Overdub and sampler MIDI Overdub remain supported.

## Actual device matrix

Twelve groups passed: all16track/pad/count-in targets; USB PCM capture; chromatic hold/release/MIDI/Undo; CHOP and mixed Overdub; Warp; format5recall; real mono44.1kHz and stereo22.05kHz WAV selection/load/resampling/tuning; corrupted WAV refusal; Chop reduction/Undo/Redo/save/load; all page contexts and FX hold/latch/capacity; mixed rendering soak; LINE/MIC/USBselection readbacks and20owned SEQ routing presses; extra Synth→Audio MIDI→Drum handoff, invalid count-in target preservation and Undo cancellation of a queued handoff.

The real-device WAV files were generated diagnostic tones in an explicitly isolated QA directory; their bytes matched before/after import. No commercial/private content was published. Read-only browser/fixture diagnostics are opt-in development tools. All15page types plus7synth views were inspected with active DRM captures. The owner16track slots, BPM and global saved FX/mix settings were compared after normal-runtime restoration.

Warp4voices+Synth8voices and Warp8voices+Synth16voices with Auto Pan were finite, bounded and showed no near-zero audio gaps or additional deadline overruns in the measured4.5s windows. The larger measured profile reached90.36% of one callback deadline, p99upper65%; this is not unlimited/worst-case CPU certification. Earlier preparation/scheduling spikes remain documented.

Physical pads/buttons, calibrated analog LINE/MIC signal quality, hardware power-cycle and arbitrary transient-heavy musical Warp listening remain separate acceptance. INPUT Overdub of a performance/tempo-changed source requires Resample to an empty Audio track first. GRID still shows first64events; history remains one volatile clip edit.

Exact hashes, measurements and sanitized group results are in validation-0.7.1.json. Raw captures and owner state are ignored/private under build/qa-device/deep-audio-0.7.1/. User projects, banks and firmware were preserved; SD publication backed up and readback-verified the20existing own boot files.
