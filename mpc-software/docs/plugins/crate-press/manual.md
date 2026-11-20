# Crate Press user manual

Version 0.2.6 · Stable release · Stereo effect for MPC standalone.

## Roll Dice and protection — 0.2.6

The same bottom row appears on all five pages: **UNLOCK ALL → ROLL DICE → UNDO**. ROLL DICE generates a restrained starting point for Drive, Compress, Punch, Release, Low/Mid/High Comp, Duck, Tone, High Cut, Lo-Fi, Drift and Low Cut. Tap a control’s padlock to preserve it; its lock is shared with the detail page. UNLOCK ALL clears protection without changing sound. UNDO restores the exact thirteen values from before the last successful roll, even if protection has since changed. Rolling with every control locked does nothing and preserves the previous Undo.

Input, Output, Mix, bypass, Peak Guard, LATCH, active performance effects and their setup are preserved. This makes Dice useful for exploring character while keeping gain and live performance choices in place. Dice also responds while transport is stopped. The first 380 parameter identities remain compatible; three commands and thirteen locks append at 380–395. Project state stores locks, but pending commands and active performance gates do not resume on recall.

The MAIN performance pads sit above the footer; LATCH and curve choices remain on its right. Lock artwork uses the same silhouette and spacing as Donut Theory with a 44×44 touch surface.


## Install and open

Download the Crate Press ZIP from its own release page. Unzip and copy the complete **kim - VST - Crate Press** folder into **VST2 Plugins** at the root of an MPC drive. Keep both ARM binaries and Plugin Skins together. Requires MPC standalone OS 3.10.1 or later. Enable **Enhancements** and **External Plugin Support** in Preferences → Enhancements (Beta), preserve your project and restart the MPC application to rescan.

Add Crate Press to an **insert FX** slot on a track or the master Out 1/2. It is an effect and will not appear as an instrument. Start with Init / Clean, keep output headroom and bring Compress or Drive up gradually. This preview was loaded and recalled on MPC Live III / OS 3.10.1.12 as a master insert. Other models, track-insert audio and full physical control coverage remain unverified.

## MAIN and Q-Links

Use Q-Link **Screen** mode to follow the current page. Program/Track assignments retain the MAIN mapping. The native Q-Link focus badge and outline may cover part of a section badge while that bank is focused.

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A · Comp / Drive | Drive | Compress | Punch | Release |
| B · Band / Duck | Low Comp | Mid Comp | High Comp | Duck |
| C · Color | Tone | High Cut | Lo-Fi | Drift |
| D · I/O / Mix | Input | Low Cut | Mix | Output |

**Drive** adds saturation. **Compress** sets the full-band compression character, combining threshold/ratio behavior. **Punch** shapes attack; **Release** sets recovery in milliseconds. These controls are musical macros, rather than a conventional compressor with independent threshold and ratio knobs.

**Low/Mid/High Comp** apply independent dynamics to the three crossover bands. **Duck** sets selective ducking depth. **Tone** tilts the color, **High Cut** rolls off brightness, **Lo-Fi** adds sampler-style coloration and **Drift** adds slow wow with faster flutter. **Input** is before processing; **Mix** blends the latency-aligned base dry and wet paths; **Output** is the final level. The performance effects follow the base Mix control.

Tap or drag a control to change it; double-tap eligible continuous controls for their value overlay. Native detail-page controls follow the same parameter values. Selection overlays hide the underlying page's Q-Link guide and restore it when closed.

## Waveform, curve and meters

MAIN always shows a measured input/output overview. Each of 64 bins stores signed stereo extrema over 12 ms, covering about 0.77 seconds. Both channels contribute; this is not a sample-resolved oscilloscope or frequency spectrum. Input is before Input gain and latency-aligned to output; Output is after processing, Mix and Output gain. Both use the same linear ±1 height scale. Very quiet signals can look flat at this scale.

Choose **FULL / LOW / MID / HIGH** in the bottom utility row to inspect that compressor's static soft-knee transfer curve. The curve shares the DSP reduction calculation but is **before makeup**, and does not include dynamic attack/release, saturation, ducking or the combined effects chain. The cursor follows detector level. **COMP GR** and the four band/duck meters show reduction in positive dB; input/output peaks are dBFS. Telemetry refreshes around 30 Hz while the host processes audio. A host-suspended silent chain does not provide a continuously running scope.

## DUCK

Choose **Bass** for internal low-frequency sensing or **Tempo** for a pulse derived from host tempo. Division sets the tempo pulse rate. Detector controls the bass detector frequency; Sensitivity sets its response threshold. Enable Duck Low/Mid/High only for the bands you want to move. Low Split and High Split set the crossover frequencies. Keep the low band protected on a master unless its movement is intentional.

This preview has **no external sidechain input** and does not receive audio from another track's dedicated sidechain bus. Internal bass sensing and tempo ducking are available.

## COLOR and MORE

COLOR adds Lo-Fi Type, Wow Rate, Flutter, Noise, Stereo Width, cuts and Peak Guard. Noise is intentional generated texture; master presets leave it off. Wow/flutter are pitch movement rather than oscillator detuning. MORE contains additional mix/output, automatic makeup and bypass controls.

Automatic makeup is restrained compensation, not LUFS matching. Peak Guard softly bounds output peaks; it is **not a certified true-peak limiter**. Keep headroom when stacking effects. The plugin reports a fixed 6 ms latency; offline alignment passes, while hardware compensation is not certified.

## Eight performance buttons

The **LATCH** switch is in the lower utility row, just above the eight performance buttons. With **LATCH off**, press and hold a button, then release it to return. With **LATCH on**, tap each effect independently to toggle it. Multiple effects can remain on. Turning LATCH off releases all eight gates. Dub Echo retains its natural feedback tail.

