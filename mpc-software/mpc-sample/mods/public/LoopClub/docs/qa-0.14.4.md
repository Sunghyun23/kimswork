# Loop Club 0.14.4 — loop launch and top-note voice leading

Local installation for MPC Sample1.3.0.12. Public remains0.14.1; no release or website deployment in this pass.

## Behavior

Running additional capture previously added preroll while an existing two-bar loop advanced, producing a mid-loop entry. Arm now uses the longest currently playing nonempty loop's next phase-zero point, without repeating preroll. Equal lengths prefer earliest origin. Loop lengths remain independent; this is a reference-selection implementation default, not a new persistent master or all-length LCM feature.

From stopped transport, completed loops stay stopped at phase0 during count-in and launch together with recording. Cancel before count-in leaves them untouched; retarget preserves the existing launch schedule. Launch initialization also occurs before a first-downbeat MIDI event, avoiding duplicate onset capture. The latest two-stage recording STOP remains intact. WAIT LOOP START is distinct from actual PRE beats, including the lower target cell.

CHORD gains optional VOICE OFF / SMOOTH / TOP NOTE. PLAY SETTINGS→CHORD SHIFT+B3 cycles it; Synth SOUND CHORD SHIFT+B2 is the alternate shortcut. TOP NOTE first minimizes highest-voice movement, then places lower pitch classes near their prior registers with strict ordering. Classic SMOOTH retains its prior cost/behavior; old project defaults are unchanged. New boolean defaults false under existing format12. Existing baked MIDI is untouched. Fixed-size candidate processing is covered by allocation/free tracking.

## Evidence

455 full native tests;39 focused optimized launch/music/realtime tests; strict Clippy; normal/QA ARM64 builds passed. After the final lower WAIT caption change, targeted render/launch/music checks and strict Clippy were rerun. Actual isolated USB MIDI and queued SDK controls verified a two-bar Drum loop followed by a one-bar Synth capture: phase-zero alignment/no second count-in; stopped third capture held existing loops until simultaneous launch. Actual active-DRM screenshots were inspected. TOP NOTE C→G pitches were [48,52,55,59]→[50,53,55,59]; OFF used [48,52,55,59]→[55,59,62,65]. Isolated save/restart recalled two-bar/one-bar clips and TOP NOTE, loading stopped LOOP. Final-render QA separately confirmed WAIT/WAIT LOOP START and TOP recall.

The owner explicitly authorized discarding current unsaved Drum/Synth work for testing. The prior saved Session's16tracks, BPM and input selection were restored without owner Save. Final normal image was separately USB-MIDI/audio smoked: finite peak0.01251764, no USB capture status flags, voices idle afterward. Runtime and SD SHA-256: `bc418975091b154d0be66a259c0480c14c67dc08086b9b0b4d7c3562e8c0595e`;20files read back,20priorfiles preserved. Left on stopped normal LOOP, one-bar preroll/metronomeOff.

A first normal smoke harness looked for audio endpoint MPC Sample instead of its actual MPC Sample Audio name; no stream was opened in that failed attempt. The corrected endpoint smoke above passed. Hardware test controls are injected callback events/USB MIDI, not physical fingers. No new analog, power-cycle or maximum CPU certification. Prior CPU/input/silence-threshold limitations still apply. Raw state/captures stay ignored; do not publish private content. See [validation](validation-0.14.4.json).
