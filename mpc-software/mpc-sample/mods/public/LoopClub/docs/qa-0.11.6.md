# Loop Club0.11.6 — all-page input ownership audit

The owner requested detailed QA of duplicate/conflicting controls across every screen after input latency optimization. All16Page variants and Synth subpages were checked. No sound-engine/record-format/SDK dependency change.

## Findings and corrections

- **Browser modal priority bug, reproduced before fix:** SHIFT+plus emitted globalRedo while the list was open; SHIFT+minus similarly reachedUndo. Plain+/minus over a Setup screen reached the background timing/project toggle instead of moving the list. The browser now owns these keys first: plain±one row, SHIFT±six rows. STOP remains global; TAP retains its explicit global tempo/metronome operation. Closing the browser restores the unchanged underlying page. Native failing regression and its passing counterpart are retained with actual MPC checks.
- **Diagnostic double counting:** hardware DATA already incremented the hardware counter, then its immediate Mod callback also incremented the remote counter. Moved remote counting to the explicit remote injection branch. This was a measurement attribution defect, not evidence of two selection actions.
- Removed stale unreachable Master/Texture branches from DATA routing; advanced FX Settings stays the one owner there, while Sidechain DATA still opens master inserts. No navigation behavior was reassigned.

## Command-observing native checks

The test-only send observer records commands emitted by the real Mod callbacks; it is completely absent from device builds.4224button combinations across16pages/3source kinds/normal-Shift/detail states check unique emitted engine commands and inert release.3072pad/modifier combinations check no duplicated emitted command and idempotent repeated release.192Synth subpage/chord/Shift soft-key cases cover conditional ownership. 640browser button/modifier cases verify modal priority across every page (STOP/TAP are explicit global exceptions). Additional checks cover modal knob/fader/pad blocking, held-note release after navigation, DATA detent normalization, Mute/Chain versus record/generation, loop/note editor exclusivity, and single Shape/Sync/Sync-division actions. All11audit tests pass in Debug and Release. Existing full387Debug tests plus3added overlay tests give390distinct Debug cases; strict all-target Clippy and ARM64 QA/normal builds pass.8package launcher cases pass.

A unique command per press does not imply every press emits exactly one command: valid FX operations intentionally send several different parameter commands, whereas unsupported controls intentionally emit none. The test detects duplicated identical commands; explicit cases verify mutually exclusive record/generation/playback routes.

## Actual MPC Sample checks

The fresh deep-audio-buttons-20261124-r1 directory isolated all recording, imports and project Save/Load. No owner Session was saved. Stable QA PID34498; all16page IDs captured from active DRM, plus synth subpages, browser, input and turntable screens. Every measured independent remote button press produced exactly its2press/release callbacks. Native queued injection exercises the busy-Mod fallback, not a physical finger press.

Verified: normal navigation; six Synth next/previous page transitions; independent chord/record-mode switches; capture start/finish and Undo/Redo;20consecutiveSEQ presses stay GRID; Mute/Chain pads do not record; mixer/output round trip;14track and14master FX momentary operations, LATCH, held release after navigation, preservation of the other slot; Shape/Sync; Drum Sound detail switch once; Kit Mix; kit/sample browser±andShift±overSetup without background changes; generated XPM bank import; generated WAV import; Chop; both fader endpoints and turntable grab release; timing/project/input Setup; Data changes input choice without committing it; isolated project recall.

Harness corrections are not counted as plugin defects: the first master-FX assertion incorrectly required unrelated stored inserts to clear; the second used the wrong Drum Sound entry (it is LOOP→B2); the XPM fixture has3nonempty banksA/B/H rather than8visible entries. The final complete run passes with these correct expectations. Old failed snapshots remain ignored development artifacts, not passing evidence.

Final normal public-ZIP library was separately installed as PID34753. It passed owner16track/BPM preservation, actual browser priority, timing switches and actual stereo USB note audio (three finite nonzero peaks, no PortAudio flags). Runtime and SD library SHA7f9cfe5a428990679745534d1968b9424a5ba32cab22a7b00b7309536592e20a;20SDfilesreadbackverified/20oldbootfilespreserved. Final UI: SETUP timing, metronomeOff/preroll1bar. Current volatile switches were not saved over the owner project. Public release/site remain0.11.2.

Evidence: [actual16screen contact sheet](../design/device-0.11.6/all-pages-contact.png), [device cases](../design/device-0.11.6/verification.json), [normal-build checks](../design/device-0.11.6/normal-verification.json), [machine validation](validation-0.11.6.json). Physical key/pad transport, power-cycle, analog mic/line and maximum-load CPU are not newly certified. No finite QA pass proves all possible bugs absent.

Reproduction: tools/qa_input_matrix.py requires --execute, --session (a fresh deep-audio- ID matching qa.so), --restore-library (SHA-matched running normal binary) and --saved-baseline. It refuses changed owner track metadata/BPM before restart and refuses a nonempty reused QA session. Compile fixture QA into build/qa-device/buttons-0.11.6/qa.so first. Preserve the resulting captured project/evidence; do not rerun against the saved session as if fresh.
