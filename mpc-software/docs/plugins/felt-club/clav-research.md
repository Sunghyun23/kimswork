# Clav — first dedicated model revision, 0.2.35

## Primary research

Gabrielli, Välimäki, Penttinen, Squartini and Bilbao, *A digital waveguide-based approach for Clavinet modeling and synthesis* (2013), DOI10.1186/1687-6180-2013-103: https://link.springer.com/article/10.1186/1687-6180-2013-103 . Describes tangent/anvil excitation, separate magnetic pickups, pitch-dependent dispersion and yarn release. Its recordings show a brief downward release pitch change. The authors use digital waveguides; our independent modal implementation is different. The linked demonstration site was inaccessible in this research turn; no reference recording was analyzed or fitted.

The original Hohner D6 owner manual explains key/tangent contact, pickup combinations and damping: https://www.clavinet.com/d6man.php . It is used to establish the mechanism, not to certify sound equivalence.

## Existing engine audit

The old Clav used a generic nonlinear piano-hammer bounce, a single combined pickup weighting at0.19/0.73 string positions, normalized velocity readout and large empirical treble gain. Although modal strings and two coefficients were present, separate pickup flux paths, tangent-specific excitation and the release-boundary behavior were absent.

## Original reduced implementation

ClavCore retains its own32modal resonators. The Clav-only tangent supplies a finite nonnegative external pulse F=(J/T)*30*u²*(1−u)² with J=mass*launchVelocity and u in[0,1]. Its continuous-time integral is J. Duration is bounded0.15–1.6ms and varies with register and tip hardness. No piano-bounce integration is used. This is a feed-forward excitation surrogate, not reciprocal contact or a certified energy/passivity solution.

The modal force position remains controlled by existing Pick. Dispersion becomes a bounded empirical register curve. String losses depend on register, harmonic order, Body and Mute; no input force persists during sustain. All states and coefficients live inside the Clav engine.

Center and bridge pickup paths are independently accumulated at assumed normalized positions0.20/0.08. The existing Bite value controls their blend. Each path uses a bounded opposite-curvature flux surrogate, differentiated to obtain voltage; this replaces frequency-normalized differentiation and its large compensating treble boost. Readout level/gap/curvature are original assumptions, not a measured magnetic field or circuit solution. Existing Tone filtering and ADSR remain.

On actual note release, the tangent pulse is canceled, modal losses engage, and a short smoothed ratio approaches2^(−3/12). This approximates a changed effective string length; no explicit nonspeaking string/yarn boundary solver is claimed. Host bend multiplies this ratio. The existing virtual sustain pedal delays the release behavior; reset/voice reuse restore the held state. Release still acts as a maximum output gate while natural modal damping may stop sound sooner. The internal release pole update stops after settling.

## Validation and bounds

Native, armv7, aarch64 and SDK validation pass. The full existing UBSan/engine/VST/state/chord/skin/isolation suite passes, with host19PASS/0WARN/0FAIL. New checks cover pulse integral within2% at3rates/3registers, distinct pickup combs, pedal/release-pitch/bend/reset and monotonic35/65/100touch energy across3rates/3registers. Full88-key output guards remain in the suite; notes outside an original instrument's range are model extrapolations.

Six other native models are sample-identical against0.2.34 at44.1/48/96kHz. All four accepted Thumb header hashes remain exact. UI/238parameter IDs/UID/57factory names are unchanged. Existing Clav programs now use this revised engine; old Clav timbre is not expected to be identical on recall.

This is an uncalibrated physically informed candidate, not Hohner/LoungeLizard/Keyscape equivalence or device CPU certification. Own matched dry audio and evidence are under design/audio-0.2.35 and design/validation-0.2.35. Owner listening and MPC checks are pending; no install/restart/release/site publication. Muted guitar refinement follows Clav acceptance, using the owner's Hikaru Utada reference; the original/mix clarification remains separate.


## 0.2.35 rejected as too bright; Superstition reference —0.2.36

The owner specifically selects Stevie Wonder's Superstition. The official Talking Book Apple preview was obtained, decoded and measured, with provenance in validation-0.2.36. The recording engineer describes using guitar effects with the Clav, including experimentation with phaser, wah and distortion: https://wsdg.com/pro-sound-news-behind-the-magic-recording-stevie-wonders-superstition/ . This does not establish an exact final effects transfer function. The repair-resource FAQ identifies a model C: https://www.clavinet.com/clavfaq.pdf . Do not call the target a verified D6 circuit emulation.

The source preview is a full mix with voice, drums, horns and bass. Its broad spectrum has more high-band energy than the isolated previous model, so fitting the entire mix as if it were isolated Clav would be misleading. These statistics are recorded as context, not a spectral-match objective. The owner's excessive-brightness feedback is addressed through the raw model's concentrated low-order partial balance and signal bandwidth. No source audio is a runtime asset.

Four original variants (baseline, mild, balanced, dark) were screened through direct C++ at Eb3/Eb4/Eb5, fixed controls/velocity72. The selected balanced version changes:

- Modal contact-compliance taper: force drive is divided by sqrt(harmonic index), retaining fundamental excitation while reducing excessive upper-mode momentum. This is empirical reduced-model voicing, not a newly solved contact boundary.
- Pickup blend becomes0.05+0.50*Bite, favoring more of the center pickup at the same control value. Pickup positions/curvature stay unchanged.
- Clav-only Tone ceiling becomes9kHz from18kHz. The second-pole blend becomes0.92−0.40*Tone. Held/fresh Tone use the same law and existing5ms smoothing.
- Clav-only output level becomes1.65 to compensate voicing-energy loss without changing user Master. The two native audition performances stay within0.3dB raw RMS of the previous version. The audition adds fixed RMS matching afterward.

In the fixed Eb3 diagnostic, H2/H1 changes from+8.91 to+5.80dB and H3/H1 from+10.19 to+5.12dB. These are own-model measurements, not measurements of separated parts of the record. The goal is stronger perceived fundamental/body while retaining the tangent attack and pickup comb. The finite pulse, yarn release/retuning, natural string losses and articulation controls are preserved. No phaser/wah/mode/UI or preset selector is added. Existing Clav programs use the new engine voicing.

Native/bothARM/SDK validation and the complete suite pass, including a new contact-taper/pickup/bandwidth check at3rates. Host19PASS0WARN0FAIL. Six other native models are sample-identical at44.1/48/96kHz; all approved Thumb source hashes remain exact.238IDs/UID/57names are unchanged. Both the earlier dry groove and an original lower-register riff have matched before/after audio in design/audio-0.2.36. This remains a reference-guided listening candidate; no exact Superstition mix, measured Clav-C geometry, perceptual certification or MPC installation/publication is claimed.


## Owner acceptance

The owner approved Clav0.2.36 on2026-11-21 and asked to proceed to Muted. This supersedes pending listening status for0.2.36. Keep its accepted-source snapshots/hash manifest intact while developing Muted. MPC verification remains pending.
