# Donut Theory1.12.0 — local Release / Pitch Drop candidate

Actual MPC XL OS3.10.1.12 Remote Screen captures. Installed on internal NVMe with official SDK update/restart/export; all173files match SHA256. Public stable remains1.10.1; no new Release/Pages publication.

- PLAY D: Release / Amount / Time / unassigned. Amount0=Off; new-state Time250ms.
- Shared Release: PLAY120→1116.7ms appeared on ENV; ENV then5000ms appeared on PLAY. Filter Release remained300ms.
- Native/ARM32/ARM64 + SDK validator passed. Full engine/UBSan/bridge/analog/harmony/skin suite passed; host simulator19passed/0warnings/0failures.
- Feature regressions: sample-offset release, exact ramp length at44.1/48/96kHz with variable blocks, no held-note drop, chord intervals, sustain/duplicate off, legato fallback, retrigger, panic/reset, strum cancellation, save/load and old291state migration. First291metadata exact; append2→293total. All66programs retained.
- MAIN and its filters, CHORD/OSC/MOD/FX trees and default mappings unchanged. Only PLAY mapping adds D.
- Remote UI gestures are distinct from physical encoder/keyboard/audio/output/CPU certification, which remains outstanding.

See verification.json and logs for evidence. Screenshots are device captures, not concept renders.
