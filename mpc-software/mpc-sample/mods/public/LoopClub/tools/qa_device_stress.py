#!/usr/bin/env python3
"""Actual Sample chord/FX/USB stress, in verified isolated QA session only."""
import argparse,json,time,wave
from pathlib import Path
import numpy as np,sounddevice as sd
from qa_device_matrix import QA,state
p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);p.add_argument('--split-usb',action='store_true');p.add_argument('--pads',type=int,choices=range(1,17),default=16);a=p.parse_args();q=QA(a.output)
s=state();assert s['project_storage']['qa'] and s['project_storage']['session_id']
q.button(81);q.select(0);assert state()['words'][16]&255==1
if a.split_usb:q.button(1);q.button(111);q.wait(lambda s:s['page']==11);q.knob(0,.5)

q.button(0);q.pad(11);q.pad(8);q.pad(0);time.sleep(.4)
if state()['words'][320]==0:q.pad(10);q.pad(0);time.sleep(.4)
q.pad(9);q.pad(5);time.sleep(.4);assert state()['words'][320]!=0 and state()['words'][324]!=0
q.button(14);q.pad(8);q.pad(11);q.pad(7);time.sleep(.4)
if state()['words'][328]==0:q.pad(10);q.pad(7);time.sleep(.4)
q.pad(6);time.sleep(.4);assert state()['words'][328]!=0 and state()['words'][332]!=0
q.button(14)
from qa_device_matrix import pair
for i in range(5):
 if pair(state()['words'][1033+i*2])[1]<.5:q.pad(i)
q.select(0);q.button(40)
while (state()['ui_position']>>8)&255:q.button(54)
if not(state()['ui_position']&(1<<16)):q.button(13)
q.knob(2,.99)
ids=[i for i,d in enumerate(sd.query_devices())if d['name']=='MPC Sample Audio' and d['max_input_channels']>=2];assert len(ids)==1
recording=sd.rec(44100*8,samplerate=44100,channels=2,dtype='float32',device=ids[0]);time.sleep(.3)
try:
    for i in range(a.pads):q.event(kind='pad',index=i,on=True,velocity=90)
    time.sleep(1);held=state();q.capture('held-chords-and-four-fx')
finally:
    for i in range(a.pads):q.event(kind='pad',index=i,on=False,velocity=0)
sd.wait();time.sleep(.4);after=state()
q.button(0);q.pad(11);q.pad(8);q.pad(11);q.button(14);q.pad(12);q.button(81);q.button(6)
with wave.open(str(a.output/'actual-stress-usb.wav'),'wb')as w:
    w.setnchannels(2);w.setsampwidth(2);w.setframerate(44100);w.writeframes((np.clip(recording,-1,1)*32767).astype('<i2').tobytes())
report={'scope':f'actual Sample {a.pads} five-note chords plus2track/2masterFX andUSBcapture; timing includes scheduler effects','voice_limit':held.get('voice_limit'),'split_usb':a.split_usb,'held':held,'after':after,'usb_peak':float(np.max(np.abs(recording))),'usb_rms':float(np.sqrt(np.mean(recording**2))),'usb_finite':bool(np.isfinite(recording).all()),'usb_status':str(sd.get_status()),'audio_timing':after.get('audio_timing')}
(a.output/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items()if k not in ['held','after']},indent=2))
