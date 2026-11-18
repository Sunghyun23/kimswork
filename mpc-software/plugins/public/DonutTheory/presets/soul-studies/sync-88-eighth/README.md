# Soul Studies — 88 BPM / eighth-note Auto Pan

Both original phrases are rendered at88BPM with the actual engine's Motion Sync enabled and Division1/8. Auto Pan depths remain70% (Dusty Sunday) and80% (Velvet Melt). Shape remains Sine. One modulation cycle is one eighth note:0.340909s, approximately2.93333Hz. No external stereo processing is used.

The host tempo/PPQ clock drives Auto Pan during the render. On MPC, set the project tempo to88BPM to reproduce it; a plugin preset saves Sync/Division but does not change host tempo. Other timbral settings and harmony are retained, with saved Sound Trim recalibrated to matched listening level. Both chunks round-trip all287parameters; all notes release, and audio/WAV/XML integrity checks pass. MPC preset-browser import remains untested.

```sh
/private/tmp/render-donut-soul-studies /private/tmp/donut-soul-studies-88-eighth --wide-pan --bpm 88 --pan-sync 1/8
python3 mpc-software/plugins/public/DonutTheory/tools/export_soul_studies.py /private/tmp/donut-soul-studies-88-eighth --output mpc-software/plugins/public/DonutTheory/presets/soul-studies/sync-88-eighth
```
