# Groove Dice 1.6.2

Retained from1.3.4: all seventeen SOUND sources have independent reverse switches, saved with user kits and projects. Start trims from the source end in reverse mode; Decay and Shift remain independent. SOUND has two in-place pages: Sample/Level/Tune/Shift, then Start/Decay/Low Cut/High Cut. Separate48px previous/next arrows surround a visible SOUND1/2 counter. REV stays available on both pages; approved per-row polarity Ø is on page2. Remote on-device page/REV/Ø interactions were checked; physical encoder and hardware-audio coverage remain separate.

One-bar neo-soul and hip-hop drums for MPC standalone, with visible Dice changes and individual locks.

- Sixteen MAIN padlocks protect individual knob values. Open controls change visibly when Roll Dice is pressed; musical bounds preserve backbeats, practical mix levels and restrained timing.
- Sixteen playable RULE styles per part. LOCK is removed from the knob/list/Q-Link range and handled by the separate padlock. Existing saved LOCK states migrate safely.
- MAIN D: Hat Rhythm, Open Hat, Ride and Crash. Twenty-four straight/dotted/triplet/fill/roll hat rhythms, ride/bell patterns and structural crash accents. Drive, Dust and real stereo Room reverb remain on FX.
- SOUND1/2: Sample, Level, Tune and Shift. SOUND2/2: Start, Decay, Low Cut and High Cut. Every source keeps one fixed SCREEN Q-Link group. Separate48px arrows and a visible counter switch all rows together; no separate Detail modal. REV is available on both pages; polarity Ø is on page2.
- Independent Ride/Crash/Bell source editing. Four kick/snare layers, linked hats and a four-role percussion set.
- Dice sound range: This Pack, All Packs or up to 64 selected kits. Whole-kit or mixed-part selection respects kit/layer locks.
- Eight-role fill grid in two views; independent recorded Fill A–D. Metrical velocity accents and live one-bar rule editing.
- Fixed blank skins caused by empty text widgets and the overlapping kit-lock caption.

## Installation and contents

MPC standalone OS3.10.1+ with Enable Enhancements and External Plugin Support. Extract the ZIP and copy the complete `kim - VST - Groove Dice` folder to `VST2 Plugins` at the drive root, preferably internal storage. Keep one installed copy across attached drives, save work and restart/rescan MPC. Read MANUAL.md for performance, fills, routing and packs.

Public package: Original Workshop only — 128 independently synthesized one-shots, 96 kit combinations, two authored beats and 93 timing templates. No private/commercial sample library is included. Optional .gdexp files remain separate and require reopening the instrument after installation/removal.

## Compatibility and validation

UID KpGy; 782 append-only parameters; existing parameter IDs and ranges retained. Existing projects/expansion identities are supported; old RULE LOCK values migrate to BASIC plus a closed padlock. Signed Shift reports fixed 30ms latency including direct pads. Newer state cannot be fully represented by old engines. External sidechain and generated MIDI output are not implemented.

Native/ARM32/ARM64 builds and SDK validation pass. Engine, UBSan, allocation guards, sample/state/expansion/skin and real VST bridge tests pass. Host simulator:19 pass,0 warnings,0 failures. The owner confirmed 1.3.3 locks/Dice/RULE behavior on MPC XL OS3.10.1.12. Actual screenshots come directly from the authenticated MPC Remote Screen framebuffer stream, with capture provenance recorded. Complete physical-control coverage, all routes and worst-case hardware CPU are not certified. No ASan pass is claimed. See VERIFICATION.md.

Version1.4.0 adds16RULES per part, with a paged left list and full-range Q-Link control. See MANUAL.md and VERIFICATION.md.

Global Dice preserves every fill and FX value. Use dedicated ROLL FILL or ROLL FX with Undo. Only MAIN has individual Dice padlocks; meaningful kit/role/layer protection remains. UNLOCK ALL changes protection only. See MANUAL.md for the complete Dice policy.

Version1.6.0 adds 1/4T, 1/8T and 1/16T hats, each plain, +FILL or +ROLL. Tap Hat Rhythm for three pages of eight choices, or turn MAIN D1 in Screen mode. Triplets retain their spacing independently of straight swing. Live changes, MAIN Dice locks and Undo share the full selector; old779-parameter snapshots migrate without replacing their notes.

## Create your own packs

[Groove Dice Pack Builder](https://kimswork.app/apps/groove-dice-pack-builder/) is a separate optional app for macOS Apple Silicon and Windows x64. Organize WAV/MIDI folders, review roles, generate/audition kits and export independent `.gdexp` files. Read the bundled PACK_BUILDER.md or [complete guide](https://kimswork.app/apps/groove-dice-pack-builder/manual/). The instrument works with its included core without the builder.

## License

Groove Dice project code and Original Workshop core content are offered under MIT; see LICENSE and Content/Licenses/. Original synthesis and authored MIDI use no external audio/MIDI/model inputs. Optional expansions retain their own source rights; neither the plugin license nor a builder checkbox grants rights to another library. Pack Builder is distributed separately with its own dependency notices.

## 1.6.2 expansion-safe catalog upgrades

Open MPC packages declare `Content/Expansions` as user data so upgrades retain installed packs and their registry. Back up important projects and packs before upgrading; older1.6.1catalog packages did not declare this folder and should not be used for catalog upgrades. The normal SDK copy workflow still requires preserving your Content/Expansions directory. The engine, parameters, UI and original bank are unchanged.
