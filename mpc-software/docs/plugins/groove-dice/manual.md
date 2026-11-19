# Groove Dice 1.6.1 — User manual

Groove Dice is a neo-soul, hip-hop and R&B drum instrument by kim for MPC standalone. It plays its own samples and generates audio internally. It does not send generated MIDI into the MPC sequencer.

## Installation

Use MPC standalone OS3.10.1 or later. In Preferences → Enhancements (Beta), enable **Enable Enhancements** and **External Plugin Support**. Current retests use MPC XL / OS3.10.1.12; earlier checks used MPC Live III. Other model compatibility and worst-case CPU are not certified by that test.

Download the complete `groove-dice-1.6.0-mpc.zip`, extract it and copy the whole `kim - VST - Groove Dice` folder into `VST2 Plugins` at the root of your MPC drive. Keep Content, VST and Plugin Skins together. Restart/rescan the MPC application and select Groove Dice under External Plugins on a plugin track. Save work before any MPC restart.

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

## Create your own expansion packs

[Groove Dice Pack Builder](https://kimswork.app/apps/groove-dice-pack-builder/) is an optional standalone desktop app for **macOS Apple Silicon** and **Windows x64**. [Download and platform details](https://kimswork.app/apps/groove-dice-pack-builder/) · [Complete builder manual](https://kimswork.app/apps/groove-dice-pack-builder/manual/). The packaged app needs no Python installation; it is separate from the MPC plugin. Groove Dice plays its included core without it.

1. Put your own WAV files in Kick, Snare, ClosedHat, OpenHat, Clap, Tom, Cymbal and Perc folders. Optional MIDI goes in MIDI/Beats and MIDI/Grooves. Named `Kits/<Name>/<Role>/` folders are also supported.
2. Open Folder, review roles, set name/creator/version and choose the appropriate rights classification.
3. Generate and edit kits, audition them, then save the source mapping as `.gdproject` and export `.gdexp`.
4. In Manage Packs, select the installed Groove Dice **Content** directory on locally accessible MPC storage and install the `.gdexp`. Choosing an arbitrary local folder does not transfer it to the MPC.
5. Safely return removable storage to MPC, close/reopen the instrument, choose the new pack in its kit browser and select a kit. Use This Pack to constrain kit rolls, or All Packs/Select Kits to broaden the pool.

Keep Pack ID stable when updating. Original source files stay external and unchanged. See bundled `PACK_BUILDER.md` for folder layout, supported WAV/MIDI, installation/removal and missing-pack recovery. Own expansion packs are independent files; no engine rebuild or core-bank merge is needed. Personal source libraries remain personal unless their terms permit redistribution.

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

All four recorded/edited fill banks and all seven master/kit FX values are preserved, including during Whole Kit replacement. The former Extra scope selector has been removed; there is no hidden Fill/FX scope.

Use **ROLL FILL** at the bottom of the fill editor to vary the selected A–D bank only. It uses Random Amount, respects existing rhythm-role protection and is blocked during recording. **ROLL FX** at the bottom of FX changes Drive/Dust/Room/Duck only. Both actions support UNDO and do not roll the kit or main pattern.

Individual Dice padlocks belong to MAIN's sixteen controls. SOUND retains its meaningful kit/role/layer protections; GROOVE remains the detailed timing, density, ghosts, velocity and fill-frequency page. UNLOCK ALL opens individual and upper-level protection without changing any sound or note.

779append-only parameters retain all earlier ranges and IDs. Obsolete candidate lock fields remain for saved-state compatibility but are not displayed as misleading locks. Native/ARM builds, SDK validation, full regression/UBSan and SDK host19PASS/0WARN/0FAIL pass. Actual MPC XL3.10.1.12 checks cover all pages, MAIN roll/fill/FX preservation, dedicated ROLL FILL/ROLL FX, Undo and native preset selection. Physical encoders, hardware audio and worst-case CPU are not newly certified. The public ZIP includes Original Workshop only.
