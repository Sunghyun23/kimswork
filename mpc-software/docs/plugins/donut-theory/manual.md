# Donut Theory 1.14.2 — User Manual

Donut Theory is an original Moog-inspired synth for MPC bass, keys, leads, pads and rich chord playing. Its nonlinear mixer, four-stage ladder low-pass and rounded VCA are an inspired design, not an exact recreation of a particular instrument or artist recording. No external samples are needed.

## First session

1. Install the complete MPC package, enable External Plugin Support and create a plugin track. A fresh instance opens on **POLY** with no startup message.
2. Tap the plugin's **PRESET** field: choose Init, Bass, Keys, Synth, Lead or Pad, then a sound. The MPC-owned top PRESET list also selects the66 factory programs, with category prefixes in their names.
3. POLY and MONO are **editing views of one shared sound**. Changing a tab changes neither sound nor actual voice mode. Use the explicit **VOICE POLY/MONO** buttons in the lower browse row to change the actual melodic mode. MONO's LEGATO choice is a separate preference.
4. Select **NOTES** for keyboard notes, **CHORDS** for assigned chord triggers or **SPLIT** for both. POLY's four large chord pads sound while held; release tails follow Amp Release and effects. Screen pads do not write MIDI into the sequencer.
5. Tap POLY again to open **CHORD**. Choose the bank inside the editor. Set roots/types/inversions/octaves, key reference, smooth voicings, chord glide and Strum there.
6. Press **ROLL DICE** to generate a complete sound. Close individual padlocks to protect values, use **UNDO** to restore the previous generation and **UNLOCK ALL** to release protection without changing the sound.

## Install, update and remove

### Requirements

- MPC standalone with **MPC OS 3.10.1 or later** and support for external plugins.
- In **Preferences → Enhancements (Beta)**, enable **Enable Enhancements** and **External Plugin Support**. Wording may vary with the MPC OS version.
- A drive accessible to the MPC with a `VST2 Plugins` directory.

The package includes ARM32 and ARM64 MPC binaries. The native development library is a test build, not a desktop instrument installer. This package does not establish support for a desktop DAW.

### Install

1. Download the Donut Theory MPC ZIP from its specific release, then extract it.
2. Locate the complete folder named `kim - VST - Donut Theory`.
3. Copy that folder into `VST2 Plugins` on your MPC-accessible drive (the internal NVMe is used on the reference device). Keep the contents and subfolders together.
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

Save and back up your MPC project first. Keep the previous package if you need to return to it. Version1.14.2 migrates the old oscillator/Sync values and preserves legacy parameter declarations. Keep a backup of important projects; older engines cannot represent all newer settings. Replace the existing plugin folder with the new complete folder, then restart MPC. Avoid duplicate installations on different drives.

Donut Theory retains its plugin identity across updates. Existing projects may retain host-side Q-Link assignments from an older instance. If a mapping looks wrong, compare a fresh plugin instance before changing the original track. Preserving the plugin identity does not guarantee that an older MPC OS can load a project saved by a newer OS.

### Remove

Save a copy of any project that uses the plugin, and render or replace its tracks if needed. Remove `kim - VST - Donut Theory` from `VST2 Plugins`, then restart MPC. Projects that reference it will require the plugin to restore those instrument tracks.


## Navigation and editing

Six MPC function keys provide **POLY / MONO / OSC / ENV / MOD / FX**. POLY's second subtab is CHORD. First press selects the relevant view; pressing POLY again toggles its two subtabs. There are no independent per-tab sound memories.

Tap a choice value to open its left sidebar. Choose a row, use NEXT/PREV for longer lists, or close with X/outside touch. A choice's Dice lock is in its sidebar header. Numeric knobs drag vertically; double-tap numerical fields where available for exact entry. Bottom actions follow the same browse/create arrangement throughout the instrument. In POLY, Settings is not a separate large footer button; use a detail view's **SETTINGS** to access the shared performance drawer.

Plugin PRESET browsing starts with categories on each new opening. The MPC-owned header remains a flat66-program list; it is not the same sidebar. Prev/next arrows step sounds. Explicit factory selection reapplies its recipe, resets both LFO Delays to0ms and preserves the chosen actual voice mode and harmony/MIDI/master settings. The host may treat the checked Random Sound row as the current custom sound; selecting another factory row or using the plugin browser selects a fresh recipe.

## Q-Link controls

Use **Screen** Q-Link mode for the current view. Program/Track remains a fixed synth map, not the CHORD editor. Existing MPC tracks may retain earlier custom maps; compare a fresh instance or reset defaults if a mapping does not match.

