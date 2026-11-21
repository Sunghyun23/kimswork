# Groove Dice 1.11.0 — User manual

## 1.11.0 — six performance pads, immediate Dice

MAIN Dice changes the eligible main groove, controls and configured kit choices immediately. Stored performances and master FX stay unchanged. PERFORM Dice regenerates only its six visible unlocked slots: Fill A/B, Snare Roll A/B and Hat Roll A/B. Recorded/edited fills lock automatically. Each screen has its own Undo. Amount 0 leaves generation unchanged. GENERATE explicitly replaces the selected slot.

Default physical performance inputs fit one C Major octave: **A1 Fill A (24), A2 Fill B (26), A3 Snare Roll A (28), A4 Snare Roll B (29), A5 Hat Roll A (31), A6 Hat Roll B (33)**. Numbers are MIDI notes, not automatic bank detection. Notes mode/root/scale affect actual incoming notes; use slot Learn for another layout. No physical preset, Dice, LOOP or STOP commands remain. Use the screen PLAY LOOP toggle or MPC transport.

Direct drums default to **C1–C8: Kick, Snare, Closed Hat, Open Hat, Clap, Tom, Cymbal, Perc**, MIDI **72/74/76/77/79/81/83/84**. ROUTING can match another layout without changing MPC settings. Fill/roll presses schedule the complete chosen length ending at a bar boundary. A late press waits for the next valid ending; releasing early does not truncate it.

Old four-slot recordings, hidden C/D sequences and custom learned notes are retained for compatibility. The simplified PERFORM page and its bulk Dice expose only A/B of each family. First 918 parameter declarations remain in place; six retired action IDs are inert compatibility data, for 924 total. They cannot enable old MIDI action commands.

The owner confirmed the supplied six-pad/startup/menu/command-removal checklist on MPC XL / OS 3.10.1.12. Native/both-ARM/SDK, full regressions and the final host run (19 PASS, 0 WARN, 0 FAIL using direct-drum notes 72/74/76) pass. This does not certify every hardware route, physical encoder, full project recall or worst-case CPU.

Groove Dice is a neo-soul, hip-hop and R&B drum instrument by kim for MPC standalone. It plays its own samples and generates audio internally. It does not send generated MIDI into the MPC sequencer.

## 1.8.1 — consistent controls

Every page and the fill editor share **UNLOCK ALL → ROLL DICE → UNDO** in the same bottom positions as Donut Theory and Crate Press. MAIN’s sixteen padlocks move to the left of their knobs. Their transparent open/closed artwork matches the other instruments; 44×44 touch targets remain, with no knob overlap.

On SOUND, the footer remains available on both pages. The **SOUND ACTIONS** button opens a drawer with **ROLL SOUND / UNDO SOUND / AUDITION / SAVE KIT**; these remain separate from global Dice. Sample-row Preview buttons and physical audition stay directly accessible. The SOUND1/2 counter and arrows remain beside the common footer.

MAIN Dice regenerates eligible MAIN values; PERFORM Dice separately regenerates visible unlocked performance phrases, honoring protection. FX uses its separate ROLL FX action. Moving buttons does not broaden randomisation. Engine, all 917 parameter definitions, presets, generated phrase logic and Q-Link targets are unchanged from 1.8.0.

## Installation

Use MPC standalone OS3.10.1 or later. In Preferences → Enhancements (Beta), enable **Enable Enhancements** and **External Plugin Support**. Current retests use MPC XL / OS3.10.1.12; earlier checks used MPC Live III. Other model compatibility and worst-case CPU are not certified by that test.

Download the complete `groove-dice-1.11.0-mpc.zip`, extract it and copy the whole `kim - VST - Groove Dice` folder into `VST2 Plugins` at the root of your MPC drive. Keep Content, VST and Plugin Skins together. Restart/rescan the MPC application and select Groove Dice under External Plugins on a plugin track. Save work before any MPC restart.

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

Each B control has **16 playable styles**, with no LOCK position. The original seven remain first; nine additions per part introduce pickups, ghost-note responses, hat accents and percussion phrases. Tap the LCD for the two-page list or turn its Q-Link. Use the separate padlock to protect a choice during Dice. The complete lists appear under Extended RULES below.

A previously saved legacy LOCK appears as BASIC with its padlock closed and frozen notes preserved. Opening that padlock releases the legacy hold. Existing host-saved custom Q-Link assignments may still point to the old compatibility parameter; choose Screen mode or update the assignment.

