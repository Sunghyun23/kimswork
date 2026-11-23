# Autonomous QA — 2026-11-22 (current status)

This record supersedes earlier preparation/pending statements in QA1–QA4 documents. Loop Club is a local development Rust preload mod for MPC Sample firmware1.3.0.12, vendor kim. It is not a flashed firmware or publicly released product. The owner authorized autonomous QA/fixes/installation while preserving existing work.

## Final identity and installation

- Runtime and SD library SHA256: `79e18f884abf09ef8dcf1e5afeba4e265480fec8da28985c479f0628cd4122f5`.
- SDK v0.7.3 commit `c7d54aaf77a8848437b7e89bd191c0f72689ac5e`. Canonical checkout unchanged. The SHA-pinned independent-controls patch `421cfdc743501b87d87355f1c859be38084e568dddc7b928f7851965fa48ad85` is applied only to temporary build staging; see platform development instructions.
- Direct Loop Club autorun; SampleBoot/menu/start question removed at owner request. Existing original boot and private banks remain preserved.
- Final persistent update: 20 files including metadata verified; all 20 previous boot files preserved at `/media/MPC SD Card/.LoopClubBackups/remote-2572-1795343019780499330`. Both SD and running library hashes match.
- The earlier directQA4 hardware boot and app-only update/restart have actual evidence. The final updated SD files are read-back verified; a hardware power-cycle of this final candidate has not been performed. Firmware and internal persistent services/settings were not changed.

## Fixed bugs and usability

1. Owned buttons/CC/encoder could fall through to stock when the SDK Mod mutex was busy. Independent hooks now enqueue bounded primitive controls without that mutex. On-device injection inside paint (mutex held) consumed40SEQ presses and returned correctly with LOOP74. This exercises the dispatch callback, not a physical finger press. Real physical ownership remains a separate acceptance check.
2. Hidden macOS `._` bank entries caused invalid-bank failures. Bank and WAV browsers filter metadata; a real private bank/kit loaded successfully without modifying its source.
3. CHOP B2 mode/B3 apply/reset fell through to generic navigation. All four modes, apply, gate switch, resampling and slice playback now pass actual controller tests.
4. Recording flags were overwritten by diagnostic counters; counters now use separate snapshot words. Pending recording state remains visible.
5. USB output missed generated audio. A bounded single-render fanout now feeds the native I2S and USB streams without advancing the engine twice. Actual USB audio is nonzero, finite and captured from the explicitly named MPC Sample Audio interface, never the Mac microphone.
6. Analog capture wrongly interleaved raw USB-input blocks with ADC blocks, overflowing its queue. Analog capture excludes the measured USB input stream; final recording commits frames with no queue loss. USB input recording is currently unsupported; controlled mic/line routing/latency remain unverified.
7. Notices were hidden by empty-track hints. Notices now take priority; empty-loop source selection is discoverable. Shared text/control labels are larger and K1–K3 parameter cards align with their values. Actual320×240 DRM pages were inspected; no host rendering is presented as a device screenshot.
8. Heavy64voice synthesis plus effects overran the callback. Phase evaluation, stationary envelopes/LFOs, silent mixing and bypass paths were optimized. Caching A/B compared576000samples exactly (before voice-cap adjustment). Shared polyphony is now32voices with16tracks retained. The prior32voice stress had an isolated108.49% scheduling-inclusive peak; the final post-patch run measured p99≤70.0% and maximum71.80%. These short tests do not certify all long-session workloads.

## Verification and scope

- Native product tests:116passed. SDK patched-stage tests:129passed; SDK format/strictClippy/ARM64 checks passed, canonical source clean.
- Actual device matrix:10/10 PASS — navigation, synth, recording modes, drums, count-in, analog frame capture, chopping, mixes, simultaneous effects, isolated project save/load.
- New QA run `autonomous-20261122-02` was verified empty before testing. No existing track was erased. Save/load waited for actual worker completion rather than an assumed fixed delay.
- Final stress sends16five-note chords (80requested notes,32shared voice limit) with2track/2master effects and8seconds of actual USB capture. Peak0.239668, RMS0.035189, finite samples, no PortAudio status errors. This is a development measurement, not a calibrated output level or unlimited voice claim.
- Full state, actual screen PNGs and audio remain in ignored `build/qa-device/qa5-final-regression/`, `qa5-final-stress/` and `qa5-independent-controls/`. They may contain private bank names and must not be published automatically.
- Machine-readable current summary: [QA JSON](qa-autonomous-2026-11-22.json).

## Remaining acceptance and recovery

Physical SEQ/pad/fader/encoder input, calibrated analog test-tone/latency, the final hardware power-cycle, extended load/thermal behavior and physical recovery remain unverified. MIDI bank import, full GRID editing/event paging, record Undo, time stretching and USB input recording are not implemented. Do not mark every hardware feature passed from controller injection.

Future updates use USB stage→SHA check→app-only restart→engine/DRM readiness. Explicit `qa_manage.py --persist` can then update only recognized own direct boot, preserving the entire old tree. It does not export SD or power-reset hardware. The current persisted development build uses its isolated QA session path. Public release, push and site publication have not been performed.

One initial test request containing bulk ERASE was rejected by automatic approval review because isolated owner-work preservation was not established. It did not execute. Tests were replaced with a verified new QA directory and fresh empty-track assertions; no rejected erase action was retried.
