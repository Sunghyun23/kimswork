# Open MPC catalog integration

Donut Theory is registered. Groove Dice1.6.1is the second submission, with explicit MIT source/core notices, original-only packages and optional Pack Builder documentation. Submitted PR: https://github.com/open-mpc/mpc-vst-plugins/pull/1035 ; upstream Linux CI44527338657passed, awaiting upstream review/merge. The public Release and updated kimswork.app guide are live. Crate Press remains outside these submissions; resolve its project licensing before adding an entry.

The source catalog registry entry is `donut-theory.json`. A one-file topic-branch PR submits it to `open-mpc/mpc-vst-plugins`; maintainers decide acceptance. Registration is not automatic publication on their site.

Merged: https://github.com/open-mpc/mpc-vst-plugins/pull/1030 on2026-11-19 (merge864411da8fb0d88ad150d316ac6fea5f279113ea). Live catalog: https://open-mpc.github.io/p/donut-theory.html ; latest1.14.2, Listed tier. Upstream PR CI44514755498 and catalog build44517697971 succeeded. Package/registry/live-release resolution and published checksum match passed. The earlier macOS local upstream suite failures remain historical evidence, not the current upstream CI state. No Verified claim was added. See submission.json and published-entry.json.

## Packaging future releases

Keep normal SDK packages and existing public assets immutable. Add two explicit per-CPU catalog assets to the same plugin-specific Release. Monorepo entries use `donut-theory-*-mpc-armv7.zip`, so unrelated software Releases are ignored. Require OS3.10.1+, preserve kim/Krql identity and all existing parameter IDs. Do not promise legacy OS2.x/3.9.x or other MPC models.

Tools used for the first submission: upstream commit `1d6bb4e4b795dd5961f18394616da812a8d32780`. Obtain that repository separately; do not modify or replace our pinned official SDK.

From this repository root, with the external tools checkout at `/path/to/open-mpc`:

```sh
python3 /path/to/open-mpc/tools/sdk_release.py mpc-software/plugins/public/DonutTheory/dist/donut-theory-1.14.2-mpc.zip --version 1.14.2 --id donut-theory --repo Sunghyun23/kimswork --license MIT --asset-name donut-theory --out /tmp/donut-openmpc-dist
python3 /path/to/open-mpc/tools/catalog_check.py /tmp/donut-openmpc-dist/donut-theory-1.14.2-mpc-armv7.zip --catalog --require-portable --expect-id donut-theory --expect-repo Sunghyun23/kimswork
python3 /path/to/open-mpc/tools/catalog_check.py /tmp/donut-openmpc-dist/donut-theory-1.14.2-mpc-aarch64.zip --catalog --require-portable --expect-id donut-theory --expect-repo Sunghyun23/kimswork
```

Compute external SHA256 files, compare all retained runtime bytes against the original SDK archive, publish both archives/checksums, and run `catalog_build.py --check-registry` plus `catalog_build.py --require-portable donut-theory` from the external checkout. Update screenshot pins when the UI changes. Retain project MIT and third-party notices. Never include private expansions.

The converter adds Open MPC standard installers and metadata. They are catalog transport artifacts; do not run their system-modifying legacy install/uninstall or benchmark scripts on our MPC. Our only remote installation path remains the official SDK `mpc_remote.py`. Actual device verification and offline package validation are distinct. No `tested.json` or Verified badge is claimed by this first submission.

## Groove Dice releases

Apply the same per-CPU conversion to the **public profile ZIP** emitted by Groove Dice tools/package_profiles.py, never the installed owner folder. Use id/stem groove-dice, SPDX MIT and explicit `groove-dice-*-mpc-armv7.zip` asset pattern. Include the core-content notices and Content/Documentation/PACK_BUILDER.md in both CPU packages. Pack Builder is optional and distributed separately for macOS Apple Silicon/Windows x64; link its pinned release/manual and do not relicense its dependencies or user packs. Check that only default.gdbank is present, its SHA matches the original procedural core and the expansion registry is empty. Do not run legacy installer scripts on the MPC. Reference: groove-dice-submission-body.md and the project design/release-1.6.1 audit.
