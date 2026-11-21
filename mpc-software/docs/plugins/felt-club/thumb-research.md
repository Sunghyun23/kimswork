# Thumb / kalimba physical-modeling study — 0.2.30

## Decision

Keep Thumb independent. Copying Upright would carry struck strings, hammer contact and piano soundboard behavior into an instrument excited by releasing a bent metal lamella. The existing Thumb already had pluck-state excitation and therefore provides the appropriate starting point. The owner-approved Tine, Wurli, Upright and Felt tones are preserved.

## Primary research

- David M. F. Chapman, *The tones of the kalimba (African thumb piano)*, JASA 131 (2012), 945–950, DOI 10.1121/1.3651090. [Abstract](https://pubmed.ncbi.nlm.nih.gov/22280717/). Uses an Euler–Bernoulli beam with one clamped end, an intermediate simple support and a free end; the support position affects modal ratios.
- Chapman, *Characterizing the sound of an African thumb piano (kalimba)*, Acoustics 2008. [Conference abstract](https://dael.euracoustics.org/confs/acoustics2008/data/articles/000215.pdf). Reports first-overtone ratios of 5.3–5.9 for its measured tine geometries.
- L. E. McNeil and S. Mitran, *Vibrational frequencies and tuning of the African mbira*, JASA 123 (2008), 1169–1178, DOI 10.1121/1.2828063. [Abstract](https://pubmed.ncbi.nlm.nih.gov/18247916/). Their measured prominent overtones are approximately 5 and 14 times the fundamental; transverse modes dominate radiation. This different mbira geometry is not our calibration target.

Only these accessible abstracts were used; no claim to have reproduced the papers' complete models or recordings. No reference sample is bundled.

## Implemented model

An original offline uniform beam calculation uses 32 Euler–Bernoulli elements with consistent mass, a clamp at node 0, displacement-only support at node 17, and a free node 32. Six mass-normalized eigenmodes provide frequency ratios and separate tip/readout and near-tip/pluck shapes. Eigenvectors are normalized relative to the fundamental for runtime lookup. The first overtone is 5.56695666 times the fundamental. The additional 8.44563163 mode and later modes follow this chosen beam geometry; do not conflate these with the two prominent measured mbira overtones.

Runtime initialization models the released static deflection: inverse modal stiffness, signed pluck shape and a velocity-dependent upper-mode rolloff. Bell and Mute still shape modal output and losses. Three empirical short resonances at 190, 575 and 1410 Hz add a small body transient; they stay fixed during pitch bends. Tines follow pitch. Existing DC blocking, smoothed Tone, ADSR and release handling remain.

The generator is tools/calibration/thumb_beam.py; run without arguments to check the shipped table, or with --write to regenerate it. NumPy is only an offline development dependency. Runtime uses fixed storage and constexpr values.

## Scope and limitations

The 17/32 support, uniform cross-section, decay laws, body frequencies and levels are original design assumptions, not a measured instrument fit. Body modes are separately excited approximations rather than reciprocal beam–body–air coupling. The mixed motion/displacement readout remains approximate radiation. Agreement of one overtone ratio with a literature range does not establish acoustic realism.

The UI, factory names/order, UID and 238 parameter identities remain unchanged. Native float comparison at 44.1, 48 and 96 kHz confirms all six other models sample-identical to 0.2.29. Tests verify supported-beam pitch changes while body poles stay fixed, reset to silence, finite full-register output, engine isolation and existing state/lock behavior. See design/validation-0.2.30 and the matched original audio in design/audio-0.2.30. Owner listening and MPC checks remain pending; no device installation or public release was performed.


## Owner rejection and new reference — 0.2.31

The owner rejected 0.2.30 as cheap sounding and clarified the desired reference as Nas **One Love**, not Represent. Its sampled source is The Heath Brothers **Smilin’ Billy Suite Part II**, featuring Stanley Cowell's mbira/kalimba. The Represent theatre-organ direction is canceled. [Library of Congress essay](https://memory.loc.gov/static/programs/national-recording-preservation-board/documents/Illmatic_McDuffie.pdf), [Percy Heath oral history](https://amhistory.si.edu/jazz/Heath-Percy/Heath_Percy_Interview_Transcription.pdf).

The official [Strata-East public preview](https://strata-east.bandcamp.com/track/smilin-billy-suite-part-ii-feat-stanley-cowell) was obtained and decoded locally. Actual spectral/envelope measurements, not a video-description inference, guide this candidate. The first E4 at approximately 0.08 seconds was used for relative bands and decay; C5 at 0.44 seconds anchors register-dependent loss; B4 at 0.80 seconds is a held-out decay check. The reference has an unknown performance and recording chain and becomes an ensemble recording. Only its first few exposed notes inform these proxies. Source audio is private temporary analysis material, never a runtime asset or public audition.

### Changes

- Change the original uniform-beam support from 17/32 to 12/32, giving a first higher bending mode at 6.66664531 times f0. This aligns an observed E4 overtone region; support position is an inferred design geometry, not a measured dimension.
- Reduce upper-mode strength to `0.3*(0.6+Character3)*sqrt(ratio)` times the signed tip shape and use a lower Thumb-only Tone ceiling of 5 kHz.
- Replace the long global loss law with a shorter fundamental-led decay. A continuous bounded register multiplier, exp2(clamp((note−64)*0.27,−0.4,2.2)), lets upper keys ring longer. Body, Mute and Resonance still affect loss; ADSR stays under user control.
- Increase the three existing fixed body transients by 10 times and set their T60 to 0.16+0.10*Body seconds. These are empirical resonances, not a calibrated coupled soundboard.
- Add a bounded asymmetric readout term `0.24*x*x/(1+x*x)` before the existing filter/DC blocker. This supplies weak additional harmonics; it is an empirical bridge/readout surrogate, not a solved nonlinear contact model.

### Evidence and limits

At the matched raw-engine controls, fundamental-band decay over the analyzed early/late windows is −10.69 dB in the reference E4, −6.72 in 0.2.30, and −11.04 in this candidate. C5 is −2.21 / −6.72 / −2.71 dB, respectively; held-out B4 is −2.50 / −6.72 / −3.21 dB. These are recording-specific proxy checks. The E4 four-feature training error drops from 751.36 to 0.17 dB²; that figure is in-sample and must not be described as generalization or acoustic equivalence. The 2.6–5 kHz feature was excluded from fitting as low-level broadband/recording content and remains different. No stereo separation or reference-audio listening by the model is claimed.

Native, ARM32, ARM64 and SDK validation pass; the full suite includes 88-key stability, held decay/touch, state/locks/chords, engine isolation and host 19 PASS. Final three-rate native preservation is recorded in design/validation-0.2.31. UI, 238 parameter identities, UID and 57 program names remain unchanged. Original paired dry audio and a lightly effected original E minor phrase are in design/audio-0.2.31. Owner listening and MPC verification remain pending; no installation/publication.


## 0.2.31 rejected; time-resolved revision 0.2.32

Owner rejected the 0.2.31 audition. Matching four broad bands over 35–105 ms and a short envelope window missed a decisive time structure. Its in-sample error is not a tone-quality acceptance gate.

The corrected audit additionally obtains the official Apple preview of Nas One Love from the Illmatic album (not the remix). Its spectrum includes vocals, bass and drums, so no instrument-isolated fitting is claimed. The exposed original Strata-East E4 is analyzed with a lowpassed/decimated matrix-pencil modal estimate at several windows and ranks. Across fits, its dominant fundamental is about 330 Hz, a nearby weaker component about 354 Hz, and strong but fast high components around 2.2 kHz. Estimated high-component T60 varies substantially with window/rank (roughly 0.06–0.14 seconds near onset, with weaker later components). These are recording-derived observations, not verified 3D mechanical modes.

### Revised runtime

Keep the six beam modes and three fixed body modes. Add two empirical pitch-following modal components: a weak 1.075× adjacent resonance and a strong, short 6.78× metal contact/radiation path. Max storage is 12 modes, active count 11; all state remains owned by Thumb. The original first higher bending mode keeps a weaker, longer ring. Thus the attack and tail are separate instead of being forced to share one damping coefficient. Higher attack modes shorten with register. Reduce the previous blind body boost from 10× to 2×; remove the asymmetric polynomial readout coloration. Fundamental/register losses, UI, MIDI/ADSR, identity and parameters are preserved.

This remains physically informed modal synthesis with empirically inferred extensions. Neither the 1.075× component nor the contact path is a calibrated solution of a real instrument's geometry. Recorded reference waveforms are not runtime assets.

### Time-window check

For the E4 opening, the 1.9–2.5 kHz versus 250–410 Hz band ratio is measured in 8–25 ms and 120–160 ms windows. Reference: −11.86 / −26.77 dB. Rejected 0.2.31: −25.46 / −28.77 dB. New 0.2.32: −12.94 / −26.42 dB. This supports the correction of early/late metal balance, not full waveform or perceptual equivalence. Source performance, timing, velocity, microphone and mastering are unknown. A first attempt that shortened the entire high component lost too much tail and was discarded before delivery; the final candidate retains the weak long component.

Full native/both-ARM builds and SDK validation pass. Full regression includes 88-key finite-output checks, short metal-attack energy loss, touch/natural decay, isolated state, locks/chords and host 19 PASS. The six other engines are sample-identical versus 0.2.31 at three rates. A brief five-note dry comparison uses the same controls, register and fixed overall RMS, avoiding the previous unrelated composition and FX differences. Evidence and original synthesized auditions are under design/validation-0.2.32 and design/audio-0.2.32. Owner listening and MPC checks pending; no install/publication.


## Softer fast-path refinement — 0.2.33

The owner finds 0.2.32 closer and asks for a softer refinement. Preserve its geometry, adjacent resonance, fundamental/register losses and weak longer metal tail. This is not an instruction to replace the tone or dull its entire bandwidth.

Only the fast metal radiation path changes: gain 4.3 becomes 3.8*(0.82+0.18*velocity), and an exponential rise uses 0.9+1.2*(1−velocity) milliseconds. Firm hits retain more bite than light ones. This is a bounded empirical readout treatment motivated by a finite release, not a solved finger-contact model or changed beam geometry. No new UI/parameters/presets/modes or global Tone EQ.

The ramp acts on the fast path after displacement differentiation. Its own previous displacement is seeded at strike and cleared at stop/reuse. Ramping displacement before differentiation would inject a false start velocity; the existing first-sample impulse test caught that intermediate approach. The delivered implementation passes it and a new three-rate rise-duration/reset check.

For the matched E4 excerpt, early metal-to-fundamental band energy becomes −15.02 dB from −12.94 dB, approximately 2.09 dB softer in the 8–25 ms window. The 120–160 ms ratio stays −26.42 dB (change below 0.001 dB). This supports a fast-path-only refinement, not full perceptual certification. Evidence and the same dry notes/control/RMS comparison are under design/validation-0.2.33 and design/audio-0.2.33.

Native/both ARM/SDK and full regression pass, including host 19 PASS/0 warnings/0 failures. Six other native engines are sample-identical at 44.1/48/96 kHz. UI/238 IDs/57 names/UID remain fixed. Owner listening and MPC checks pending; no installation/publication.


## Tail and touch refinement — 0.2.34

The owner asks for further improvement after 0.2.33. A source audit finds the weak longer metal mode still inherits the fundamental's large upper-register decay multiplier. This makes high notes keep a high metallic ring even though the softened short attack is correct. Separate this loss behavior while preserving the fundamental and short radiation path.

Four offline variants (baseline, tail-only, tail+register, tail+register+touch) are screened at four keys and three velocities. The selected final variant preserves isolated fundamental late energy and fast metal attack energy exactly in all 12 cases. No auditory ranking is claimed. Candidate sources, a reproducible probe and candidate-audit.json are in design/validation-0.2.34.

Changes are confined to Thumb:

- The long upper mode uses min(fundamentalRegisterDecay, exp2(clamp((64−note)/24,−1,0))) instead of automatically inheriting the increasingly long high-key fundamental. E4 and below retain their existing modal loss; upper metal settles sooner. At C5/velocity72, its isolated 250–550 ms modal energy is 27.96 dB lower. This is one weak component, not total output level or all high-frequency energy. The high fundamental continues to sustain.
- The weak adjacent empirical resonance becomes 1+0.075*exp2(clamp((64−note)/48,−0.5,0.35)). E4 remains 1.075× and C5 becomes 1.0668174×, reducing the mechanically repeated constant-ratio grain across the upper register. This is bounded empirical voicing, not a measured geometry or calibrated 3D torsion model.
- Its amplitude is multiplied by 0.72+0.28*sqrt(velocity), giving light touches a little less grain while preserving full-velocity strength.

The earlier 0.9–2.1 ms fast-path radiation rise, source geometry, fundamental pitch/decay, fixed body resonances, Tone, ADSR and master effects remain. No random timing/pitch, UI, preset or parameter additions. The new three-rate test checks that high fundamentals sustain while upper metal settles sooner and adjacent grain follows touch. Native/both-ARM builds, SDK/full regression and host19PASS/0WARN/0FAIL pass. Six other native engines are sample-identical at three rates; UID/238IDs/57names remain fixed. The same dry five-note/RMS comparison is in design/audio-0.2.34. Owner listening and MPC checks pending; no installation/publication.


## Owner-approved Thumb baseline

On 2026-11-21 the owner confirmed the delivered 0.2.34 tone as final. This supersedes pending owner-listening status for that version. Preserve its independent engine and sound while working on other models. Header snapshots and hashes are in design/validation-0.2.34/accepted-source/manifest.json. Hardware validation remains pending; approval did not request installation or publication. Remaining engine refinement order is Clav, then Muted.