MAIN repeats one bar, with snare backbeats on 2/4 and kick motifs away from those exact beats. Rule and hat-division changes apply live to their part at the continuing transport position. Elapsed hits are not replayed. Velocity follows a metrical strong / weak / medium / weak shape, with restrained random variation; ghost notes stay softer than backbeats.

Hat Rhythm has **24 choices**: 1/4, 1/8, 1/16, 1/8 DOT, 1/16 DOT, 1/4 T, 1/8 T and 1/16 T, each plain, +FILL or +ROLL. FILL adds short pickups; ROLL adds short 32nd-note bursts. Dotted divisions continue across the bar boundary. Open Hat offers OFF / OFFBEAT / BAR END / FILL END, replacing eligible closed hits rather than doubling them. Ride offers OFF / 1/4 / 1/8 / 1/4+BELL / 1/8+BELL and replaces automatic MAIN hats while active; recorded fill hats remain available. Crash offers OFF / START / AFTER FILL / EVERY 4 / EVERY 8 / EVERY 16. It marks structural boundaries, not every repeated bar. Count-in remains silent.

**T means triplet**: three notes occupy the time of two notes of that division. Plain 1/4T, 1/8T and 1/16T give 6, 12 and 24 hits per 4/4 bar before RULE filtering. Tap Hat Rhythm to open its three-page left list; page buttons select 1/3, 2/3 or 3/3. The first fifteen choices retain their old meanings. MAIN D1 and GROOVE use the same full selector. A change rebuilds only the hat part immediately, without pressing Dice. Straight Swing does not warp dotted or triplet spacing; Hat Shift and bounded Random Nudge still apply. A Hat RULE, Density, active Ride or Open Hat replacement may reduce or replace closed hits. Fill and Roll suffixes add the existing short pickups or 32nd-note ending; they are separate from recorded Fill A–D.

The Hat Rhythm padlock protects its selection during global Dice; manual edits remain possible. Existing host-saved custom maps pointing to the legacy 15-choice Hat Rhythm parameter still work for those choices. Use **Screen** mode or reassign to **Full Hat Rhythm** to access all 24 choices. Old projects migrate their stored selection and per-pattern metadata without changing their note data. New Triplet data requires version1.6.0 or newer.

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

Direct drum notes are **72/74/76/77/79/81/83/84** (C-bank bottom eight in the reference C Major layout). The six performance triggers use the A-bank notes listed above. Custom learned mappings are retained.

Default closed/open hats share choke group1. Other parts are independent. KITS offers choke groups0(off)–16 and a short Choke Time for click control. A new closed hat can stop an open-hat tail. User kit settings store this configuration.

## Recording fills

Open **EDIT FILL** from MAIN. Select fill A–D and its duration (1beat/2beats/1bar/2bars/4bars). Choose Replace or Overdub and an optional count-in. Press REC FILL, then play the physical pads. The selected slot stays fixed during recording. Recording stops at the chosen boundary. Automatic pattern output is not recorded back into the fill.

The fill editor shows the recorded note count and grid. Its first four rows are Snare / Tom / Perc / Clap; PERC / DRUMS switches to Kick / Closed Hat / Open Hat / Cymbal. Both views edit the same fill, so every recorded role is accessible. Select a note and edit Nudge, Velocity, Gate and Chance. In Screen Q-Link mode A controls fill slot/length/mode/count-in, and B controls the selected note's timing/velocity/gate/chance. C/D are intentionally unassigned. Changing page must not leave MAIN's sound controls active there.

MAIN’s **PLAY FILL** or the fill editor’s **AUDITION** queues the selected fill for the next bar once, then returns to the base groove. Fill Every can schedule periodic fills from the GROOVE page. PERFORM Dice regenerates its visible unlocked fill slots; recorded/edited fills lock automatically. Older projects also load populated fill slots locked to protect existing work.

Internal fill notes are saved in the plugin/project state. They are not MIDI clips recorded into MPC's sequencer. Save the MPC project or a host preset for persistence.

## FX and routing

Drive adds saturation, Dust adds worn texture, Room adds a short stereo diffused reverb (replacing the earlier feedback echo) and Duck is an internal envelope effect on FX. Low Cut removes unwanted low energy. Mod Rate/Depth apply slow level movement. Master controls final gain. Complete per-part insert chains, a separate master FX chain and external audio sidechain input are not implemented.

