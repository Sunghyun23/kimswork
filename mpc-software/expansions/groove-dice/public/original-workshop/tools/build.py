#!/usr/bin/env python3
"""Reproducible original synthesis bank. Reads no sampled audio, source MIDI or model output.
Builds a separate candidate folder; never overwrites the installed/private source bank.
Requires numpy; all waveforms are generated here from oscillators and seeded white noise.
"""
from pathlib import Path
import argparse,hashlib,json,math,struct,wave,zlib,io
import numpy as np
P=Path(__file__).resolve().parents[1]
RATE=44100
ROLES=['Kick','Snare','Closed Hat','Open Hat','Clap','Tom','Cymbal','Perc']
FAMILIES=['Velvet Dry','Round Circuit','Wood Room','Dust Cut']
PEAK=[.62,.48,.20,.24,.30,.38,.27,.22]

def shaped_noise(rng,n,low,high):
 x=rng.normal(0,1,n);f=np.fft.rfftfreq(n,1/RATE)
 hp=(f/np.maximum(f,low))**3;lp=1/np.sqrt(1+(f/high)**8)
 y=np.fft.irfft(np.fft.rfft(x)*hp*lp,n);return y/max(np.std(y),1e-9)

def synth(role,family,variant):
 rng=np.random.default_rng(0x4b494d00+role*997+family*71+variant)
 duration=[.65,.55,.22,1.0,.5,.8,2.0,.55][role]
 t=np.arange(round(duration*RATE))/RATE;n=len(t);q=variant/3
 def sine(freq):return np.sin(2*np.pi*freq*t)
 def exp(seconds):return np.exp(-t/seconds)
 if role==0:
  root=[48,43,58,50][family]*(.94+.12*q);drop=[110,90,65,140][family]
  tau=.012+.009*q;phase=2*np.pi*(root*t+drop*tau*(1-exp(tau)))
  body=np.sin(phase)*exp(.10+.035*q)
  x=body+.13*np.sin(phase*2.03)*exp(.036)+(.05+.025*family)*shaped_noise(rng,n,1800,8500)*exp(.004)
 elif role==1:
  root=[184,166,214,196][family]+variant*6
  body=(sine(root)+.45*sine(root*1.77))*exp(.033+.008*q)
  wires=shaped_noise(rng,n,[950,1300,1700,1100][family],[9000,7300,10500,8000][family])
  x=.42*body+(.24+.025*family)*wires*exp(.056+.024*q)
  x+=.09*shaped_noise(rng,n,3000,14500)*exp(.006)
 elif role in (2,3):
  metallic=np.zeros(n)
  for freq in [3211,4237,5179,6197,7537,8819]:metallic+=np.tanh(3*sine(freq*(1+.01*variant+.018*family)))
  f=np.fft.rfftfreq(n,1/RATE);metallic=np.fft.irfft(np.fft.rfft(metallic)*(f/np.maximum(f,3800))**3,n)
  metallic/=max(np.std(metallic),1e-9)
  noise=shaped_noise(rng,n,4500-family*350,15000-family*1000)
  decay=(.020+.009*q+.004*family)if role==2 else(.15+.05*q+.015*family)
  x=(.32*metallic+.68*noise)*exp(decay)
 elif role==4:
  noise=shaped_noise(rng,n,800+family*170,8500-family*400);envelope=np.zeros(n)
  for j,delay in enumerate([0,.013+q*.003,.028+q*.004]):
   u=np.maximum(t-delay,0);envelope+=(t>=delay)*(.8**j)*(1-np.exp(-u/.0008))*np.exp(-u/.008)
  x=noise*(envelope+.20*exp(.055+.015*q))+.045*sine(1220+family*90)*exp(.025)
 elif role==5:
  root=[102,87,142,116][family]*(.85+.3*q);tau=.022
  phase=2*np.pi*(root*t+root*.48*tau*(1-exp(tau)))
  x=(np.sin(phase)+.26*np.sin(phase*1.59)+.11*np.sin(phase*2.24))*exp(.10+.05*q)
  x+=.035*shaped_noise(rng,n,1300,6500)*exp(.009)
 elif role==6:
  metal=np.zeros(n)
  for j,freq in enumerate([537,823,1297,1931,2861,4073,6311,9173,12143]):
   metal+=sine(freq*(1+.021*variant+.013*family)+.3*j)*exp(.20+.034*j+.05*q)/(1+.15*j)
  noise=shaped_noise(rng,n,2600,14500)
  x=.18*metal+.28*noise*exp(.25+.11*q)
 elif role==7:
  if family==0: # Wooden rim: damped, inharmonic resonances.
   x=(sine(810+variant*45)+.55*sine(1467+variant*61)+.22*sine(2511))*exp(.016+.004*q)
   x+=.13*shaped_noise(rng,n,2200,10000)*exp(.003)
  elif family==1: # Shaker, naturally uneven noise envelope.
   envelope=exp(.035+.015*q)*(1+.2*sine(43+variant*3))
   x=shaped_noise(rng,n,3200,13000)*envelope
  elif family==2: # Conga/bongo-like modal percussion.
   root=190+variant*19;x=(sine(root)+.55*sine(root*1.48)+.24*sine(root*2.11))*exp(.055+.02*q)
   x+=.10*shaped_noise(rng,n,1200,7000)*exp(.008)
  else: # Bell: two detuned square-like modes with a warm short tail.
   x=(np.tanh(4*sine(533+variant*13))+.7*np.tanh(4*sine(811+variant*17)))*exp(.045+.009*q)
 # Original deterministic coloration; no imported texture or recorded surface noise.
 if family==1:x=np.tanh(x*1.35)
 elif family==2:x=np.tanh(x*.95)+.035*np.sin(2*np.pi*137*t)*exp(.02)
 elif family==3:
  x=np.tanh(x*1.9);x=np.repeat(x[::2],2)[:n];x=np.round(x*2048)/2048
 x*=np.minimum(1,t/.0004);x-=np.mean(x)
 x*=np.minimum(1,t/.0003)*np.minimum(1,(duration-t)/.015)
 # Remove subsonic DC without borrowing an impulse response.
 f=np.fft.rfftfreq(n,1/RATE);x=np.fft.irfft(np.fft.rfft(x)*(f/np.maximum(f,22))**2,n)
 x*=np.minimum(1,t/.0004)*np.minimum(1,(duration-t)/.015)
 peak=np.max(np.abs(x));assert peak>1e-4 and np.isfinite(x).all()
 x*=PEAK[role]/peak
 live=np.flatnonzero(np.abs(x)>PEAK[role]*10**(-65/20));end=min(n,int(live[-1])+round(.015*RATE))
 x=x[:end];x[-min(len(x),441):]*=np.linspace(1,0,min(len(x),441))
 return x

