#!/usr/bin/env python3
"""Actual device layer, mixer, save/recall and USB-audio QA. Isolated QA sessions only."""
import argparse,json,time,wave
from pathlib import Path
import numpy as np,sounddevice as sd
from qa_device_matrix import QA,state,pair

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);a=p.parse_args();q=QA(a.output)
 initial=state();assert initial['project_storage']['qa'] and initial.get('ui_revision')=='looper-first-layers'
 q.button(81);q.select(1);assert state()['words'][28]&255==2
 q.button(13);q.wait(lambda s:s['page']==10 and not(s['ui_position']&(1<<19)));q.pad(1);q.button(14);q.wait(lambda s:s['ui_position']&(1<<18))
 q.button(111);q.wait(lambda s:s['ui_position']&(1<<17));time.sleep(.3);q.capture('layer-bank-list')
 q.button(13);time.sleep(.6);q.capture('layer-samples');q.button(54);q.capture('layer-samples-page-two');q.button(55)
 seq=state()['project_io']['sequence'];q.button(13)
 q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==5,timeout=15);q.wait(lambda s:s['page']==10)
 b=800+(1*2+1)*7
 q.wait(lambda s:s['words'][b+2]&5==5)
 for i,v in enumerate([.28,.52,.62]):q.knob(i,v)
 q.wait(lambda s:abs(pair(s['words'][b])[0]-.28)<.001 and pair(s['words'][b])[1]==1 and pair(s['words'][b+1])[0]==12)
 q.capture('layer-two');q.button(54);q.wait(lambda s:s['ui_position']&(1<<19));q.knob(0,.03);q.knob(1,1);q.knob(2,1)
 q.wait(lambda s:s['words'][b+2]&3==3);q.capture('layer-detail-reverse');q.button(55);q.wait(lambda s:not(s['ui_position']&(1<<19)))
 q.button(13)
 for i,v in enumerate([.7,22/48,.58]):q.knob(i,v)
 q.capture('layer-one')
 # Record from explicitly named Sample USB input, never the Mac mic.
 ids=[i for i,d in enumerate(sd.query_devices())if d['name']=='MPC Sample Audio' and d['max_input_channels']>=2];assert len(ids)==1
 audio=sd.rec(44100*7,samplerate=44100,channels=2,dtype='float32',device=ids[0]);time.sleep(.2)
 q.button(14,True);q.pad(1,.15);time.sleep(.8) # only primary
 q.button(14,True);q.button(13,True);q.pad(1,.15);time.sleep(.8) # only secondary
 q.button(13,True);q.pad(1,.15);time.sleep(.8) # both
 sd.wait();assert np.isfinite(audio).all() and np.max(np.abs(audio))>1e-5
 with wave.open(str(a.output/'actual-layer-preview-usb.wav'),'wb')as w:w.setnchannels(2);w.setsampwidth(2);w.setframerate(44100);w.writeframes((np.clip(audio,-1,1)*32767).astype('<i2').tobytes())
 q.button(81);expected=state()['words'][800+2*7:800+4*7]
 seq=state()['project_io']['sequence'];q.setup();q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15)
 q.button(6);q.button(13);q.button(14);q.knob(0,.1);q.wait(lambda s:abs(pair(s['words'][b])[0]-.1)<.001)
 seq=state()['project_io']['sequence'];q.setup();q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==2,timeout=15)
 q.select(1);q.button(13);q.wait(lambda s:s['page']==10);assert state()['words'][814:828]==expected
 q.capture('layers-recalled');q.button(1);q.capture('track-mixer');q.button(12);q.wait(lambda s:s['page']==4);q.capture('drum-mixer');q.button(6);q.capture('main-ready')
 report={'scope':'actual device remote controls, active DRM and USB recording; not physical fingers','layer_sample_load':'PASS','volume_tune_shift':'PASS','reverse_enable':'PASS','save_recall_both_layers_and_pcm':'PASS','mixer_bars':'actual captures','usb_peak':float(np.max(np.abs(audio))),'usb_rms':float(np.sqrt(np.mean(audio**2))),'usb_finite':True,'portaudio_status':str(sd.get_status()),'audio_timing':state().get('audio_timing')}
 (a.output/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
