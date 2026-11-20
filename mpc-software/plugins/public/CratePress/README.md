# Crate Press 0.2.6 public preview

Dice randomizes13MAIN character values with13shared locks and exact Undo; Input/Output/Mix/safety/LATCH/performance gates and setup remain unchanged. First380declarations retained;396total. Same32factory programs and Q-Link targets.

Public preview 0.2.6 is installed and remotely checked on MPC XL OS 3.10.1.12. The shared footer provides UNLOCK ALL / ROLL DICE / UNDO on all five pages. Locks use the same native silhouette and spacing as Donut Theory, with 44×44 touch targets. Actual remote QA is separate from physical/audio/CPU certification.

Public preview 0.2.6. Stereo character insert effect for MPC tracks and master buses. Vendor`kim`, UID`K36Z`. Public preview release; hardware verification is partial.

Independent full-band character compression, Linkwitz–Riley three-band compression, selective bass-sensing or host-tempo ducking, drive, sampler coloration, wow/flutter, tone/cuts and aligned dry/wet mix. This is not a proprietary circuit model or an endorsed artist product. External sidechain input is not implemented.

## Main controls

| Group | Controls in Q-Link order |
|---|---|
| A COMP / DRIVE | Drive / Compress / Punch / Release |
| B BAND / DUCK | Low Comp / Mid Comp / High Comp / Duck |
| C COLOR | Tone / High Cut / Lo-Fi / Drift |
| D I/O / MIX | Input / Low Cut / Mix / Output |

MAIN always displays input/output waveforms, FULL/LOW/MID/HIGH transfer selection, full compression reduction and LOW/MID/HIGH/DUCK reduction. DUCK/COLOR/PERFORM/MORE hold detector, targets, crossover, tempo division, noise, wow/flutter, width and bypass details. Manufacturer text is removed from the visible design; metadata stays`kim`.

The waveform is real stereo signed min/max audio,64bins ×12ms ≈0.77seconds, refreshed30Hz. Both channels contribute extrema so opposite-polarity stereo does not cancel. Input is before Input gain, latency-aligned to the output; Output is after all processing, Mix and Output gain. This is a transient overview, not a sample-by-sample oscilloscope or spectrum. Strong sub-bin details are retained as extrema; height quantization is42pixels per half. Input and output have the same linear ±1 scale.

The graph is the selected compressor's STATIC soft-knee curve before automatic makeup/attack/release,48points over−48..0dB. It shares the exact reduction calculation with DSP. Its vertical cursor is the current detector level, not a frequency. Ducking, color and makeup are visible in the output waveform but not folded into this static compression curve. Display telemetry is non-automatable; Curve Band is a view choice. Parameter indices0–39 are preserved; telemetry/view parameters append at40–346.

## Factory programs

32programs: Init1, Drums8, Keys6, Samples6, Master8, Performance3. `factory-presets.json` is authoritative; `tools/generate_presets.py` emits complete native SDK program vectors. See the full [preset guide](../../../docs/plugins/crate-press/presets.md). Names appear in the host PRESET selector; category-first native host browsing is not verified.

Master presets disable noise/drift and preserve stereo width and low-band duck protection by default. Automatic makeup is a restrained gain compensation, not LUFS matching. Peak Guard is a soft output peak bound, not a certified true-peak limiter.

## Build and test

From the repository root:

```sh
python3 mpc-software/scripts/build_plugin.py CratePress --target all
python3 mpc-software/plugins/public/CratePress/tools/test.py
```

One unmodified official MPC Plugin Builder SDK at pinned commit`febd2ca5bd9eb16c4a412837196e145566bc981b`; framework/DSP utilities are MIT. DSP, factory programs and skin generator are project implementation. Design PNGs are AI-generated concept art; runtime uses only header, material/knob derivatives and code-generated widgets. No private drum library is included. `design/` is never copied wholesale into the package.

## Install / verification limits

Build output: `build/kim - VST - Crate Press/`. Copy into a drive's`VST2 Plugins/` folder. Requires MPC OS3.10.1+ and Enable Enhancements / External Plugin Support. Rescan by restarting the MPC app after preserving current work. Load as an EFFECT insert on a track or master, not an instrument.

Fixed reported latency is6ms; offline bypass impulse alignment is tested. Actual MPC master insertion, project recall, MAIN values, tab navigation and independent latched UI states were checked. Full audio audition, compensation, Q-Link hardware, finger hold/multitouch, track-insert audio and worst-case CPU remain unverified. Offline layout previews are not MPC screenshots. See [verification](../../../docs/plugins/crate-press/validation.md).


## Performance — implemented in0.2.0 prototype

MAIN has8momentary150×82px buttons: TAPE STOP / REVERSE / REPEAT / DUB ECHO | FILTER DIP / CRUSH / WOBBLE / DROPOUT. Raw Mouse Down Toggle Switch starts a gate; a transparent continuous Slider above the art owns ResetOnGestureEnd. No Button or Label touch surface covers it. Physical Crate Press touch verification remains pending even though this input pattern was tested in Donut Theory on MPC LiveIII3.10.1.12.

