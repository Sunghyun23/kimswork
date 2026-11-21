# Felt Club user manual

Version **0.3.2** · kim · MPC standalone instrument

## CPU and compatibility notice

Known limitation: the owner reported MPC CPU peaks above 100% during playing after this optimization. The triggering engine and exact workload were not isolated. Worst-case hardware CPU is not certified; this release does not claim that overload is fixed.

0.3.2 preserves all seven engines, 24 voice owners, presets and controls. Native pitch caching, silent initial-body/FX skips and Strum scheduling reduce measured desktop cost; 77 matched audio fixtures stay within 1e-6 FS (SNR above100dB). These are desktop measurements. No MIDI Dice/preset commands or PAD COMMANDS menus remain; six retired action parameters are inert, for246 stable IDs. Ordinary notes and chord MIDI mappings remain.

## Start here

Felt Club makes sound from seven independent reduced physical models. Choose a factory sound, play with varied velocity, then add motion or color. The instrument contains no recorded-note samples. Its vintage appearance is the interface; each engine’s hammer, string/reed or pickup is separate.

[Product and downloads](/plugins/felt-club/) · [Illustrated beginner guide](/learn/physical-modeling/)

## Installation and updates

1. Use MPC OS **3.10.1 or later**.
2. Enable **Enhancements** and **External Plugin Support** in Preferences → Enhancements (Beta).
3. Extract the SDK ZIP. Copy `kim - VST - Felt Club/` into `VST2 Plugins/` at the root of your chosen MPC drive. Keep the folder intact.
4. Save your project and restart the MPC application. Select an instrument track, then External Plugins → Felt Club.
5. For an update, replace that one installed folder. Avoid duplicate copies on mounted drives. Back up your projects before replacing an installed version.

The standard SDK ZIP has both ARM32 and ARM64 binaries. Per-architecture portable ZIPs are separate downloads for compatible Open MPC installers. Standalone support here requires OS 3.10.1+ even if another installer can register the binary on older firmware.

The verified installation is on internal NVMe. Removable-storage copying is separate from audio execution support. Library/download selection does not require an external sound bank.

![Actual Felt Club MAIN on MPC XL, with Noise at A4](/images/felt-club-main-0.3.0.webp)

## Choose an instrument

The top row groups models by role. **EP** contains Tine and Wurli; **PIANO** contains Upright and Felt; **PLUCK** contains Clav, Muted and Thumb. Selecting a model changes the engine; changing a page does not change the model or sound settings. Clav uses a key-driven string excitation, not a guitar pick.

| Engine | Character | Start with |
|---|---|---|
| Tine | Rounded metal body with a bell-like attack | Candle Tine |
| Wurli | Reed and capacitive pickup, soft to biting | Copper Wurli |
| Upright | Felt hammer, strings, bridge and soundboard | Sunday Upright |
| Felt | Independent piano basis with softer contact | Felt Letters |
| Clav | Dark, short key-driven strings and pickups | Dusty Clav |
| Muted | Palm-damped guitar pluck with a pitched body | Muted Thread |
| Thumb | Softened metal-tongue attack and body tail | Thumb Notes |

All are reduced, physically informed models with empirical voicing. Listening acceptance of a model does not establish an exact match to any commercial instrument.

## Factory bank

There are **57 programs**: Init plus **eight for each engine**. The first sound in each engine’s set is dry, so you can hear its attack and response. Later recipes add specific voicing, rhythm, motion or space. Names and indices match earlier local builds, while selecting a factory sound deliberately loads its new0.3.0 values.

Use the **bottom SOUND PRESET** name to open the category browser; choose EP, Piano or Pluck, then a sound. Arrows step through sounds. The same browser appears on every native page. The MPC’s upper **PRESET** selector also exposes the57 built-in programs, but its browsing presentation is controlled by the host.

Factory sound selection changes sound controls only: your chord roots/types/voicings/octaves, key, transposition, pad assignments and slot configuration remain. Stored project chunks retain their actual parameter values rather than being silently revoiced on load.

### The complete bank