There are16mono channels arranged as eight stereo pairs. With Separate Out off, the instrument sums to the first pair. With it on, roles route to selected stereo pairs; pair1 defaults to Kick/Main, then Snare, Closed Hat, Open Hat, Clap, Tom, Cymbal, Perc. There is no extra ninth stereo mix output. Set destinations in MPC's mixer; the plugin does not create sequencer tracks automatically. All destination paths have not been exhaustively certified on hardware.

## Create your own expansion packs

[Groove Dice Pack Builder](https://kimswork.app/apps/groove-dice-pack-builder/) is an optional standalone desktop app for **macOS Apple Silicon** and **Windows x64**. [Download and platform details](https://kimswork.app/apps/groove-dice-pack-builder/) · [Complete builder manual](https://kimswork.app/apps/groove-dice-pack-builder/manual/). The packaged app needs no Python installation; it is separate from the MPC plugin. Groove Dice plays its included core without it.

1. Put your own WAV files in Kick, Snare, ClosedHat, OpenHat, Clap, Tom, Cymbal and Perc folders. Optional MIDI goes in MIDI/Beats and MIDI/Grooves. Named `Kits/<Name>/<Role>/` folders are also supported.
2. Open Folder, review roles, set name/creator/version and choose the appropriate rights classification.
3. Generate and edit kits, audition them, then save the source mapping as `.gdproject` and export `.gdexp`.
4. In Manage Packs, select the installed Groove Dice **Content** directory on locally accessible MPC storage and install the `.gdexp`. Choosing an arbitrary local folder does not transfer it to the MPC.
5. Safely return removable storage to MPC, close/reopen the instrument, choose the new pack in its kit browser and select a kit. Use This Pack to constrain kit rolls, or All Packs/Select Kits to broaden the pool.

Keep Pack ID stable when updating. Original source files stay external and unchanged. See bundled `PACK_BUILDER.md` for folder layout, supported WAV/MIDI, installation/removal and missing-pack recovery. Own expansion packs are independent files; no engine rebuild or core-bank merge is needed. Personal source libraries remain personal unless their terms permit redistribution.

## Save, update and expansions

Open MPC1.6.2+ packages mark `Content/Expansions` and `Content/Captures` as persistent user data. The standard catalog installer copies your installed packs and registry into the updated folder before replacing the old installation. Back up important packs/projects anyway. Avoid using the older1.6.1catalog packages for upgrades: they omitted this declaration. For manual SDK-folder replacement, preserve and restore your own Content/Expansions directory; an ordinary folder copy is not the catalog installer.


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

MAIN ROLL DICE regenerates eligible MAIN controls, main groove and configured kit choices only. PERFORM ROLL DICE separately regenerates its unlocked six visible performance slots. Recorded/edited fills lock automatically. Rate and Length retain their selected rhythmic roles; open slots vary their phrases and bounded Velocity/Humanize. FX values remain unchanged. ROLL FILL and ROLL FX are separate actions in their respective editors. Older saved Extra values remain compatibility data and no longer select the global Dice scope. Each screen has its own UNDO: MAIN preserves performance slots; PERFORM preserves the main groove/kit/FX. Current padlocks are retained.

## Automatic MPC record count-in

FOLLOW recognizes both negative-position pre-roll and measured MPC XL positive free-running record pre-roll, waiting for the actual recording seek/bar anchor. No extra setting is required; Preview cannot bypass it and direct pads remain playable. The user confirmed count-in On and Off on MPC XL OS3.10.1.12. A stopped recording deliberately begun strictly inside a bar may wait until the next anchor/seek; aligned starts and punch-in during existing playback have automated coverage.

## Updating from earlier versions

Version1.8.0retains the first880parameter declarations exactly and appends37controls (917total). The four original fill banks and old snare-roll trailer keep their layout; new hat data uses a separate suffix. Old projects retain their notes and learned mappings while new hat slots initialize. Older engines cannot retain all new settings. Keep important projects and earlier installers for compatibility.

Current release notes describe automated checks, device confirmations and remaining hardware limits. Older screenshots are labeled with their captured version; they must not be mistaken for fresh 1.3.3 captures.

## 1.3.3 compatibility and verification

UID KpGy and the first 672 parameter identities remain unchanged. Four 16-style RULE selectors append at 672–675 (676 parameters total); earlier selectors remain as compatibility aliases for saved projects and automation. Bank/expansion formats are unchanged. Older plugin versions cannot preserve all new controls. Keep important projects and earlier installers before updating.

Native, ARM32 and ARM64 builds, SDK validation, UBSan/allocation guards, state/expansion compatibility and 19 host-simulator checks pass. The owner confirmed the installed 1.3.3 MAIN locks, visible Dice changes and RULE selectors work on MPC XL OS3.10.1.12. This does not certify every physical control, output destination or maximum hardware load. Actual MPC screen captures accompany this release; offline UI renders are not represented as device captures.

## 1.3.4 update

All SOUND rows now have an independent REV toggle. The original643 parameter declarations are retained; seventeen automatable reverse flags and four UI reverse relays, four retained Tune relays and four polarity relays append for672 total. Existing GDLY/GDCY bank/layer layouts remain unchanged. A separate versioned trailer stores the reverse settings of eight user-kit slots. Actual MPC remote-click verification and direct screen captures confirm page switching, category retention and independent REV/Ø highlights. Physical encoder and hardware-audio confirmation remain separate.

## Extended RULES — 1.4.0

Each MAIN B part now offers16playable styles, with no LOCK value. The first seven keep their earlier musical meaning; nine new rules per part add36patterns. The padlocks remain separate. Use the existing Q-Link or tap the value for the two-page left list. MAIN reading order, four Q-Links and guide bounds stay unchanged. The footer shows PAGE1/2 and PAGE2/2. Selecting a style updates the live one-bar pattern without pressing Dice; Dice can choose the new styles while honoring locks and Amount bounds.

## Kick

BASIC, VARIED, BUSY, DOWNBEAT, OFFBEATS, SYNCOPATE, DOUBLE HIT, ONE THREE, ANTICIPATE, LATE ANSWER, PICKUP, THREE HITS, FOUR HITS, SYNC PAIR, STAGGER, END DOUBLE.

## Snare

BASIC, VARIED, BUSY, SOFT, PRE GHOST, POST GHOST, 2 GHOSTS, SECOND UP, FIRST UP, LOW ACCENT, POST BOTH, EARLY PAIR, LATE PAIR, TRIPLET, DRAG IN, BAR TURN.

## Hat

BASIC, VARIED, BUSY, SPARSE, OFFBEATS, ACCENTED, OPEN HAT, FIRST HALF, LAST HALF, ON BEATS, UPBEATS, TWO ON ONE, ONE ON TWO, SOFT TAIL, CRESCENDO, SKIP TWO.

## Perc

BASIC, VARIED, BUSY, SPARSE, OFFBEATS, ANSWER, PERC LOOP, CLAP REPLY, TOM TURN, SHAKER, CALL REPLY, OFFBEAT TOM, CLAP PAIR, LATIN THREE, TOM CLAP, END PICKUP.

## Musical behavior

Kick variants use distinct bounded sixteenth-note motifs, keep the downbeat and avoid the snare's exact2/4beats. Snare variants always keep both backbeats, changing main accents and adding quiet pre/post, triplet, drag or bar-ending ghosts. DRAG IN alone permits close32nd ghost spacing; caps and duplicate/backbeat protection remain. Hats keep the separate Hat Rhythm division, shaping halves, beat placement, omissions or crescendo/tail accents. Explicit FILL/ROLL ornaments remain controlled by Hat Rhythm. Perc uses role7percussion, role5toms and role4claps in distinct call/reply motifs; sound kit/choke routing is unchanged.

The repeating MAIN bar remains one bar. Recorded fills and other parts are preserved by a live part edit. Rules do not modify direct pad/MIDI playing. Existing Hat/Open/Ride/Crash and count-in behavior remain.

## Compatibility and checks

All672previous declarations/normalized ranges remain exact. Four16-choice controls append at672–675(total676); old4/8/7-choice aliases keep their ranges. Old state seeds the appended selectors; newer state retains them. Old aliases remain available for automation; legacy LOCK migration still becomes BASIC plus a closed padlock. Saved tracks may cache the previous seven-choice map; reset to defaults or compare a new instance to reach all16through Q-Link. Full-range controls, Dice and Undo share the same canonical styles. Legacy fallback values are readback aliases, not extra current styles.

Native/ARM32/ARM64 and SDK validation pass; full test/UBSan/bridge/skin/host suite is required. Targeted tests compare36distinct raw and36distinct actual scheduled new patterns, with variable32/512blocks, backbeats, kick exclusions, other-part/fill preservation and old672/full-state migration. Actual-device outcomes follow separately. Owner installs must preserve all four existing expansion banks and registry; never publish personal packs. This extended-rule system remains available in1.5.1.


## Predictable Dice — 1.5.1

**ROLL DICE** changes eligible MAIN controls and regenerates the repeating one-bar main pattern. MAIN padlocks preserve individual values; LOCK PATTERN preserves its notes. With LOCK KIT off, Dice can choose sounds within the selected pack/kit pool. LOCK KIT preserves the kit and MAIN mix trims. Random Amount0 changes nothing.

From1.7.0, unlocked fills/rolls regenerate with global Dice; locked slots and all seven FX values are preserved, including during Whole Kit replacement. The former Extra scope selector remains removed.

Use **ROLL FILL** at the bottom of the fill editor to vary the selected A–D bank only. It uses Random Amount, respects existing rhythm-role protection and is blocked during recording. **ROLL FX** at the bottom of FX changes Drive/Dust/Room/Duck only. Both actions support UNDO and do not roll the kit or main pattern.

MAIN has sixteen value padlocks, and PERFORM has six visible sequence padlocks. SOUND retains its meaningful kit/role/layer protections; GROOVE remains the detailed timing, density, ghosts, velocity and fill-frequency page. UNLOCK ALL opens individual and upper-level protection without changing any sound or note.

779append-only parameters retain all earlier ranges and IDs. Obsolete candidate lock fields remain for saved-state compatibility but are not displayed as misleading locks. Native/ARM builds, SDK validation, full regression/UBSan and SDK host19PASS/0WARN/0FAIL pass. Actual MPC XL3.10.1.12 checks cover all pages, MAIN roll/fill/FX preservation, dedicated ROLL FILL/ROLL FX, Undo and native preset selection. Physical encoders, hardware audio and worst-case CPU are not newly certified. The public ZIP includes Original Workshop only.


## Musical rolls and fills

Generation chooses complete rhythmic cells, rests and accent contours. Snare rolls use strong/weak subdivisions, restrained grace pickups and ending accents. Hat rolls use closed hats and replace the automatic hat/ride/bell lane during their finite phrase; other drum parts continue. Both retain the chosen straight/triplet division, with ACCEL tightening the last half. Strong Swing interpolates intermediate32nds so they do not collide with the next beat.

A generated fill keeps the main phrase before its transition, retains its kick motif and2/4snare anchors, then adds sparse snare, tom or percussion replies. The four profiles give different instrument routes. Recorded/edited fills keep their actual notes and automatic locks. One roll slot is active at a time; a new queued roll replaces the previous request. Rolls can overlay fills while replacing their matching snare/hat role. The same four Screen Q-Links edit whichever of six visible slots you select. SELECT TO EDIT suppresses screen audition, while physical assigned pads remain playable.

## PERFORM: play, capture and export

Press the first native MAIN/PERFORM key to switch pages. Six large pads group Fill A/B, Snare Roll A/B and Hat Roll A/B. Select a slot for Rate/Velocity/Humanize/Length on Screen Q-Link group A; other groups are inactive. SELECT TO EDIT makes screen pads select without playing. Assigned physical notes still perform. MAIN has a separate EDIT FILL entry for note editing.

MAIN Dice preserves performances; PERFORM Dice preserves the main groove/kit/FX. Both apply immediately. Locks, automatic protection of recordings and independent Undo remain. There are no physical Dice/preset/loop action pads.

CAPTURE records the actual generated groove, fills, snare rolls and directly played drum notes internally. Press again to stop; EXPORT MIDI writes the take to `Content/Captures/Groove-Dice-<timestamp>-<serial>.mid` inside this installed plugin folder. MIDI SAVED confirms export; EXPORT FAILED preserves the take for retry. Captures are temporary until exported; the limit is32768note-ons. The SMF is type0,960PPQ, with the starting tempo. Tempo changes during a take are not exported as a tempo map.

Use the MPC browser to find the installed plugin folder on that drive, open Content/Captures and import the .mid to a track using Groove Dice. Switch off automatic playback before playing the imported sequence to avoid doubling. Exported notes follow drum assignments; Ride/Crash/Bell use reserved replay notes96/97/98. This is file import/export, not live plugin MIDI output.

Keep `Content/Expansions` and `Content/Captures` when manually replacing an SDK folder. Open MPC portable installers declare both as user_data. Public packages contain neither owner expansions nor captured performances. See [complete performance specification](https://github.com/Sunghyun23/kimswork/blob/main/mpc-software/docs/plugins/groove-dice/performance.md).
