# Groove Dice 1.0.0 — Original Workshop

A neo-soul, hip-hop and R&B drum instrument by kim for standalone Akai MPC.

Includes128 procedurally synthesized one-shots and96 playable kit configurations. Kick, snare, closed/open hats, clap, tom, cymbal and percussion are all generated for this project. Two authored MIDI beats and93 timing templates are included; no third-party sample-pack recordings or MIDI are bundled in this edition.

MAIN offers four Q-Link groups:

| Group | Controls |
|---|---|
| A — Timing | Swing / Snare Late / Hat Shift / Random Nudge |
| B — Rules | Kick / Snare / Hat / Perc variation rules |
| C — Sounds | Kick / Snare / Hat Pair / Perc Set |
| D — Character | Drive / Dust / Room / Hat Rhythm |

Hat Rhythm has15choices: quarter/eighth/sixteenth/dotted eighth/dotted sixteenth, each plain/Fill/Roll. Choose a rhythm, then use a Pattern-scope ROLL DICE. Snare backbeats stay on2/4; kick motifs avoid those exact beats. Record your own fills from the physical pads with EDIT/REC. Internal Duck remains onFX. External sidechain routing is not implemented.

Expansion kits appear in the kit browser and MPC's native PRESET list. Browser header arrows switch between all installed packs and an individual pack. ROLL DICE respects that filter. Expansion payloads remain separate files; adding a pack does not require recompiling the engine. The included Python installer manages Groove Dice `.gdexp` files on a locally accessible plugin Content directory. Reopen the plugin after installing/removing a pack.

MPC OS3.10.1or newer is required. Enable both Enable Enhancements and External Plugin Support in Preferences → Enhancements (Beta). Copy the complete `kim - VST - Groove Dice` folder to `VST2 Plugins` on a drive, then restart/rescan the MPC application if necessary. See INSTALL.txt.

This is the first stable public release. Verification covers the builds and documented automated/device checks; complete physical Q-Link coverage, every output destination and worst-case hardware performance are not certified. See the verification summary supplied with the package. Previously saved projects may reference optional expansions; removing a required pack reports missing samples instead of substituting unrelated sounds. Original0.8andolder engines cannot read new expansion identities.

The SDK's MIT notice and original-content provenance are included under Content/Licenses. Optional expansions retain their own terms. No personal-only expansion is bundled in this edition.
