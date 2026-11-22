# Loop Club manual

Loop Club by kim is a 16-track audio/MIDI looper for **MPC Sample firmware 1.3.0.12**. Version 0.1.0 is a public preview. It is a preload mod on SD, not an MPC OS VST or flashed firmware. See [installation](https://kimswork.app/mods/loop-club/public-install.txt).

## Start with the loop

1. Press SAMPLE to open LOOP. Press a pad to **select** its track; this does not start recording.
2. B1 **SOURCE** opens Audio / Synth / Drum. Choose with pads1–3. A nonempty track keeps its source; choose an empty track for a different source.
3. B2 **SOUND** opens the selected source's editor: synth pages, drum layers, or audio WAV selection/CHOP.
4. Press **REC**. The configured zero/one/two-bar count-in precedes capture.
5. Press REC again to establish a whole-bar loop. Recording continues over successive passes. Press REC again to finish capture at the loop boundary while playback continues.
6. B3 switches **REPLACE / OVERDUB**. Replace is the default; changes during capture apply at the next pass boundary. Drum Replace replaces only the lanes you play, keeping untouched lanes.
7. DATA or SHIFT+track pad toggles an existing loop. STOP stops transport and releases notes. Physical SEQ opens GRID.

Each track has its own whole-bar length. A one-bar track repeats four times while another track plays four bars; it is not stretched to that length. Tempo is 40–240 BPM and is frozen while loops exist. Recording memory defaults to16seconds per track; the actual longest whole-bar length depends on tempo. There is no time stretching.

## Source and sound

### Synth

B2 switches NOTES / CHORD without changing the sound or recorded notes. Pads play notes/chords. +/- cycles eight pages; note performance remains available while editing.

| Page | K1 | K2 | K3 | SHIFT controls |
|---|---|---|---|---|
| PLAY | Cutoff | Attack | Release | Wave / Detune / Octave |
| OSC1 | Wave | Tune | Level | K2 Fine |
| OSC2 | Wave | Tune | Level | K2 Fine |
| FILTER | Cutoff | Resonance | LP/BP/HP | K2 Env Amount |
| AMP ADSR | Attack | Decay | Sustain | K3 Release |
| FILTER ADSR | Attack | Decay | Sustain | K3 Release |
| LFO | Rate | Pitch depth | Filter depth | K1 Shape / K3 Volume depth |
| PAN / LFO | Base Pan | Pan depth | Shared LFO rate | Same controls |

Oscillators each offer Sine, Saw, Square and Triangle with independent Level/Tune/Fine. Amp and filter ADSRs are independent. The LFO shapes are Sine, Triangle, Saw Up, Saw Down, Square and Random Hold. Rate is0.05–20Hz; transport sync is not implemented for the synth LFO.

Base Pan is the same value as MIX K1, not a second offset. Pan depth0 leaves it static. Increasing depth moves around that position, bounded at hard left/right. Pan uses one continuously running track LFO with the same Rate/Shape settings as the per-note Pitch/Filter/Volume LFO; their phases are not promised identical. Volume LFO is tremolo; Pan LFO changes stereo balance. Synth B2 CHORD mode changes PLAY controls to Key / Scale / Color; SHIFT provides Octave / Voicing / Release.

### Drums and layers

DRUM PLAY pads audition16sounds: Kick, Snare, Closed Hat, Open Hat, Clap, Rim, Tambourine, Cowbell, Low Tom, High Tom, Conga, Shaker, Ride, Crash, Bell and FX. SHIFT+instrument pad generates a musical one-bar pattern for that part. SHIFT+ERASE+instrument pad removes that generated part. This gesture belongs in DRUM PLAY, not the main track selector.

Generated velocities follow strong / soft / medium / soft subdivision accents with small seeded variation. Triplets have their own grouped accents; snare ghosts stay softer than backbeats. The profile is tied to subdivision position, so a rule that omits a stroke does not move the remaining accents.

MAIN -> SOUND opens DRUM SOUND. Pads select and preview a sound. Each has two independent layers:

- B2 Layer1 / B3 Layer2 selects the layer; SHIFT+B2/B3 enables/disables it. Layer2 starts off.
- K1 Volume / K2 Tune (-24..+24semitones) / K3 relative Shift (-50..+50ms).
- DATA opens bank -> individual WAV selection for that layer, preserving other sounds and the pattern.
- +/- or SHIFT+DATA switches DETAIL: K1 Decay / K2 On-Off / K3 REV.
- The left fader adjusts the overall sound volume, with pickup when changing sounds.

Shift aligns layers relative to the earliest enabled layer. A negative Shift does **not** play before a live pad arrives; the earliest layer is immediate and the other is delayed. REV is sampled-PCM reverse, separate from polarity inversion; procedural drums cannot be reversed. Closed Hat chokes both Open Hat layers on the same track.

Optional `.lcbank` files live in `<card>/LoopClub/Expansions/`. Whole-kit selection retains the pattern and track mix, but resets both layer settings to the new kit defaults. Banks are independent files; no commercial bank is included. Defaults:16seconds per source,64MiB per loaded kit,256MiB across loaded kits. WAV PCM/float mono/stereo is supported. Retained source MIDI in optional banks is not currently imported into the sequencer.

### Audio and CHOP

Choose a WAV on an empty Audio track, or record the input. Imported audio keeps its speed and is padded to a whole bar. On recorded/imported Audio, SOUND opens CHOP. Equal, Transient, BPM and Manual modes support start/end edits, split/merge, level, gate/one-shot and one-step chop undo. The original PCM remains intact. Slice sequencing and recording Undo/export are not implemented. Resample creates an explicit bounce on an empty Audio track rather than feeding a track back into itself. Raw USB-input recording is not supported; the recording timeline uses analog input.

## Mix and FX navigation

In the FX section, the three **upper buttons** remain B1 MIX / B2 TRACK FX / B3 MASTER FX. They are not the three rotary knobs.

MIX shows16track pads and horizontal volume-setting bars. Select a pad, then use the left fader; pickup prevents abrupt jumps. K1 Pan / K2 Delay send / K3 Room send. On a Drum track, press B1 again (or SHIFT+B1) for the16sound DRUM MIX. These bars show gain settings, not live peak meters.

### Direct4×4FXpad boards

B2 TRACK FX and B3 MASTER FX each show a4×4pad grid matching physical pad positions. **Tap an effect to turn it on; tap it again to turn it off.** ON/OFF captions and gold fills show all active effects, not just the selected one. The lower K1–K3 controls edit the selected effect.

TRACK FX has two serial slots. Tapping an inactive effect finds an empty slot automatically; tapping an active effect turns off its own slot. At two active effects, a third is refused with a visible message rather than silently replacing one. Pads9/10 select SLOT1/2 for editing, pad11 switches the default TOGGLE mode to optional momentary HOLD, and pad12 turns track effects off. SHIFT reveals more effect types. Other pads retain Dry/BusA/BusB routing and Setup. HOLD remains an optional performance mode; it is not required to enable an effect.

MASTER FX has five fixed texture effects, independent DUCK and two custom performance effects. All eight can remain on together. Their physical layout is:

| Pad13 | Pad14 | Pad15 | Pad16 |
|---|---|---|---|
| ALL OFF | — | — | — |
| **Pad9: PERF EDIT** | **Pad10: SC SET** | **Pad11: BUS A** | **Pad12: BUS B** |
| **Pad5: DUB ECHO** | **Pad6: DUCK** | **Pad7: FX1** | **Pad8: FX2** |
| **Pad1: DUST** | **Pad2: DRIVE** | **Pad3: SQUEEZE** | **Pad4: WOBBLE** |

Turning DATA selects a master effect for editing without toggling it. FX1/2 show their actual selected names. On first use they start as Filter/Repeat; PERF EDIT opens the existing4×4performance selector to change their types. Their ON/OFF bypass preserves type and parameter values across project save/load. SC SET (or pressing DATA) opens sidechain routing. ALL OFF bypasses the master effects and ducking while retaining settings. The source track is not changed.

### Master texture stages

Pads1–5 independently toggle the following stages. Select one by turning DATA to adjust it without turning it off. Each stage keeps separate saved parameters.

| Stage | Character | K1 | K2 | K3 |
|---|---|---|---|---|
| DUST | Reduced sample detail and softened highs | Grain | Tone | Mix |
| DRIVE | Rounded saturation | Drive | Tone | Mix |
| SQUEEZE | Dense, recovering compression | Amount | Release | Mix |
| WOBBLE | Slow pitch instability | Rate | Depth | Mix |
| DUB ECHO | Tempo-related repeats | Division | Feedback | Mix |

Processing order is Dust -> Drive -> Squeeze -> Wobble -> Dub Echo, after the existing master/performance processing and before final output gain/protection. All stages start off. Mix and bypass transitions fade; effect tails/history remain. This is an original lightweight DSP implementation, not a model certified to reproduce a particular commercial effect or artist's chain. Final protection is a sample peak clamp, not a true-peak limiter.

## Sidechain: let one part move another

Example: a Kick triggers a dip in a sustained synth, while the Kick itself stays clear.

1. Open MASTER FX -> SC SET (pad10), or press DATA.
2. **Turn DATA** to select the source track.
3. Press an instrument pad to choose that drum sound. B2 switches between an individual drum and the whole source track. A non-drum source always uses the whole track.
4. B3 cycles **ALL EXCEPT SOURCE / SELECTED TRACKS / FULL MASTER**. SHIFT+pad toggles a target track in the custom mask. The source track is excluded from the first two modes.
5. K1 DUCK sets depth (0 is bypass). K2 RELEASE sets recovery time. K3 SENSE makes softer triggers count. SHIFT+B2 toggles release tempo sync. The gold bar shows actual gain reduction.

The detector reads the selected sound after its own sound mute/solo/level/pan and before track FX/ducking. Whole-track detection uses the pre-insert track signal; source track mute/solo and its fader affect either detector. It never feeds a processed target back into the detector.

**ALL EXCEPT SOURCE** protects the entire source track, including its other drum sounds. **SELECTED TRACKS** processes only marked tracks, also excluding the source. These modes attenuate after track inserts, before bus/send routing; existing shared effect tails may continue. **FULL MASTER** ducks the entire output including the source, shared buses and returns. It runs after texture processing and before final output gain/protection. These choices are intentionally distinct. There is one sidechain source/configuration per project, not16independent compressors.

DUCK on the master board bypasses sidechain without erasing its depth/source. Free Release spans40ms–2seconds. Sync Release offers1/32,1/16,1/8,dotted1/8,1/4,dotted1/4,1/2,dotted1/2 and1bar at the current BPM. Detection has a short attack; there is no look-ahead, external USB detector input, or per-band sidechain yet.

## USB destinations

MIX -> press DATA -> OUTPUT. Pads1/2/3 or K1 choose MAIN / USB / MAIN+USB for the selected track; B2/B3 change tracks. The default is MAIN+USB. Output settings are saved with the project. Each destination has independent bus/send/master effect state so a USB-only track does not leak into MAIN returns.

**The current device USB configuration exposes one stereo pair. Individual drum multichannel USB output remains unimplemented.** This destination selector does not add USB channels or route stock-app audio. MAIN measurements in the development report are engine telemetry; the physical analog output jack was not independently recorded.

## Save and compatibility

Stop transport and wait for release tails before saving. SHIFT+DATA opens PROJECT; pad8 SAVE / pad9 LOAD. A save preserves a previous-session copy and replaces the new file atomically. On launch, the last project is restored stopped, starting at LOOP. Version3 project metadata embeds both drum layers and names; version1/2 projects remain readable. New Pan depth, sidechain, texture and output fields have safe legacy defaults (Pan/duck off, texture off, output MAIN+USB).

32voices are shared across tracks/layers. Up to4096MIDI events per track; GRID currently displays the first64. CPU headroom varies with polyphony/effects; actual QA recorded isolated scheduling-inclusive peaks above one callback deadline. Save regularly. Physical control operation, calibrated mic/line latency, latest power-cycle boot/recovery and long-duration stability need further acceptance. Do not describe automated remote inputs as physical pad tests.
