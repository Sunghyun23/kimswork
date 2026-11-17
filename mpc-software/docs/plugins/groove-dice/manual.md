# Groove Dice 1.3.4 — User manual

Groove Dice is a neo-soul, hip-hop and R&B drum instrument by kim for MPC standalone. It plays its own samples and generates audio internally. It does not send generated MIDI into the MPC sequencer.

## Installation

Use MPC standalone OS3.10.1 or later. In Preferences → Enhancements (Beta), enable **Enable Enhancements** and **External Plugin Support**. Current retests use MPC XL / OS3.10.1.12; earlier checks used MPC Live III. Other model compatibility and worst-case CPU are not certified by that test.

Download the complete `groove-dice-1.3.4-mpc.zip`, extract it and copy the whole `kim - VST - Groove Dice` folder into `VST2 Plugins` at the root of your MPC drive. Keep Content, VST and Plugin Skins together. Restart/rescan the MPC application and select Groove Dice under External Plugins on a plugin track. Save work before any MPC restart.

The base package contains **Original Workshop**:128 independently synthesized WAV one-shots,96 kit configurations, two authored beats and93 timing templates. Ninety-six kits are combinations/settings, not96 separately recorded drum libraries. Optional expansions are separate files; the base release contains no private/commercial third-party drum library.

## First loop

1. Open Groove Dice. Fresh instances start on MAIN.
2. Use the kit selector to choose a kit, or use the MPC's common PRESET list above the plugin.
3. Press **PLAY LOOP** to hear a loop while MPC transport is stopped. Press it again to stop.
4. Enable **MPC SYNC** (Follow) for playback controlled by MPC PLAY/STOP, tempo and position. The engine's bar-based behavior assumes4/4.
5. Open the padlocks beside controls you want Dice to change, choose a sound pool, then press **ROLL DICE**. Close a padlock to preserve that value. **LOCK KIT** protects sound selections and the four MAIN mix trims.
6. Use **UNDO** to return to the immediately preceding roll. There is one undo level, no redo. Undo restores rolled settings and notes; it does not change the padlock configuration. Save the MPC project to keep your work.

If nothing sounds, check the plugin track, monitoring, main output and transport/Preview state. After choosing a new large sample, allow its asynchronous load to finish. Missing expansion sounds are muted and reported; unrelated sounds are never silently substituted.

## MAIN Q-Link groups

Choose **Screen** Q-Link mode to follow the current plugin page. Program/Track maps stay fixed and do not become fill editing controls.

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A Timing | Swing | Snare Late | Hat Shift | Random Nudge |
| B Rules | Kick | Snare | Hat | Perc |
| C Drum Mix | Kick Level | Snare Level | Hat Level | Perc Level |
| D Hats / Cymbals | Hat Rhythm | Open Hat | Ride | Crash |

Swing shifts suitable subdivisions. Snare Late and Hat Shift offset their parts. Random Nudge adds bounded timing movement. RULES constrain generated variations; they do not restrict your direct physical pad playing.

Each B control has **seven playable styles**, with no LOCK position. Choose BASIC, VARIED or BUSY, plus role-specific options: downbeat/offbeat/syncopated/double-hit kicks; soft/ghost snare variations; sparse/offbeat/accented/open hats; and sparse/offbeat/snare-answer/percussion-loop patterns. Tap the LCD for its list or turn its Q-Link. Use the separate padlock to protect a choice during Dice.

A previously saved legacy LOCK appears as BASIC with its padlock closed and frozen notes preserved. Opening that padlock releases the legacy hold. Existing host-saved custom Q-Link assignments may still point to the old compatibility parameter; choose Screen mode or update the assignment.

MAIN repeats one bar, with snare backbeats on 2/4 and kick motifs away from those exact beats. Rule and hat-division changes apply live to their part at the continuing transport position. Elapsed hits are not replayed. Velocity follows a metrical strong / weak / medium / weak shape, with restrained random variation; ghost notes stay softer than backbeats.