### POLY

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A OSC / Sound | Selected OSC Wave | Pitch: Tune/Fine | Selected OSC Level | Drift |
| B Filter / Tone | Filter Cutoff | Resonance | Body | Drive |
| C Selected LFO | Rate | Depth | Delay | Target |
| D Amp / Motion | Amp Attack | Amp Release | Motion Rate | Motion Depth |

### MONO

A, B and C are identical shared controls. D is **Glide / Amp Decay / Amp Release / Pitch Drop**. MONO and LEGATO choices sit above Glide; Glide Mode selects always versus overlap-only sliding. Pitch Drop Time remains a compact touch field. Release is the same saved Amp Release as POLY and ENV.

A's **OSC1/OSC2** selector chooses which oscillator to edit. **TUNE/FINE** changes the pitch edit resolution, not either saved value: semitones and cents add together. Changing focus does not rewrite oscillator settings. Drift is shared; Sub remains on OSC.

C's **LFO1/LFO2** selector chooses an editor. Both independent LFOs remain active when their Depth is nonzero. Shape and FREE/SYNC are touch choices, keeping four Q-Links for Rate/Depth/Delay/Target. Changing focus neither clears a value nor resets modulation phase.

## Oscillators, filter and envelopes

**OSC** exposes four banks:

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A OSC1 | Wave | Tune | Fine | Level |
| B OSC2 | Wave | Tune | Fine | Level |
| C Texture/Pulse | Sub | Drift | Pulse Width1 | Pulse Width2 |
| D Noise/FM/Tone | Noise Level | FM Amount | Tone | Unassigned |

Each oscillator has independent pitch, level and pulse width. Sub adds a one-octave-lower bass component; Drift adds slow analog-like pitch movement. FM and noise add edge. Body shapes weight independently of filter cutoff.

Filter type and Drive type remain local to the front B controls. Low-pass uses the nonlinear ladder; the other filter modes use the existing alternate filter path. Filter Cutoff sets the frequency and Resonance emphasizes the cutoff region. Drive adds saturation; FX contains its detailed mix/type and cleanup settings.

**ENV** places both editable ADSR curves together. Screen A is Amp Attack/Decay/Sustain/Release; B is the separate Filter ADSR. C is Filter Cutoff/Resonance/Env Amount/Key Track; D is unassigned. Amp Release is shared with both performance views; Filter Release is independent. Env Amount sets filter-envelope influence and Key Track makes response follow played pitch.

In MONO, **Pitch Drop** bends a released voice downward by0–24semitones over Drop Time. Amount0 is off. The remaining Amp Release determines how much of the bend remains audible. Sustain delays gate-off; fallback to an older held legato note does not trigger a release drop. The amount/time are retained across Dice and explicit factory changes.

## Modulation and Sync

Each LFO has per-voice **Delay0–5000ms**: it waits after note-on, then fades modulation in over10ms. Delay is deliberately outside Dice, and is independent for the two LFOs. Set Depth0 to remove that module's influence.

With FREE selected, RATE uses Hz. With SYNC selected, the same Rate knob traverses15 beat periods clockwise from slow to fast. Small turns accumulate; switching back to FREE retains the module's Hz value.

The base divisions are **1/16,1/8,1/4,1/2and1bar**, each straight, Dotted or Triplet. **D** is1.5times the straight duration; **T** is2/3. At120BPM,1/8=250ms,1/8D=375ms and1/8T≈166.7ms. A bar division is four quarter-note beats. Tap the synced Rate value for the8+7-row sidebar, which spells out Dotted/Triplet. All three modules—LFO1, LFO2 and shared Motion—have the same choices and remember their divisions independently.

**MOD** shows both LFOs simultaneously. Screen A=LFO1Rate/Depth/Delay/Target; B=LFO2Rate/Depth/Delay/Target; C=Motion Rate/Depth/Pan/Vibrato; D unassigned. Each LFO has its own Shape and FREE/SYNC controls.

Motion switches between **Tremolo** (volume motion) and **Auto Pan** (stereo movement), with Rate/Depth shared by these two alternatives. It is separate from both LFOs. Mode/Shape/FREE-SYNC sit with its controls. Pan sets the static stereo balance; Vibrato adds pitch motion. CC1 also adds pitch vibrato. For the Soul Studies factory sounds, set MPC to88BPM and use1/8Auto Pan for the approved wide movement; the plugin does not set host tempo.

## FX and output

