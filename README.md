# kimswork

Kim’s Work: musical software and sound libraries for a vintage-inspired workflow.

- Website: [kimswork.app](https://kimswork.app/)
- Source: [Sunghyun23/kimswork](https://github.com/Sunghyun23/kimswork)
- [Repository policy](docs/repository-policy.md) · [Migration record](docs/structure-migration.md)

## Structure

```text
site/                                      Astro catalog, manuals, GitHub Pages
mpc-software/
  plugins/public/DonutTheory/               Released synth (1.15.0)
  plugins/public/CratePress/                Stereo effect (0.2.6 stable)
  plugins/public/GrooveDice/                Drum instrument (1.9.0 stable)
  plugins/public/FeltClub/                  Physical keys/plucks (0.3.2 stable)
  plugins/private/                         Other local-only plugin projects
  apps/public/GrooveDicePackBuilder/        Standalone desktop pack authoring app
  expansions/groove-dice/
    public/original-workshop/              Own sounds, kits and MIDI authoring
    private/splat-to-clat/                 Local-only source and optional pack
  shared/groove-dice-expansion/             Dedicated .gdexp manager/validator
  mpc-sample/                              MPC Sample hardware mods/tools/docs
    mods/public/LoopClub/                  Loop Club 16-track looper (0.9.3 preview)
  docs/                                    Product specifications and records
  scripts/                                 Build/test/bootstrap entry points
  third_party/mpc-plugin-builder-sdk/       One pinned, unmodified SDK checkout
```

Private projects and expansion payloads are Git-ignored and excluded from public packages. A `private` directory is not a GitHub access-control mechanism. Groove Dice public packages use the independently created Original Workshop core only. Groove Dice 1.9.0 ships only the original core; optional personal packs remain private.

## Development

From the repository root:

```sh
python3 mpc-software/scripts/bootstrap_sdk.py
python3 mpc-software/scripts/build_plugin.py DonutTheory --target all
python3 mpc-software/scripts/test_plugin.py
python3 mpc-software/plugins/public/GrooveDice/tools/build.py all
python3 mpc-software/plugins/public/GrooveDice/tools/test.py
```

Install developer dependencies from `mpc-software/requirements-dev.txt`. Follow the pinned SDK skills for building, testing and packaging. Each plugin, app and expansion has an independent version; moving folders does not change plugin UID, parameter IDs or historical release tags.

## Products

| Product | Status | Documentation |
|---|---|---|
| Donut Theory | [1.15.4 stable](https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.15.4) | [Handoff](mpc-software/docs/plugins/donut-theory/handoff.md), [manual](mpc-software/docs/plugins/donut-theory/manual.md) |
| Crate Press | [0.2.6 stable](https://github.com/Sunghyun23/kimswork/releases/tag/crate-press/v0.2.6), partial hardware verification | [Manual](mpc-software/docs/plugins/crate-press/manual.md), [verification](mpc-software/docs/plugins/crate-press/validation.md) |
| Groove Dice | [1.11.0 stable](https://github.com/Sunghyun23/kimswork/releases/tag/groove-dice/v1.11.0), original core | [README](mpc-software/plugins/public/GrooveDice/README.md), [verification](mpc-software/plugins/public/GrooveDice/VALIDATION.md) |
| Felt Club | [0.3.2 stable](https://github.com/Sunghyun23/kimswork/releases/tag/felt-club/v0.3.2) | [Manual](mpc-software/docs/plugins/felt-club/manual.md), [verification](mpc-software/plugins/public/FeltClub/design/validation-0.3.2/README.md) |
| Loop Club | [0.9.3 public preview](https://github.com/Sunghyun23/kimswork/releases/tag/loop-club/v0.9.3), MPC Sample1.3.0.12 | [Manual](mpc-software/mpc-sample/mods/public/LoopClub/docs/manual.md), [verification](mpc-software/mpc-sample/mods/public/LoopClub/docs/release-0.9.3.md) |
| Groove Dice Pack Builder | 1.0.0 standalone macOS/Windows app | [Project](mpc-software/apps/public/GrooveDicePackBuilder/README.md) |

MPC OS plugin installation requires standalone OS3.10.1+ and Enable Enhancements / External Plugin Support. Current NVMe builds are Donut Theory1.15.4, Groove Dice1.11.0, Felt Club0.3.2 and Crate Press0.2.6. Existing owner Groove packs/captures are preserved; public ZIPs contain only the original core. The latest SD copy is **pending**: card backup disconnected before any new SD write. Do not overwrite old card content without preserving it. SSD remains untouched. Open MPC submission/CI, hardware limits and actual-image scope are recorded in [current publication](mpc-software/docs/releases/2026-11-22.md).

Groove Dice 1.3.3 adds visible MAIN randomisation and16 persistent Dice locks, seven playable RULE selectors without a LOCK value, HATS/CYMBALS, tap-to-browse sample selection, isolated preview and Decay-first SOUND controls. Native/ARM builds, SDK validation and regressions pass; host simulator19 PASS /0 WARN /0 FAIL. The owner confirmed locks/Dice/RULE operation on MPC XL OS3.10.1.12. Public packages contain only Original Workshop. Release notes, manual and actual-device screenshots document remaining hardware limits.

Groove Dice1.3.4 adds two in-place SOUND pages (Sample/Level/Tune/Shift, then Start/Decay/LowCut/HighCut), independent REV and polarity, explicit48px page arrows and a visible counter. Actual MPC remote UI captures and per-input verification are recorded under the project design/device-1.3.4 folder; physical encoder and hardware-audio limits remain documented.

Latest updates: Donut Theory 1.15.0 adds main-screen Phase/FREE–RESET and eight waves per oscillator, retaining the default sound of all 66 factory programs. Groove Dice 1.9.0 separates MAIN and PERFORM Dice/Undo, preserving prepared phrases and master FX while retaining consistent footer icons and controls. Versioned evidence records automated and actual remote MPC checks separately from physical/audio/CPU coverage.

Donut Theory1.10.1 and Crate Press0.2.4 group plugin-owned browse/performance actions at the bottom while retaining original artwork and sound engines. Groove Dice1.4.0 stays unchanged. Actual MPC XL captures and detailed verification are stored per product.

Donut Theory1.12.0 combines Amp/Filter ADSR on ENV and adds Release Pitch Drop on PLAY D, with shared Amp Release and293parameters (first291retained). MAIN filters remain unchanged. Actual MPC XL captures, internal NVMe file hashes and full automated regressions accompany the release.

## Licenses

This monorepo uses project-level licenses rather than one blanket repository license. [Donut Theory](mpc-software/plugins/public/DonutTheory/LICENSE), [Groove Dice](mpc-software/plugins/public/GrooveDice/LICENSE), [Crate Press](mpc-software/plugins/public/CratePress/LICENSE), [Felt Club](mpc-software/plugins/public/FeltClub/LICENSE), the [Groove Dice expansion backend](mpc-software/shared/groove-dice-expansion/LICENSE) and the [original core generator/content](mpc-software/expansions/groove-dice/public/original-workshop/LICENSE) are MIT-licensed. Original Workshop content terms and SDK notices are included in the plugin Content/Licenses directories. Third-party SDK/components keep their own notices. Pack Builder dependencies have [separate notices](mpc-software/apps/public/GrooveDicePackBuilder/THIRD_PARTY.md); optional/private sound packs retain their own source rights and are not covered by the plugin MIT license.

Felt Club 0.3.2 includes seven isolated physical engines, 57 curated factory sounds, instrument-preserving Dice and a dedicated front-panel Noise control. Internal NVMe installation and all 600 runtime-file SHA-256s were independently read back on MPC XL / OS 3.10.1.12. All seven pages, factory browsing, Noise, locks, Dice and Undo were checked through Remote Screen. Native/both-ARM builds, SDK and host checks pass; hardware audio, physical Q-Links/pads, worst-case CPU and full MPC project recall remain unverified. Site audio is explicitly labeled native rendering. See the product verification record for precise coverage.

Current release update (2026-11-22): Donut Theory1.15.4, Groove Dice1.11.0 and Felt Club0.3.2 remove MIDI preset/Dice action commands. Groove alone retains six C Major performance triggers and C-bank direct drums. Crate Press0.2.6 remains stable unchanged. **Felt Club has a known owner-reported MPC CPU peak above100%; the optimization does not certify worst-case hardware load.** Public code is MIT within each public plugin folder; private expansions and commercial reference material are excluded.

Open MPC: [Felt Club](https://open-mpc.github.io/p/felt-club.html), [Donut Theory](https://open-mpc.github.io/p/donut-theory.html), [Groove Dice](https://open-mpc.github.io/p/groove-dice.html), [Crate Press](https://open-mpc.github.io/p/crate-press.html). All four updates are merged and live; SD copying remains pending device reconnection.

## MPC Sample hardware development

A separate [mpc-sample area](mpc-software/mpc-sample/README.md) holds device-specific Rust mods and a SampleBoot integration plan. Dependencies are independently pinned in sample-sdk.lock.json. Existing MPC VST products/SDK remain separate; no Sample mod or boot change is installed by setup.

Loop Club is separate MPC Sample software, requiring Sample firmware1.3.0.12 rather than MPC OS. Its public ZIP has original procedural drums only; individual USB drum stems and complete physical/power-cycle acceptance remain pending. Original Loop Club source is MIT licensed in its project folder.
