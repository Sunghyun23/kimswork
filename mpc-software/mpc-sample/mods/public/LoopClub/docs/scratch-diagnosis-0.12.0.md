# Remaining scratch problems after the0.12.0 demonstration

The owner rejected the25.913s remote demo as unnatural. Toggle/CUT/USB validity does not establish scratch realism. No new engine/device/SD change was made during this diagnosis.

## Input timing remains uneven

The host attempted20ms position updates. Measured request-midpoint interval estimates: median19.97ms, P9538.09ms, max82.88ms;48intervals below5ms and38above40ms. These are host transport estimates, not exact device input or physical fader timestamps. Absolute scheduling avoids accumulated sleep drift, but late HTTP points still arrive in bursts. The fast input route bypasses paint dispatch; it cannot remove network/server/capture jitter before arrival.

## The interpolation is still sensitive to packet gaps and quantization

Motion::position clamps at its endpoint. Once elapsed reaches duration, further samples hold the same head until another target arrives. The duration is based on the previous arrival interval, which cannot predict the next gap. A packet gap thus creates an unintended stop; a burst then changes the computed secant speed. Even nominal timing can incur a one-frame gap and zero the carried speed, restarting a ramp.

The new examples/scratch_demo_audit.rs replays the exact logged commands through the actual Player without outputting audio. It compares nominal unquantized, nominal7bit and estimated-arrival7bit inputs. Baby steady movement is intended1.2x, excluding80ms near reversals. Native speed standard deviations are0.265x /0.381x /0.756x respectively. Under estimated arrivals,12.68%of open-gate samples are near stationary versus0.44%nominal. These are model-replay results, not measured percentages in the recorded USB waveform; clock mapping/device processing are approximations. They expose a test gap: prior ideal-speed and bounds tests did not cover a long7bit real-demo stream with packet jitter.

## The performance script is also not a faithful gesture reference

The baby uses linear triangular displacement and instantaneous intended reversals, rather than a deliberately shaped acceleration/deceleration contour. Phrases mostly repeat the same travel and timing. The chirp-style gate is closed for180ms across the top reversal plus90ms near the return to cue. The LCME TTM guide describes a chirp as a close-then-immediately-open click at the top reversal; the gate schedule is too broad to claim this reference technique. Transformer-style periodic chopping alone also does not establish skilled phrasing.

Reference: https://lcme.uwl.ac.uk/media/tmpfvcmu/ttm-guide-may-2021.pdf (baby acceleration/deceleration, stab constant-speed gating, chirp click). Studio Scratches comparative technique demo: https://studioscratches.com/scratch-techniques-demo-video/ (Baby00:06, Stab03:59, Chirp05:51). The text/notation was reviewed; no claim of having heard an externally downloaded performance or perceptual audio-model comparison is made.

## Required next acceptance criteria

Keep the approved PAD1toggle/PAD2CUT/left-fader layout. First address steady7bit/jittered motion and stop/reversal behavior in isolation, with explicit tradeoffs between smoothing lag and stale-input stopping. A timestamped audio-clock QA schedule must separate engine behavior from HTTP arrival artifacts; it must not be presented as a physical-control latency test. Build a short reference baby and forward-cut phrase with proper cue, smooth reversals and gate timing before another long montage. Do not call bounds, USB peaks or successful video encoding a realism pass.
