# Character bus effect — requirements and proposed design

Historical proposal. Current implementation is **Crate Press 0.2.2 public preview**, vendor **kim**, UID K36Z. See ../../plugins/crate-press/spec.md, manual.md and validation.md for the approved design, implemented behavior and exact partial hardware verification scope.

## Confirmed user requirements

- A new Akai MPC audio effect for J Dilla / neo-soul / lo-fi / DIBIA$E-oriented production.
- Refer to the public Vulf Compressor concept and functionality.
- Use on both individual tracks and the master bus.
- Include actual band compression with ducking usable on the master.
- Keep operation simple.

No claim of recreating a particular artist's recordings or Vulf Compressor's proprietary algorithm is intended. This is an independent implementation. Public reference: https://goodhertz.com/vulf-comp/manual/ (reviewed2026-11-16). The reference combines compression, wow/flutter, lo-fi and overall mix; advanced parameters are available separately.

## Proposed sound and processing

Stereo2in/2out track or master insert; no MIDI-dependent triggers. Input gain → character compressor → actual three-band dynamics/duck stage → saturation/degradation → wow/flutter → tone cuts → aligned dry/wet and output. Final order is subject to audio comparisons, not a claim about Vulf's internal topology.

- Character compression: a small number of macro controls, program-dependent recovery, punch/transient preservation and restrained automatic makeup.
- Three-band dynamics: low/mid/high are real audio bands with independent compression amounts, not EQ gain and not merely a filtered detector. Proposed LR4 splits at approximately160Hz/3kHz with phase-correct recombination; verify response before choosing defaults. Crossover edits remain advanced.
- Duck: low-frequency detector controls selected target bands. Default mid+high attenuation preserves the kick/sub while momentarily making room elsewhere. Target chips LOW/MID/HIGH and detector/source choices sit next to the band display. A low-band detector can also react to bass, so it must not be described as guaranteed isolated kick detection.
- Tempo duck: use host BPM/beat position for a repeatable pulse; effects receive no MPC MIDI. Rate/division and target chips are touch settings. Free release and tempo-synced recovery behavior must be made explicit in the eventual UI.
- External kick-track sidechain: requested meaning is awaiting clarification. Pinned SDK exposes generic input channels but no documented auxiliary-bus selection; this is not proof that MPC can route an external detector to this plugin. Do not label unsupported external routing as implemented, and do not replace an explicit external-sidechain requirement with internal ducking without agreement.
- Saturation: soft/warm to rough transient grit. Lo-fi: sampler-style rate/resolution coloration; independent noise defaults off. Drift combines restrained wow/flutter on MAIN, with their balance/speed in advanced controls.
- Stereo-linked detector is the master-safe default. Preserve low-frequency image; stereo motion/width remains optional rather than automatically widened.

## Proposed MAIN / 16 Q-Link map

The flow reads input/dynamics → bands → color → final tone/mix/output. All controls below are proposals pending design review.

| Group | Q1 | Q2 | Q3 | Q4 |
|---|---|---|---|---|
| A COMP / DRIVE | Drive | Compress | Punch | Release |
| B BAND / DUCK | Low Comp | Mid Comp | High Comp | Duck |
| C COLOR | Tone | High Cut | Lo-Fi | Drift |
| D I/O | Input | Low Cut | Mix | Output |

Input and Output meters and three per-band gain-reduction meters should be visible. Band targets, duck source, tempo division, stereo link, noise on/off and effect-stage bypass are touch choices. Threshold, ratio, attack, detailed wow/flutter, crossover and detector tuning stay in advanced panels. Main uses the existing MPC usable area rather than consuming the host header/function keys.

Internal parameter0 is harmless output/master gain; internal index order is independent of Q-Link order. UID and schema are established only after identity confirmation. Output/mix/bypass should permit reliable level-matched comparison, not turn a preset louder merely to imply improvement.

## Preset plan

Init (clean), Track Drums (punch/pump), Track Keys (gentle warmth/drift), Track Samples (rougher sampler texture), Master (low-level glue/band duck), Performance (strong pumping/cuts). Artist references are sound-design directions, not endorsements. Master presets start with restrained compression, linked stereo, noise/drift off and low-cut off or minimal.

## Engineering verification gates

- In-place stereo input handling, missing inputs,32–512frame blocks, multiple sample rates, nonfinite guards and bounded output.
- No allocation/locks/files/logging in process; smoothed controls and bounded state.
- Measure crossover reconstruction, band isolation and target-only ducking. Check that bass is preserved when only mid/high are targeted.
- Detector response and steady bass vs kick behavior; host-tempo/seek/stop behavior; disabled-stage and mix0 behavior.
- Delay-line modulation, fixed-latency dry alignment, bypass transitions, stereo/mono compatibility. SDK has latencySamples/setLatency; actual MPC compensation needs device verification. Do not promise zero latency or phase-neutral parallel wow.
- Native/ARM32/ARM64 builds, SDK validator, host simulator, level-matched original-loop renders, real track and master insert testing on MPC Live III3.10.1.12.
- Confirm actual external routing support if selected; otherwise report the gap clearly before claiming that feature.

The SDK remains the single unmodified pinned checkout. Existing Donut Theory/Groove Dice and their releases are unaffected by this proposal.

## User correction — I/O grouping

Input, Mix and Output must be grouped together. MAIN D becomes Input/Low Cut/Mix/Output. Drive moves into A beside compression; High Cut moves into C beside Tone. All16 existing front controls are retained. Parameter IDs/internal order are unchanged; only UI/Q-Link order changes. Design v2 preserves the approved material direction and band/duck display, pending visual review.


## Implemented development direction — 2026-11-16

See`../../plugins/crate-press/spec.md` for confirmed groups,32programs and permanent MAIN actual-audio monitor. Visiblekim branding is removed; vendor metadata retained. Prototype builds/tests are distinct from pending hardware verification and public release.
