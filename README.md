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
  plugins/public/GrooveDice/                Drum instrument (1.8.1 stable)
  plugins/private/                         Other local-only plugin projects
  apps/public/GrooveDicePackBuilder/        Standalone desktop pack authoring app
  expansions/groove-dice/
    public/original-workshop/              Own sounds, kits and MIDI authoring
    private/splat-to-clat/                 Local-only source and optional pack
  shared/groove-dice-expansion/             Dedicated .gdexp manager/validator
  docs/                                    Product specifications and records
  scripts/                                 Build/test/bootstrap entry points
  third_party/mpc-plugin-builder-sdk/       One pinned, unmodified SDK checkout
```

Private projects and expansion payloads are Git-ignored and excluded from public packages. A `private` directory is not a GitHub access-control mechanism. Groove Dice public packages use the independently created Original Workshop core only. Groove Dice 1.8.1 ships only the original core; optional personal packs remain private.

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
| Donut Theory | [1.15.0 release](https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.15.0) | [Handoff](mpc-software/docs/plugins/donut-theory/handoff.md), [manual](mpc-software/docs/plugins/donut-theory/manual.md) |
| Crate Press | [0.2.6 stable](https://github.com/Sunghyun23/kimswork/releases/tag/crate-press/v0.2.6), partial hardware verification | [Manual](mpc-software/docs/plugins/crate-press/manual.md), [verification](mpc-software/docs/plugins/crate-press/validation.md) |
| Groove Dice | [1.8.1 stable](https://github.com/Sunghyun23/kimswork/releases/tag/groove-dice/v1.8.1), original core | [README](mpc-software/plugins/public/GrooveDice/README.md), [verification](mpc-software/plugins/public/GrooveDice/VALIDATION.md) |
| Groove Dice Pack Builder | 1.0.0 standalone macOS/Windows app | [Project](mpc-software/apps/public/GrooveDicePackBuilder/README.md) |

MPC installation requires standalone OS 3.10.1+ and Enable Enhancements / External Plugin Support. Owner-device installs now target the internal NVMe, not SD or USB. Latest device operation: local candidates Donut Theory 1.15.2 (244 files), Groove Dice 1.8.1 (181 files) and Crate Press 0.2.6 (165 files) are installed in `/nvme/VST2 Plugins/` on MPC XL, verified by independent SHA-256 readback. Their shared UNLOCK ALL / ROLL DICE / UNDO footer was checked across all pages; Crate Press adds protected randomisation of 13 character controls. All 19 owner expansion/capture files were preserved. SD and SSD were untouched. Donut Theory remains an unpublished local candidate; Groove Dice 1.8.1 is now approved for stable publication. Crate Press 0.2.6 is a stable release; the product table above lists public versions. Re-read device drives after a hardware change; numeric drive selections are not portable. Actual device verification and remaining limits are recorded per product. Pages uses the existing `site/` project and `kimswork.app`; local restructuring does not itself publish a new site or Release.

Groove Dice 1.3.3 adds visible MAIN randomisation and16 persistent Dice locks, seven playable RULE selectors without a LOCK value, HATS/CYMBALS, tap-to-browse sample selection, isolated preview and Decay-first SOUND controls. Native/ARM builds, SDK validation and regressions pass; host simulator19 PASS /0 WARN /0 FAIL. The owner confirmed locks/Dice/RULE operation on MPC XL OS3.10.1.12. Public packages contain only Original Workshop. Release notes, manual and actual-device screenshots document remaining hardware limits.

Groove Dice1.3.4 adds two in-place SOUND pages (Sample/Level/Tune/Shift, then Start/Decay/LowCut/HighCut), independent REV and polarity, explicit48px page arrows and a visible counter. Actual MPC remote UI captures and per-input verification are recorded under the project design/device-1.3.4 folder; physical encoder and hardware-audio limits remain documented.

Latest updates: Donut Theory 1.15.0 adds main-screen Phase/FREE–RESET and eight waves per oscillator, retaining the default sound of all 66 factory programs. Groove Dice 1.8.0 adds musical roll/fill motifs, four hat rolls and selected-slot controls. Versioned evidence records automated and actual remote MPC checks separately from physical/audio/CPU coverage.

Donut Theory1.10.1 and Crate Press0.2.4 group plugin-owned browse/performance actions at the bottom while retaining original artwork and sound engines. Groove Dice1.4.0 stays unchanged. Actual MPC XL captures and detailed verification are stored per product.

Donut Theory1.12.0 combines Amp/Filter ADSR on ENV and adds Release Pitch Drop on PLAY D, with shared Amp Release and293parameters (first291retained). MAIN filters remain unchanged. Actual MPC XL captures, internal NVMe file hashes and full automated regressions accompany the release.

## Licenses

This monorepo uses project-level licenses rather than one blanket repository license. [Donut Theory](mpc-software/plugins/public/DonutTheory/LICENSE), [Groove Dice](mpc-software/plugins/public/GrooveDice/LICENSE), [Crate Press](mpc-software/plugins/public/CratePress/LICENSE), the [Groove Dice expansion backend](mpc-software/shared/groove-dice-expansion/LICENSE) and the [original core generator/content](mpc-software/expansions/groove-dice/public/original-workshop/LICENSE) are MIT-licensed. Original Workshop content terms and SDK notices are included in the plugin Content/Licenses directories. Third-party SDK/components keep their own notices. Pack Builder dependencies have [separate notices](mpc-software/apps/public/GrooveDicePackBuilder/THIRD_PARTY.md); optional/private sound packs retain their own source rights and are not covered by the plugin MIT license.
