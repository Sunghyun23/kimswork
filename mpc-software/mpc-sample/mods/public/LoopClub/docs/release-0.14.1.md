# Loop Club 0.14.1 — DATA project menu

Public preview for MPC Sample firmware1.3.0.12. This is an SD preload mod, not flashed firmware or an MPC OS plugin.

- SHIFT+SEQ opens a separate PROJECT menu. Turn DATA and press to choose New Project, Open Project, Save or Save a Copy. The upper buttons are Back/Select/Loop.
- New/Open use the same DATA confirmation list, initially highlighting Cancel. Failed saves/invalid files retain the current live project. Stop transport and release notes/tails before saving or replacement.
- Numbered projects embed source audio, kits/layers, MIDI, sound and FX settings. Atomic saves retain a previous copy. Save a Copy does not overwrite the old file. Existing Session stays in the list; the last saved/opened project restores stopped on LOOP.
- Project changes reset UI and external-MIDI selection consistently. Pending sample/kit imports finish before replacement. No DSP or project-format change: format12 reads1–11; older versions reject12.

Native regression, strict Clippy, ARM64 normal/QA builds and launcher integrity checks are recorded in release-0.14.1.json. Actual Sample remote controls/DRM verify the menu, confirmations, save/copy/open and stopped app-restart recall. Public package and installed feature-candidate library hashes are recorded separately. Existing owner16tracks/BPM and previous SD files are retained; tests save only in isolated QA storage.

Physical fingers, analog latency, power-cycle recovery and maximum/long-session CPU remain separately unverified.32shared Synth/Drum voices,16sampler voices, stereo USB, first64grid events and bounded Warp/XPM limits remain unchanged. Automatic numbered project names have no text rename editor; the project list is limited to256 entries. Each embedded copy consumes storage.

Install the complete autorun folder after saving work and backing up the old boot and projects. User project/expansion directories are outside autorun and are not part of this update. No private bank, raw third-party sample/MIDI, firmware or QA agent is included. Original procedural drums remain built in. Existing videos and earlier release assets are unchanged. Source MIT and dependency notices are bundled. Manual: https://kimswork.app/mods/loop-club/manual/
