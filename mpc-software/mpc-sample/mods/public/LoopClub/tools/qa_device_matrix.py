#!/usr/bin/env python3
"""Actual Loop Club device QA via USB; requires isolated QA5 project storage.
Does not touch regular owner Session.lcproj. Stops on first failed assertion.
"""
import argparse,io,json,struct,time,urllib.request,urllib.error
from datetime import datetime,timezone
from pathlib import Path
from PIL import Image
BASE='http://192.168.155.1:8090'
def request(path,data=None):
    req=urllib.request.Request(BASE+path,data=None if data is None else json.dumps(data).encode(),headers={'Content-Type':'application/json'})
    with urllib.request.urlopen(req,timeout=5)as r:return r.read(2*1024*1024)
def state():return json.loads(request('/status.json'))
def pair(word):return struct.unpack('<ff',int(word).to_bytes(8,'little'))
class QA:
    def __init__(self,out):self.out=out;out.mkdir(parents=True,exist_ok=True);self.results=[]
    def wait(self,predicate,timeout=5):
        start=time.monotonic();s=state()
        while not predicate(s):
            if time.monotonic()-start>timeout:raise AssertionError('Timeout waiting for device state')
            time.sleep(.08);s=state()
        return s
    def event(self,**kw):request('/input',kw);time.sleep(.085)
    def button(self,code,shift=False):
        for on in [True,False]:self.event(kind='button',code=code,on=on,shift=shift)
    def pad(self,index,hold=.12,shift=False,erase=False):
        self.event(kind='pad',index=index,on=True,velocity=84,shift=shift,erase=erase);time.sleep(hold)
        self.event(kind='pad',index=index,on=False,velocity=0,shift=shift,erase=erase)
    def knob(self,index,value,shift=False):self.event(kind='knob',index=16+index,value=value,shift=shift)
    def select(self,index):self.button(6);self.pad(index);self.wait(lambda s:s['words'][3]==index)
    def setup(self):self.button(6);self.button(111,True);self.wait(lambda s:s['page']==7)
    def kind(self,index,kind):self.select(index);self.setup();self.pad(kind);self.wait(lambda s:s['words'][16+index*12]&255==kind)
    def capture(self,name):
        time.sleep(.3) # Wait for the visible DRM page to catch up with queued state.
        deadline=time.monotonic()+10
        while True:
            try:raw=request('/screen.bmp');break
            except urllib.error.HTTPError:
                if time.monotonic()>deadline:raise
                time.sleep(.2)
        Image.open(io.BytesIO(raw)).save(self.out/(name+'.png'))
        (self.out/(name+'.json')).write_text(json.dumps(state(),indent=2))
    def check(self,name,fn):
        before=state();started=time.monotonic()
        try:fn();result={'case':name,'result':'PASS'}
        except Exception as e:result={'case':name,'result':'FAIL','error':str(e)};raise
        finally:
            result.update(seconds=round(time.monotonic()-started,3),before=before,after=state());self.results.append(result)
            (self.out/'results.json').write_text(json.dumps({'scope':'actual remote controller and DRM; NOT physical input/audio certification','cases':self.results},indent=2));print(name,result['result'],flush=True)
    def navigate(self):
        self.kind(0,1)
        for code,page in [(6,0),(7,2),(40,1),(1,3),(0,5)]:
            self.button(code);self.wait(lambda s:s['page']==page);self.capture(f'nav-{page}')
        self.button(14);self.wait(lambda s:s['page']==13);self.capture('master-fx')
        self.button(6)
        for _ in range(25):self.button(7)
        self.wait(lambda s:s['page']==2);self.capture('grid-25-remote-presses')
    def synth(self):
        self.kind(0,1);self.knob(0,1);self.pad(4);self.button(40)
        self.wait(lambda s:s['page']==1)
        self.button(13);self.wait(lambda s:(s['ui_position']>>16)&1==1)
        self.pad(0);self.button(13);self.wait(lambda s:(s['ui_position']>>16)&1==0)
        for page in range(1,7):
            self.button(54);self.wait(lambda s:(s['ui_position']>>8)&255==page)
            for knob,value in enumerate([.25,.4,.6]):self.knob(knob,value)
            self.capture(f'synth-{page}')
        self.knob(0,.6,True);self.knob(2,.5,True);self.capture('lfo-shift')
        # Other per-track controls must not change during editor navigation.
        self.button(14);self.wait(lambda s:s['words'][16]&(1<<18)!=0)
        self.button(54);self.wait(lambda s:(s['ui_position']>>8)&255==7);self.knob(1,0);self.capture('synth-pan');self.button(54);self.wait(lambda s:(s['ui_position']>>8)&255==0)
        self.button(14);self.wait(lambda s:s['words'][16]&(1<<18)==0)
    def record(self):
        self.select(0);assert state()['words'][20]==0,'Recording test needs a fresh QA session';self.setup();self.pad(1);self.button(40);self.button(73)
        self.wait(lambda s:((s['words'][16]>>8)&255)==3)
        self.pad(0,.14);self.pad(4,.14);self.button(73)
        self.wait(lambda s:s['words'][20]>0,timeout=4)
        self.button(73);self.pad(0,.12);self.pad(4,.12);self.wait(lambda s:s['words'][9]==0,timeout=4)
        s=state();assert s['words'][22]>=2,'Expected recorded note events'
        assert s['words'][20]>0 and s['words'][20]%(4*(1<<24))==0,'Expected whole-bar loop in engine fixed-point ticks'
        self.button(7);self.knob(2,.4);self.capture('recorded-midi-grid')
        self.button(40);self.button(14);before=state()['words'][22]
        self.button(73);self.wait(lambda s:((s['words'][16]>>8)&255)==3,timeout=4)
        self.pad(7,.12);self.button(73);self.wait(lambda s:s['words'][9]==0,timeout=4)
        assert state()['words'][22]>=before+2,'OVERDUB must preserve old notes and add new ones'
        self.button(14);self.button(81);self.wait(lambda s:s['words'][4]&1==0)
    def drums(self):
        self.kind(1,2);assert state()['words'][32]==0,'Drum test needs a fresh QA session';self.button(40)
        for i in [0,1,2,3,7,13]:self.pad(i)
        self.pad(0,shift=True);self.wait(lambda s:s['words'][481]&1!=0)
        before=state()['words'][34];assert before>0
        self.pad(0,shift=True,erase=True);self.wait(lambda s:s['words'][481]&1==0)
        self.pad(0,shift=True);self.pad(1,shift=True);self.pad(2,shift=True)
        self.capture('generated-drums')
        self.button(36);self.wait(lambda s:s['ui_position']&(1<<17)!=0);self.capture('banks')
        self.button(13);time.sleep(.4);self.capture('bank-kits')
        self.button(13);self.wait(lambda s:s['words'][787]!=0,timeout=12);self.capture('loaded-drum-kit')
        self.wait(lambda s:s['ui_position']&(1<<17)==0)
        self.pad(2);self.knob(1,.2);self.capture('sampled-hat-decay')
    def countin(self):
        for track,bars in [(3,1),(4,2)]:
            self.button(81);self.kind(track,1);self.pad(4+bars);self.button(40);self.button(73)
            self.wait(lambda s:((s['words'][16+track*12]>>8)&255)==2)
            self.capture(f'countin-{bars}-bar');self.pad(0,.1)
            self.wait(lambda s:((s['words'][16+track*12]>>8)&255)==3,timeout=5)
            self.pad(7,.1);self.button(73);self.wait(lambda s:s['words'][16+track*12+4]>0,timeout=3)
            self.button(81);self.wait(lambda s:s['words'][9]==0)
            count=state()['words'][16+track*12+6];assert count>=2
        self.setup();self.pad(4);self.button(6)
    def analog(self):
        self.button(81);self.kind(5,0);self.pad(4);self.button(73)
        self.wait(lambda s:((s['words'][76]>>8)&255)==3,timeout=4)
        time.sleep(.25);self.button(73);self.wait(lambda s:s['words'][80]>0,timeout=4)
        self.button(73);self.wait(lambda s:s['words'][9]==0,timeout=4)
        s=state();assert s['words'][87]>0,'Analog capture must commit PCM frames'
        assert s['words'][792]==0 and s['words'][793]==0,'Input queue must not overflow/underrun'
        self.button(81);self.capture('analog-recording-committed')
    def chop(self):
        self.select(2);assert state()['words'][44]==0,'Resample test needs an empty destination';self.setup();self.pad(0);self.button(82);self.pad(9)
        self.wait(lambda s:s['words'][9]==3 and ((s['words'][40]>>8)&255)==3)
        time.sleep(.25);self.button(73);self.wait(lambda s:s['words'][9]==0,timeout=5)
        s=state();assert s['words'][44]>0 and s['words'][51]>0,'Expected resampled audio loop'
        peak=max(max(abs(v)for v in pair(x)) for x in s['words'][512:768]);assert peak>1e-7,'Resampled source must contain signal'
        self.button(81);self.button(75);self.wait(lambda s:s['page']==8);self.capture('chop-equal')
        self.knob(0,.2,True);self.button(14);self.wait(lambda s:s['words'][774]==0)
        for mode in [1,2,3]:
            self.button(13);self.wait(lambda s:(s['words'][506]>>8)&255==mode)
            self.button(14);self.wait(lambda s:s['words'][774]==0);self.capture(f'chop-mode-{mode}')
        before=state()['words'][506]&1;self.button(75,True);self.wait(lambda s:s['words'][506]&1!=before)
        self.pad(0,.15);self.capture('chop-played');self.button(6)
    def mix(self):
        self.button(1);self.wait(lambda s:s['page']==3)
        self.pad(1,erase=True);self.wait(lambda s:s['words'][28]&(1<<16)!=0)
        self.pad(1,erase=True);self.wait(lambda s:s['words'][28]&(1<<16)==0)
        self.pad(1,shift=True,erase=True);self.wait(lambda s:s['words'][28]&(1<<17)!=0)
        self.pad(1,shift=True,erase=True)
        for i,v in enumerate([.3,.4,.5]):self.knob(i,v)
        self.capture('mix-level-sends');self.button(12,True);self.wait(lambda s:s['page']==4)
        self.pad(2,shift=True);self.pad(2,erase=True);self.wait(lambda s:s['words'][222]&1!=0)
        self.pad(2,erase=True);self.capture('kit-mix')
    def effects(self):
        self.button(0);self.wait(lambda s:s['page']==5)
        self.pad(11) # default pad-toggle mode; clear only own QA effects
        self.pad(0);self.wait(lambda s:s['words'][320]!=0)
        self.pad(9);self.pad(7);self.wait(lambda s:s['words'][324]!=0)
        self.capture('two-track-fx');self.pad(11);self.capture('track-fx-off')
        self.button(14);self.wait(lambda s:s['page']==13);self.pad(0);self.pad(1)
        self.capture('master-fx-toggle');self.pad(12);self.button(81)
    def projects(self):
        self.button(81);time.sleep(1);self.select(0);self.button(40)
        while (state()['ui_position']>>8)&255:self.button(54)
        expected=state()['words'][349];sequence=state()['project_io']['sequence'];self.setup();self.pad(7)
        self.wait(lambda s:s['project_io']['sequence']>sequence and s['project_io']['result']==1,timeout=12)
        self.button(40);self.knob(0,.1 if pair(expected)[0]>1000 else .91)
        self.wait(lambda s:s['words'][349]!=expected,timeout=6)
        sequence=state()['project_io']['sequence'];self.setup();self.pad(8)
        self.wait(lambda s:s['project_io']['sequence']>sequence and s['project_io']['result']==2,timeout=12)
        self.wait(lambda s:s['words'][349]==expected,timeout=6)
        self.capture('project-loaded');self.button(6)
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path);p.add_argument('--section',choices=['navigation','synth','record','drums','countin','analog','chop','mix','effects','projects']);a=p.parse_args()
    out=a.output or Path('build/qa-device')/datetime.now(timezone.utc).strftime('matrix-%Y%m%dT%H%M%SZ')
    qa=QA(out);s=qa.wait(lambda s:s.get('build_id')=='qa5' and s['paint_count']>0)
    qa.button(81);s=qa.wait(lambda s:s['words'][4]&1==0 and s['words'][9]==0)
    assert s['words'][4]&1==0 and s['words'][9]==0,'QA session must be stopped'
    storage=s.get('project_storage',{});assert storage.get('qa') is True and storage.get('session_id'),'A dedicated QA run directory must be verified before tests'
    if a.section is None:assert all(s['words'][16+i*12+4]==0 for i in range(16)),'Full matrix requires a fresh empty QA run; no existing track is erased'
    sections=[('navigation',qa.navigate),('synth',qa.synth),('record',qa.record),('drums',qa.drums),('countin',qa.countin),('analog',qa.analog),('chop',qa.chop),('mix',qa.mix),('effects',qa.effects),('projects',qa.projects)]
    for name,fn in sections:
        if a.section is None or name==a.section:qa.check(name,fn)
    print(out.resolve())
if __name__=='__main__':main()
