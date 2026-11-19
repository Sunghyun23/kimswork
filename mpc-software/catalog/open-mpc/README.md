# Open MPC catalog integration

Donut Theory is the first submission. Groove Dice and Crate Press are outside this submission; resolve their project licensing before adding entries.

The source catalog registry entry is `donut-theory.json`. A one-file topic-branch PR submits it to `open-mpc/mpc-vst-plugins`; maintainers decide acceptance. Registration is not automatic publication on their site.

Submitted: https://github.com/open-mpc/mpc-vst-plugins/pull/1030 (awaiting upstream review/merge). Source diff contains only `catalog/plugins/donut-theory.json`. Package/registry/live-release resolution passed; the local upstream full suite ran216tests with49failures on macOS, including upstream shell parsing/installer failures. GitHub workflow44514755498 is `action_required`, awaiting upstream execution approval. Linux CI and review remain upstream decisions; see the submission body and CI record.

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
