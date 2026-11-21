# Felt Piano contact audition

Version 0.2.26 starts the dedicated Felt review. A prepared felt piano places soft material between hammer and string. The existing Felt engine multiplied contact stiffness by an exponential with a six-semitone register span, reaching a factor of 256 before the additional multiplier. The new register law uses 24 semitones and bounds 0.3–4, with a 1.3 material multiplier. Positive velocity-dependent contact loss and a sinc-shaped finite contact footprint soften excitation of upper string modes. The additional upper-key level rise is reduced.

The soundboard pedal ramp, damper ramp and closed-string loss now use seconds, retaining the old behavior approximately around 44.1 kHz. The string/radiation path remains a reduced model. This first candidate still uses the existing motion readout and compact body bank. It has no measured-felt reference fit.

The supplied Dusty Tones MIDI was rendered at its named 75 BPM through the normal VST entry point, with identical notes, velocities and pedals. Comparison audio uses matched phrase RMS. Both native and ARM builds, SDK validation, regression and 19 host checks passed. Six other engines match their previous native audio at three sample rates. Owner listening and MPC device checks are pending.

[Prepared felt piano structure](https://www.spectrasonics.net/products/keyscape/models/double-felt-grand/) · [MIDI source](https://www.patreon.com/musicianparadise/posts/soul-dust-rhodes-139302500)


## User-directed restart, 0.2.27

The owner found the 0.2.26 audition insufficiently piano-like and requested a copy of the approved Upright engine as the starting point. Felt now owns independent copies of its mechanical contact, string modes, calibration tables, case radiation, panel response and output EQ. The copied calibration retains its Upright provenance. A distinct physical felt preparation remains future work.

The host routes direct Felt string pressure and its profile/differential drives to the dedicated Felt body. Both acoustic bodies follow tuning with separate state. At matching controls, the native Upright and Felt models produce identical samples for the same test notes and chords at 44.1, 48 and 96 kHz. The other six models match 0.2.26 native output at those rates. Full builds, SDK and regression checks passed, including engine isolation and 19 host checks. Device verification is pending.


## Felt-layer audition 0.2.28

The owner requested refinement of the copied Upright basis. A local compliant layer scales the effective contact stiffness from 0.64 toward 0.82 as touch increases. Added contact width of up to 0.004 string lengths fades with velocity. The positive contact-loss coefficient rises by up to 0.025. The reciprocal force transfer and acoustic radiation stay in the independently owned Felt implementation.

Three candidates varied the layer strength. The least aggressive retained more firm-strike upper partials and was selected for listening. In the C4 diagnostic window 10–140 ms, the 2–8 kHz share falls by 3.6 dB at velocity 35 and about 1 dB at velocity 100. These are measurements of the model, with no recorded-felt calibration or perceptual realism certification. The same 75 BPM Dusty Tones MIDI is rendered dry for the audition, matching phrase RMS between versions.

Native, both ARM builds, SDK validation, regression and 19 host checks passed. Six other engines match the baseline at three sample rates. Listening and device validation are pending. Evidence is in design/validation-0.2.28.


## Darker contact, 0.2.29

The owner requested a more muffled Felt. This candidate selects the previously screened medium layer: base stiffness factor 0.46, additional contact width 0.007 and additional loss 0.045, each depending on strike velocity. The same acoustic output and controls are retained. The dry 75 BPM audition uses the same MIDI and RMS matching as 0.2.28. Native and both ARM builds, SDK, regressions and 19 host checks passed. The six other models match native samples at three rates. Owner listening and MPC verification are pending.


The owner approved the 0.2.29 listening example on 2026-11-21. It is the accepted Felt tone baseline. MPC device verification remains pending. Next in the sequential refinement workflow is Thumb, the kalimba-family engine.
