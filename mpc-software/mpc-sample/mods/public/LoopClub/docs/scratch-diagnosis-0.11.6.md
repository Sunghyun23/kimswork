# Why the classic-Ahh scratch demonstration feels unnatural

This audit diagnoses the existing demo and engine; no production DSP/UI code, installed library, SD boot or owner project was changed. The recognized classic source is present in the actual USB recording; sourcePCMpeak0.497 and decoded video peak0.837, with no PortAudio flags. Those delivery checks do not establish musical or turntable realism.

## Confirmed engine response defect

`deck::Player::move_to` maps fader position to an absolute source target. `advance` chases that target with a fixed2ms one-pole and caps motion at±4source frames/output frame. It does not use the time between incoming fader samples. Position steps therefore produce a fast burst followed by an exponential slowdown and stationary hold. This explains the chopped pitch ramps even when the intended hand movement is steady. The final DC blocker makes a stationary head silent, which is appropriate by itself but exposes those artificial pauses.

The reproducible native example calls the exact actualPlayer.sample code with a45%fader ramp over300ms,100BPM/1beat range/44.1kHz. Intended continuous speed is0.9x. With position updates at10Hz, measured near-stationary fraction(|speed|<.01x) is67.46%, and4x cap occupies20.50%of the movement. Mean0.9x looks correct and would conceal the defect; speed stddev1.62x reveals the severe fluctuation. At100Hz, speed still swings from about4x to0.032x between updates (stddev1.10x).7bit quantization changes the result only slightly in this reproduction. None of these frequencies is asserted to be the measured physical MPC fader frequency.

Reproduce: `tools/run.py run --release --example scratch_response_audit`. Exact summaries/first100ms trace: design/validation-scratch-0.11.6/response.json. No recorded soundtrack is replaced by this experiment.

## Confirmed cut-path mismatch

The demo drives K3 trackLevel for cuts. Engine track gain is smoothed with `gain += (target-gain)*0.002`: at44.1kHz it takes26.1ms to remove90%of the old amplitude;41.4%still remains after10ms. The script closes Level and immediately commands a return stroke, so that return can remain audible during the fade. A normal mix level is doing the job of a performance cut. The existing3ms automatic DownCut envelope is separate and was disabled for the bidirectional demo. For comparison, Mixxx explicitly documents its fast-cut crossfader setup for scratching: https://mixxx.org/news/2008-04-28-scratching-with-mixxx-1-6-0/ . This does not prescribe a universal millisecond standard.

## Confirmed demo-control limitations

The script sends isolated fader positions then sleeps100/120/140/180ms, plus HTTP request time. It does not send a continuous hand trajectory. Only the start of the baby phrase is aligned to a bar; later gestures and level cuts accumulate relative sleeps. At100BPM a sixteenth is150ms, so these repetitions are not a deliberately clocked rhythm. Historical exact per-command application timestamps were not recorded, so we cannot assign a measured network-jitter value to the take.

Remote QA knob commands are consumed in on_paint, unlike physical controls that may use the immediate dispatch path. Multiple queued positions can be sent to the audio side together; command density alone cannot establish smooth motion. Six-ish display frames/second also limit visual fluidity, but the native rate reproduction shows that the issue is not merely a low-frame-rate video.

## Correction priorities

1. Timestamp fader positions at arrival; reconstruct an audio-clock trajectory/velocity with bounded interpolation and a defined stop/reversal response. Test steady-speed ripple, low-rate steps, reversal, no-input stop,7bit quantization, playback/seek and realtime safety. Avoid solving it only by increasing the current chase time.
2. Separate the temporary performance cut from stored trackLevel, with a short click-safe gate and control timestamps. Preserve the level balance and other tracks.
3. Build the demo from beat-referenced forward/back trajectories and cut events, using a native scheduled/replay path or measured physical controls. Do not describe sparse HTTP steps as equivalent to a skilled hand performance.

The earlier functional QA checked bounds, direction, grab/release, persistence and control ownership. It did not check continuous velocity or a scratch cut curve, so its passing status did not justify claiming realistic scratching. A new video before these issues are addressed would repeat the same limitation.
