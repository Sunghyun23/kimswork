# Groove Dice 1.2.2 — User manual

Groove Dice is a neo-soul, hip-hop and R&B drum instrument by kim for MPC standalone. It plays its own samples and generates audio internally. It does not send generated MIDI into the MPC sequencer.

## Installation

Use MPC standalone OS3.10.1 or later. In Preferences → Enhancements (Beta), enable **Enable Enhancements** and **External Plugin Support**. Current retests use MPC XL / OS3.10.1.12; earlier checks used MPC Live III. other model compatibility and worst-case CPU are not certified by that test.

Download the complete `groove-dice-1.2.2-mpc.zip`, extract it and copy the whole `kim - VST - Groove Dice` folder into `VST2 Plugins` at the root of your MPC drive. Keep Content, VST and Plugin Skins together. Restart/rescan the MPC application and select Groove Dice under External Plugins on a plugin track. Save work before any MPC restart.

The base package contains **Original Workshop**:128 independently synthesized WAV one-shots,96 kit configurations, two authored beats and93 timing templates. Ninety-six kits are combinations/settings, not96 separately recorded drum libraries. Optional expansions are separate files; the base release contains no private/commercial third-party drum library.

## First loop

1. Open Groove Dice. Fresh instances start on MAIN.
2. Use the kit selector to choose a kit, or use the MPC's common PRESET list above the plugin.
3. Press **PREVIEW** to hear a loop while MPC transport is stopped. Press it again to stop.
4. Enable **FOLLOW** for playback controlled by MPC PLAY/STOP, tempo and position. The engine's bar-based behavior assumes4/4.
5. Choose **Kit + Pattern** scope and press **ROLL DICE** for a new kit and musical phrase. Lock the kit if you want to change only the rhythm.
6. Use **UNDO** to return to the immediately preceding roll. There is one undo level, no redo. KEEP retains the current groove/seed behavior; it is not a disk-save action.

If nothing sounds, check the plugin track, monitoring, main output and transport/Preview state. After choosing a new large sample, allow its asynchronous load to finish. Missing expansion sounds are muted and reported; unrelated sounds are never silently substituted.

## MAIN Q-Link groups

Choose **Screen** Q-Link mode to follow the current plugin page. Program/Track maps stay fixed and do not become fill editing controls.

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A Timing | Swing | Snare Late | Hat Shift | Random Nudge |
| B Rules | Kick | Snare | Hat | Perc |
| C Drum Mix | Kick Level | Snare Level | Hat Level | Perc Level |
| D Character | Drive | Dust | Room | Hat Rhythm |

Swing shifts suitable subdivisions. Snare Late and Hat Shift offset their parts. Random Nudge adds bounded timing movement. RULES constrain generated variations; they do not restrict your direct physical pad playing.

Each part has eight choices. LOCK preserves notes; BASIC, VARIED and BUSY increase variation. Kick adds DOWNBEAT / OFFBEATS / SYNCOPATED / DOUBLE HIT; Snare adds SOFT / GHOST BEFORE / GHOST AFTER / DOUBLE GHOST; Hat adds SPARSE / OFFBEATS / ACCENTED / OPEN HAT; Perc adds SPARSE / OFFBEATS / ANSWER SNARE / PERC LOOP.

MAIN repeats one bar, with snare backbeats on 2/4 and kick motifs away from those exact beats. Dice applies immediately without restarting transport. Rule changes rebuild only their part. Locked material is preserved; impossible locked combinations report a status rather than truncating notes.

Hat Rhythm offers 1/4, 1/8, 1/16, 1/8 DOT and 1/16 DOT, each plain, +FILL or +ROLL: 15 choices. Changes apply live without another roll. FILL adds response pickups; ROLL adds short 32nd-note bursts. A locked hat part remains unchanged.

## Kit and sound browsing

MAIN's kit LCD opens the kit browser. Header arrows choose all installed packs or one pack; its kits, role-sound browser and library share the filter. ROLL chooses kits and parts within that pack selection. PRIVATE labels identify a locally installed personal pack; they do not grant redistribution permission.

