## What and why

Add Donut Theory by kim, an analog-inspired instrument with two oscillators, two delayed LFOs and editable chord slots. This PR changes only `catalog/plugins/donut-theory.json`.

- Source: https://github.com/Sunghyun23/kimswork/tree/donut-theory/v1.14.2/mpc-software/plugins/public/DonutTheory
- Project MIT license: https://github.com/Sunghyun23/kimswork/blob/donut-theory/v1.14.2/mpc-software/plugins/public/DonutTheory/LICENSE
- Release: https://github.com/Sunghyun23/kimswork/releases/tag/donut-theory/v1.14.2
- Homepage/manual: https://kimswork.app/plugins/donut-theory/
- Screenshot is an actual MPC XL capture pinned to the release source commit.
- `min_os: 3.10.1` reflects the supported official SDK route. No OS2.x/3.9.x support claim.
- This is a multi-product monorepo: the plugin-specific `asset_pattern` separates Donut Theory from the other products. This submission includes neither Groove Dice nor Crate Press.

### Packages and validation

Created both archives with upstream `tools/sdk_release.py` at commit `1d6bb4e4b795dd5961f18394616da812a8d32780`, using `--id donut-theory --repo Sunghyun23/kimswork --license MIT --asset-name donut-theory`. The standard converter retains original binaries, skins and Content byte-for-byte (245 retained runtime files per archive); only transport metadata and standard installers are added. Existing SDK release assets are unchanged. No ROMs, firmware or third-party sample libraries are included.

```text
catalog_check.py <armv7.zip> --catalog --require-portable --expect-id donut-theory --expect-repo Sunghyun23/kimswork
OK: donut-theory 1.14.2 (0 warnings)
catalog_check.py <aarch64.zip> --catalog --require-portable --expect-id donut-theory --expect-repo Sunghyun23/kimswork
OK: donut-theory 1.14.2 (0 warnings)
catalog_build.py --check-registry
289 entries, 6 collections, 0 problems
catalog_build.py --require-portable donut-theory
1 entries checked for the portable layout (0 more built for their dependencies), 0 problems
```

SHA-256:
- `donut-theory-1.14.2-mpc-armv7.zip`: `eeab4bc62577ba486da9a2f94892f5f229425f7a12dbf3c2d6e7a3d8b03429d7`
- `donut-theory-1.14.2-mpc-aarch64.zip`: `46bdafd19fa6b5b0bc3cb51f87304a4b824ee07544f23de96653144592eeed3a`

### Testing scope

Please list without a Verified claim; no `tested.json` is submitted. Existing SDK-installed1.14.2 QA covers actual MPC XL / OS3.10.1.12 remote UI, all seven pages and representative selection/Sync behavior; SDK host simulator19PASS/0WARN/0FAIL, native/ARM builds and skin validation pass. Physical encoder/audio/worst-case CPU/full project recall are not newly certified. The converted archives' legacy install/uninstall scripts were not run on a device, and no Gen1 bench is supplied.

Local upstream `tools/test_catalog.py` on macOS ran216tests with49failures, including installer/store shell tests (e.g. `tools/mpc-store.sh: command substitution: line60: syntax error near unexpected token newline`). No upstream tool or installer files are changed by this PR. Linux CI is requested; do not interpret the local full suite as passing. The dedicated registry, both archive checks and actual-release resolution above passed.

## Checks
- [ ] Offline first: upstream full catalog suite is not green on macOS; exact result noted above. Project host/skin checks and dedicated catalog validations pass.
- [ ] Full device verification of converted distribution: not claimed; existing device scope and remaining checks are documented above.
- [x] Catalog-conformant: both `catalog_check.py --catalog --require-portable` checks pass.
- [x] New catalog entry and screenshot contain no manufacturer product name or logo in the plugin name or skin art.
- [x] Starts as Listed; no `tested.json` or Verified claim.
- [x] Topic branch, one entry per PR, pinned screenshot and archive SHA-256 included.
- [x] No docs needed: registry-only addition, no tool, installer, workflow or product feature changes.
- [x] PR diff stays device-generic; no IPs, serials, Akai stock skins or Steinberg SDK.
