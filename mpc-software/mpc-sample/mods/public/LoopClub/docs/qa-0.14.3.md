# Loop Club 0.14.3 — STOP, timing and four-beat preroll

Verified local candidate for MPC Sample1.3.0.12; public release remains0.14.1. This pass does not publish a release or website update.

## Findings and repairs

The owner's current Drum clip was read without mutation: stored note-on times were spread through the loop, rather than collapsed. Native reproduction identified scheduler initialization on resume: cursor0 plus last-phaseMAX dispatched all prior note-ons together at the stopped phase. Initial/resume scheduling now seeks to the current phase; ordinary loop wrap still starts at event0. Deck-control replay retains its separate state-restoration behavior.

Latest owner-approved two-stage STOP finalizes recorded input instead of using capture cancellation and keeps transport/other loops playing. While not recording, STOP stops all playback and rewinds in one press. New/Replace takes end at the last musical bar, not a following empty bar; Overdub retains its old length. MIDI respects attacks, paired melodic release/held notes, and ignores unpaired release or drum release-only extensions. Audio uses a fixed finite peak threshold0.001(-60dBFS). Remaining final-bar audio is padded with zeros in64-frame pieces, with no realtime allocation/free. Input latency is drained against the original capture-stop timestamp independently of continuing playback; previous audio/features and Undo/Redo stay valid. Truly empty/count-in takes retain the earlier clip. The next STOP requests global stop/rewind, deferred only until any audio finalization is safe.

Recording from a stopped partial bar previously waited until the next downbeat before its four clicks. It now seeks to that downbeat immediately. Running alignment remains quantized and is labeled WAIT FOR BAR separately from PRE4. Snapshot adds a transient count-in-origin word; project format12 is unchanged. Stopped MIDI/audio cursors stay visible; zero-position observation resets the detailed grid viewport to its first bar.

USB testing also exposed that a single missing input frame canceled the whole active take. Missing frames remain counted and preserve their zero-filled time slots; only a sustained10ms gap cancels the current take. Finishing STOP padding no longer demands an active input stream. Startup/no-input/rate guards remain.

## Evidence and limits

448 full native tests, strict Clippy and normal/QA ARM64 builds are recorded in the companion JSON. 36 focused optimized realtime/STOP tests passed, including allocation/free tracking. Earlier test failures from obsolete STOP-cancel expectations were updated to distinguish explicit Cancel from STOP commit; the existing latency-tail regression still passes.

Actual isolated device checks passed: four USB click groups from stopped transport (first click0.175s after capture start including0.15s intentional capture lead); native tests separately cover old saved partial stopped clocks; USB MIDI notes remain distinct, last empty bar omitted, resume skips past events, Recording STOP1commits and keeps playing; STOP2stops/rewinds; actual USB input with tone only in bar1 commits exactly1bar/44100frames at240BPM, waveform peak0.13; all-silent USB input creates no clip. Physical controls remain separate from injected independent callback tests. Initial harness failures were corrected for asynchronous command acknowledgement, insufficient margin near bar boundaries and stream priming; captures of those failed trials are not acceptance evidence.

Owner explicitly authorized discarding the new unsaved drum work for this pass. The prior saved Session is restored without saving over it. Exact normal image and persistent-installation evidence are recorded separately from the isolated QA image; the QA and final normal images share the latest two-stage STOP feature source; the exact normal image also passed USB MIDI/audio smoke. Existing projects/private banks and prior SD boot files are retained.

No physical-finger latency, analog input, power-cycle or maximum CPU certification. Two QA callbacks crossed their deadlines including scheduler interruption; CPU is not unlimited. The silence threshold can exclude very quiet wanted tails from length calculation. Brief input-gap tolerance does not certify a dropout-free USB chain. See [validation-0.14.3.json](validation-0.14.3.json).

## Latest installed result

The owner superseded the earlier STOP1-global-stop/STOP2-rewind design. Recording STOP1 now commits only the take and keeps transport/other loops running; playback STOP stops and rewinds in one press, including a second press while audio padding is pending. Native tests and actual queued SDK-button/USB checks were rerun after this change. In the supplemental actual test, Synth and Audio loops plus the newly recorded Drum all remained PLAY after the first STOP, with USB peak0.14364; the next STOP stopped all and returned tick0.

Current normal/runtime/SD SHA-256: `a0664a6b76dcadf45d78f7fe816d78456ea8894e2a8c05a5c334d61082e196ba`.20files read back and20priorfiles preserved. Saved Session16tracks/BPM/input match the baseline; no owner Save. Left on stopped normal LOOP with1bar preroll/metronomeOff. Public remains0.14.1. Project storage and private banks were not redistributed or deleted.

A full debug-symbol cache exhausted local disk during validation. Only this project's regenerable target/debug cache was removed. run.py now defaults to debug-symbol level0 without changing debug assertions, optimization level or tests; explicit Cargo profile environment can restore symbols. All acceptance checks above were subsequently rerun.
