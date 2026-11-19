## What and why

Add Groove Dice by kim, a neo-soul/hip-hop drum instrument with one-bar groove generation, layered drums, rhythm rules, fill recording and independent expansion packs. This PR changes only `catalog/plugins/groove-dice.json`; Donut Theory is already listed from#1030.

- Source: https://github.com/Sunghyun23/kimswork/tree/groove-dice/v1.6.2/mpc-software/plugins/public/GrooveDice
- Project MIT license: https://github.com/Sunghyun23/kimswork/blob/groove-dice/v1.6.2/mpc-software/plugins/public/GrooveDice/LICENSE
- Core-content MIT/provenance: `Content/Licenses/Original-Workshop-MIT.txt` and `Original-Workshop-Provenance.txt` in that project and every package.
- Release: https://github.com/Sunghyun23/kimswork/releases/tag/groove-dice/v1.6.2
- Product/manual: https://kimswork.app/plugins/groove-dice/ and https://kimswork.app/plugins/groove-dice/manual/

### Original core and optional Pack Builder

The public core contains128 procedurally synthesized one-shots,96kit configurations reusing those sounds,2authored beats and93independently authored timing templates. Its generator reads no external audio/MIDI/model input. The bank SHA256 is `b98c4b813ff381fb5befca49bfa7385c5c7ace9d54355739569596cf8b81bb8f`. The actual SDK and portable archives were inspected: exactly one original bank, an empty expansion registry, and no private or commercial expansion payloads. Other optional packs keep their own source rights.

Pack Builder is an **optional, separate desktop app**, not a dependency of the instrument and not another entry in this PR. The core works without it. It turns the user's own WAV/MIDI folders into `.gdexp` packs and can manage them on locally accessible MPC storage. Documentation covers role folders, kit generation/audition, export, stable Pack ID, install/select/remove and private-vs-redistributable classification:

- Builder for macOS Apple Silicon and Windows x64: https://kimswork.app/apps/groove-dice-pack-builder/
- Builder manual: https://kimswork.app/apps/groove-dice-pack-builder/manual/
- Included walkthrough: `PACK_BUILDER.md` in the SDK ZIP and `Content/Documentation/PACK_BUILDER.md` in every portable folder.
- Builder release/checksums: https://github.com/Sunghyun23/kimswork/releases/tag/groove-dice-pack-builder/v1.0.0

Builder platform binaries and third-party notices are distributed separately; this entry does not relicense them. Exported packs keep the creator's source rights. The format is specific to Groove Dice, not MPC XPN.

### Packages and checks

The project uses the pinned official MPC SDK. Portable archives were made by upstream `tools/sdk_release.py` at commit864411da8fb0d88ad150d316ac6fea5f279113ea, with explicit id/repo/license/stem. The plugin-specific `asset_pattern` separates it from other products in the monorepo. `min_os: 3.10.1` reflects supported official SDK installation; no OS2.x/3.9.x support claim.

```text
catalog_check.py <armv7.zip> --catalog --require-portable --expect-id groove-dice --expect-repo Sunghyun23/kimswork
OK: groove-dice 1.6.2 (0 warnings)
catalog_check.py <aarch64.zip> --catalog --require-portable --expect-id groove-dice --expect-repo Sunghyun23/kimswork
OK: groove-dice 1.6.2 (0 warnings)
catalog_build.py --check-registry
290 entries, 6 collections, 0 problems
catalog_build.py --require-portable groove-dice
1 entries checked for the portable layout (0 more built for their dependencies), 0 problems
```

SHA-256:
- `groove-dice-1.6.2-mpc-aarch64.zip`: `e3298327e25e500c6246b13cc68b9c82310cd300b948503a07379ffb6520c7de`
- `groove-dice-1.6.2-mpc-armv7.zip`: `803c8c8cb11595284921a0c3a74131ec2abb50df940282f142ac8348f92fe1e4`

### Testing scope

1.6.2 fixes catalog packaging; DSP/UI/782parameter metadata and the original core bank are unchanged except native version10602. Current native/ARM/SDK, host19PASS/0WARN/0FAIL and skin checks pass. Existing1.6.1engine/bridge/authoring checks remain scoped to that version.

New public-only actual MPC XL / OS3.10.1.12 captures show128WAV/95MIDI. The SD public profile was temporarily installed through the official SDK; the owner NVMe was untouched. After capture, the original SD copy including owner packs was restored and all167files were verified SHA256-identical to the pre-test backup. The catalog screenshot is pinned to source commit e1811423e910f79bd28241ee79ed1e9289a25173: https://raw.githubusercontent.com/Sunghyun23/kimswork/e1811423e910f79bd28241ee79ed1e9289a25173/mpc-software/plugins/public/GrooveDice/design/device-1.6.2/public-main.png

Actual generated installers were tested in isolated Linux (MPC_INSTALL_TEST=1, temporary fake settings/targets only), never run on the real MPC. For both CPU archives the test reproduces1.6.1pack loss, verifies1.6.2upgrade and repeat-install preservation of pack/registry/nested bytes, then verifies uninstall retains declared user data. Test: https://github.com/Sunghyun23/kimswork/blob/groove-dice/v1.6.2/mpc-software/plugins/public/GrooveDice/tests/open_mpc_upgrade_test.py ; results: https://github.com/Sunghyun23/kimswork/blob/groove-dice/v1.6.2/mpc-software/plugins/public/GrooveDice/design/device-1.6.2/upgrade-test.log

Both top-level and portable-folder mpc-plugin.json files declare `"user_data": ["Content/Expansions"]`; the installer has matching USER_DATA. The public-only package wrapper always passes this flag and rejects missing declarations. Older1.6.1assets remain unchanged with an upgrade warning in their release notes.

Physical encoders/audio/CPU/all routes/full device recall remain uncertified. No tested.json or Verified claim. The new UI captures support the summary's scoped MPC XL UI statement, not wider hardware claims.

## Checks
- [x] Offline first: project host/skin, expansion install/remove, authoring and both catalog-package checks passed. Upstream Linux CI requested; the full upstream macOS shell suite is not claimed as passing.
- [x] Actual1.6.2public-core MPC XL UI captured; no full physical/audio/CPU certification claimed.
- [x] Catalog-conformant: both archives pass `catalog_check.py --catalog --require-portable`.
- [x] No manufacturer product name/logo in plugin name, skin art or screenshot.
- [x] Starts as Listed; no tested.json/Verified claim.
- [x] Topic branch, one entry per PR, pinned screenshot and archive SHA-256 included.
- [x] No docs needed: catalog-entry-only PR; product/source documents are in the author's repository. No upstream tool/installer/workflow change.
- [x] PR diff is device-generic; no IPs, serials, Akai stock skins or Steinberg SDK.
