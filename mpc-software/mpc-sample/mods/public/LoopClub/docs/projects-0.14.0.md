# Loop Club 0.14.0 — project management

Owner request: expose New Project, Open and Save rather than overwrite/reload a single hidden Session file.

Entry: LOOP → SHIFT+DATA → + to PROJECT2/2 → B2 PROJECTS. Upper physical B1/B2/B3 map to NEW/OPEN/SAVE. SHIFT+B3 makes a copy. SAMPLE returns to LOOP. Existing pad8/pad9 now enter named Save/Open; pad15 starts New confirmation. Other SETUP functions remain.

New/Open always ask SAVE+GO / DISCARD / CANCEL. Storage failure aborts replacement; running transport, active recording, held notes or release tails prevent writes/replacement. The modal consumes background pads, CC knobs/fader, transport start/record, Undo/Redo and unrelated navigation. STOP remains available; TAP retains its existing global behavior. Filesystem operations and Engine allocation/destruction run on the existing exclusive project worker, never on the audio callback. Replacement resets UI ownership/pickup and history without changing hardware master gain.

Legacy `Session.lcproj` remains untouched unless explicitly saved. Numbered projects use `Projects/Project-NNN.lcproj`, plus per-project `*.previous.lcproj`. Startup uses a validated local `active-project.txt` choice, falling back to Session if missing/invalid. Names are auto-numbered, not text-editable. Only plain files with safe ASCII stems are listed; previous copies, temporary files and symlinks are excluded. Maximum list256. QA directories are isolated using the existing compile-time QA session; never save tests in owner storage.

Format12 and DSP unchanged. Native regression includes New/Save/Open/Copy, PCM recall, hardware-volume preservation, atomic previous copy, failed Save+New, corrupt/missing Open, busy refusal, unsafe paths, symlinks, cancel and list boundaries. Actual device QA checks the visible manager, DATA selection, save/copy/confirm, STOP protection and stopped named-project startup after app restart. This is remote input + actual DRM evidence, not physical finger acceptance or power-cycle certification.

See `validation-0.14.0.json` for final build identities and results. No new Release/site publication is implied by this local feature request; existing0.13.0 assets/videos stay unchanged.
