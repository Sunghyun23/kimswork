# Groove Dice 1.11.0

Neo-soul and hip-hop drums with musical grooves, original sounds and independent expansion packs.

## 1.11.0 — six performance pads, immediate Dice

MAIN Dice changes the eligible main groove, controls and configured kit choices immediately. Stored performances and master FX stay unchanged. PERFORM Dice regenerates only its six visible unlocked slots: Fill A/B, Snare Roll A/B and Hat Roll A/B. Recorded/edited fills lock automatically. Each screen has its own Undo. Amount 0 leaves generation unchanged. GENERATE explicitly replaces the selected slot.

Default physical performance inputs fit one C Major octave: **A1 Fill A (24), A2 Fill B (26), A3 Snare Roll A (28), A4 Snare Roll B (29), A5 Hat Roll A (31), A6 Hat Roll B (33)**. Numbers are MIDI notes, not automatic bank detection. Notes mode/root/scale affect actual incoming notes; use slot Learn for another layout. No physical preset, Dice, LOOP or STOP commands remain. Use the screen PLAY LOOP toggle or MPC transport.

Direct drums default to **C1–C8: Kick, Snare, Closed Hat, Open Hat, Clap, Tom, Cymbal, Perc**, MIDI **72/74/76/77/79/81/83/84**. ROUTING can match another layout without changing MPC settings. Fill/roll presses schedule the complete chosen length ending at a bar boundary. A late press waits for the next valid ending; releasing early does not truncate it.

Old four-slot recordings, hidden C/D sequences and custom learned notes are retained for compatibility. The simplified PERFORM page and its bulk Dice expose only A/B of each family. First 918 parameter declarations remain in place; six retired action IDs are inert compatibility data, for 924 total. They cannot enable old MIDI action commands.

The owner confirmed the supplied six-pad/startup/menu/command-removal checklist on MPC XL / OS 3.10.1.12. Native/both-ARM/SDK, full regressions and the final host run (19 PASS, 0 WARN, 0 FAIL using direct-drum notes 72/74/76) pass. This does not certify every hardware route, physical encoder, full project recall or worst-case CPU.

## Install and update

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the complete `kim - VST - Groove Dice` folder to `VST2 Plugins`, preferably internal storage. Save work and restart/rescan MPC. Keep one active copy across attached drives.

Preserve **Content/Expansions** and **Content/Captures** during manual updates. Both are protected user_data in Open MPC portable installers. Older1.6.1catalog installers did not protect packs and should not be used for upgrades. Public packages contain only Original Workshop:128original synthesized sounds,96kit combinations,2authored beats and93timing templates. Optional `.gdexp` packs stay separate; no private/commercial library is bundled.

## Compatibility and verification

Vendor kim / UID KpGy; 924 stable parameter declarations. Retired action IDs remain inert. Existing custom note maps, hidden legacy slot data and recordings are retained. Native/both-ARM/SDK/full regressions and host19 PASS/0 WARN/0 FAIL pass. Owner six-pad acceptance covers the current MPC XL / OS3.10.1.12 checklist, not all hardware audio/CPU/project-recall conditions. Signed Shift retains a30ms baseline latency.

## Create packs

[Groove Dice Pack Builder](https://kimswork.app/apps/groove-dice-pack-builder/) is a separate optional app for macOS Apple Silicon and Windows x64. Organize WAV/MIDI folders, review roles, generate/audition kits and export independent `.gdexp` files. See PACK_BUILDER.md and the [complete guide](https://kimswork.app/apps/groove-dice-pack-builder/manual/). The included core works without the builder.

## Development

Build: `python3 mpc-software/plugins/public/GrooveDice/tools/build.py all`

Tests: `python3 mpc-software/plugins/public/GrooveDice/tools/test.py`

## License

Plugin code and Original Workshop core are MIT; see LICENSE and Content/Licenses/. Original synthesis/authored MIDI use no external audio/MIDI/model inputs. Optional expansions retain their source rights. Pack Builder is separate and retains its dependency notices.
