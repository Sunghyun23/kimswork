# Soul Studies — Donut Theory 1.8.0

Two original sound-design studies inspired by warm, unhurried neo-soul/hip-hop keyboard textures. No artist recording, sample library or third-party effect is used. These are separate audition/user-preset files; the existing64factory programs and installed plugin are unchanged.

- **Keys / Dusty Sunday:** sine with gentle2:1phase modulation and a triangle layer; rounded key attack, soft low-pass, low-mid Body, light tape drive and subtle independent pitch/stereo movement. One-shot8ms strum.
- **Synth / Velvet Melt:** saw/triangle blend, mild detuning, slower attack/release, Body and tape density, slow filter/mix movement and Auto Pan. Chord Glide65ms with6ms strum and overlapping chord triggers.

Audio is the actual1.8.0native engine after restoring the provided state chunk. The original82BPM phrase is Cmaj9 → Am9/E → F6/9/D → G9sus4/D. The top note moves D4 → C4 → C4 → C4. Velocity and small note-on offsets vary; no drums or backing audio mask the patch. Average levels are matched using the saved Sound Trim parameter, with no external mastering, added reverb or post-render normalization. WAV is48kHz/stereo/16-bit PCM.

The JSON recipes contain all287plain parameter values by stable ID. The `.xpl` files embed the same verified native state and target `/nvme/VST2 Plugins/kim - VST - Donut Theory/VST/donut-theory-arm64.so` (MPC XL internalNVMe). XML/base64, state round-trip and WAV integrity were verified. **Import through the actual MPC user-preset browser has not been tested.** The files will not automatically appear as new built-in programs; a different installation path/architecture needs a matching preset descriptor. They also contain the demonstration chord bank and default individual pad mapping.

Reproduce from the repository root with the reviewed1.8.0engine:

```sh
clang++ -std=c++17 -O2 -I mpc-software/third_party/mpc-plugin-builder-sdk/framework mpc-software/plugins/public/DonutTheory/tools/render_soul_studies.cpp -o /private/tmp/render-donut-soul-studies
/private/tmp/render-donut-soul-studies /private/tmp/donut-soul-studies
python3 mpc-software/plugins/public/DonutTheory/tools/export_soul_studies.py /private/tmp/donut-soul-studies
```

The renderer verifies all287parameter values after loading each saved chunk, checks finite/unclipped output, and checks complete note release. `verification.json` records levels and hashes. The renderer intentionally rejects another engine version until the studies are reviewed again. Original code/patches/audio follow the repository's MIT license.

User-requested stronger movement: [Wide Pan revision](wide-pan/README.md) raises Auto Pan depths to70%/80%, retaining the slow rates. The original18%/24% versions above remain available.
