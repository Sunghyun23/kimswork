#!/usr/bin/env python3
"""Opt-in real MPC Sample0.7.0 USB QA.
Requires authorized owner save/app restart, current idle owner project, USB input,
matching isolated qa.so + normal.so in build/qa-device/audio-performance-0.7.0,
and macOS numpy/sounddevice/Pillow. No SD boot publication or private pack writes.
Restores normal runtime/owner tracks in finally; fails on any changed owner slot.
"""
import argparse
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--execute',action='store_true',help='Run authorized owner-save, isolated runtime and USB tests')
if not parser.parse_args().execute:parser.error('Explicit --execute is required; this saves owner work and restarts the app')
import sys,json,time,subprocess,shutil
from pathlib import Path
from collections import deque
import numpy as np,sounddevice as sd
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state,pair
"""Bounded CoreMIDI output to the explicitly named MPC Sample endpoint only."""
import ctypes as c
class SampleMidi:
 def __init__(self):
  self.m=c.CDLL('/System/Library/Frameworks/CoreMIDI.framework/CoreMIDI');self.f=c.CDLL('/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation')
  m,f=self.m,self.f
  f.CFStringCreateWithCString.argtypes=[c.c_void_p,c.c_char_p,c.c_uint32];f.CFStringCreateWithCString.restype=c.c_void_p
  f.CFStringGetCString.argtypes=[c.c_void_p,c.c_void_p,c.c_long,c.c_uint32];f.CFRelease.argtypes=[c.c_void_p]
  m.MIDIClientCreate.argtypes=[c.c_void_p,c.c_void_p,c.c_void_p,c.POINTER(c.c_uint32)];m.MIDIClientDispose.argtypes=[c.c_uint32]
  m.MIDIOutputPortCreate.argtypes=[c.c_uint32,c.c_void_p,c.POINTER(c.c_uint32)];m.MIDIPortDispose.argtypes=[c.c_uint32]
  m.MIDIGetNumberOfDestinations.restype=c.c_ulong;m.MIDIGetDestination.argtypes=[c.c_ulong];m.MIDIGetDestination.restype=c.c_uint32
  m.MIDIObjectGetStringProperty.argtypes=[c.c_uint32,c.c_void_p,c.POINTER(c.c_void_p)]
  self.client=c.c_uint32();self.port=c.c_uint32();name=f.CFStringCreateWithCString(None,b'Loop Club isolated QA',0x08000100)
  assert m.MIDIClientCreate(name,None,None,c.byref(self.client))==0
  assert m.MIDIOutputPortCreate(self.client,name,c.byref(self.port))==0;f.CFRelease(name)
  key=f.CFStringCreateWithCString(None,b'name',0x08000100);self.endpoint=None
  for i in range(m.MIDIGetNumberOfDestinations()):
   endpoint=m.MIDIGetDestination(i);text=c.c_void_p()
   if m.MIDIObjectGetStringProperty(endpoint,key,c.byref(text))==0:
    buf=c.create_string_buffer(512);f.CFStringGetCString(text,buf,512,0x08000100);f.CFRelease(text)
    if buf.value==b'MPC Sample':self.endpoint=endpoint;break
  f.CFRelease(key);assert self.endpoint is not None,'MPC Sample MIDI endpoint missing'
  m.MIDIPacketListInit.argtypes=[c.c_void_p];m.MIDIPacketListInit.restype=c.c_void_p
  m.MIDIPacketListAdd.argtypes=[c.c_void_p,c.c_ulong,c.c_void_p,c.c_uint64,c.c_ulong,c.c_void_p];m.MIDIPacketListAdd.restype=c.c_void_p
  m.MIDISend.argtypes=[c.c_uint32,c.c_uint32,c.c_void_p];self.held=set()
 def note(self,note,on,velocity=84):
  assert 0<=note<=127 and 0<=velocity<=127
  raw=(c.c_ubyte*3)(0x90 if on else 0x80,note,velocity if on else 0)
  packet=c.create_string_buffer(1024);current=self.m.MIDIPacketListInit(packet)
  assert self.m.MIDIPacketListAdd(packet,1024,current,0,3,raw)
  assert self.m.MIDISend(self.port,self.endpoint,packet)==0
  if on:self.held.add(note)
  else:self.held.discard(note)
 def close(self):
  for note in list(self.held):self.note(note,False)
  self.m.MIDIPortDispose(self.port);self.m.MIDIClientDispose(self.client)