| Category / model | Name | Role |
|---|---|---|
| Init | Init | Neutral dry Tine starting point |
| EP / Tine | Candle Tine | Dry rounded tine |
| EP / Tine | Sunday Velvet | Soft chord bed |
| EP / Tine | Dusty Suitcase | Dusty suitcase |
| EP / Tine | Afterglow | Synced autopan |
| EP / Tine | Amber Bell | Bell-forward keys |
| EP / Tine | Low Ceiling | Dark accompaniment |
| EP / Tine | Warm Current | Gentle chorus |
| EP / Tine | Night Letters | Evening echo |
| EP / Wurli | Copper Wurli | Dry reed |
| EP / Wurli | Soft Static | Soft reed chords |
| EP / Wurli | Basement Waltz | Pulsing tremolo |
| EP / Wurli | Brick Dust | Grainy rhythm |
| EP / Wurli | Wurli Bloom | Open reed chords |
| EP / Wurli | Quiet Block | Quiet accompaniment |
| EP / Wurli | Evening Grain | Slow stereo motion |
| EP / Wurli | Honey Circuit | Driven reed |
| Piano / Upright | Sunday Upright | Approved dry upright |
| Piano / Upright | Soft Floor | Soft upright chords |
| Piano / Upright | Loose Felt | Intimate piano |
| Piano / Upright | Low Window | Darker piano |
| Piano / Upright | Paper Keys | Clear rhythmic keys |
| Piano / Upright | After Hours | Late room |
| Piano / Upright | Dust In Light | Subtle stereo movement |
| Piano / Upright | Room Sketch | Room piano |
| Piano / Felt | Felt Letters | Dry felt piano |
| Piano / Felt | Small Hours | Dark quiet keys |
| Piano / Felt | Moss Chords | Soft chorus bed |
| Piano / Felt | Warm Rain | Warm delayed piano |
| Piano / Felt | Muted Ivory | Muffled dry keys |
| Piano / Felt | Slow Bloom | Slow stereo bloom |
| Piano / Felt | Hush Piano | Quiet accompaniment |
| Piano / Felt | Quiet Tape | Dusty felt |
| Pluck / Clav | Dusty Clav | Dry dark clav |
| Pluck / Clav | Copper Steps | Clear rhythm |
| Pluck / Clav | Short Answer | Short dry stabs |
| Pluck / Clav | Side Street | Stereo syncopation |
| Pluck / Clav | Pocket Clav | Tight pocket |
| Pluck / Clav | Muted Talk | Soft clav chords |
| Pluck / Clav | Wood Circuit | Warm body |
| Pluck / Clav | Friday Bounce | Driven funk |
| Pluck / Muted | Muted Thread | Dry palm-muted guitar |
| Pluck / Muted | Short Velvet | Soft muted chords |
| Pluck / Muted | Small Pick | Pick articulation |
| Pluck / Muted | Porch Light | Warm guitar room |
| Pluck / Muted | Soft Sync | Synced stereo plucks |
| Pluck / Muted | Dry Flowers | Dark dry plucks |
| Pluck / Muted | Night Pocket | Eighth-note echo and room |
| Pluck / Muted | Worn Strings | Worn muted guitar |
| Pluck / Thumb | Thumb Notes | Dry rounded thumb |
| Pluck / Thumb | Little Lantern | Warm thumb body |
| Pluck / Thumb | Brass Leaf | Gentle metal edge |
| Pluck / Thumb | Warm Pins | Short plucks |
| Pluck / Thumb | Soft Chimes | Soft chiming notes |
| Pluck / Thumb | Cloud Steps | Stereo floating notes |
| Pluck / Thumb | Slow Pebble | Dark rounded plucks |
| Pluck / Thumb | Paper Tines | Small echo figures |

## MAIN and Q-Links

Select **Screen** mode for page-specific assignments. Program and Track mappings remain the main sound mapping. The visible A–D groups each have four controls; details expose their own Screen controls. Sidebars hide the guides belonging to the page behind them.

| Group | Knob 1 | Knob 2 | Knob 3 | Knob 4 |
|---|---|---|---|---|
| A CHARACTER | Tone | Body | Model character3 | Noise |
| B AMP ENVELOPE | Attack | Decay | Sustain | Release |
| C MOTION | Rate | Depth | Drift | Width |
| D COLOR / FX | Drive | Chorus | Delay | Room |

**Tone** balances brightness. **Body** influences the resonating instrument response. The third Character label follows the model: Bell for Tine/Thumb, Reed for Wurli, String for pianos and Bite for Clav/Muted. The fourth MAIN knob is Noise. Bark, Board and Mute remain in the MODEL detail page. A character label describes a musical control, not a measured physical unit.

**Attack** shapes how quickly the played level rises; **Decay** moves that level toward **Sustain**; **Release** lets the sound finish after key-up. Physical damping also happens inside the instrument. These are two distinct processes: Sustain does not continuously excite a dead string, and Release cannot bring it back to life.

