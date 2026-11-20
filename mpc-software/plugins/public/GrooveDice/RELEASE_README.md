# Groove Dice 1.9.0

Neo-soul and hip-hop drums for MPC: generate a one-bar groove, perform musical fills and rolls, then capture your take as editable MIDI.

- PERFORM uses the first native MAIN/PERFORM key. Bank B1–B4: snare rolls; B5–B8: fills; B9–B12: hat rolls; B13: LOOP; B14: STOP. B15/B16are unused. A-bank direct drums remain unchanged.
- One press plays a complete selected1/2/4-beat phrase ending at a bar boundary. Late presses wait for the next valid ending. Early release never truncates it; recorded fills retain their full stored length.
- Phrase-based random generation uses rhythmic cells, rests, strong/weak accents and restrained grace hits. Fills keep the main kick motif and2/4snare anchors, adding coherent snare/tom/perc replies. Hat rolls replace the automatic hat/ride/bell lane during their phrase.
- Rate/Velocity/Humanize/Length share four selected-slot Screen Q-Links. All twelve slots have locks and explicit GENERATE. MAIN Dice changes the main groove/controls and configured kit choices, preserving stored performance slots and FX. PERFORM Dice regenerates its unlocked twelve slots only. Each screen has its own Undo; recordings/edits lock automatically.
- MAIN has sixteen value padlocks, sixteen RULE styles per part and24straight/dotted/triplet/fill/roll hat rhythms. Hat Rhythm/Open Hat/Ride/Crash stay on MAIN; Drive/Dust/real stereo Room reverb stay on FX.
- Four kick/snare layers, linked hats, a percussion ensemble and independent Ride/Crash/Bell. SOUND1/2=Sample/Level/Tune/Shift; SOUND2/2=Start/Decay/LowCut/HighCut, with48px page arrows, REV on both pages and independent polarity on page2.
- Choose sounds from This Pack, All Packs or up to64selected kits. Kit/role/layer protection, user kits, MIDI learn, sample preview and eight stereo output pairs remain available.
- CAPTURE stores actual generated/direct drum notes; EXPORT MIDI writes a960PPQ SMF for import into MPC. Performance trigger keys are consumed. Live plugin MIDI output to the sequencer is unavailable.

## Install and update

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the complete `kim - VST - Groove Dice` folder to `VST2 Plugins`, preferably internal storage. Save work and restart/rescan MPC. Keep one active copy across attached drives.

Preserve **Content/Expansions** and **Content/Captures** during manual updates. Both are protected user_data in Open MPC portable installers. Older1.6.1catalog installers did not protect packs and should not be used for upgrades. Public packages contain only Original Workshop:128original synthesized sounds,96kit combinations,2authored beats and93timing templates. Optional `.gdexp` packs stay separate; no private/commercial library is bundled.

## Compatibility and verification

Vendor kim/UID KpGy. First917parameter declarations remain exact;PerformUndo appends for918total. Original fill/snare state layouts are retained; a separate suffix stores new hat sequences. Known default LOOP/STOP62/64move to69/71; custom learned mappings remain. A new hat slot conflicting with a retained mapping shows LEARN PAD until assigned. The plugin never changes MPC pad/scale settings. Older engines cannot retain the new settings.

Native/ARM32/ARM64, SDK validation, complete engine/UBSan/allocation/state/skin/expansion/VST suites pass; host simulator19PASS/0WARN/0FAIL. Actual MPC XL/OS3.10.1.12 remote tests verify the twelve-slot UI, hat playback/end, editing, locks/Dice and a55-note balanced MIDI export. The previous B1/B3physical confirmation remains targeted; fresh complete physical-pad/Q-Link, hardware-audio, latency/route and worst-case CPU certification are not implied. Signed Shift retains a30msbaseline latency. See VALIDATION.md and the [performance specification](../../../docs/plugins/groove-dice/performance.md).

## Create packs

[Groove Dice Pack Builder](https://kimswork.app/apps/groove-dice-pack-builder/) is a separate optional app for macOS Apple Silicon and Windows x64. Organize WAV/MIDI folders, review roles, generate/audition kits and export independent `.gdexp` files. See PACK_BUILDER.md and the [complete guide](https://kimswork.app/apps/groove-dice-pack-builder/manual/). The included core works without the builder.

## Development

Build: `python3 mpc-software/plugins/public/GrooveDice/tools/build.py all`

Tests: `python3 mpc-software/plugins/public/GrooveDice/tools/test.py`

## License

Plugin code and Original Workshop core are MIT; see LICENSE and Content/Licenses/. Original synthesis/authored MIDI use no external audio/MIDI/model inputs. Optional expansions retain their source rights. Pack Builder is separate and retains its dependency notices.
