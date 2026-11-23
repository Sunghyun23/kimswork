#!/usr/bin/env python3
"""Actual isolated0.11.1 Sample QA. Requires matching normal/QA builds in build/qa-device/deep-audit-0.11.1.
Uses queued controls/active DRM/stereo USB, not physical control certification.
Refuses a changed known owner state and restores normal runtime in finally.
"""
from pathlib import Path
import sys,json,time,subprocess,shutil,os
from collections import deque
import numpy as np,sounddevice as sd
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state,pair,request
q=QA(p/'build/qa-device/deep-audit-0.11.1');report={};audio=deque(maxlen=2048);stream=None
PPQ=1<<24
QA_SESSION=os.environ.get("LOOP_CLUB_QA_SESSION","deep-audio-audit-20261123-r1")
def browser():return json.loads(request('/browser.json'))
def choose(name):
 entries=browser()['entries'];entry=next(e for e in entries if e['name']==name);row=state()['browser']['row'];target=entry['index']
 for _ in range(abs(target-row)):q.event(kind='encoder',delta=1 if target>row else -1)
 q.wait(lambda s:s['browser']['row']==target);q.button(13)
def fixture(name):
 q.button(36);q.wait(lambda s:s['browser']['open']and s['browser']['total']>0)
 root=json.loads(request('/boot-storage.json'))['card'];parts=Path(state()['project_storage']['directory']).relative_to(root).parts+('Fixtures',)
 for part in parts:
  old=browser()['path'];choose(part);deadline=time.monotonic()+15
  while browser().get('path')==old:
   assert time.monotonic()<deadline;time.sleep(.1)
 choose(name);q.wait(lambda s:not s['browser']['open']and s['words'][27]>0,timeout=20)
def signal(seconds):
 audio.clear();time.sleep(seconds);x=np.concatenate(list(audio));assert np.isfinite(x).all();return x
def peak(x):return float(np.max(np.abs(x)))
def install(file,name):
 print(name,flush=True)
 with(q.out/(name+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(file),'--restart','--discard-session'],stdout=f,stderr=subprocess.STDOUT,check=True)
def edge(index,on):q.event(kind='pad',index=index,on=on,velocity=90 if on else 0)
def stop():q.button(81);q.wait(lambda s:s['words'][1055]==1,timeout=10)
def deck():q.button(6);q.button(13);q.wait(lambda s:s['page']==1)
def swing(amount,grid16=False):
 q.setup();q.knob(2,1 if grid16 else 0,shift=True);q.knob(2,(amount-50)/25)
 s=q.wait(lambda s:(s['words'][1188]&(1<<33))!=0 and abs(pair(s['words'][1188])[0]-amount)<=1 and bool(s['words'][1188]&(1<<32))==grid16);assert abs(pair(s['words'][1188])[0]-amount)<=1
 assert bool(s['words'][1188]&(1<<32))==grid16