Hat Rhythm has **15 choices**: 1/4, 1/8, 1/16, 1/8 DOT and 1/16 DOT, each plain, +FILL or +ROLL. FILL adds short pickups; ROLL adds short 32nd-note bursts. Dotted divisions continue across the bar boundary. Open Hat offers OFF / OFFBEAT / BAR END / FILL END, replacing eligible closed hits rather than doubling them. Ride offers OFF / 1/4 / 1/8 / 1/4+BELL / 1/8+BELL and replaces automatic MAIN hats while active; recorded fill hats remain available. Crash offers OFF / START / AFTER FILL / EVERY 4 / EVERY 8 / EVERY 16. It marks structural boundaries, not every repeated bar. Count-in remains silent.

Drive, Dust and Room are on **FX**, alongside Duck, Low Cut, modulation and Master.

## Kit and sound browsing

MAIN's kit LCD opens a paged kit browser. Pack arrows choose one installed pack or all expansions. The host's PRESET header also selects Init or an installed kit program. That header is MPC-owned and may display a flat list rather than factory-instrument category navigation.

**USE SOUNDS FROM** controls the Dice pool:

- **THIS PACK** uses the current pack filter. ALL EXPANSIONS includes all installed packs; select a named pack to restrict it.
- **ALL PACKS** includes the core and every installed expansion.
- **SELECT KITS** opens a checkbox browser for up to 64 kits across packs. CLEAR empties the set; DONE closes it. An empty or missing set does not borrow outside sounds.

Whole Kit selects one eligible kit. Mix Parts combines Kick, Snare, a linked Closed/Open Hat pair and a four-role Perc ensemble from eligible kits. Extra enabled/unlocked kick and snare layers also stay within the pool. Kit Fixed and layer locks protect sounds. Manually choosing a kit or host preset is still allowed.

## SOUND: two pages per sound

Choose KICK, SNARE, HI-HAT or PERC. Kick and Snare have up to four layers; HI-HAT has Closed/Open and Perc contains Clap/Tom/Cymbal/Perc. Each visible row owns one Screen Q-Link group, A through D. Hat C/D and Cymbals D have no sound assigned and do not change anything.

The lower-left **< SOUND 1 / 2 >** control has separate 48×48 previous/next arrows. Either arrow moves between the two pages; the center shows the current page. All rows switch together. Moving between drum categories or CYMBALS keeps the selected page.

| Page | Q-Link 1 | Q-Link 2 | Q-Link 3 | Q-Link 4 |
|---|---|---|---|---|
| SOUND 1 / 2 | Sample | Level | Tune | Shift |
| SOUND 2 / 2 | Start | Decay | Low Cut | High Cut |

Sample, level, pitch and timing are on page1. Page2 shortens tails and edits the sample start/filters. Turn **Decay** down to shorten hats or any sound smoothly. No separate End parameter is required. There is no separate selected-sound DETAIL screen: all rows expose the same second-page controls.

Page1 sample names open the left-side list. Previous/next sample arrows and the sample Q-Link step the same catalogue. In the list only Screen A1 selects the sample; other sound relays are inactive. Pack arrows filter choices, PREVIEW auditions it and DONE closes the list, returning to the current sound page. Background guides are suppressed while the browser is open.

Each row has ON / SOLO / LOCK / REV. **REV** reverses that source from its end on the next hit or preview; playing tails retain their original direction. Start trims from the source end when reversed. Decay and Shift remain independent. Page2 adds **Ø** beside the sound name for polarity inversion, which changes sign rather than playback direction. REV and Ø are separate, independent per source and saved with user kits/projects. Factory-kit selection resets reverse to forward. Sample browsing and sound Dice preserve direction.

Page1's row PREVIEW auditions only that layer and never records it into a fill. It bypasses row mute/solo and MAIN mix trim but retains sound processing, Master and FX. Preview goes to Main even with Separate Out and does not choke performance voices. Allow asynchronous sample loading after a selection; an empty layer has no sound. On page2 select the row and use AUDITION in the footer.

### Ride, Crash and Bell

