# kimswork

Kim’s Work: musical software and sound libraries for a vintage-inspired workflow.

- Website: [kimswork.app](https://kimswork.app/)
- Source: [Sunghyun23/kimswork](https://github.com/Sunghyun23/kimswork)
- [Repository policy](docs/repository-policy.md) · [Migration record](docs/structure-migration.md)

## Structure

```text
site/                                      Astro catalog, manuals, GitHub Pages
mpc-software/
  plugins/public/DonutTheory/               Released synth (1.2.3)
  plugins/public/CratePress/                Stereo effect (0.2.3 public preview)
  plugins/public/GrooveDice/                Drum instrument (1.2.2 stable)
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

Private projects and expansion payloads are Git-ignored and excluded from public packages. A `private` directory is not a GitHub access-control mechanism. Groove Dice public packages use the independently created Original Workshop core only. Groove Dice 1.2.2 ships only the original core; optional personal packs remain private.

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
| Donut Theory | [1.2.3 release](https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.2.3) | [Handoff](mpc-software/docs/plugins/donut-theory/handoff.md), [manual](mpc-software/docs/plugins/donut-theory/manual.md) |
| Crate Press | 0.2.3 public preview, partial hardware verification | [Manual](mpc-software/docs/plugins/crate-press/manual.md), [verification](mpc-software/docs/plugins/crate-press/validation.md) |
| Groove Dice | 1.2.2 stable, public source with original core | [README](mpc-software/plugins/public/GrooveDice/README.md), [verification](mpc-software/plugins/public/GrooveDice/VALIDATION.md) |
| Groove Dice Pack Builder | 1.0.0 standalone macOS/Windows app | [Project](mpc-software/apps/public/GrooveDicePackBuilder/README.md) |

MPC installation requires standalone OS 3.10.1+ and Enable Enhancements / External Plugin Support. Owner-device installs now target the internal NVMe, not SD or USB. Latest verified owner installs: Crate Press 0.2.2 (0.2.3 changes metadata/docs only), Donut Theory 1.2.3 and Groove Dice 1.2.2 (including personal expansions) are installed in `/nvme/VST2 Plugins/` on the MPC XL. Re-read device drives after a hardware change; numeric drive selections are not portable. Actual device verification and remaining limits are recorded per product. Pages uses the existing `site/` project and `kimswork.app`; local restructuring does not itself publish a new site or Release.
