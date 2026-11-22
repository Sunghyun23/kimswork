# MPC Sample development

This is the **MPC Sample hardware** area of the kimswork monorepo, not a folder of sample audio. Existing MPC OS VST projects remain in `mpc-software/plugins/` with their own SDK and releases.

```text
mpc-software/mpc-sample/
  mods/public/<Project>/    Independently versioned redistributable Rust mods
  mods/private/<Project>/   Owner-only source/content; Git-ignored
  tools/                    Local setup and read-only diagnostics
  docs/                     Platform, compatibility and SampleBoot notes
mpc-software/third_party/
  sample-sdk.lock.json       Separate exact dependency/firmware contract
  mpc-sample-sdk/            Ignored, unmodified Rust SDK checkout
  sampleboot/                Ignored, unmodified boot-selector source
```

The infrastructure itself does not assign product identities. The first owner-selected product is [Loop Club](mods/public/LoopClub/README.md), by kim, currently a native development prototype. A product is created only after those choices are made. Keep each product's code, state, tests and release identity independent. Extract shared DSP only when actual reuse warrants it.

## Prepare

```sh
python3 mpc-software/scripts/bootstrap_sample_sdk.py --with-sampleboot
python3 mpc-software/mpc-sample/tools/check_environment.py
python3 mpc-software/mpc-sample/tools/inspect_device.py
```

The device inspector only GETs the connected device's `/version.json`; default URL is `http://mpc-sample.local`. Pass `--base-url http://<device-address>` if name resolution is unavailable. It never updates, installs, restarts, changes settings or reads the SD card. A matching version string does not establish a matching application fingerprint.

The selected SDK0.7.3needs Rust1.97+, Cargo, uv, Zig, LLVM tools, Python and zip for ARM64 packaging. See [development requirements](docs/development.md). Existing native/ARM VST toolchains are unchanged. Sample has a320×240screen and three parameter knobs; existing1280×800skins and16Q-Link layouts do not transfer directly.

## Current scope

Dependency preparation and Loop Club native prototype development. SampleBoot, the preserved SampQOL entry and the Loop Club development mod have now been copied to the owner-confirmed Sample SD. No firmware/internal storage changes or mod launch have occurred. USB identity and actual firmware1.3.0.12 were read on2026-11-22. No app hash, hooks, hardware audio, control mapping or SampleBoot operation was device-verified. [Compatibility and boot design](docs/platform.md).

Local validation:126SDK unit tests+1doc test, strict Clippy and ARM64 cargo check pass with isolated Rust1.97.0. The newer0.7.4was rejected due to its own test/lint failures, not silently patched. Run `python3 mpc-software/mpc-sample/tools/check_sdk.py` to reproduce.

## 16-track looper requirements

Current review draft: [SRS0.1](docs/looper-srs.md). It distinguishes owner decisions, proposals and open items; Loop Club has an initial native core/controller implementation; SDK integration, eight device views and a local guarded package are implemented; SD copy and SHA256 read-back are complete; physical boot/audio/controls/CPU verification is pending. See [SD installation](mods/public/LoopClub/docs/sd-installation.json). Latest FX-section keys are B1MIX/B2TRACK FX/B3MASTER FX; K1-K3 remain rotary parameter controls.

## Latest public product status — 2026-11-22

Loop Club0.1.0 is the first public preview, authorized by the owner. This supersedes historical source-only/pending-publication notes above. See its current README/manual and release verification. Public downloads exclude optional commercial banks and QA agents. Actual USB output/remote UI are tested; physical controls/power-cycle acceptance and individual drum USB stems remain outstanding.
