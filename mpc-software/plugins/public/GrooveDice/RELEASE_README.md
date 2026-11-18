# Groove Dice 1.4.0

Retained from1.3.4: all seventeen SOUND sources have independent reverse switches, saved with user kits and projects. Start trims from the source end in reverse mode; Decay and Shift remain independent. SOUND has two in-place pages: Sample/Level/Tune/Shift, then Start/Decay/Low Cut/High Cut. Separate48px previous/next arrows surround a visible SOUND1/2 counter. REV stays available on both pages; approved per-row polarity Ø is on page2. Remote on-device page/REV/Ø interactions were checked; physical encoder and hardware-audio coverage remain separate.

One-bar neo-soul and hip-hop drums for MPC standalone, with visible Dice changes and individual locks.

- Sixteen MAIN padlocks protect individual knob values. Open controls change visibly when Roll Dice is pressed; musical bounds preserve backbeats, practical mix levels and restrained timing.
- Seven playable RULE styles per part. LOCK is removed from the knob/list/Q-Link range and handled by the separate padlock. Existing saved LOCK states migrate safely.
- MAIN D: Hat Rhythm, Open Hat, Ride and Crash. Fifteen straight/dotted/fill/roll hat rhythms, ride/bell patterns and structural crash accents. Drive, Dust and real stereo Room reverb remain on FX.
- SOUND1/2: Sample, Level, Tune and Shift. SOUND2/2: Start, Decay, Low Cut and High Cut. Every source keeps one fixed SCREEN Q-Link group. Separate48px arrows and a visible counter switch all rows together; no separate Detail modal. REV is available on both pages; polarity Ø is on page2.
- Independent Ride/Crash/Bell source editing. Four kick/snare layers, linked hats and a four-role percussion set.
- Dice sound range: This Pack, All Packs or up to 64 selected kits. Whole-kit or mixed-part selection respects kit/layer locks.
- Eight-role fill grid in two views; independent recorded Fill A–D. Metrical velocity accents and live one-bar rule editing.
- Fixed blank skins caused by empty text widgets and the overlapping kit-lock caption.

## Installation and contents

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the ZIP and copy the complete `kim - VST - Groove Dice` folder to `VST2 Plugins` at the drive root, preferably internal storage. Keep one installed copy across attached drives, save work and restart/rescan MPC. Read MANUAL.md for performance, fills, routing and packs.

Public package: Original Workshop only — 128 independently synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. No private/commercial sample library is included. Optional .gdexp files remain separate and require reopening the instrument after installation/removal.

## Compatibility and validation

UID KpGy; 672 append-only parameters, first643 unchanged. Existing projects/expansion identities are supported; old RULE LOCK values migrate to BASIC plus a closed padlock. Signed Shift reports fixed 30ms latency including direct pads. Newer state cannot be fully represented by old engines. External sidechain and generated MIDI output are not implemented.

Native/ARM32/ARM64 builds and SDK validation pass. Engine, UBSan, allocation guards, sample/state/expansion/skin and real VST bridge tests pass. Host simulator:19 pass,0 warnings,0 failures. The owner confirmed 1.3.3 locks/Dice/RULE behavior on MPC XL OS3.10.1.12. Actual screenshots come directly from the authenticated MPC Remote Screen framebuffer stream, with capture provenance recorded. Complete physical-control coverage, all routes and worst-case hardware CPU are not certified. No ASan pass is claimed. See VERIFICATION.md.

Version1.4.0 adds16RULES per part, with a paged left list and full-range Q-Link control. See MANUAL.md and VERIFICATION.md.
