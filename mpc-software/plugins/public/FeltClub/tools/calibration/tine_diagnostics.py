#!/usr/bin/env python3
"""Tine spectrum/decay diagnostics: retain non-integer attack components.
Reference performance residuals can include other notes/FX: never call them
isolated tine measurements or evidence of perceptual equivalence.
"""
from pathlib import Path
import argparse,json,wave
import numpy as np

def read(path):
 with wave.open(str(path),'rb')as f:
  assert f.getsampwidth()==2;sr=f.getframerate();channels=f.getnchannels();x=np.frombuffer(f.readframes(f.getnframes()),'<i2').astype(float).reshape(-1,channels).mean(axis=1)/32768
 return x,sr

def masks(f0,sr,n):
 hz=np.fft.rfftfreq(n,1/sr);width=max(sr/n*1.8,f0*.035);harm=np.zeros(len(hz),bool)
 for h in range(1,int(min(32,sr*.45/f0))+1):harm|=abs(hz-h*f0)<width
 # Wide mode neighborhoods allow real tuning-fork geometry to differ from an
 # ideal beam. Excluding harmonic cores prevents ordinary H6 being called bell.
 bands={'harmonic':harm}
 for name,lo,hi in [('bell_low',4.8,8.0),('bell_high',14.5,20.5)]:bands[name]=(hz>lo*f0)&(hz<hi*f0)&~harm
 for ratio in [6.267,17.55,34.39]:bands['mode_'+str(ratio)]=(abs(hz-ratio*f0)<width)&~harm
 bands['evaluated']=(hz>30)&(hz<min(18000,sr*.45));return hz,bands

def spectrum(x,sr,f0,n=8192):
 if len(x)<n:x=np.pad(x,(0,n-len(x)))
 power=abs(np.fft.rfft(x[:n]*np.hanning(n)))**2;hz,bands=masks(f0,sr,n);total=power[bands['evaluated']].sum()+1e-30
 return {k:float(10*np.log10((power[v].sum()+1e-30)/total))for k,v in bands.items()if k!='evaluated'}

def regression():
 sr=48000;n=8192;t=np.arange(n)/sr;f0=220
 a=np.sin(2*np.pi*f0*t);b=a+.3*np.sin(2*np.pi*f0*6.267*t)+.15*np.sin(2*np.pi*f0*17.55*t)
 c=a+.6*np.sin(2*np.pi*f0*6.267*t)+.3*np.sin(2*np.pi*f0*17.55*t)
 old=spectrum(a,sr,f0);mid=spectrum(b,sr,f0);high=spectrum(c,sr,f0)
 assert mid['mode_6.267']>old['mode_6.267']+40 and mid['mode_17.55']>old['mode_17.55']+40
 assert 4<high['bell_low']-mid['bell_low']<7 and 4<high['bell_high']-mid['bell_high']<7
 return {'test':'adding and doubling inharmonic components changes both evaluated bell bands','pass':True}

def compare(before,after,out,reference=None):
 result={'scope':'Tine0.2.4versus current native render; identical settings/MIDI, not Keyscape render','reference_limits':'Performance residuals can contain chord notes/FX; unknown MIDI/velocity/age, no perceptual equivalence claim','regression':regression(),'models':{}}
 for tag,path in [('before',before),('after',after)]:
  x,sr=read(path);events=[]
  for i in range(9):
   note=48+i//3*12;f0=440*2**((note-69)/12);onset=.12+i*.46;windows=[]
   for age in [.005,.07,.16]:
    start=int((onset+age)*sr);windows.append({'age_seconds':age,'bands_db':spectrum(x[start:start+8192],sr,f0)})
   early=x[int((onset+.005)*sr):int((onset+.06)*sr)];late=x[int((onset+.20)*sr):int((onset+.32)*sr)];events.append({'note':note,'velocity':38+i%3*40,'windows':windows,'late_over_early_rms_db':float(10*np.log10((np.mean(late*late)+1e-30)/(np.mean(early*early)+1e-30)))})
  result['models'][tag]=events
 if reference:
  refs={}
  for name in ['rhodes-classic','rhodes-la']:
   x,sr=read(reference/(name+'.wav'));n=8192;hop=int(sr*.06);hz=np.fft.rfftfreq(n,1/sr);candidates=[]
   for start in range(0,len(x)-n,hop):
    y=x[start:start+n];sp=abs(np.fft.rfft(y*np.hanning(n)))**2
    if np.mean(y*y)<1e-5:continue
    sel=(hz>55)&(hz<1200);j=np.where(sel)[0][np.argmax(sp[sel])];v=np.log(sp[j-1:j+2]+1e-30);den=v[0]-2*v[1]+v[2]
    if abs(den)<1e-15:continue
    f=(j+.5*(v[0]-v[2])/den)*sr/n;pitch=69+12*np.log2(f/440);note=round(pitch)
    if abs(pitch-note)>.20:continue
    values=spectrum(y,sr,f);residual=10**(values['bell_low']/10)+10**(values['bell_high']/10)
    # Keep visibly bell-rich residuals instead of demanding90%integer harmonics.
    if residual>.025 and values['harmonic']>-5:candidates.append({'seconds':round(start/sr,3),'dominant_note_candidate':note,'bands_db':values,'classification':'dominant-note performance residual, may contain other notes or FX'})
   refs[name]={'candidate_count':len(candidates),'examples':sorted(candidates,key=lambda z:z['bands_db']['bell_low'],reverse=True)[:12]}
  result['reference_diagnostics']=refs
 out.parent.mkdir(parents=True,exist_ok=True);out.write_text(json.dumps(result,indent=2)+'\n');return result

if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--before',type=Path);p.add_argument('--after',type=Path);p.add_argument('--out',type=Path);p.add_argument('--reference',type=Path);p.add_argument('--self-test',action='store_true');a=p.parse_args()
 if a.self_test:print(regression())
 else:
  assert a.before and a.after and a.out;d=compare(a.before,a.after,a.out,a.reference);print('Tine diagnostics recorded; synthetic inharmonic sensitivity passed')
