# Seven independent Felt Club engines —0.2.8

User explicitly required all seven physical instruments to be isolated, not only Tine/Wurli. This is an architecture refactor and isolation fix. It does not accept or improve the previously rejected Wurli0.2.7sound. Approved Tine0.2.6voicing coefficients are retained; compare numerical limits below. No installation, restart, push or public/site release was requested/performed.

## Ownership

| Model | Independent source directory | Owned DSP |
|---|---|---|
| Tine |src/engines/tine/|TineVoice/Core/Voicing/Mechanics/Profile|
| Wurli |src/engines/wurli/|WurliVoice/Core/Voicing/Mechanics|
| Upright |src/engines/upright/|UprightVoice/Core/Voicing/Body|
| Felt |src/engines/felt/|FeltVoice/Core/Voicing/Body|
| Clav |src/engines/clav/|ClavVoice/Core/Voicing|
| Muted |src/engines/muted/|MutedVoice/Core/Voicing|
| Thumb |src/engines/thumb/|ThumbVoice/Core/Voicing|

Each backend owns its hammer and resonator types/implementation, pickup/radiation computation, tone/circuit filters, ADSR instance, strike/release/damping law, noise state, calibration coefficients and live voice settings. No backend includes another backend header. Physically similar Mode/Hammer implementations are deliberately independent copies, so changing one model cannot silently retune another. Runtime calibration is constexpr per backend; independent mutable calibration switches exist only for offline tools/tests. There is no common model-indexed voicing/calibration table.

VoiceTypes.h and VoicingTypes.h are only the parameter/state schema and stateless numeric helpers. They contain no per-model physical coefficients or shared mutable audio storage. The SDK remains pinned/unmodified; common SDK low-level algorithms such as ADSR are instantiated with separate state in each backend. This is code/state isolation, not separate OS processes, plugin UIDs or seven independent plugins.

PhysicalVoice.h is a fixed-storage std::variant dispatcher. It selects/prepares one backend without heap allocation and exposes that backend's own VoiceState to MIDI ownership/voice allocation. There is no mirrored master voice state. A stolen/reselected voice is constructed/prepared in place; exact fresh-instance equivalence is tested at three rates. External owner reassignment for chord glide writes the selected engine's state directly.

Upright and Felt own independent soundboard/sympathetic classes. The host routes bridge sends by originating voice model and sums their final outputs. Non-piano models send no soundboard energy. Audio from different physical instruments no longer excites one common piano body.

## Host boundary and intentional global controls

UI, stable parameter IDs, preset/chord/MIDI routing, voice allocation and user-facing global FX remain in FeltClub. This request does not introduce per-model stored UI knob banks. MIDI note-off/pedal/stop are intentional host performance messages; global tuning and post-mix FX intentionally apply to the plugin output. Those shared features must not be confused with shared internal physical simulation state.

A real isolation bug was also corrected: global live Tone/ADSR edits formerly updated every active voice even if it originated from another model. Writes now target only voices whose backend matches the selected model; selecting a model invalidates the live settings comparison so its current controls apply correctly. Another model's held/release voices preserve their own tone/envelope. Existing voice owners are still releasable. No UI widgets/locks/Q-Link mapping changed.

## Evidence and limits

- Native/ARM32/ARM64 build, SDK0errors/0warnings; full engine/VST/physical/UI tests and host19PASS/0WARN/0FAIL.
- Seven standalone backend translation units compile with no cross-engine dependencies; seven independent Mode/Hammer types.
-49profile mutation comparisons: exactly the targeted model changes, other six outputs are bit-identical.
-42ordered engine pairs: interleaved strikes/filter/envelope/glide/release do not alter an independently held engine. Fresh-instance reuse across seven models and three rates, and separate piano-body state pass.
-7392all-model/key/velocity/sample-rate finite strikes and RT allocation guard, MIDI/CC/state/hold/strum/glide/factory regressions pass.
-21actual native pre/post float comparisons:7models×44.1/48/96kHz, same notes/velocities/chords/dry settings. Upright/Felt/Clav/Muted/Thumb are sample-identical. Tine/Wurli have tiny floating-point differences from specialization: maximum2.105e-7and8.941e-8full scale, respectively; relative RMS errors below-129.97/-131.45dB. Do not call those two bit-identical or claim new auditory certification. Fixed acceptance bounds are peak<2e-6and relative RMS<-90dB; all pass.
-591runtime skin files match prior0.2.5hashes. UID KnAw,238parameter IDs/order and57programs retained; both offline calibration bridges compile after relocation.

Evidence: plugins/public/FeltClub/design/validation-0.2.8. Reproduce old/new float comparisons with tools/compare_engine_refactor.py --before <saved-native-library> --after <current-native-library> --out <report.json>. That tool discards only its own temporary float captures. Full checks: tools/build.py all then tools/test.py.

The previous native0.2.7is preserved at /private/tmp/felt-engine-separation-baseline/felt-club.dylib for local comparison, not bundled. Device files remain0.2.5with runtime restart validation pending;0.2.6–0.2.8have not been installed. SD/SSD untouched. Desktop host timing is not an MPC CPU test.

## Maintenance rule

For future model tuning, edit only that model's engine directory and its dedicated tests/reference evidence. Do not reintroduce a shared tunable hammer/resonator/pickup/profile implementation or silently promote unapproved Wurli coefficients. Any intentional change to shared host/global FX or stable schema requires full cross-model regression. Preserve the approved Tine reference and user-facing visual design.


2026-11-21 Upright0.2.12 expands only its own fixed modal storage and owns the64mode main-radiation backend/passive unison projection. Voice variant memory grows but no other backend coefficients or state change. Six other source trees and three-rate native output samples remain exact versus0.2.11; no shared physics introduced. See validation-0.2.12/preservation.json.
