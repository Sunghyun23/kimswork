#!/usr/bin/env python3
"""Measured, loudness-matched before/after pairs; no EQ/limiter or new harmonics."""
from pathlib import Path
import argparse,json,wave
import numpy as np
p=argparse.ArgumentParser();p.add_argument('input',type=Path);p.add_argument('out',type=Path);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=True)
def read(path):
 with wave.open(str(path),'rb')as f:
  assert(f.getframerate(),f.getnchannels(),f.getsampwidth())==(48000,2,2)
  return np.frombuffer(f.readframes(f.getnframes()),'<i2').astype(np.float64).reshape(-1,2)/32768

def write(path,x):
 assert np.max(np.abs(x))<.99
 with wave.open(str(path),'wb')as f:f.setnchannels(2);f.setsampwidth(2);f.setframerate(48000);f.writeframes(np.round(x*32767).astype('<i2').tobytes())
report={'sample_rate':48000,'bpm':88,'effects':'Drive/Chorus/Delay/Room/Motion/Drift off; Noise/KeyOff0','comparison':'same fixed settings/MIDI; independently adjusted constant gain to equal RMS, shared peak headroom; no compression','models':{}}
for name in ['tine','wurli','upright','felt','clav','muted','thumb']:
 xs=[read(args.input/(side+'-'+name+'-dry.wav'))for side in ['before','after']];rms=[float(np.sqrt(np.mean(x*x)))for x in xs];gains=[10**(-24/20)/r for r in rms];headroom=min(1,.89/max(np.max(np.abs(x))*g for x,g in zip(xs,gains)));ys=[x*g*headroom for x,g in zip(xs,gains)]
 pair=np.concatenate([ys[0],np.zeros((24000,2)),ys[1]])
 write(args.out/(name+'-before-after.wav'),pair);write(args.out/(name+'-improved-dry.wav'),ys[1]);metrics=[]
 for tag,x,g in zip(['before','after'],xs,gains):
  segments=[]
  for i in range(9):
   a=int((.12+i*.46+.025)*48000);b=a+int(.25*48000);y=x[a:b].mean(axis=1);spectrum=np.abs(np.fft.rfft(y*np.hanning(len(y))))**2;freq=np.fft.rfftfreq(len(y),1/48000);segments.append({'midi_note':48+(i//3)*12,'velocity':38+(i%3)*40,'rms_dbfs':round(float(10*np.log10(np.mean(y*y)+1e-30)),3),'spectral_centroid_hz':round(float(np.sum(freq*spectrum)/(np.sum(spectrum)+1e-30)),1)})
  metrics.append({'engine':tag,'peak_dbfs':round(float(20*np.log10(np.max(np.abs(x))+1e-30)),3),'raw_rms_dbfs':round(float(20*np.log10(np.sqrt(np.mean(x*x))+1e-30)),3),'comparison_gain_db':round(float(20*np.log10(g*headroom)),3),'velocity_segments':segments})
 assert abs(np.sqrt(np.mean(ys[0]**2))-np.sqrt(np.mean(ys[1]**2)))<1e-10
 report['models'][name]=metrics
(args.out/'comparison.json').write_text(json.dumps(report,indent=2)+'\n');print('Rendered7matched before/after pairs and7improved dry examples,88BPM; metrics recorded')