def wav_bytes(x):
 ints=np.round(np.clip(x,-1,1)*8388607).astype(np.int32);u=ints.astype(np.uint32)
 b=np.column_stack([u&255,(u>>8)&255,(u>>16)&255]).astype(np.uint8).tobytes();out=io.BytesIO()
 with wave.open(out,'wb')as w:w.setnchannels(1);w.setsampwidth(3);w.setframerate(RATE);w.writeframes(b)
 return out.getvalue()
def vlq(n):
 a=[n&127];n>>=7
 while n:a.insert(0,128|(n&127));n>>=7
 return bytes(a)
def midi(notes,length=15360):
 events=[]
 for tick,role,vel in notes:events.extend([(tick,bytes([0x99,60+role,vel])),(tick+60,bytes([0x89,60+role,0]))])
 events.sort(key=lambda e:(e[0],e[1][0]));track=bytearray();prev=0
 for tick,data in events:track+=vlq(tick-prev)+data;prev=tick
 track+=vlq(max(0,length-prev))+b'\xff\x2f\0'
 return b'MThd'+struct.pack('>IHHH',6,0,1,960)+b'MTrk'+struct.pack('>I',len(track))+track

def beat(which):
 notes=[];motifs=[[0,6,10],[0,3,8,14]]
 for bar in range(4):
  off=bar*3840
  for step in motifs[which]:notes.append((off+step*240,0,105 if step==0 else 89))
  for step in [4,12]:notes.append((off+step*240,1,100))
  for step in range(0,16,2):notes.append((off+step*240,2,76 if step%4==0 else 52))
  if bar%2:notes.append((off+3360,7,48))
 return sorted(notes)
