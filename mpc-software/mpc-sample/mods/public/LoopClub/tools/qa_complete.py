#!/usr/bin/env python3
"""Opt-in real MPC Sample0.9.0 USB QA.
Requires authorized owner save/app restart, current idle owner project, USB input,
matching isolated qa.so + normal.so in build/qa-device/deep-audio-0.9.0,
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

q=QA(p/'build/qa-device/deep-audio-0.9.0');details={};streams=[];midi=None;recorded=deque(maxlen=4096)
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
 q.kind(0,0);target(0);q.setup();q.knob(0,.4);q.pad(4);q.button(6);
 if state()['words'][16]&(1<<18):q.button(14)
 record();time.sleep(.2);q.pad(0,hold=.04);assert state()['words'][507]==0;details['unfinished_input_cannot_audition_stale_pcm']=True;finish();s=state();assert s['words'][27]>40000 and s['words'][22]==0 and s['words'][16]&(1<<20)==0;details['input_pcm_frames']=s['words'][27];q.capture('actual-recorded-audio-loop');idle();assert state()['input_source']['selected']==2
 # WAV browser remains accessible only on empty tracks, protecting the new asset.
 q.select(1);q.kind(1,0);q.button(36);q.wait(lambda s:s['browser']['open']);q.capture('actual-existing-wav-browser');q.button(14);q.wait(lambda s:not s['browser']['open']);q.select(0)
def chromatic():
 target(1);edge(12,True);x=capture_audio(2.5);edge(12,False);m=metrics(x);assert m['finite'] and m['peak']>.01 and 420<m['crossing_hz']<460,m;time.sleep(.5);quiet=metrics(capture_audio(.4));assert quiet['peak']<1e-5,quiet;details['held_octave_repeats_after_source_end_and_release']=m;details['release_quiet']=quiet;q.capture('actual-chromatic-loop-ready')
 record();q.button(13);q.wait(lambda s:s['page']==1);q.capture('actual-sound-during-sampler-recording');q.button(6);q.capture('actual-chromatic-capture');q.pad(7,hold=.25);q.pad(12,hold=.2);finish();s=state();assert s['words'][16]&(1<<20)!=0 and s['words'][27]==details['input_pcm_frames'];assert s['words'][22]>=4;assert all(s['words'][353+i*2]&(1<<24)==0 for i in range(min(64,s['words'][22])));q.capture('actual-chromatic-recorded-notes');q.button(7);q.capture('actual-chromatic-note-grid');q.button(6);idle();q.button(55,True);q.wait(lambda s:s['words'][16]&(1<<20)==0);q.button(54,True);q.wait(lambda s:s['words'][16]&(1<<20)!=0);details['chromatic_capture_and_undo_redo']=True
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
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);before=state();q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20);s=state();assert s['words'][16]==before['words'][16]and s['words'][22]==before['words'][22]and s['words'][27]==details['input_pcm_frames']and s['words'][13]==before['words'][13];details['v6_save_load_pcm_sequence_settings']=True;q.button(6);q.capture('actual-audio-sequence-recalled')
def st(s,i=0):return(s['words'][16+i*12]>>8)&255
def all16_preflight():
 for i in range(16):
  q.select(i)
  if state()['words'][20+i*12]>0:
   q.setup();q.pad(3,erase=True);q.wait(lambda s:s['words'][20+i*12]==0)
  q.kind(i,1)
 idle();q.setup();q.knob(0,1);q.pad(6);q.button(6)
 for i in range(16):
  q.pad(i,shift=True);q.wait(lambda s:s['words'][9]==i+1 and st(s,i)==2);q.pad(i,shift=True);q.wait(lambda s:s['words'][9]==0)
 q.select(0)
 for i in range(16):q.pad(i,hold=.02);assert state()['words'][9]==0 and state()['words'][20]==0
 q.capture('actual-all-16-pads-idle');details['all_16_record_targets_and_all_16_performance_pads']=True
 idle()

def get(path):
 from qa_device_matrix import request
 return request(path)
def browser():return json.loads(get('/browser.json'))
def choose(name):
 view=browser();entry=next((e for e in view['entries']if e['name']==name),None);assert entry is not None,(name,[e['name']for e in view.get('entries',[])])
 target=entry['index'];current=state()['browser']['row']
 for _ in range(abs(target-current)):q.event(kind='encoder',delta=1 if target>current else -1)
 q.wait(lambda s:s['browser']['row']==target);return entry
def open_fixture(name):
 q.button(36);q.wait(lambda s:s['browser']['open']);q.wait(lambda s:s['browser']['total']>0)
 root=json.loads(get('/boot-storage.json'))['card'];parts=Path(state()['project_storage']['directory']).relative_to(root).parts+('Fixtures',)
 for part in parts:
  before=browser()['path'];choose(part);q.button(13)
  deadline=time.monotonic()+10
  while browser().get('path')==before:
   assert time.monotonic()<deadline;time.sleep(.1)
 choose(name);q.capture('actual-wav-selection-'+name.replace(' ','-'));q.button(13)
def clear_audio():
 idle();q.select(0)
 if state()['words'][20]>0:q.setup();q.pad(3,erase=True);q.wait(lambda s:s['words'][20]==0)
 q.kind(0,0);q.button(6)
def wav_load_and_original_tune():
 import hashlib
 details['wav_loads']=[]
 for name,url,hz,expected in [('QA Mono 220.wav','/fixture-mono.wav',220,11025),('QA Stereo 330.wav','/fixture-stereo.wav',330,11024)]:
  clear_audio();before_bytes=get(url);before=hashlib.sha256(before_bytes).hexdigest();open_fixture(name);q.wait(lambda s:not s['browser']['open']and s['words'][27]==expected,timeout=15);q.capture('actual-loaded-'+str(expected));assert hashlib.sha256(get(url)).hexdigest()==before
  q.button(13);q.wait(lambda s:s['page']==1);q.knob(2,0);q.button(6);target(1);edge(0,True);x=capture_audio(1.2);edge(0,False);m=metrics(x);assert m['finite']and m['peak']>.01 and abs(m['crossing_hz']-hz)<8,m
  if expected==11024:
   right=metrics(x[:,::-1]);assert abs(right['crossing_hz']-440)<8,right
  # Whole loop Tune works with Warp OFF; audibly pitched source must retain its silence padding.
  idle();q.button(13);q.knob(1,.75);q.button(6);q.button(82);x=capture_audio(5.5);idle();peak=float(np.max(np.abs(x)));threshold=np.max(np.abs(x),axis=1)>1e-4;active=x[threshold,0];cross=np.count_nonzero((active[:-1]<=0)&(active[1:]>0));duration=len(active)/44100
  assert .01<peak<.981 and duration>0.1
  # Concatenating active segments creates at most one extra crossing per gate.
  inferred=cross/max(duration,1e-6);assert abs(inferred-hz*2)<35,{'name':name,'inferred':inferred,'active_seconds':duration}
  silence=float(np.mean(~threshold));assert silence>.55,{'silence_fraction':silence}
  q.button(13);q.knob(1,.5);q.button(6)
  details['wav_loads'].append({'name':name,'source_frames':expected,'source_sha256_before_after':before,'held_audio':m,'raw_loop_tune_hz':inferred,'silence_fraction':silence})
 # Invalid file is rejected while empty, without creating content or overwriting anything.
 clear_audio();open_fixture('QA Broken.wav');deadline=time.monotonic()+10
 while not browser().get('error'):
  assert time.monotonic()<deadline;time.sleep(.1)
 assert state()['words'][20]==0 and state()['words'][27]==0;q.capture('actual-invalid-wav-error');q.button(14)
 # Restore an actual fixture for the structural edit/save test.
 open_fixture('QA Mono 220.wav');q.wait(lambda s:not s['browser']['open']and s['words'][27]==11025,timeout=15);details['invalid_wav_rejected_without_content']=True;idle()
def chop_edit_recall():
 q.select(0);target(2)
 if state()['words'][16]&(1<<18):q.button(14)
 record();q.pad(15,hold=.22);finish();idle();s=state();assert any(s['words'][353+i*2]&255==15 and s['words'][353+i*2]&(1<<24)for i in range(min(64,s['words'][22])))
 q.button(75);q.knob(0,0,True);q.button(14);q.wait(lambda s:(s['words'][506]>>16)&255==1);s=state();assert all(s['words'][353+i*2]&255==0 for i in range(min(64,s['words'][22])));q.capture('actual-reduced-chops-remapped-notes');q.button(55,True);q.wait(lambda s:(s['words'][506]>>16)&255==16);assert state()['words'][353]&255==15;q.button(54,True);q.wait(lambda s:(s['words'][506]>>16)&255==1)
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20);assert state()['words'][353]&255==0 and state()['words'][27]==11025;q.button(6);q.capture('actual-remapped-chop-project-reloaded');details['real_chop_reduce_undo_redo_save_load']=True
 # INPUT OVERDUB must report explicit resample guidance without touching the performance.
 target(0);q.button(14);before=state()['words'][22];q.pad(0,shift=True);q.wait(lambda s:s['words'][7]==12);assert state()['words'][9]==0 and state()['words'][22]==before;q.capture('actual-input-overdub-protection');q.button(14);target(1);idle()
def pages_and_fx():
 q.kind(1,1);q.button(40)
 for i in range(7):
  q.wait(lambda s:((s['ui_position']>>8)&255)==i);q.capture('actual-synth-page-'+str(i))
  if i<6:q.button(54)
 q.button(6)
 for code,page in [(7,2),(12,9)]:
  if code==12:q.button(6)
  q.button(code);q.wait(lambda s:s['page']==page);q.capture('actual-global-page-'+str(page))
 q.setup();q.capture('actual-global-page-7');q.button(6);q.button(1);q.wait(lambda s:s['page']==3);q.button(111);q.wait(lambda s:s['page']==11);q.capture('actual-global-page-11');q.button(12);q.button(13);q.wait(lambda s:s['page']==5)
 # Two simultaneous hold FX, extra effect refusal, release after navigation and latch.
 edge(8,True);edge(9,True);q.wait(lambda s:s['words'][320]!=0 and s['words'][324]!=0);before=(state()['words'][320],state()['words'][324]);edge(10,True);assert (state()['words'][320],state()['words'][324])==before;edge(10,False);q.capture('actual-two-held-fx');q.button(6);edge(8,False);edge(9,False);q.wait(lambda s:s['words'][320]==0 and s['words'][324]==0)
 q.button(0);q.pad(12,shift=True);q.wait(lambda s:s['words'][320]==13);q.event(kind='encoder',delta=1);q.button(111);q.wait(lambda s:s['page']==14);q.capture('actual-global-page-14');q.button(12);q.button(14);q.wait(lambda s:s['page']==13);q.capture('actual-global-page-13');
 for _ in range(16):
  if (state()['ui_position']>>32)&255==5:break
  q.event(kind='encoder',delta=1)
 q.button(111);q.wait(lambda s:s['page']==12);q.capture('actual-global-page-12');q.button(12);q.button(6)
 q.kind(2,2);q.button(40);q.pad(0,shift=True);q.pad(1,shift=True);q.pad(2,shift=True);q.capture('actual-drum-generation');q.button(6);q.button(13);q.wait(lambda s:s['page']==10);q.capture('actual-global-page-10');q.button(1);q.button(12);q.wait(lambda s:s['page']==4);q.capture('actual-global-page-4');idle();details['all_page_contexts_and_fx_hold_latch_capacity']=True
 q.button(6)
def mixed_audio_soak():
 idle();q.select(0);target(1);q.button(13);q.knob(2,1);q.button(6)
 measurements=[]
 for samples,synths in [(4,8),(8,16)]:
  q.select(0)
  for n in range(48,48+samples):midi.note(n,True,55)
  q.select(1)
  for n in range(72,72+synths):midi.note(n,True,48)
  time.sleep(.5);before=state()['audio_timing'];x=capture_audio(4.5);after=state()['audio_timing'];assert np.isfinite(x).all() and 1e-4<float(np.max(np.abs(x)))<.981
  quiet=np.max(np.abs(x),axis=1)<1e-7;edges=np.diff(np.r_[False,quiet,False].astype(np.int8));a=np.flatnonzero(edges==1);b=np.flatnonzero(edges==-1);gap=max((b-a),default=0)
  measurements.append({'sampler_voices':samples,'synth_voices':synths,'peak':float(np.max(np.abs(x))),'longest_near_zero_frames':int(gap),'before_timing':before,'after_timing':after})
  for n in list(midi.held):midi.note(n,False)
  idle()
 q.capture('actual-post-soak-responsive');details['mixed_warp_synth_fx_soak']=measurements


def input_selection_and_owned_transport():
 idle();q.button(6,True);q.wait(lambda s:s['page']==7)
 selected=[]
 for i in [0,1,2]:
  q.pad(i);q.wait(lambda s:s['input_source']['selected']==i and not s['input_source']['pending'] and s['input_source']['result']==1,timeout=12);selected.append(dict(state()['input_source']))
  q.capture('actual-input-source-'+str(i))
 q.button(6)
 for _ in range(20):
  for on in [True,False]:q.event(kind='transport_button',code=7,on=on,shift=False,erase=False)
  assert state()['page']==2
 assert state()['remote_transport_consumed'];details['input_selection_and_owned_seq_transport']={'source_readbacks':selected,'queued_seq_presses':20,'scope':'real codec readback and synthetic owned hardware route; no physical button or analog signal claim'};q.capture('actual-owned-seq-route');q.button(6)
def loop_edit_and_chain():
 idle();q.select(2);assert state()['words'][16+2*12]&255==2 and state()['words'][20+2*12]>0
 # Direct UI copy from pad3 to pad5, preserving Drum and generated lanes.
 q.button(74,True);q.capture('actual-loop-edit-copy-source');q.button(12);q.pad(2);q.capture('actual-loop-edit-choose-target');seq=state()['project_io']['sequence'];q.pad(4);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==6,timeout=15)
 a=state();assert a['words'][16+4*12]&255==2 and a['words'][22+4*12]==a['words'][22+2*12];q.capture('actual-loop-copied')
 q.button(55,True);q.wait(lambda s:s['words'][20+4*12]==0);q.button(54,True);q.wait(lambda s:s['words'][20+4*12]>0)
 # Moving changes both locations, Undo restores both.
 q.button(14);q.pad(4);seq=state()['project_io']['sequence'];q.pad(5);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==7,timeout=15);assert state()['words'][20+4*12]==0 and state()['words'][20+5*12]>0;q.capture('actual-loop-moved')
 q.button(55,True);q.wait(lambda s:s['words'][20+4*12]>0 and s['words'][20+5*12]==0);q.button(54,True);q.wait(lambda s:s['words'][20+4*12]==0 and s['words'][20+5*12]>0)
 # Occupied target refuses atomically; delete needs same pad twice.
 q.button(12);q.pad(2);seq=state()['project_io']['sequence'];q.pad(5);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==8,timeout=15);assert state()['words'][20+2*12]>0 and state()['words'][20+5*12]>0;q.capture('actual-copy-occupied-refused')
 q.button(13);q.pad(5);assert state()['words'][20+5*12]>0;q.capture('actual-loop-delete-confirm');q.pad(5);q.wait(lambda s:s['words'][20+5*12]==0);q.button(55,True);q.wait(lambda s:s['words'][20+5*12]>0);q.button(54,True);q.wait(lambda s:s['words'][20+5*12]==0)
 q.button(111);q.button(6);q.button(82);q.button(74,True);q.capture('actual-loop-edit-running-refused');assert state()['words'][4]&1;idle()
 details['loop_copy_move_delete_undo_redo_and_refusals']=True
 # Mute leaves loop running; chain advances prepared loops and stops cleanly.
 q.button(6);q.button(13,True);q.pad(2);q.wait(lambda s:s['words'][16+2*12]&(1<<16)!=0);q.pad(2);q.wait(lambda s:s['words'][16+2*12]&(1<<16)==0)
 q.button(14,True);q.pad(2);q.pad(4) # pad5 is empty after move, must not be added
 q.capture('actual-chain-empty-loop-refused');q.pad(2);q.capture('actual-chain-two-steps');q.button(111);q.wait(lambda s:s['words'][1048]&(1<<9)!=0);time.sleep(2.5);q.capture('actual-chain-running');q.button(111);q.wait(lambda s:s['words'][1048]&(1<<9)==0);idle();q.button(6);details['mute_and_chain_start_stop']=True
 # Copy the existing Audio source as well, and persist both kinds without sharing buffers.
 q.button(74,True);q.button(12);q.pad(0);seq=state()['project_io']['sequence'];q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==6,timeout=15)
 a=state();assert a['words'][16+8*12]&255==0 and a['words'][27+8*12]==a['words'][27] and a['words'][22+8*12]==a['words'][22];q.capture('actual-audio-performance-copied');q.button(111);q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20);assert state()['words'][27+8*12]==a['words'][27]and state()['words'][22+8*12]==a['words'][22];details['copied_audio_source_midi_and_project_recall']=True;q.button(6)

s=state();assert s['version']in ['0.8.0','0.9.0']and s['words'][4]&1==0 and s['words'][9]==0 and s['words'][1055]==1 and s['project_storage']['session_id']is None
q.setup();seq=state()['project_io']['sequence'];q.pad(7);before=q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);(q.out/'owner-before.json').write_text(json.dumps(before,indent=2))
try:
 install(q.out/'qa.so','qa-stage');s=state();assert s['version']=='0.9.0'and s['project_storage']['session_id']=='deep-audio-20261123-r9';midi=SampleMidi();device=next(i for i,d in enumerate(sd.query_devices())if'MPC Sample'in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 phase=[0]
 def output(data,frames,timing,status):
  t=np.arange(frames)+phase[0];phase[0]+=frames;data[:]=(.1*np.sin(t*2*np.pi*220/44100))[:,None]
 def duplex(indata,outdata,frames,timing,status):
  output(outdata,frames,timing,status);recorded.append(indata.copy())
 stream=sd.Stream(device=(device,device),samplerate=44100,channels=2,dtype='float32',callback=duplex);stream.start();streams.append(stream);time.sleep(.6)
 for name,fn in [('all_16_tracks_and_pads',all16_preflight),('input_and_wav_browser',pcm),('chromatic_hold_capture_history_midi',chromatic),('chop_capture_mixed_overdub',chops),('warp_and_audio',warp),('project_recall',save_load),('actual_wav_load_and_tune',wav_load_and_original_tune),('chop_structure_and_recall',chop_edit_recall),('pages_and_fx',pages_and_fx),('mixed_soak',mixed_audio_soak),('input_and_owned_transport',input_selection_and_owned_transport),('loop_edit_mute_chain',loop_edit_and_chain)]:
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
 idle();
 if state()['input_source']['selected']!=before['input_source']['selected']:
  q.button(6,True);q.pad(before['input_source']['selected']);q.wait(lambda s:s['input_source']['selected']==before['input_source']['selected']and not s['input_source']['pending'],timeout=12)
 install(q.out/'normal.so','normal-owner-restore');s=state();assert s['project_storage']['session_id']is None
 for i in range(16):
  b=16+i*12
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][b+j]==before['words'][b+j],(i,j,s['words'][b+j],before['words'][b+j])
 assert s['words'][2]==before['words'][2]
 for w in [5,6,328,329,330,331,332,333,334,335,336,337,338,339,340,341,342,343,1024,1025,1026,1027,1028,1029,1030,1031,1032,1033,1034,1035,1036,1037,1038,1039,1040,1041,1042]:
  if w in [5,1028]:assert s['words'][w]&0xffffffff==before['words'][w]&0xffffffff,('global',w)
  elif w not in [6,1024]:assert s['words'][w]==before['words'][w],('global',w)
 (q.out/'owner-restored.json').write_text(json.dumps(s,indent=2));print('OWNER 16 TRACKS RESTORED',flush=True)
 for suffix in ['so','elf','map']:shutil.copy2(q.out/f'normal.{suffix}',p/f'build/device/loop-club.{suffix}')
