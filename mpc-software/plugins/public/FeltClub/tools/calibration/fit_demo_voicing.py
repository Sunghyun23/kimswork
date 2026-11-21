#!/usr/bin/env python3
"""Conservative spectrum proxy fit to public performance excerpts, not a clone.
No demo audio is copied into the project or used at runtime. Unknown velocities,
MIDI timing and FX prohibit claims of matched attacks, decay or equivalence.
"""
from pathlib import Path
import argparse,ctypes,json,wave,time
from concurrent.futures import ThreadPoolExecutor
import numpy as np
p=argparse.ArgumentParser();p.add_argument('--reference',type=Path,required=True);p.add_argument('--bridge',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--model',type=int,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
if a.model==0:p.error('Harmonic-only Tine fitting is disabled. Use tine_diagnostics.py to include inharmonic attack/decay; performance excerpts are not isolated physical measurements.')
sources={0:'rhodes-classic',1:'wurli-140b',2:'upright',3:'double-felt',4:'clavinet'};name=sources[a.model];scan=json.loads((a.reference/'audio-scan.json').read_text())[name]
with wave.open(str(a.reference/(name+'.wav')),'rb')as f:
 assert(f.getframerate(),f.getnchannels(),f.getsampwidth())==(44100,1,2);audio=np.frombuffer(f.readframes(f.getnframes()),'<i2').astype(float)/32768
rate=44100;nfft=8192;window=np.hanning(nfft);freq=np.fft.rfftfreq(nfft,1/rate);frames=int(rate*1.2);ages=[.03,.15,.4,.8];base=np.array([1.,1.,0.,0.],np.float32)
lib=ctypes.CDLL(str(a.bridge.resolve()));ptr=ctypes.POINTER(ctypes.c_float);lib.fc_demo.argtypes=[ptr,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_int,ctypes.c_float,ptr]
items=[];used=set()
for c in scan['candidate_windows']:
 note=c['note']
 if note in used or c['harmonic_purity']<.90:continue
 hz=440*2**((note-69)/12);masks=np.array([abs(freq-h*hz)<max(8,hz*.04) for h in range(1,17)])
 pos=int(c['seconds']*rate);x=audio[pos:pos+nfft]
 if len(x)!=nfft:continue
 # Verify the fundamental and align tuning before comparing higher harmonics.
 fine=abs(np.fft.rfft(x*window,65536));ff=np.fft.rfftfreq(65536,1/rate);sel=np.where((ff>hz/1.07)&(ff<hz*1.07))[0];j=sel[np.argmax(fine[sel])];v=np.log(fine[j-1:j+2]+1e-20);den=v[0]-2*v[1]+v[2]
 if abs(den)<1e-15:continue
 delta=.5*(v[0]-v[2])/den;actual_hz=(j+delta)*rate/65536;pitch=69+12*np.log2(actual_hz/440);cents=(pitch-note)*100
 if abs(cents)>20:continue
 hz=actual_hz;tuning=440*2**(cents/1200);masks=np.array([abs(freq-h*hz)<max(8,hz*.04)for h in range(1,17)])
 sp=abs(np.fft.rfft(x*window))**2;union=masks.any(axis=0);band=(freq>40)&(freq<10000);purity=float(sp[union&band].sum()/(sp[band].sum()+1e-30))
 if purity<.90:continue
 def feature(y,masks=masks):
  sp=abs(np.fft.rfft(y*window))**2;bands=np.einsum('b,hb->h',sp,masks);return np.maximum(-45,10*np.log10(bands/(bands.sum()+1e-30)+1e-30))
 items.append({'note':note,'seconds':c['seconds'],'quality':purity,'cents':float(cents),'tuning':float(tuning),'target':feature(x),'masks':masks,'feature':feature});used.add(note)
 if len(items)==12:break
items.sort(key=lambda z:z['note'])
assert len(items)>=5,(name,'too few dominant-note excerpts',len(items))
for i,z in enumerate(items):z['train']=i%3!=1
# Different notes are held out; clip velocities and note ages remain latent.
velocities=[38,78,118];lo=np.array([np.log2(.65),np.log2(.5),-.35,-.35]);hi=np.array([np.log2(1.55),np.log2(2.0),.55,.35]);active=np.array([0,1,2,3]if a.model==0 else[0,1,2]if a.model==1 else[1,2])
def decode(y):
 v=lo+y*(hi-lo);v[:2]=2**v[:2]
 if a.model>=2:v[0]=1
 if a.model!=0:v[3]=0
 return np.array(v,np.float32)
def render(v,note,vel,tuning=440):
 out=np.empty(frames,np.float32);lib.fc_demo(v.ctypes.data_as(ptr),a.model,note,vel,frames,tuning,out.ctypes.data_as(ptr));return out

def assess(v,train=True):
 losses=[]
 for z in items:
  if z['train']!=train:continue
  variants=[]
  for vel in velocities:
   y=render(v,z['note'],vel,z['tuning'])
   if not np.isfinite(y).all()or np.max(abs(y))>32:return 1e6
   variants.extend(z['feature'](y[int(t*rate):int(t*rate)+nfft])for t in ages)
  actual=np.array(variants);target=z['target'];weight=.15+.85*np.clip((target+42)/32,0,1);error=np.sum(weight*(actual-target)**2,axis=1)/weight.sum();losses.append(float(error.min()))
 return float(np.mean(losses))
start=time.monotonic();before_train=assess(base);before_hold=assess(base,False);rng=np.random.default_rng(20261121+a.model);best=base.copy();score=before_train
with ThreadPoolExecutor(max_workers=4)as pool:
 candidates=[decode(x)for x in rng.random((160,4))];values=list(pool.map(assess,candidates));j=int(np.argmin(values))
 if values[j]<score:best=candidates[j];score=values[j]
 for step in[.12,.06,.03,.015]:
  center=np.array([np.log2(best[0]),np.log2(best[1]),best[2],best[3]]);norm=(center-lo)/(hi-lo);candidates=[]
  for i in active:
   for sign in[-1,1]:x=norm.copy();x[i]+=sign*step;candidates.append(decode(np.clip(x,0,1)))
  values=list(pool.map(assess,candidates));j=int(np.argmin(values))
  if values[j]<score:best=candidates[j];score=values[j]
after_hold=assess(best,False);accepted=after_hold<before_hold*.90 and score<before_train*.90
# Preserve middle-C mean energy across representative velocities with one gain.
def mean_energy(v):return np.mean([np.mean(render(v,60,vel)**2)for vel in velocities])
gain=float(np.sqrt(mean_energy(base)/(mean_energy(best)+1e-30))) if accepted else 1.
result={'source':name,'model':a.model,'baseline':[1,1,0,0],'candidate':best.tolist(),'accepted':bool(accepted),'level':gain,'train_before':before_train,'train_after':score,'heldout_before':before_hold,'heldout_after':after_hold,'training_notes':[z['note']for z in items if z['train']],'heldout_notes':[z['note']for z in items if not z['train']],'windows':[{k:z[k]for k in['note','seconds','quality','cents','train']}for z in items],'seconds':time.monotonic()-start,'limits':'Dominant-note spectrum proxy only. MIDI/velocity/age/FX unknown; not isolated dry samples, attack/decay calibration, listening certification or Keyscape equivalence.'}
(a.out/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result),flush=True)
