#!/usr/bin/env python3
from pathlib import Path
import sys,json,time,subprocess,shutil,io
import argparse
parser=argparse.ArgumentParser(description='Opt-in isolated real Sample XPM QA; saves owner project and restarts app')
parser.add_argument('--execute',action='store_true')
if not parser.parse_args().execute:parser.error('--execute is required')
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state,request
q=QA(p/'build/qa-device/deep-audio-xpm-0.8.0');details={}
def install(path,label):
 with (q.out/(label+'.log')).open('w')as f:
  subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(path),'--restart','--discard-session'],stdout=f,stderr=subprocess.STDOUT,check=True)
def browser():return json.loads(request('/browser.json'))
def choose(name):
 v=browser();entry=next((e for e in v['entries'] if e['name']==name),None);assert entry,(name,v)
 target=entry['index'];current=state()['browser']['row']
 for _ in range(abs(target-current)):q.event(kind='encoder',delta=1 if target>current else -1)
 q.wait(lambda s:s['browser']['row']==target)
def fixtures(name,source=False):
 if source:q.button(12);q.pad(4)
 else:q.button(36,True)
 q.wait(lambda s:s['browser']['open'] and s['browser']['total']>0)
 root=json.loads(request('/boot-storage.json'))['card'];parts=Path(state()['project_storage']['directory']).relative_to(root).parts+('Fixtures',)
 for part in parts:
  previous=browser()['path'];choose(part);q.button(13)
  deadline=time.monotonic()+12
  while browser()['path']==previous:
   assert time.monotonic()<deadline,(part,browser());time.sleep(.1)
 choose(name);q.capture('actual-select-'+name.replace(' ','-'));q.button(13)
def save_load():
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20);q.button(6)
def run():
 q.kind(0,2);q.button(6);fixtures('QA Banks.xpm',True);q.wait(lambda s:s['browser']['total']==3);assert [e['name'] for e in browser()['entries']]==['BANK A / 16 PADS','BANK B / 16 PADS','BANK H / 16 PADS'];q.capture('actual-xpm-bank-choice')
 choose('BANK B / 16 PADS');q.button(13);q.wait(lambda s:not s['browser']['open'] and s['words'][787]==2,timeout=20);q.capture('actual-bank-b-imported');details['file_bank_and_pad_position']=True
 # All four velocity zones must be playable; record actual output through USB full duplex.
 import numpy as np,sounddevice as sd
 from collections import deque
 blocks=deque(maxlen=200);device=next(i for i,d in enumerate(sd.query_devices())if'MPC Sample' in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 def callback(indata,outdata,frames,timing,status):outdata.fill(0);blocks.append(indata.copy())
 with sd.Stream(device=(device,device),samplerate=44100,channels=2,dtype='float32',callback=callback):
  time.sleep(.5);peaks=[]
  for vel in [16,48,80,112]:
   blocks.clear();q.event(kind='pad',index=1,on=True,velocity=vel);time.sleep(.4);q.event(kind='pad',index=1,on=False,velocity=0);time.sleep(.2);a=np.concatenate(list(blocks));assert np.isfinite(a).all();peaks.append(float(np.max(np.abs(a))))
  assert min(peaks)>.001 and peaks==sorted(peaks),peaks;details['four_zone_usb_peaks']=peaks
 q.button(13);q.capture('actual-xpm-layer-name');q.button(6);save_load();assert state()['words'][787]==2;details['project6_embedded_recall']=True
 fixtures('QA Missing.xpm');q.wait(lambda s:s['browser']['total']==1);q.button(13);q.wait(lambda s:bool(browser().get('error')) and 'Missing XPM' in browser()['error'],timeout=15);assert state()['words'][787]==2;q.capture('actual-missing-sample-keeps-kit');q.button(14);details['missing_sample_keeps_kit']=True
 fixtures('QA Banks.xpm');q.wait(lambda s:s['browser']['total']==3);q.button(12);q.wait(lambda s:browser()['path'].endswith('Fixtures'));choose('QA Banks.xpm');q.button(13);q.wait(lambda s:s['browser']['total']==3);choose('BANK H / 16 PADS');q.button(13);q.wait(lambda s:not s['browser']['open']and s['words'][787]==32768,timeout=20);q.pad(15);q.capture('actual-bank-h-pad16');details['bank_back_and_h_position']=True
 q.button(36);q.wait(lambda s:s['browser']['open']);assert not any(e['name'].endswith('.xpm')for e in browser()['entries']);q.capture('actual-native-bank-browser-preserved');q.button(14);details['native_lcbank_browser_preserved']=True
s=state();assert s['version'] in ['0.7.1','0.8.0'] and s['project_storage']['session_id']is None and s['words'][9]==0 and s['words'][4]&1==0
q.setup();seq=state()['project_io']['sequence'];q.pad(7);before=q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);(q.out/'owner-before.json').write_text(json.dumps(before,indent=2))
try:
 install(q.out/'qa.so','qa-stage');assert state()['project_storage']['session_id']=='deep-audio-xpm-20261123-r1'
 run();(q.out/'verification.json').write_text(json.dumps(details,indent=2));print('XPM DEVICE PASS',details,flush=True)
finally:
 if state()['browser']['open']:q.button(14)
 q.button(81);install(q.out/'normal.so','normal-owner-restore');s=state();assert s['project_storage']['session_id']is None
 for i in range(16):
  b=16+i*12
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][b+j]==before['words'][b+j],('owner',i,j)
 assert s['words'][2]==before['words'][2];assert s['input_source']['selected']==before['input_source']['selected'];(q.out/'owner-restored.json').write_text(json.dumps(s,indent=2))
 for ext in ['so','elf','map']:shutil.copy2(q.out/f'normal.{ext}',p/f'build/device/loop-club.{ext}')
 print('OWNER16 RESTORED',flush=True)