def fnv(data):
 n=2166136261
 for x in data:n=((n^x)*16777619)&0xffffffff
 return n

def build(out):
 out.mkdir(parents=True,exist_ok=True);files=[];samples=[]
 for role in range(8):
  for v in range(16):
   family=v//4;x=synth(role,family,v%4);data=wav_bytes(x);name=f'WAV/Kits/O{family+1:02d}/{ROLES[role]}/{ROLES[role].replace(" ","")}_{v+1:02d}.wav'
   e=dict(path=name,kind=0,role=role,rate=RATE,channels=1,frames=len(x),sha=hashlib.sha256(data).hexdigest(),peak=float(np.max(np.abs(x))),rms=float(np.sqrt(np.mean(x*x))),family=FAMILIES[family],variant=v)
   target=out/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data);files.append(e);samples.append(data)
 assert len({x['sha']for x in files})==128
 for i in range(95):
  if i<2:name=f'MIDI/Beats/{["Velvet Window","Pocket Lantern"][i]}.mid';data=midi(beat(i));kind=3
  else:
   j=i-2;rng=np.random.default_rng(0x4b494d+j);swing=[0,12,24,36,48,60][j%6];spread=2+j//6
   events=[(max(0,k*240+(swing if k%2 else 0)+int(rng.integers(-spread,spread+1))),2,76 if k%4==0 else 48)for k in range(16)]
   name=f'MIDI/Grooves/Workshop Feel {j+1:02d}.mid';data=midi(events,3840);kind=4
  target=out/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data);files.append(dict(path=name,kind=kind,role=0,rate=0,channels=0,frames=0,sha=hashlib.sha256(data).hexdigest()))
 # Compatible64-sound prefix, eight variants per role, authored beat instead of extracted MIDI.
 directory=[];payload=bytearray()
 for r in range(8):
  for v in range(8):
   index=r*16+v;e=files[index];data=samples[index]
   directory.append(struct.pack('<64s8I',e['sha'].encode(),r,RATE,1,e['frames'],len(payload),len(data),fnv(data),0));payload+=data
 notes=beat(0);payload+=struct.pack('<8s4I',b'GDPATT01',1,1,15360,len(notes))+b''.join(struct.pack('<4I',t,r,v,80)for t,r,v in notes)
 blob=bytearray(struct.pack('<8s6I',b'GDBANK01',1,64,32,96,32+64*96,len(payload))+b''.join(directory)+payload)
 libstart=len(blob);blob+=struct.pack('<8s6I',b'GDLIB002',2,len(files),384,0,0,0)+b'\0'*(384*len(files));records=[]
 for e in files:
  data=(out/e['path']).read_bytes();offset=len(blob);blob+=data
  records.append(struct.pack('<64s256sQ10I16x',e['sha'].encode(),e['path'].encode(),offset,len(data),zlib.crc32(data),e['kind'],e['role'],e['rate'],e['channels'],e['frames'],0,0,0))
 blob[libstart+32:libstart+32+384*len(files)]=b''.join(records)
 def part(role,variant):return [0,0,[650,400,180,1100,350,650,1800,500][role],[-2,0,10,14,-8,-12,8,18][role],1 if role in(2,3)else 0]
 pools=[[],[],[],[]]
 for g,size in enumerate([440,546,376,244]):
  role=[0,1,2,7][g]
  for j in range(size):
   variant=j%16;sample=files[role*16+variant];second=files[3*16+variant]if g==2 else None
   settings=part(role,variant);settings[0]=-1.5*(j//16%3);settings[1]=((j//48)%5-2)*.35;settings[2]*=.75+.10*(j//16%6)
   pools[g].append(dict(samples=[sample['sha'],second['sha']if second else''],parts=[settings,part(3,variant)if second else[0,0,500,0,0]],name=f'{FAMILIES[variant//4]} {ROLES[role]} {j+1:03d}'))
 poolstart=len(blob);blob+=struct.pack('<8s2I',b'GDSND004',1606,200)
 for g,pool in enumerate(pools):
  for c in pool:
   record=struct.pack('<I',g)+b''.join(s.encode().ljust(64,b'\0')for s in c['samples'])+b''.join(struct.pack('<5f',*p)for p in c['parts'])+b'\0'*24;assert len(record)==196;blob+=record+struct.pack('<I',zlib.crc32(record))
 blob+=struct.pack('<8sQ',b'GDSEND04',poolstart);kitstart=len(blob);blob+=struct.pack('<8s2I',b'GDKITS03',96,800);kits=[];seen=set()
 styles=['Velvet','Pocket','Amber','Loose','Honey','Dust']
 for style in range(6):
  rng=np.random.default_rng(0x4b490000+style)
  for k in range(16):
   families=[[0,0,0,0,0,0,0,0],[1,1,1,1,0,1,1,1],[1,2,2,2,0,2,2,2],[3,0,3,3,3,2,3,0],[1,0,0,0,0,1,0,2],[3,3,3,3,3,3,3,3]][style]
   while True:
    variants=[families[r]*4+int(rng.integers(0,4))for r in range(8)]
    if tuple(variants)not in seen:seen.add(tuple(variants));break
   ids=[files[r*16+v]['sha']for r,v in enumerate(variants)];parts=[part(r,v)for r,v in enumerate(variants)]
   # Deliberate kit character: dry/round/woody/broken/warm/dusty, not arbitrary FX extremes.
   fx=[[5,2,4,8,24,.25,0,.7],[8,0,2,12,22,.25,0,.7],[6,3,9,5,28,.25,0,.7],[12,9,4,10,25,.25,0,.7],[9,3,6,8,24,.25,0,.7],[16,14,3,12,30,.25,0,.7]][style]
   name=f'V{style+1:02d} / {styles[style]}{k+1:02d}';kit=dict(name=name,samples=ids,parts=parts,fx=fx,variants=variants,source='Original deterministic oscillator/noise synthesis; no sampled source input')
   kits.append(kit);record=struct.pack('<64s',name.encode())+b''.join(s.encode()for s in ids)+b''.join(struct.pack('<5f',*p)for p in parts)+struct.pack('<8f',*fx)+b'\0'*28;assert len(record)==796;blob+=record+struct.pack('<I',zlib.crc32(record))
 blob+=struct.pack('<8sQ',b'GDKEND03',kitstart)+struct.pack('<8sQ',b'GDLEND02',libstart)
 bank=out/'Content/Banks/default.gdbank';bank.parent.mkdir(parents=True,exist_ok=True);bank.write_bytes(blob)
 manifest=dict(name='Groove Dice Original Workshop',version=1,rate=RATE,bits=24,wav=128,kits=96,sound_choices=[440,546,376,244],midi=95,beats=2,groove_templates=93,prefix_notes=len(notes),bytes=len(blob),bank_sha256=hashlib.sha256(blob).hexdigest(),generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),source='Only equations and deterministic PRNG in build_original_bank.py; no external audio/MIDI/model inputs',files=files,factory_kits=kits)
 (out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');(out/'factory-kits.json').write_text(json.dumps(dict(version=3,kits=kits),indent=2)+'\n');(out/'role-sounds.json').write_text(json.dumps(dict(version=4,pools=pools),indent=2)+'\n')
 (out/'README.txt').write_text('Groove Dice Original Workshop\n128 original synthesized one-shots / 96 kit configurations / 2 original beats / 93 original timing templates.\nAll audio is synthesized by tools/build_original_bank.py from equations and seeded noise, without external audio, MIDI or AI model input.\nThe standalone candidate bank uses the existing Groove Dice loader. Existing restricted default bank and installed plugin are not overwritten.\nDo not restore restricted-source project sample IDs into this bank expecting the old sounds.\n')
 print(json.dumps({k:manifest[k]for k in ['name','wav','kits','midi','prefix_notes','bytes','bank_sha256']},indent=2))
if __name__=='__main__':
 parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,default=P/'work');args=parser.parse_args();build(args.out.resolve())
