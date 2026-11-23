# Current installation status

See [autonomous QA and installation evidence](qa-autonomous-2026-11-22.md) for the current direct-boot development build and exact verification limits. The SampleBoot instructions below describe the earlier candidate and are retained only as history. Current owner instruction is direct Loop Club without a mod menu.

# Loop Club installation candidate

**Local development build; physical boot, controls, latency, audio and CPU remain unverified.** This is not a public release. The connected Sample's version API reports 1.3.0.12; its app hash has not been read locally. Package metadata explicitly labels the expected first-4KiB hash as the pinned SDK's upstream reference. The launcher measures the actual app prefix on the device and compares it before preloading; the unmodified SDK then verifies its hook fingerprints before installation.

## Prepare locally

```sh
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/validate.py
```

This checks the native engine/controller/runtime, allocation behavior, shared layout, ARM64 library, glibc imports and candidate launcher. It creates `dist/loop-club-0.1.0-sample-development.zip`. It does not touch a device or SD card. Host previews under `build/previews/` are generated from the shared drawing code, not captured from a Sample.

SampleBoot is separately pinned to 1.0.2. Its downloaded release checksum is `61ea76816035496a969690c514a05fa7ba10dc0e8a447ec7233f59bc05c17c9f`, verified against GitHub's release asset digest. The local cache is `.sdk-tools/mpc-sample/downloads/SampleBoot-1.0.2.zip`.

## SD preparation

Save the current native project and power the device off before moving its card. Preserve existing autorun/projects. The owner must identify the intended card; never use an inferred old MPC XL address/volume. The Mac helper defaults to a read-only plan:

```sh
python3 mpc-software/mpc-sample/mods/public/LoopClub/tools/install_card.py --card '/Volumes/ACTUAL CARD'
```

It requires the exact root of an external mounted FAT/exFAT volume. If autorun exists but is not recognized as SampleBoot, it refuses by default. For inspected SampQOL only, `--migrate-sampqol` stages a SampleBoot tree, verifies copies of every original boot file, retains the original under a hidden backup folder and preserves SampQOL as its own menu entry. Activation failure restores the original root. Unrecognized boot systems are still refused. This is a boot/config backup, not a full SD backup. An existing Loop Club version folder is also refused. `--apply` creates only a new installation/version folder and verifies every new Loop Club file by SHA-256. It does not format, delete projects or restart the MPC. A failed partial copy is not a successful installation; inspect it before retrying.

Expected layout:

```text
<card>/
  autorun/
    autorun.sh / autorun.json / autorun.check  # SampleBoot
    sampleboot/
    mods/Loop Club 0.1.0/                     # complete Loop Club autorun payload
  LoopClub/
    observed-app-prefix-sha256.txt            # actual startup measurement, when run
    Projects/
      startup.json                           # emitted after SDK startup and an output callback
      Session.lcproj                         # explicit Save; restored stopped on next launch
      Session.previous.lcproj                # preserved previous session before Save
```

The launcher locates itself from its script path and the card from the longest matching mount. It works at card-root autorun or relocated inside SampleBoot, including spaces in card names. Library replacement in `/run` is atomic. The systemd drop-in is in `/run` only, not persistent internal storage. Firmware/library mismatches do not call restart. The direct-autorun manifest asks for confirmation because startup restarts the stock app.

## First device test

1. Use a saved/stopped or empty native project. Choose Loop Club in SampleBoot. If a native page appears first, press SAMPLE to enter the Loop Club LOOP overlay.
2. Verify all four pad rows, B1–B3, DATA, left fader and K1–K3 against the manual. The continuous repaint path and control delivery need actual latency/CPU checks.
3. Verify input source capture, count-in exclusion, one-/three-bar loops, MIDI grid and note termination, independent track/kit levels, effect HOLD/LATCH and save/reload.
4. Check actual audio under 16-track/voice/FX loads. Mac benchmark percentages do not predict Sample CPU.
5. Read `startup.json` and the observed hash after safely shutting down/remounting. Their presence is startup evidence, not full acceptance.

