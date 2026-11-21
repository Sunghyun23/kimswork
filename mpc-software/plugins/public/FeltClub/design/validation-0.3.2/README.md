# Felt Club0.3.2 — CPU optimization

The final build is installed on internal NVMe; all600files match SHA-256. The owner accepted pad/startup behavior but later reported MPC CPU peaks above100%. Exact engine/workload was not isolated. CPU overload remains unresolved; desktop improvement measurements below are not hardware certification. The owner requested publishing as-is and ending optimization.

## Changes

- Each independent piano body skips only its virgin zero acoustic state. Pedal damping history advances with one scalar recurrence and is restored to the resonators on first excitation. Once excited it keeps running, preserving its complete tail and later engine changes. No arbitrary tail truncation or sleeping active resonators.
- All seven independent modal backends cache identical pitch coefficients; configure invalidates the cache. Tine's coupled tone-bar coefficients use the same cache policy. Drift, bend, tuning, glide and Clav release pitch continue to recalculate when they change.
- Piano unison coupling algebra eliminates reconstructing/cancelling the same quadrature. Cached velocity/return factors replace repeated divisions; damped modes retain their division. The normalized-energy passivity regression still passes.
- After contact and attack build-up finish, one/two/three-string groups are advanced locally in order. No mode count, maximum24voices, hammer solver, oversampling or physical profile was reduced. Original path remains for contact/attack and partial Nyquist-trimmed groups.
- The192entry strum queue is scanned when the next event is due, instead of every sample. Cancellations, offset order, chord glide and sustain retain the same behavior.
- Virgin unused Delay/Room skip processing until first use; once used they continue rendering tails even with zero send. Dry Chorus keeps recording input history and advances phase, while skipping its unused wet read.

FactorySounds.h and the full246parameter declarations match0.3.1byte-for-byte. UID, preset names/values, UI and per-model isolation remain unchanged. Seven own engine implementations remain independent; only mechanical-equivalent arithmetic and redundant work changed.

## Validation

Native/ARM32/ARM64 builds and unmodified pinned SDK validation:0errors/0warnings. Complete tools/test.py including the new CPU invariants passes, with host19PASS/0WARN/0FAIL. Coverage includes exact idle-body wake/pedal/tuning history at3rates, seven pitch-cache invalidations, Chorus history/FX on-off tails, factory velocity bounds, energy/contact/damper/isolation/voices/strum/state/RT allocation and seven-page skin checks.

77matched audio cases compare the pre-optimization source with the candidate:35basic scenes and42dynamic scenes at44.1/48/96kHz. Dynamic scenes include Drift/bend, pedal, Strum/Chord Glide, Tone edits, delayed FX enabling/off/on, model changes,30-note stealing and panic with variable32/97/128/511frame blocks. Maximum absolute error is below1e-6FS; differing cases have over100dB signal/error ratio. The two piano models have tiny floating-point differences from algebraic reassociation; do not claim bit-identical piano audio. Performance was not obtained by reducing voice/mode counts or changing factory timbre.

Mac arm64 clang-O3 timing is evidence of native cost reduction, not MPC CPU percentages or a hardware worst-case guarantee. For six-note musical FX/Drift9%/pedal scenes, the measured processing-time reductions are Tine60.12%, Wurli58.25%, Upright26.12%, Felt24.73%, Clav54.65%, Muted37.23%, Thumb67.37%. Twelve-note figures and exact raw timing conditions are in musical-cpu.json. Idle/dry scenes are in cpu-comparison.json. Benchmarks use the best of3runs/1200blocks after200warmup blocks,48kHz/128frames. Thermal/frequency effects and hardware architecture can change these percentages.

No new hardware audio/CPU, every-model MPC stress, or all saved project recall claim is made before the owner tests it.

## Reproduction

Run tools/build.py all and tools/test.py. Native tools/cpu_benchmark.cpp emits35scenes plus raw floating-point audio to an existing output folder; tools/cpu_audio_audit.cpp emits42dynamic comparisons. Compile with the SDK framework include path and-O3; FELT_CPU_SOURCE may point to a separately retained baseline source tree for before/after comparison. Benchmark PCM and the pre-install rollback ZIP are private scratch files under/private/tmp/felt-cpu and are not in public packages. See cpu-comparison.json, musical-cpu.json, audio-comparison.json and installation.json.