FX Screen A=Drive/Drive Mix/Tone/Low Cut; B=Chorus/Space/Delay/Feedback; C=Sound Trim/Master Volume/Pan; D unassigned. Drive type, FX preset and Delay division use nearby touch lists. Space adds ambience, Chorus adds width and Delay/Feedback adds repeats. Low Cut removes unnecessary bass. Sound Trim belongs to the sound; Master Volume is retained across sound changes and excluded from Dice.

SETTINGS exposes shared Volume, Expression, Bend Range, Trigger Base, Strum, Motion and Pad Map. Input mode remains beside the sound browser. Changing a utility value explicitly changes that shared value; navigating to the drawer does not.

## Roll Dice, locks and Undo

ROLL DICE creates a new oscillator-to-FX sound within bounded musical families. **Dice Target** can follow the actual voice mode (Auto) or choose a family. The visible POLY/MONO tab is never the generation mode selector. Actual voice mode is preserved by Dice and factory selection.

There are61 shared value locks. The same parameter shown on different pages shares its lock. Selected OSC controls follow their actual oscillator; waveform/pair ownership stays linked. Each module's Rate/Sync/Division shares one lock. Choice locks live in sidebar headers. Locked values are excluded before generation; manual editing and explicit factory selection remain allowed.

Dice preserves harmony, key/voicing/octaves, all pad/MIDI settings, slot count/bank, master controls, Strum, Chord Glide, Pitch Drop and both LFO Delays. Unlocked modulation modules are generated with Sync enabled. A locked rate group keeps its existing rate/Sync/division state.

UNDO restores the complete sound from before the last successful generation, including exact saved Sync choices; one level, no redo, and not retained after plugin reload. UNLOCK ALL changes protection only and does not generate sound or alter notes. Save the MPC project to retain the sound and locks.

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

Press a POLY chord pad to play it and release to end its gate. This works in every Input mode. Release is musical: a long Amp Release, chorus or delay can continue after your finger leaves the screen. It is not a latching switch.

Changing the visible bank releases screen-held chords that leave the page. It does not release physical MIDI chords merely because their bank is no longer visible.

Screen pads audition the internal synth directly. **They do not send MIDI notes into the MPC sequencer.** To record a performance as MIDI, use physical pads or a MIDI keyboard with the appropriate input routing.

### Connect physical pads with LEARN

The plugin receives MIDI note numbers, not labels such as A1. MPC Pad Perform scales, octave settings and note layouts can change the notes sent by the same physical pad. You do not need to alter MPC's layout to match Donut Theory.

1. Press the first function key from POLY to open CHORD.
2. Select the desired four-slot bank. Add slots first if needed.
3. Tap **LEARN** for the target slot.
4. Press the physical pad or keyboard key you want to use.
5. The first note is consumed silently for registration. Confirm the displayed MIDI number, completion status and LEARN disengagement.
6. Press the same pad again to play its chord. Use CHORDS or SPLIT input mode. Learning while in NOTES changes the mode to SPLIT.

Use **CANCEL LEARN** to exit without assigning a note. Learning an already assigned note swaps slot assignments to retain unique mappings. A slot's MIDI Note display also supports double-tap manual assignment. MIDI Note is intentionally outside the chord Q-Link controls.

The first four default mappings are **24, 26, 28 and 29**. The complete sixteen-slot defaults are 24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 41, 43, 45, 47, 48 and 50. Only active slots respond. Use MIDI numbers as the reference: software can label the same number with different octave names.

In **CHORD → PAD SETUP**, **Individual** pad mapping uses each slot's assignment and is the recommended mode. **Consecutive / Trigger Base** retains older projects' consecutive-note routing. Ensure its range does not exceed MIDI note 127 when using many slots.

### Sustain and controllers

- **CC64** sustains MIDI notes/chords until the pedal is released.
- **CC1** adds pitch vibrato.
- Pitch bend follows the **Bend Range** setting.
- MIDI All Notes Off / All Sound Off and controller reset are handled by the engine.
- **CHORD → PAD SETUP → ALL NOTES OFF** clears screen, keyboard and physical-pad notes, sustain and effect tails. Use it to recover from an interrupted input stream.

## Chord editor

### Key reference and circle of fifths

The CHORD editor has a KEY selector and MAJOR/MINOR switch beside a circle of fifths. Selecting a key updates the reference diagram and seven diatonic chords; it does not change any stored chord or keyboard note. MINOR uses the natural minor scale. The outer ring shows major keys and the inner ring their relative minors. The highlighted sector identifies the selected key pair. This diagram is a reference, not a chord trigger. Enharmonic spellings are chosen for readability.

