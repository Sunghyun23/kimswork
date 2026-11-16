# Verification — 2026-11-16

Crate Press 0.2.2 public preview. Automated verification passed; hardware verification is partial.

| Check | Result |
|---|---|
| Native/ARM32/ARM64 build | Passed |
| SDK package validator |0errors /0warnings |
| Host simulator44.1kHz/128 |20PASS /0WARN /0FAIL |
| Host simulator44.1kHz/32 |20PASS /0WARN /0FAIL |
| Host simulator48kHz/512 |20PASS /0WARN /0FAIL |
|32programs ×44.1/48/96kHz | Audible, finite, peak bounded with factory settings |
| Signed stereo waveform | Measured both channels; opposite polarity retained |
| Bypass impulse | Measured latency matches reported6ms sample count |
| Static transfer curves | All48points ×4views match shared DSP reduction function |
| State restore | All33sound settings restore, each program/rate |
| Automation/input edge cases | Variable31/128/511frame blocks, sweeps, mono, aliased I/O, missing input passed |
| RT allocation counter |0allocations in process |
| Undefined Behavior Sanitizer | Passed |
| Skin regression |16Screen/Program assignments,4guide bounds, alloverlay hide flags,256real waveform handles, runtime-only assets passed |

Reproduce with`python3 mpc-software/scripts/build_plugin.py CratePress --target all` then`python3 mpc-software/plugins/public/CratePress/tools/test.py`. Build logs,32synthetic-inputWAVs, per-preset peak/RMS CSV, measured UI snapshot and machine-readablevalidation.json are local ignoredbuild/outputs. Noise/drift are disabled in master presets. No subjective listening evaluation is claimed.

Monitor updates30Hz. After reset/state restore, the next telemetry snapshot arrives within~33ms; transient display values are not a sound-setting restoration failure. Waveform is64time bins of signed extrema, not a sample-resolved scope. Curve is static pre-makeup; it does not claim to depict all color/duck/output processing. Host peak level values are dBFS, GR is positive dB reduction.

Crate Press0.2.0 installed on MPC drive36C7-1ACA using the officialSDKtool.49files verified. The previousGroove Dice QA state was observed unchanged. An initial automatic approval review rejected restart because current-session unsaved changes/approval were not explicit; no bypass was attempted. The user then explicitly approved discarding current changes and restarting, and SDKrestart succeeded. Subsequently 0.2.2 was installed, loaded on master Out 1/2, saved to the dedicated cratepressqa project and recalled after restart. MAIN numeric values, compact typography, tab navigation and two independent LATCH states (Reverse+Repeat; release Reverse leaves Repeat; Latch Off releases all) were observed. This verifies UI states, not hardware sound or physical multitouch.

Designv4removes the visiblekim badge; manufacturerkim andUIDK36Z remain. Runtime layout proof`build/main-runtime-preview-screen.png` comes from actualTUI and engine-measured values, with a simulated host frame; it is NOT a hardware screenshot. No Release, push or website publication performed.

## Eight-effect performance verification

- Every effect changes synthetic stereo input; all-off is bit-exact through the performance module.
-28pairs ×44.1/48/96kHz =84combination runs. Each pair differs measurably from BOTH solo effects; after releasing one, the remaining effect remains audible. Peak output finite/bounded at tested settings.
- All8held, staggered releases, zero-input tail decay, variable block partitioning and zero RT allocations passed with Undefined Behavior Sanitizer.
- Repeat matches the chosen1/8period at120BPM; captured loops remain frozen across10seconds. Echo tail persists afterrelease then decays. Sound preset selection preserves performance setup. Restored state disarms held gates.
- A discovered interaction where Repeat masked Tape Stop/Reverse was corrected: Repeat uses reverse read direction whenReverse is held; Tape Stop changes speed and fade for both captured loop engines. Continuous macros use20ms smoothing.
- Reproduce via tools/test.py. Details:build/combinations.csv (84rows including delta vs either solo),performance.csv,combinations-test.log. Previewmain-performance-preview-screen.png uses actual DSP parameter values with REVERSE+REPEAT both held; it is an offline layout render, not a device screenshot.
- Physical two-finger multitouch is NOT synthesized by the remote mouse. Actual independent touch tracking must be checked on the MPC before claiming simultaneous touch performance.

## 0.2.2 device corrections

- Main nested value labels no longer self-remap Data to Data. Values were blank before this fix and visible after SDK update/restart.
- Unbound SDK detail switch captions marked Hide were blank on the actual host. Captions are baked into each button image; no caption Label/Image sibling covers the hit surface. Focus-only parent actions were removed from those choice widgets. Parameter IDs remain unchanged.
- Typography uses the approved reference with fractional stroke reduction. Section ink height18px; badge38×20.
- Automated logs: /tmp/crate-release-build.log and /tmp/crate-release-test.log; build/validation.json is an offline test report, not a device certification.
- A quiet built-in noise source was tried through two master insert instances. It does not establish musical audio audition; silent-chain suspension and very low waveform amplitude limit that check. No subjective hardware audio claim is made.
- Remaining: physical Q-Link rotations, finger hold/multi-touch, track-insert audio, full hardware audio/latency compensation and worst-case CPU. Public download is explicitly a preview.

- A host-cache bug was reproduced: native PRESET name changed but MAIN values remained Init. The plugin-local dispatcher now queues program/state refresh and engine telemetry; non-render value reads/idle drain host notifications. The shared SDK is unchanged. A dedicated UBSan bridge regression proves program cache refresh and live telemetry while rejecting callbacks from render-thread reads. Actual MPC Pocket Press then showed Drive18/Compress55/Punch76/Release140 with its corresponding curve.

- Final actual-device choice regression: PERFORM Reverse1BAR and Echo1/4 dotted changed via caption-center remote clicks; DUCK Tempo and COLOR Crunch changed similarly. MORE loaded. Double-tap Compress opened its value overlay without the underlying Q-Link guide; outside tap restored MAIN. Final LATCH Reverse+Repeat UI states and all-off recovery passed. This does not certify physical finger multitouch or musical audio audition.

## Final artifact SHA256

- `src/CratePress.cpp`: `fb1a285b89f7171ea33e0d49a51fd31e5968e6c82bc617ac6be0a4c414d3a424`
- `src/Performance.h`: `fb419e3e4520cba9d16194f2655309ed086aa31bf82327a889ea6aa9f2e3eb26`
- `build/native/crate-press.dylib`: `40a7f31d42cd609f9513490274c41ad8995a43a4ea7ed6f0fb15e0610e06c82b`
- `build/kim - VST - Crate Press/VST/crate-press-arm32.so`: `cfe5c8637a182b2eff054d4e625fe291e8e6e7210b985fb35033f69f86056fd9`
- `build/kim - VST - Crate Press/VST/crate-press-arm64.so`: `7f072ea20ea238ee228777f2887288dc6068894ed00ab603046a074bdf6eb624`
- `skins/TUI.json`: `4c11be336ee584b07df65df705429ad0c54b52c39dbd709bd972731d2b9da4d6`
- `skins/Q-Links.json`: `396651c2119c2426fa8fc2c8c7420638a1ab3af16c445a92609d26b52105751c`
