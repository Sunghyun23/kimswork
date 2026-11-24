# Loop Club 0.14.5 — source-loading maintenance

Public preview for MPC Sample firmware1.3.0.12. An SD preload mod, not flashed firmware or an MPC OS plugin.

- Empty Turntable sources initialize through the background file worker. A configured user default WAV takes priority, without replacing audio already stored in a project.
- User content lives outside autorun and remains in place during app upgrades. Invalid configured WAV files report an error; prior source audio remains protected.
- Existing recording alignment, two-stage STOP, chord voice leading, MIDI timing and project-format12 compatibility are retained. The app still reads formats1–11; older versions cannot read new format12 saves.

459 native tests, strict Clippy, ARM64 build and8 launcher checks passed. The exact ZIP library was installed on MPC Sample and its USB output checked: finite audio peak0.26518, no USB status flags.20SD boot files read back,20previous files preserved. Remote queued controls and actual DRM/USB are verified separately from physical fingers, analog latency, power-cycle recovery and maximum/long-session CPU.

Save work and back up autorun and projects before upgrading. Replace the complete autorun directory; preserve LoopClub/Projects, optional banks and configured user files. USB is stereo, not individual stems.32shared Synth/Drum voices and16sampler voices remain bounded. Existing videos and earlier release assets are unchanged. Application source/SDK are MIT; bundled audio has separate provenance/status documented in SCRATCH-SOURCE-NOTICE.txt and assets/scratch/source.json, and is not covered by that MIT grant. An original-rightsholder redistribution grant has not been independently verified.

[Manual](https://kimswork.app/mods/loop-club/manual/)