before=state();assert before['project_storage']['session_id']is None and before['words'][9]==0 and before['words'][4]&1==0
baseline=p/'build/qa-device/turntable-swing-0.11.0/owner-restored.json'
if baseline.exists():
 known=json.loads(baseline.read_text())
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert before['words'][16+i*12+j]==known['words'][16+i*12+j], 'Owner state changed; preserve it before testing'
(q.out/'owner-before.json').write_text(json.dumps(before,indent=2))
try:
 install(q.out/'qa.so','stage-qa');s=state();assert s['version']=='0.11.1' and s['project_storage']['session_id']==QA_SESSION
 device=next(i for i,d in enumerate(sd.query_devices())if'MPC Sample'in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 def callback(indata,outdata,frames,timing,status):outdata.fill(0);audio.append(indata.copy())
 stream=sd.Stream(device=(device,device),samplerate=44100,channels=2,dtype='float32',callback=callback);stream.start()
 q.kind(0,0);fixture('QA Mono 220.wav');assert state()['words'][27]==11025
 deck()
 for _ in range(4):q.button(12)
 s=q.wait(lambda s:((s['words'][16]>>21)&3)==3);assert s['page']==1;q.capture('actual-turntable-ready')
 level=s['words'][17];q.knob(4,.5);q.knob(1,1);q.wait(lambda s:pair(s['words'][1189])[1]==2)
 edge(0,True);q.knob(4,1);forward=q.wait(lambda s:s['words'][1191]&2 and pair(s['words'][1190])[1]>.01)
 x=signal(.15);assert peak(x)>.002;report['forward_scratch_usb_peak']=peak(x);q.capture('actual-turntable-grab')
 q.knob(4,0);reverse=q.wait(lambda s:pair(s['words'][1190])[1]<-.01)
 x=signal(.15);assert peak(x)>.002;report['reverse_scratch_usb_peak']=peak(x)
 assert state()['words'][17]==level
 # A stationary fader settles to silence; release restores varispeed at Slip position.
 x=signal(.8);assert peak(x[-5000:])<.0001,peak(x[-5000:]);report['held_stationary_usb_peak']=peak(x[-5000:])
 edge(0,False);q.wait(lambda s:s['words'][1191]&2==0);x=signal(.4);assert peak(x[-6000:])>.002;report['slip_release_usb_peak']=peak(x[-6000:]);q.capture('actual-turntable-slip-play')
 # Holding owner is released after navigating to MIX, without changing gain.
 edge(0,True);q.button(1);q.wait(lambda s:s['page']==3);edge(0,False);q.wait(lambda s:s['words'][1191]&2==0);assert state()['words'][17]==level;report['fader_volume_isolation_and_release_after_navigation']='PASS';stop()
 # Whole MIDI Swing values, 8th/16th UI and live generated drums.
 swing(70,False);q.capture('actual-project-swing-70');q.kind(1,2);q.button(40);q.pad(2,shift=True);q.wait(lambda s:s['words'][16+12+10]!=0 and s['words'][4]&1)
 events_before=state()['words'][352:480];x=signal(1.4);assert peak(x)>.002
 swing(75,True);q.capture('actual-project-swing-16th');assert state()['words'][352:480]==events_before;report['live_swing_all_midi_controls_and_original_events']={'swing_percent':pair(state()['words'][1188])[0],'sixteenth':bool(state()['words'][1188]&(1<<32)),'peak':peak(x)};stop()
 # New scratch replacement: original PCM remains, control playback bypasses MIDI Swing.
 q.select(0);q.setup();q.pad(4);q.button(6);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1);q.wait(lambda s:s['words'][8]>=s['words'][10],timeout=5)
 edge(0,True);q.knob(4,.9);time.sleep(.2);q.knob(4,.1);time.sleep(.2);edge(0,False);q.capture('actual-turntable-recording');q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=7);stop()
 s=state();count=s['words'][22];assert count>=4 and s['words'][16]&(1<<20) and s['words'][27]==11025,(count,s['words'][16]);assert s['words'][16]&(1<<23)
 q.button(55,True);q.wait(lambda s:s['words'][16]&(1<<20)==0);q.button(54,True);q.wait(lambda s:s['words'][16]&(1<<20)!=0 and s['words'][22]==count)
 q.button(82);x=signal(2.8);assert peak(x)>.002;report['recording_undo_redo_and_replay']={'control_events':count,'source_frames':s['words'][27],'usb_peak':peak(x)};stop()
 # No-op overdub preserves gestures and their count.
 deck();q.button(14);q.wait(lambda s:s['words'][16]&(1<<18));q.button(6);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1);q.wait(lambda s:s['words'][8]>=s['words'][10],timeout=5);time.sleep(.3);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=7);stop();assert state()['words'][22]==count
 report['noop_overdub_preserves_gestures']='PASS'
 # Punch a new gesture into the existing recording, without replacing PCM.
 q.button(6);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1);q.wait(lambda s:s['words'][8]>=s['words'][10],timeout=5);time.sleep(.4);edge(0,True);q.knob(4,.8);time.sleep(.15);q.knob(4,.2);time.sleep(.12);edge(0,False);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=7);stop();s=state();count=s['words'][22];assert count>=4 and s['words'][27]==11025
 q.button(82);x=signal(2.7);assert peak(x)>.002;stop();report['punch_overdub_and_replay']={'control_events':count,'source_frames':s['words'][27],'usb_peak':peak(x)}
 # Save/recall only in isolated storage. Owner project never saved.
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=30);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=30);q.select(0);assert state()['words'][22]==count and state()['words'][27]==11025 and abs(pair(state()['words'][1188])[0]-75)<1
 deck();q.capture('actual-turntable-recalled');report['format9_swing_and_deck_recall']='PASS'
 # Cross-mode guards and source/grid identity, verified against actual USB silence.
 print('deep cross-mode checks',flush=True)
 deck();q.button(12);q.button(12);q.wait(lambda s:(s['words'][16]>>21)&3==1);q.button(6);saved_count=state()['words'][22];q.pad(0,shift=True)
 q.wait(lambda s:s['words'][7]!=0 and s['words'][9]==0);assert state()['words'][22]==saved_count;report['deck_to_note_overdub_refused']='PASS'
 deck();q.button(14);q.wait(lambda s:s['words'][16]&(1<<18)==0);q.setup();q.pad(5);q.button(6);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1 and s['words'][8]<s['words'][10]);q.button(7);q.capture('actual-replace-note-grid-countin')
 q.wait(lambda s:s['words'][8]>=s['words'][10],timeout=7);x=signal(.25);assert peak(x)<.00001 and state()['words'][1191]&4==0,peak(x)
 q.button(6);edge(0,True);x=signal(.2);assert peak(x)>.002;edge(0,False);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=7);stop();assert state()['words'][16]&(1<<23)==0;report['replace_suppresses_old_deck_and_keeps_live_notes']={'silent_peak':0.0,'note_peak':peak(x)}
 q.button(7);q.capture('actual-replaced-note-grid');q.button(55,True);q.wait(lambda s:s['words'][16]&(1<<23));q.button(82);x=signal(.3);assert peak(x)>.002;stop();q.button(54,True);q.wait(lambda s:s['words'][16]&(1<<23)==0);report['cross_mode_undo_redo']='PASS'
 q.button(55,True);q.wait(lambda s:s['words'][16]&(1<<23));deck()
 for _ in range(3):q.button(12)
 q.wait(lambda s:(s['words'][16]>>21)&3==3);q.button(14);q.wait(lambda s:s['words'][16]&(1<<18));q.button(6);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1);q.wait(lambda s:s['words'][8]>=s['words'][10],timeout=7);x=signal(.2);assert peak(x)>.002
 q.button(55,True);q.wait(lambda s:s['words'][9]==0 and s['words'][1191]&4==0);x=signal(.3);assert peak(x)<.00001;report['cancelled_automated_take_returns_to_silence']=peak(x);stop()
 # PAD1 release must target its original track, even after DATA selects another.
 deck();q.knob(4,.5);edge(0,True);q.select(2);edge(0,False);q.select(0);q.wait(lambda s:s['words'][1191]&2==0);report['grab_release_after_track_change']='PASS';stop()
 # Real malformed-file refusal and stereo resampling through the browser.
 print('deep WAV browser checks',flush=True)
 q.select(2);q.button(36);q.wait(lambda s:s['browser']['open'] and s['browser']['total']>0)
 root=json.loads(request('/boot-storage.json'))['card'];parts=Path(state()['project_storage']['directory']).relative_to(root).parts+('Fixtures',)
 for part in parts:
  old=browser()['path'];choose(part);deadline=time.monotonic()+15
  while browser().get('path')==old:
   assert time.monotonic()<deadline;time.sleep(.1)
 choose('QA Broken.wav');q.wait(lambda s:not browser().get('pending',True) and bool(browser().get('error','')));assert state()['words'][16+2*12+11]==0;q.capture('actual-broken-wav-refusal');report['malformed_wav_refused_atomically']=browser().get('error')
 choose('QA Stereo 330.wav');q.wait(lambda s:not s['browser']['open']and s['words'][16+2*12+11]>0,timeout=20);assert state()['words'][16+2*12+11]==11024
 deck()
 for _ in range(4):q.button(12)
 q.wait(lambda s:(s['words'][16+2*12]>>21)&3==3);edge(0,True);edge(0,False);x=signal(.5);assert peak(x)>.002
 frequencies=[]
 for channel in range(2):
  a=x[-16000:,channel];spec=np.abs(np.fft.rfft(a*np.hanning(len(a))));frequencies.append(float(np.fft.rfftfreq(len(a),1/44100)[int(np.argmax(spec))]))
 assert abs(frequencies[0]-330)<10 and abs(frequencies[1]-440)<10,frequencies
 report['stereo_wav_resampling_and_channels']={'frames':11024,'frequencies':frequencies,'peak':peak(x)};q.capture('actual-stereo-turntable');stop()
 (q.out/'device-verification.json').write_text(json.dumps(report,indent=2));print('DEVICE PASS',json.dumps(report),flush=True)
finally:
 if stream is not None:stream.stop();stream.close()
 try:q.button(81)
 except Exception:pass
 install(q.out/'normal.so','restore-owner');s=state();assert s['project_storage']['session_id']is None
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][16+i*12+j]==before['words'][16+i*12+j],(i,j)
 assert s['words'][2]==before['words'][2];q.button(6);(q.out/'owner-restored.json').write_text(json.dumps(state(),indent=2))
 for ext in ['so','elf','map']:shutil.copy2(q.out/f'normal.{ext}',p/f'build/device/loop-club.{ext}')
 print('OWNER RESTORED',flush=True)
