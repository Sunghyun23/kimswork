#!/usr/bin/env python3
"""Opt-in live USB response measurement. Requires idle Loop Club0.11.4; changes transient selection/metro, restores SETUP; never saves/restarts."""
import argparse
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--execute',action='store_true')
if not parser.parse_args().execute:parser.error('--execute required')
from pathlib import Path
import sys,time,json,io,threading
import numpy as np,sounddevice as sd
from PIL import Image
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state,request
initial=state();out=p/('build/qa-device/latency-midi-'+initial['version']);q=QA(out);assert initial['version'] in ('0.11.4','0.11.5')and not initial['words'][4]&1 and initial['words'][9]==0
report={'version':initial['version'],'pid':initial['pid'],'scope':'Host HTTP send to state/active-DRM/actual USB capture; includes remote transport and observation, not physical key-to-analog latency. No owner project save/restart.','before_audio_timing':initial['audio_timing']};stream=None

def stats(a):return {'n':len(a),'min_ms':float(np.min(a)),'median_ms':float(np.median(a)),'p95_ms':float(np.percentile(a,95)),'max_ms':float(np.max(a))}
def edge(code,on):return request('/input',{'kind':'transport_button','code':code,'on':on,'shift':False})
def screen():return np.asarray(Image.open(io.BytesIO(request('/screen.bmp'))).convert('RGB'))[62:126,5:150].copy()
try:
 import ast,ctypes as c
 tree=ast.parse((p/'tools/qa_audio_performance.py').read_text());node=next(x for x in tree.body if isinstance(x,ast.ClassDef)and x.name=='SampleMidi');exec(compile(ast.Module(body=[node],type_ignores=[]),'existing-core-midi-helper','exec'))
 midi=SampleMidi();q.select(0);q.button(40);q.wait(lambda s:s['page']==1)
 device=next(i for i,d in enumerate(sd.query_devices())if'MPC Sample'in d['name']and d['max_input_channels']>=2 and d['max_output_channels']>=2)
 chunks=[];flags=[]
 def cb(x,y,n,clock,status):
  now=time.perf_counter();chunks.append((now,clock.inputBufferAdcTime-clock.currentTime,x.copy()));y.fill(0)
  if status:flags.append(str(status))
 stream=sd.Stream(device=(device,device),samplerate=44100,blocksize=128,channels=2,dtype='float32',callback=cb);stream.start();time.sleep(1);audio=[]
 for i in range(30):
  time.sleep(.7);chunks.clear();t=time.perf_counter();midi.note(60,True,100);deadline=t+2;hit=None
  while time.perf_counter()<deadline:
   for delivery,offset,x in list(chunks):
    indices=np.flatnonzero(np.max(np.abs(x),axis=1)>.001)
    if len(indices):
     sample=int(indices[0]);hit={'callback_observed_ms':(delivery-t)*1000,'adc_timestamp_estimate_ms':(delivery+offset+sample/44100-t)*1000};break
   if hit:break
   time.sleep(.001)
  midi.note(60,False);assert hit,'No USB MIDI sound';audio.append(hit)
 report['synth_usb_midi']={k:stats([x[k]for x in audio])for k in audio[0]};report['audio_trials']=audio;report['audio_callback_flags']=flags;report['usb_stream_latency_seconds']=list(stream.latency)
 report['after_audio_timing']=state()['audio_timing'];report['new_audio_deadline_overruns']=report['after_audio_timing']['overrun_blocks']-report['before_audio_timing']['overrun_blocks'];print(json.dumps({k:v for k,v in report.items()if 'trials'not in k},indent=2),flush=True)
finally:
 if stream:stream.stop();stream.close()
 if 'midi'in globals():midi.close()
 q.select(initial['words'][3]);q.button(111,True);q.wait(lambda s:s['page']==7 and s['ui_position']&(1<<52));q.capture('final-setup');report['final_pid']=state()['pid'];(out/'measurement.json').write_text(json.dumps(report,indent=2))
