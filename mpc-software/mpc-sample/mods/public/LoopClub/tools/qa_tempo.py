from pathlib import Path
import sys,json,time,subprocess,collections,wave
import numpy as np,sounddevice as sd
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'));from qa_device_matrix import QA,state,request
import argparse
parser=argparse.ArgumentParser(description='Isolated actual tempo/count/USB QA. Requires explicitly authorized discard/restarts.')
parser.add_argument('--execute',action='store_true')
if not parser.parse_args().execute:parser.error('--execute is required')
out=p/'build/qa-device/tempo-0.11.4';q=QA(out);saved=json.loads((p/'build/qa-device/pad-notes-0.11.3/public-owner-restored.json').read_text());audio=collections.deque(maxlen=4000);stream=None;report={}
(out/'discard-approved-before.json').write_text(json.dumps(state(),indent=2))
def install(lib,name):
 with(out/(name+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(lib),'--restart','--discard-session'],check=True,stdout=f,stderr=subprocess.STDOUT)
def bpm(s=None):return __import__('struct').unpack('<d',(s or state())['words'][2].to_bytes(8,'little'))[0]
def button(shift=False):
 for on in [True,False]:q.event(kind='transport_button',code=53,on=on,shift=shift)
def taps(period):
 time.sleep(2);start=time.monotonic()
 for i in range(6):
  delay=start+i*period-time.monotonic()
  if delay>0:time.sleep(delay)
  button()
 q.wait(lambda s:abs(bpm(s)-60/period)<=2)
def samples():
 assert audio,'No actual USB input';x=np.concatenate(list(audio));assert np.isfinite(x).all();return x

def count(x):
 level=np.max(np.abs(x),axis=1);hits=np.flatnonzero(level>max(.0001,float(np.max(level))*.12));
 if not len(hits):return []
 return np.r_[hits[0],hits[1:][np.diff(hits)>4410]].astype(int).tolist()
def capture_sound(seconds):audio.clear();time.sleep(seconds);return samples()
def stop():q.button(81);q.wait(lambda s:s['words'][4]&1==0);time.sleep(.1)
try:
 install(out/'qa.so','qa-stage');assert state()['project_storage']['session_id']=='tempo-count-20261124-r1';pid=state()['pid'];assert all(state()['words'][20+12*i]==0 for i in range(16))
 q.kind(0,1);taps(.5);report['tap_120_bpm']=bpm();taps(.6);report['tap_100_bpm']=bpm();q.capture('actual-tap-tempo')
 b=bpm();button(True);q.wait(lambda s:s['words'][4]&(1<<16));assert bpm()==b;button(True);q.wait(lambda s:s['words'][4]&(1<<16)==0);assert bpm()==b;report['shift_tap_only_toggles_metronome']=True
 q.setup();q.knob(1,.5);q.wait(lambda s:(s['words'][4]>>8)&255==1);q.button(6)
 device=next(i for i,d in enumerate(sd.query_devices())if'MPC Sample'in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 def callback(i,o,n,t,status):audio.append(i.copy());o.fill(0)
 stream=sd.Stream(device=(device,device),samplerate=44100,channels=2,dtype='float32',callback=callback);stream.start();time.sleep(.2)
 # Start from a deliberately stopped offbeat to reject partial/extra count clicks.
 q.button(82);time.sleep(.2);stop();audio.clear();q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1 and s['words'][8]>=s['words'][10],timeout=7);time.sleep(.12);x=samples();onsets=count(x);assert len(onsets)==4,onsets;assert all(abs((v-u)/44100-60/bpm())<.035 for u,v in zip(onsets,onsets[1:]));report['one_bar_preroll']={'click_count':len(onsets),'onsets_seconds':[i/44100 for i in onsets],'peak':float(np.max(np.abs(x)))}
 with wave.open(str(out/'actual-four-count.wav'),'wb')as w:w.setnchannels(2);w.setsampwidth(2);w.setframerate(44100);w.writeframes((np.clip(x,-1,1)*32767).astype('<i2').tobytes())
 q.capture('actual-four-count-recording');button(True);q.wait(lambda s:s['words'][4]&(1<<16));q.capture('actual-metronome-on');x=capture_sound(1.8);assert 2<=len(count(x))<=4;report['metronome_on_during_recording_clicks']=len(count(x));button(True);q.wait(lambda s:s['words'][4]&(1<<16)==0);time.sleep(.1);x=capture_sound(.8);assert float(np.max(np.abs(x)))<.0001;assert state()['words'][9]==1;report['metronome_off_recording_peak']=float(np.max(np.abs(x)));q.button(73);q.wait(lambda s:s['words'][9]==0,timeout=7);stop()
 # The USB host sends digital zero; monitoring clicks must not become source PCM.
 assert state()['input_source']['selected']==2
 q.kind(1,0);button(True);q.wait(lambda s:s['words'][4]&(1<<16));q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==2 and s['words'][8]>=s['words'][10],timeout=7);time.sleep(.2);q.pad(1,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=7);stop();button(True);q.wait(lambda s:s['words'][4]&(1<<16)==0);assert state()['words'][39]>0;q.button(13);q.button(12);q.wait(lambda s:(s['words'][28]>>21)&3==1);q.event(kind='pad',index=0,on=True,velocity=90);x=capture_sound(.4);q.event(kind='pad',index=0,on=False,velocity=0);assert float(np.max(np.abs(x)))<.0001;report['click_not_printed_to_recorded_usb_pcm_peak']=float(np.max(np.abs(x)))
 button(True);q.wait(lambda s:s['words'][4]&(1<<16));q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=15);q.button(6);q.wait(lambda s:s['words'][4]&(1<<16));q.capture('actual-metronome-recalled');assert state()['pid']==pid;report['saved_metronome_option_recalls']=True;report['same_pid_entire_test']=pid
 (out/'verification.json').write_text(json.dumps(report,indent=2));print('TAP / METRONOME / FOUR-COUNT DEVICE PASS',json.dumps(report),flush=True)
finally:
 if stream:stream.stop();stream.close()
 try:stop()
 except Exception:pass
 install(out/'previous.so','owner-restore');s=q.wait(lambda s:s['project_io']['result']==2,timeout=10)
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][16+12*i+j]==saved['words'][16+12*i+j],(i,j)
 assert s['words'][2]==saved['words'][2]and s['input_source']['selected']==saved['input_source']['selected'];(out/'owner-restored.json').write_text(json.dumps(s,indent=2));print('SAVED OWNER16 RESTORED',flush=True)