The small degree under each play pad identifies its root in the selected scale. Capital/lowercase follows the stored chord's major/minor quality; a dash means its root is outside the selected scale. It is a root-degree aid, not a full harmonic-function analysis or chord-quality validator. Borrowed chords remain editable.

**TRANSPOSE** opens a confirmation drawer. Set FROM to the original key of your progression; TO KEY is the selected reference key. The default FROM is C for older projects, and remembers the destination after a successful apply. The drawer shows the signed semitone movement and active slot count. APPLY moves the root of every active slot (4, 8, 12 or 16), including banks not currently visible, by the same nearest interval (−5 to +6 semitones; a tritone goes up). CANCEL and outside-touch dismissal make no note changes. Reapplying to the same key gives zero movement. Changing Major to Minor alone does not reharmonize the progression.

Chord type, voicing, per-slot octave, MIDI Note/LEARN, input mode, sound and inactive slots are preserved. If any root would exceed MIDI24–84, the entire operation is rejected; adjust the source key or chord register before trying again. A held screen chord follows the edited root; a physical MIDI chord uses the new root on its next trigger, with its existing release ownership preserved.

**PAD SETUP** holds the MIDI Note readouts/manual assignment and LEARN for the four slots in the selected bank. Close it to choose a different bank. The sixteen CHORD SCREEN Q-Links remain Root / Type / Voicing / Octave; Program/Track retains the synth mapping. All new drawers suppress background Q-Link guides.


![CHORD editor running on MPC XL](/images/donut-theory-chord-device-1.14.2.webp)

### Four to sixteen slots

New instances start with **four active slots**. The editor's **+ / −** changes the active count in groups of four: 4, 8, 12 or 16. Reducing the count retains the inactive slots' settings for later reactivation. Changing the active count clears sounding notes.

Use the in-screen **1–4 / 5–8 / 9–12 / 13–16** bank buttons to edit the available banks. POLY shares this selection and always shows four large pads. Its page arrows change banks; its CHORD PRESET arrows change the recipe for the visible bank. Version 1.1.2 gives both arrow pairs 44×44 touch targets while retaining the four large chord pads. Physical note mappings continue to address their slots independently of the visible bank.

### Q-Link editing

In **Q-LINK SCREEN**, groups A–D represent the four visible slots. Each group has:

| Q-Link | Setting |
|---|---|
| 1 | Root |
| 2 | Type |
| 3 | Voicing |
| 4 | Octave |

For bank 5–8, A edits slot 5, B slot 6, C slot 7 and D slot 8. MIDI Note stays a separate assignment field. Octave is available as a donut control and a value selection list.

### Expanded harmony, chord glide and strum

The instrument offers 32 chord types and 96 four-chord progression presets. The original11types/32recipes stay in their original positions;64additional recipes are grouped by name into Neo, Pocket, Vamp, RnB, Modal, Gospel, Color and Turn. Use CHORD PRESET on POLY to fill the visible four-slot bank. Each of four banks retains its own selection. The library includes minor11/13, major6/9, major9#11/13, suspended dominants, altered dominants, half-diminished, diminished7, augmented7 and minor-major7 colors. See [the complete harmony library](#harmony-library) for names and formulas.

**CHORD GLIDE** in the editor enables newest-held chord connection independently for each MIDI channel. It shares the existing GLIDE time control on MONO / CHORD and the CHORD touch field. Overlap two triggers: exact common MIDI tones keep their existing voices/envelopes, and remaining tones use a minimum-distance assignment. Releasing the newest trigger returns to an older still-held chord; releasing the last ends the chord. MIDI channel1 and screen chords share a chain. Separate channels remain separate. Sequential, non-overlapping presses snap rather than sliding from release tails. Global Mono/Legato still governs ordinary melody notes separately. A zero GLIDE time gives immediate pitch changes. The underlying time control is an exponential time constant, not a guaranteed end-to-end slide duration.

**STRUM** is the existing Chord Spread, now directly editable beside GLIDE. It plays each new chord once, bass-to-top, with0–40ms between tones. For example8ms gives a five-tone chord a32ms first-to-last spread. It does not repeat automatically. With chord glide enabled, common tones continue; moved pitches start their transitions in that same ascending sequence. New extra voices use the same onset spacing. Early note-off/close/panic cancels pending starts and pitch transitions. These touch fields share existing parameters; the sixteen CHORD SCREEN Q-Links remain Root/Type/Voicing/Octave, with the Type controls now spanning all32choices.