## Motion and effects

**TREM** moves volume. **PAN** moves the signal between left and right. Depth controls the strength; Rate controls the speed. **SYNC** follows host tempo with straight, dotted(D)and triplet(T)divisions. With Sync off, Rate is free-running. Motion Delay waits before applying movement; Fade In grows it smoothly. Drift is a small pitch motion, not oscillator detune. Width adjusts stereo spread.

**Drive** adds nonlinear color. **Chorus** adds a modulated short delay; its speed/depth are on FX. **Delay** is tempo-related echo, with division and feedback. **Room** is a diffuse reverb with Time and Damping. Damping darkens repeated reflections. Check in stereo and add effects after choosing the dry sound.

Muted’s **Night Pocket** is an eighth-note-delay/room recipe inspired by the approved audition direction: Delay24%, Feedback27%, Room24%, RoomTime43%, RoomDamping67%. It is an original sound recipe, not a recreation of a recorded guitar part.

## Noise and Mechanical are different

MAIN A4 **Noise** adds a soft, continuous, band-limited hiss after the instrument color/effect chain and before Master. It remains audible between notes while turned up; this is intentional. It has a 25 ms level smoother and does not depend on hammer strikes, pedal or note gates. It is generated noise, not a sampled tape recording or a measured tape-machine model. Set Noise to 0 for silence. Master still controls its level.

**Mechanical** on TOUCH is the existing strike-related noise control. **Bark / Board / Mute** remains on MODEL with its original stored values and musical meaning. Width stays on MAIN C4. Noise and Lock Texture Noise are appended at indices 238/239; the first 238 parameter declarations remain exactly unchanged. Loading an older saved sound defaults the new pair to Off, so the new noise floor cannot unexpectedly leak into old projects.

## Detail pages

| Page | What is here |
|---|---|
| MODEL | Model, Tone, Body, Pick; model characters, Detune and Tuning |
| TOUCH | Velocity sensitivity/curve, Hammer, pedal; Mechanical, Key Off, Resonance, Release |
| MOTION | Rate, Depth, Drift, Width; Shape, Motion Delay, Fade In, Drift Rate; TREM/PAN/SYNC |
| FX | Four effects; delay division/feedback, room time/damping; chorus rate/depth, Master, Width |
| SETUP | Master, tuning, bend range, keyboard/pad mode; key, transpose, strum velocity, pedal |

Duplicate controls on different pages refer to the same stored parameter. Moving them on either page updates the same sound. Navigation itself does not load presets or revoice held notes.

## Roll Dice, locks and Undo

Roll Dice generates a **new sound within the selected instrument**. Tine stays Tine, Wurli stays Wurli, and so on. The selected engine, velocity curve, Sync and all playing/chord settings are retained even with their locks open.

The seven profiles set different ranges for tone/body/character/contact, decay/release/detune, Drift and motion depth. Attacks remain short, Sustain remains high enough to expose the physical decay, and extreme detune/pitch drift are avoided. Body and Release are drawn together: a fuller Body reduces the generated upper Release bound.

Dice draws at most **one new color effect** from Drive, Chorus, Delay or Room. If a locked effect is already on, it does not add other unlocked color effects. More than one locked effect can remain active because a lock preserves your choices. Factory recipes and manual editing can also combine effects; this restriction is specifically for generated sounds. Random mechanical/key-off noise is disabled. Continuous tape-like hiss is a separate, occasional low-level draw at 5–18%; its own lock protects it.

Tap a small **lock at a knob’s upper left** to protect it. Locks represent shared parameters, including duplicates on detail pages. A protected value is preserved even outside the random range. **UNLOCK ALL** clears sound locks; it does not load a sound. **UNDO** restores the sound parameters and previous Dice random-generator state from the last roll, while retaining the locks you currently chose. Roll Dice/locks/Undo are in the same bottom action row on all pages.

Dice never changes chord presets, roots, types, inversions, octaves, key/transpose, pad notes, slot/page count, strum, chord glide, keyboard/pad mode, tuning or pitch-bend range. It does not create a chord progression or send MIDI notes out to the MPC sequencer.

![Actual MPC FX settings](/images/felt-club-fx-0.3.0.webp)

## Chords and pads

Tap the first MAIN function key again to switch to **CHORD**. It is a real native editing page; selecting another function key changes the page without altering sound. MAIN touch pads play only while held, followed by the current Release. A short strum queues the chord tones; releasing the source cancels notes that have not started.

