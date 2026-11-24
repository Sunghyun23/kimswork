#!/usr/bin/env python3
from pathlib import Path
import sys,json,time,subprocess,hashlib,io
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state,pair,request
import argparse
parser=argparse.ArgumentParser(description='Opt-in isolated all-page input QA. Requires approved iterative restarts and an unchanged saved owner baseline; never saves the owner project.')
parser.add_argument('--execute',action='store_true')
parser.add_argument('--session',required=True,help='Fresh deep-audio- session matching the separately built qa.so')
parser.add_argument('--restore-library',required=True,type=Path)
parser.add_argument('--saved-baseline',required=True,type=Path,help='Known saved-owner status JSON; refuse changed track metadata/BPM')
a=parser.parse_args()
if not a.execute:parser.error('--execute required')
if not a.session.startswith('deep-audio-') or len(a.session)>48 or not all(c.isascii() and (c.isalnum() or c=='-')for c in a.session):parser.error('Invalid isolated fixture session')
out=p/'build/qa-device/buttons-0.11.6';q=QA(out);report={'scope':'Actual Sample queued independent button callbacks/controller inputs + active DRM. Not physical finger transport certification.','cases':[],'screens':{}};before=state();session=a.session;pid=None
assert before['words'][9]==0 and not before['words'][4]&1
assert hashlib.sha256(a.restore_library.read_bytes()).hexdigest()==json.loads(request('/boot-storage.json'))['runtime_sha256']
import shutil
baseline=json.loads(a.saved_baseline.read_text())
for i in range(16):
 for j in [0,1,2,4,6,7,8,9,10,11]:assert before['words'][16+12*i+j]==baseline['words'][16+12*i+j], 'Owner state differs; preserve it before restarting'
assert before['words'][2]==baseline['words'][2], 'Owner BPM differs; preserve it before restarting'
shutil.copy2(a.restore_library,out/'previous.so')
(out/'owner-before.json').write_text(json.dumps(before,indent=2))
def install(path,name):
 with(out/(name+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(path),'--restart','--discard-session'],check=True,stdout=f,stderr=subprocess.STDOUT)
def same():
 if pid is not None:assert state()['pid']==pid,'Unexpected app restart'
def button(code,shift=False):
 same();n=state()['button_callbacks']
 for on in [True,False]:q.event(kind='transport_button',code=code,on=on,shift=shift)
 s=q.wait(lambda s:s['button_callbacks']>=n+2);assert s['button_callbacks']==n+2,(code,n,s['button_callbacks']);same();report['cases'].append({'button':code,'shift':shift,'callback_edges':2})
def screen(name,page):
 q.wait(lambda s:s['page']==page);same();q.capture(name);report['screens'][name]=page;print('SCREEN',name,page,flush=True)
 from PIL import Image
 assert Image.open(out/(name+'.png')).getpixel((1,60))[:3]==(20,38,32),'Not active Loop Club body'
def stop():button(81);q.wait(lambda s:s['words'][9]==0 and not s['words'][4]&1,timeout=8)
def raw(pad,on,shift=False):same();q.event(kind='pad',index=pad,on=on,velocity=84 if on else 0,shift=shift)
def browse():return json.loads(request('/browser.json'))
def choose(name):
 b=browse();assert not b.get('error'),b.get('error');target=next(v['index']for v in b['entries']if v['name']==name);row=state()['browser']['row']
 for _ in range(abs(target-row)):q.event(kind='encoder',delta=1 if target>row else -1)
 q.wait(lambda s:s['browser']['row']==target);button(13);time.sleep(.25)
def fixtures():
 for name in ['LoopClub','Projects','QA',session,'Fixtures']:
  old=browse()['path'];choose(name);q.wait(lambda s:browse()['path']!=old,timeout=10)
