# Pack Builder 1.0.0 verification

- Local macOS26.5.2 / Apple Silicon / Python3.13.7 / PySide6 6.10.2: four backend integration tests passed; actual Qt controls exercised role selection, part lock/gain, kit lock, regeneration, project save/reopen and `.gdexp` export. The packaged standalone `.app` launched and passed its offscreen smoke test.
- The authored Workshop Sessions pack contains32 own WAV,2 own MIDI and8 kits. It was installed into a clean local copy of Groove Dice Content. A compiled actual C++ engine test loaded104 total kits/105 native programs, asynchronously decoded all eight roles, produced finite non-silent audio and restored the selected kit identity from state. GM and native MIDI mappings were tested separately.
- Input guards cover changed source hashes, invalid/unsupported MIDI, path escape/symlinks, incompatible role mapping, duplicate IDs, private opt-in and omitted instruments. A missing-role pack requires explicit silence permission.
- Platform builds are not interchangeable. Windows x64 CI run43814119104 (commit6af7dfa) completed successfully: backend tests, actual Qt controls and standalone executable smoke test. Hosted macOS job of the first workflow was cancelled before starting; the shipped Mac binary is the verified local Apple Silicon build. Real Windows audio-device and exhaustive hardware compatibility are not inferred from the Mac tests.
- macOS package is ad-hoc signed, not notarized; Windows package has no signing certificate. Dynamic-library notices accompany each bundle. No third-party/private audio is included.

Local evidence: ignored build/qa/, /tmp/packbuilder-*.log and dist/build-report.json. Native UI and device results are appended after observation.

- Native Mac UI: actual packaged app auditioned a kit, exported Workshop Sessions via the real save dialog and displayed8kits/32WAV/2MIDI. The exact GUI-exported file was installed on MPC; pack filter/selection/49.821s audio/preset save-load worked. Advanced Pack ID was observed collapsed by default in the final native UI. Actual app screenshot stored in build/qa/app-1.0.0-actual.jpg.

- Final Mac bundle installed at ~/Applications/Groove Dice Pack Builder.app, strict ad-hoc signature verification passed and the installed executable smoke test exited successfully. Final Windows run43814119104 passed after audition-lock/export-extension fixes. Both published platform ZIPs include runtime license texts and SHA256 sidecars.