q=QA(p/'build/qa-device/audio-performance-0.7.0');details={};streams=[];midi=None;recorded=deque(maxlen=4096)
def install(path,name):
 with (q.out/(name+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(path),'--restart','--discard-session'],check=True,stdout=f,stderr=subprocess.STDOUT)
def idle():
 q.button(81);q.wait(lambda s:s['words'][1055]==1,timeout=12)
def target(i):
 q.button(6);q.button(12);q.wait(lambda s:s['page']==9);q.pad(4+i);q.wait(lambda s:(s['words'][16]>>21)&3==i);q.capture('actual-source-'+['input','chromatic','chop'][i]);q.button(6)
def edge(pad,on):q.event(kind='pad',index=pad,on=on,velocity=85 if on else 0,shift=False,erase=False)
def capture_audio(seconds):
 recorded.clear();time.sleep(seconds);return np.concatenate(list(recorded),axis=0)
def metrics(x):
 peak=float(np.max(np.abs(x))) if x.size else 0.;mono=x[:,0];mono=mono[int(len(mono)*.1):];crossings=np.count_nonzero((mono[:-1]<=0)&(mono[1:]>0));return {'peak':peak,'crossing_hz':float(crossings*44100/max(1,len(mono))),'finite':bool(np.isfinite(x).all())}
def finish():q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==0,timeout=6)
def record():q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1 and (s['words'][16]>>8)&255==3,timeout=6)
def pcm():
 q.kind(0,0);q.setup();q.knob(0,.4);q.pad(4);q.button(6);record();time.sleep(.2);q.pad(0,hold=.04);assert state()['words'][507]==0;details['unfinished_input_cannot_audition_stale_pcm']=True;finish();s=state();assert s['words'][27]>40000 and s['words'][22]==0 and s['words'][16]&(1<<20)==0;details['input_pcm_frames']=s['words'][27];q.capture('actual-recorded-audio-loop');idle();assert state()['input_source']['selected']==2
 # WAV browser remains accessible only on empty tracks, protecting the new asset.
 q.select(1);q.kind(1,0);q.button(36);q.wait(lambda s:s['browser']['open']);q.capture('actual-existing-wav-browser');q.button(14);q.wait(lambda s:not s['browser']['open']);q.select(0)
def chromatic():
 target(1);edge(12,True);x=capture_audio(2.5);edge(12,False);m=metrics(x);assert m['finite'] and m['peak']>.01 and 420<m['crossing_hz']<460,m;time.sleep(.5);quiet=metrics(capture_audio(.4));assert quiet['peak']<1e-5,quiet;details['held_octave_repeats_after_source_end_and_release']=m;details['release_quiet']=quiet;q.capture('actual-chromatic-loop-ready')
 record();q.capture('actual-chromatic-capture');q.pad(7,hold=.25);q.pad(12,hold=.2);finish();s=state();assert s['words'][16]&(1<<20)!=0 and s['words'][27]==details['input_pcm_frames'];assert s['words'][22]>=4;assert all(s['words'][353+i*2]&(1<<24)==0 for i in range(min(64,s['words'][22])));q.capture('actual-chromatic-recorded-notes');q.button(7);q.capture('actual-chromatic-note-grid');q.button(6);idle();q.button(55,True);q.wait(lambda s:s['words'][16]&(1<<20)==0);q.button(54,True);q.wait(lambda s:s['words'][16]&(1<<20)!=0);details['chromatic_capture_and_undo_redo']=True
 # MIDI note release observes the same live sampler owner.
 midi.note(60,True,80);q.wait(lambda s:s['words'][1044]&(1<<60)!=0);midi.note(60,False);q.wait(lambda s:s['words'][1044]&(1<<60)==0);details['actual_usb_midi_sampler_note_off']=True
 idle()
def chops():
 target(2);q.button(13);q.wait(lambda s:s['page']==1);q.button(13);q.wait(lambda s:s['page']==8);q.capture('actual-chop-boundary-editor');q.button(6);record();q.pad(2,hold=.3);q.pad(5,hold=.18);finish();s=state();assert s['words'][27]==details['input_pcm_frames'] and s['words'][22]>=4;assert all(s['words'][353+i*2]&(1<<24)!=0 for i in range(min(64,s['words'][22])));q.capture('actual-chop-recorded-sequence');idle();target(1)
 # Existing chop events retain their tag after choosing CHROMATIC.
 assert all(state()['words'][353+i*2]&(1<<24)!=0 for i in range(min(64,state()['words'][22])));q.button(82);x=capture_audio(2.4);assert metrics(x)['peak']>.01;idle();q.button(14);q.wait(lambda s:s['words'][16]&(1<<18)!=0);record();q.pad(12,hold=.22);finish();s=state();flags=[s['words'][353+i*2] for i in range(min(64,s['words'][22]))];assert any(f&(1<<24)for f in flags)and any(f&(1<<24)==0 for f in flags);q.capture('actual-mixed-chop-chromatic-overdub');details['chop_record_and_cross_mode_overdub']=True;idle()
