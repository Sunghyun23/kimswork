# Loop Club

A16-track audio/MIDI looper for **MPC Sample**, by **kim**. Public preview0.11.1 targets firmware1.3.0.12. This is an SD preload mod, not an MPC OS VST or a firmware image.

[Manual](docs/manual.md) · [Installation](docs/public-install.txt) · [Verification](docs/release-0.11.1.md) · [Source specification](../../../docs/looper-srs.md)

- LOOP/MUTE/CHAIN pad modes; record from LOOP without changing the screen; REC ends at the bar boundary.
- Independent bar-aligned audio, synth and drum loops; count-in; Replace/Overdub; MIDI grid.
- Two-oscillator synth, separate amp/filter ADSRs, six-shape LFO to Pitch/Filter/Volume/Pan.
- Original procedural drums, musical velocity accents and optional standalone user banks.
- Two layers per drum sound: sample selection, Volume/Tune/relative Shift, Decay/enable/REV.
- Pad-shaped track/drum mixers, pickup fader and per-track MAIN/USB stereo destinations.
- Direct4×4FXpad boards with tap ON/OFF: two simultaneous track slots, five master texture stages, Duck and two saved custom master effects.
- Sidechain from an individual drum or whole track, to other/custom tracks or the full master.

Native checks, real active-DRM screens, USB audio and remote-control tests are recorded separately from unverified physical inputs. The32voice cap is not an unlimited CPU guarantee; brief scheduling-inclusive peaks have exceeded the callback budget. USB multichannel drum stems and source-bank MIDI import are not implemented. Local0.7.0 adds bounded sample Warp; its limitations are documented below. See the manual for exact limits.

## Build

From the repository root:

```sh
python3 mpc-software/scripts/bootstrap_sample_sdk.py
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/run.py test -- --test-threads=1
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/run.py clippy --all-targets -- -D warnings
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/build.py --pack
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/package.py --public --direct
```

Use the pinned SDK and repository-local toolchain. A SHA-pinned input dispatch patch is applied only to temporary staging; the canonical SDK stays clean. See [development](../../../docs/development.md). QA upload/agent tools are opt-in development utilities, not part of the public ZIP. Runtime-only updates disappear at power-off until explicitly persisted.

Original source is [MIT](LICENSE); the SDK retains its own license. No firmware, commercial WAV/MIDI, personal expansion, owner project, or management agent is included in the public download. Existing card boot files must be backed up before installation; projects and expansions remain outside autorun.

Local0.1.1 tempo revision: new projects start at100BPM. Press SHIFT+DATA to open PROJECT, then use K1 for tempo or DATA for1BPM steps (40–240). Stop playback first. Existing audio loops lock tempo because audio time stretching is not implemented; stopped synth/drum MIDI loops retain their musical event positions when tempo changes. Saved projects retain their stored tempo.

Current local candidate0.3.0 restores all16active loop tracks, keeps LOOP visible during capture, switches pads to instrument performance and uses REC to finish at the bar boundary. SHIFT+B1/B2/B3select LOOP/MUTE/CHAIN directly; SOURCE/SOUND/REPLACE–OVERDUB remain fixed without SHIFT. CHAIN DATA push starts/stops only its member loops; background tracks continue. Existing16-slot project data is retained. Public0.1.0 downloads remain unchanged until a separately requested release.

Drum SOUND and PLAY: SHIFT+instrument pad generates its lane; SHIFT+ERASE+the samepad undoes the most recent generation. Generation is disabled during capture and replaces a previously recorded lane deliberately, retaining other lanes. Smooth chord voicing defaults ON; CHORD SHIFT+B2 toggles it. LFO/PAN SHIFT+B3 toggles BPM Sync; Rate selects straight/dotted/triplet divisions when enabled. SHIFT+SAMPLE opens LINE/MIC/USBrecord input selection. Input switching requires stopped transport; MIC mutes the built-in speaker, restoring its earlier state on leaving MIC. Actual USB input recording is verified on the pinned Sample geometry.


