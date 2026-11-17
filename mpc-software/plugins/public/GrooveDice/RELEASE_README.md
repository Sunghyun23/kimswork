# Groove Dice 1.3.4 candidate

REV is new in this candidate: all seventeen SOUND sources have independent reverse switches, saved with user kits and projects. Start trims from the source end in reverse mode; Decay and Shift remain independent. SOUND has two in-place pages: Sample/Level/Tune/Shift, then Start/Decay/Low Cut/High Cut. Separate48px previous/next arrows surround a visible SOUND1/2 counter. REV stays available on both pages; approved per-row polarity Ø is on page2. Physical REV touchscreen verification is pending.

One-bar neo-soul and hip-hop drums for MPC standalone, with visible Dice changes and individual locks.

- Sixteen MAIN padlocks protect individual knob values. Open controls change visibly when Roll Dice is pressed; musical bounds preserve backbeats, practical mix levels and restrained timing.
- Seven playable RULE styles per part. LOCK is removed from the knob/list/Q-Link range and handled by the separate padlock. Existing saved LOCK states migrate safely.
- MAIN D: Hat Rhythm, Open Hat, Ride and Crash. Fifteen straight/dotted/fill/roll hat rhythms, ride/bell patterns and structural crash accents. Drive, Dust and real stereo Room reverb remain on FX.
- Every SOUND row/layer: Sample, Level, Decay and Shift. Tap sample names for the left browser, use arrows or Q-Link, and PREVIEW a single selected layer. Tune/Start/filter settings remain in Details.
- Independent Ride/Crash/Bell source editing. Four kick/snare layers, linked hats and a four-role percussion set.
- Dice sound range: This Pack, All Packs or up to 64 selected kits. Whole-kit or mixed-part selection respects kit/layer locks.
- Eight-role fill grid in two views; independent recorded Fill A–D. Metrical velocity accents and live one-bar rule editing.
- Fixed blank skins caused by empty text widgets and the overlapping kit-lock caption.

## Installation and contents

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the ZIP and copy the complete `kim - VST - Groove Dice` folder to `VST2 Plugins` at the drive root, preferably internal storage. Keep one installed copy across attached drives, save work and restart/rescan MPC. Read MANUAL.md for performance, fills, routing and packs.

Public package: Original Workshop only — 128 independently synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. No private/commercial sample library is included. Optional .gdexp files remain separate and require reopening the instrument after installation/removal.

## Compatibility and validation

UID KpGy; 672 append-only parameters, first643 unchanged. Existing projects/expansion identities are supported; old RULE LOCK values migrate to BASIC plus a closed padlock. Signed Shift reports fixed 30ms latency including direct pads. Newer state cannot be fully represented by old engines. External sidechain and generated MIDI output are not implemented.

Native/ARM32/ARM64 builds and SDK validation pass. Engine, UBSan, allocation guards, sample/state/expansion/skin and real VST bridge tests pass. Host simulator:19 pass,0 warnings,0 failures. The owner confirmed 1.3.3 locks/Dice/RULE behavior on MPC XL OS3.10.1.12. Actual screenshots come directly from MPC through Remote Screen Viewer. Complete physical-control coverage, all routes and worst-case hardware CPU are not certified. No ASan pass is claimed. See VERIFICATION.md.
