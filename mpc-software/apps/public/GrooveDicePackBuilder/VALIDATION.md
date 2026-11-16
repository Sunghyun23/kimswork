# Pack Builder 1.0.0 verification

- Local macOS26.5.2 / Apple Silicon / Python3.13.7 / PySide6 6.10.2: four backend integration tests passed; actual Qt controls exercised role selection, part lock/gain, kit lock, regeneration, project save/reopen and `.gdexp` export. The packaged standalone `.app` launched and passed its offscreen smoke test.
- The authored Workshop Sessions pack contains32 own WAV,2 own MIDI and8 kits. It was installed into a clean local copy of Groove Dice Content. A compiled actual C++ engine test loaded104 total kits/105 native programs, asynchronously decoded all eight roles, produced finite non-silent audio and restored the selected kit identity from state. GM and native MIDI mappings were tested separately.
- Input guards cover changed source hashes, invalid/unsupported MIDI, path escape/symlinks, incompatible role mapping, duplicate IDs, private opt-in and omitted instruments. A missing-role pack requires explicit silence permission.
- Platform builds are not interchangeable. Windows and hosted macOS CI results are pending until the workflow completes. Real Windows audio-device and exhaustive hardware compatibility are not inferred from the Mac tests.
- macOS package is ad-hoc signed, not notarized; Windows package has no signing certificate. Dynamic-library notices accompany each bundle. No third-party/private audio is included.

Local evidence: ignored build/qa/, /tmp/packbuilder-*.log and dist/build-report.json. Native UI and device results are appended after observation.
