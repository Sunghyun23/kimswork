# Groove Dice 1.11.0

## 1.11.0 — six performance pads, immediate Dice

MAIN Dice changes the eligible main groove, controls and configured kit choices immediately. Stored performances and master FX stay unchanged. PERFORM Dice regenerates only its six visible unlocked slots: Fill A/B, Snare Roll A/B and Hat Roll A/B. Recorded/edited fills lock automatically. Each screen has its own Undo. Amount 0 leaves generation unchanged. GENERATE explicitly replaces the selected slot.

Default physical performance inputs fit one C Major octave: **A1 Fill A (24), A2 Fill B (26), A3 Snare Roll A (28), A4 Snare Roll B (29), A5 Hat Roll A (31), A6 Hat Roll B (33)**. Numbers are MIDI notes, not automatic bank detection. Notes mode/root/scale affect actual incoming notes; use slot Learn for another layout. No physical preset, Dice, LOOP or STOP commands remain. Use the screen PLAY LOOP toggle or MPC transport.

Direct drums default to **C1–C8: Kick, Snare, Closed Hat, Open Hat, Clap, Tom, Cymbal, Perc**, MIDI **72/74/76/77/79/81/83/84**. ROUTING can match another layout without changing MPC settings. Fill/roll presses schedule the complete chosen length ending at a bar boundary. A late press waits for the next valid ending; releasing early does not truncate it.

Old four-slot recordings, hidden C/D sequences and custom learned notes are retained for compatibility. The simplified PERFORM page and its bulk Dice expose only A/B of each family. First 918 parameter declarations remain in place; six retired action IDs are inert compatibility data, for 924 total. They cannot enable old MIDI action commands.

The owner confirmed the supplied six-pad/startup/menu/command-removal checklist on MPC XL / OS 3.10.1.12. Native/both-ARM/SDK, full regressions and the final host run (19 PASS, 0 WARN, 0 FAIL using direct-drum notes 72/74/76) pass. This does not certify every hardware route, physical encoder, full project recall or worst-case CPU.

Full current instructions: [release README](RELEASE_README.md), [manual](../../../docs/plugins/groove-dice/manual.md), [validation](VALIDATION.md).

## 1.9.0 — independent MAIN / PERFORM Dice

- MAIN ROLL DICE changes only eligible MAIN values, the main groove and the configured kit/part pool. Stored performance phrases and master FX remain unchanged, including when a new kit is picked.
- PERFORM ROLL DICE regenerates the twelve unlocked snare/hat/fill slots only. Generated fills use the current main groove at generation time; later MAIN changes leave those saved fills intact. Recorded/edited fills auto-lock. GENERATE remains an explicit selected-slot operation and can replace that selected slot even if locked.
- Each screen has its own UNDO history. PERFORM Undo/Generate do not replace MAIN history, and MAIN Undo does not restore performance banks. While playing, bulk Dice applies at the next bar; stopped, it applies immediately. PERFORM Undo cancels its queued Dice. Count-in, stop and seek cancel pending performance Dice. Random Amount0 is inert for both bulk generators.
- Every ROLL DICE button includes the same dice icon; MAIN, PERFORM and the fill editor retain identical footer geometry.
- PERFORM UNLOCK ALL clears only its twelve slot locks; MAIN retains its existing general unlock action. Footer geometry and all Q-Link assignments are unchanged.
- First917parameter declarations stay exact; append non-automatable PerformUndo at917 (918total). New transient history is cleared on restore/reset; persistent phrase/state suffix formats remain unchanged.
- The owner authorized NVMe installation and stable publication. Actual MPC XL/OS3.10.1.12 remote touch checks confirm independent MAIN/PERFORM Dice and Undo, slot protection, next-bar queue status and the shared dice icon. Evidence:design/device-1.9.0/. Automated validation results follow below; do not infer new physical testing from older release evidence.

# Groove Dice 1.9.0

Common footer on seven pages and fill editor. SOUND ACTIONS retains local Roll Sound/Undo Sound/Audition/Save Kit. Independent MAIN/PERFORM Dice and Undo;918parameters with the original917declarations preserved. Q-Link targets stay unchanged.

Version1.9.0 is installed on internal NVMe. All sixteen MAIN locks sit on the left and use the same transparent open/closed silhouette and padding as Donut Theory and Crate Press. The 44×44 touch surface is preserved. Actual remote QA is recorded separately from physical/audio/CPU certification.

Neo-soul and hip-hop drums for MPC: generate a one-bar groove, perform musical fills and rolls, then capture your take as editable MIDI.