Local 0.3.1 FX revision: effect-only pad boards. Ordinary press is momentary HOLD; SHIFT+press toggles per-effect LATCH. Two simultaneous track insert slots remain; extra insert reports FULL without replacing an active effect. Actual USB/audio, remote-input and DRM screen evidence is in docs/validation-0.3.1.json. Public download unchanged.


Local 0.4.0 adds Tremolo/Auto Pan/Auto Filter, Wobble Sync/shape and DATA-entry FX settings. SOUND keeps pitch/filter LFO; saved legacy volume/pan values remain editable through a conditional compatibility page. Existing two-insert capacities remain. 203 Debug + 203 Release tests, strict host/ARM64 Clippy, actual remote controls/USB audio, saved master LATCH and SD/runtime SHA readback passed. See docs/validation-0.4.0.json for evidence and limits. New saves use project format4; older formats remain readable, but older apps cannot read format4. Public0.1.0 unchanged.


Local 0.5.0 adds physical SHIFT+− UNDO / SHIFT++ REDO for one shared last clip edit: completed recording, pattern generation, note/chop editing and clear. Plain +/− retain their previous roles. History is volatile; sound/mix/FX parameters are outside its scope. See the manual and validation-0.5.0.json. Public download remains 0.1.0.


Local0.6.0 makes normal LOOP pads always perform. SHIFT+pad starts/finishes recording; SHIFT+another pad queues a next-bar handoff without extra count-in. The current sound stays active until that boundary.225 Debug+225 Release tests, strict host/ARM64 Clippy, actual remote Sample/MIDI/USB audio and runtime/SD readback passed; the original16track owner project was restored. Physical SHIFT/pad/REC and latest power-cycle remain separate. See docs/validation-0.6.0.json. Public0.1.0 unchanged.


Local0.6.1 completes a detailed QA pass and fixes Audio cancel metadata/playback, held count-in, external MIDI release/selection, saved chord mode and two misleading UI hints.240 Debug+240 Release tests, strict host/ARM64 Clippy, real USB/DRM checks and runtime/SD readback passed. See [detailed QA](docs/qa-0.6.1.md) and its explicit startup/physical-test limits. Public0.1.0 unchanged.


Local0.7.0 adds explicit Audio REC: INPUT / CHROMATIC / CHOP, existing WAV import, held chromatic/chop looping, source-preserving performance MIDI, Root/Tune/Warp and format5 recall. Choose the target in LOOP→SOURCE (pads5/6/7) or loaded Audio SOUND (B1). SHIFT+same track pad starts/ends recording. Public0.1.0 downloads are unchanged. See the latest manual section for the complete recording workflow and Warp/voice/compatibility limits.

0.7.0 local validation and runtime/SD installation are complete; see [Audio performance QA](docs/qa-0.7.0.md) and [machine-readable evidence](docs/validation-0.7.0.json). Actual USB/DRM/MIDI checks and original16track restoration are distinct from remaining physical/analog/power-cycle and arbitrary musical-Warp acceptance.


Local0.8.0 adds Drum XPM file→bank→16pads import through SOURCE pad5 or SHIFT+BROWSE. Up to four original sample zones/velocity modes are retained under L1; missing data keeps the previous kit. Project6 embeds imported audio. Native MPC FX/Warp are not reproduced. See [manual](docs/manual.md), [QA](docs/qa-0.8.0.md) and [validation](docs/validation-0.8.0.json). Public0.1.0 downloads remain unchanged.

Local0.9.0 adds in-place LOOP editing: stopped **ERASE/COPY** for DELETE or **SHIFT+ERASE/COPY** for COPY, then B1 COPY / B2 DELETE / B3 MOVE. Copy/move target an empty slot and preserve the full source/kit/sound configuration; deletion confirms with the same pad and retains the instrument. All support latest-edit Undo/Redo. The detailed follow-up also fixes loading saved projects after changing the SOURCE of an imported XPM kit. See [manual](docs/manual.md) and [QA](docs/qa-0.9.0.md). Public0.1.0 remains unchanged.


