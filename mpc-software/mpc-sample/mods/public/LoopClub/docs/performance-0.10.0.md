# Local0.10.0 performance controls

This candidate is installed on the owner's MPC Sample runtime and SD; the public release and website remain0.9.3. At this historical0.10.0 checkpoint Turntable was a proposal. Current work is described in [local0.11.0](turntable-swing-0.11.0.md).

## Play and record

Audio SOUND B1 explicitly cycles INPUT → CHROMATIC → CHORD → CHOP. Load/record original PCM first. CHORD uses the shared Key/Scale/Color and smooth voicing settings; it triggers pitched copies of the original sample and records expanded MIDI without replacing that PCM. CHOP keeps its own slice identities.

SOUND or DRUM SOUND → SHIFT+B1 opens PLAY SETTINGS. B1 returns; B2 cycles the source's sections. B3 toggles NOTES/CHORD for melodic sources; SHIFT+B3 toggles smooth voicing. Audio chord selection explicitly switches its recording target to chromatic sample MIDI. Stop capture before changing that target or the voice/trigger mode.

| Section | K1 | K2 | K3 |
|---|---|---|---|
| Synth VOICE | Poly / Mono / Legato | Glide0–2000ms | Strum0–80ms per note |
| CHORD | Key | Scale | Triad / seventh / ninth |
| Synth TRIGGER | Gate / One Shot | Shot Gate10–8000ms | Unused |
| Audio TRIGGER | Gate / One Shot | Unused | Unused |
| Drum TRIGGER | KIT / Gate / One Shot | Unused | Unused |

VOICE SHIFT+K3 selects ascending, descending or alternating Strum. CHORD SHIFT+K1/K2/K3 selects chord-register octave, open/close voicing and Strum spacing. Legato preserves envelopes only for overlapping melody notes; Mono retriggers them. The last held melody note has priority and releasing it returns to the previous held owner. Chord groups retain polyphony. Glide applies to Mono melody; Legato glides only across overlaps. Audio remains a polyphonic sampler.

In normal LOOP, hold SHIFT and turn K1 for pad-performance octave−2…+2. This applies to Synth notes/chords and Audio CHROMATIC/CHORD, and the resulting MIDI pitches are recorded. Existing loop playback and external MIDI pitches are not transposed again; held notes still release their original pitch. CHOP numbers and drum roles are unchanged. Normal K1 remains Pan. Existing oscillator and chord-register octave settings remain sound/voicing parameters.

SHIFT+track pad starts recording/count-in and SHIFT+the same pad or REC finishes at the bar. LOOP stays visible. Strum records the real staggered note onsets; Gate release cancels unplayed pending notes. One Shot completes an already-triggered strum after pad release; STOP cancels pending work. Save/recall retains the settings and expanded events.

## Gate and One Shot

- Audio Gate repeats while held and releases with a short fade; One Shot ignores note-off and ends at the sample endpoint. Warp/Tune retain their existing timing behavior. Chord polyphony shares the existing16sampler voices.
- Synth Gate enters ADSR Release on note-off. One Shot ignores that physical release and automatically enters ADSR Release after Shot Gate time; it cannot sustain forever.
- Drum Gate releases both sampled and procedural layers, including cancellation before a shifted layer starts. One Shot plays its natural sound/Decay and ignores physical release. KIT retains existing XPM per-pad gate metadata and original procedural one-shots.
- Loop playback remains looping. These options describe the instrument/sample note trigger, not a new stop mode for entire recorded clips.

## Compatibility and verification

Project format8 stores new settings; formats1–7 load with defaults preserving prior behavior. Earlier apps reject format8 saves. The original owner format7 session was reloaded and its16track contents compared after isolated testing. Own-SD boot files were backed up and20readbacks verified. Physical input, power-cycle/recovery and worst-case CPU acceptance remain separate.

318Debug/318Release tests, strict host/ARM64 Clippy and normal/QA ARM64 builds pass. Actual Sample remote-controller, CoreMIDI and stereo USB checks cover five-note sample chords, hold release, one-shot endpoint silence, Mono fallback, Legato ownership, Strum recording, transposed pitches60/64/67/71, delayed-drum Gate silence and format8 recall. See validation-0.10.0.json and the actual images in design/device-0.10.0.
