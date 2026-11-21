# Felt Club — approved implementation baseline

Current0.3.0: owner authorized factory/Dice revoice, front Noise, public release/site update and NVMe installation. Vendor kim, UIDKnAw;240parameters with the original238prefixexact,57programs. Final600fileNVMe SHAreadback and current remoteMPCXL checks are recorded in the0.3.0evidence. Historical records below do not override this current policy.

## Artwork contract

The current visual reference is [approved vintage MAIN](../../../plugins/public/FeltClub/design/approved-vintage-main.png), approved2026-11-20 with an explicit requirement to match its design. The older [MAIN with ADSR](design/approved-main-adsr.png) is historical. Earlier brown Physical Keys, green Physical Keys, and Felt Club Touch/Resonance mockups are historical. Preserve faded olive green, aged ivory, restrained burnt-orange active states, engraved ivory value fields, reference-derived serif branding, native-size black ridged bakelite knobs, transparent upper-left locks, generous four chord pads and common footer. No artist/manufacturer logo on the skin. The concept is not an executable skin; transform only its plugin body into native1280×628 while the MPC owns its110px header/62px function-key bar. Do not copy the concept's host header into runtime.

MAIN fixed4×4SCREEN and Program Q-Link order:

| Group | Controls |
|---|---|
| A CHARACTER | Tone, Body, model Character3, Noise |
| B AMP ENVELOPE | Attack, Decay, Sustain, Release |
| C MOTION | Rate, Depth, Drift, Width |
| D COLOR/FX | Drive, Chorus, Delay, Room |

A3/A4labels: TineEP=Bell/Bark, WurliEP=Reed/Bark, Piano=String/Board, Pluck=Bite/Mute. Every group has four identically sized knobs and a separate44×44lock surface. Motion's TREM/PANandSYNCtouch choices stay to the right of its title, not over captions. Header family selection EP/PIANO/PLUCKand separate model choice. Explicit selection changes model; navigating a page must never change model or sound settings.

Sound-preset previous/next and model browser on the top row. CHORD PRESETprevious/next, one EDITentry and70px chord pads below the groups. UNLOCK ALL/ROLL DICE/UNDOuse shared footer x16/188/440,y578,h44. Slot-page arrows on its right. Six function keys: MAIN/CHORD, MODEL, TOUCH, MOTION, FX, SETUP. First key toggles MAIN/CHORD; no startup messages or extra edit buttons on individual chord pads.

## Sound and playing

Independent physically informed modal engines, no commercial samples: Tine/Wurli(Reed)EP, Upright/FeltPiano, Clav/Muted/ThumbPluck family. Clav is a struck-string instrument grouped functionally with short rhythmic sounds; do not describe its excitation as identical to a guitar pluck. Source models must have physically related resonant modes, excitation/hammer or pluck-position response, natural frequency-dependent damping, and acoustic radiation/pickup behavior. ADSRshapes the naturally decaying output and must not supply indefinite energy at Sustain. Piano realism/calibration, CPU, hardware and physical-input validation must be reported honestly.

Velocity/hammer hardness, resonance/pedal and key-off/mechanical noise live in TOUCH/MODEL detail, not MAINB. Family/model-specific state retains its meaning when tabs change. Smooth audible changes; bounded levels; MIDI keyboard, velocity, sustain, pitch bend and panic. Separate ownership for direct MIDI and held chord sources; no stuck notes on overlap, page/slot-count changes, state load or sustain changes.

Chord features mirror the explicitly approved functional direction:4default/16maximumslots, one shared editor, key/transpose, chord types/inversions/octave, top-note-minimizing voicing, Strum and optional chord glide, user-adjustable MIDI mapping/learn without changing host pad layout. Touch chords play only while held; release honors AmpRelease. Physical pad triggers follow assigned notes. Tabs do not transpose/revoice or switch engines implicitly.

Dice generates sound only; chord/voicing/key/slot count/MIDIlearn state are preserved. Honor per-value locks, exact Undo and UnlockAll. Preserve Sync on sound randomisation; support straight/dotted/triplet rates without resetting fractional Q-Link movement. Factory presets original, categorized for internal browsing; Init plus musical EP/Piano/Pluck sounds. Do not promise category-first behavior in host-owned preset UI unless verified.

## Validation and device policy

Preserve every existing project and uncommitted Donutwork. Use pinned unmodifiedSDK. Native/ARM32/ARM64builds and SDKvalidation, meaningful audio/MIDI/state/bridge/Q-Link/layout tests and actual comparison with approved art. Only official SDKmpc_remote.pyfor NVMeinstallation; SDreads/writes and SSDoperations remain prohibited. Preserve all19Groove owner files. No existing runtime replacement needed for this independent plugin. Do not publish a mockup as an actual screenshot.