Latest public preview0.9.3 supersedes the historical local-candidate/public0.1.0 status notes above. Includes Audio/XPM/loop edits, format7 phase-preserving recall/copy, and immediate bottom K1/K2/K3 controls when pressing FX pads.293 Debug/293 Release, strict host/ARM64 Clippy and actual39FX contexts plus phase/copy recall passed. See docs/release-0.9.3.md/json for hashes and remaining limits. Public video source banks are not included in the ZIP.


Local0.10.0 adds audio chords, Synth Poly/Mono/Legato and Glide, timed Strum recorded as expanded MIDI, and Gate/One Shot for all source types. LOOP SHIFT+K1 changes pad-performance octave±2 without changing already-recorded notes or CHOP indices. Source PCM remains intact. PLAY SETTINGS is SOUND→SHIFT+B1.318Debug/318Release, strict host/ARM64, actual remote/CoreMIDI/USB and own-SD readbacks pass; owner tracks restored. Public download remains0.9.3. See [local performance guide](docs/performance-0.10.0.md) and [verification](docs/validation-0.10.0.json). Turntable was a design proposal at that historical0.10.0 checkpoint.

## Local0.11.0 candidate

[Turntable and whole-project MIDI Swing](docs/turntable-swing-0.11.0.md) extends the local0.10 performance controls. Public release/site remain0.9.3. Device verification is recorded in validation-0.11.0.json; follow its precise scope rather than historical local-version claims.

## Local0.11.1 detailed audit

[Detailed QA and fixes](docs/qa-0.11.1.md) covers cross-mode recording, playback ownership, Swing metadata and capacity handling. Public downloads remain0.9.3. See validation-0.11.1.json for the final tested build/device/SD identity.

Publication0.11.1 supersedes historical local-only status notes above: Turntable/whole-project MIDI Swing/performance controls and the detailed audit are included in the public preview. Scratch Pocket is a new actual-device video; earlier full-session/FX videos remain available at their existing0.9.3 URLs.

Local0.11.3 adds Drum/CHOP MIDI-lane Erase/Copy, explicit Done/toggle exit, transport-preserving generation and optional Turntable DOWN CUT with format10 direction recall. Public release remains0.11.2 until separately published. Verification is recorded in docs/qa-0.11.3.md when complete.

Local0.11.4 adds physical Tap Tempo and SHIFT+Tap monitor metronome, explicit four-click count-in and non-printing monitor routing. Native/build and actual USB/display results are in docs/validation-0.11.4.json. Final normal binary is installed in runtime and verified SD autorun; public release remains0.11.2.

Local0.11.5 optimizes nonblocking hardware input dispatch with lossless busy fallback. Actual30trial remote button-state median145→82ms/frame observation202→139ms; USB MIDI onset estimate remains about5ms. DATA remote observation unchanged. See [latency verification](docs/qa-0.11.5.md) for scope, lifetime startup overrun and physical measurement limits. Public0.11.2 unchanged.

Local0.11.6 fixes browser±/Shift±leaking into background Setup/Undo/Redo, corrects DATA diagnostic attribution and audits16pages.390distinct Debug cases/11Release input-audit tests/actual all-page DRM and controller checks pass; final normal runtime and SD verified. [Detailed QA and limits](docs/qa-0.11.6.md). Public0.11.2 unchanged.

Local0.12.0 scratch response update: PAD1 toggles GRAB, PAD2 holds source CUT, left fader moves the platter, K3 retains mix Level. Input-timed bounded motion replaces live target chase; format11 retains legacy trajectory playback for old projects. Native401checks/29Release guards, strict Clippy/ARM64 and actual remote DRM/USB pass; normal runtime/SD verified with owner tracks preserved. Physical controls and scratch realism are not certified by these tests. See [verification](docs/qa-0.12.0.md). Public release/site remain0.11.2.

Local0.13.0 refines scratch motion with median input cadence and zero-end-velocity braking; PAD1toggle/PAD2CUT remain unchanged. Format12 preserves old1–11gesture curves and exact float recall.405Debug/31optimizedrealtime+motion tests and strict Clippy pass. A17sactualSample/USBaudio demonstration uses sample-clock programmed7bit input, not physical hand performance. Public release/site remain0.11.2. Installation hashes and limits are recorded separately in docs/validation-0.13.0.json.