| Button | Behavior |
|---|---|
| Tape Stop | Slows and fades the current signal or captured playback |
| Reverse | Freezes a recent segment and loops it backward |
| Repeat | Freezes a recent segment and loops it at a tempo division |
| Dub Echo | Sends into a filtered feedback echo; release stops new sends |
| Filter Dip | Applies a low-pass dip |
| Crush | Adds bit and sample-rate reduction |
| Wobble | Modulates a short delay for pitch movement |
| Dropout | Starts a softened cut, then irregular cut/recovery intervals |

Order is Tape Stop → Reverse → Repeat → Filter → Crush → Wobble → Dropout, with echo send/return last. Reverse changes Repeat's read direction when combined. Tape Stop affects the speed/fade of captured Reverse/Repeat playback too. All 28 pairs were tested at three sample rates against both solo effects; this is automated audio verification, not physical multi-touch certification.

Captured history is limited to eight seconds and to audio available since the instance was prepared. Reverse/Repeat length changes apply on the next trigger. Onset is softened over 5 ms; release is adjustable. Stored projects retain setup and LATCH preference, but active effects are deliberately disarmed on load. Gate readbacks clear at the next display tick, around 33 ms.

## PERFORM Q-Links

| Group | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| A | Tape Time | Reverse Length | Repeat Division | Echo Division |
| B | Echo Feedback | Echo Mix | Filter Dip | FX Amount |
| C | Crush Bits | Crush Rate | Wobble Rate | Wobble Depth |
| D | Dropout Rate | Dropout Depth | FX Release | FX Output |

Tape Time is 80–2000 ms. Reverse offers 1/8, 1/4, 1/2 and one bar. Repeat offers 1/4, 1/8, 1/16 and 1/32. Echo offers straight and dotted divisions (the centered dot in a button caption means dotted). Echo Feedback is limited to 80%; FX Release spans 5–200 ms. FX Output trims the performance chain independently. LATCH is a touch control and does not replace a Q-Link.

## Factory presets

The host PRESET field exposes 32 programs: Init (1), Drums (8), Keys (6), Samples (6), Master (8), Performance (3). Names include their category. Category-first browsing in the MPC host is not promised. Init is the dry starting point; master programs disable Noise and Drift by default.

Sound programs change the base processing while preserving PERFORM settings and LATCH preference. A full project save stores both. Preset selection disarms transient performance gates. See the preset guide for every program and intended use.

## Verification and known limits

Native, ARM32 and ARM64 builds pass with zero SDK validator errors/warnings. Host simulator passes at 44.1 kHz / 32 and 128 frames and 48 kHz / 512 frames. UBSan, 32 programs at three sample rates, stereo waveform accuracy, curve consistency, variable block sizes, state restoration, performance release/tails and zero render-time allocations pass.

Actual MPC checks cover master-insert loading, project save/reload, corrected MAIN numeric values, native program refresh, compact titles, all five tabs, caption-center choice touches, value overlays with guides hidden, and independent two-button LATCH states. Physical Q-Link rotations, finger hold/multi-touch, full track/master audio audition, worst-case CPU, other MPC models and hardware latency compensation remain outstanding. No subjective listening evaluation is claimed. Use this public preview for evaluation before relying on it in a live set.

## Credits

Independent project implementation by kim. Built with the unmodified MIT-licensed MPC Plugin Builder SDK at commit febd2ca5bd9eb16c4a412837196e145566bc981b. Concept art and derived runtime lettering/material assets were generated for this project. No private drum bank or third-party compressor code is bundled. This is not an endorsed artist product or a proprietary compressor circuit model.

## 0.2.3 maintenance update

This update refreshes package metadata and installation documentation. DSP, presets, UI, parameter identities and performance behavior are unchanged from 0.2.2. Prefer internal storage: place the complete `kim - VST - Crate Press` folder under `VST2 Plugins`. Keep one installed copy across attached drives, preserve current work, then restart the MPC application. The previously documented partial hardware verification still applies; this update does not claim new physical Q-Link, multi-touch or audio-routing coverage.

## 0.2.4 layout update

LATCH and curve-view selection now sit immediately above the eight performance buttons. Original artwork, reference-derived lettering, 64 px knobs and I/O faders remain; the live monitor and panel spacing are compacted to make room. The MPC-owned preset/save/bypass header stays at the top. There is no new random generator or duplicate local preset browser.

DSP, all 380 parameter identities, 32 programs, detailed pages and Q-Link assignments are unchanged. On MPC XL / OS3.10.1.12, remote checks confirm track-insert loading, curve choices, momentary hold/release, two and all eight latched states, LATCH-off clearing, and PERFORM navigation. These checks do not certify physical encoders, finger multitouch, hardware output listening/recording or worst-case CPU. The release remains a public preview.

## License and preview distribution

Project source, factory programs and original runtime artwork are offered under MIT. The SDK retains its own MIT notice. The ZIP includes LICENSE and Content/Licenses/. Optional Open MPC portable archives provide one build per CPU; the standard SDK ZIP keeps both. Use one active installation across drives.

0.2.5 changes documentation/licensing/distribution only. Engine, parameters, presets and UI are unchanged. Current hardware screenshots remain explicitly labeled0.2.4, and earlier device evidence is historical. Version 0.2.5 was a preview requiring beta/experimental filtering in Open MPC. Version 0.2.6 is now a regular release. New physical Q-Link, multitouch, hardware audio, worst-case CPU and full project recall certification are not claimed.

## Stable release status

0.2.6 was promoted from preview to a regular release with owner approval. Plugin downloads and checksums are unchanged, so existing0.2.6installations need no update. The README bundled at first publication records its initial preview status; this manual and the Release page describe its current stable status. Hardware testing remains scoped to the evidence above.
