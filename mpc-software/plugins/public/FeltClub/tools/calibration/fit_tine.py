#!/usr/bin/env python3
"""Local research: fit12global physical coefficients, never copy sample data.
Reference audio stays outside the project and is NOT a runtime resource.
Numpy only; actual C++ physical voice is rendered through a narrow test bridge.
"""
from pathlib import Path
import argparse,ctypes,json,time,wave,hashlib
from concurrent.futures import ThreadPoolExecutor
import numpy as np
p=argparse.ArgumentParser();p.add_argument('--reference',type=Path,required=True);p.add_argument('--bridge',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--generations',type=int,default=24);p.add_argument('--seed',type=int,default=20261120);p.add_argument('--warm-start',type=Path);p.add_argument('--wide',action='store_true');p.add_argument('--extra',type=Path);p.add_argument('--verify-profile',type=Path);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
rate=44100;frames=rate*4;starts=[0,.18,.5,1.2,2.7];nfft=8192;window=np.hanning(nfft);freq=np.fft.rfftfreq(nfft,1/rate);ratios=list(range(1,11))+[6.267,17.55]
lib=ctypes.CDLL(str(a.bridge.resolve()));ptr=ctypes.POINTER(ctypes.c_float);lib.fc_tine.argtypes=[ptr,ctypes.c_int,ctypes.c_int,ctypes.c_int,ptr]
names=['gapScale','gapKey','offset','velocityPower','stiffnessScale','upperScale','decayScale','decayKey','lossLinear','couplingScale','squash','speedFloor']
base=np.array([1,0,.52,.85,1,1,1,.15,.12,1,1,.35],float);log_indices=[0,4,5,6,8,9]
lo=np.array([-5,-.4,.30,.6,-2,-2,-1.5,-.1,-6,-2,0,.03]);hi=np.array([1,.4,1.3,2.5,2.5,3,1.5,.4,-1,2,1,.35]);span=hi-lo
if a.wide:lo[1]=-.65;lo[4]=-3;hi[6]=2;hi[11]=.65;span=hi-lo

def encode(x):
 y=np.array(x,float)
 for i in log_indices:y[i]=np.log2(y[i])
 return (y-lo)/span

def decode(y):
 x=lo+y*span
 for i in log_indices:x[i]=2**x[i]
 return np.asarray(x,dtype=np.float32)

items=[]
for file in sorted(list(a.reference.glob('*.wav'))+(list(a.extra.glob('*.wav'))if a.extra else[])):
 note=int(file.stem.split('_')[1]);layer=int(file.stem[-1]);velocity={5:36,4:60,3:84,2:106,1:120}[layer]
 with wave.open(str(file),'rb')as f:
  assert f.getframerate()==rate and f.getnchannels()==1 and f.getsampwidth()==2
  x=np.frombuffer(f.readframes(f.getnframes()),'<i2').astype(np.float32)/32768
 onset=int(np.argmax(abs(x)>np.max(abs(x))*.015));x=x[onset:onset+frames];x=np.pad(x,(0,max(0,frames-len(x))))
 hz=440*2**((note-69)/12);masks=[abs(freq-r*hz)<max(6,hz*.075)for r in ratios]
 items.append({'note':note,'velocity':velocity,'file':file.name,'x':x,'masks':masks,'train':note in[40,55,65]and layer in[1,4,5]})
assert len(items)>=18 and sum(z['train']for z in items)==9

def features(x,masks):
 powers=[];envelope=[]
 for start in starts:
  offset=int(start*rate);y=x[offset:offset+nfft];s=abs(np.fft.rfft(y*window))**2
  bands=np.array([float(np.sum(s[m]))for m in masks]);powers.append(bands);envelope.append(float(np.mean(y*y)))
 powers=np.array(powers);norm=np.sum(powers[:,:10],axis=1,keepdims=True)+1e-30
 spectral=np.maximum(-55,10*np.log10(powers/norm+1e-30))
 env=np.maximum(-45,10*np.log10((np.array(envelope)+1e-30)/(max(envelope[:2])+1e-30)))
 early=[];ref=float(np.mean(x[int(.02*rate):int(.20*rate)]**2))+1e-30
 for l,r in [(0,.002),(.002,.005),(.005,.012),(.012,.03),(.03,.06)]:early.append(np.clip(10*np.log10((float(np.mean(x[int(l*rate):int(r*rate)]**2))+1e-30)/ref),-45,18))
 return spectral,env,np.array(early)
for z in items:z['target']=features(z['x'],z['masks'])

def render(profile,note,velocity):
 out=np.empty(frames,np.float32);x=np.ascontiguousarray(profile,np.float32);lib.fc_tine(x.ctypes.data_as(ptr),note,velocity,frames,out.ctypes.data_as(ptr));return out

def assess(profile,training=True):
 errors=[]
 for z in items:
  if z['train']!=training:continue
  y=render(profile,z['note'],z['velocity'])
  if not np.all(np.isfinite(y))or np.max(abs(y))>150:return 1e6
  actual=features(y,z['masks']);target=z['target'];audibility=.20+.80*np.clip((target[0]+48)/35,0,1)
  spectral=float(np.sum(audibility*(actual[0]-target[0])**2)/np.sum(audibility));env=float(np.mean((actual[1]-target[1])**2));attack=float(np.mean((actual[2]-target[2])**2))
  errors.append(.68*spectral+.22*env+.10*attack)
 return float(np.mean(errors))

if a.verify_profile:
 result=json.loads(a.verify_profile.read_text());before=assess(base,False);after=assess(np.array(result['candidate_profile']),False);check={'validation_samples':sum(not z['train']for z in items),'baseline':before,'candidate':after,'reference':'processed real Rhodes, not Lounge Lizard'};(a.out/'verification.json').write_text(json.dumps(check,indent=2)+'\n');print(check);raise SystemExit(0)
begin=time.monotonic();baseline=assess(base);print('baseline training loss',round(baseline,4),flush=True)
rng=np.random.default_rng(a.seed);population=rng.random((48,12));population[0]=encode(base)
if a.warm_start:
 warm=np.array(json.loads(a.warm_start.read_text())["candidate_profile"]);population[1]=encode(warm)
 for i in range(9,25):population[i]=np.clip(encode(warm)+rng.normal(0,.10,12),0,1)
# Include physically motivated removal of the synthetic displacement squash.
for i in range(1,9):population[i]=np.clip(encode(base)+rng.normal(0,.12,12),0,1);population[i,10]=0;population[i,0]=rng.uniform(.35,.72);population[i,3]=rng.uniform(.3,.8)
history=[]
with ThreadPoolExecutor(max_workers=4)as pool:
 scores=np.array(list(pool.map(lambda z:assess(decode(z)),population)))
 for generation in range(a.generations):
  trials=[]
  for i in range(len(population)):
   indices=rng.choice([j for j in range(len(population))if j!=i],3,replace=False);x,y,z=population[indices];mutant=np.clip(x+.65*(y-z),0,1);cross=rng.random(12)<.8;cross[rng.integers(12)]=True;trials.append(np.where(cross,mutant,population[i]))
  new=np.array(list(pool.map(lambda z:assess(decode(z)),trials)));accept=new<scores;population[accept]=np.array(trials)[accept];scores[accept]=new[accept];best=int(np.argmin(scores));record={'phase':'differential','iteration':generation+1,'evaluations':48*(generation+2),'loss':float(scores[best]),'seconds':round(time.monotonic()-begin,2),'profile':decode(population[best]).tolist()};history.append(record);print(json.dumps(record),flush=True);(a.out/'progress.json').write_text(json.dumps(history,indent=2)+'\n')
 best=population[np.argmin(scores)].copy();bestscore=float(min(scores))
 for scale in [.12,.06,.03,.015,.0075,.003]:
  for repeat in range(2):
   candidates=[np.clip(best+np.eye(12)[i]*sign*scale,0,1)for i in range(12)for sign in[-1,1]];values=list(pool.map(lambda z:assess(decode(z)),candidates));winner=int(np.argmin(values))
   if values[winner]<bestscore:best=candidates[winner];bestscore=values[winner]
   else:break
  record={'phase':'coordinate','scale':scale,'loss':bestscore,'seconds':round(time.monotonic()-begin,2),'profile':decode(best).tolist()};history.append(record);print(json.dumps(record),flush=True)
profile=decode(best);holdout_base=assess(base,False);holdout_after=assess(profile,False)
result={'target':'EQ-processed real1977RhodesMarkI, not LoungeLizard','source_manifest':json.loads((a.reference/'manifest.json').read_text()),'profile_names':names,'baseline_profile':base.tolist(),'candidate_profile':profile.tolist(),'training_baseline':baseline,'training_candidate':bestscore,'heldout_baseline':holdout_base,'heldout_candidate':holdout_after,'seconds':time.monotonic()-begin,'history':history,'limits':'Loss is spectral/envelope fit, not perceived realism. Reference samples are private analysis only and must not be bundled.'}
(a.out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
for z in items:
 if z['train']:continue
 for tag,pv in [('baseline',base),('candidate',profile)]:np.save(a.out/(f"{tag}-{z['note']}-{z['velocity']}.npy"),render(pv,z['note'],z['velocity']))
print('FINAL',json.dumps({k:result[k]for k in['training_baseline','training_candidate','heldout_baseline','heldout_candidate','seconds']}),flush=True)
