from pathlib import Path
import sys,json,time,subprocess,collections
import numpy as np,sounddevice as sd
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
import qa_device_matrix as matrix
from qa_device_matrix import QA,state,pair
original_request=matrix.request
def request(path,data=None):
 for attempt in range(5):
  try:return original_request(path,data)
  except (OSError, __import__('http.client',fromlist=['RemoteDisconnected']).RemoteDisconnected):
   if data is not None or attempt==4:raise
   time.sleep(.25)
matrix.request=request
import argparse
parser=argparse.ArgumentParser(description='Fresh isolated pad-note/scratch QA; requires explicitly authorized app restarts.')
parser.add_argument('--execute',action='store_true')
parser.add_argument('--session',required=True)
parser.add_argument('--restore-library',required=True,type=Path)
a=parser.parse_args()
if not a.execute:parser.error('--execute is required')
out=p/'build/qa-device/pad-notes-0.11.3';out.mkdir(exist_ok=True,parents=True)
import hashlib,shutil
boot=json.loads(request('/boot-storage.json'))
if hashlib.sha256(a.restore_library.read_bytes()).hexdigest()!=boot['runtime_sha256']:parser.error('Restore library must match current runtime')
shutil.copy2(a.restore_library,out/'previous.so')
q=QA(out);
original_capture=q.capture
def resilient_capture(name):
 for attempt in range(4):
  try:
   result=original_capture(name)
   from PIL import Image
   if Image.open(out/(name+'.png')).getpixel((1,60))[:3]!=(20,38,32):raise OSError('Native overlay rather than Loop Club screen')
   return result
  except OSError:
   if attempt==3:raise
   time.sleep(.4)
q.capture=resilient_capture
saved=json.loads((out/'saved-baseline.json').read_text());report={};audio=collections.deque(maxlen=160);stream=None
(out/'discard-approved-before.json').write_text(json.dumps(state(),indent=2))
def install(lib,name):
 with(out/(name+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(lib),'--restart','--discard-session'],check=True,stdout=f,stderr=subprocess.STDOUT)
def button(code,shift=False):
 for on in [True,False]:q.event(kind='transport_button',code=code,on=on,shift=shift)
 time.sleep(.2)
def active(s):return bool(s['ui_position']&(1<<56))
def notes(i):return state()['words'][22+12*i]
def edge(i,on):q.event(kind='pad',index=i,on=on,velocity=87 if on else 0)
def stop():q.button(81);q.wait(lambda s:s['words'][4]&1==0);time.sleep(.2)
def signal(t=.2):
 audio.clear();time.sleep(t);x=np.concatenate(list(audio));assert np.isfinite(x).all();return float(np.max(np.abs(x[-4096:])))
def browser():return json.loads(request('/browser.json'))
def choose(name):
 e=next(e for e in browser()['entries']if e['name']==name);delta=e['index']-state()['browser']['row']
 for _ in range(abs(delta)):q.event(kind='encoder',delta=1 if delta>0 else -1)
 q.wait(lambda s:s['browser']['row']==e['index']);q.button(13)