def warp():
 # Keep saved sequence silent during live pitch/performance inspection.
 q.button(13);q.wait(lambda s:s['page']==1);q.knob(2,1);q.wait(lambda s:s['words'][13]&(1<<8)!=0);q.capture('actual-audio-root-tune-warp');q.setup();q.knob(0,.24);q.event(kind='encoder',delta=1);q.button(6);assert abs(__import__('struct').unpack('<d',state()['words'][2].to_bytes(8,'little'))[0]-88)<.01
 edge(12,True);x=capture_audio(2.7);edge(12,False);m=metrics(x);assert m['finite']and m['peak']>.01 and 415<m['crossing_hz']<465,m;details['warp_octave_at_88_bpm']=m;idle()
 before=state()['audio_timing'];held=[0,4,7,12]
 for i in held:edge(i,True)
 x=capture_audio(4.)
 for i in held:edge(i,False)
 time.sleep(.2);after=state()['audio_timing'];assert np.isfinite(x).all();details['four_warp_voices_timing']={'before':before,'after':after,'audio':metrics(x)};q.capture('actual-warp-four-voices');idle()
def save_load():
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);before=state();q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20);s=state();assert s['words'][16]==before['words'][16]and s['words'][22]==before['words'][22]and s['words'][27]==details['input_pcm_frames']and s['words'][13]==before['words'][13];details['v5_save_load_pcm_sequence_settings']=True;q.button(6);q.capture('actual-audio-sequence-recalled')
s=state();assert s['version']in ['0.6.1','0.7.0']and s['words'][4]&1==0 and s['words'][9]==0 and s['words'][1055]==1 and s['project_storage']['session_id']is None
q.setup();seq=state()['project_io']['sequence'];q.pad(7);before=q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);(q.out/'owner-before.json').write_text(json.dumps(before,indent=2))
try:
 install(q.out/'qa.so','qa-stage');s=state();assert s['version']=='0.7.0'and s['project_storage']['session_id']=='audio-performance-final-20261123';midi=SampleMidi();device=next(i for i,d in enumerate(sd.query_devices())if'MPC Sample'in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 phase=[0]
 def output(data,frames,timing,status):
  t=np.arange(frames)+phase[0];phase[0]+=frames;data[:]=(.1*np.sin(t*2*np.pi*220/44100))[:,None]
 def duplex(indata,outdata,frames,timing,status):
  output(outdata,frames,timing,status);recorded.append(indata.copy())
 stream=sd.Stream(device=(device,device),samplerate=44100,channels=2,dtype='float32',callback=duplex);stream.start();streams.append(stream);time.sleep(.6)
 for name,fn in [('input_and_wav_browser',pcm),('chromatic_hold_capture_history_midi',chromatic),('chop_capture_mixed_overdub',chops),('warp_and_audio',warp),('project_recall',save_load)]:
  print('DEVICE',name,flush=True);q.check(name,fn);print('PASS',name,flush=True)
 (q.out/'verification.json').write_text(json.dumps({'groups':q.results,'details':details},indent=2));print('DEVICE GROUPS PASS',flush=True)
except Exception:
 (q.out/'partial-results.json').write_text(json.dumps(details,indent=2));
 try:q.capture('failure')
 except Exception:pass
 raise
finally:
 if midi is not None:midi.close()
 for stream in streams:stream.stop();stream.close()
 for i in range(16):edge(i,False)
 if state()['browser']['open']:q.button(14)
 idle();install(q.out/'normal.so','normal-owner-restore');s=state();assert s['project_storage']['session_id']is None
 for i in range(16):
  b=16+i*12
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][b+j]==before['words'][b+j],(i,j,s['words'][b+j],before['words'][b+j])
 assert s['words'][2]==before['words'][2]
 (q.out/'owner-restored.json').write_text(json.dumps(s,indent=2));print('OWNER 16 TRACKS RESTORED',flush=True)
 for suffix in ['so','elf','map']:shutil.copy2(q.out/f'normal.{suffix}',p/f'build/device/loop-club.{suffix}')
