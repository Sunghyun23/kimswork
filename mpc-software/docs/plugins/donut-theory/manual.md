# Donut Theory — User Manual

Donut Theory is a synthesis-based instrument by **kim** for Akai MPC standalone. It combines warm basses, worn keys, analog-style leads, textured pads and playable chord progressions in one instrument. Its character draws on soul and hip-hop production; it is not an official artist product or an exact recreation of a particular record, keyboard or tape machine.

This manual describes the current instrument layout. Check the release notes for the installation package's exact version, tested hardware and remaining limitations. The development reference device is **MPC Live III running MPC OS 3.10.1.12**. This does not establish compatibility or performance on every MPC model. Multiple simultaneous screen touches, every physical Q-Link and worst-case CPU use require their own device checks.

## Contents

1. [Install, update and remove](#install-update-and-remove)
2. [First session](#first-session)
3. [Navigation and selection](#navigation-and-selection)
4. [MAIN and Q-Link controls](#main-and-q-link-controls)
5. [Sounds and presets](#sounds-and-presets)
6. [Keyboard, physical pads and screen chords](#keyboard-physical-pads-and-screen-chords)
7. [Chord editor](#chord-editor)
8. [Synthesis reference](#synthesis-reference)
9. [Saving, recording and automation](#saving-recording-and-automation)
10. [Troubleshooting](#troubleshooting)
11. [Manual pad location hints](#manual-pad-location-hints)
12. [Factory sound list](#factory-sound-list)
13. [What changed in 1.2.0](#what-changed-in-120)
14. [Earlier changes in 1.1.2](#earlier-changes-in-112)

## Install, update and remove

### Requirements

- MPC standalone with **MPC OS 3.10.1 or later** and support for external plugins.
- In **Preferences → Enhancements (Beta)**, enable **Enable Enhancements** and **External Plugin Support**. Wording may vary with the MPC OS version.
- A drive accessible to the MPC with a `VST2 Plugins` directory.

The package includes ARM32 and ARM64 MPC binaries. The native development library is a test build, not a desktop instrument installer. This package does not establish support for a desktop DAW.

### Install

1. Download the Donut Theory MPC ZIP from its specific release, then extract it.
2. Locate the complete folder named `kim - VST - Donut Theory`.
3. Copy that folder into `VST2 Plugins` on your MPC-accessible drive. Keep the contents and subfolders together.
4. Restart the MPC application or device so it can discover the plugin.
5. Create an instrument/plugin track and select **Donut Theory** from the external plugins.

The directory should look like this:

```text
VST2 Plugins/
└── kim - VST - Donut Theory/
    ├── plugin.json
    ├── version.xml
    ├── VST/
    │   ├── donut-theory-arm32.so
    │   └── donut-theory-arm64.so
    ├── Plugin Skins/
    └── Content/
```

Avoid an extra nested copy of `kim - VST - Donut Theory` inside itself. Do not copy only the `.so` file: the skin and manifest are required. Normal installation does not require developer mode or SSH.

### Update

Save and back up your MPC project first. Keep the previous package if you need to return to it. **The 1.2.0 engine changes the sound of saved projects even when their controls are unchanged. Render tracks before updating if you need exact old sound.** Replace the existing plugin folder with the new complete folder, then restart MPC. Avoid duplicate installations on different drives.

Donut Theory retains its plugin identity across updates. Existing projects may retain host-side Q-Link assignments from an older instance. If a mapping looks wrong, compare a fresh plugin instance before changing the original track. Preserving the plugin identity does not guarantee that an older MPC OS can load a project saved by a newer OS.

### Remove

Save a copy of any project that uses the plugin, and render or replace its tracks if needed. Remove `kim - VST - Donut Theory` from `VST2 Plugins`, then restart MPC. Projects that reference it will require the plugin to restore those instrument tracks.

## First session

1. Open Donut Theory. The plugin starts on **MAIN**, without a startup tutorial popup.
2. Tap **SOUND PRESET**, choose a category, then choose a sound. Try a Lead or Pad for an analog-style starting point.
3. Set **INPUT** to **NOTES** to play ordinary notes from pads or a keyboard.
4. Adjust **Filter Cutoff**, **Sub** and **Drift**. Use **Attack** and **Release** to change how the sound starts and ends.
5. Press and hold a large chord pad at the bottom of MAIN. Release it to end the gate; the Amp Release and effects determine the remaining tail.
6. To play those chords from physical MPC pads, open **CHORD → PAD SETUP**, use **LEARN** for each slot, and choose **CHORDS** or **SPLIT** input mode.

Start at a comfortable listening level. Sound Trim helps balance the factory bank, but velocity, note range, resonance, overlapping chords and drive can still change perceived loudness.

## Navigation and selection

The lower MPC function-key area provides **MAIN**, **OSC**, **FILTER**, **AMP**, **MOD** and **FX**. The first key has two views: pressing it while MAIN is visible opens the single **CHORD** editor; pressing it again returns to MAIN. Choose the chord bank *inside* the editor rather than repeatedly pressing MAIN to cycle banks.

The gray MPC track/plugin/preset bar belongs to the host. The controls within the illustrated Donut Theory panel belong to the plugin.

### Selection lists

Tap a plugin selection field to open a list in the **left sidebar**, while retaining the current page behind it. Choose an item to apply it and close the list. Longer lists provide page controls. Use **X** or tap outside the sidebar to close it without making another selection. Continuous controls can be dragged; value fields may also provide double-tap editing where shown.

**SOUND PRESET** uses category-first browsing: choose **Init**, **Bass**, **Keys**, **Synth**, **Lead** or **Pad**, then a sound. Use the browser's back control to choose another category. Reopening SOUND PRESET starts from the category list.

The gray MPC **PRESET** selector is different. With the current host/SDK integration it exposes a flat list of 64 programs, with category prefixes such as `Bass / Pocket Anchor`. These prefixes are names, not host folder navigation. Use the plugin's SOUND PRESET control for category-first selection.

## MAIN and Q-Link controls

![MAIN running on MPC Live III](https://kimswork.app/images/donut-theory-main-1.2.2.webp)

MAIN arranges four groups in reading order: A upper left, B upper right, C lower left and D lower right. Each group has four Q-Link controls.

| Group | Control 1 | Control 2 | Control 3 | Control 4 |
|---|---|---|---|---|
| **A · SOUND / PLAY** | OSC Pair | OSC Mix | Sub | Drift |
| **B · AMP ENVELOPE** | Attack | Decay | Sustain | Release |
| **C · FILTER / LFO** | Filter Cutoff | Resonance | LFO Rate | LFO Depth |
| **D · FX / TREMOLO** | Drive | Glide | Tremolo Rate | Tremolo Depth |

With the supplied column mapping, the four physical controls read top to bottom in the same order as each group reads left to right. The host's 16-control positions are A = 13/9/5/1, B = 14/10/6/2, C = 15/11/7/3 and D = 16/12/8/4.

**Q-LINK SCREEN** follows the current plugin view. On CHORD it becomes the chord editor mapping described below. Program/Track mappings retain the synth controls; they do not become chord editing controls simply because the editor is visible.

OSC Mix is immediately beside OSC Pair on MAIN. Glide is in group D and also available on AMP. Space remains on FX. SUB controls an independent sine oscillator exactly one octave below OSC1. It follows note pitch, glide and pitch modulation, shares the voice filter and amp envelope, and does not alter OSC Mix. At 0% it is silent; raise it to reinforce bass.

The filter type, LFO Shape/Target and drive type have touch selectors near their controls. Drive type sits above Drive. Vibrato and Expression remain available as separate touch controls; they do not occupy the main 16 Q-Link positions.

### What to reach for

| Desired change | Control |
|---|---|
| Different oscillator character | OSC Pair |
| Octave-down bass weight | Sub |
| Slowly wandering, tape-like pitch | Drift |
| Static beating between oscillators | Detune on OSC |
| Sliding between notes | Glide and Glide Mode |
| Softer or brighter tone | Filter Cutoff, then Resonance |
| Short pluck or long pad | Amp ADSR |
| Periodic filter movement | LFO Target = Filter, Rate and Depth |
| Volume pulsation independent of filter movement | Tremolo Rate and Depth |
| Saturation or distortion | Drive Type, Drive and Drive Mix |
| More width and echoes | Space; refine Chorus and Delay on FX |

## Sounds and presets

### Factory sounds

Version **1.2.0** uses a **Moog-inspired synthesis engine**, not just revised preset recipes. The default low-pass path combines nonlinear oscillator mixing, a four-stage nonlinear ladder and rounded VCA response. All 63 playable factory sounds have been refitted for its cutoff, resonance and envelope behavior, and output trims are remeasured. No additional mode needs to be enabled. It is an inspired hybrid engine rather than a component-accurate recreation of a named Moog instrument. Existing parameter values load, but existing project sounds change through the new engine. Keep 1.1.2 and render tracks before upgrading when exact old sound is needed.

There are **64 factory sounds**: Init 1, Bass 12, Keys 15, Synth 12, Lead 12 and Pad 12. They use the same synthesis engine rather than acoustic instrument samples. Keys such as tines, bells and reeds describe their synthesized character.

**Init** is a simple starting sound with Low-cut and effects off. The other sounds include individual Low-cut and Sound Trim settings. Low-cut removes unwanted bass; it is separate from the main creative filter. These settings are starting points for a mix, not a guarantee that every arrangement needs the same EQ.

Selecting a sound changes the timbre, envelopes, modulation, effects, Low-cut and Sound Trim, and clears currently sounding notes. It preserves the master Volume, input mode, chord slot count/page, chord settings, chord presets and pad assignments. You can audition different sounds without rebuilding the progression.

The SOUND PRESET arrows browse sounds. They are independent of the **CHORD PRESET** arrows at the bottom.

### Chord progression recipes

There are **32 four-chord recipes**: eight base progressions in four transpositions, at 0, +2, −2 and +5 semitones. They are not 32 distinct sixteen-chord songs.

| Base recipe | Four chords at its base transposition |
|---|---|
| Sunday Soul | Dm9 → G13 → Cmaj9 → Am9 |
| Basement Loop | Fm9 → Bbm9 → Eb9 → Abmaj7 |
| After Hours | Dm9 → Gm9 → A7♭9 → Dm9 |
| Velvet Morning | Ebmaj9 → Cm9 → Fm9 → Bb13 |
| Dusty Sunset | Am9 → Fmaj9 → Dm9 → E7♭9 |
| Warm Windows | Gmaj9 → Em9 → Am9 → D13 |
| Open Sky | Csus2 → Fmaj9 → Asus2 → Gsus2 |
| Soul Turnaround | Cmaj9 → A7 → Dm9 → G9 |

**CHORD PRESET affects only the four slots currently visible on MAIN.** It replaces their Root and Type. It preserves their Voicing, Octave and Pad Note assignments, and leaves the other twelve slots untouched. Each of the four banks remembers its own preset selection.

For example, with 16 slots active and bank **9–12** visible, choosing Sunday Soul changes slots 9–12 only. Slots 1–8 and 13–16 retain their chords. The MOD page's **PRESET 1–4** control is explicitly for the first bank; it does not follow the visible bank.

## Keyboard, physical pads and screen chords

### Input mode versus voice mode

These are separate settings.

| Input mode | Incoming MIDI behavior |
|---|---|
| **NOTES** | Every incoming note plays an ordinary melodic note. |
| **CHORDS** | Mapped notes trigger active chord slots; other notes are ignored. |
| **SPLIT** | Mapped notes trigger chords; other notes play ordinary melody. |

SPLIT is based on assigned notes, not a universal keyboard split point. Chords and melody use the same sound; this is not a multitimbral instrument.

**Mono**, **Legato** and **Poly** govern ordinary melody in both NOTES and SPLIT. Mono/Legato are single-note modes with last-note fallback. Legato carries the envelope through genuinely overlapping notes; separated notes retrigger it. The Glide Mode setting controls whether slides occur always or only with overlapping notes.

Chord expansion remains polyphonic regardless of melodic Voice Mode. In Mono/Legato, one voice is reserved for ordinary melody so a chord does not take over the melodic voice.

### Hold-to-play screen pads

Press a MAIN chord pad to play it and release to end its gate. This works in every Input mode. Release is musical: a long Amp Release, chorus or delay can continue after your finger leaves the screen. It is not a latching switch.

Changing the visible bank releases screen-held chords that leave the page. It does not release physical MIDI chords merely because their bank is no longer visible.

Screen pads audition the internal synth directly. **They do not send MIDI notes into the MPC sequencer.** To record a performance as MIDI, use physical pads or a MIDI keyboard with the appropriate input routing.

### Connect physical pads with LEARN

The plugin receives MIDI note numbers, not labels such as A1. MPC Pad Perform scales, octave settings and note layouts can change the notes sent by the same physical pad. You do not need to alter MPC's layout to match Donut Theory.

1. Press the first function key from MAIN to open CHORD.
2. Select the desired four-slot bank. Add slots first if needed.
3. Tap **LEARN** for the target slot.
4. Press the physical pad or keyboard key you want to use.
5. The first note is consumed silently for registration. Confirm the displayed MIDI number, completion status and LEARN disengagement.
6. Press the same pad again to play its chord. Use CHORDS or SPLIT input mode. Learning while in NOTES changes the mode to SPLIT.

Use **CANCEL LEARN** to exit without assigning a note. Learning an already assigned note swaps slot assignments to retain unique mappings. A slot's MIDI Note display also supports double-tap manual assignment. MIDI Note is intentionally outside the chord Q-Link controls.

The first four default mappings are **24, 26, 28 and 29**. The complete sixteen-slot defaults are 24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48 and 50. Only active slots respond. Use MIDI numbers as the reference: software can label the same number with different octave names.

In **CHORD → ADVANCED**, **Individual** pad mapping uses each slot's assignment and is the recommended mode. **Consecutive / Trigger Base** retains older projects' consecutive-note routing. Ensure its range does not exceed MIDI note 127 when using many slots.

### Sustain and controllers

- **CC64** sustains MIDI notes/chords until the pedal is released.
- **CC1** adds pitch vibrato.
- Pitch bend follows the **Bend Range** setting.
- MIDI All Notes Off / All Sound Off and controller reset are handled by the engine.
- **CHORD → ADVANCED → ALL NOTES OFF** clears screen, keyboard and physical-pad notes, sustain and effect tails. Use it to recover from an interrupted input stream.

## Chord editor

### Key reference and circle of fifths — 1.3.0

The CHORD editor has a KEY selector and MAJOR/MINOR switch beside a circle of fifths. Selecting a key updates the reference diagram and seven diatonic chords; it does not change any stored chord or keyboard note. MINOR uses the natural minor scale. The outer ring shows major keys and the inner ring their relative minors. The highlighted sector identifies the selected key pair. This diagram is a reference, not a chord trigger. Enharmonic spellings are chosen for readability.

The small degree under each play pad identifies its root in the selected scale. Capital/lowercase follows the stored chord's major/minor quality; a dash means its root is outside the selected scale. It is a root-degree aid, not a full harmonic-function analysis or chord-quality validator. Borrowed chords remain editable.

**TRANSPOSE** opens a confirmation drawer. Set FROM to the original key of your progression; TO KEY is the selected reference key. The default FROM is C for older projects, and remembers the destination after a successful apply. The drawer shows the signed semitone movement and active slot count. APPLY moves the root of every active slot (4, 8, 12 or 16), including banks not currently visible, by the same nearest interval (−5 to +6 semitones; a tritone goes up). CANCEL and outside-touch dismissal make no note changes. Reapplying to the same key gives zero movement. Changing Major to Minor alone does not reharmonize the progression.

Chord type, voicing, per-slot octave, MIDI Note/LEARN, input mode, sound and inactive slots are preserved. If any root would exceed MIDI24–84, the entire operation is rejected; adjust the source key or chord register before trying again. A held screen chord follows the edited root; a physical MIDI chord uses the new root on its next trigger, with its existing release ownership preserved.

**PAD SETUP** holds the MIDI Note readouts/manual assignment and LEARN for the four slots in the selected bank. Close it to choose a different bank. The sixteen CHORD SCREEN Q-Links remain Root / Type / Voicing / Octave; Program/Track retains the synth mapping. All new drawers suppress background Q-Link guides.


![CHORD editor running on MPC Live III](https://kimswork.app/images/donut-theory-chords-1.2.2.webp)

### Four to sixteen slots

New instances start with **four active slots**. The editor's **+ / −** changes the active count in groups of four: 4, 8, 12 or 16. Reducing the count retains the inactive slots' settings for later reactivation. Changing the active count clears sounding notes.

Use the in-screen **1–4 / 5–8 / 9–12 / 13–16** bank buttons to edit the available banks. MAIN shares this selection and always shows four large pads. Its page arrows change banks; its CHORD PRESET arrows change the recipe for the visible bank. Version 1.1.2 gives both arrow pairs 44×44 touch targets while retaining the four large chord pads. Physical note mappings continue to address their slots independently of the visible bank.

### Q-Link editing

In **Q-LINK SCREEN**, groups A–D represent the four visible slots. Each group has:

| Q-Link | Setting |
|---|---|
| 1 | Root |
| 2 | Type |
| 3 | Voicing |
| 4 | Octave |

For bank 5–8, A edits slot 5, B slot 6, C slot 7 and D slot 8. MIDI Note stays a separate assignment field. Octave is available as a donut control and a value selection list.

### Expanded harmony, chord glide and strum — 1.5.0

The current development candidate offers32chord types and96four-chord progression presets. The original11types/32recipes stay in their original positions;64additional recipes are grouped by name into Neo, Pocket, Vamp, RnB, Modal, Gospel, Color and Turn. Use CHORD PRESET on MAIN to fill the visible four-slot bank. Each of four banks retains its own selection. The library includes minor11/13, major6/9, major9#11/13, suspended dominants, altered dominants, half-diminished, diminished7, augmented7 and minor-major7 colors. See [the complete harmony library](#harmony-library) for names and formulas.

**CHORD GLIDE** in the editor enables newest-held chord connection independently for each MIDI channel. It shares the existing GLIDE time control on MAIN/AMP and the CHORD touch field. Overlap two triggers: exact common MIDI tones keep their existing voices/envelopes, and remaining tones use a minimum-distance assignment. Releasing the newest trigger returns to an older still-held chord; releasing the last ends the chord. MIDI channel1 and screen chords share a chain. Separate channels remain separate. Sequential, non-overlapping presses snap rather than sliding from release tails. Global Mono/Legato still governs ordinary melody notes separately. A zero GLIDE time gives immediate pitch changes. The underlying time control is an exponential time constant, not a guaranteed end-to-end slide duration.

**STRUM** is the existing Chord Spread, now directly editable beside GLIDE. It plays each new chord once, bass-to-top, with0–40ms between tones. For example8ms gives a five-tone chord a32ms first-to-last spread. It does not repeat automatically. With chord glide enabled, common tones continue; moved pitches start their transitions in that same ascending sequence. New extra voices use the same onset spacing. Early note-off/close/panic cancels pending starts and pitch transitions. These touch fields share existing parameters; the sixteen CHORD SCREEN Q-Links remain Root/Type/Voicing/Octave, with the Type controls now spanning all32choices.

Legacy parameter declarations and ranges remain unchanged; full-range controls append separately. Old project values migrate by their saved plain choices. The initial roughly half-second load replay guard prevents old MPC normalized aliases from replacing new full-range choices. Legacy automation continues after that replay window. An older project with a stored Q-Link map may need its SCREEN assignments reset or a fresh instance to use the new full-range Type relays. Shared sixteen-voice and hardware-validation limits still apply.

### Smooth voicing — 1.4.0

Tap **SMOOTH VOICING** above TRANSPOSE in CHORD. Choose **CURRENT 4** for the bank you are editing, or **ALL SLOTS** for all active slots. The panel shows each selected slot's top note before and after the proposal, plus the total semitone movement around the entire loop, including its last-to-first connection. Notes use the MPC octave naming convention.

The first selected chord's current voicing and octave stay fixed as the anchor. Other chords are chosen from their supported inversions/Open voicing and −2..+2 octave range. Top-note travel is minimized first; ties favor small closest-tone/bass movement, then fewer changes from the current voicings. New spans are limited to two octaves, or the original span if it was already wider. Every chord tone must remain in MIDI0–127 without clamping or collapsed notes. If the current chord register is unsafe, adjust it before trying again.

**PREVIEW** auditions the proposed progression once, one chord per beat at the available host tempo (90BPM if unavailable), without writing its voicings into the stored slots. Tap PREVIEW again to stop, or close the panel with CANCEL/X/outside touch. Normal envelope/FX tails may continue after gate release. Audition shares the existing sixteen-voice instrument; dense simultaneous playing can still steal voices. It does not send or record MIDI into the MPC sequencer.

**APPLY** changes only the selected slots' Voicing and Octave. Root, Type, MIDI/LEARN, input mode, Key and all unselected slots stay intact. CANCEL closes without applying. Editing a source chord or changing the scope invalidates a previous proposal and stops its audition; the control thread prepares a fresh proposal before it can be applied. Preview and Apply are transient commands and are not replayed on project load.

For the default Dm9 / G13 / Cmaj9 / Am9 progression, one supported solution keeps E as the top note of all four chords, reducing loop travel from42 to0semitones. This is optimization within the existing voicing/octave choices, not arbitrary per-note editing or automatic reharmonization. The sixteen CHORD SCREEN Q-Links remain Root / Type / Voicing / Octave.

### Root, type, voicing and octave

**Root** sets the chord's root note and register, over MIDI notes 24–84. **Octave** shifts the whole resulting voicing by −2 to +2 octaves without changing the trigger assignment. This lets you keep a comfortable pad layout while placing chords in a useful register.

| Type | Intervals above the root, in semitones | Tones |
|---|---|---|
| Minor 7 | 0, 3, 7, 10 | 4 |
| Major 7 | 0, 4, 7, 11 | 4 |
| Minor 9 | 0, 3, 7, 10, 14 | 5 |
| Major 9 | 0, 4, 7, 11, 14 | 5 |
| Dominant 7 | 0, 4, 7, 10 | 4 |
| Dominant 9 | 0, 4, 7, 10, 14 | 5 |
| Dominant 13 | 0, 4, 10, 14, 21 | 5 |
| Sus 2 | 0, 2, 7 | 3 |
| Minor | 0, 3, 7 | 3 |
| Major | 0, 4, 7 | 3 |
| 7 flat 9 | 0, 4, 7, 10, 13 | 5 |

Dominant 13 intentionally omits the fifth. It is a five-note voicing rather than a seven-note stack.

**Voicing** offers Root, available inversions and Open. Triads provide 1st and 2nd inversions; four-note chords add 3rd; five-note chords add 4th. An inversion moves a chord member into the bass. Open raises alternating members by an octave. Changing to a shorter chord clamps an unavailable inversion to its last valid position.

**Spread**, on the detailed controls, introduces 0–40 ms between chord tones, from low to high. It is a strum-like onset spread, not stereo width.

Editing a screen-held chord refreshes its voicing. A chord already held through MIDI retains its current notes until the next trigger. This avoids changing a recorded or pedal-held MIDI chord halfway through its gate.

### Slots are not voices

The engine has **16 audio voices**, not sixteen complete chords of unlimited size. A chord consumes three to five voices. Several overlapping chords, long releases or sustained notes can exhaust the pool and cause voice stealing. In Mono/Legato, the melody reservation leaves fifteen voices for chords. Reduce overlap or release time if earlier chords disappear.

## Synthesis reference

### OSC

| Control | Function |
|---|---|
| Oscillator 1 / 2 Wave | Sine, Triangle, Saw or Pulse. |
| OSC Pair | Selects one of sixteen two-waveform combinations; does not replace Mix, Detune, filter or envelope settings. |
| Osc Mix | Balance of the two oscillators, 0–100%. |
| Detune | Static oscillator-2 offset, −25 to +25 cents. |
| Oscillator 2 Octave | −2 to +2 octaves. |
| Pulse Width | Pulse duty cycle, 5–95%. |
| Sub | Sine sub-oscillator level, 0–100%, exactly one octave below OSC1. Also available on MAIN group A / Q-Link 3. |
| Noise | Noise amount, 0–30%. |
| FM | Phase-modulation character on **Sine oscillator 1**, 0–100%; not a general multi-operator FM matrix. |
| Tone | Harmonic/saturation character macro; separate from the stored base cutoff. |
| Drift | Shared slow wow and faster flutter-like pitch movement. |

**Drift is not Detune.** Detune creates a steady offset between oscillators. Drift moves pitch over time for worn-tape instability, with a fine low-end taper for subtle settings. Vibrato is another distinct effect: regular periodic pitch movement. Motion, on MOD, adds its own slow movement and chorus contribution.

### FILTER

**Low-pass** uses a four-stage nonlinear ladder with a 24 dB/octave asymptotic slope. Its input and resonant stages respond to level; higher resonance naturally reduces low-frequency gain. The loop gain is limited below autonomous self-oscillation. The displayed cutoff sets the pole frequency: at zero resonance the four-stage response is approximately −12 dB there, rather than a calibrated −3 dB point. The ladder runs at 2× internal rate with output filtering to reduce nonlinear aliasing.

**Band-pass, High-pass and Notch** retain the existing 12 dB/octave SVF implementation as extensions. They are not ladder variants. The oscillator summer and VCA also add gentle nonlinear coloration with external Drive off. Oscillator wave generation remains band-limited PolyBLEP/wavetable synthesis; phase continues between retriggers. This is not a complete analog oscillator circuit simulation.

| Control | Range / purpose |
|---|---|
| Filter Cutoff | 20 Hz–18 kHz; center frequency in modes where appropriate. |
| Resonance | 0–100%; emphasizes the cutoff region. |
| Filter Env | −100 to +100%; negative closes, positive opens relative to the base setting. |
| Filter Attack / Decay / Release | 1–5000 ms each. |
| Filter Sustain | 0–100%. |
| Key Track | 0–100%; higher notes raise the filter's operating frequency. |

Filter ADSR shapes brightness and is separate from Amp ADSR. Velocity-dependent Expression can also affect brightness. The **Low-cut** on FX is a final bass-cleanup filter; it does not replace this filter or follow its LFO/envelope.

### AMP and play settings

| Control | Range / purpose |
|---|---|
| Amp Attack | 1–5000 ms from note-on to full level. |
| Amp Decay | 1–5000 ms from peak to sustain. |
| Amp Sustain | 0–100% level while held. |
| Amp Release | 1–5000 ms after the gate ends. |
| Volume | Master level, −60 to 0 dB; retained across sound selection. |
| Expression | 0–100% velocity response in level and brightness. |
| Voice Mode | Mono, Legato or Poly for ordinary melody. |
| Glide | 0–500 ms; zero disables the slide. |
| Glide Mode | Always or Legato Only. |
| Bend Range | 0–24 semitones. |
| Input Mode | Notes, Chords or Split. |
| Trigger Base | Legacy consecutive chord-trigger range. |
| Spread | 0–40 ms between chord tones. |
| Chord Slot | Selected/edited slot indication; not a MIDI trigger assignment. |

A short release still has a short envelope tail. A pad's long release can sound sustained after a screen pad is released; lower Release to distinguish this from a stuck gate.

### MOD: LFO, vibrato and movement

The main LFO offers **Sine, Triangle, Square and Sample & Hold**, with target **Pitch, Filter, Pulse Width or Amp**. Shape and Target are touch choices on MAIN. Sync and Division are on MOD. There is no separate duplicate LFO Settings page.

- **Rate:** 0.05–20 Hz when Sync is off.
- **Depth:** 0–100%.
- **Sync:** follows MPC tempo when enabled.
- **Division:** 1/16, 1/8, 1/4, 1/2 or 1 bar when synchronized.

With Sync on, Division and host tempo determine the effective speed. The free-running Hz value is retained, but turning that value does not set the synchronized speed. Change Division for a different rhythmic rate.

**Vibrato** has a dedicated 5.2 Hz pitch oscillator, with depth control and additional CC1 response. **Motion** combines slow pitch movement with chorus contribution. These can coexist with Drift and the main LFO; moderate their combined amount for stable tuning.

MOD's **PRESET 1–4** selects the first bank's chord recipe only, even while another bank is visible on MAIN.

### FX: saturation, space and cleanup

| Control | Function |
|---|---|
| Drive Type | Tape, Tube or Fuzz waveshaping character. These are not exact circuit models. |
| Drive | Distortion intensity, 0–100%. |
| Drive Mix | Dry/wet balance, 0–100%. |
| Chorus | Chorus amount, 0–100%. |
| Delay | Delay mix, 0–100%. |
| Feedback | Echo regeneration, 0–65%. |
| Delay Division | Tempo-related 1/8, 1/4, dotted 1/4 or 1/2. |
| Space | Combined chorus/width and delay-wet macro, 0–100%. |
| Tremolo Rate | Independent volume modulation, 0.1–15 Hz. |
| Tremolo Depth | 0–100%; zero disables the pulsation. |
| Low-cut | Final low-frequency cleanup, Off to 250 Hz. |
| Sound Trim | Per-sound output correction, −18 to +18 dB. |

Tremolo is independent of the main LFO, so filter movement and volume pulsation can run together. **Tremolo is free-running in Hz; it has no tempo-sync division.** It follows Drive and precedes chorus/delay, so echoes may continue between its pulses.

Low-cut acts on the stereo output after chorus/delay. Typical factory starting ranges are 18–28 Hz for Bass, 85–160 Hz for Keys, 85–140 Hz for Synth, 100–160 Hz for Lead and 120–200 Hz for Pad. Adjust to the arrangement. Raising Low-cut too far can remove a bass patch's fundamental.

Sound Trim balances a patch's output; Volume is your overall listening/track level. The factory trim measurements use a standardized performance and cannot predict every combination of notes and effects.

#### FX recipes

The FX selector offers **Custom**, **Dry**, **Warm Tape**, **Wide Chorus**, **Slap Delay**, **Space Echo**, **Fuzz Box** and **Dark Dub**. A recipe sets Drive Type, Drive, Drive Mix, Chorus, Delay, Feedback, Delay Division and Space. Dry also zeros Motion, Tremolo Depth and Drift.

Custom leaves the existing controls unchanged. After editing a recipe's controls, its displayed name describes the last selection, not a verified exact match. To reapply the same recipe, select Custom and then select the recipe again. Loading another sound loads that sound's effects settings.

## Saving, recording and automation

Save the MPC project to retain the instrument's settings. Chord content, voicings, octave shifts, pad assignments, active count, selected bank and bank-specific preset choices are part of plugin state. Screen-held notes and pending commands are temporary, not saved performance data.

Saving ends active **screen audition** gates on the next audio processing block. It does not discard held physical MIDI ownership. Reloading a project never intentionally starts a chord just because it was being touched during the save.

Use physical pads or MIDI keys to record chord-trigger notes into a sequence. The plugin expands those triggers internally; it does not output the expanded chord as recorded MIDI. Keep the corresponding slot and input settings with the project so playback expands the same triggers.

For automation, use the actual synth or per-slot chord parameters exposed by MPC. The sixteen bank-following CHORD Q-Link editing controls are non-automatable editing proxies: recording them as though they permanently represented slots 1–4 would be ambiguous after changing banks. Each actual slot retains its own automation target.

Selecting a different sound or changing input/voice routing clears current notes. Changing the visible chord bank releases screen audition gates but preserves physical MIDI-held chords. When changing several settings during a performance, release notes first if you want a predictable transition.

## Troubleshooting

| Symptom | Check |
|---|---|
| Plugin does not appear | OS requirement, Enhancements and External Plugin Support; complete package under `VST2 Plugins`; no extra nested folder; restart MPC. |
| Background or knobs are missing | Replace the complete package, including `Plugin Skins`; a binary-only installation is incomplete. |
| Presets appear in one long list | The gray host PRESET list is flat. Use the illustrated panel's SOUND PRESET for category-first browsing. |
| Physical pads play single notes | Select CHORDS/SPLIT and LEARN each desired pad; NOTES intentionally bypasses chord expansion. |
| A physical pad is silent | Confirm track/input routing, active slot count and the actual learned MIDI number; in CHORDS unmapped notes are ignored. |
| First learned pad press is silent | Expected: the first event registers its note. Press again to play. |
| Screen chords do not record into the sequence | Screen pads are internal audition gates. Record physical-pad or keyboard triggers. |
| Sound continues after releasing a pad | Check Amp Release, delay and CC64 sustain. Use ALL NOTES OFF if an input was interrupted. |
| Legato still produces a full chord | Legato governs ordinary melody; chord triggers intentionally remain polyphonic. |
| Some held chords disappear | The sixteen-voice limit is shared by notes and chord tones. Reduce overlap, sustain or release. |
| LFO Rate seems ineffective | With Sync on, change Division; free Hz is retained but does not set synchronized speed. |
| Chords change in the wrong bank | MAIN CHORD PRESET follows its visible bank; MOD PRESET 1–4 always addresses the first bank. |
| Q-Link still changes synth controls in CHORD | Select Q-LINK SCREEN. Program/Track retain the synth map. |
| An old project has an old Q-Link layout | Preserve the project and compare a fresh instance; host-side assignments can be stored independently of the current skin. |
| Pitch sounds too unstable | Reduce Drift, Motion, Vibrato/CC1 and Pitch-target LFO depth; Detune is a separate static offset. |
| Sound is thin after editing | Check Low-cut, main filter mode/cutoff, OSC Mix and Sub. |

When reporting an issue, include the package version, MPC model/OS, input mode, voice mode, sound name, active chord bank, Q-Link mode and whether it occurs in a new instance or an older project. A screenshot and a short reproducible sequence are more useful than an isolated symptom.

## Manual pad location hints

The manual bank/cell location hint feature introduced in 1.1.2 was removed in **1.2.2**. It is no longer displayed on MAIN or CHORD. This retires only a visual reminder, not physical-pad support. Use **MIDI Note / LEARN** to connect each physical pad as described above. Existing note assignments and all sixteen chord slots are retained. The old location parameter values remain stored solely for state compatibility and no longer drive any visible controls.

## Factory sound list

| Category | Sounds |
|---|---|
| **Init · 1** | Init |
| **Bass · 12** | Pocket Anchor; Maple Thump; Brick Bounce; Molasses; Crate Knock; Soft Cone; Motor Hum; Felt Punch; Honey Slide; Walnut Pluck; Low Canvas; Dust Motor |
| **Keys · 15** | Felt Circuit; Worn Tines; Honey Reeds; Kitchen Bell; Soft Hammer; Velour Clav; Milk Glass; Tape Celeste; Reed Cabinet; Bronze Mallet; Evening Felt; Pocket Organ; Pearl Hammer; Cocoa Tines; Dust Harp |
| **Synth · 12** | Chopped Brass; Mellow Block; Velvet Stab; Amber Stack; Rubber Chord; Hollow Print; Record Bloom; Copper Pluck; Faded Poly; Small Ensemble; Syncopated Ink; Glass Chord |
| **Lead · 12** | Ribbon Whistle; Bent Wire; Narrow Alley; Honey Horn; Syrup Line; Ink Flute; Frayed Saw; Pocket Squeak; Golden Thread; Melted Neon; Muted Trumpet; Round Needle |
| **Pad · 12** | Draped Velvet; Paper Halo; Warm Projection; Woven Air; Satin Room; Faded Choir; Ember Strings; Curtain Light; Soft Horizon; Opaque Silk; Dusk Film; Golden Haze |

For exact release compatibility and completed validation, consult the download's release notes. The repository also maintains [preset design notes](https://github.com/Sunghyun23/kimswork/blob/main/mpc-software/docs/plugins/donut-theory/presets.md) and the [implementation and verification record](https://github.com/Sunghyun23/kimswork/blob/main/mpc-software/docs/plugins/donut-theory/implementation.md).

## Earlier changes in 1.1.2

- **Default sound:** all 63 playable factory sounds have Moog-inspired voicing: revised oscillator blends, low-pass settings, envelopes, restrained detune and Tape saturation. Effects and pitch movement are reduced where appropriate. This is recipe retuning within the existing synthesis engine, not a new ladder-filter model.
- **Level and low end:** stereo renders were used to recalibrate Sound Trim. Individual low-cut settings remain; dry Init is unchanged. Velocity, register, resonance, drive and overlapping chords still affect output level.
- **Chord navigation:** chord-preset and slot-page arrows have 44 × 44 touch targets. Four large 222 × 70 chord pads remain. Repeated HOLD TO PLAY captions are removed, with chord names and optional location hints sharing the pad face.
- **Physical-position hints:** the shared CHORD editor has a manual A–H / 1–16 location drawer. MAIN shows the assigned label and 4 × 4 highlight. Clear removes the hint without changing MIDI. Register the actual input with LEARN.
- **Compatibility:** previous parameter IDs/order and Q-Link mappings are preserved; sixteen nonautomatable position parameters are appended. Existing project sounds retain their saved settings. Select a factory sound to hear its revised recipe. Old projects load with unassigned hints.

Builds for native, ARM32 and ARM64, SDK validation, host simulator 19 checks, engine/UBSan/VST bridge and skin regressions pass. All 64 sounds have finite, non-silent offline renders within the measured test bounds. On MPC Live III / OS 3.10.1.12, the new location selection/clear/MAIN display and Q-Link guide hide/restore were checked through Remote Screen Viewer. A complete hardware listening review of all revoiced sounds, fresh physical LEARN inputs, all physical Q-Link rotations, simultaneous finger multi-touch and a new hardware project save/restart round-trip were not completed for this update. Automated state migration and note ownership tests pass.


## What changed in 1.2.0

- **Engine:** new nonlinear oscillator summer, four-stage nonlinear ladder Low-pass, simultaneous resonant feedback solve, 2× ladder processing with FIR output filtering, and rounded VCA response. No new UI or engine mode. The other filter types and downstream FX are retained.
- **Factory bank:** all 63 playable sounds have new cutoff/resonance/envelope operating points and gentler external drive. Stereo renders recalibrate Sound Trim; names, categories, Low-cut and Init parameter recipe remain. Intrinsic engine coloration applies to Init too.
- **Compatibility:** UID and all 208 parameter declarations/order/ranges, UI and Q-Link maps remain. Saved parameters restore but do not produce identical old sound. Use1.1.2 for exact old-engine recall.
- **Performance:** the ladder costs more CPU. A desktop16-voice benchmark measured about 5.8% average / 10.5% worst block in the recorded run; this does not predict MPC headroom. Overlapping long-release chords are the most demanding case.
- **Evidence:** dedicated filter response/nonlinearity/modulation/rate tests, engine/UBSan/bridge, host simulator19 and skin checks pass. All 64 sounds render finite/non-silent offline. Same-state A/B audio isolates engine changes from preset changes. Hardware verification and remaining limits are documented in the release notes.

[Engine design and research references](https://github.com/Sunghyun23/kimswork/blob/main/mpc-software/docs/plugins/donut-theory/engine.md)

Hardware follow-up in 1.2.1: the engine ran on MPC Live III / OS 3.10.1.12. All 64 factory programs changed and produced audible screen chords. Four screen holds and release tails, cutoff/resonance sweeps, all filter types and native preset restoration after restart were checked. Overlapping long-release chords showed 41% on the host CPU display (12% after release), not a worst-case guarantee. Remote Viewer reported output dropouts whose cause was not isolated. Every physical Q-Link, fresh physical MIDI/LEARN, simultaneous finger multi-touch and a full MPC project round-trip were not re-certified.


## What changed in 1.2.1

Fixed a layering bug that left chord-page arrow controls above the left selection drawer. Modal sidebars now render and intercept above ordinary tab widgets; Q-Link guides hide in the drawer and restore on closing. All seven plugin pages were visually checked on MPC. That update used actual 1.2.1 captures; current images show 1.2.2. DSP, 208 parameter identities and Q-Link mappings are unchanged from 1.2.0. The device save/reload check used a native .xpl plugin preset, not a complete MPC project.


## What changed in 1.2.2

Manual pad-location selectors, tiny grid hints and their bank/cell popup trees were removed after reports of stalled MAIN-to-CHORD switching and unreliable position display. Hidden widgets still cost construction work: the expanded CHORD tree drops from 10,889 to 1,913 widgets. MAIN and CHORD remain two native sub-tabs of the first function key; pressing it again switches between them. No navigation help popup is used.

On MPC Live III / OS 3.10.1.12, a fresh instance opened MAIN and entered CHORD, and repeated remote transitions followed by inversion selection, 4→8→12→16 expansion and bank 13–16 selection worked. Remote capture latency affected some rapid-test frames; this is not a timing benchmark or certification of every physical interaction. The original stall’s exact cause is not proven by a controlled old/new hardware A/B. The engine, sound recipes, UID, all 208 stored parameter declarations and Q-Link mappings remain unchanged. MIDI Note and LEARN remain the physical-pad connection controls. The old 16 location values are retained but unused by the UI.


Hardware follow-up for 1.2.2 (2026-11-15): the user confirmed normal MAIN↔CHORD switching by repeatedly tapping the first tab directly on the MPC. This supplements the remote check; all physical Q-Link and fresh LEARN inputs remain a separate verification scope.

## 1.2.3: front-panel SUB

MAIN A is now **OSC Pair / OSC Mix / Sub / Drift**. The Sub knob controls the existing independent sine sub at half OSC1 frequency; it does not add a second duplicate sub. It follows pitch/glide, then goes through the same filter and amp envelope as the other oscillators. Use Notes with Mono or Legato and lower Low-cut appropriately for bass playing. Raising Sub does not change oscillator balance. Factory programs retain their previous combined mix/sub amounts by moving the former Body contributions into explicit OSC Mix and Sub values. The deprecated Body parameter ID remains for host/state compatibility but is no longer exposed in the custom pages and no longer shapes audio. Old user projects containing Body automation may therefore sound different; their IDs are retained, not their old Body-dependent sound.

Native/ARM32/ARM64 builds, SDK validation, engine/UBSan, host bridge, analog-engine tests, host simulator and skin/Q-Link tests pass. An offline spectral test confirms110Hz SUB beneath220Hz OSC, silence at Sub0, increasing level at50/100 and no audio-thread allocations. These are automated tests; the new SUB hardware knob/listening behavior on MPC XL has not yet been confirmed. Version 1.2.3 publishes this change; hardware coverage remains as stated above.

## Harmony library


32 chord types and96four-slot progression presets. First11types and first32recipes retain their legacy meanings. Recipes change only the selected bank’s Root/Type; supported Voicing/Octave and MIDI settings remain. Reference Key does not automatically transpose a recipe.

### All chord types

| Type | Display | Voicing formula |
|---|---|---|
| Minor 7 | Cm7 | 1 · b3 · 5 · b7 |
| Major 7 | Cmaj7 | 1 · 3 · 5 · 7 |
| Minor 9 | Cm9 | 1 · b3 · 5 · b7 · 9 |
| Major 9 | Cmaj9 | 1 · 3 · 5 · 7 · 9 |
| Dominant 7 | C7 | 1 · 3 · 5 · b7 |
| Dominant 9 | C9 | 1 · 3 · 5 · b7 · 9 |
| Dominant 13 | C13 | 1 · 3 · b7 · 9 · 13 |
| Sus 2 | Csus2 | 1 · 2 · 5 |
| Minor | Cm | 1 · b3 · 5 |
| Major | C | 1 · 3 · 5 |
| 7 flat 9 | C7b9 | 1 · 3 · 5 · b7 · b9 |
| Minor 6 | Cm6 | 1 · b3 · 5 · 6 |
| Major 6 | C6 | 1 · 3 · 5 · 6 |
| Minor 6/9 | Cm6/9 | 1 · b3 · 5 · 6 · 9 |
| Major 6/9 | C6/9 | 1 · 3 · 5 · 6 · 9 |
| Minor 11 | Cm11 | 1 · b3 · b7 · 9 · 11 |
| Minor 13 | Cm13 | 1 · b3 · b7 · 9 · 13 |
| Major 9 #11 | Cmaj9#11 | 1 · 3 · 7 · 9 · #11 |
| Major 13 | Cmaj13 | 1 · 3 · 7 · 9 · 13 |
| 7 sus4 | C7sus4 | 1 · 4 · 5 · b7 |
| 9 sus4 | C9sus4 | 1 · 4 · 5 · b7 · 9 |
| 13 sus4 | C13sus4 | 1 · 4 · b7 · 9 · 13 |
| 7 sharp 9 | C7#9 | 1 · 3 · 5 · b7 · #9 |
| 7 flat 13 | C7b13 | 1 · 3 · 5 · b7 · b13 |
| 7 altered | C7alt | 1 · 3 · b7 · b9 · #11 |
| Minor 7 flat 5 | Cm7b5 | 1 · b3 · b5 · b7 |
| Diminished 7 | Cdim7 | 1 · b3 · b5 · bb7 |
| Augmented 7 | C7#5 | 1 · 3 · #5 · b7 |
| Minor major 7 | CmMaj7 | 1 · b3 · 5 · 7 |
| Add 9 | Cadd9 | 1 · 3 · 5 · 9 |
| Minor add9 | Cmadd9 | 1 · b3 · 5 · 9 |
| Sus 4 | Csus4 | 1 · 4 · 5 |

Extended voicings use at most five tones; optional fifths/elevenths are omitted as shown. Existing inversions/Open voicing apply. Minor11, Minor13, Major9#11 and Major13 are compact synth voicings, not six- or seven-note stacks.

### All progression presets

New recipes are original harmonic designs for neo-soul, hip-hop, R&B, vamps, modal colors, gospel movement, borrowed chords and turnarounds; no recorded artist performance was copied. The64new harmonic fingerprints are distinct even after root transposition. They are starting points, not a claim of a specific artist’s exact chords.

| # | Family | Name | Chords |
|---|---|---|---|
| 1 | Classic | Sunday Soul | Dm9 → G13 → Cmaj9 → Am9 |
| 2 | Classic | Basement Loop | Fm9 → Bbm9 → Eb9 → Abmaj7 |
| 3 | Classic | After Hours | Dm9 → Gm9 → A7b9 → Dm9 |
| 4 | Classic | Velvet Morning | Ebmaj9 → Cm9 → Fm9 → Bb13 |
| 5 | Classic | Dusty Sunset | Am9 → Fmaj9 → Dm9 → E7b9 |
| 6 | Classic | Warm Windows | Gmaj9 → Em9 → Am9 → D13 |
| 7 | Classic | Open Sky | Csus2 → Fmaj9 → Asus2 → Gsus2 |
| 8 | Classic | Soul Turnaround | Cmaj9 → A7 → Dm9 → G9 |
| 9 | Classic | Sunday Soul +2 | Em9 → A13 → Dmaj9 → Bm9 |
| 10 | Classic | Basement Loop +2 | Gm9 → Cm9 → F9 → Bbmaj7 |
| 11 | Classic | After Hours +2 | Em9 → Am9 → B7b9 → Em9 |
| 12 | Classic | Velvet Morning +2 | Fmaj9 → Dm9 → Gm9 → C13 |
| 13 | Classic | Dusty Sunset +2 | Bm9 → Gmaj9 → Em9 → F#7b9 |
| 14 | Classic | Warm Windows +2 | Amaj9 → F#m9 → Bm9 → E13 |
| 15 | Classic | Open Sky +2 | Dsus2 → Gmaj9 → Bsus2 → Asus2 |
| 16 | Classic | Soul Turnaround +2 | Dmaj9 → B7 → Em9 → A9 |
| 17 | Classic | Sunday Soul -2 | Cm9 → F13 → Bbmaj9 → Gm9 |
| 18 | Classic | Basement Loop -2 | Ebm9 → Abm9 → C#9 → F#maj7 |
| 19 | Classic | After Hours -2 | Cm9 → Fm9 → G7b9 → Cm9 |
| 20 | Classic | Velvet Morning -2 | C#maj9 → Bbm9 → Ebm9 → Ab13 |
| 21 | Classic | Dusty Sunset -2 | Gm9 → Ebmaj9 → Cm9 → D7b9 |
| 22 | Classic | Warm Windows -2 | Fmaj9 → Dm9 → Gm9 → C13 |
| 23 | Classic | Open Sky -2 | Bbsus2 → Ebmaj9 → Gsus2 → Fsus2 |
| 24 | Classic | Soul Turnaround -2 | Bbmaj9 → G7 → Cm9 → F9 |
| 25 | Classic | Sunday Soul +5 | Gm9 → C13 → Fmaj9 → Dm9 |
| 26 | Classic | Basement Loop +5 | Bbm9 → Ebm9 → Ab9 → C#maj7 |
| 27 | Classic | After Hours +5 | Gm9 → Cm9 → D7b9 → Gm9 |
| 28 | Classic | Velvet Morning +5 | Abmaj9 → Fm9 → Bbm9 → Eb13 |
| 29 | Classic | Dusty Sunset +5 | Dm9 → Bbmaj9 → Gm9 → A7b9 |
| 30 | Classic | Warm Windows +5 | Cmaj9 → Am9 → Dm9 → G13 |
| 31 | Classic | Open Sky +5 | Fsus2 → Bbmaj9 → Dsus2 → Csus2 |
| 32 | Classic | Soul Turnaround +5 | Fmaj9 → D7 → Gm9 → C9 |
| 33 | Neo Soul | Neo / Blue Hour | Dm11 → G13sus4 → Cmaj13 → Am13 |
| 34 | Neo Soul | Neo / Honey Glass | Fmaj9#11 → E7alt → Am11 → D9sus4 |
| 35 | Neo Soul | Neo / Warm Linen | C6/9 → Ebmaj9#11 → Dm11 → G7alt |
| 36 | Neo Soul | Neo / Soft Landing | Bbmaj13 → A7alt → Dm11 → G13sus4 |
| 37 | Neo Soul | Neo / Velvet Steps | Fmaj13 → F#dim7 → Gm11 → C13sus4 |
| 38 | Neo Soul | Neo / Amber Room | Ebmaj9#11 → D7alt → Gm13 → C9sus4 |
| 39 | Neo Soul | Neo / Open Curtains | Cmaj13 → Bm7b5 → E7alt → Am11 |
| 40 | Neo Soul | Neo / Quiet Light | F6/9 → Fm6/9 → Cmaj13 → G13sus4 |
| 41 | Hip-hop | Pocket / Back Porch | Am11 → Am13 → Fmaj13 → E7#9 |
| 42 | Hip-hop | Pocket / Sidewalk | Dm11 → G9sus4 → Dm13 → Eb7alt |
| 43 | Hip-hop | Pocket / Head Nod | Cm11 → F13sus4 → Cm6/9 → G7#9 |
| 44 | Hip-hop | Pocket / Night Bus | Gm11 → C13sus4 → Bbmaj9#11 → A7alt |
| 45 | Hip-hop | Pocket / Dust Loop | Fm11 → F#maj9#11 → Fm6/9 → C7alt |
| 46 | Hip-hop | Pocket / Loose Ends | Am6/9 → Fmaj9#11 → Dm11 → E7alt |
| 47 | Hip-hop | Pocket / Low Ceiling | Em11 → A9sus4 → Fmaj13 → F#m7b5 |
| 48 | Hip-hop | Pocket / Tape Corner | Dm6/9 → Fmaj13 → Em7b5 → A7#9 |
| 49 | Vamps | Vamp / Two Chairs | Cm11 → Cm13 → F9sus4 → F13sus4 |
| 50 | Vamps | Vamp / Same Window | Am11 → Am6/9 → AmMaj7 → Am7 |
| 51 | Vamps | Vamp / Moon Carpet | Dm11 → Ebmaj9#11 → Dm11 → Ebmaj13 |
| 52 | Vamps | Vamp / One Block | Gm13 → C13sus4 → Gm11 → C9sus4 |
| 53 | Vamps | Vamp / Copper Haze | Fm6/9 → Bb9sus4 → Fm11 → Bb13sus4 |
| 54 | Vamps | Vamp / Slow Orbit | Cmaj9#11 → D9sus4 → Cmaj13 → D13sus4 |
| 55 | Vamps | Vamp / Last Train | Em11 → Fmaj9#11 → Em13 → F6/9 |
| 56 | Vamps | Vamp / Still Warm | Amadd9 → Am11 → F6/9 → E7sus4 |
| 57 | R&B | RnB / Late Call | Fmaj13 → G13sus4 → Em11 → A7alt |
| 58 | R&B | RnB / Silk Morning | Cmaj13 → Bm7b5 → E7#9 → Am13 |
| 59 | R&B | RnB / Half Awake | Am11 → Dm6/9 → G13sus4 → C6/9 |
| 60 | R&B | RnB / Deep Blue | Ebmaj13 → Cm11 → F9sus4 → Bb7alt |
| 61 | R&B | RnB / Soft Focus | Dm11 → G7alt → Cmaj9#11 → A7#9 |
| 62 | R&B | RnB / Slow Reply | Fmaj9#11 → Em11 → Eb7alt → Dm13 |
| 63 | R&B | RnB / Felt Letter | Bbmaj13 → Am6/9 → Ab7alt → Gm11 |
| 64 | R&B | RnB / Afterglow | C6/9 → E7alt → Am6/9 → G13sus4 |
| 65 | Modal | Modal / Mist Walk | Dm11 → Fmaj9#11 → G9sus4 → Bbmaj13 |
| 66 | Modal | Modal / Wide Street | Cm11 → Ebmaj13 → F13sus4 → Abmaj9#11 |
| 67 | Modal | Modal / Pale Gold | Fmaj9#11 → Gmaj9#11 → Em11 → A9sus4 |
| 68 | Modal | Modal / Waterline | Am11 → Cmaj13 → G9sus4 → Dm13 |
| 69 | Modal | Modal / Paper Sky | D9sus4 → Fmaj13 → Ebmaj9#11 → Dm11 |
| 70 | Modal | Modal / Far Window | Gm13 → Bbmaj9#11 → Fmaj13 → C9sus4 |
| 71 | Modal | Modal / Dusk Garden | Em11 → Gmaj13 → D9sus4 → Fmaj9#11 |
| 72 | Modal | Modal / Soft Horizon | Cmaj9#11 → Dmaj13 → Fmaj9#11 → G13sus4 |
| 73 | Gospel | Gospel / Front Door | Cmaj13 → Fmaj13 → Fm6/9 → C6/9 |
| 74 | Gospel | Gospel / Sunday Air | Fmaj13 → F#dim7 → Gm11 → C7alt |
| 75 | Gospel | Gospel / Home Keys | C6/9 → C13sus4 → Fmaj9#11 → Fm6 |
| 76 | Gospel | Gospel / Warm Choir | Dm7b5 → G7alt → Cm11 → Cm6/9 |
| 77 | Gospel | Gospel / Open Hands | F6/9 → F#dim7 → Cmaj13 → A7#9 |
| 78 | Gospel | Gospel / Golden Step | Cmaj13 → Ebdim7 → Dm11 → G7alt |
| 79 | Gospel | Gospel / Back Home | Fmaj13 → Fm11 → Em11 → A7alt |
| 80 | Gospel | Gospel / Amen Room | Cmaj13 → Fmaj9#11 → Abmaj13 → G13sus4 |
| 81 | Borrowed | Color / Side Door | Fmaj9#11 → Abmaj13 → Ebmaj9#11 → Cmaj13 |
| 82 | Borrowed | Color / Half Light | Cmaj13 → Ebmaj13 → Abmaj9#11 → G7alt |
| 83 | Borrowed | Color / Paper Moon | Dm11 → C#maj9#11 → Cmaj13 → Eb7alt |
| 84 | Borrowed | Color / Blue Carpet | Am6/9 → Abmaj9#11 → Fmaj13 → E7alt |
| 85 | Borrowed | Color / Bent Frame | C6/9 → C#dim7 → Dm11 → C#7alt |
| 86 | Borrowed | Color / Hidden Stair | Fmaj9#11 → Edim7 → Ebmaj13 → D7alt |
| 87 | Borrowed | Color / Warm Shadow | Cmaj13 → Cm11 → Fm6/9 → Bb13sus4 |
| 88 | Borrowed | Color / Velvet Turn | Ebmaj9#11 → Dm7b5 → C#7alt → C6/9 |
| 89 | Turnarounds | Turn / Night Return | Cmaj13 → A7alt → Dm11 → G13sus4 |
| 90 | Turnarounds | Turn / Soft Resolve | Bm7b5 → E7b9 → Am11 → Fmaj13 |
| 91 | Turnarounds | Turn / Back Alley | Dm11 → C#7alt → Cmaj13 → A7#9 |
| 92 | Turnarounds | Turn / Slow Corner | Fmaj13 → E7alt → Am11 → G13sus4 |
| 93 | Turnarounds | Turn / Dim Lantern | Cmaj13 → C#dim7 → Dm11 → G7#9 |
| 94 | Turnarounds | Turn / Late Checkout | Em7b5 → A7alt → Dm11 → G7alt |
| 95 | Turnarounds | Turn / Round Trip | C6/9 → F#7alt → Fmaj13 → G13sus4 |
| 96 | Turnarounds | Turn / Last Cup | Fmaj9#11 → Em11 → A7alt → D9sus4 |

## Shared motion — 1.6.0 development candidate

MAIN D / FX–MOTION has Drive, Glide, Mod Rate and Mod Depth on its four existing Q-Links. Touch MODE to choose Tremolo (volume pulses) or Auto Pan (left/right motion). Touch WAV for Sine, Triangle, Saw, Square or S&H. Rate0.1–15Hz and Depth0–100% are shared; only the selected effect runs after the mode transition settles. Depth0 stops motion. Filter LFO remains independent. The module is free-running; it does not add tempo synchronization.

For gentle floating stereo, start with Auto Pan, Sine, Rate0.3–0.6Hz and Depth20–35%. FX detail contains the base Pan knob: Center is unchanged stereo, L/R shift the balance; motion is centered on this position and clamps at the extremes. This is balance panning of the complete stereo signal, not per-note pan or stereo widening. Old projects open with Tremolo/Sine/Center. Legacy automation names Tremolo Rate/Depth refer to the same shared controls displayed as Mod Rate/Depth.

## Tempo-synced rates — 1.7.0 development candidate

The SYNC button beside each MAIN Rate knob is independent. With SYNC off, the knob and Q-Link set Hz. With SYNC on, they select1/16,1/8,1/4,1/2or1bar; the LCD shows the division. Turning SYNC off restores that module's previous free rate. Filter LFO reuses the MOD tab's existing Sync/Division; Motion sync applies to the selected Tremolo or Auto Pan mode. Filter and Motion can use different divisions. No dotted/triplet choices are added in this version.

Tempo follows the host. Valid PPQ while playing locks phase to the song; while stopped, the oscillator continues at its tempo-derived frequency. Missing/invalid tempo uses120BPM. The old1.6free-running-only description is superseded by this section. Existing custom/cached rate Q-Link assignments can still point to the legacy Hz parameter; reset them to the current plugin defaults or use a fresh instance to obtain mode-aware rate controls. Program/Track and Screen defaults both use the new rate relays.
