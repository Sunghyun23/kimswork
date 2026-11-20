# Groove Dice 1.8.0

- Musical snare rolls use short rhythmic motifs, rests, beat accents, restrained grace notes and shaped endings. Generated fills retain the main kick motif and 2/4 snare anchors, adding sparse replies.
- Four finite hat rolls join four snare rolls and four fills on PERFORM. B1–B4: snare; B5–B8: fill; B9–B12: hats; B13: loop; B14: stop. Learn assignments inside the plugin to match a different host scale.
- Choose straight/triplet rates or acceleration, Velocity, Humanize and Length with four selected-slot Screen Q-Links. Rolls and fills play their full length at the earliest valid bar ending; late triggers wait for the next valid ending.
- Global Dice regenerates unlocked phrases. Every slot has a lock; Undo restores the generated phrases and settings. Recorded/edited fills remain protected.
- Count-in cancels previously queued or playing performance phrases, while direct pad monitoring remains available. Generated playback stays silent during count-in.
- First 880 parameter declarations are unchanged; 917 total. Older states and custom learned maps migrate. Portable packages preserve Content/Expansions and Content/Captures.

Native/ARM32/ARM64 builds and SDK validation pass. Full regression/UBSan and SDK host checks pass (19 PASS / 0 WARN / 0 FAIL). Actual MPC XL / OS 3.10.1.12 remote checks cover all four hat slots, generation, locks, Undo, rate editing and bar-end completion. An actual exported MIDI take contains 55 balanced drum notes, including 44 closed-hat hits and no trigger keys. New public-core screenshots show 128 WAV / 95 MIDI; optional owner packs were not loaded for those frames and all owner files were preserved. Physical B1/B3 confirmation belongs to the preceding 1.7.1 release; full new physical-pad/encoder, hardware-audio, MIDI import and worst-case CPU certification remain separate.

The download contains only Original Workshop: 128 original synthesized sounds, 96 kit combinations and 95 authored/template MIDI assets. No commercial or personal packs are included. Optional Pack Builder 1.0.0 for macOS Apple Silicon and Windows x64 creates separate .gdexp files; see PACK_BUILDER.md and the website.

Requires MPC standalone OS 3.10.1+ with Enhancements and External Plugin Support enabled. Keep one active plugin copy across attached drives. Export captured MIDI before closing the instance. This instrument does not send generated MIDI directly into the MPC sequencer.

All installation operations targeted internal NVMe only. SD and SSD were not read or modified. Official SDK remote tooling was used.