Physical modeling references: [Bank et al., research overview](https://home.mit.bme.hu/~bank/publist/jasp03.pdf), [AudioLabs modal piano study](https://audiolabs-erlangen.de/resources/MIR/2023-ISMIR-PianoSynth/). Independent implementation; no sample/IR/code from these studies is bundled.

2026-11-20 user explicitly requested Wurlitzer-style EP. Keep stable Model1 identity and expose WURLI in the EP model selector, with eight original Wurli presets. Independently implement short reed high-mode damping and velocity-dependent asymmetric pickup/bark; do not claim measured/calibrated hardware equivalence. MAIN layout stays unchanged.

Implementation contract: the approved MAIN EDIT opens a quick touch editor; its modal hides native sound guides and masks MAIN Screen relay writes so hidden sound controls cannot change. The first native MAIN/CHORD function key provides the same editor with sixteen properly mapped/outlined chord Screen Q-Links. The pinned SDK has no documented arbitrary native-tab jump; do not invent one or silently remap serialized sound parameters into chord fields. Model/sound values survive tab changes.

2026-11-20 latest user override: move SOUND PRESET previous/browser/next to the common bottom action row on all pages (x586/638/950,y578;44/304/44px), after UNDO and before slot paging. Keep chord preset/EDIT/pads in place. Move the existing family/model row to the vacated top-left selector area. This is an explicit layout exception to the approved artwork; retain the approved materials and typography elsewhere.

## Engine and dedicated knob revision0.2.0 — 2026-11-20

User requested all models become less artificial, with Lounge Lizard as a quality target, and explicitly replaced screen-cropped knobs with a separately made asset. Keep the approved layout/footer and all parameters. Dedicated transparent72pxface,97pointer-only frames and unchanged lighting are mandatory. Body/FX changes cannot substitute for better excitation/resonator/pickup behavior.

Implemented reduced-order physics now covers unilateral felt contact; reciprocal tine/tone-bar coupling; different magnetic and electrostatic pickup approximations; register-dependent hammer mass/stiffness; piano unison strings/shared driven body/approximate pedal-open bank; physically deflected string and cantilever release; modal damper losses. Revoice57factory sounds to avoid slow attacks/large drift/heavy effects masking the source. No commercial samples, recorded IRs, AAS code or proprietary model extraction. Existing238IDs/order and UIDKnAw stay stable.

Research background: [Pfeifle, DAFx2017](https://www.dafx.de/paper-archive/2017/papers/DAFx17_paper_79.pdf) discusses hammer/resonator/pickup components for electric pianos; [Bank & Sujbert, DAFx2002](https://dafx.de/papers/DAFX02_Bank_Sujbert_piano_synthesis.pdf) describes nonlinear hammer interaction. Our finite modal reduction/coefficients are independently implemented, not a reproduction of their full solvers or measured data. [AAS official description](https://www.applied-acoustics.com/lounge-lizard-ep-5/in-depth/) is the user's quality reference only. Do not promise matching its realism without controlled listening and reference measurements.

2026-11-20 Tine-only0.2.1 candidate: selected round2 of3 against EQ-processed real Rhodes reference;20heldout examples improve weighted spectral/envelope fit180.1569→24.0017. Reference audio private-only, never bundled. Twelve global physics coefficients plus level/register balance; runtime constexpr, mutable calibration only in offline bridge. Six other model renders bit-identical;238parameters/UID/UI preserved. BothARM/native/SDK/full regression/host19PASS0WARN0FAIL. NVMe597files verified; restart/device validation pending. No public publication or Lounge Lizard equivalence claim. See design/validation-0.2.1 and design/audio-0.2.1.

2026-11-21 user requested full-model clarity0.2.2. All7models: unified strike/held Tone law,5ms smoothing, variable second pole, model-sensitive bandwidth; shorter hammer contact/EP upper modes; Muted motion pickup/Thumb approximate motion radiation and balanced gains. Preserve approved UI,238params/UID/57names.0.2.1Rhodes fit scores are historical; no commercial-equivalence claim for0.2.2. Proof and original matched audio under design/validation-0.2.2 and audio-0.2.2. NVMe final install/readback and restart approval tracked there; do not touch SD/SSD/other projects.

2026-11-21 further-improvement0.2.3: bounded Wurli/piano/Clav register gain; true initial displacement seeds Muted/Thumb pickup derivative (no first-sample DC impulse); less double Release attenuation; voice reuse clears noise history;20Hz sample-rate-independent output DC blocker. UI/UID/238params/57names preserved. Evidence and matched7model audio in design/validation-0.2.3 and audio-0.2.3. Native/bothARM/SDK/full regression/host passed; NVMe readback status tracked in evidence, restart approval still pending. Other projects/SD/SSD untouched; no public release or perceptual equivalence claim.

2026-11-21 latest reference override: Keyscape YouTube demos only; no direct rendering of installed Keyscape. Canceled real-room/sample fit; runtime remains0.2.3, no fitted profile applied. Official reference metadata and audio-access limits recorded in mpc-software/docs/plugins/felt-club/keyscape-reference-status.md. Do not claim actual listening/audio analysis from video descriptions; playback access is unavailable. Await accessible reference audio for measured comparison.

2026-11-21 superseding audio-access limitation: public YouTube download + local PCM decode succeeded; actual official demos analyzed, no direct Keyscape render.0.2.4accepts Wurli/Upright/Felt global pickup/modal readout profiles after >=10%training+heldout spectrum-proxy improvement. Tine/Clav failed and remain0.2.3, with Muted/Thumb; four model renders byte-identical. Unknown MIDI/velocity/age/FX means no direct A/B or perceptual/Keyscape-equivalence claim. Evidence/provenance/own audio in design/validation-0.2.4 and audio-0.2.4. Native/bothARM/SDK/full regression/host passed. UI/238IDs/UID/57names unchanged. NVMe readback tracked there; restart pending current Custom work permission. No public release/push/site publish.

2026-11-21 user-approved research application0.2.5: Tine-only mass-normalized exact linear tone-bar coupling, separate vertical/horizontal flux readout with axial curvature, dissipative contact and separate register gain. No full nonlinear beam/globalPHS/perceptual-equivalence claim. Six other native model renders bit-identical;UI/238IDs/UID/57names unchanged. New Tine diagnostics include inharmonic bell/decay; old harmonic-only optimizer rejects--model0. Native/bothARM/SDK/full regressions/host19PASS0WARN0FAIL; extra coupling-energy/2Dflux/contact tests. Evidence/audio in design/validation-0.2.5 and audio-0.2.5. NVMe readback tracked there; restart still pending Custom-work permission. SD/SSD/other projects untouched;no public release or Keyscape render.


## Roll Dice, 0.2.25

Roll Dice retains the selected model and velocity curve. Each of the seven engines has its own ranges for Tone, Body, model character, Hammer, Release and Detune. Tine, Wurli and Upright use ranges centered on the separately developed engine responses. Attack remains short and velocity sensitivity stays at 60–75%. Synced motion rates select actual straight, dotted or triplet divisions.

Body and Release are drawn together. More Body lowers the maximum generated Release, using the actual Body value when it is locked. One main color effect is drawn from dry, Drive, Chorus, Delay and Room. If any of those effects is locked at a nonzero value, the remaining unlocked color effects are zeroed. Locked controls retain the owner's values, including settings outside the generation ranges. Existing sound locks, Sync retention, chord settings and exact Undo apply.

The bounded ranges reduce extreme combinations. Musical preference remains a listening decision. Native and both ARM builds, SDK validation and full regression passed. Tests exercised 336 locked rolls with exact Undo, and 84 audible unlocked rolls across seven engines. The SDK host returned 19 passes. MPC device performance and physical controls are pending.


## Current0.3.0 Noise/identity policy

MAIN A4 is continuous tape-like Noise(Hiss238), with its own lock239. Character4(Bark/Board/Mute) remains MODEL and all values are retained. Mechanical remains TOUCH. C4Width stays in place. Prefix238declarations and IDs/orderexact; bothnewvaluesdefault0onoldstate restore. MAIN Screen relay3 and Program A4 bothtargetHiss; CHORD masks hiddenMAIN relays and retains its fourRoot/Type/Voicing/Octaverows. Enginecode/state/calibration remainindependent and unchanged. Noise0audioexact at3ratesfor7engines. Generatedcaptionheightmatchesprintedlegends, and standaloneCHORDSTRUM/GLIDEcaptionsarelightforcontrast. Widerorganizationproposal isdesignonly, notapproved/notruntime.

Factory57names/indicesarefixed; newexplicitrecipesin factory-sounds.json. Dice profilesinSoundProfiles.h retainselectedengine/Sync/velocitycurve/performance/chords, all38locks, pairedBody/Release, onegeneratedcolor and occasionalsubtleHiss. Undo restores38soundcontrols andDiceRNG while retaininglocks.0.3.0current verification/publicationstatus is in implementation.md and plugin design/{device,validation}-0.3.0.
