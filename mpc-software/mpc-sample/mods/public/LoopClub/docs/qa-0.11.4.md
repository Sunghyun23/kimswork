# Loop Club 0.11.4 — Tap Tempo / metronome / count-in

Owner requests four count-in clicks, ordinary TAP TEMPO for tempo and SHIFT+TAP for metronome. Physical code53 was confirmed by an owner press on0.11.3 (physical edge count100→102, press/release). New feature acceptance is separate from code identification.

Implementation: bounded up-to-four-interval estimator, enqueue-time monotonic hardware timestamps, bounce rejection, long-pause reset,40–240BPM clamp with boundary-jitter margin. Modifier tap only toggles the monitor flag and resets the estimator; release does nothing. Tap retains existing protected-tempo refusal, not unsafe source retiming. CLK/ON-OFF shows state and the optional serde-default flag saves with format10; old projects Off. No audio/event reinterpretation.

Count-in has an explicit full start window. One bar has4complete beats, two bars8, Off0. Click sampled before process advances the output frame, then added to monitor after resample/source capture and inserts; it is not printed into PCM/resample. Normal metronome works during running/recording and is silent stopped. Count-in stays audible independently of its switch; normal enabled ticks can continue before a quantized count-in window. Handoff/running overdub does not introduce a second count-in.

Native validation:374-case full Debug suite plus one new waveform-renderer case (375 distinct checks), nine tempo cases and73 focused Release cases (setup/turntable/tempo/realtime/recording/shift), strict host Clippy and ARM64 QA/normal builds pass. WAV-level native tests count full pulses at multiple BPM/start phases and verify recorded zero PCM excludes clicks. Realtime allocation/free guard passes. Local ZIP/build verification is appended to validation-0.11.4.json once complete.

Hardware/SD install and actual USB verification are pending explicit consent to discard the current cleared tracks1/2 and PrerollOff state, which differs from saved baseline. Do not claim0.11.4 installed or actual USB/physical behavior passed until recorded. Current installed0.11.3/public0.11.2 remain. The prepared isolated tool must use a fresh tempo-count-20261124-r1 session and restore the known saved owner baseline only after approval. No owner project file has been overwritten.


## Additional owner requests: SETUP and both fader endpoints

SETUP1/2 uses B1/Pad1 METRO, B2/Pad2 PREROLL, B3/DATA/SAMPLE Back, +/-PROJECT2/2. Separate large On/Off cards preserve project controls and source/recording safety. The QA helper explicitly opens Project when needed; defaults for direct native controller tests remain compatible. Last1/2-bar count-in restores within the UI session. The short METRO caption and compact CLK header avoid clipping. Code-rendered previews are in design/host-0.11.4 and are not hardware screenshots.

Fader min/max are relative to the current grab and selected beat range, not absolute WAV bounds. Verified raw0/127→0/1, both displacement endpoints, wrap, and no jump after released reposition/regrab. Fixed snapshot epsilon wrapping to match the audio reader, a stale CHOP zoom on TURN entry, generic transport cursor duplication, and a margin mismatch between preview waveform and scratch marker. TURN draws directly across310pixels with exactly one source-head cursor; source/chop regions stay unchanged. Physical potentiometer calibration and actual device acceptance remain pending permission/physical access.
