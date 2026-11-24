# Loop Club 0.14.2 — additional validation

Local maintenance candidate for MPC Sample firmware1.3.0.12. **Public release remains0.14.1.** This record describes tested local code; it is not a public release announcement.

## Corrections and behavior

- Drum SHIFT+pad records only. Generation is explicit DRUM PLAY B2; SHIFT+B2 undoes generation. Generation preserves stopped/running state and refuses active capture.
- Drum Replace suppresses all previous sequenced lanes and fades old one-shots. Overdub retains the previous pattern. Cancel/Undo preserve the completed clip.
- Full-current-bar drum occupancy and short hit markers include later events. Drum GRID maps its visible events back to original event indices for edits.
- SOURCE Audio pad8 TURN fills only an empty source with this project's original procedural1.8s AHH. File-worker preparation keeps allocation/I/O outside audio; existing source PCM is retained.
- All nested pages expose BACK. A bounded UI history restores the actual previous context. SAMPLE goes to LOOP; audio SOUND B3 is BACK. Synth deep editors return to PLAY first; Drum detail returns to overview first. INPUT returns through SETUP2/2 and TIMING. PROJECT BACK retains the underlying page.
- FX B1 is BACK; physical MIX opens MIX; SHIFT+B1 opens DRUM MIX for a Drum track. LOOP edit B1 COPY/B2 DELETE/B3 BACK, SHIFT+B1 MOVE. Note-edit B3 BACK.
- Visual inspection found and corrected TURN's custom header still saying REPLACE despite B3 already returning BACK. Added a separate rendering regression for all four audio targets. Corrected the Timing footer and Move's highlight so BACK is not selected as Move.

## Evidence

429 full native tests, strict Clippy and ARM64 normal/QA builds pass.40 optimized realtime/motion/navigation checks also passed; this is an additional configuration of overlapping tests, not469 distinct tests.

Actual active-DRM/independent queued control coverage: all16 page types,124 complete button presses with exactly two callback edges each, Synth subpages, record/Undo/Redo, Mute/Chain,14 track and14 master effects, hold/latch/release across navigation, advanced FX/sidechain, layer/detail, XPM/WAV generated fixtures, browser DATA/plus/minus, Setup/input and isolated project save/open.

Actual USB MIDI recorded72+events. A later-bar global event64 was edited to velocity25, then Undo restored106. Old drums during no-hit Replace measured peak0; Overdub remained audible. The original scratch source was audible and finite, and reentry retained PCM.

CUT dry-source USB peak was0 while held, versus about0.1367 before/after. Its head continued forward. Release after BACK and a PROJECT overlay restored audio. Background drums continued with peak about0.2224 while scratch CUT was held.20 repeated CUT cycles left no owner stuck. Native tests verify the3ms anti-click envelope at8k/44.1k/48k; this does not measure physical pad latency.

The initial all-page pass and final affected-page pass are distinct builds: the latter corrects only the remaining captions/Move highlight and is separately captured. Raw isolated evidence stays under ignored `build/interface-flow-0.14.2/`. Machine-readable public-safe summary: [validation-0.14.2.json](validation-0.14.2.json).

## Installation and limits

Initial validation-pass runtime and SD library SHA-256: `654d82460fb94470f7ad446ef2f3ad9dd844e72e130763c513c19a3b48945bdd`.20 installed files were read back;20 previous files preserved. The saved owner's16tracks, BPM and input settings matched the earlier saved baseline; no owner Save was performed. Left stopped on LOOP, volatile preroll1bar and metronomeOff. The exact normal image also passed real USB MIDI/audio smoke, peak0.01645, no USB flags.

These are actual device DRM/USB tests with injected controls, not human finger/button acceptance or analog/power-cycle certification. A final QA callback exceeded its deadline including scheduler interruption; maximum CPU and long-session behavior remain limited. Drum GRID exposes up to64events per visible bar, while full occupancy scans the clip. Legacy Synth/Audio event windows remain. No release/site deployment occurred in this validation pass.

## Project-action completion follow-up

Owner requested direct LOOP return after New/Open/Save. A monotonic in-memory successful-operation counter now closes PROJECT and routes to normal LOOP after New, Open, Save and Save a Copy. Scans, busy operations, cancellation and errors never trigger it. Save preserves selection/content; only New/Open reset the project UI. Return exits MUTE/CHAIN and LOOP edit, without starting playback. No project-format or DSP change.

431 full native tests and strict Clippy pass. Actual isolated QA passed8cases: cancel retains GRID; New returns LOOP; Save from SOUND, Save Copy from GRID and Open from MIX return LOOP; running Save fails in-menu; saving from MUTE and LOOP edit returns normal LOOP. Only isolated QA projects were saved.

Current normal runtime/SD SHA-256: `49666d327cd6a846828d3f79ebf68f189fb95ef37e8e5821105e84f6e8c5088f`.20files read back/20oldfiles retained. During development the owner created an empty unsaved Project-001; its zero-content16track/BPM/preroll state was recorded, recreated after QA and compared. Existing Session files stayed untouched; no owner Save. Left on the empty Project-001 LOOP with PROJECT closed. These replace the earlier installation-state/hash paragraph only; earlier audio/screen evidence retains its original build identity. Public remains0.14.1.