try:
 install(out/'qa.so','qa-stage');assert state()['project_storage']['session_id']==a.session
 assert all(state()['words'][20+i*12]==0 and state()['words'][27+i*12]==0 for i in range(16)), 'Use a fresh isolated session'
 initial_pid=state()['pid']
 (out/'full-journal-before.txt').write_bytes(request('/app-journal.txt'))
 def ensure_pid():
  if state()['pid']!=initial_pid:
   log=request('/app-journal.txt').decode(errors='replace');(out/'unexpected-restart-journal.txt').write_text(log);raise RuntimeError('MPC app restarted; journal preserved')
 original_event=q.event
 def guarded_event(**kw):
  ensure_pid();original_event(**kw);ensure_pid()
 q.event=guarded_event
 old_capture=q.capture
 def guarded_capture(name):
  ensure_pid();old_capture(name);ensure_pid()
 q.capture=guarded_capture
 q.setup();q.knob(0,1);q.knob(1,0);q.button(6);q.kind(0,2);q.button(40);q.pad(0,shift=True);q.pad(1,shift=True);q.wait(lambda s:s['words'][22]>0)
 assert state()['words'][9]==0 and state()['words'][4]&1==0;report['generation_without_record_or_auto_play']=True
 n=notes(0);button(9);q.wait(active);q.pad(0);q.wait(lambda s:s['words'][22]<n);q.capture('actual-drum-erase');q.button(55,True);q.wait(lambda s:s['words'][22]==n)
 button(9);q.wait(lambda s:not active(s));button(9,True);q.wait(active);q.pad(0);q.pad(2);q.wait(lambda s:s['words'][22]>n);after=notes(0);q.capture('actual-drum-copy');q.pad(0);q.pad(1);q.wait(lambda s:s['words'][7]!=0);assert notes(0)==after;q.button(14);q.wait(lambda s:not active(s));report['drum_erase_copy_undo_occupied_done']=True
 q.button(6);button(9);q.wait(lambda s:s['ui_position']&(1<<58));q.capture('actual-loop-edit-exit');button(9);q.wait(lambda s:s['ui_position']&(1<<58)==0);report['loop_edit_toggle_exit']=True
 q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1);q.wait(lambda s:s['words'][8]>=s['words'][10],timeout=6);assert state()['words'][26]==1
 button(9);q.wait(active);q.pad(0);q.capture('actual-drum-record-erase');button(9);q.wait(lambda s:not active(s));q.button(55,True);q.wait(lambda s:s['words'][9]==0);stop();report['drum_record_pad_does_not_generate_and_capture_erase_cancels']=True
 q.kind(1,0);assert state()['words'][39]==0, 'Fresh Audio track required';q.button(36);q.wait(lambda s:s['browser']['open'])
 for part in ['LoopClub','Projects','QA','deep-audio-audit-20261123-r1','Fixtures']:
  old=browser()['path'];choose(part);q.wait(lambda s:browser()['path']!=old,timeout=10)
 choose('QA Mono 220.wav');q.wait(lambda s:not s['browser']['open']and s['words'][39]>0,timeout=15);source=state()['words'][39];q.button(13);q.button(12);q.button(12);q.button(12);q.wait(lambda s:((s['words'][28]>>21)&3)==2);q.button(6)
 q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==2 and s['words'][8]>=s['words'][10],timeout=6);q.pad(0,.1);q.pad(1,.1);q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=6);stop();n=notes(1);assert n>=4
 q.button(75);button(9);q.wait(active);q.pad(0);q.wait(lambda s:s['words'][34]<n);assert state()['words'][39]==source;q.capture('actual-chop-erase');q.button(55,True);q.wait(lambda s:s['words'][34]==n);button(9);button(9,True);q.pad(0);q.pad(2);q.wait(lambda s:s['words'][34]>n);q.capture('actual-chop-copy');q.button(14);q.wait(lambda s:not active(s));assert state()['words'][39]==source;report['chop_note_edit_preserves_source_regions']=True
 q.button(6);q.button(14);q.wait(lambda s:s['words'][28]&(1<<18));n=notes(1);q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==2 and s['words'][8]>=s['words'][10],timeout=6);button(9);q.wait(active);q.pad(0);q.wait(lambda s:s['words'][34]<n);q.button(55,True);q.wait(lambda s:s['words'][9]==0 and s['words'][34]==n);q.button(14);stop();q.button(6);report['chop_capture_erase_cancel_restores_notes']=True
 q.setup();q.knob(0,0);q.button(6);q.button(13);q.button(12);q.wait(lambda s:((s['words'][28]>>21)&3)==3);q.knob(1,1);assert state()['words'][1191]&8==0;button(13,True);q.wait(lambda s:s['words'][1191]&8!=0)
 device=next(i for i,d in enumerate(sd.query_devices())if 'MPC Sample'in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 def callback(i,o,n,t,st):audio.append(i.copy());o.fill(0)
 stream=sd.Stream(device=(device,device),samplerate=44100,channels=2,dtype='float32',callback=callback);stream.start()
 q.knob(4,.9);edge(0,True);q.knob(4,.1);q.wait(lambda s:pair(s['words'][1190])[1]<0);reverse=signal(.14);assert reverse>.002,reverse
 q.knob(4,.9);q.wait(lambda s:s['words'][1191]&16!=0);time.sleep(.1);returned=signal(.15);assert returned<.0001,returned;q.capture('actual-down-cut-return')
 q.knob(4,.1);q.wait(lambda s:s['words'][1191]&16==0 and pair(s['words'][1190])[1]<0);assert signal(.14)>.002;edge(0,False);q.wait(lambda s:s['words'][1191]&2==0);forward=signal(.2);assert forward>.002;stop();report['down_cut_usb']={'reverse_peak':reverse,'return_peak':returned,'release_forward_peak':forward}
 q.capture('actual-down-cut-ready');button(13,True);q.wait(lambda s:s['words'][1191]&8==0);q.knob(4,.1);edge(0,True);q.knob(4,.9);assert signal(.14)>.002;edge(0,False);stop();report['bidirectional_option_preserved']=True
 # Record both stroke and muted return, then recall only the isolated QA project.
 button(13,True);q.wait(lambda s:s['words'][1191]&8!=0)
 if state()['words'][28]&(1<<18):q.button(14)
 q.setup();q.knob(0,.3);q.button(6);q.knob(4,.9);q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==2 and s['words'][8]>=s['words'][10],timeout=6)
 edge(0,True);q.knob(4,.1);time.sleep(.12);q.knob(4,.9);time.sleep(.12);edge(0,False);q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=6);stop();assert state()['words'][28]&(1<<23);assert notes(1)>=4
 q.button(13);q.capture('actual-down-cut-recorded');report['down_cut_gesture_recording']=True
 # Only the isolated QA project is saved; never the owner Session.
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=15);q.button(6);q.button(13);q.wait(lambda s:s['words'][1191]&8!=0);report['format10_down_cut_recall']=True
 (out/'verification.json').write_text(json.dumps(report,indent=2));print('PAD NOTES / EXIT / GENERATOR / DOWN CUT DEVICE PASS',json.dumps(report),flush=True)
except BaseException:
 try:
  (out/'failure-state.json').write_text(json.dumps({'state':state(),'browser':browser()},indent=2))
  (out/'failure-journal.txt').write_bytes(request('/app-journal.txt'))
 except Exception:pass
 raise
finally:
 if stream:stream.stop();stream.close()
 try:stop()
 except Exception:pass
 install(out/'previous.so','owner-restore');s=q.wait(lambda s:s['project_io']['result']==2,timeout=10)
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][16+i*12+j]==saved['words'][16+i*12+j],(i,j)
 assert s['words'][2]==saved['words'][2] and s['input_source']['selected']==saved['input_source']['selected']
 (out/'owner-restored.json').write_text(json.dumps(s,indent=2));print('SAVED OWNER16 RESTORED',flush=True)