CYMBALS uses the same two pages and controls, with A/B/C assigned to Ride/Crash/Bell. There is no extra selected-source D row. AUTO / CURRENT KIT chooses the matching family from the current kit's cymbal pack; BUILT-IN chooses the original synthesized fallback. Missing chosen assets keep their identity and stay silent. Ride/Bell follow Hat Mix and Crash follows Perc Mix; all share the Cymbal role's pan/choke/output pair. They do not create extra MIDI roles or output buses.

Signed Shift ±30ms uses a fixed reported 30ms baseline latency for every role, including physical pad monitoring. Hardware compensation and playing feel are not assumed. Sample start trims source audio; Shift moves onset.


## Physical pads

The default MIDI notes for Kick/Snare/Closed Hat/Open Hat/Clap/Tom/Cymbal/Perc are **24/26/28/29/31/33/35/36**. This matches the previously verified bottom-eight C1 Major pad layout on the test device. Your Pad Perform settings may differ. Set the eight Pad Note values on ROUTING to the actual notes sent by your pads; changing MPC's global pad scale is not required.

Default closed/open hats share choke group1. Other parts are independent. KITS offers choke groups0(off)–16 and a short Choke Time for click control. A new closed hat can stop an open-hat tail. User kit settings store this configuration.

## Recording fills

Open **EDIT FILL** from MAIN. Select fill A–D and its duration (1beat/2beats/1bar/2bars/4bars). Choose Replace or Overdub and an optional count-in. Press REC FILL, then play the physical pads. The selected slot stays fixed during recording. Recording stops at the chosen boundary. Automatic pattern output is not recorded back into the fill.

The fill editor shows the recorded note count and grid. Its first four rows are Snare / Tom / Perc / Clap; PERC / DRUMS switches to Kick / Closed Hat / Open Hat / Cymbal. Both views edit the same fill, so every recorded role is accessible. Select a note and edit Nudge, Velocity, Gate and Chance. In Screen Q-Link mode A controls fill slot/length/mode/count-in, and B controls the selected note's timing/velocity/gate/chance. C/D are intentionally unassigned. Changing page must not leave MAIN's sound controls active there.

MAIN’s **PLAY FILL** or the fill editor’s **AUDITION** queues the selected fill for the next bar once, then returns to the base groove. Fill Every can schedule periodic fills from the GROOVE page. Independent recorded fills are preserved by a Pattern-only roll; Fill/All scope explicitly permits fill variation.

Internal fill notes are saved in the plugin/project state. They are not MIDI clips recorded into MPC's sequencer. Save the MPC project or a host preset for persistence.

## FX and routing

Drive adds saturation, Dust adds worn texture, Room adds a short stereo diffused reverb (replacing the earlier feedback echo) and Duck is an internal envelope effect on FX. Low Cut removes unwanted low energy. Mod Rate/Depth apply slow level movement. Master controls final gain. Complete per-part insert chains, a separate master FX chain and external audio sidechain input are not implemented.

There are16mono channels arranged as eight stereo pairs. With Separate Out off, the instrument sums to the first pair. With it on, roles route to selected stereo pairs; pair1 defaults to Kick/Main, then Snare, Closed Hat, Open Hat, Clap, Tom, Cymbal, Perc. There is no extra ninth stereo mix output. Set destinations in MPC's mixer; the plugin does not create sequencer tracks automatically. All destination paths have not been exhaustively certified on hardware.

## Save, update and expansions

SAVE KIT copies current settings into one of eight internal user slots; LOAD KIT recalls it. **This is not disk saving.** Save the MPC project or use the host preset save function to preserve it.

Use Pack Builder's Manage Packs, choose the locally accessible installed plugin **Content** folder, then install a `.gdexp`. Files remain under `Content/Expansions/<pack-id>/` with a registry. Reopen Groove Dice after adding/removing a pack. This is a Groove Dice format, not MPC's native XPN Expansion Browser.

Removing a required pack can produce MISSING SAMPLE/PATTERN/GROOVE. Keep its original .gdexp. Reinstalling the same pack identity reconnects references, including projects re-saved while it was absent. Do not change a pack's stable ID for an update. Original0.8-and-older engines cannot read new expansion identities; keep their package if you need to return to those versions.

## Verification and limits

