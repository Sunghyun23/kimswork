#!/usr/bin/env python3
"""Calculate factory trims from audit_presets --untrimmed stereo-peak measurements.

Render at the audit tool's fixed -8dB master, velocity96 and category registers.
This is peak matching for that pattern, not a perceived-loudness guarantee.
"""
import argparse,csv,json,math
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('report',type=Path);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
rows=list(csv.DictReader(args.report.open()));assert len(rows)==64 and rows[0]['name']=='Init / Init'
trims={}
for row in rows:
 name=row['name'];peak=float(row['peak']);assert math.isfinite(peak) and 0<peak<1,name
 if name=='Init / Init':trims[name]=0;continue
 target_db=-15 if name.startswith(('Bass /','Lead /')) else -12
 # The engine applies Sound Trim immediately before its final tanh limiter.
 target=10**(target_db/20)
 trim=20*math.log10(math.atanh(target)/math.atanh(peak))
 assert -18<=trim<=18,(name,trim)
 trims[name]=round(trim,2)
args.out.write_text(json.dumps({'reference':'44100Hz, master -8dB, velocity96, MIDI36 bass/MIDI72 lead/MIDI60 minor7 poly, 2s hold/3s release; stereo-peak trim through final tanh, not equal-loudness certification','trim_db':trims},indent=2)+'\n')
print(f'Calibrated {len(trims)} sounds; Init trim unchanged; range {min(trims.values()):.2f}..{max(trims.values()):.2f}dB')
