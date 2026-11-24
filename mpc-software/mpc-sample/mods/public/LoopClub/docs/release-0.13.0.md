# Loop Club 0.13.0 — public preview

A16-track audio/MIDI looper for MPC Sample firmware1.3.0.12. This is an SD preload mod, not firmware or an MPC OS plugin.

- Turntable PAD1 toggles Grab; PAD2 is a dedicated momentary source CUT. The left fader moves the platter, K3 keeps the mix level. Median-cadence/zero-speed braking reduces endpoint stops and pitch ripple; smoother tracking adds following lag.
- TAP TEMPO, SHIFT+TAP metronome and SETUP metronome/preroll switches. One-bar count-in has four full monitor-only clicks.
- Physical ERASE/COPY entry/back, recorded Drum/CHOP note editing and separate generation/record ownership. All-page/browser input conflicts corrected; immediate controls retain queued fallback.
- Format12 reads1–11 and preserves old scratch trajectories. Keep project backups: older apps reject12. Exact JSON float recall is retained.

405Debug tests,31optimized realtime/motion tests, strict Clippy, ARM64 normal/QA builds and8launcher fixtures pass. Actual Sample active-display/stereo-USB checks and runtime/SD readbacks are recorded in validation-0.13.0.json.32Synth/Drum and16sampler voices are shared limits. Physical controls, analog latency, power-cycle recovery and maximum/long-session CPU are not newly certified. USB remains stereo; GRID shows first64events. Warp/XPM compatibility limits remain in the manual.

The new92.42second105BPM full take shows kit selection, live USB-MIDI drum recording, four-bar mono bass recording and programmed scratch performance in that order. The earlier24.24second scratch-only take is also attached. Both use the owner’s Cosmic Vintage Drums / Stand2gether E105 kit with straight16th hats, plus the Classic Ahhh practice source credited to Beside/Change The Beat via Studio Scratches. Audio and screen are recorded from the MPC. Fader/CUT gestures are programmed against the audio clock, not a human-hand performance or physical-latency test. The commercial bank and raw source samples are not distributed. Existing performance video URLs/assets remain unchanged.

Install the complete autorun folder after saving work and backing up existing boot/projects. No firmware, private banks, raw third-party audio, MIDI, QA agent or QA schedule endpoint is included/enabled in the normal package. Original procedural drums remain built in. Project/data folders are separate from autorun. Source is MIT within the project; dependencies keep bundled notices. Individual product-tag links and SHA-256 checksums accompany this release.
