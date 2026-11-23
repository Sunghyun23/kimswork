# Loop Club0.6.1 detailed QA

This is a local MPC Sample1.3.0.12 candidate. Normal runtime/SD SHA-256: `e2f4c0b9fcffd341997be3f2ca199336b8fd6678647b98ef71167c7c34b59e7b`. Public download remains0.1.0. The original owner project was saved before isolated tests, restored afterward and compared across all16track slots plus BPM/master/FX/selected synth settings.

## Findings fixed

| # | Observed problem | Change |
|---|---|---|
| 1 | Completed Audio REPLACE playback length was shortened before commit; cancel could make source metadata unloadable. | Keep completed PCM frame length intact until commit; capture preview stays separate. |
| 2 | A synth note held through count-in had no recorded note-on; its note-off could become orphaned. | Track held count-in notes and seed them once at tick0; exclude released notes and early drum hits. |
| 3 | After automatic recording handoff ended, external MIDI could target the older instrument. | Publish the final recorder/selection owner after each audio block while preserving newer queued UI selection. |
| 4 | Sound CHROMATIC/CHORD mode was UI-only and disappeared on load/restart. | Store a per-track boolean with serde defaults; refresh UI from the engine snapshot. |
| 5 | Rejected Audio handoff could still change selected instrument while another track was recording. | Failed input validation leaves the active recording selection/queue intact. |
| 6 | Actual USB MIDI note-off did not arrive at the SDK upper note hook. | Observe original ALSA sequencer note-on/off without changing its arguments/result/event bytes; retain port/channel ownership and suppress duplicate upper replay. |
| 7 | Browsing from an occupied Audio track gave the misleading instruction to select an Audio track. | Show USE EMPTY AUDIO TRACK while preserving the existing non-destructive import guard. |
| 8 | The instrument performance home and idle Audio track advertised misleading pad roles. | Restrict recording instructions to LOOP; performance home shows16PADS:PLAY and Audio shows input recording guidance. |

The MIDI issue was confirmed on the real device: two note-on/off pairs produced two upper SDK note-ons and no upper note-offs, while the ALSA sequencer supplied both starts and releases. The new observer follows the [official ALSA event ABI](https://github.com/alsa-project/alsa-lib/blob/master/include/seq_event.h), leaves native reads/events unchanged and identifies client/port/channel owners. The canonical SDK stayed unchanged. Both standard NoteOff and NoteOn-velocity-zero releases were then verified on hardware.

## Automated checks

- 240 Debug and 240 Release tests passed.
- Strict host/all-targets and ARM64-lib Clippy; normal and isolated ARM64 builds passed.
- New regression cases reproduce the Audio cancel, held count-in, post-handoff MIDI, mode recall and rejected input bugs. A deterministic4000-step transport/edit sequence checks bounded valid clips and finite output.
- Callback allocation/deallocation tests remain zero, including record-buffer handoffs, Undo/Redo and FX processing. Startup-only symbol resolution runs on the native MIDI input thread.
- Bypass FX now have a small inlined path that still records complete Reverse/Repeat history. Bounded history wrapping avoids repeated floating modulo; abrupt-window fallback and boundary behavior are tested. No voice count or effect capacity was reduced.

## Real device checks

All9main groups passed on the core candidate; the final change only corrected pad-role text, followed by repeated automated checks and actual supplemental/audio/screen checks. The core and final hashes are recorded separately in JSON. Coverage:16track targets/count-in cancellations and16ordinary performance pads; held count-in and excluded released notes; recording handoff and subsequent external MIDI; chord-mode save/load; nonempty Audio REPLACE cancel followed by valid project save/load; four chop modes plus split Undo/Redo; bank/kit browsing and two-layer controls; mixer/routes/MUTE/CHAIN advancement; FX settings/HOLD/LATCH/capacity,25repeated SEQ presses and save/recall.

Supplemental checks passed both actual MIDI release forms, the master-insert board with release after navigation, chop Gate versus One Shot and continuous USB capture. All15page types and8synth detail views were captured from the active DRM display; values were unchanged during the final owner-screen review. Personal bank names/payloads and raw project captures stay in ignored private evidence.

## CPU and audio observations

With32live MIDI notes, recorded loops, two track FX, five master texture stages and Duck, the main run reached a callback maximum of110.97% of its block budget; cumulative p99 upper bound was85%. Its measured dense window had0deadline overruns. The warmed continuous repeat had0new overruns, a longest near-zero run of0frames and no host capture status errors. All recorded output was finite and below clipping.

The first newly opened recorder window contained near-zero regions; exact positions are preserved in the JSON. These cannot by themselves distinguish host/USB startup from a device output gap, so a warmed continuous stream was checked separately. Separate restart runs have also recorded an isolated deadline crossing before the warmed window. These results do not certify worst-case load or a universal absence of startup glitches.32-note heavy-FX operation has limited headroom; no DSP capacity was silently reduced.

## Installation and remaining acceptance

The normal build is running, the original project/settings are restored, and20SD boot files were read back against the runtime hash;20prior files were preserved. No firmware was flashed. Physical SHIFT/pad/DATA/fader transport, analog jacks, hot-plug and latest cold power-cycle remain separate acceptance checks. USB remains one stereo pair.

Machine-readable results: [validation-0.6.1.json](validation-0.6.1.json).
