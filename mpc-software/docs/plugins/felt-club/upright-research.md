# Upright research and candidate scope


## Upright-only 0.2.11 candidate — 2026-11-21

Following the owner's move from Tine/Wurli to Upright, change only the independent acoustic upright backend. Add velocity-dependent unilateral dissipative hammer contact (loss .035 + .055 × Hammer). Split the existing unison resonators into prompt (.72 × existing T60) and longer aftersound (1.16 × T60) loss branches. This is an empirical reduced normal-mode approximation; the strings are not a new reciprocal bridge solver and the ratios are not measured upright material parameters. Preserve existing stiffness, pitch, register balance, modal count, profile and user ADSR/Tone.

Fix the pedal body time laws: 11.33 ms opening smoother, 3 ms damper engagement and 32 ms additional closed-string damping T60 are expressed in seconds, replacing sample-count constants. The 8 soundboard/24 sympathetic resonators remain an original low-order, uncalibrated radiation approximation. No commercial recording or impulse response is included. These changes do not establish a Keyscape/Lounge Lizard equivalent or listening approval.

Sources reviewed: Bank & Sujbert, [On the Nonlinear Commuted Synthesis of the Piano (DAFx 2002)](https://www.dafx.de/papers/DAFX02_Bank_Sujbert_piano_synthesis.pdf), nonlinear hammer excitation and beating/two-stage decay; Zambon, Lehtonen & Bank, [Simulation of Piano Sustain-Pedal Effect by Parallel Second-Order Filters (DAFx 2008)](https://dafx.de/paper-archive/2008/papers/dafx08_37.pdf), independent body/sympathetic filter paths with modal losses as a reduced approximation. Neither paper supplies the numeric voicing chosen here; no measured calibration or reproduction of their full algorithms is claimed.

Evidence: `design/validation-0.2.11/` includes complete native/ARM32/ARM64/SDK logs, UBSan/full regressions, host 19 PASS / 0 WARN / 0 FAIL, 7,392 finite original strikes, and added positive-contact-loss/unison-decay/three-rate pedal time checks. All six other engines' source hashes are unchanged and their actual native float outputs are sample-identical at 44.1/48/96 kHz. All 591 runtime skin files, UID KnAw, 238 parameter IDs/order and 57 factory programs remain unchanged.

Original native auditions: `design/audio-0.2.11/`, same 88 BPM Dm9/G13/Cmaj9/Am9 performance, 28 note-ons at velocities 29–70 and real CC64. Dry before/after and subtle Room18 version use identical MIDI/settings, constant whole-take RMS normalization and a final fade only. No per-note gain, external EQ or compression. Isolated notes 48/60/72 × velocity35/65/95 use one shared gain and remain monotonic in body RMS. Render recipes, hashes and numeric measurements are saved. Audio has been generated and numerically checked; auditory acceptance remains for the owner, not asserted by the agent.

No MPC installation, restart, public release, push or website publication in this task. Device files remain 0.2.5 with the earlier runtime-restart status; this is an offline 0.2.11 candidate. Other products/SD/SSD untouched.


## Superseding owner rejection and structural review — 2026-11-21

The owner rejected the 0.2.11 Upright audition as not recognizably piano-like. All earlier “improved”/candidate labels refer to a numerically tested experiment, not accepted tone. Do not install or publish 0.2.11 as an approved piano. No synthesis or binary changes were made during this review; Tine/Wurli and every other engine are preserved.

### Findings, ordered by importance

1. **Soundboard is an additive coloration, not the main acoustic radiation path.** `FeltClub.cpp` directly mixes the voice's dry signal, then adds `AcousticBodies` radiation. `UprightVoice` first differentiates a weighted string-displacement sum and divides by fundamental frequency, resembling a generic motion readout; it does not expose a dimensioned transverse force at the bridge. Its `bodySend` is taken after Tone/ADSR, not directly from bridge force. An acoustic model should derive bridge force from modal string slopes and pass that through the body-to-pressure radiation response. A small separate room reverb cannot supply the missing instrument response. A local numerical audit measures the board contribution, not its subjective adequacy.
2. **Only 8 fixed soundboard modes, with strong register imbalance.** At 48 kHz, first-second stereo energy, velocity65, pedal off, existing audition settings: body/direct ratios are MIDI36 -8.238 dB, 48 -13.457 dB, 60 -15.492 dB, 72 -25.609 dB, 84 -32.314 dB. Thus high notes largely bypass the modeled body. This does not prove that making the body uniformly louder will sound right; the force-to-pressure transfer and its phase/spectral shape need redesign, not a wet gain tweak.
3. **Budget and coupling are oversimplified.** Forty-eight resonators are divided among 1/2/3 strings, giving only 48/24/16 harmonics per string, abrupt representation changes at notes40/55, and additional Nyquist muting in the treble. Detuned resonators beat, but there is no reciprocal energy exchange between strings or a feedback bridge admittance. The .72/1.16 T60 split is unmeasured voicing, not a demonstrated physical explanation of two-stage decay. More modes alone are not a quality metric.
4. **Hammer/contact is not a validated coupled integration.** Only the hammer runs four internal steps; the string surface is frozen or interpolated from the previous host step. The unilateral loss is dissipative in the local contact formula but does not prove global energy conservation of the hammer/string discretization. Audit contact durations for velocity65 are 3.458/1.938/1.042/.563/.292 ms at MIDI36/48/60/72/84. These numbers are observations, not measured real-upright targets. The exp2((note-60)/6) stiffness multiplier is an uncalibrated register heuristic.
5. **The old acceptance metric cannot judge piano identity.** The legacy voicing halves upper partial readout and tilts it by ratio^(-.35), based on dominant-note harmonic windows from a mixed performance. That procedure excludes transient/mixed-note structure and is confounded by unknown velocity, note age, pedal and effects. Retire its >=10% proxy-fit gate for Upright; never equate that score or finite output with realism.

### Reference basis and limits

Reopened Bank/Sujbert DAFx2002 and Wogram's primary piano-acoustics lecture. They motivate the distinct roles of hammer excitation, string bridge coupling and the frequency/position-dependent soundboard radiation. Wogram's particular grand-piano frequencies are not adopted as measured upright parameters. Bank's simpler perceptual beating approaches also show that detuning can be useful without exact coupling; missing reciprocal coupling alone is not proof of the owner's perceptual complaint.

- https://www.dafx.de/papers/DAFX02_Bank_Sujbert_piano_synthesis.pdf
- https://www.speech.kth.se/music/5_lectures/wogram/modal.html
- https://www.dafx.de/paper-archive/2008/papers/dafx08_37.pdf

The existing private `upright.wav` is an actual 100-second, 44.1kHz mono decode from the official Shaun Martin Keyscape performance, chapter offset1967 seconds; source metadata and encoded-file SHA remain in validation-0.2.4/reference-provenance.json. File decoding/finite PCM was rechecked, but this review did not claim listening or compute a new acoustic match. It is a performed, processed sample-based instrument, not a dry measured bridge impulse or known-velocity isolated note. Keep it for approximate attack/body/tail and register comparisons only; do not bundle/re-publish it. Official Wing model page: https://www.spectrasonics.net/products/keyscape/models/wing-upright-piano/ . No installed commercial plugin rendering.

### Next implementation, with acceptance gates

Replace the Upright-only emission path first: explicit modal bridge force, independently owned main soundboard/radiation filter with documented physical approximation, and deliberately controlled direct-string leakage. Model Body as instrument response, keep global Room separate. Prototype a denser original plate/modal response without inventing “measured” data; assess resonant peaks, register balance, decay and target MPC CPU before fixing mode count. Do not copy a commercial IR.

Then review coupled string/contact integration and preserve sufficient harmonic bandwidth independently of string count. Add passive/energy-bounded coupling or an explicitly identified reduced approximation; verify global energy and sample-rate convergence rather than simply non-negative local hammer loss. Remove arbitrary legacy darkening only after comparing attack and body separately.

Audit dry single notes at multiple registers/velocities, release and pedal, then the same expressive musical sequence. Compare attack0–40ms, body40–300ms, late decay, inharmonic partial tracks and beating; do not optimize integer-harmonic purity or aggregate centroid as realism. Use the official recording only with its unknown-performance limits. Keep numerical stability/RT/CPU/isolation gates separate from the owner's listening acceptance. Do not mark complete, install or publish another “improved piano” solely because tests pass.

Reproduce the path audit by compiling `tools/calibration/audit_upright_structure.cpp` with clang++ -std=c++17 -O2 and SDK framework include. JSON evidence is in design/validation-0.2.11/rejected-review/. This review adds diagnostics/documents only; version, runtime DSP, UI and presets remain unchanged.


## Upright emission redesign 0.2.12 — 2026-11-21 (listening acceptance pending)

Owner requested implementation after rejecting 0.2.11. This is a new audition candidate, not an accepted real-piano match. No installation/publication was performed; the rejected0.2.11 record remains historical and its status is not overwritten.

Implemented only in src/engines/upright:

- Bridge-force readout replaces the old differentiated/fundamental-normalized displacement pickup. For the ideal string with modal mass mu*L/2 and f0=sqrt(T/mu)/(2L), the coefficient of endpoint slope force is T*h*pi/L=8*pi*modalMass*f0^2*h, with alternating endpoint phase and the existing normalized string coordinates. The finite contact footprint supplies a sinc weight. The stiff-string modal frequencies remain an approximation to a real wound string.
- Increase fixed storage from48to96string modes; three-string notes have32partials/string before Nyquist muting. Remove the old harmonic-proxy upper gain .5 and ratio^(-.35) tilt. Keep the independent profile hook/level for isolation, now neutral. Register gain is an explicit bounded calibration (.075*exp2((60-note)*.40/12), clamped factor .25..3), not a measured material property.
- Add a common-velocity dissipative bridge projection per unison harmonic. Transform pole state into oscillator quadrature, contract only the shared normalized velocity, then transform back. This transfers/damps collective unison motion and leaves differential motion to ring. It is a passive reduced viscous coupling in the tested normalized modal energy; it is NOT a full measured reciprocal string/soundboard admittance or a proof of globally passive nonlinear hammer integration. Remove the unmeasured .72/1.16T60 split.
- Replace the8mode additive body by64original body modes as the main force-to-radiation path:24low effective orthotropic plate modes,40higher strip-family modes, a bandlimited residual mobility floor and the existing independently owned sympathetic bank. The model uses BPF modal velocity responses with positive bandwidths and a low/high-frequency radiation rolloff. Plate/rib dimensions, stiffness, radiation weights and losses are original reduced voicing assumptions; no full Rayleigh radiation integral, measured plate/IR, source copying or real-instrument calibration is claimed.
- Voice dry leakage is2.5% of the shaped bridge signal; full bridge send drives the independent body. The host's general mixer contract and all other model paths are untouched. Tone/ADSR intentionally remain performance shaping of the bridge signal, not physical material parameters. Body Room remains the separate existing global effect.
- Bound hammer register stiffness with exp2((note-60)/24) instead of /6; retain unilateral local dissipation. Hammer-only4substeps/previous-surface treatment is still an approximation: this task does not claim a newly solved fully implicit or jointly oversampled hammer/string contact.

Research basis: Bank/Sujbert, [DAFx2002 piano synthesis](https://www.dafx.de/papers/DAFX02_Bank_Sujbert_piano_synthesis.pdf), bridge/string excitation and soundboard response; Boutillon/Ege, [Vibroacoustics of the piano soundboard: Reduced models, mobility synthesis, and acoustical radiation regime](https://arxiv.org/abs/1305.3057), orthotropic low regime and localized inter-rib high regime. The implemented64mode model is a reduced original design informed by those concepts; it does not reproduce their measured systems/complete algorithms or borrow their fitted coefficients.

### Evidence and scope

`design/validation-0.2.12/`: native+ARM32+ARM64 build and SDK0errors/0warnings; full UBSan/regression/skin/isolation tests; final binary host19PASS/0WARN/0FAIL;7,392original88-key/model/velocity/rate physical-strike cases. Additional528full-radiation88-key/2velocity/3rate bright/hard strikes: finite, nonzero, maximum1.326788 before host gain/limiter. Added shared-velocity projection energy checks at3rates/3pitch ratios and main-radiation dominance/reset checks at5registers/3rates. These are stability/routing checks, not perceptual certification or MPC CPU measurements.

At velocity65,48kHz, first-second energy, pedal off, body/direct leakage ratios are +26.934/+27.261/+27.715/+28.491/+28.176dB at MIDI36/48/60/72/84. The rejected version was -8.238/-13.457/-15.492/-25.609/-32.314dB. This verifies the architectural change; it does not establish ideal piano spectral balance.

All six other backend sources SHA256-identical and actual native float samples identical at44.1/48/96kHz versus0.2.11. All591runtime skin files unchanged;238parameter IDs/order,57programs,UIDKnAw/vendor kim remain stable. Final comparison SHA matches the final all-target build. Final host and audio were repeated for that final binary after the build changed its binary identity.

`design/audio-0.2.12/`: actual original native render, no commercial plugin rendering. Dry before-after (rejected0.2.11 first,0.2.12 second); dry reworked; subtle Room18 reworked;9isolated notes at MIDI48/60/72 andvel35/65/95. Musical sequence uses identical28note-ons atvel29–70 and actualCC64;88BPM Dm9/G13/Cmaj9/Am9 with sparse ghost/passing notes. Same MIDI/settings are byte-compared; normalize each whole musical take toward -25dBFS active RMS subject to .8peak ceiling, apply only final150msfade. Isolated notes share one gain to preserve dynamics. Save recipes/raw hashes/final binary SHA and metrics; no external EQ/compressor, no per-note normalization. Current late/body/attack metrics are descriptive; do not revive the retired harmonic-proxy realism gate. Audio was generated and numerically inspected; the agent does not claim to have listened.

No MPC installation/restart, physical/hardware-audio/performance certification, public release/push/site update. Device files remain0.2.5with prior runtime status. UI, Tine/Wurli/Felt/Clav/Muted/Thumb and other products/SD/SSD preserved. Owner listening acceptance and actual-device CPU/audio checks remain open.


## Matched Upright recordings and 0.2.13 candidate

Compared the 0.2.12 and 0.2.13 native plugins against real Yamaha Disklavier upright recordings. MAPS ENSTDkCl provides paired close-mic WAV and MIDI. The MIDI carries returned Disklavier pitch and velocity, with a roughly two-second hold. Used 18 strikes across six pitches for calibration and 17 strikes across seven other pitches for validation. Excluded MIDI 37 at velocity 56 because the acoustic strike occurs about 0.24 seconds after the aligned MIDI. Used 14 BiVib Yamaha DU1A strikes at velocities 23 and 100 as an independent check. The BiVib excerpts came from the authors' public preprocessed demos. BiVib release was not scored because the gate was unspecified. Reference audio stays in temporary analysis storage. The provenance and hashes identify the recordings in `design/validation-0.2.13/`.

Sources are the [MAPS dataset and license](https://zenodo.org/records/18160555), the [BiVib dataset](https://zenodo.org/records/2573232), and the [BiVib reference excerpts from the paper authors](https://riccardovib.github.io/NeuralSample-basedPiano_pages/).

The new Upright voicing decreases upper partials progressively through the middle and high register and reduces excess fundamental in the low bass. The string dispersion coefficient increases toward the treble and is bounded at extreme notes. Its frequency law preserves the tuned fundamental. Shorter modal decay applies at the bottom and top of the keyboard. Finite hammer width now weights both excitation and hammer feedback. BOARD again shapes the lower-mid range of the bridge signal for newly struck notes. Upright's amplitude release uses a finite squared fade with the selected Release duration alongside the existing modal damper. No reference audio, sampled responses, or commercial plugin renders enter the engine.

Measured errors are relative to the reference note's own body level and use acoustically aligned onsets. Native matched renders use linear MIDI velocity, the recorded pitch and gate, and no post-effects or pedal. Initial parameters are Tone 60, Body 62, String 36, Board 20, Hammer 32, Velocity 65, Detune 1.2, Resonance 30, Master −10 dB, Attack 1 ms, Decay 1.8 s, Sustain 100% and Release 720 ms. Spectral error is mean absolute dB difference across normalized bands from 45 Hz to 12 kHz, with each band floored at −40 dB. The partial-level error compares prominent partial peaks relative to the fundamental. Stretch is the error of their frequencies relative to integer harmonics, expressed in cents.

| Validation set | Body spectrum error | Partial level error | Partial stretch error | 0.85–1.05 s level error |
| --- | --- | --- | --- | --- |
| MAPS, 17 held-out strikes | 7.34 → 5.47 dB | 15.34 → 7.71 dB | 10.45 → 4.20 cents | 4.93 → 3.56 dB |
| BiVib, 14 strikes | 8.35 → 6.46 dB | 22.85 → 12.02 dB | 11.93 → 9.42 cents | 6.01 → 4.06 dB |

On MAPS overall, the variation from soft to hard strikes improved in some keys and increased the aggregate span error from 3.80 to 4.32 dB. The 10–80 ms release error went from 9.75 to 12.35 dB. The 120–250 ms window in high notes is often near the reference recording's noise floor. Neither release metric was used to establish an acoustic damper match. The audible result is pending the owner's audition.

Native, ARM32 and ARM64 build, SDK validation, all UBSan/host/regression checks, and the pedal-up repeated note-off test passed. The host simulator passed 19 checks with zero warnings or failures. At 48 kHz, 128-frame blocks and 24 held notes, a local native benchmark measured 33.70% → 33.82% median callback time budget and 37.00% → 35.90% p99. Device timing and audio are pending. The six other engines are sample-identical at three rates. All 591 skin files, 238 parameters, 57 programs and UID KnAw are preserved. Local candidate only. MPC installation and public release are pending.

The 88 BPM dry before-and-after render in `design/audio-0.2.13/` uses identical note and pedal events and one fixed loudness adjustment per complete take. The report records the hashes and gain applied to each take.


## Upright action and radiation, 0.2.14 local audition

The owner heard a small improvement in 0.2.13 and still did not recognize its tone as Upright. Attack analysis of the paired MAPS and BiVib notes found a low, short component at frequencies below the notes' fundamental. This component was especially weak in the synthesis from middle C upward. The results are consistent with a separate mechanical action and case response. The acoustic measurements combine the instrument and microphone perspective, so the attribution to specific structures remains an interpretation. Listening research independently shows that mechanical action sounds can affect a listener's judgment. [Perception of touch quality in piano tones](https://pubmed.ncbi.nlm.nih.gov/25373983/)

Upright now drives six damped resonances from 96 to 478 Hz with the existing hammer contact force. The total injected impulse is capped at twice the initial hammer momentum. The cabinet frequencies stay fixed across pitches and the impulse gain follows a smooth register curve. This is a compact case response whose mode centers, decay and readout are voicing choices. It retains the existing strings and does not require recorded impulses or a delayed note-on. A high-frequency click prototype and small changes to unison drive/radiation failed to give a consistent improvement and were excluded. The new body path also models two short radiation arrivals, 0.30 and 1.48 ms, at an opposite-sign gain of 0.19 in the two channels. Tests at 44.1, 48 and 96 kHz verify the side component and preservation of the summed mono output.

For each strike, the first 40 ms 125–500 Hz band energy was normalized to total 125 Hz–12 kHz energy at 80–160 ms. The mean absolute dB error, with a −45 dB floor, fell from 15.14 to 4.42 dB over 35 MAPS strikes and 12.60 to 4.33 dB across 14 BiVib strikes. Middle C at velocity 63 measured −11.38 dB below the note body in the sub-fundamental band. Native 0.2.13 was −35.35 dB and 0.2.14 was −17.00 dB. C3 remained weak in this band and the brightness change from soft to hard in some registers is still smaller than in the recordings. The new body stereo returned a MAPS side-to-mid median of −11.73 dB versus −30.39 dB before. The close-mic recordings gave −6.55 dB. Mono late residual (220–320 ms) between old and new synthesis was under −31 dB relative to the old signal across MAPS and under −42 dB across BiVib. These comparisons describe parts of the sound. Listening acceptance remains pending. [MAPS source](https://zenodo.org/records/18160555) · [BiVib source](https://zenodo.org/records/2573232)

Native, ARM32, ARM64, SDK, complete regression and host tests passed. The force-driven response settles and resets at all three tested sample rates. The existing 528 all-key radiation checks passed. Native 24-note sustained CPU at 48 kHz and 128 frames remained near 36% of the callback budget. This desk benchmark gives no MPC CPU measurement. Six other engines are sample-identical at the three sample rates and all 591 skin images/files are preserved. State layout stays at 238 parameters and 57 factory programs. The native comparison WAV tests use the same five strikes or the same original 88 BPM performance. See `design/audio-0.2.14/` and `design/validation-0.2.14/`. Device installation, testing and publication are pending.


## Upright touch, treble decay and pedal, 0.2.15 local audition

The owner found 0.2.14 closer but still insufficient. Retained its action/radiation path. On light strikes, lower felt compression is approximated with a bounded velocity-dependent stiffness multiplier. The adjustment reaches unity at normalized strike velocity 0.62. Experiments with stronger global stiffness/loss changes or a second polarization bank produced inconsistent results across reference sets. Added a loss rate above MIDI 68 that increases with partial order, leaving the fundamental decay and lower register unchanged. The new rate is a measured voicing adjustment within the model, not a material measurement.

The old pedal bank used 24 modes at whole-tone intervals, giving alternate keys very different sympathetic gain. It now covers the same register with 48 semitone modes from A1 through G#5. Pole-pair drive normalization sets the resonant gain independently of sample rate. The body tracks Master Tuning and suspends silent resonance processing while the pedal is up. A tone-driven output test shows a 1.23 dB span across 12 consecutive keys and 0.55 dB maximum change versus 48 kHz at 44.1 and 96 kHz. The matching resonances follow 452 Hz tuning in the same test. This verifies coverage and gain consistency. The isolated recordings have limited pedal information, so they do not establish calibrated pedal color. [Reduced sustain-pedal filter model](https://www.dafx.de/paper-archive/2008/papers/dafx08_37.pdf)

Final native comparisons use 30 MAPS and 8 independent BiVib mid/upper strikes with matching pitch, velocity and gate. The high-band attack span error measures the difference in the share of 2–12 kHz power from soft to strong strikes in 3–40 ms. It declined from 5.54 to 3.43 dB across 10 MAPS held-out strikes and from 4.28 to 3.02 dB in BiVib. Treble decay error, relative to the initial body amplitude for each reliable partial among the first six, went from 8.79 to 6.55 dB in 13 MAPS notes and 5.49 to 4.62 dB in four BiVib notes. Absolute high-band error at 80–160 ms increased on the MAPS held-out subset from 8.58 to 10.17 dB, while decreasing on BiVib from 11.54 to 10.27 dB. The comparisons cover attack response and decay. The remaining sustained tone and perceptual match require listening. [MAPS](https://zenodo.org/records/18160555) · [BiVib](https://zenodo.org/records/2573232)

Native, both ARM targets, SDK checks and all regressions passed. The other six engines were sample-identical at three sample rates. The runtime skin, 238 parameter IDs and order, 57 programs and UID KnAw are preserved. The dry matched audios are in `design/audio-0.2.15/`. Six isolated strikes use one shared gain across the two versions. The original pedaled 88 BPM progression was rendered with identical events and per-take level matching. The build, tests, exact render hashes and numerical measurements are under `design/validation-0.2.15/`. Installation, device QA and publication are pending.


## Upright partial buildup, 0.2.16 local audition

The owner continued to hear an unnatural tone in 0.2.15. In 12 MAPS midrange strikes, the reliable partials typically reached half of their peak early power in 13 ms in the reference and 4 ms in the simulation. The median spread of those times within a note was 19.5 ms in the reference and 0 ms in the simulation. Six comparable BiVib strikes gave 19 ms and a 12 ms spread. The tests used aligned acoustic onsets, 3 ms time steps and demodulation centred on each partial. A split between transient and sustained partials is also discussed in the literature. [Sines, Transient, Noise Neural Modeling of Piano Notes](https://arxiv.org/abs/2409.06513)

0.2.16 varies the radiation buildup of the first six partial groups. The fundamental and second partial build more slowly than the upper groups. Time constants follow the note period and are bounded from 1.5 to 42 ms. Part of the tonal signal and the existing case strike are emitted immediately. The low-bass contribution is faded in over MIDI 35 to 43 after full-depth trials worsened the rise on MIDI 33 and 35. This is an effective radiation envelope fitted to recorded note timing. It changes neither the string states nor the force solving the hammer contact. No new voice, recorded waveform or global ADSR delay is involved.

Final native rise-time mean absolute error fell from 11.33 to 7.94 ms for the 12 MAPS midrange notes and from 11.67 to 4.08 ms for the six BiVib midrange notes. Per-note spread moved from 0 to 18 ms and 0 to 10.5 ms respectively. The lower and upper notes taken together also improved on this timing measure after the low-bass taper, although the adjacent time windows in BiVib still have small level regressions. The timing checks are limited to these recordings and do not establish perceptual acceptance. Alternative board bandwidths, a different reciprocal hammer update and experimental axial bass were evaluated in scratch and kept outside this candidate because their benefits did not consistently address the same listening issue. [MAPS](https://zenodo.org/records/18160555) · [BiVib](https://zenodo.org/records/2573232)

Native, both ARM builds, SDK validation and complete regression passed. The new test confirms unchanged strike forces and transverse string state while the early radiation builds, plus matching held output at three sample rates. The six other engines remain sample-identical and all 591 runtime skin files match the previous hashes. Parameter IDs, order, program catalog and UID are preserved. The original 88 BPM passage and isolated-note comparison use identical MIDI and disabled pedal events, reverb and post effects. Evidence is in design/validation-0.2.16 and design/audio-0.2.16. Audio acceptance and MPC listening are pending.


## Upright prompt high partials, 0.2.17 audition

The owner found 0.2.16's attack too rounded. The native comparison with isolated MAPS and BiVib strikes showed missing 5–10 kHz energy in the first 25 ms of many midrange notes. The 1–5 kHz bands often already matched or exceeded the reference. Trial increases in hammer stiffness also brightened the sustained tone, so the selected change adds a decaying observation gain only to the existing partials in a tapered 4.55–11.2 kHz region. The gain increases with strike velocity and has a 10 ms time constant. This is voicing of the early bridge radiation. The hammer contact, string modes, lower-partial buildup and body resonance continue to use their 0.2.16 mechanics. No click, noise layer or recorded impulse is introduced. [MAPS](https://zenodo.org/records/18160555) · [BiVib](https://zenodo.org/records/2573232)

The final native render raises the median 5–10 kHz energy relative to body level by 3.27 dB in 0–8 ms and 1.06 dB in 8–25 ms across nine MAPS midrange medium-to-strong strikes. Three similarly selected BiVib strikes give 4.26 and 1.30 dB. Median changes below 5 kHz and at 80–160 ms were at most 0.01 dB in these subsets. Some recorded notes already have a bright attack relative to their body, so the measured change describes the intervention rather than a uniformly closer acoustic match. See `design/validation-0.2.17/attack-summary.json` and the individual measurements for all 49 strikes.

Native, ARM32, ARM64, SDK and complete regression checks passed. The six other models are sample-identical at three sample rates and all 591 skin files match. In `design/audio-0.2.17/`, the dry note file alternates 0.2.16 then 0.2.17 for each of six identical strikes. The dry phrase file uses the same 88 BPM events, with shared gain between versions. Device and listening checks remain pending.


## Upright unison radiation, 0.2.18 audition

The owner still found the Upright tone unnatural after the 0.2.17 attack change. Isolated strikes showed a second-partial trough about half a second into the held note as the model's mistuned strings fell out of phase. Temporarily matching the string frequencies removed the trough, but the second partial still failed to rise relative to the fundamental as it did in the recordings. Trials that shifted the common mode by roughly 2 Hz improved some levels and increased the stretch error.

The selected candidate weights the middle-register second partial toward the nominally tuned string and away from the flanking unisons. The additional weights have a mean of one and are computed at note-on. The normalized gain for identical string motion stays at unity, and unequal motion radiates through the existing body. The aperture fades in over MIDI 46–54 and out over 66–75. It approximates unequal radiation from adjacent contacts. The exact gains are voicing choices. The damper, hammer, unison spacing and existing velocity coupling retain their previous calculations. The literature describes the differing radiation and decay of coupled string modes. [Coupled string motion](https://www.speech.kth.se/music/5_lectures/weinreic/strings.html)

The final native plugin was rendered dry with a fixed 1.17 s hold. Across the 16 middle-register strikes with enough reference audio, mean absolute H2/H1 error at 400–620 ms fell from 12.55 to 7.86 dB. MAPS improved from 10.90 to 5.73 dB and BiVib from 15.30 to 11.40 dB. Middle C, velocity 63, measured −16.19 to −3.38 dB against +0.63 dB. The same window for E4, velocity 100, was −16.82 to −4.90 dB against +7.97 dB. Error in the first body window, 60–160 ms, was 5.33 to 5.41 dB. Combined whole-note RMS decline error rose from 2.04 to 2.32 dB across the held windows. The 25 outer-register strikes had identical measurements in all four windows. These recordings support a narrower correction to the partial balance. Listening acceptance is pending. [MAPS](https://zenodo.org/records/18160555) · [BiVib](https://zenodo.org/records/2573232)

The native, ARM32 and ARM64 builds, SDK validator, regression and all 19 host checks passed. The six other engines are sample-identical at 44.1, 48 and 96 kHz. All 591 runtime skin files and the plugin's 238 parameter definitions, 57 program names and UID match. Paired notes and the 88 BPM dry comparison share gain between both versions. See design/validation-0.2.18 and design/audio-0.2.18. Device installation, device listening and public release are pending.


## Upright F3–E4 calibration, 0.2.19 audition

Felt Club's Upright was tuned against one Yamaha Disklavier upright and one close-microphone perspective. The 36 MAPS ENSTDkCl recordings cover F3–E4 at three strike intensities. The MIDI tracks identify the actual velocities, gates and pedal events. Training used the light and strong strikes at ten pitches. All medium strikes on those pitches and all three strikes on MIDI 56 and 61 were reserved for validation. The latter keys use interpolated calibration data from adjacent notes. The native audit follows the reference pedal state for each take and aligns the sounding onset. Only the model's calibration coefficients and numeric measurements are stored in the project. [MAPS recordings and license](https://zenodo.org/records/18160555)

At note-on, the calibration adjusts the bridge radiation gain and the positive modal loss of the first twelve partial groups. The first six also use different losses and radiation weights across their near-unison strings. The additional radiation weights retain a unit mean across each group. The paired losses keep the current detune response and bound the decay time. The two edge keys at each end progressively blend into the previous register. These are effective calibration coefficients for the existing modal synthesis and soundboard path. A calibrated mechanical model of the actual Disklavier bridge would require measurements of its interaction with the strings.

Mean absolute dB errors relative to the note body in the final native renders follow. Early body covers 65–165 ms. Held decay measures the change in prominent partials from that body window through 1.31 s.

| Validation group | Early partial levels | Held partial levels | Change from body to held |
| --- | --- | --- | --- |
| Ten withheld medium strikes | 9.19 → 5.29 dB | 9.17 → 6.21 dB | 5.85 → 3.87 dB |
| Six strikes on two withheld pitches | 6.51 → 4.18 dB | 7.65 → 5.96 dB | 5.27 → 4.57 dB |
| All 36 strikes | 8.35 → 4.80 dB | 8.93 → 6.03 dB | 5.81 → 4.02 dB |

The release-window error across all strikes was 5.28 → 5.54 dB. The full data and source hashes are in design/validation-0.2.19. The fit concerns one instrument and recording position. Owner listening acceptance determines whether to extend it to additional pitches.

Native, ARM32 and ARM64 builds, SDK validation, full regressions and all 19 host checks passed. Six other engines are sample-identical at three rates, and 27 tested Upright strikes outside the calibrated octave are sample-identical at 48 kHz. All 591 runtime skin files, 238 parameters, 57 program names and the plugin UID match. The paired notes and dry four-chord comparison in design/audio-0.2.19 use the same MIDI and a common playback gain for both versions. MPC hardware checks and publication are pending.



## Upright touch and damper closure, 0.2.20 audition

The owner heard 0.2.19 as closer to an upright. The next candidate targets touch and note endings on the same F3–E4 octave. The 36 MIDI-aligned MAPS Disklavier takes reveal excess high-frequency energy during light F3–A3 attacks. Note endings show excess energy in upper partials, especially on notes released under the pedal before pedal-up. [MAPS recordings](https://zenodo.org/records/18160555)

At note-on, a bounded launch exponent widens the soft-to-strong response. It fades out as velocity rises and keeps hammer speed increasing at every MIDI velocity. Light strikes on the lower keys use softer felt contact. Firm strikes in the upper portion compress the felt more stiffly. These values are effective adjustments to the instrument's existing unilateral contact model. The register weights taper at the calibration boundaries. [Hammer–string parameter fitting research](https://pubmed.ncbi.nlm.nih.gov/16119368/)

When the damper closes, the string modes receive additional losses that increase with partial index. Direct note-off leaves fundamental damping at its previous value. Pedal-up adds loss to the fundamental of a sustained note as well as to its upper partials. The existing Release duration controls the amplitude envelope. This treats the measured release spectrum with passive mode losses. The study of part-pedaling describes real changes in timbre during damper contact. [Damper contact measurements](https://pubmed.ncbi.nlm.nih.gov/19640015/)

| Matched native measurements | 0.2.19 → 0.2.20 |
| --- | --- |
| Soft F3–A3 attack, mean band-level error at 13–48 ms | 5.99 → 2.90 dB |
| Soft-to-strong body-level span error, all 12 keys | 2.41 → 1.31 dB |
| Attack band-level error, the two withheld pitches | 3.68 → 2.72 dB |
| Release H1–H6 error, 24 medium and strong strikes | 10.23 → 7.44 dB |
| Release H1–H6 error, 12 pedal-up cases | 12.73 → 8.71 dB |
| Change in partial levels from body to held window, all takes | 4.02 → 4.02 dB |

Attack errors use five frequency bands relative to the summed power in those bands and omit reference bands below −46 dB. Touch dynamics are the body level of each take relative to the same note's strong strike. Release errors use H1–H6 from 20 to 235 ms relative to those partials before closure, keeping reference windows with adequate signal over the noise floor. Broadband release error for direct note-off rose from 4.01 to 4.44 dB, and the held partial-level error rose from 6.03 to 6.12 dB. This evaluation concerns one upright and one microphone perspective.

The final implementation passed a strictly monotonic launch check at 5,334 velocity, key and sensitivity combinations, 2,916 contact renders, and 72 damper checks at 44.1, 48 and 96 kHz with Release from 15 to 6000 ms. The native and both ARM builds, SDK validator, regression suite and all 19 host checks passed. The six other model renders match at three rates. Forty-two outside-octave dry and pedal-enabled cases match the prior version sample for sample. The 591 skin files, parameter definitions, program names and plugin UID match as well.

The short-note and four-chord A/B files are under design/audio-0.2.20. Both pairs use a common gain, the same velocities and events for both versions, and disabled post effects. The final two short-note pairs include pedal release. Evidence, reference hashes and build logs are in design/validation-0.2.20. Owner listening and MPC hardware checks remain pending.



## Upright soundboard radiation, 0.2.21 audition

The owner heard the previous Upright as Clav-like. Native matching and a separate instrumented body render traced the issue to a narrow early body response. In 24 medium and strong MAPS strikes, the 2.4–4.8 kHz band was relatively close during the 12–44 ms attack and dropped below the recording in the 65–165 ms window. In 21 dry diagnostic strikes, the unresolved broadband term was about 6 dB higher in RMS level than the resolved board modes in that window. Their relative phase also affects the final output level. [MAPS](https://zenodo.org/records/18160555)

Each Upright voice now sends ordinary bridge force, a middle-register portion of that force, and a separate tapered high-partial pressure. The last input uses opposite weights for central and flanking unison strings. Its weights sum to zero for identical motion. At the center of the register profile the original board modes receive 30% more drive, and the broadband term receives 20% less. This raises their relative level by 4.2 dB. The pressure from unison differences takes two short paths of 0.62 and 1.41 ms to the two channels. It passes through the same smoothed tone control and voice envelope. The full effect covers MIDI 53–64 and tapers to zero by MIDI 47 and 70. Bass input from a concurrent voice enters the shared board independently of the taper. The method is a feed-forward radiation estimate informed by the observed locality of upper soundboard modes. [Upright soundboard measurements and reduced models](https://arxiv.org/abs/1305.3057)

The final native results used matched note and velocity, matching the pedal state where known. Analysis masks bands below the reference fundamental and those close to the reference noise floor.

| MAPS medium and strong strikes, MIDI 53–64 | 0.2.20 → 0.2.21 |
| --- | --- |
| 65–165 ms spectral error across bands, 24 strikes | 4.32 → 3.34 dB |
| 65–165 ms spectral error, four strikes on two withheld pitches | 2.92 → 2.37 dB |
| 65–165 ms signed difference at 2.4–4.8 kHz, median | −7.90 → +0.19 dB |
| 12–44 ms spectral error across bands | 4.12 → 4.21 dB |
| 430–660 ms gain-normalized per-band decay error | 3.88 → 3.13 dB |

The 2.4–4.8 kHz balance at 220–380 ms moved from +2.17 to +4.87 dB relative to the recordings. Independent BiVib spectral error at 65–165 ms was 7.65 to 7.71 dB across 14 notes. The native measurements describe the response of this model and the two reference recording perspectives. [BiVib](https://zenodo.org/records/2573232)

The new wide-chord A/B preserves the recorded timing and touch gestures adapted to the original chord progression in the last audition. The render uses dry settings and a shared gain. The 2.4–4.8 kHz band gained 1.51 dB of relative energy through the phrase, while raw overall level moved −0.17 dB. The waveform difference after gain matching was −22.31 dB relative RMS. The isolated-note comparison alternates baseline and candidate per strike. [ASAP performance dataset](https://github.com/fosfrancesco/asap-dataset)

Native and both ARM targets, SDK validation, regression tests and all 19 host checks passed. The test suite exercises the new inputs at three rates and checks that their outputs add linearly and reset to silence. The 42 tested native cases outside the taper and all six other models at three rates match the baseline sample for sample. The UID, parameter/program definitions and 591 runtime skin files match. The single desktop 24-voice run at 48 kHz and 128 frames gave a median of 34.33% to 35.16% of the callback time budget. This is desktop timing. MPC verification and listening acceptance are pending. See design/validation-0.2.21 and design/audio-0.2.21.



## Upright bass voicing, 0.2.22 audition

The owner still heard a Clav-like quality in the bass of the 0.2.21 chord example. Matched soft, medium and strong MAPS upright recordings showed excess fourth-partial pressure on F2 and low, fast-fading fundamentals on G2. C3 exhibited slower fundamental decay than the reference even while its overall sound decayed faster. The different behavior across keys informed a local response change. [MAPS](https://zenodo.org/records/18160555)

The effective hammer contact position moves by up to one percent of string length, applying to both excitation and the felt’s view of string motion. A frequency-dependent bridge pressure correction reduces excess partials around 349 and 604 Hz. Positive common-velocity loss multipliers for the lowest four partials slow G2’s fundamental decay and speed C3’s. The calibration affects MIDI 39–52 with a taper at both ends. This is an empirical reduced model calibrated to recordings.

| MIDI-aligned acoustic comparison | 0.2.21 → 0.2.22 |
| --- | --- |
| 65–165 ms harmonic error, 12 strikes on F2, G2, C3 and D3 | 5.30 → 3.35 dB |
| 65–165 ms harmonic error, three comparison pitches F♯2, B♭2 and E♭3 | 4.84 → 2.84 dB |
| Medium F2 fourth partial, signed error | +13.0 → +5.7 dB |
| Medium G2 fundamental, signed error | −8.6 → −2.0 dB |
| Medium G2 relative RMS at 760–1020 ms, signed error | −9.4 → −3.8 dB |

The F2 fourth partial and G2 late decay still differ from the recording. The note comparison alternates versions for each pitch. The dry chord A/B reuses the previous timing, velocities and pedal gestures, and plays both versions at a shared gain. Whole-phrase RMS level rises 0.21 dB. Auditions and reports are in design/audio-0.2.22 and design/validation-0.2.22.

Native, both ARM targets, SDK validation and regression tests passed. The unison loss test covers five keys, three rates and three pitch bends. Sixty tested notes outside the bass profile and all six other engines at three rates match the previous native output sample for sample. UID, program and parameter definitions and 591 skin files match. MPC device and owner listening verification are pending.


## Upright reciprocal contact and pedal release, 0.2.23 audition

The owner still heard an artificial quality in the 0.2.22 Upright. Inspection found that the previous hammer solver substepped the hammer against a delayed string estimate. It then advanced the string modes using the averaged force at the audio rate. The strongest strikes in the sampled cases gained mechanical energy, and their transfer varied with sample rate.

The updated solver uses exact half-sample free modal rotations before and after a shared midpoint impulse, with equal and opposite momentum transfer to the hammer and strings. Felt loss uses the midpoint relative contact velocity and the effective reciprocal mass. During contact, the mode recurrence receives the centered impulse response. After separation, the usual free-mode recurrence and bridge radiation resume. The effective felt stiffness decreases by up to 30% across the bass profile to soften the measured prompt brightness. No audio examples are included in the instrument data.

Pedal-up for sustained notes uses two positive loss levels. The felt captures the moving string over the first 32 ms and a weaker contact loss permits the remaining acoustic tail to fade. The transition uses the existing smooth damper ramp. MIDI 38–51 receives a light version of the correction and MIDI 55–66 receives its full weight. This is a reduced contact model informed by measured decay through damper closure. Recorded late tails can include room contributions. [Acoustic damper measurements](https://pubmed.ncbi.nlm.nih.gov/19640015/)

| Matched MAPS comparison, 51 total strikes | 0.2.22 → 0.2.23 |
| --- | --- |
| Low-register spectral error, 0–8 ms | 5.95 → 4.63 dB |
| Low-register spectral error, 8–25 ms | 4.36 → 3.23 dB |
| Low-register spectral error, 25–60 ms | 3.55 → 2.78 dB |
| Low-register spectral error, 60–165 ms | 2.81 → 2.92 dB |
| Pedal-up partial decay error, 12–47 ms | 6.79 → 5.79 dB |
| Pedal-up partial decay error, 155–250 ms | 11.58 → 9.04 dB |

The spectral errors use level-relative bands with a reference energy floor. The pedal comparison follows the MIDI and measures H1–H6 relative to the same note before pedal-up, excluding partials below a reference signal-to-noise threshold. Some higher notes still ring too loudly after pedal-up. [MAPS](https://zenodo.org/records/18160555)

The 720-case mechanical sweep includes hammer kinetic, string modal and estimated contact spring energy. The largest energy excursion at 48 kHz falls from 27.89% to 0.41% above the initial strike energy. The single-step splitting still has a small numerical error. Native and both ARM builds, SDK validation, regression and all 19 host checks passed. In a desktop stress test that cleared and struck 24 notes each 128-frame block at 48 kHz, median block time rose from 44.43% to 52.13% of the budget at velocity 77. The new p99 was 57.24%. MPC performance requires a device test. All six other engines matched the baseline across three sample rates. The two dry A/B recordings use shared gain. Listening verification is pending.


## Approved lighter voicing, 0.2.24

The owner approved a lighter Dusty Tones audition for neo-soul and hip-hop use. The existing Piano / Sunday Upright program loads that dry voicing, with Body 50 and Release 520 ms. An Upright-only stereo peak filter applies −1.8 dB at 330 Hz, Q 0.76 to the dry string and soundboard output before the shared effects. The other factory setting arrays and serialized parameter schema retain their values. Existing Upright projects retain their saved knobs and receive the updated output voicing.

The native factory render on the same MIDI differs from the approved reference by −59.28 dB relative RMS after gain matching. With the same note C4, velocity 35 to 100 increases first-100 ms RMS by 9.40 dB and the 1.5–9 kHz share by 4.83 dB in the 10–100 ms attack window. This confirms a timbre change with touch under the approved preset. Only the generated C4 takes and numerical evidence are archived in the project. [Original session MIDI source](https://www.patreon.com/musicianparadise/posts/soul-dust-rhodes-139302500)

Native, ARM32, ARM64, SDK, regression and all 19 host checks passed. Across 44.1, 48 and 96 kHz the six remaining models matched the previous native samples. A 0.2.23 Sunday Upright chunk and a customized chunk each retained their Body and Release values in 0.2.24. See design/validation-0.2.24 and design/audio-0.2.24. MPC hardware verification is pending.
