# Actual-device USB QA candidate

This candidate adds an explicit, opt-in QA service at `192.168.155.1:8090`. The normal package never sets `LOOP_CLUB_QA=1` and never starts the server. It binds only the USB address, accepts only USB-subnet peers, and rejects browser-origin input requests. No USB gadget, firmware, authentication, network configuration or native project is changed by enabling it.

- `GET /screen.bmp` (QA3): follows the active DRM CRTC framebuffer and reads its validated320×240XRGB pixels. No modeset, master takeover, display write, kernel patch or host rendering. Unsupported format, permissions or mapping returns an error; no fb0 fallback. Readback can tear if the app updates during capture.
- `GET /status.json`: live Loop Club state, page, paint count and physical/remote button callback counters.
- `POST /input` with JSON: bounded button/pad/knob/encoder commands. UI changes run only in the live owning paint callback and use the same controller handlers. The remote controller path does not reproduce the SDK's physical input lock/forwarding path, which needs a separate physical-button test.

Build with `python3 tools/package.py --qa`. The ZIP is `dist/loop-club-0.1.0-sample-usb-qa.zip`, separate from the normal development ZIP. Install it into a new folder using `install_card.py --archive ... --folder-name 'Loop Club USB QA'`; existing mod versions are retained.

Save current work before leaving the device for SD access/restart. After booting the QA entry, press SAMPLE once to enter Loop Club if necessary. UI remote inputs depend on Sample-page painting; if the native SEQ page takes over, framebuffer capture remains available, but the physical SAMPLE button may be needed to recover. Capture this condition instead of claiming the native page exit is fixed.

User-reported issues remain open: repeated SEQ can escape to native SEQ; text is too small; knob/value alignment and mode discoverability need actual-screen review. Source inspection found that SDK `with_mod` forwards events when its nonblocking UI mutex is busy; it is a candidate cause, not a physically proven diagnosis. The pinned SDK and hook fingerprints remain unmodified.

## First actual-screen verification pass

1. Preserve the owner session; boot only the separate USB QA entry. Open SAMPLE once if needed.
2. Use `tools/qa_client.py --capture <new-path.png>` to save actual LCD pixels and a live-state JSON sidecar. Keep owner captures in ignored build/private storage.
3. Capture LOOP, PROJECT/type selection, SYNTH NOTES/CHORD and all7editor pages, DRUM bank/kit browsing, GRID, CHOP modes and MIX/FX.
4. Repeated physical SEQ presses need a before/after LCD capture and callback count. Remote queued buttons alone cannot certify the SDK physical-input path. If native SEQ takes over, capture it before recovering with physical SAMPLE.
5. Fix typography/knob alignment against native320x240captures, using the three parameter columns as physical-knob anchors. Then repeat both remote controller and physical routing checks.
6. Record actual results separately from the103host tests and launcher fixtures. No actual LCD capture or physical fix has been completed at QA preparation time.

## QA2 follow-up

The owner reported LOOP visible after QA1 launch, but the USB QA port remained closed while updater HTTP80 was reachable. No actual screenshot was obtained. The launch unit's printf output was checked independently and did contain real newline-separated Environment assignments; do not report a proven environment-string bug.

QA2 embeds a compile-time build identity and displays `Loop Club QA2`, so the selected library can be distinguished from the original. It writes startup/thread/bind errors and read-only systemd service properties under `<card>/LoopClub/QA-qa2`. The existing file worker attempts up to20actual framebuffer captures at3-second intervals starting after5seconds. Failed captures leave errors, never host-rendered substitutes. USB bind retries cover120seconds of late enumeration. Existing standard and QA1 menu entries are retained.

## Actual QA2 result and QA3 candidate

QA2 is now confirmed running: HTTP controller applied four button edges; SEQ selected GRID(page2) and SAMPLE restored LOOP(page0). Latest sample had8746paint callbacks. These remote events bypass the SDK physical dispatch mutex and do not certify physical SEQ behavior.

The first actual fb0 file (`build/qa-device/qa2-first.png`) is the SampleBoot startup splash, while the app paint count increases and the owner sees LOOP. **Do not call this a current LCD/app screenshot.** The app uses a separate DRM buffer; fbdev readback is unsuitable here. The older capture instructions and QA2 startup-unreachable notes above are historical, not present state. No screenshot of the current app has yet been obtained.

