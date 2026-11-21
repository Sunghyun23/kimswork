# Muted engine — muted guitar reference direction

## Owner requirement — 2026-11-21

Muted must be a muted-guitar voice. The owner names Hikaru Utada, Fly Me to the Moon, as the tonal reference. Preserve the approved Thumb 0.2.34 baseline and all other accepted engines. Remaining refinement order stays Clav, then Muted; this reference instruction does not start an unrequested Muted implementation or change the UI.

## Recording identity

Official artist recordings distinguish the original Fly Me To The Moon (In Other Words), on Wait & See Risk (2000), from the 2007 Mix. Original official audio: https://www.youtube.com/watch?v=ZqZ90KnIjwU . 2007 Mix / 2021 remaster official audio: https://www.youtube.com/watch?v=JW_vRrFPsDE . Apple 2007 Mix listing: https://music.apple.com/us/song/1445033670 .

Version clarification has been requested. The 2000 original is the provisional reference, not an owner-confirmed version. Metadata alone does not establish the actual guitar technique, instrument, processing or sound. Audio listening/numerical comparison has not been performed in this reference-registration turn. Do not claim matched timbre or confirmed guitar equipment from these links.

## Current code audit

MutedVoice uses 32 stiff-string harmonic modes, a pick-position initial deflection, one sinusoidal pickup/readout position, frequency-dependent empirical losses and a motion/displacement readout. The hammer is disabled. Mute currently adjusts initial height and global/modal loss; no explicit distributed palm-contact interaction is implemented. A copied comment mentions Clav and two pickups, but the actual expression is one position at 0.24 string lengths. Do not mistake that comment for two-pickup implementation or a separately modeled muted guitar.

## Modeling points to evaluate later

Target a recognizable muted string rather than only shortening a generic tone with ADSR: a finite pluck/pick release, bridge-side palm contact and frequency-dependent damping, a short pitched body, controlled residual harmonics and subtle articulation noise. Separate the instrument decay from recording-chain EQ/FX. Confirm string type, pickup/body behavior and actual reference articulation from accessible audio before choosing coefficients. Preserve existing Bite/Mute and other parameter identities unless a separate UI change is requested.

Evaluate identical dry notes across register and soft/medium/firm touch before adding production effects. Keep Muted DSP/state/calibration isolated from Clav, Thumb and the piano/EP models. Actual-device behavior, perceptual match and publication remain separate gates.


## Muted Guitar 0.2.37 — current local audition

Supersedes the initial audit above. The 32-mode stiff-string model now uses finite velocity-dependent pick width and two passive viscous contact points near the bridge. The contact positions span a short patch; resistance increases with Mute. Each contact removes modal velocity energy while preserving current displacement. An optimized two-projection update includes the cross term and matches the sequential reference to1e-12 in tested cases. Trigonometric state coefficients are cached per strike/pitch change. The existing single magnetic pickup, controls, parameter order and UI remain. This is a reduced physically informed model, not a full nonlinear hand/string/body simulation.

Primary research: Biral/d’Alessandro/Freed2014, https://speech.di.uoa.gr/ICMC-SMC-2014/images/VOL_2/1483.pdf supports bridge-side pressure-dependent palm damping, not our exact solver. Lee/Depalle/Scavone2011, https://www.music.mcgill.ca/caml/lib/exe/fetch.php?media=publications:caa2011_lee.pdf informs excitation/readout considerations; no code or recorded audio was imported. The two official Utada versions are full mixes, not isolated guitar measurements. No exact recording match is claimed.

Native/bothARM/SDK/full regressions pass; native host19PASS0WARN0FAIL. Passive projection equivalence/energy and stronger-pressure normalized-tail checks pass. All six other native engines match0.2.36 at44.1/48/96kHz, and accepted Clav/Thumb headers retain their hashes. See design/validation-0.2.37. Original92BPM dry matched auditions use velocities45–75 and one fixed gain per take, no external recordings/FX. Owner listening remains pending; no agent listening claim. MPC verification remains pending; no device install or plugin Release.

Owner separately authorized publishing all modeling approaches on the existing website with many beginner-friendly illustrations. Public article /learn/physical-modeling/ labels this engine In audition and the six accepted baselines Listening approved. It is not a Felt Club release.
