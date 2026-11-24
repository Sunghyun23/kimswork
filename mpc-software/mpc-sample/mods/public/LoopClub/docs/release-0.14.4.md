# Loop Club 0.14.4 — aligned recording and smoother chords

Public preview for MPC Sample firmware1.3.0.12. SD preload mod; not flashed firmware or an MPC OS plugin.

- Additional recording waits for the first beat of the longest currently playing loop, without a second preroll. From stopped transport, completed loops wait through count-in and launch together with capture. Independent loop lengths stay unchanged.
- Optional TOP NOTE chord voice leading prioritizes minimal highest-note movement. PLAY SETTINGS → CHORD: SHIFT+B3 cycles VOICE OFF / SMOOTH / TOP NOTE. Existing SMOOTH defaults and recorded MIDI remain intact.
- Recording STOP1 commits the last musical bar while other loops continue; the next STOP stops and rewinds. Empty takes preserve the old clip. MIDI timing stays intact; Audio trims trailing silent bars using a fixed -60dBFS threshold. Very quiet wanted tails below that threshold do not extend the take.
- Drum REPLACE suppresses the old sequence and keeps only new hits. OVERDUB retains prior material. SHIFT+pad records only; explicit DRUM PLAY B2 generates a pattern, SHIFT+B2 undoes it. Later-bar drum events display/edit correctly.
- Empty Turntable sources can load an original procedural AHH. Deep views provide BACK; successful New/Open/Save/Save a Copy return to LOOP. One-bar stopped preroll has four clicks.

455 native tests,39 focused optimized tests, strict Clippy, normal/QA ARM64 builds and8 launcher checks passed. Actual isolated USB MIDI/audio and active-DRM checks verify recording alignment, STOP, original scratch playback, navigation, optional chord voicing and save/restart recall. The exact public ZIP is integrity checked; its library was rebuilt from the verified source. A new unsaved owner project prevented restarting into that exact packaged image, so its separate device smoke remains unperformed. The installed0.14.4 feature image was USB-smoked and persisted with20file readbacks/20priorfiles preserved.

Format12 reads1–11; older versions cannot read newly saved format12 projects. Physical-finger controls, calibrated analog latency, power-cycle recovery and maximum/long-session CPU remain separate. Earlier scheduler-inclusive deadline crossings remain a limit, not unlimited CPU headroom. USB is stereo, not individual stems.32shared Synth/Drum voices and16sampler voices remain bounded.

Save work and back up autorun/projects before installation. Replace the complete autorun directory; keep LoopClub/Projects and optional packs. No personal bank, commercial sample/MIDI, firmware or QA management agent is in this ZIP. Built-in sounds are original. Existing videos and prior release assets are unchanged. MIT source/dependency notices bundled. [Manual](https://kimswork.app/mods/loop-club/manual/).
