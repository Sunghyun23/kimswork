# Platform and boot contract

Sources checked2026-11-22:

- [Sample SDK0.7.3](https://github.com/ioma8/mpc-sample-sdk/tree/c7d54aaf77a8848437b7e89bd191c0f72689ac5e), MIT. Rust cdylib/preload mods targeting Sample1.3.0.12; events, drawing, sample objects, audio and MIDI. Its loader verifies firmware before hooks. This is community software, not the official inMusic VST SDK.
- [SampleBoot1.0.2](https://github.com/pxcm/SampleBoot/tree/d05cfc23acf7ee4d4520674b66db24bcd4a919b2), MIT. SD autorun launcher tested upstream on1.3.0.12. Starts the stock app or one selected system; not a kernel/partition multiboot or simultaneous-plugin host.
- [Open MPC Sample notes](https://github.com/open-mpc/mpc-vst-plugins/blob/main/docs/MPC_SAMPLE.md), community observations; version-sensitive, not this device's verified configuration.320×240display, RK3566/A55budget and distinct loader/UI controls require separate testing. Do not inherit its historical power descriptions as hardware facts.

Planned SD layout, not installed:

```text
autorun/                       SampleBoot's own verified release
  sampleboot/                  Launcher files/settings
  mods/
    <OurMod-Version>/          Our complete autorun package, relocated intact
```

Each mod must locate itself using its launch script path and locate the SD root using mounts. Keep user samples/projects separate from distributable code. Removing/renaming autorun and rebooting returns to stock according to upstream; that recovery path has not been tested on this device. A stock choice remains available. Back up existing autorun before installation; do not install this layout during platform preparation.

## Actual device evidence

Read-only USB/network inspection found MPC Sample and its USB NCM interface. Its own updater's GET `/version.json` returned:

```json
{"fullVersion":"1.3.0.12-release-g40e46ca7991","version":"1.3.0.12"}
```

This matches the documented version target only. SSH was closed; no login was attempted, no SSH access was enabled, no firmware/update request was sent, and SD content was not read. Device-specific network addresses and identifiers stay out of public source. App fingerprint, mods, SampleBoot, physical inputs/audio/CPU and recovery remain unverified.

## SDK selection evidence

Latest0.7.4was reviewed first, unmodified. Its unit suite passed125cases and failed the firmware fingerprint-count assertion (63vs64). Strict Clippy also found four errors in new editor-related code. Its ARM64 cargo check passed, which does not override those failures.0.7.4is not the selected baseline.

Previous0.7.3passes126unit tests,1doc test, strict Clippy and ARM64 cargo check on the isolated Rust1.97.0toolchain. Pin this reviewed commit without modifying its sources. No firmware guards were removed and no checks were skipped. This is local code/toolchain evidence, not device hook or boot certification.
