# Loop Club manual

Loop Club by kim is a16-track audio/MIDI looper for **MPC Sample firmware 1.3.0.12**. Public preview0.9.3 provides all16active tracks, LOOP/MUTE/CHAIN, in-place copy/delete/move and playable FX. It is a preload mod on SD, not an MPC OS VST or flashed firmware. See [installation](https://kimswork.app/mods/loop-club/public-install.txt).

## Start with the loop

1. Press SAMPLE to open LOOP. All16tracks are active. DATA selects a track without recording.
2. B1 **SOURCE** selects Audio / Synth / Drum for an empty track. B2 **SOUND** opens its sound settings. Synth B2 selects CHROMATIC / CHORD; that choice is retained when returning to LOOP.
3. In normal LOOP mode, ordinary pads always play the selected track’s drum kit or configured synth notes/chords. **SHIFT+pad** selects that numbered loop track and starts recording with the current count-in. The screen **stays LOOP**. The shifted trigger does not play a note. Audio explicitly captures INPUT PCM or CHROMATIC/CHOP MIDI against its source; loaded chromatic/chop pads repeat while held and stop on release. DATA selects a track while idle. External MIDI follows the active recording track.
4. Press **SHIFT+the recording track’s pad** or **REC** to finish at the next bar boundary. Playback continues and ordinary pads remain performers. Pressing an unshifted instrument pad never starts or ends recording. Before count-in finishes, the same shifted track pad or REC cancels capture.
5. B3 selects REPLACE / OVERDUB. REPLACE records a new whole-bar length. OVERDUB keeps the existing length and untouched tail. Cancelled recordings preserve the previous completed clip.
6. The upper preview shows incoming audio or MIDI/drum activity; the lower4×4board always shows all16loops. SEQ explicitly opens the detailed note grid/waveform.

### LOOP / MUTE / CHAIN

On LOOP, **SHIFT+B1 selects LOOP, SHIFT+B2 selects MUTE, SHIFT+B3 selects CHAIN** directly. SAMPLE returns from other pages to normal LOOP. Finish recording before changing mode. In all three modes, unshifted B1/B2/B3 remain SOURCE / SOUND / REPLACE–OVERDUB.

- **LOOP:** ordinary pads play the selected sound. SHIFT+pad starts/finishes that loop’s recording. SHIFT+another track pad queues a recording handoff at the next bar; DATA selects silently while not recording. Use MUTE and PLAY/STOP for loop playback control; SHIFT+pad no longer toggles playback.
- **MUTE:** pads toggle each track’s mute. Transport and loop phase continue, so unmuting returns in time.
- **CHAIN:** pads append recorded loops to one ordered chain, up to16steps. A loop can appear more than once. DATA or +/− chooses a step; K1 sets its1–16repeats; K2 toggles whole-chain repeat. DATA push starts/stops the chain. ERASE+pad removes the selected matching step (or its first occurrence). Chain start aligns to a bar; switching waits for the current loop and repeats to finish.

Only chain members switch or stop. Independent tracks continue playing. During playback, list/repeat edits apply on the next step; the current cached repeats finish. The status strip shows the active step, remaining repeats and next loop. Configuration is saved with the project, but loading never starts the chain automatically. Recording/clearing an active chain member is blocked; stop the chain first. STOP always stops all tracks.

Each track has its own whole-bar length. A one-bar track repeats four times while another track plays four bars; it is not stretched to that length. New projects default to100BPM. SHIFT+DATA opens PROJECT: K1 selects40–240BPM and DATA moves1BPM. Stop playback first. Stopped MIDI loops follow tempo. Raw Audio loops must have Warp enabled before changing tempo; changing raw-audio INPUT Overdub timing requires an explicit resample first. Recording memory defaults to16seconds per track; the actual longest whole-bar length depends on tempo. Warp is available as bounded granular processing; transient-heavy or extreme stretching may smear.

## Source and sound

### Synth

B2 switches CHROMATIC / CHORD without changing the sound or recorded notes. Pads play notes/chords. +/- cycles seven pages; note performance remains available while editing. Old projects with nonzero volume/pan LFO settings show an additional LEGACY MOD page so those saved settings can be adjusted or zeroed.

| Page | K1 | K2 | K3 | SHIFT controls |
|---|---|---|---|---|
| PLAY | Cutoff | Attack | Release | Wave / Detune / Octave |
| OSC1 | Wave | Tune | Level | K2 Fine |
| OSC2 | Wave | Tune | Level | K2 Fine |
| FILTER | Cutoff | Resonance | LP/BP/HP | K2 Env Amount |
| AMP ADSR | Attack | Decay | Sustain | K3 Release |
| FILTER ADSR | Attack | Decay | Sustain | K3 Release |
| LFO | Rate | Pitch depth | Filter depth | K1 Shape |
| LEGACY MOD (saved legacy values only) | Base Pan | Pan depth | Shared LFO rate | K2 Volume depth |

Oscillators each offer Sine, Saw, Square and Triangle with independent Level/Tune/Fine. Amp and filter ADSRs are independent. The LFO shapes are Sine, Triangle, Saw Up, Saw Down, Square and Random Hold. Free rate is0.05–20Hz. In SOUND LFO (and optional LEGACY MOD), SHIFT+B3 toggles BPM Sync. LFO K1 or PAN K3 then selects1bar,1/2,1/4,1/8,1/16or1/32with dotted(D) and triplet(T) variants on the fractional values. This follows tempo; per-note retrigger and continuous pan phases remain independent. The free Hz value is retained when switching Sync.

For saved legacy Pan modulation, Base Pan is the same value as MIX K1, not a second offset. Pan depth0 leaves it static. Increasing depth moves around that position, bounded at hard left/right. Pan uses one continuously running track LFO with the same Rate/Shape settings as the per-note Pitch/Filter/Volume LFO; their phases are not promised identical. Volume LFO is tremolo; Pan LFO changes stereo balance. Smooth voicing defaults ON. It searches inversions/octaves against the previous chord to reduce voice and top-note movement while retaining pitch classes and voice count. SHIFT+B2 in CHORD toggles Smooth without switching NOTES/CHORD. Key/scale/color/octave/voicing changes reset the comparison history. Synth B2 CHORD mode changes PLAY controls to Key / Scale / Color; SHIFT provides Octave / Voicing / Release.

### Drums and layers

DRUM PLAY pads audition16sounds: Kick, Snare, Closed Hat, Open Hat, Clap, Rim, Tambourine, Cowbell, Low Tom, High Tom, Conga, Shaker, Ride, Crash, Bell and FX. In DRUM PLAY or DRUM SOUND, SHIFT+instrument pad generates a musical one-bar pattern for that part. This writes the loop directly and starts playback; REC is not required. Existing recorded notes in that lane are replaced, while other lanes stay. SHIFT+ERASE+the samepad restores the most recent generation (one-step undo). Capture must finish first. This gesture belongs in the drum context, not LOOP track selection.

Generated velocities follow strong / soft / medium / soft subdivision accents with small seeded variation. Triplets have their own grouped accents; snare ghosts stay softer than backbeats. The profile is tied to subdivision position, so a rule that omits a stroke does not move the remaining accents.

MAIN -> SOUND opens DRUM SOUND. Pads select and preview a sound. Each has two independent layers:

- B2 Layer1 / B3 Layer2 selects the layer; SHIFT+B2/B3 enables/disables it. Layer2 starts off.
- K1 Volume / K2 Tune (-24..+24semitones) / K3 relative Shift (-50..+50ms).
- DATA opens bank -> individual WAV selection for that layer, preserving other sounds and the pattern.
- +/- or SHIFT+DATA switches DETAIL: K1 Decay / K2 On-Off / K3 REV.
- The left fader adjusts the overall sound volume, with pickup when changing sounds.

Shift aligns layers relative to the earliest enabled layer. A negative Shift does **not** play before a live pad arrives; the earliest layer is immediate and the other is delayed. REV is sampled-PCM reverse, separate from polarity inversion; procedural drums cannot be reversed. Closed Hat chokes both Open Hat layers on the same track.

To load a whole kit: select a DRUM track in LOOP, press the physical BROWSE button there, turn DATA to choose a bank, press B2 OPEN / LOAD, then choose a kit and press B2 again. +/- also moves one row; SHIFT+/- moves six. B3 closes the browser. SOUND is the individual layer editor; its DATA/BROWSE loads a single layer rather than a whole kit.

Optional `.lcbank` files live in `<card>/LoopClub/Expansions/`. Whole-kit selection retains the pattern and track mix, but resets both layer settings to the new kit defaults. Banks are independent files; no commercial bank is included. Defaults:16seconds per source,64MiB per loaded kit,256MiB across loaded kits. WAV PCM/float mono/stereo is supported. Retained source MIDI in optional banks is not currently imported into the sequencer.

### Audio and CHOP

Load a WAV into an empty Audio track through BROWSE, or record LINE/MIC/USB. SOURCE explicitly selects INPUT / CHROMATIC / CHOP recording. INPUT records source PCM; the other targets record MIDI timing/velocity and keep the original source unchanged. SOUND shows Root, Tune and Warp, and B2 opens CHOP EDIT. Equal, Transient, BPM and Manual chopping support boundaries, split/delete, level and gate. Recorded slice tags survive switching recording targets and structural edits remap them safely. Hold chromatic/chop performance pads to repeat; release to stop.

Warp keeps musical source duration while Tune changes pitch; Warp off is varispeed. This is bounded waveform-aligned granular playback, not a transient-aware commercial stretch engine. Audio performance capture can Replace/Overdub MIDI while preserving source PCM. To INPUT Overdub a performance or tempo-changed PCM, first explicitly Resample to an empty track. Resample is an audio bounce, not automatic self-overdub.

## Mix and FX navigation

In the FX section, the three **upper buttons** remain B1 MIX / B2 TRACK FX / B3 MASTER FX. They are not the three rotary knobs.

MIX shows16track pads and horizontal volume-setting bars. Select a pad, then use the left fader; pickup prevents abrupt jumps. K1 Pan / K2 Delay send / K3 Room send. On a Drum track, press B1 again (or SHIFT+B1) for the16sound DRUM MIX. These bars show gain settings, not live peak meters.

### FX performance pads

Press an effect pad to select and enable it immediately. Its K1/K2/K3 controls appear directly below the4×4board; no DATA focus or separate settings page is required. The header names the selected effect. K1/K2 edit its two parameters and K3 edits Mix (Duck uses Depth/Release/Sense). The last touched effect receives knob edits; with two effects active the other remains unchanged. After release the effect turns off and its slot settings remain editable without re-enabling it. The board shows effect names and OFF / HOLD / LATCH states. Press normally for immediate HOLD and release to disable it. SHIFT+press toggles LATCH; SHIFT+press again disables it. Releases follow the original effect after navigation. Track inserts have two slots; a third effect shows FULL and never replaces an active effect.

Track pads1–14are Filter, Isolator, Repeat, Reverse, Tape Stop, Dub Echo, Crush, Wobble, Drive, Compressor, LoFi, Tremolo, Auto Pan and Auto Filter. MASTER pads1–8retain Dust, Drive, Squeeze, Wobble, Dub Echo, Duck and two named custom effects; pads9–11offer Tremolo, Auto Pan and Auto Filter using those same two custom slots. When a custom slot contains a dedicated modulation effect, its old duplicate tile is blank/inert. Other positions are inert blanks.

**Tremolo** modulates volume. **Auto Pan** moves the stereo image left/right. **Auto Filter** sweeps a low-pass filter. **Wobble** modulates pitch. These work on Audio, Synth and Drum tracks.

### FX settings

Turn DATA to focus an effect (thin highlight), then push DATA to open FX SETTINGS. Opening/editing settings never turns the effect on. DATA is optional for advanced Shape/Sync settings; use the inline strip for ordinary performance edits. B1 or DATA returns to the board and its editing slot. For modulation effects, K1 controls Rate, K2 Depth and K3 Mix. B2 cycles Sine/Triangle/Saw Up/Saw Down/Square/Random Hold; B3 toggles Sync. With Sync on, K1 or DATA rotation selects straight, dotted(D) and triplet(T) divisions from1bar to1/32T. The previous free rate is retained. Rate follows BPM; transport phase lock is not promised. Other FX show their relevant parameter names. Focus Duck and push DATA for sidechain setup.

Processing order is Dust -> Drive -> Squeeze -> Wobble -> Dub Echo, after the existing master/performance processing and before final output gain/protection. All stages start off. Mix and bypass transitions fade; effect tails/history remain. This is an original lightweight DSP implementation, not a model certified to reproduce a particular commercial effect or artist's chain. Final protection is a sample peak clamp, not a true-peak limiter.

## Sidechain: let one part move another

Example: a Kick triggers a dip in a sustained synth, while the Kick itself stays clear.

1. In the master texture board turn DATA to focus Duck, then push DATA to open sidechain setup.
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

Stop transport and wait for release tails before saving. SHIFT+DATA opens PROJECT; pad8 SAVE / pad9 LOAD. A save preserves a previous-session copy and replaces the new file atomically. On launch, the last project is restored stopped, starting at LOOP. Project format7 retains loop origins and the stopped transport frame, source PCM, imported zones, MIDI, master LATCH/bypass and sound settings. Formats1–6 remain readable; they cannot restore alignment offsets that their version never stored. Older Loop Club versions cannot read new format7 saves. Copy the project/previous-session backup before upgrading. Loading starts stopped. New legacy fields retain safe defaults.

32voices are shared across tracks/layers. Up to4096MIDI events per track; GRID currently displays the first64. CPU headroom varies with polyphony/effects; actual QA recorded isolated scheduling-inclusive peaks above one callback deadline. Save regularly. Physical control operation, calibrated mic/line latency, latest power-cycle boot/recovery and long-duration stability need further acceptance. Do not describe automated remote inputs as physical pad tests.


## Sixteen-track workspace and compatibility

LOOP shows the selected audio/MIDI timeline above a4×4status board. Physical pads1–4are the bottom row and13–16the top row. All16track slots are active, including data saved by the earlier eight-track candidate. DATA wraps1–16in LOOP/MUTE; CHAIN DATA selects arrangement steps. During recording pads play the selected instrument while the LOOP screen stays visible. REC ends capture at a bar boundary; SEQ explicitly opens the full waveform/note editor. MIX has16track cells; DRUM MIX has16sounds. GRID displays the first64MIDI events; recorded event capacity is unchanged.

## Record input

Stop transport, press SHIFT+SAMPLE, then choose PAD1LINE/PAD2MIC/PAD3USB. DATA chooses and B2 or DATA press confirms. B1 returns to PROJECT; B3 returns to LOOP. The selected input displays ACTIVE only after hardware readback. Selection and prior speaker state are saved in the device’s local InputSource.json, separately from project clips.

LINE and MIC switch the validated Sample codec inputs. MIC disables the built-in speaker to prevent feedback; the earlier speaker state is restored on leaving MIC. A worker keeps the speaker off against delayed native startup changes while Loop Club is active. USB records the incoming stereo stream and excludes analog samples. The current hardware/firmware uses128-frameADC and24-frameUSBperiods; arbitrary devices or USBmultichannel input are not claimed. Input selector/USBcapture and MICspeaker switching were verified; calibrated LINE/MICsignal listening and a complete hardware power-cycle were not.


## Transport buttons

PLAY starts all nonempty loops; STOP stops all playback and cancels capture that has not reached completion. SAMPLE REC and SEQ REC currently share the selected-track loop recording command: start capture, or request bar-end completion. They are not yet separate sampling/sequence workflows. SHIFT+− undoes the latest clip edit; SHIFT++ redoes it. See the bounded one-step history below.


## UNDO / REDO — local 0.5.0

Hold **SHIFT and press −** for UNDO. Hold **SHIFT and press +** for REDO. These work from all Loop Club pages, including the browser. Without SHIFT, +/− retain the existing selection/page/edit behavior. This is one shared last-edit history across tracks, not a separate history per track; changing the selected track does not change the edit being restored.

Supported edits are completed Audio/Synth/Drum REPLACE and OVERDUB recordings, per-part drum generation, note-grid edits, track clear and applied/manual chop edits. Audio history restores the original PCM, loop length, waveform features and chop markers; MIDI history restores note timing/velocity and clip length. Repeated passes during one continuous recording are one gesture. Mix, sound, routing and FX settings are not rewound.

During recording, UNDO cancels an unfinished take. If that recording gesture has already completed passes, it also restores the clip from before the gesture; otherwise the prior history remains available. REDO is refused while recording. A new committed edit replaces the old history and clears REDO. A canceled recording or rejected edit does not consume the prior history. Active chain members are protected until chain playback stops.

History is held only in RAM and is cleared on project load/restart. It is not a multi-level project backup. Changing an empty track’s source kind or importing new content invalidates that track’s old history. An accepted BPM change invalidates Audio history to prevent restoring PCM against a different tempo. No available edit shows NOTHING TO UNDO / NOTHING TO REDO.

PCM history uses one preallocated recording-sized buffer for the entire engine, exchanged with capture and track buffers at commit. MIDI history has a fixed 4096-event capacity. UNDO/REDO performs no heap allocation, file I/O or locks in the audio callback. Local automated and device results are recorded separately in validation-0.5.0.json.


## Continuous recording handoff — local 0.6.0

While recording track01, SHIFT+pad02 shows **01 REC > 02 NEXT**. Ordinary pads and external MIDI continue to play track01 until the end of the current bar. At that boundary track01 commits and loops; track02 begins recording immediately, with no extra count-in. The ordinary pads then play track02’s own Synth CHROMATIC/CHORD or Drum configuration. The first recording still uses the configured count-in. During the initial count-in, choosing another shifted track pad retargets that same scheduled start instead of creating an empty loop.

SHIFT+another target replaces the pending next track. SHIFT+the current recording track (or REC) removes the pending target and requests finish only. STOP or recording UNDO cancels any pending handoff. REPLACE creates the newly recorded whole-bar length. OVERDUB retains an existing target’s loop length and untouched tail even if the handoff cuts recording at an earlier bar. At a handoff into an existing loop, its recording phase starts at that boundary. Active chain members cannot be recorded over. MUTE and CHAIN retain their own pad operations; always-play applies to normal LOOP.

History remains one last clip edit. Each track recording in a handoff chain is a separate gesture: canceling the next unfinished take leaves the preceding completed loop intact, with that completion still available to UNDO.


## QA fixes — local0.6.1

Audio REPLACE keeps the completed recording intact until commit, including when canceled. A synth note held through count-in records its onset at the first downbeat; notes released during count-in and early drum hits are excluded. Sound CHROMATIC/CHORD mode now survives project load/restart; older projects default to CHROMATIC. External USB MIDI releases and post-recording handoff ownership are corrected. MIDI input observes the native ALSA sequencer while leaving its events unchanged.

A filled Audio track remains protected against sample import; the browser now explicitly requests an empty Audio track. SHIFT+pad recording instructions appear on LOOP only. Performance home shows16PADS:PLAY. See [detailed QA](qa-0.6.1.md) for real-device results, CPU observations and remaining physical acceptance.


## Audio input, WAV files and recorded sample performance — local 0.7.0

The Audio track keeps one source recording and, optionally, a performance of that source. Its type stays **AUDIO**. Choose a recording target explicitly; it never changes just because a take finished:

| REC target | Captured material | Ordinary pads |
|---|---|---|
| INPUT | LINE, MIC or USB PCM | Audition loaded source chromatically |
| CHROMATIC | Which pitches were held, their timing, velocity and releases | 16 chromatic notes starting at MIDI48 |
| CHOP | Which source slices were held, their timing, velocity and releases | 16 assigned chops |

From LOOP, press **B1 SOURCE**. Choose AUDIO with pad1 if needed, then use **pad5 INPUT / pad6 CHROMATIC / pad7 CHOP**. Press SAMPLE to return to LOOP. On a loaded Audio track, **B2 SOUND** opens sample settings: **B1 REC target**, **B2 CHOP EDIT**, **B3 REPLACE/OVERDUB**. K1 sets the source's Root MIDI note (default48), K2 Tune (−24…+24 semitones), K3 Warp OFF/ON. Volume remains on the pickup fader; LOOP and MIX retain Pan/Delay/Room.

**Record external sound:** choose INPUT; use SHIFT+SAMPLE to select LINE/MIC/USB while stopped. In LOOP hold SHIFT and press the desired track pad, wait for count-in, then perform into the input. **SHIFT+the same track pad** requests the end: capture completes at the bar boundary and loops. A later INPUT REPLACE deliberately replaces the source only on commit; cancel preserves the previous clip.

**Load an existing recording:** select an empty Audio track, press **BROWSE** or **SOURCE → LOAD**, select a WAV with DATA (folders entered with the browser's confirm control), then confirm LOAD. The selected WAV is copied into project storage; the original file is unchanged. Filled Audio tracks are protected: use another empty track, or explicitly clear a clip after preserving it. Loading does not automatically start capture or change REC target. Current import supports WAV, not arbitrary compressed audio formats.

**Record chromatic performance:** choose CHROMATIC, return to LOOP, then SHIFT+the track pad. After count-in, ordinary pads play the source at different pitches. A pad repeats while held and stops on release (with a short click-prevention fade). SHIFT+the same track pad requests bar-end completion. The loop now replays that performance. The original source PCM remains intact. External MIDI can perform and record it too.

**Record chops:** prepare slice boundaries in CHOP EDIT (equal/transient/BPM/manual and individual boundaries), choose REC: CHOP, and return to LOOP. Start/end using the same SHIFT+track-pad gesture. Each ordinary pad holds/repeats its assigned slice; release stops it. Recorded slice identities survive switching REC target. OVERDUB can combine existing chops and new chromatic notes while preserving the source and original performance-loop length. REPLACE creates a new performance length. With no existing sample performance, a first OVERDUB retains the original full-source loop as a root-note event. Use REPLACE to record a performance without that background layer.

The upper LOOP timeline shows waveform for INPUT/raw Audio and notes for captured sample performances. SEQ opens that waveform or note grid explicitly. SOUND always offers the original source waveform. The context strip says **SHIFT+PAD: START / END REC**; ordinary pads do not start recording. REC remains an alternative finish/start control; SHIFT+same-pad is the primary gesture. MUTE/CHAIN keep their specialized pad roles.

**Warp:** OFF uses normal pitched playback (higher notes are faster). ON separates pitch from musical duration and follows current BPM. Root changes the source tuning reference; Tune transposes it. For a raw Audio loop, enable Warp before changing BPM and stop transport first. Other raw Audio tracks must also have Warp ON; sampler MIDI clips follow the clock. Warp uses bounded waveform-aligned grains and can smear transient-heavy sources or extreme pitch/tempo changes. It is not equivalent to a commercial transient-aware stretcher. Chromatic/CHOP auditions share16sample voices; excessive simultaneous holds steal the oldest voice.

**Save and undo:** INPUT commits store PCM; CHROMATIC/CHOP commits store performance events alongside the immutable source. Projects embed both and retain REC target/Root/Tune/Warp/chops. Format5 saves read in0.7.0+, while older formats1–4 remain readable; older apps cannot open the new saves. Keep earlier backups if reverting the app. SHIFT+− / SHIFT++ undo/redo the last completed clip edit; parameters remain outside history. Resample is the separate way to turn a performance plus effects into a new Audio clip.


## Audio safety and playback corrections — local0.7.1

Chop regeneration/split/merge remaps existing recorded slice references; Undo/Redo restores both the region layout and those references. Original PCM remains unchanged. Raw-loop Tune works with Warp OFF (pitch and duration change together); negative Tune can continue across multiple host bars. Unity Warp preserves the loop's padded silence. SOUND remains accessible during CHROMATIC/CHOP capture.

INPUT Overdub is supported for ordinary PCM at its recorded tempo. To add external audio over a recorded CHROMATIC/CHOP performance, or a raw source warped to a new BPM, first Resample that playback to an empty Audio track. The app refuses the ambiguous operation with RESAMPLE BEFORE INPUT OVERDUB instead of losing event data or truncating input. INPUT REPLACE remains the deliberate source-replacement route.

## Drum XPM bank import

On a Drum track, open **SOURCE → pad 5 IMPORT XPM**, or press **SHIFT+BROWSE**. Browse to a `.xpm` Drum Program with its referenced WAV folders on the same SD storage. DATA or plain +/− selects; DATA push/B2 opens; B1 goes back; B3 cancels. Select the program, then choose one populated **A–H bank**. All 16 positions in that bank replace the current track's kit together; empty positions remain silent. Other tracks and the original files are unchanged. Missing, ambiguous, unsupported or oversized data refuses the import and keeps the previous kit. Keygroup programs and `.xpn` expansion installers are not supported; `.xpm` is a program definition, not an expansion archive.

Up to four original sample zones per pad are retained with their velocity ranges, cycle/random mode, mute groups/within-bank mute targets, mono and gate settings. They form the pad's existing **L1**, shown as `[N layers]`; existing L2 remains an optional additional sound. Replacing L1 intentionally replaces its entire imported zone set. Cross-bank mute targets cannot act on absent pads. Playback shares the existing 32 Synth/Drum voices; four simultaneous zones consume four voices, so dense performances can steal old voices.

Sample trim, reverse, tuning, layer/program gain and pan are prepared in the file worker. Native MPC inserts, filters/envelopes, Warp and plugin instruments are not executed or reproduced; this is a sample-program import, not identical MPC program playback. The bank chooser warns about FX/Warp when enabled in the source. Loop Club's own sound controls and FX remain available.

Save the project after importing: project **format 7** embeds all imported WAV zones, so project recall does not require the original XPM/WAV files. Versions 1–6 remain readable; older Loop Club versions cannot open new format7 projects. Limits: 16 seconds per decoded/tuned zone, 64 MiB per kit and 256 MiB total kit PCM across tracks. Only the selected bank's WAV data is decoded. Personal commercial expansions remain personal and are never bundled in public packages.

## Copy, delete and move from LOOP

Stop transport and release played notes/held FX. Press **SHIFT+LOOP** to keep the 4×4 loop overview and open **COPY / DELETE / MOVE** on B1/B2/B3. Pads now select edit locations and do not play notes or arm recording. Press DATA, LOOP or SAMPLE to leave edit mode. REC refuses while EDIT is open. The usual plain-pad performance and SHIFT+pad recording resume on exit.

- **COPY (B1):** press the source track pad, then an empty destination pad. Copy includes audio/MIDI, chops, sampler recording target/Warp, instrument/kit/velocity layers, mixer, routing and FX settings. The original stays intact. Copies do not share mutable sample buffers. Current held FX and effect tails are not copied.
- **DELETE (B2):** press the loop pad, then the same pad again to confirm. Another pad changes the confirmation target. Delete clears the recorded loop/performance, preserving the instrument/kit and sound settings for another take.
- **MOVE (B3):** press the source pad, then an empty destination pad. The loop and its sound settings exchange places with that empty slot. Chain entries and sidechain track references follow the move.

A destination containing a loop, recorded events, source audio or loaded kit is refused rather than overwritten. A configured Synth with no recorded content is an empty destination; its old settings are retained in Undo. Copy/move require stopped transport and no recorder/active notes/held FX. Insufficient kit memory refuses the operation. No existing file is changed until you explicitly save the project.

**SHIFT+− Undo / SHIFT++ Redo** applies to the latest edit only, including COPY/MOVE and confirmed DELETE. Copy/move Undo/Redo also requires stopped transport and released notes/held FX. A later recording/edit replaces this history. History is not stored in projects and disappears after load/restart. Copy/move preparation and memory cleanup run in the worker; realtime Undo/Redo swaps existing buffers without allocating/freeing them.

0.9.0 QA correction: changing an empty imported-kit track to Audio or Synth preserves its dormant kit, as with existing native banks. Project format6 now validates and restores those dormant XPM zones too, so switching SOURCE cannot produce an unloadable save. They only sound when the track is Drum.


## Loop alignment in 0.9.1

Save/load preserves the stopped transport frame and each independent loop origin. A copied loop inherits its source origin, so its phase stays aligned on resume. Old project formats lacked those offsets; the app preserves their legacy zero-origin interpretation. This format7 change does not alter audio generation or the FX shown in the recorded0.9.0 performance video.


## Direct FX controls in 0.9.3

Press any track/performance FX pad and its controls appear immediately below the pad board. The header names the selected effect; K1/K2 edit its parameters and K3 edits Mix. Master texture pads show their matching controls, including Duck depth/recovery/sensitivity. Press/release still means HOLD; SHIFT+press still means LATCH. After release, editing the stored slot values does not switch the effect back on. With two effects, the last pressed pad receives edits; a refused third effect cannot change either occupied insert. DATA remains optional for advanced modulation Shape/Sync. Original artwork/colors and the4×4physical-pad mapping are retained.
