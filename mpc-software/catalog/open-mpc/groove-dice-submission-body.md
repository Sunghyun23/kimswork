## What and why

Add Groove Dice by kim, a neo-soul/hip-hop drum instrument with one-bar groove generation, layered drums, rhythm rules, fill recording and independent expansion packs. This PR changes only `catalog/plugins/groove-dice.json`; Donut Theory is already listed from#1030.

- Source: https://github.com/Sunghyun23/kimswork/tree/groove-dice/v1.6.1/mpc-software/plugins/public/GrooveDice
- Project MIT license: https://github.com/Sunghyun23/kimswork/blob/groove-dice/v1.6.1/mpc-software/plugins/public/GrooveDice/LICENSE
- Core-content MIT/provenance: `Content/Licenses/Original-Workshop-MIT.txt` and `Original-Workshop-Provenance.txt` in that project and every package.
- Release: https://github.com/Sunghyun23/kimswork/releases/tag/groove-dice/v1.6.1
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
OK: groove-dice 1.6.1 (0 warnings)
catalog_check.py <aarch64.zip> --catalog --require-portable --expect-id groove-dice --expect-repo Sunghyun23/kimswork
OK: groove-dice 1.6.1 (0 warnings)
catalog_build.py --check-registry
290 entries, 6 collections, 0 problems
catalog_build.py --require-portable groove-dice
1 entries checked for the portable layout (0 more built for their dependencies), 0 problems
```

SHA-256:
- `groove-dice-1.6.1-mpc-aarch64.zip`: `e92a6338e46ab949d60ee25b626d1dcaa5483734046459ef45a94e3999483aae`
- `groove-dice-1.6.1-mpc-armv7.zip`: `079ee9f3f3ad528b7864d39e66a65d45a91660d5cc5ffe11fafb13b89de8b01a`

### Testing scope

1.6.1 is a documentation/licensing/distribution patch. Native version metadata is10601; the engine implementation, all782parameter declarations, exact UI/Q-Link bytes and original core bank match1.6.0. Native/ARM32/ARM64 builds and SDK validation pass; host simulator19PASS/0WARN/0FAIL; existing skin/expansion-manager checks and4Pack Builder authoring tests pass. Existing compiler warnings are retained, not represented as warning-free compilation.

Actual screenshot is the previous1.6.0MPC XL3.10.1.12 remote capture, pinned to its source commit; UI is unchanged. The owner device had optional packs installed (the library total in the header is not the public package's sound count). Earlier remote UI verification does not certify1.6.1on hardware. New physical encoders/audio/CPU/all routes/project recall are not claimed; portable legacy installers were not run on the device. No `tested.json` or Verified badge claim: please list at Listed tier.

## Checks
- [x] Offline first: project host/skin, expansion install/remove, authoring and both catalog-package checks passed. Upstream Linux CI requested; the full upstream macOS shell suite is not claimed as passing.
- [ ] New1.6.1device execution: not claimed; current packaging is offline-verified and previous device scope is stated above.
- [x] Catalog-conformant: both archives pass `catalog_check.py --catalog --require-portable`.
- [x] No manufacturer product name/logo in plugin name, skin art or screenshot.
- [x] Starts as Listed; no tested.json/Verified claim.
- [x] Topic branch, one entry per PR, pinned screenshot and archive SHA-256 included.
- [x] No docs needed: catalog-entry-only PR; product/source documents are in the author's repository. No upstream tool/installer/workflow change.
- [x] PR diff is device-generic; no IPs, serials, Akai stock skins or Steinberg SDK.