Native/ARM32/ARM64 builds, SDK validation, host simulator, engine/UBSan, sample bank, musical generation, fill, routing, skin and expansion-manager checks are recorded per release. Device checks and app-authored-pack tests are listed in the release notes and VERIFICATION.md. Passing those checks does not certify every physical Q-Link, audio device, multi-output destination or maximum hardware load.

The audio thread performs no file I/O, allocation, locks or logging. A bounded asynchronous cache loads selected WAV files. Installed pack count and maximum format size are technical bounds, not a recommendation to load the maximum on an MPC.

## Roll Dice, locks and Extra

Each of MAIN's sixteen knobs has a **44×44 padlock target**. Cream/open allows Dice to change its value; orange/closed preserves it. Manual touch/Q-Link changes remain possible. The locks are persistent project settings and are not randomised.

Roll Dice changes eligible visible values, so the readouts reflect what changed. Discrete choices can repeat by chance. Amount zero performs no randomisation. Timing ranges are bounded: Swing 50–66%, Snare Late −2–28ms, Hat Shift −12–8ms and Random Nudge 0–6ms. Lower Amount narrows their movement. Drum mix ranges stay practical: Kick −2–0dB, Snare −4–1dB, Hat −8–−2dB and Perc −12–−4dB. Divisions/cymbal accents use weighted choices rather than equal-probability chaos.

**KIT FIXED** protects all sound selections and the four MAIN mix trims. **Pattern Lock** additionally protects rule/cymbal choices and pattern regeneration. A knob padlock protects its value, not the whole note sequence: use Pattern Lock when the entire pattern must stay fixed.

**Extra: None / Pattern / Groove / Fill / All / FX** controls supplementary operations. None still rolls eligible visible MAIN values and unlocked sounds. Directly recorded fills change only when the chosen extra includes Fill. Undo restores the preceding rolled values, sources, notes and seeds while retaining the current padlocks. Manual kit and preset selection remain available when Kit Fixed is on.

## Automatic MPC record count-in

FOLLOW recognizes both negative-position pre-roll and measured MPC XL positive free-running record pre-roll, waiting for the actual recording seek/bar anchor. No extra setting is required; Preview cannot bypass it and direct pads remain playable. The user confirmed count-in On and Off on MPC XL OS3.10.1.12. A stopped recording deliberately begun strictly inside a bar may wait until the next anchor/seek; aligned starts and punch-in during existing playback have automated coverage.

## Updating from earlier versions

The first643 parameter declarations remain unchanged;1.3.4 appends29 controls for672 total. Old states initialize extra layers off. New layer states need the newer engine; keep older packages if you need to restore older projects. ROOM now sounds different for existing wet presets. Back up important projects before updating.

Current release notes describe automated checks, device confirmations and remaining hardware limits. Older screenshots are labeled with their captured version; they must not be mistaken for fresh 1.3.3 captures.

## 1.3.3 compatibility and verification

UID KpGy and the first 639 parameter identities remain unchanged. Four seven-style RULE selectors append at 639–642 (643 parameters total); legacy selectors remain for saved projects and automation. Bank/expansion formats are unchanged. Older plugin versions cannot preserve all new controls. Keep important projects and earlier installers before updating.

Native, ARM32 and ARM64 builds, SDK validation, UBSan/allocation guards, state/expansion compatibility and 19 host-simulator checks pass. The owner confirmed the installed 1.3.3 MAIN locks, visible Dice changes and RULE selectors work on MPC XL OS3.10.1.12. This does not certify every physical control, output destination or maximum hardware load. Actual MPC screen captures accompany this release; offline UI renders are not represented as device captures.

## 1.3.4 update

All SOUND rows now have an independent REV toggle. The original643 parameter declarations are retained; seventeen automatable reverse flags and four UI reverse relays, four retained Tune relays and four polarity relays append for672 total. Existing GDLY/GDCY bank/layer layouts remain unchanged. A separate versioned trailer stores the reverse settings of eight user-kit slots. Actual MPC remote-click verification and direct screen captures confirm page switching, category retention and independent REV/Ø highlights. Physical encoder and hardware-audio confirmation remain separate.