Tape Stop slows and fades the signal; Reverse loops a captured segment backward; Repeat loops it forward at the chosen host-tempo division. Both freeze capture while held and soften loop edges. Dub Echo is a send: releasing stops new input and lets feedback decay. Filter Dip is a two-stage low-pass dip; Crush adds sample-rate/bit reduction; Wobble modulates a short delay; Dropout starts with an immediate softened cut followed by irregular cut/recovery intervals.

PERFORM contains16Screen Q-Links: A Tape Time/Reverse Length/Repeat Division/Echo Division; B Echo Feedback/Echo Mix/Filter Dip/FX Amount; C Crush Bits/Crush Rate/Wobble Rate/Wobble Depth; D Dropout Rate/Dropout Depth/FX Release/FX Output. MAIN/Program mappings stay unchanged. These settings survive factory program selection; full project state stores them. Factory programs intentionally contain only the first347legacy parameters, so selecting a sound never overwrites a live performance setup. Parameters347–354 are transient gates,355–370 setup,371–378 activity readbacks. IDs0–346 unchanged.

The performance path follows the base dry/wet mix and precedes bypass/output/Peak Guard. Overlapping effects run in a fixed order: Tape Stop→Reverse→Repeat→Filter→Crush→Wobble→Dropout, with Echo send/return last. They process the full stereo signal, including on a master bus. Tape/Reverse/Repeat history is at most8seconds; long capture selections at slow tempi are clamped to available history. Four stereo history buffers allocate in prepare(), about10.8MiB at44.1kHz, plus small fixed buffers. No render-time allocation or capture copying. Reset invalidates history without clearing megabytes in the render callback.

Loading a state/preset disarms performance immediately and clears transient gate values on the first30Hz display tick (~33ms). Held performance effects never resume from a saved project. Normal press/release uses5ms onset and adjustable5–200ms release (default15ms); LATCH mode can hold them independently. Dub Echo alone retains its tail. Performance gain/filter/mix changes are not a true-peak mastering limiter; use output headroom.


Simultaneous behavior: all28two-effect pairs were tested at3sample rates, comparing each pair with either effect alone. Both contribute, and releasing one preserves the remaining effect. Reverse changes a Repeat capture's read direction; Tape Stop slows/fades captured Reverse/Repeat playback too. The remote mouse cannot prove physical two-finger tracking; actual MPC multitouch remains a separate verification gate. Reverse/Repeat length changes apply on the next trigger.


LATCH is a touch-only global header button (parameter379, appended). Off: eight momentary surfaces retain the verified hold pattern. On: the eight surfaces become normal tap toggles, allowing multiple effects to remain engaged. Turning LATCH off releases every effect gate; echo tails decay naturally. The mode preference is retained in full state and stays independent of factory programs; loading state never resumes saved active effects. The16main Q-Links and16PERFORM Q-Links remain unchanged. The VST tail budget is240seconds at the actual sample rate, covering the maximum slow-tempo/feedback echo rather than the old short tail.

Native skin cache updates use a plugin-local bridge: sound programs and state loads queue a refresh; measured telemetry queues changed values; non-render host reads/idle drain notifications. No notification is sent from process(), including parameter reads made on a registered render thread. The shared SDK is unmodified.

## 0.2.3 maintenance update

DSP, presets, parameter identities and UI are unchanged from 0.2.2. Updated package metadata and installation guidance: prefer a single complete installation under internal storage `VST2 Plugins`; remove duplicate older copies from removable drives before rescanning. Keep `VST`, `Content` and `Plugin Skins` together. This remains a public preview with the hardware limits documented in the manual.

## 0.2.4 action layout

Original artwork, lettering,64px knobs/I/O faders and eight performance buttons are retained. LATCH and Curve Band selectors move to the bottom utility row. Full offline tests and MPC XL remote UI checks pass; see design/device-0.2.4 and spec/validation for limits. Public preview0.2.4 includes the footer layout; physical/audio/CPU limitations remain documented.

## License and catalog distribution

Crate Press project code, factory programs and original runtime artwork are offered under MIT; see LICENSE. The unmodified official SDK retains its separate MIT notice in Content/Licenses/. No third-party audio library, proprietary firmware, ROMs or original manufacturer skin files are bundled.

Version0.2.5 is a documentation/licensing/distribution patch: DSP, all380parameter identities,32factory programs, skin/Q-Link bytes and performance behavior remain0.2.4. Existing actual MPC captures are labeled0.2.4; they are not new0.2.5hardware certification. The preview status remains. Dedicated Open MPC ARM32/ARM64 archives supplement the normal SDK ZIP. In Open MPC enable beta/experimental releases to see this preview.
