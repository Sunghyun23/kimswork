## What and why

Add Crate Press by kim, a public-preview stereo insert effect for MPC tracks and masters. It combines full-band/three-band compression, selective internal-bass or host-tempo ducking, drive, sampler color and wow/flutter with eight momentary/latched performance effects. This PR changes only `catalog/plugins/crate-press.json`.

- Source: https://github.com/Sunghyun23/kimswork/tree/crate-press/v0.2.5/mpc-software/plugins/public/CratePress
- Project MIT license: https://github.com/Sunghyun23/kimswork/blob/crate-press/v0.2.5/mpc-software/plugins/public/CratePress/LICENSE
- Pinned SDK MIT notice: `Content/Licenses/MPC-SDK-MIT.txt`, retained in every archive.
- Preview release: https://github.com/Sunghyun23/kimswork/releases/tag/crate-press/v0.2.5
- Product/manual: https://kimswork.app/plugins/crate-press/ and https://kimswork.app/plugins/crate-press/manual/

### Preview and feature scope

The author is preserving public-preview status:0.2.5 is a GitHub prerelease, not a promotion to stable. Experimental/beta filtering is appropriate; no `tested.json` or Verified badge is claimed. External sidechain input is not implemented. Ducking uses internal bass detection or host tempo. Peak Guard is a soft output bound, not a true-peak limiter.

Main controls and measured input/output extrema, static compressor curve and reduction displays are unchanged. LATCH lets multiple performance effects stay engaged independently. The eight effects are Tape Stop, Reverse, Repeat, Dub Echo, Filter Dip, Crush, Wobble and Dropout. It is an effect (`kind: effect`), not an instrument, and needs no extra plugin, ROM, sample bank or network connection at runtime.

### Packages and checks

0.2.5 is documentation/licensing/distribution only. Native version205; DSP,380parameter definitions,32factory programs and exact TUI/Q-Link bytes match0.2.4. Project code/programs/original runtime art are MIT, the official SDK remains unmodified with its own MIT notice. No proprietary firmware, ROMs, commercial audio/MIDI or stock manufacturer skins are included.

Both CPU archives were made by upstream `sdk_release.py` at commit864411da8fb0d88ad150d316ac6fea5f279113ea. Retained runtime files match the original SDK ZIP byte-for-byte; standard transport metadata and installer files are added. The explicit `crate-press-*-mpc-armv7.zip` pattern separates this product from other Releases in the monorepo. `min_os: 3.10.1` reflects supported official SDK installation, with no OS2.x/3.9.x claim.

```text
catalog_check.py <armv7.zip> --catalog --require-portable --expect-id crate-press --expect-repo Sunghyun23/kimswork
OK: crate-press 0.2.5 (0 warnings)
catalog_check.py <aarch64.zip> --catalog --require-portable --expect-id crate-press --expect-repo Sunghyun23/kimswork
OK: crate-press 0.2.5 (0 warnings)
catalog_build.py --check-registry
290 entries, 6 collections, 0 problems
catalog_build.py --require-portable crate-press
1 entries checked for the portable layout (0 more built for their dependencies), 0 problems
```

SHA-256:
- `crate-press-0.2.5-mpc-aarch64.zip`: `a432eb88bc666b84c408470fbdc79dfd179dea40e267dbcfff42e6ce3f718577`
- `crate-press-0.2.5-mpc-armv7.zip`: `fc99e1614fabeeb6aca10cc26ddebccc0d800a8d60352a71aba8548b9803bc78`

### Testing scope

Current native/ARM32/ARM64 build, SDK validator, UBSan engine/performance/host-bridge, all28effect pairs at3rates and skin contracts pass. Three SDK host simulator runs (44.1kHz/32and128frames;48kHz/512frames) each report20PASS/0WARN/0FAIL. No render-time allocation is observed by these tests. The full upstream macOS shell suite is not claimed as passing; upstream Linux CI is requested.

The actual MPC screenshot is pinned to its original0.2.4 source commit; UI is unchanged and the product page labels the capture0.2.4. Existing MPC XL3.10.1.12 remote checks cover track insert loading, footer curve/LATCH, momentary release and multiple latched states; earlier Live III evidence includes master loading/recall. They are not new0.2.5hardware certification. Physical encoders, finger multitouch, hardware audio/latency compensation, worst-case CPU and full device recall are not newly certified. No Gen1 bench supplied; converted legacy installers were not run on a device.

## Checks
- [x] Offline first: project engine/performance/combination/bridge, host and skin checks plus both catalog package validations pass. Upstream Linux CI requested.
- [ ] New0.2.5device execution: not claimed; existing screenshot/verification scope stated above.
- [x] Catalog-conformant: both `catalog_check.py --catalog --require-portable` checks pass.
- [x] No manufacturer product name/logo in plugin name, skin art or screenshot.
- [x] Preview-only entry: Experimental/beta from prerelease; no tested.json/Verified claim.
- [x] Topic branch, one entry per PR, pinned actual screenshot and archive SHA-256 included.
- [x] No docs needed: catalog-entry-only PR; product source/docs are in the author's repo. No upstream tool/installer/workflow change.
- [x] PR diff is device-generic; no IPs, serials, Akai stock skins or Steinberg SDK.
