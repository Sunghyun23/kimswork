#!/usr/bin/env python3
"""Capture only actual MPC Sample Audio USB input; never the default microphone."""
import argparse,time,json,wave
from pathlib import Path
import sounddevice as sd,numpy as np
from qa_device_matrix import QA,state

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);p.add_argument('--track',type=int,default=0);p.add_argument('--pattern',action='store_true');a=p.parse_args()
    out=a.output;out.mkdir(parents=True,exist_ok=True);q=QA(out);q.button(81);q.select(a.track);q.button(40)
    q.wait(lambda s:s['words'][1]==44100 and s['words'][16+a.track*12]&255 in (1,2))
    during=[]
    devices=sd.query_devices();matches=[i for i,d in enumerate(devices)if d['name']=='MPC Sample Audio' and d['max_input_channels']>=2]
    if len(matches)!=1:raise RuntimeError('Exactly one MPC Sample Audio input is required; default mic is never used')
    index=matches[0];sd.check_input_settings(device=index,samplerate=44100,channels=2,dtype='float32')
    audio=sd.rec(44100*6,samplerate=44100,channels=2,dtype='float32',device=index)
    time.sleep(.3)
    if a.pattern:q.button(82)
    else:
        for pad in [0,4,7,11]:
            q.event(kind='pad',index=pad,on=True,velocity=90);time.sleep(.25);during.append(state());q.event(kind='pad',index=pad,on=False,velocity=0)
    sd.wait();q.button(81)
    peak=float(np.max(np.abs(audio)));rms=float(np.sqrt(np.mean(audio**2)))
    with wave.open(str(out/'actual-mpc-usb.wav'),'wb')as w:
        w.setnchannels(2);w.setsampwidth(2);w.setframerate(44100);w.writeframes((np.clip(audio,-1,1)*32767).astype('<i2').tobytes())
    report={'source':'actual MPC Sample Audio USB input; not offline rendering','device_index':index,'device_name':devices[index]['name'],'sample_rate':44100,'frames':len(audio),'peak':peak,'rms':rms,'portaudio_status':str(sd.get_status()),'finite':bool(np.isfinite(audio).all()),'nonzero_samples':int(np.count_nonzero(audio))}
    (out/'held-states.json').write_text(json.dumps(during,indent=2)+'\n')
    (out/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