def count(i):return state()['words'][22+12*i]
try:
 install(out/'qa.so','qa-stage');assert state()['project_storage']['session_id']==session;pid=state()['pid'];assert all(state()['words'][20+12*i]==0 for i in range(16))
 q.kind(0,1);q.setup();q.knob(0,1);q.knob(1,0);button(6);screen('01-loop',0)
 button(12);screen('02-source',9);button(12);button(40);screen('03-synth-play',1)
 for expected in range(1,7):
  button(54);q.wait(lambda s:((s['ui_position']>>8)&255)==expected);screen('synth-subpage-'+str(expected),1)
 for expected in range(5,-1,-1):button(55);q.wait(lambda s:((s['ui_position']>>8)&255)==expected)
 # Toggle once, release must not toggle back. Chord and record mode stay independent.
 original=bool(state()['words'][16]&(1<<19));mode=bool(state()['words'][16]&(1<<18));button(13);q.wait(lambda s:bool(s['words'][16]&(1<<19))!=original);assert bool(state()['words'][16]&(1<<18))==mode;button(13);q.wait(lambda s:bool(s['words'][16]&(1<<19))==original)
 button(14);q.wait(lambda s:bool(s['words'][16]&(1<<18))!=mode);button(14);q.wait(lambda s:bool(s['words'][16]&(1<<18))==mode)
 button(12,True);screen('04-perform',15);button(12);q.wait(lambda s:s['page']==1)
 button(6);q.pad(0,shift=True);q.wait(lambda s:s['words'][9]==1 and s['words'][8]>=s['words'][10],timeout=6);q.pad(0,.12);q.pad(4,.12);button(73);q.wait(lambda s:s['words'][9]==0 and s['words'][20]>0,timeout=6);stop();n=count(0);assert n>=4
 button(7);screen('05-midi-grid',2)
 for _ in range(20):button(7);q.wait(lambda s:s['page']==2)
 button(55,True);q.wait(lambda s:count(0)==0);button(54,True);q.wait(lambda s:count(0)==n)
 report['record_finish_undo_redo_once']=True
 button(6);button(13,True);q.wait(lambda s:(s['ui_position']>>29)&3==1);q.pad(0);q.wait(lambda s:s['words'][16]&(1<<16));q.pad(0);q.wait(lambda s:s['words'][16]&(1<<16)==0)
 button(14,True);q.wait(lambda s:(s['ui_position']>>29)&3==2);q.pad(0);q.wait(lambda s:s['words'][1048]&255==1);assert state()['words'][9]==0;button(12,True);q.wait(lambda s:(s['ui_position']>>29)&3==0);report['mute_chain_no_record_fallthrough']=True
 button(1);screen('06-mix',3);button(111);screen('07-output',11);button(111);q.wait(lambda s:s['page']==3)
 button(0);screen('08-track-fx',5)
 ids=[2,5,6,7,8,9,10,11,1,3,4,12,13,14]
 def board(base):
  for pad,effect in enumerate(ids):
   old=[state()['words'][base+4*j:base+4*j+4]for j in range(2)];raw(pad,True);live=q.wait(lambda s:any(s['words'][base+4*j]==effect for j in range(2)));slot=next(j for j in range(2)if live['words'][base+4*j]==effect);raw(pad,False);q.wait(lambda s:s['words'][base+4*slot]==0);assert state()['words'][base+4*(1-slot):base+4*(1-slot)+4]==old[1-slot]
  raw(0,True,True);raw(0,False,True);q.wait(lambda s:s['words'][base]==2);raw(0,True,True);raw(0,False,True);q.wait(lambda s:s['words'][base]==0)
 board(320)
 # A held insert must release against its original slot after changing page.
 raw(0,True);q.wait(lambda s:s['words'][320]==2);button(14);q.wait(lambda s:s['page']==13);raw(0,False);q.wait(lambda s:s['words'][320]==0);screen('09-master-texture',13)
 for i in range(11):raw(i,True);q.knob(0,.3);raw(i,False)
 # Use DATA to focus Duck without touching any background track.
 for _ in range(12):
  if (state()['ui_position']>>32)&255==5:break
  q.event(kind='encoder',delta=1)
 assert (state()['ui_position']>>32)&255==5;button(111);screen('10-sidechain',12);button(111);screen('11-master-inserts',6);board(328)
 raw(11,True,True);raw(11,False,True);q.wait(lambda s:12 in [s['words'][328],s['words'][332]]);button(111);screen('12-fx-settings',14)
 base=328 if state()['words'][328]==12 else 332
 old=bool(state()['words'][base+3]&1);button(14);q.wait(lambda s:bool(s['words'][base+3]&1)!=old);shape=(state()['words'][base+3]>>5)&7;button(13);q.wait(lambda s:(s['words'][base+3]>>5)&7==(shape+1)%6);button(111);q.wait(lambda s:s['page']==6);raw(11,True,True);raw(11,False,True);q.wait(lambda s:all(s['words'][328+4*j]==0 for j in range(2)))
 report['track14_master14_latch_fx_navigation_release_and_settings']=True
 q.kind(1,2);button(6);button(13);screen('13-drum-sound',10)
 # +/- only changes one detail page per press/release.
 for _ in range(6):
  old=bool(state()['ui_position']&(1<<19));button(54);q.wait(lambda s:bool(s['ui_position']&(1<<19))!=old)
 button(1);button(12,True);screen('14-kit-mix',4)
 # Library modal owns +/-, including Shift+/- over PROJECT SETUP.
 q.setup();timing=bool(state()['ui_position']&(1<<52));button(36);q.wait(lambda s:s['browser']['open']);q.wait(lambda s:s['browser']['total']>0,timeout=15);bpm=state()['words'][2];row=state()['browser']['row'];total=state()['browser']['total']
 for code,shift,delta in [(54,False,1),(55,False,-1),(54,True,6),(55,True,-6)]:
  expected=max(0,min(total-1,row+delta));button(code,shift);q.wait(lambda s:s['browser']['row']==expected);row=expected;assert state()['words'][2]==bpm and bool(state()['ui_position']&(1<<52))==timing
 button(14);assert not state()['browser']['open'];report['kit_browser_modal_priority_plus_minus_and_shift']=True
 # Generated XPM only; no commercial payload is loaded or saved.
 button(36,True);q.wait(lambda s:s['browser']['open']);fixtures();choose('QA Banks.xpm');q.wait(lambda s:state()['browser']['total']==3,timeout=15);button(13);q.wait(lambda s:not s['browser']['open']and s['words'][787]!=0,timeout=15);report['generated_xpm_bank_load']=True
 q.kind(2,0);q.setup();button(36);q.wait(lambda s:s['browser']['open']);fixtures();q.capture('sample-browser-generated-fixtures');choose('QA Mono 220.wav');q.wait(lambda s:not s['browser']['open']and s['words'][51]>0,timeout=15)
 button(6);button(40);q.wait(lambda s:s['page']==1);button(75);screen('15-chop',8);q.pad(0,.12)
 button(40)
 for _ in range(6):
  if (state()['words'][40]>>21)&3==3:break
  button(12)
 q.wait(lambda s:(s['words'][40]>>21)&3==3);q.knob(4,.5);raw(0,True);q.knob(4,0);q.wait(lambda s:pair(s['words'][1192])[0]==0);q.knob(4,1);q.wait(lambda s:pair(s['words'][1192])[0]==1);raw(0,False);q.wait(lambda s:s['words'][1191]&2==0);stop();q.capture('turntable-release')
 q.setup();button(54);q.wait(lambda s:s['ui_position']&(1<<52));screen('16-setup-timing',7)
 metro=bool(state()['words'][4]&(1<<16));button(12);q.wait(lambda s:bool(s['words'][4]&(1<<16))!=metro);button(12);q.wait(lambda s:bool(s['words'][4]&(1<<16))==metro)
 old=(state()['words'][4]>>8)&255;button(13);q.wait(lambda s:((s['words'][4]>>8)&255)!=old);button(13);q.wait(lambda s:((s['words'][4]>>8)&255)==old)
 button(54);q.wait(lambda s:s['ui_position']&(1<<52)==0);screen('setup-project',7)
 # Same modal regression in sample browser over Setup, using an empty audio slot.
 q.kind(3,0);q.setup();timing=bool(state()['ui_position']&(1<<52));button(36);q.wait(lambda s:s['browser']['open']);q.wait(lambda s:s['browser']['total']>1,timeout=15)
 row=state()['browser']['row'];button(54);q.wait(lambda s:s['browser']['row']==row+1);assert bool(state()['ui_position']&(1<<52))==timing;button(55);q.wait(lambda s:s['browser']['row']==row);button(14)
 button(6,True);q.wait(lambda s:s['page']==7 and s['ui_position']&(1<<28));q.capture('setup-input');old=state()['input_source']['selected'];q.event(kind='encoder',delta=-1);assert state()['input_source']['selected']==old;button(14);q.wait(lambda s:s['page']==0)
 report['sample_browser_project_modal_input_selection_not_auto_committed']=True
 # Save/recall is confined to this unique QA directory.
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20)
 assert set(report['screens'].values())==set(range(16)),report['screens'];report['all16_pages_captured']=True;report['same_pid']=pid;report['isolated_save_recall']=True;print('ALL PAGE DEVICE PASS',json.dumps(report),flush=True)
except BaseException:
 try:(out/'failure-state.json').write_text(json.dumps(state(),indent=2));q.capture('failure')
 except Exception:pass
 raise
finally:
 try:
  for i in range(16):raw(i,False)
  button(81)
 except Exception:pass
 install(out/'previous.so','owner-restore');s=q.wait(lambda s:s['project_io']['result']==2,timeout=15)
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][16+12*i+j]==before['words'][16+12*i+j],(i,j)
 assert s['words'][2]==before['words'][2]and s['input_source']['selected']==before['input_source']['selected'];report['saved_owner16_restored']=True;(out/'verification.json').write_text(json.dumps(report,indent=2));print('OWNER16 RESTORED',flush=True)