Start with four slots and use +/− to activate up to 16in groups of four. Page arrows select the visible four slots. Reducing the count hides slots without deleting their definitions. Physical MIDI assignments are independent of the currently visible page.

In Screen mode, CHORD Q-Link groups A–Deach edit one visible slot: **Root / Type / Voicing / Octave**. Select a chord type, choose an available inversion/open voicing and adjust its octave. Key provides the harmonic reference; transpose shifts playback. Smooth Top Notes searches for voicings with a small top-note movement; it is an explicit editing action, not an automatic page-change effect.

Assign a MIDI note or enable **LEARN** for a slot, then press a physical pad or keyboard note. The first note registers the assignment and is consumed silently; the next one plays. A conflict message means another slot already uses that note. This does not edit the MPC’s pad layout or infer its displayed bank/pad position.

**Notes** mode plays direct notes; **Chords** uses mapped chord triggers; **Split** combines mapped triggers with ordinary direct notes. A bank label is not a guarantee of MIDI pitch: if a physical pad does not trigger a slot, learn the actual note sent by your current MPC pad setup.

**Strum** spreads a chord’s notes in time, with direction and velocity variation. **Chord Glide** connects overlapping chord triggers using the Glide time. Release the earlier chord after triggering the next; non-overlapping chords start independently. Touch and MIDI owners are separate, so releasing one source does not cut another owner’s note.

![Actual MPC chord editor with readable Strum and Glide](/images/felt-club-chord-0.3.0.webp)

## Save, recall and performance

Save the MPC project or a host preset for later recall. Plugin chunks preserve sound/chord parameters and locks; active touch gates, Learn and commands are disarmed on load to avoid stuck notes. Existing saved sound values remain stored; a factory index alone is not a promise of the old factory recipe after an update.

The engine has up to24 voice owners and a bounded pending-strum queue. It is designed for small expressive voicings. The acoustic models cost more than simple subtractive synthesis; actual worst-case MPC polyphony/CPU across all models is not certified. If a project overloads, reduce overlapping releases and chord density and use fewer instances.

## Verification and limits

Native, ARM32 and ARM64 builds plus SDK validation pass. The final native SDK host passes19 checks with no warnings/failures. Added checks exercise57 factory programs at 3 sample rates,3 registers and3 velocities (1,539 strikes),266 individual lock/Undo cases,448 unlocked bounded rolls, RT guards, state/MIDI/strum release, engine isolation and skin/Q-Link structure.

MPC XL / OS 3.10.1.12: 0.3.0 UI was checked; the latest0.3.2 is installed on internal NVMe, with all 600 files independently SHA-256 read back against the release folder. After the authorized restart, remote checks confirmed all seven native pages, stable page navigation, category→Sunday Velvet selection, a nonempty named host PRESET list, MAIN Noise adjustment, individual Tone/Noise locks, Dice engine retention and Undo. Tone 51% and Noise 60% stayed locked while other values changed; Undo restored the earlier visible sound. Final CHORD captions were checked after their contrast fix.

Physical Q-Link rotation, pad/keyboard/pedal input, device audio listening/recording, worst-case CPU/polyphony and full MPC project-save recall remain unverified. SD and SSD were untouched.

The native audio examples on the site are computer-rendered plugin output, not recordings from the MPC. Owner listening feedback guided the engines; no claim is made of an exact Lounge Lizard, Keyscape, real-instrument or recorded-song match. Unknown processing/performance in musical references does not constitute measured calibration. See the illustrated modeling guide for primary research and assumptions.

## Troubleshooting

- **Not in the browser:** check the required OS/preferences, folder name and startup scan; avoid duplicate installed folders.
- **No chord from a pad:** check input mode, active slot count and actual incoming MIDI assignment. Use Learn; do not assume a bank/pad name is a fixed note.
- **Dry note fades with Sustain high:** normal physical decay. Try a softer strike, more Body or another engine, then add a little Room.
- **Dice appears unchanged:** check locks. The engine and protected performance/chord settings intentionally remain.
- **Sync changes slowly:** Rate traverses eighteen discrete musical divisions. D means dotted and T means triplet; inspect the readout while moving the Q-Link.
- **Hung notes:** use the MPC all-notes-off/stop control or the plugin All Notes Off command; check sustain and overlapping MIDI sources.

## License

Original plugin code and artwork: MIT. SDK notice: MIT, separately included. Reference recordings and commercial plugin binaries are excluded from all public packages. Research links and musical references describe inspiration and limitations, not affiliation or endorsement.