Selecting a kit changes sample choices, part sound settings and kit FX. It preserves master volume, pad MIDI notes, output routing, groove controls, recorded fills and user slots. **Whole Kit** rolls choose a different kit; **Mix Parts** can combine role choices. Hat sound choices pair closed and open hats. Perc is a set of Clap/Tom/Cymbal/Perc, not a single percussion note.

Open SOUND for KICK / SNARE / HI-HAT / PERC. Kick and Snare each have four layers. In Screen mode, A–D match the four rows: Sample / Level / Tune / Shift. Each sample field shows its name, previous/next arrows and vertical drag input; the first Q-Link of that row selects the same catalogue. Empty layers show + ADD SOUND and enable when a sound is selected. Hat retains closed/open voices (C/D edit their details); Perc retains Clap/Tom/Cymbal/Perc.

ON, SOLO and LOCK apply per row. DETAILS edits Start / Low Cut / High Cut / Decay plus polarity. ROLL SOUND respects enabled/unlocked rows and the selected pack, preserving rhythm, mix and timing. UNDO, AUDITION and SAVE KIT remain available. Saved user kits are part of plugin/project state, not exported expansion files.

Signed Shift ±30ms uses a fixed reported 30ms baseline latency for every role, including physical pad monitoring. Hardware compensation and playing feel are not assumed. Sample start trims source audio; Shift moves onset.

The MPC header PRESET list is host-owned. It contains Init and the installed kit programs; the plugin cannot promise the same category-first navigation as factory Akai instruments.

## Physical pads

The default MIDI notes for Kick/Snare/Closed Hat/Open Hat/Clap/Tom/Cymbal/Perc are **24/26/28/29/31/33/35/36**. This matches the previously verified bottom-eight C1 Major pad layout on the test device. Your Pad Perform settings may differ. Set the eight Pad Note values on ROUTING to the actual notes sent by your pads; changing MPC's global pad scale is not required.

Default closed/open hats share choke group1. Other parts are independent. KITS offers choke groups0(off)–16 and a short Choke Time for click control. A new closed hat can stop an open-hat tail. User kit settings store this configuration.

## Recording fills

Open **EDIT / REC** from MAIN. Select fill A–D and its duration (1beat/2beats/1bar/2bars/4bars). Choose Replace or Overdub and an optional count-in. Press REC FILL, then play the physical pads. The selected slot stays fixed during recording. Recording stops at the chosen boundary. Automatic pattern output is not recorded back into the fill.

The fill editor shows the recorded note count and grid. Select a note and edit Nudge, Velocity, Gate and Chance. In Screen Q-Link mode A controls fill slot/length/mode/count-in, and B controls the selected note's timing/velocity/gate/chance. C/D are intentionally unassigned. Changing page must not leave MAIN's sound controls active there.

AUDITION/TRIGGER FILL queues the selected fill for the next bar once, then returns to the base groove. Fill Every can schedule periodic fills from the GROOVE page. Independent recorded fills are preserved by a Pattern-only roll; Fill/All scope explicitly permits fill variation.

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

## Kit lock and random scope

Press LOCK KIT on MAIN; KIT FIXED reports that samples are protected from Dice. Effective scopes become Pattern Only, Groove Only, Fill Only, Rhythm + FX or FX Only; Kit Only becomes No Changes. Manual kit browsing and native preset selection still work. The lock is saved with the plugin state. Visible status labels report actions without cumulative numeric counters.

## Automatic MPC record count-in

FOLLOW recognizes both negative-position pre-roll and measured MPC XL positive free-running record pre-roll, waiting for the actual recording seek/bar anchor. No extra setting is required; Preview cannot bypass it and direct pads remain playable. The user confirmed count-in On and Off on MPC XL OS3.10.1.12. A stopped recording deliberately begun strictly inside a bar may wait until the next anchor/seek; aligned starts and punch-in during existing playback have automated coverage.

## Updating from earlier versions

The first 342 legacy parameter definitions are unchanged; new controls are appended (533 total). Old states initialize extra layers off. New layer states need the newer engine; keep older packages if you need to restore older projects. ROOM now sounds different for existing wet presets. Back up important projects before updating.

Current release notes describe automated checks, device confirmations and remaining hardware limits. Older screenshots are labeled with their captured version; they must not be mistaken for fresh 1.2.2 captures.
