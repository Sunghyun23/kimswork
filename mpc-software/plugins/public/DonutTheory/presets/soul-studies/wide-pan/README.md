# Soul Studies — wider Auto Pan

User feedback: the original18%/24% Auto Pan was too subtle. Dusty Sunday now uses70% depth; Velvet Melt uses80%. Rates remain0.22/0.18Hz. The engine, harmony, oscillators, envelopes and other effects are unchanged. Sound Trim is recalibrated to match average listening level. The original files remain in the parent folder for comparison.

WAVs are actual Donut Theory1.8.0engine renders from restored saved preset states. No external stereo processing is applied. Native state/WAV checks pass; MPC user-preset browser import remains untested. XPL targets the known MPC XL internalNVMe ARM64installation.

Reproduce with the existing renderer and exporter:

```sh
/private/tmp/render-donut-soul-studies /private/tmp/donut-soul-studies-wide-pan --wide-pan
python3 mpc-software/plugins/public/DonutTheory/tools/export_soul_studies.py /private/tmp/donut-soul-studies-wide-pan --output mpc-software/plugins/public/DonutTheory/presets/soul-studies/wide-pan
```
