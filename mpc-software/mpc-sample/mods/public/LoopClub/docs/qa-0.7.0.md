# Loop Club0.7.0 — Audio performance QA

Local development install; public0.1.0 unchanged. Runtime and verified SD direct-boot library SHA-256: `366d8f60cf9c7e35bed5de002d91c630b8f1073af53ab9ff039551cb366cd815`. Normal runtime PID17891 starts stopped on LOOP; original16owner track slots restored and compared after isolated tests. Existing boot preserved before readback-verified publication. No firmware or private expansion changes.

## Implemented

- Explicit Audio REC targets INPUT / CHROMATIC / CHOP, no automatic target changes after recording/import. SOURCE pads5/6/7; loaded Audio SOUND B1 cycles target, B2 opens CHOP EDIT.
- SHIFT+track pad starts; SHIFT+same pad requests bar-end finish; LOOP stays visible. REC remains an alternative. Existing queued handoff/mute/chain behavior is retained.
- Existing WAV import into an empty Audio track (BROWSE or SOURCE→LOAD), preserving the file. Held chromatic/chop performance repeats and releases; recording stores tagged events alongside immutable source PCM. Mixed Overdub keeps chop identity. Source duration is independent of performance duration.
- Root/Tune/Warp sample controls; original waveform in SOUND and source-aware MIDI/PCM preview/SEQ. INPUT commits PCM; sampler commits MIDI; bounded Undo/Redo and format5 recall preserve the source/settings.
- Prevented audition of stale track PCM during an unfinished first INPUT take. Removed overlapping generic SOURCE footer; error/action notices remain visible below target choices. LOOP/MIX Pan/Delay/Room remain unchanged.

## Automated evidence

Full Debug256 and Release256 passed. Subsequent final boundary correction passed15Audio tests and12realtime tests (allocator counts include allocations and frees); final SOURCE notice revision passed10view tests. Strict host/all-targets and ARM64-lib Clippy pass. Normal and isolated ARM64/glibc/symbol builds pass. Canonical pinned SDK is clean; approved SDK patch only applied to ephemeral staging.

## Actual MPC Sample results

All5groups passed via the tested USB QA interface: input capture/WAV picker, chromatic hold/capture/history/actual USB MIDI releases, CHOP capture/mixed Overdub, Warp/audio/timing, and format5save/load. This is remote-controller evidence, not physical button certification. Captures are active DRM320×240 images, not mockups.

The generated220Hz USB input recorded 88200stereo source frames. A held octave-up pad repeated beyond the source's pitched duration at 440.85Hz; after release measured output peak was0.0. With Warp ON and tempo88BPM the same note measured440.15Hz. Four simultaneous Warp voices produced finite stereo output, peak0.14400, with0new deadline overruns over1715callbacks. Cumulative final-run max77.86% and p99 upper30%. Earlier development runs included preparation/scheduling crossings; do not call this worst-case CPU certification.

Inspected actual SOURCE selection, Audio Root/Tune/Warp, original waveform, CHROMATIC capture/grid, CHOP edit/recorded sequence, mixed Overdub, WAV browser and loaded-project screens. SOURCE labels no longer overlap the old footer. Generated tone was the source fixture; no private sample packs were copied into reports or public assets.

## Remaining limits

Actual WAV picker UI and host WAV decode/import/immutable-file/performance tests passed; a supplied WAV's physical-device selection/load was not performed in this pass. LINE/MIC signal quality, physical controls, latest power-cycle and arbitrary/transient-heavy musical Warp quality remain separate acceptance. Four Warp voices is a measured case, not unlimited polyphony/FX headroom. Chromatic/CHOP share16sample voices; GRID still shows first64events. Format5saves are incompatible with older apps; prior saves/boot backups are preserved.

Evidence: `build/qa-device/audio-performance-0.7.0/` contains logs, actual captures, original/restored owner snapshots and final runtime readback (ignored/private). Reproducible opt-in device test: `tools/qa_audio_performance.py --execute`; prepare matching normal.so and isolated qa.so (`--qa-session audio-performance-final-20261123`) first, and obtain session authorization for owner save/restart. The manual contains the user workflow.