QA3 is built but not installed. It queries DRM resources/active CRTC/GETFB, obtains a readback mapping and converts only the device pixels. It closes its own GETFB handle and never takes display ownership. API and bounded worker captures use this same path. Failures stay explicit. The source includes a read-only surface-event counter outside the Mod mutex for physical-route diagnosis, forwarding every observed event unchanged. The pinned SDK stays unmodified.

QA3 also enlarges shared labels from9to11px and the top soft-button labels to11px, and gives each K1–K3control a matching value card with12px values. Native103tests, strict Clippy, normal ARM64 build, benchmark, launcher8and installer7checks pass. A separate QA3 ARM64 build also passes with highest GLIBC2.30and8launcher fixtures. Host previews are layout checks, not device screenshots. Physical SEQ escape, live typography/alignment, and mode discovery remain open until QA3 runs and can be inspected.

Reference for the display finding: [MPC Sample lab audio/display analysis](https://github.com/gospodindilly-ops/mpc-sample-lab/blob/main/docs/audio-display.en.md). DRM ABI is based on Linux UAPI drm.h/drm_mode.h; no closed-source SampQOL code was copied.

## QA3 actual capture confirmed / QA4 remote development agent

QA3 has now booted and its active DRM readback correctly captures the visible Loop Club screen.14actual page captures are under ignored `build/qa-device/qa3-pages`; these include PROJECT, synth NOTES/CHORD and6edit pages, GRID and drum bank browser.20queued remote SEQ presses kept GRID active. This does not reproduce or certify the physical SDK button path. Actual fonts remain smaller than desired; UI work is paused for the owner's higher-priority autonomous update request.

QA4 adds a separate `loop-club-qa-agent` executable on USB8091, launched by `systemd-run --unit=loopclub-qa-agent --collect` into its own transient service. It survives ac50 app crashes/restarts. The launcher verifies the agent SHA before copying it into `/run`; normal builds contain no agent. The agent binds only192.168.155.1, accepts only the USB /28, refuses browser Origin headers and requires a specific management header for writes. No arbitrary command, file path or shell input is accepted. This is a local opt-in development channel, not a public network service or cryptographic remote-administration boundary.

Operations: GET `/status.json`; POST `/stage` with a bounded8MiB ARM64 ELF .so and SHA256; POST `/restart` with exactly `{"discard_session":true}`. Stage verifies SHA256 by the device's existing sha256sum and publishes only a new `/run/loop-club.so` inode. The old mapped library remains intact until restart. The previous runtime is retained; if the new QA HTTP service with a new PID does not return in20seconds, restore the previous runtime and restart ac50 again. The agent rejects restart when a reachable Loop Club snapshot reports playback/recording. The caller must save owner work or explicitly authorize discarding the volatile QA session; it is never auto-saved over a project. Firmware, native settings, owner projects/samples and SD contents are not written by this path. A power cycle removes the agent and runtime updates; the unchanged SD candidate boots next time.

```sh
python3 tools/qa_manage.py
python3 tools/qa_manage.py --stage build/device/loop-club.so
python3 tools/qa_manage.py --restart --discard-session
```

QA4 compiled app and independent agent pass ARM64 build (maxGLIBC2.30), strictClippy,106native tests and8launcher fixtures. Agent tests verify wrong architecture/hash rejection, byte readback, symlink refusal and preservation of an already-open old library inode. Actual systemd isolation, upload/restart and rollback are **not yet physically verified**. QA3 cannot install QA4 through its existing API, so one initial SD install/boot is still necessary. This work removes the need for SD access during subsequent development updates; it does not claim native SD Card Access navigation or persistent SD installation has been remotely implemented. The SDK replay-button API requires a genuine captured current-page event and cannot safely be called with invented zero context.

## Current result supersedes earlier preparation notes

See [2026-11-22 autonomous QA](qa-autonomous-2026-11-22.md) for actual DRM/USB/device regression results,32voice budget, SDK build patch, persistent installation and remaining physical acceptance. Earlier fbdev images were stale launcher frames and are not current-screen proof.
