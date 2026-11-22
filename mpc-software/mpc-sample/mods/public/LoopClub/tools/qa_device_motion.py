#!/usr/bin/env python3
"""Actual Sample USB audio, PAN, multi-FX and sidechain checks; isolated QA only."""
import argparse,json,time,struct,wave,math
from pathlib import Path
import numpy as np,sounddevice as sd
from qa_device_matrix import QA,state,pair

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);a=p.parse_args();q=QA(a.output);assert state()['project_storage']['qa']
 ids=[i for i,d in enumerate(sd.query_devices())if d['name']=='MPC Sample Audio' and d['max_input_channels']==2];assert len(ids)==1
 def page(n):
  q.button(40)
  while (state()['ui_position']>>8)&255!=n:q.button(54)
 def tone():
  q.button(81);q.select(0)
  q.button(0);q.pad(11);q.button(14);q.pad(8);q.pad(11);q.button(14)
  for i in range(5):
   if pair(state()['words'][1033+i*2])[1]>.5:q.pad(i)
  q.button(111);q.knob(0,0);q.button(111)
  page(1);q.knob(0,0);q.knob(1,.5);q.knob(2,.6)
  page(2);q.knob(0,0);q.knob(1,.5);q.knob(2,.3)
  page(3);q.knob(0,1);q.knob(1,0);q.knob(1,.5,True);q.knob(2,0)
  page(4);q.knob(0,.15);q.knob(1,.3);q.knob(2,.8);q.knob(2,.2,True)
  page(6);q.knob(0,0,True);q.knob(1,0);q.knob(2,0);q.knob(2,0,True)
 def capture(name,seconds):
  audio=sd.rec(round(44100*seconds),samplerate=44100,channels=2,dtype='float32',device=ids[0]);return audio,time.monotonic()
 def save(name,audio):
  assert np.isfinite(audio).all();assert np.max(np.abs(audio))<.99
  with wave.open(str(a.output/(name+'.wav')),'wb')as w:w.setnchannels(2);w.setsampwidth(2);w.setframerate(44100);w.writeframes((audio*32767).astype('<i2').tobytes())
 tone();page(7);q.knob(0,.5);q.knob(1,1);q.knob(2,math.log(40)/math.log(400));page(0)
 audio,start=capture('auto-pan',5);q.event(kind='pad',index=0,on=True,velocity=94);time.sleep(4);q.event(kind='pad',index=0,on=False,velocity=0);sd.wait();save('actual-auto-pan',audio)
 energies=[np.mean(audio[i:i+4410]**2,axis=0) for i in range(22050,132300,4410)];ratios=[float((v[0]+1e-10)/(v[1]+1e-10))for v in energies];assert max(ratios)>4 and min(ratios)<.25,ratios
 # Put the trigger drum left, and a held target synth right, so right-channel
 # attenuation measures ducking without being masked by the trigger itself.
 q.button(81);q.select(1);q.button(0);q.pad(11);q.button(1);q.knob(0,0);q.knob(1,0);q.knob(2,0);q.knob(4,1);q.knob(4,.8)
 q.select(0);page(7);q.knob(0,1);q.knob(1,0);page(0);q.event(kind='pad',index=15,on=True,velocity=94)
 q.button(0);q.button(14);q.button(111);assert state()['page']==12
 delta=1-(state()['words'][1026]&255)
 if delta:q.event(kind='encoder',delta=delta)
 q.pad(0);q.knob(0,.9);q.knob(1,.4);q.knob(2,1)
 q.capture('actual-sidechain-settings')
 audio,start=capture('duck',7);time.sleep(1);baseline=(time.monotonic()-start-.4,time.monotonic()-start-.1)
 q.select(1);q.button(40);reductions=[];hits=[]
 for _ in range(4):
  q.event(kind='pad',index=0,on=True,velocity=100);hits.append(time.monotonic()-start);time.sleep(.12);reductions.append(pair(state()['words'][1028])[1]);q.event(kind='pad',index=0,on=False,velocity=0);time.sleep(.5)
 q.button(81);sd.wait();save('actual-sidechain',audio)
 def energy(bounds):return float(np.mean(audio[int(bounds[0]*44100):int(bounds[1]*44100),1]**2))
 before=energy(baseline);after=[energy((x+.02,x+.12)) for x in hits];assert max(reductions)>.5,reductions;assert min(after)<before*.5,(before,after)
 # Explicitly validate persisted settings, five master stages and two inserts.
 q.select(0);q.button(0);q.pad(11);q.pad(8);q.pad(0);time.sleep(.4)
 if state()['words'][320]==0:q.pad(10);q.pad(0);time.sleep(.4)
 q.pad(9);q.pad(5);time.sleep(.4);assert state()['words'][320]!=0 and state()['words'][324]!=0
 q.capture('actual-two-track-fx')
 q.button(14)
 for i in range(5):
  if pair(state()['words'][1033+i*2])[1]<.5:q.pad(i)
 q.capture('actual-master-texture')
 q.button(111);q.capture('actual-sidechain');q.button(12);q.select(0);page(7);q.capture('actual-pan-lfo');page(0)
 audio,start=capture('multi-fx',5);q.event(kind='pad',index=0,on=True,velocity=94);time.sleep(3);q.event(kind='pad',index=0,on=False,velocity=0);q.button(81);sd.wait();save('actual-multi-fx',audio);assert np.sqrt(np.mean(audio**2))>1e-5
 time.sleep(.5);seq=state()['project_io']['sequence'];q.setup();q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15)
 report={'result':'PASS','scope':'actual Sample USB recording and remote controls; not physical finger or analog-jack test','pan_left_right_energy_ratios':ratios,'sidechain_reductions':reductions,'sidechain_baseline_right_energy':before,'sidechain_post_trigger_right_energy':after,'five_texture_stages':True,'two_track_inserts':True,'saved':True,'audio_timing':state().get('audio_timing'),'finite':True,'portaudio_status':str(sd.get_status())}
 (a.output/'report.json').write_text(json.dumps(report,indent=2)+'\n');q.button(6);q.capture('actual-main');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
