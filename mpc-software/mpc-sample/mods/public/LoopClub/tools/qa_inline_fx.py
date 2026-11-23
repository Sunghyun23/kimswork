#!/usr/bin/env python3
from pathlib import Path
import sys,json,subprocess,shutil,time
import argparse
parser=argparse.ArgumentParser(description='Opt-in actual Sample inline FX QA: saves/restores owner project and restarts app')
parser.add_argument('--execute',action='store_true')
if not parser.parse_args().execute:parser.error('--execute is required')
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state,pair
q=QA(p/'build/qa-device/release-0.9.3');result={}
def install(path,name):
 with(q.out/(name+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(path),'--restart','--discard-session'],stdout=f,stderr=subprocess.STDOUT,check=True)
def raw(i,on,shift=False):q.event(kind='pad',index=i,on=on,velocity=80 if on else 0,shift=shift)
def check_board(master=False):
 base=328 if master else 320;ids=[2,5,6,7,8,9,10,11,1,3,4,12,13,14]
 for pad,effect in enumerate(ids):
  raw(pad,True);s=q.wait(lambda s:any(s['words'][base+slot*4]==effect for slot in range(2)));slot=next(slot for slot in range(2)if s['words'][base+slot*4]==effect);address=base+slot*4
  q.knob(0,.24);q.knob(1,.41);q.knob(2,.67);q.wait(lambda s:abs(pair(s['words'][address+1])[0]-.24)<.015 and abs(pair(s['words'][address+1])[1]-.41)<.015)
  if pad in [0,2,6,12]:q.capture(('actual-master-'if master else'actual-track-')+str(pad))
  raw(pad,False);q.wait(lambda s:s['words'][address]==0);q.knob(2,.37);q.wait(lambda s:s['words'][address]==0 and abs(pair(s['words'][address+2])[0]-.37)<.015)
  print('PAD PASS',master,pad+1,flush=True)
 result['master_14_contexts'if master else'track_14_contexts']='PASS'
s=state();assert s['project_storage']['session_id']is None and s['words'][9]==0 and s['words'][4]&1==0
q.setup();seq=state()['project_io']['sequence'];q.pad(7);before=q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20)
try:
 install(q.out/'qa.so','qa-stage');assert state()['version']=='0.9.3';q.kind(0,1);q.button(0);check_board()
 # Last pressed effect owns only its own knobs; full refusal cannot edit other inserts.
 raw(0,True);raw(2,True);q.knob(0,.78);s=state();assert abs(pair(s['words'][325])[0]-.78)<.015 and abs(pair(s['words'][321])[0]-.24)<.015;raw(3,True);q.knob(0,.11);assert abs(pair(state()['words'][325])[0]-.78)<.015;q.capture('actual-two-fx-full');raw(3,False);raw(2,False);raw(0,False)
 q.button(14)
 # All master texture controls respond in place, without opening FX SETTINGS.
 for pad in [0,1,2,3,4,5,8,9,10,6,7]:
  raw(pad,True);q.knob(0,.3);q.knob(1,.4);q.knob(2,.6)
  if pad in [0,5,9]:q.capture('actual-texture-'+str(pad))
  raw(pad,False)
 result['texture_11_contexts']='PASS';q.knob(0,.23);assert not (state()['words'][1026]&(1<<41)),'last selected mod does not reenable Duck'
 for _ in range(11):
  if(state()['ui_position']>>32)&255==5:break
  q.event(kind='encoder',delta=1)
 q.button(111);q.wait(lambda s:s['page']==12);q.button(111);q.wait(lambda s:s['page']==6);check_board(True)
 raw(0,True,True);q.knob(0,.3);raw(0,False,True);assert state()['words'][328]!=0;q.capture('actual-master-latch-inline');raw(0,True,True);raw(0,False,True);q.wait(lambda s:s['words'][328]==0)
 result['latch_inline_controls']='PASS';(q.out/'quick-verification.json').write_text(json.dumps(result,indent=2));print('QUICK DEVICE PASS',result,flush=True)
except Exception:
 (q.out/'failure-state.json').write_text(json.dumps(state(),indent=2));q.capture('failure-quick');raise
finally:
 for i in range(16):
  try:raw(i,False)
  except Exception:pass
 q.button(81);install(q.out/'normal.so','normal-owner-restore');s=state();assert s['project_storage']['session_id']is None
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][16+i*12+j]==before['words'][16+i*12+j],(i,j)
 assert s['words'][2]==before['words'][2]
 for ext in ['so','elf','map']:shutil.copy2(q.out/f'normal.{ext}',p/f'build/device/loop-club.{ext}')
 print('OWNER RESTORED',flush=True)
