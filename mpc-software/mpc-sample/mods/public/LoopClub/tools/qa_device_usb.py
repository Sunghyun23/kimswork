#!/usr/bin/env python3
"""Verify actual USB stereo audio versus live MAIN bus telemetry; own QA only."""
import argparse,json,time,wave,struct
from pathlib import Path
import numpy as np,sounddevice as sd
from qa_device_matrix import QA,state

def floats(word):return struct.unpack('<ff',int(word).to_bytes(8,'little'))
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);a=p.parse_args();q=QA(a.output);s=state();assert s['project_storage']['qa']
 q.button(81);q.select(0);assert state()['words'][16]&255==1
 q.button(1);q.button(111);q.wait(lambda s:s['page']==11);q.knob(0,0);q.wait(lambda s:(s['words'][25]>>8)&255==0)
 q.button(40)
 # Isolate routing from the saved patch's changing filter/LFO envelope, using a steady own-QA tone.
 while (state()['ui_position']>>8)&255:q.button(54)
 for _ in range(3):q.button(54)
 q.knob(0,1);q.knob(1,0);q.knob(2,0);q.knob(1,.5,True)
 q.button(54);q.knob(0,0);q.knob(1,.2);q.knob(2,.8)
 q.button(54);q.button(54);q.knob(1,0);q.knob(2,0);q.knob(2,0,True);q.button(54);q.knob(1,0);q.button(54)

 ids=[i for i,d in enumerate(sd.query_devices())if d['name']=='MPC Sample Audio' and d['max_input_channels']==2];assert len(ids)==1
 audio=sd.rec(44100*15,samplerate=44100,channels=2,dtype='float32',device=ids[0]);started=time.monotonic();q.event(kind='pad',index=0,on=True,velocity=100)
 q.button(1);q.button(111);q.wait(lambda s:s['page']==11)
 windows=[]
 for index,(name,value)in enumerate([('MAIN',0),('USB',.5),('MAIN+USB',1)]):
  q.knob(0,value);q.wait(lambda s:(s['words'][25]>>8)&255==index);time.sleep(1.3);s=state();end=time.monotonic()-started;windows.append({'route':name,'start':max(0,end-.7),'end':end-.1,'main_peak':max(floats(s['words'][5])[1],floats(s['words'][6])[0]),'usb_meter_peak':max(floats(s['words'][1024]))});q.capture('route-'+str(index))
 q.event(kind='pad',index=0,on=False,velocity=0);q.button(81);sd.wait()
 for item in windows:
  segment=audio[round(item['start']*44100):round(item['end']*44100)];assert len(segment)>1000
  item['actual_usb_rms']=float(np.sqrt(np.mean(segment**2)));item['actual_usb_peak']=float(np.max(np.abs(segment)))
 (a.output/'measurements.json').write_text(json.dumps(windows,indent=2)+'\n')
 assert windows[0]['actual_usb_rms']<1e-5 and windows[0]['main_peak']>1e-4,windows
 assert windows[1]['actual_usb_rms']>1e-4 and windows[1]['main_peak']<1e-5,windows
 assert windows[2]['actual_usb_rms']>1e-4 and windows[2]['main_peak']>1e-4,windows
 assert np.isfinite(audio).all()
 with wave.open(str(a.output/'actual-usb-routing.wav'),'wb')as w:w.setnchannels(2);w.setsampwidth(2);w.setframerate(44100);w.writeframes((np.clip(audio,-1,1)*32767).astype('<i2').tobytes())
 # Persist one routed setting, mutate it, and verify recall before restoring compatibility default.
 q.knob(0,.5);seq=state()['project_io']['sequence'];q.setup();q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15)
 q.button(1);q.button(111);q.knob(0,1);seq=state()['project_io']['sequence'];q.setup();q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==2,timeout=15);assert (state()['words'][25]>>8)&255==1
 q.button(1);q.button(111);q.knob(0,1);q.capture('output-ready');seq=state()['project_io']['sequence'];q.setup();q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15);q.button(6)
 report={'usb_channels':2,'sample_rate':44100,'windows':windows,'main_evidence':'actual live engine MAIN bus telemetry; physical analog jack not recorded','usb_evidence':'actual MPC Sample Audio USB capture','save_recall':'PASS','final_track0':'MAIN+USB','finite':True,'portaudio_status':str(sd.get_status())};(a.output/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