Legacy parameter declarations and ranges remain unchanged; full-range controls append separately. Old project values migrate by their saved plain choices. The initial roughly half-second load replay guard prevents old MPC normalized aliases from replacing new full-range choices. Legacy automation continues after that replay window. An older project with a stored Q-Link map may need its SCREEN assignments reset or a fresh instance to use the new full-range Type relays. Shared sixteen-voice and hardware-validation limits still apply.

### Smooth voicing

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


## Saving, automation and verification

Save sounds/locks/chord setup in your MPC project. The66 factory programs and original first 353parameter declarations retain their identity;391 parameters are present in1.14.2. Old oscillator aliases and five-choice Sync IDs retain their ranges and migrate to their original straight periods. New D/T choices cannot be fully represented by older engines. Temporary selectors/commands are not a replacement for saved musical state.

Source tests cover state migration, finite audio, voice/chord ownership, small incremental Sync input, all15 periods at multiple tempos, lock/Undo behavior, native presets and zero idle notification storms. Real MPC captures and remote UI interaction checks are recorded separately from physical encoders/multitouch, hardware audio listening/recording, full project round-trip and worst-case CPU, which are not newly certified. Other MPC models/firmware are not certified by an MPC XL test. See the versioned release notes and verification record for the exact scope.

If a value or map looks stale after an update, save work, restart/rescan and compare a fresh plugin instance. Keep only one active copy across attached drives; multiple copies appear as duplicate entries. Ensure the selected track is a plugin track, Input and monitoring are appropriate, Master Volume is audible and the intended MIDI notes actually arrive. If a chord pad plays only a short sound, check Amp Release and whether a sample-like envelope is intended; screen pads are hold gates, not sustain toggles.

## Factory sound list

| Program | Category | Sound |
|---|---|---|
| 1 | Init | Init |
| 2 | Bass | Pocket Anchor |
| 3 | Bass | Maple Thump |
| 4 | Bass | Brick Bounce |
| 5 | Bass | Molasses |
| 6 | Bass | Crate Knock |
| 7 | Bass | Soft Cone |
| 8 | Bass | Motor Hum |
| 9 | Bass | Felt Punch |
| 10 | Bass | Honey Slide |
| 11 | Bass | Walnut Pluck |
| 12 | Bass | Low Canvas |
| 13 | Bass | Dust Motor |
| 14 | Keys | Felt Circuit |
| 15 | Keys | Worn Tines |
| 16 | Keys | Honey Reeds |
| 17 | Keys | Kitchen Bell |
| 18 | Keys | Soft Hammer |
| 19 | Keys | Velour Clav |
| 20 | Keys | Milk Glass |
| 21 | Keys | Tape Celeste |
| 22 | Keys | Reed Cabinet |
| 23 | Keys | Bronze Mallet |
| 24 | Keys | Evening Felt |
| 25 | Keys | Pocket Organ |
| 26 | Keys | Pearl Hammer |
| 27 | Keys | Cocoa Tines |
| 28 | Keys | Dust Harp |
| 29 | Synth | Chopped Brass |
| 30 | Synth | Mellow Block |
| 31 | Synth | Velvet Stab |
| 32 | Synth | Amber Stack |
| 33 | Synth | Rubber Chord |
| 34 | Synth | Hollow Print |
| 35 | Synth | Record Bloom |
| 36 | Synth | Copper Pluck |
| 37 | Synth | Faded Poly |
| 38 | Synth | Small Ensemble |
| 39 | Synth | Syncopated Ink |
| 40 | Synth | Glass Chord |
| 41 | Lead | Ribbon Whistle |
| 42 | Lead | Bent Wire |
| 43 | Lead | Narrow Alley |
| 44 | Lead | Honey Horn |
| 45 | Lead | Syrup Line |
| 46 | Lead | Ink Flute |
| 47 | Lead | Frayed Saw |
| 48 | Lead | Pocket Squeak |
| 49 | Lead | Golden Thread |
| 50 | Lead | Melted Neon |
| 51 | Lead | Muted Trumpet |
| 52 | Lead | Round Needle |
| 53 | Pad | Draped Velvet |
| 54 | Pad | Paper Halo |
| 55 | Pad | Warm Projection |
| 56 | Pad | Woven Air |
| 57 | Pad | Satin Room |
| 58 | Pad | Faded Choir |
| 59 | Pad | Ember Strings |
| 60 | Pad | Curtain Light |
| 61 | Pad | Soft Horizon |
| 62 | Pad | Opaque Silk |
| 63 | Pad | Dusk Film |
| 64 | Pad | Golden Haze |
| 65 | Keys | Dusty Sunday |
| 66 | Synth | Velvet Melt |

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
