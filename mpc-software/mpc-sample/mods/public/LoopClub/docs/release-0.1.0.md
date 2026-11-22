# Loop Club 0.1.0 — public preview

16-track looper for MPC Sample firmware1.3.0.12, by kim. Not an MPC OS VST or flashed firmware.

## Included

Independent audio/synth/drum loops; Replace/Overdub and count-in; musical generated velocity accents; two drum layers; shared PAN and LFO Pan; two simultaneous track inserts; five independently enabled master texture stages; drum/whole-track sidechain with explicit targets; saved stereo output destinations. No commercial samples, personal packs, QA management agent or firmware is in the ZIP.

## Verification

Direct4×4FXboards support tap ON/OFF, simultaneous state display, free-slot allocation and refusing a third track effect without replacement. Master bypass retains effect choice/parameters across save/load.


- 144 native tests, strict Clippy, ARM64 build and imported glibc2.30 check pass.
- 8 launcher fixtures, 4 direct-install preservation/rollback tests and 7 other installer tests pass.
- 10 actual remote-device matrix cases pass: navigation, synth, recording, drums, count-in, analog frame capture, chop, mix, effects and projects.
- Actual USB recordings confirm left/right auto-pan and kick-driven attenuation. Two track inserts and five texture stages remain enabled together. Output stays finite, without PortAudio status errors.
- The exact public binary was staged, restarted and checked on the device, including the initial MAIN view and the SENSE100% boundary. Its SHA-256 matches the ZIP library.
- Website screenshots are actual active-DRM320×240captures from a fresh session using original procedural sounds. No private bank was loaded in that screenshot session.

## Remaining limits

This is a preview. The final 32-voice /two-track-insert/two-master-performance/five-texture/split-output run verified every effect ON: p99<=90% and maximum 127.01% of the callback deadline. The final audio/UI run recorded maximum 127.01% with p99<=35%. These include scheduler interruptions; occasional deadline overruns remain unresolved, and the heaviest combination has little headroom. They are not a blanket CPU or long-session certification. Earlier stress records whose FX had already switched off are not used to certify this workload.

Physical finger controls, calibrated analog latency, latest power-cycle boot/recovery and long-duration reliability remain unverified. MAIN output evidence is engine telemetry rather than a separately recorded analog jack. USB exposes one stereo pair; individual drum USB stems are not implemented. No time stretching, raw USB input recording or source-bank MIDI import. 32 voices shared; up to4096 MIDI events/track, first 64 visible in GRID. User sample banks are separate optional files.

## Install

Read [the installation guide](public-install.txt), preserve your existing autorun and save your work first. Direct boot and SampleBoot relocation pass host fixtures; the latest complete power-cycle acceptance is still outstanding. Existing projects and expansions stay outside autorun and are not in the release ZIP. The owner development device retains its opt-in QA environment separately from the public package.

## Checksums

ZIP SHA-256: `ed826ef9fd5039ea05c432931c5b0cfb6f5b22280dd419cff224263e216ae605`

Library SHA-256: `ce0e6cf9673da144e1c288485ddddeaa40cc0232441b9d928a9f9c604de95e1c`

[Machine-readable summary](release-0.1.0.json) · [Manual](manual.md) · [MIT source license](../LICENSE)
