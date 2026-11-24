# Loop Club 0.11.4 response measurements — actual MPC Sample

2026-11-24, same final installed normal binary and PID33114 throughout.30 trials per path, stopped transport, saved owner synth/drum tracks, USB stereo44.1kHz. No owner project save, restart, source edits or SD changes. Selection and SETUP restored; metroOff/preroll1bar. No new audio callback deadline overruns or PortAudio flags observed. This is a light-load measurement, not worst-case16-track certification.

| Path | Median | P95 | Maximum |
|---|---:|---:|---:|
| CoreMIDI note send → USB audio onset, PortAudio ADC timestamp estimate |5.04ms|6.30ms|6.63ms|
| Same MIDI event → host audio callback delivery |34.52ms|39.64ms|52.39ms|
| HTTP remote drum pad → USB onset, ADC timestamp estimate |40.64ms|63.31ms|66.95ms|
| Remote DATA → observed selected-track state |75.10ms|134.08ms|138.94ms|
| Remote independent button → observed metro state |145.44ms|151.39ms|154.40ms|
| Same button → observed changed active-DRM image |201.64ms|207.56ms|247.35ms|

HTTP status round-trip median4.08ms/P955.89ms. These are separate remote observations, not quantities that can simply be subtracted to recover pure app time. Screen retrieval includes image readback/download and serial polling; the measured image time is an observation upper bound, not panel scanout latency. PortAudio ADC onset estimates use host callback clock mapping and threshold0.001FS; they include USB/MIDI transport and the saved synth preset attack, and are not calibrated against an oscilloscope. Physical pad/fader/key-to-analog latency was not measured. Capture delivery includes host buffering; don't confuse34.52ms callback receipt with5.04ms estimated onset.

Code inspection explains distinct paths: independent hardware controls are drained in `device.rs::on_paint`; injected TransportButton first waits in the QA queue and then the hardware queue, which can add an extra paint turn. Injected pad/DATA also enter through paint. USB MIDI follows the note path. Therefore the slower injected pad/button numbers do not certify actual physical pad delay. The UI/control paint dependency is a candidate for a separate latency optimization; no realtime/control redesign was performed as part of measurement.

Raw30trial evidence: `design/device-0.11.4/latency/{controls,midi}.json`. Reproduce with `tools/qa_latency_controls.py --execute` and `tools/qa_latency_midi.py --execute` using the project's audio-qa Python environment. Requires the named MPC Sample CoreMIDI/stereo endpoint and current idle0.11.4. These tools preserve stored projects, but operate transient controls and produce audible notes.
