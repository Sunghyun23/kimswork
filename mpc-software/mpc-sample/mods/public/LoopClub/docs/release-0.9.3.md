# Loop Club 0.9.3 — public preview

MPC Sample firmware1.3.0.12 SD preload mod, by kim. This is not an MPC OS VST or firmware image. Public0.9.2 is superseded; this update remains preview because physical input, power-cycle/recovery and worst-case performance acceptance are incomplete.

## Changes

0.9.3 moves the contextual FX controls below the pad board in Track, Perform and Texture. Navigation stays above the pads; processing and pad mapping are unchanged. The following features are included from the earlier development milestones.

- Sixteen-track LOOP/MUTE/CHAIN workspace; plain pads perform, SHIFT+track pad records with count-in; same pad/REC finishes at the bar; another shifted track queues a bar-end recording handoff.
- Audio INPUT / CHROMATIC / CHOP recording with immutable source PCM for performance MIDI, correct hold/release, bounded Warp and global latest-edit Undo/Redo.
- Correct complete USB MIDI ownership/releases, source/chop remapping, varispeed tuning and refusal of unsafe INPUT Overdub without an explicit resample.
- Drum XPM file → A–H bank →16pad import, up to four original zones/velocity/cycle/random/mute/gate, portable embedded PCM. Dormant kits survive source-kind changes and project recall.
- SHIFT+LOOP COPY / DELETE / MOVE with empty-target protection, confirmation, full source/sound/kit copy, latest-edit Undo/Redo, and chain/sidechain reference remapping.
- Format7 saves retain the stopped clock and independent loop origins; copies retain the source origin. Old formats1–6 open with legacy zero origins; offsets they never stored cannot be recovered. Older apps cannot read new saves.
- **Immediate FX controls:** press an effect pad; the bottom K1/K2/K3 strip and header change immediately. No DATA focus/setup step is required. Last-touched effect gets edits; occupied unrelated slots are protected. HOLD release bypasses without discarding slot values; SHIFT+press LATCH remains. Advanced modulation Shape/Sync stays on DATA settings.

## Verification and boundaries

293Debug and293Release tests, strict host/all-target and ARM64/lib Clippy, normal/QA/public ARM64 packaging. Actual device testing covers all14track and14performance FX contexts,11master texture contexts, HOLD/LATCH/value retention and full-slot protection. Actual nonzero-clock project recall retains distinct phases and copy alignment. Earlier0.9.0 checks repeated12real-device groups plus Audio/XPM/copy supplements; see qa-0.9.0.md and validation-0.9.0.json.

Actual DRM screens and USB recordings are distinguished from physical input tests.8Warp+16Synth/AutoPan measured p99upper65%, peak93.2832% in one4.5second window; other screen-recording/scheduling-inclusive runs have over-budget blocks. No unlimited-polyphony, longest-session or calibrated analog latency claim is made.32Synth/Drum voices and16sample voices are shared. GRID shows first64events. XPM native inserts/envelopes/Warp, XPN installers and bank MIDI pattern import remain unsupported; USB exposes stereo, not individual drum stems.

The nine-file ZIP includes original procedural drums in the library and MIT/SDK/roxmltree notices. It has no commercial WAV/MIDI, personal bank, project, firmware or QA agent. Existing user expansions/projects live outside autorun and are preserved. Installation backs up old boot files; the package itself does not certify a hardware power-cycle.

## Videos

Actual MPC Sample screen/USB output, with remote control/MIDI input. Song-building is from0.9.0; the FX section is newly captured on0.9.3 and shows inline parameters. The user approved publication. The optional Cosmic Vintage kit heard in the performance is not in the plugin ZIP; its source WAV/program files are not distributed.

See [installation](public-install.txt), [manual](manual.md) and [exact hashes/verification](release-0.9.3.json).
