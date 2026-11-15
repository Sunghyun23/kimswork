# Moog-inspired engine — 1.2.0

## Confirmed scope

The user requested an engine redesign rather than preset-only voicing, with no added mode or UI. The new default voice path is two band-limited oscillators plus sub/noise/optional phase modulation → nonlinear oscillator summer → four-stage nonlinear ladder low-pass → amp envelope / velocity / rounded VCA → existing drive, tremolo, chorus, delay, low-cut and master. Oscillator wave generation still uses the pinned SDK PolyBLEP/wavetable components. Phase is retained between note retriggers; this is not continuously running idle VCO simulation. Drift stays explicitly controlled.

## Filter implementation

`AnalogLadder.h` is original MIT-licensed project code. Each of four buffered stages uses the differential-pair form `dy/dt = w * (sat(previous) - sat(y))`; the first input includes negative feedback from stage four. Trapezoidal integrators and a bounded simultaneous Newton solve (at least one, at most six iterations per substep) avoid an explicit sample delay in the resonance loop. Cutoff prewarping is evaluated at control rate. A rational, bounded saturation and its exact derivative are evaluated without per-sample transcendental calls.

The ladder runs at twice the host rate with linear input interpolation and a 17-tap Blackman halfband decimator. This reduces nonlinear aliasing; it does not establish alias-free operation. Mixer and VCA saturation run at host rate. At low signal/resonance the four poles give an asymptotic 24 dB/octave response. The displayed cutoff sets each pole: the complete cascade is about -12 dB at that frequency at zero resonance, not a calibrated -3 dB point. Resonance is mapped to loop gain 0..3.95, below the small-signal self-oscillation threshold. Increasing it reduces low-frequency gain; no full bass-loss compensation is applied.

Existing Band-pass/High-pass/Notch choices remain SDK 12 dB SVF extensions. They are not described as transistor-ladder modes. Final Low-cut remains the independent stereo cleanup filter. FX algorithms and envelopes are retained. There is no hidden legacy switch and no additional engine selector.

## Identity, migration and sound changes

UID Krql, all 208 parameter declarations/order/ranges, UI and Q-Link mappings remain unchanged. Saved settings load, but their sound changes through the new mixer/filter/VCA. This is parameter/state compatibility, not sound-identical compatibility. Keep 1.1.2 and render old tracks before updating if preserving their exact sound is necessary. Init retains its parameter recipe with Low-cut/FX off; intrinsic engine coloration still applies.

All 63 playable factory recipes are refitted for the new filter operating points; names, categories and program order stay fixed. Cutoff/resonance/envelope excursions and Tone/Drive are revised, and Sound Trim is remeasured from stereo renders. Presets are inputs to the new engine, not its implementation.

## Research and licensing

The physical differential-pair principle is informed by Antti Huovilainen, [Non-Linear Digital Implementation of the Moog Ladder Filter, DAFx2004](https://dafx.de/paper-archive/2004/P_061.PDF), particularly the cascaded nonlinear stage equations. Our solver uses trapezoidal integration rather than copying that paper's Euler realization. General topology-preserving integration background: Vadim Zavalishin, [The Art of VA Filter Design](https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.2.pdf). No external implementation was copied or linked. The result is a Moog-inspired hybrid polyphonic engine, not a measured component-accurate recreation of a named Moog instrument, an official Moog product, or an autonomous self-oscillating ladder.

## Verification

The dedicated analog test measures the four-pole response, resonant emphasis, amplitude-dependent resonance, rapid cutoff jumps, 32/44.1/48/96 kHz operation and reset silence. Instrument regressions additionally cover MIDI offsets, varying blocks, note ownership, state migration, no allocation, presets, LEARN and UI mappings. `compare_engines.cpp` loads old/new native libraries and feeds identical serialized settings and MIDI, exporting raw audio and timing for1/4/16 voices; this separates engine changes from recipe changes. Exact results and hardware limits are in implementation.md and the release notes.