- PERFORM uses the first native MAIN/PERFORM key. Bank B1–B4: snare rolls; B5–B8: fills; B9–B12: hat rolls; B13: LOOP; B14: STOP. B15/B16are unused. A-bank direct drums remain unchanged.
- One press plays a complete selected1/2/4-beat phrase ending at a bar boundary. Late presses wait for the next valid ending. Early release never truncates it; recorded fills retain their full stored length.
- Phrase-based random generation uses rhythmic cells, rests, strong/weak accents and restrained grace hits. Fills keep the main kick motif and2/4snare anchors, adding coherent snare/tom/perc replies. Hat rolls replace the automatic hat/ride/bell lane during their phrase.
- Rate/Velocity/Humanize/Length share four selected-slot Screen Q-Links. All twelve slots have locks and explicit GENERATE. MAIN Dice changes the main groove/controls and eligible kit pool while preserving performance slots and FX. PERFORM Dice changes its unlocked twelve slots only. Independent Undo histories preserve the other screen; recordings/edits auto-lock.
- MAIN has sixteen value padlocks, sixteen RULE styles per part and24straight/dotted/triplet/fill/roll hat rhythms. Hat Rhythm/Open Hat/Ride/Crash stay on MAIN; Drive/Dust/real stereo Room reverb stay on FX.
- Four kick/snare layers, linked hats, a percussion ensemble and independent Ride/Crash/Bell. SOUND1/2=Sample/Level/Tune/Shift; SOUND2/2=Start/Decay/LowCut/HighCut, with48px page arrows, REV on both pages and independent polarity on page2.
- Choose sounds from This Pack, All Packs or up to64selected kits. Kit/role/layer protection, user kits, MIDI learn, sample preview and eight stereo output pairs remain available.
- CAPTURE stores actual generated/direct drum notes; EXPORT MIDI writes a960PPQ SMF for import into MPC. Performance trigger keys are consumed. Live plugin MIDI output to the sequencer is unavailable.

## Install and update

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the complete `kim - VST - Groove Dice` folder to `VST2 Plugins`, preferably internal storage. Save work and restart/rescan MPC. Keep one active copy across attached drives.

Preserve **Content/Expansions** and **Content/Captures** during manual updates. Both are protected user_data in Open MPC portable installers. Older1.6.1catalog installers did not protect packs and should not be used for upgrades. Public packages contain only Original Workshop:128original synthesized sounds,96kit combinations,2authored beats and93timing templates. Optional `.gdexp` packs stay separate; no private/commercial library is bundled.

## Compatibility and verification

Vendor kim/UID KpGy. First880parameter declarations remain exact;37append for917total. Original fill/snare state layouts are retained; a separate suffix stores new hat sequences. Known default LOOP/STOP62/64move to69/71; custom learned mappings remain. A new hat slot conflicting with a retained mapping shows LEARN PAD until assigned. The plugin never changes MPC pad/scale settings. Older engines cannot retain the new settings.

Native/ARM32/ARM64, SDK validation, complete engine/UBSan/allocation/state/skin/expansion/VST suites pass; host simulator19PASS/0WARN/0FAIL. Actual MPC XL/OS3.10.1.12 remote tests verify the twelve-slot UI, hat playback/end, editing, locks/Dice and a55-note balanced MIDI export. The previous B1/B3physical confirmation remains targeted; fresh complete physical-pad/Q-Link, hardware-audio, latency/route and worst-case CPU certification are not implied. Signed Shift retains a30msbaseline latency. See VALIDATION.md and the [performance specification](../../../docs/plugins/groove-dice/performance.md).

## Create packs

[Groove Dice Pack Builder](https://kimswork.app/apps/groove-dice-pack-builder/) is a separate optional app for macOS Apple Silicon and Windows x64. Organize WAV/MIDI folders, review roles, generate/audition kits and export independent `.gdexp` files. See PACK_BUILDER.md and the [complete guide](https://kimswork.app/apps/groove-dice-pack-builder/manual/). The included core works without the builder.

## Development

Build: `python3 mpc-software/plugins/public/GrooveDice/tools/build.py all`

Tests: `python3 mpc-software/plugins/public/GrooveDice/tools/test.py`

## License

Plugin code and Original Workshop core are MIT; see LICENSE and Content/Licenses/. Original synthesis/authored MIDI use no external audio/MIDI/model inputs. Optional expansions retain their source rights. Pack Builder is separate and retains its dependency notices.
