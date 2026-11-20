# Donut Theory 1.15.0

- Control OSC2 phase relative to OSC1 from the main A4 Q-Link: switch DRIFT / PHASE without changing either value.
- FREE retains the existing oscillator history; RESET aligns new articulations. Connected legato and chord glide preserve phase. Strum notes reset when their delayed gate starts.
- Eight waves per oscillator: Sine, Triangle, Saw, Pulse, Soft Saw, Soft Pulse, Organ and Hollow. Live phase and wave changes crossfade.
- Phase has its own Dice lock. Wave choices share the existing oscillator lock. Dice includes the new palette and phase; Undo restores the previous sound.
- Existing 66 factory programs remain sample-identical to 1.14.2 in native A/B tests at default phase settings. The first 391 parameter declarations remain unchanged; 399 total.

Native, ARM32 and ARM64 builds and SDK validation pass. Full regression/UBSan tests and the SDK host simulator pass (19 PASS / 0 WARN / 0 FAIL). Actual MPC XL / OS 3.10.1.12 remote checks cover Phase, FREE/RESET, Drift isolation, wave browsing, Dice locks, Undo, native presets and page rendering. NVMe installation is verified by independent export and SHA-256. This does not certify physical encoders, hardware audio, multitouch, worst-case CPU or full project recall.

Requires MPC standalone OS 3.10.1+ with Enhancements and External Plugin Support enabled. Copy the complete plugin folder into VST2 Plugins, keep one active copy across attached drives, and restart/rescan after saving work. UID kim/Krql and existing state meanings are preserved. See the included README and online manual for phase, waveform and compatibility details.

All installation operations targeted internal NVMe only. SD and SSD were not read or modified. Official SDK remote tooling was used.
