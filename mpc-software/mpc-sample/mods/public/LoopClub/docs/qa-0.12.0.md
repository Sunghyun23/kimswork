# Local scratch response update 0.12.0

PAD1 now toggles GRAB with a plain press. Pad release keeps it engaged; the second press resumes forward playback. PAD2 is momentary source-only CUT. No SHIFT latch remains. STOP/app exit releases GRAB; LOOP SHIFT+pad still records. K3 LEVEL keeps its mix-volume role.

The new live trajectory uses arrival intervals and bounded monotone cubic segments instead of fixed2ms target chase. Duplicate packets retain elapsed time, same-block updates coalesce, and speed stays bounded to4x. Fader/CUT use a scratch-only lock-free command route while the visible page accepts Turntable performance; other pages/browser retain their own control ownership. CUT has a separate3ms DSP fade rather than the mix-level smoother. This is a DSP fade duration, not a measured physical input latency.

Native actual-Player10Hz reproduction with explicit arrival intervals: mean0.900x, near-stationary0.091%,4x-cap0%, compared with the earlier67.46% stationary/20.50%4x result. Initial acceleration is included, and these synthetic trajectories are not a realism/listening certificate. New recordings preserve motion and CUT in format11; formats1–10 load with their original legacy trajectories. Older apps reject11.

401distinct Debug tests pass (400full-suite plus one additional all-page route guard),29optimized realtime/motion tests, strict Clippy, ARM64 normal/QA builds and8launcher checks pass. Actual Sample1.3.0.12 remote controller/DRM verifies toggle retention, second-press release, CUT release across page navigation and STOP release. Actual USB audio peaks: open0.1944, CUT0.0, reopened0.1956; no capture status flags. Private Classic Ahhh demo source stayed in its private/QA location and is excluded from ZIPs.

Normal ZIP library is installed and read back from runtime/SD;20old boot files preserved,20new files verified. All16owner track metadata and BPM compared unchanged, without saving the owner project. See validation-0.12.0.json and design/device-0.12.0. No public Release/site update or new demonstration video is implied. Physical finger/pot acceptance, power-cycle, analog and maximum multi-deck CPU remain separate.