The owner-confirmed Sample SD now contains SampleBoot1.0.2, the preserved SampQOL entry, Loop Club0.1.0 and three separate personal banks. Every mod/bank copy was SHA256-read back. The original SampQOL boot/config has verified Mac and card backups. macOS/exFAT normalizes AppleDouble sidecars, so their exact bytes are retained in `autorun-original.zip` and the moved original; the active SampQOL clone retains runtime files unchanged. No device restart or mod launch has occurred. See [installation record](sd-installation.json). The earlier stopped MPC XL SD-copy task remains stopped.

## Return to stock

According to upstream, power off, remove the card or rename its root autorun, then power on to start stock. SampleBoot also offers its stock entry. This recovery and menu-switch path has not yet been verified on the connected Sample. Returning from the Loop Club overlay through PROJECT → EXIT leaves hooks loaded until reboot; it stops Loop Club and forwards native controls, rather than claiming to unload a live library.

Sources: [pinned SDK install guide](https://github.com/ioma8/mpc-sample-sdk/blob/c7d54aaf77a8848437b7e89bd191c0f72689ac5e/docs/22-build-and-install.md), [SampleBoot 1.0.2](https://github.com/pxcm/SampleBoot/releases/tag/v1.0.2), [OS author relocation notes](https://github.com/pxcm/SampleBoot/blob/d05cfc23acf7ee4d4520674b66db24bcd4a919b2/docs/OS-AUTHORS.md).

## Optional personal banks

Place standalone `.lcbank` files in `<card>/LoopClub/Expansions/`, outside autorun. The host helper accepts repeated `--bank` arguments and defaults to a read-only plan:

```sh
python3 tools/install_banks.py --card '/Volumes/ACTUAL CARD' --bank '/absolute/path/personal.lcbank' --allow-private
```

Only add `--apply` for an owner-identified card. All source/destination identities are checked before writing; different existing banks are refused, identical banks are SHA256-verified and kept. Every new copy is independently hashed back. It never formats, deletes projects or restarts the device. Personal bank files are not part of the mod ZIP. Do not launch Loop Club while the card is exported to the Mac; finish copying and safely leave SD access first.

## First launch after the completed copy

Safely eject the card or leave native SD access before restarting the Sample. In SampleBoot choose **Loop Club0.1.0**; if the native page appears first, press SAMPLE to enter LOOP. Start with an empty/stopped native sequence to avoid native playback doubling. Check B1/B2/B3 and pad/knob/fader ownership, then a short audio and MIDI loop. For the new drum banks, choose a DRUM track → SAMPLE SELECT → bank → kit → B2 Load, then audition pads1–6/8/14 and verify hats choke. File copy success is not boot/audio/CPU certification.

To restore the original boot exactly, first power off and connect the SD to the Mac, move the active `autorun` aside, and move the installation record's `.LoopClubBackups/<stamp>/autorun-original` back to the card root. Do not delete the current tree or existing projects. The same backup includes a verified ZIP of every original boot file, including sidecars. The card root's original `mpcqol.conf` remains unchanged. Actual boot-menu switching and recovery remain unverified until tested on the device.

## Current owner direction: direct development boot

The latest owner request removes SampleBoot from the active boot path and retains only Loop Club QA4. Historical multi-menu instructions above no longer describe the target installation. The new candidate `dist/loop-club-0.1.0-sample-usb-qa-direct.zip` has a root autorun and no start question. It retains firmware/fingerprint/integrity checks and the opt-in transient USB development agent. This is not a firmware replacement.

Use `tools/install_direct.py --card <mounted confirmed card> --volume-uuid <confirmed UUID> --archive <direct ZIP> --apply`. Dry-run is the default. It verifies the archive and direct-boot manifest, builds/read-backs new content separately, archives every existing boot byte including AppleDouble, moves the intact original autorun into `.LoopClubBackups/<stamp>/autorun-original`, then atomically activates the new root. Failed activation restores the original. It does not touch projects, samples, private banks or mpcqol.conf. Previous menu versions remain only in the recovery backup, never as active menu entries.3host migration checks and8launcher checks pass; actual direct boot remains pending SD access/install.
