# Loop Club0.11.1 — public preview

16-track audio/MIDI looper by kim for MPC Sample firmware1.3.0.12; SD preload mod, not MPC OS VST or firmware flash.

Turntable adds PAD1 Grab, relative fader scratch, Slip On/Off, Speed/Range/Level and source-preserving gesture recording. Replace creates a new scratch loop; Overdub changes grabbed regions only. Project-wide eighth/sixteenth MIDI Swing retains source times and avoids double baked Swing. Audio chords, Synth Poly/Mono/Legato, Glide/Strum, Gate/One Shot and pad octave are included.

The detailed audit reproduces/corrects seven defects involving mixed Overdub, automated scratch/note ownership at Replace/cancel, recording/grid identity, generator timing metadata and capacity feedback.345Debug/345Release tests, strict host/ARM64 Clippy, ARM64 normal/QA/public builds pass. Actual active-DRM/queued-control/stereo-USB checks and exact public-ZIP library smoke are recorded in validation-0.11.1.json and release-0.11.1.json.

Install the complete autorun folder after preserving the old boot tree/projects. No commercial samples, personal banks, firmware or QA agent are included. Original procedural drums are built in; optional user content remains separate. Project format9 reads1–8; older apps reject9. Keep previous backups. timing_revision1 repairs generator metadata without rewriting original note times.

Scratch Pocket is a new actual MPC Sample screen/USB performance with original procedural drums and an original on-device synth resample. Previous full-session and FX videos remain at their existing0.9.3 URLs. No offline replacement synth/audio was used; only captions, scaling and linear output gain are added to the recording.

Physical controls, calibrated analog latency, power-cycle recovery and maximum/long-session CPU acceptance remain incomplete. The detailed audit observed one scheduling-inclusive deadline crossing in24655blocks(max103.26%,p99upper40%).32Synth/Drum and16sampler voices are shared; USB is stereo and GRID shows first64events. Warp is bounded granular processing; imported XPM native inserts/envelopes/Warp are not reproduced.

Source/SDK are MIT with roxmltree notices bundled; optional third-party banks retain their own terms. SHA-256 values and exact asset identities are in release-0.11.1.json and the Release checksums.
