# Legacy integer-harmonic scan for non-Tine proxy fitting. Tine uses tine_diagnostics.py.
from pathlib import Path
import wave,json,argparse,numpy as np
parser=argparse.ArgumentParser();parser.add_argument('reference',type=Path);args=parser.parse_args();p=args.reference;rate=22050;n=4096;hop=662;freq=np.fft.rfftfreq(n,1/rate);notes=np.arange(33,89);hertz=440*2**((notes-69)/12);mask=[];fund=[]
for hz in hertz:
 m=np.zeros(len(freq),bool)
 for h in range(1,19):m|=abs(freq-h*hz)<max(8,hz*.04)
 mask.append(m);fund.append(abs(freq-hz)<max(8,hz*.04))
mask=np.array(mask,float);fund=np.array(fund,float);valid=(freq>40)&(freq<10000);mask*=valid;window=np.hanning(n);report={}
for name in ['rhodes-classic','rhodes-la','wurli-140b','upright','double-felt','clavinet']:
 path=p/(name+'.wav')
 if not path.exists():continue
 with wave.open(str(path),'rb')as f:
  assert f.getframerate()==44100 and f.getnchannels()==1;x=np.frombuffer(f.readframes(f.getnframes()),'<i2').astype(float)[::2]/32768
 power=[];positions=[]
 for off in range(0,len(x)-n,hop):power.append(abs(np.fft.rfft(x[off:off+n]*window))**2);positions.append(off/rate)
 sp=np.array(power);total=sp[:,valid].sum(axis=1)+1e-30;purity=np.einsum('fb,nb->fn',sp,mask)/total[:,None];fraction=np.einsum('fb,nb->fn',sp,fund)/total[:,None];purity[fraction<.04]=0;best=purity.argmax(axis=1);quality=purity[np.arange(len(best)),best];rms=np.sqrt(total);loud=rms>rms.max()*.03
 candidates=[]
 for i in np.argsort(-quality):
  i=int(i)
  if not loud[i] or quality[i]<.75:continue
  note=int(notes[best[i]])
  if any(abs(c['seconds']-positions[i])<.45 for c in candidates):continue
  candidates.append({'seconds':round(positions[i],3),'note':note,'harmonic_purity':round(float(quality[i]),4),'fundamental_energy_fraction':round(float(fraction[i,best[i]]),4)})
  if len(candidates)>=30:break
 # Report only aggregate features and event metadata; no demo waveform is copied to project.
 assert np.isfinite(purity).all() and np.isfinite(fraction).all();cent=(sp*freq).sum(axis=1)/(sp.sum(axis=1)+1e-30);energy=sp.sum(axis=1);music=energy>np.percentile(energy,40)
 report[name]={'duration':round(len(x)/rate,2),'candidate_windows':candidates,'median_centroid_hz':round(float(np.median(cent[music])),1),'high_band_fraction':round(float(np.sum(sp[music][:,freq>2000])/np.sum(sp[music])),4),'notes_with_purity_above_85pct':sorted(set(c['note']for c in candidates if c['harmonic_purity']>.85))}
 print(name,'candidate windows',len(candidates),'best purity',candidates[0]['harmonic_purity']if candidates else None,'notes',report[name]['notes_with_purity_above_85pct'],flush=True)
(p/'audio-scan.json').write_text(json.dumps(report,indent=2)+'\n')
