# Loop Club0.11.5 — input latency optimization

Button/CC/DATA/pad hooks now try the SDK's existing nonblocking exclusive Mod dispatch immediately. If the Mod is busy or reentrant, the exact event remains owned and falls back to bounded queues; no native forwarding leak. Fallbacks are drained before a new immediate event, at paint start, after injected inputs and after drawing. This removes an avoidable extra paint wait. Pad ownership and release pairing, DATA CC deduplication, queued modifier/tap timestamps, native-volume forwarding, emergency-stop-on-full and all recording semantics remain intact. No SDK patch or audio callback design changed; no retained Graphics/component pointer, background UI mutation or repaint pump.

Queues remain multi-producer/single-consumer: only SDK-exclusive Mod callbacks drain them, with a reentry guard.15focused Release cases include ordered once-only pending controls, momentary FX press/release and reentrant drain exclusion. Full Debug378 plus one direct-dispatch case (379 distinct)/strict Clippy/ARM64/8package launcher cases pass. Actual repeated20SEQ, SETUP switches and owner16tracks/BPM preservation pass. Owner project never saved. Exact public-ZIP normal binary tested; measured PID33376 stable.

## Actual30-trial comparison with0.11.4

| Remote observation / USB estimate | Before median | After median | After P95 |
|---|---:|---:|---:|
| Independent remote button → metro state |145.44ms|81.84ms|99.33ms|
| Same → changed active-DRM image observed |201.64ms|139.25ms|179.92ms|
| Remote DATA → selection state |75.10ms|76.19ms|133.61ms|
| Remote drum pad → ADC-timestamp USB onset estimate |40.64ms|35.44ms|60.91ms|
| Real USB CoreMIDI note → ADC-timestamp USB onset estimate |5.04ms|5.24ms|6.42ms|

Button state observation improved44%, changed-frame observation31%. DATA is essentially unchanged; do not claim universal improvement. Remote injection still waits for a paint and the DRM request includes readback/download, so these are not physical panel scanout or physical pad latency. Actual physical fast-dispatch benefit is not separately certified by this test. MIDI onset uses PortAudio clock mapping/0.001FS threshold and includes USB/preset attack, not oscilloscope calibration. Host callback delivery itself remains about34ms due capture buffers.

No PortAudio flags or new audio deadline overruns during either measurement window. A lifetime overrun1/max142.37% already existed before measurement after app startup; do not describe the complete boot-to-end run as overrun-free. P99 processing budget upper bound45%. No16track/maximumFX/physical/power-cycle certification. Raw30trials and timing counters are in design/device-0.11.5; baseline methodology docs/latency-0.11.4.md.

Installed runtime/persistence details are recorded in validation-0.11.5.json after readback. Owner saved16tracks are unchanged. Current volatile metronomeOff/preroll1bar; existing saved prerollOff was not overwritten. Public0.11.2 remains unchanged; no release or website publication performed for optimization.

Persistence completed: exact tested normal library844d37f0e537a573008a43e8b7cafd6b9af11233e5044b3edd05dd6d0c9dbbb7;20new readbacks/20old boot files preserved. Final SETUP captured with metroOff/preroll1bar.
