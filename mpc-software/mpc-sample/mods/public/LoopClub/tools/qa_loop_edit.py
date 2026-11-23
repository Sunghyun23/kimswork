#!/usr/bin/env python3
from pathlib import Path
import sys,json,time,subprocess,shutil
import argparse
parser=argparse.ArgumentParser(description='Final isolated Loop Club copy/move/delete/source-switch QA; saves owner project and restarts app')
parser.add_argument('--execute',action='store_true')
if not parser.parse_args().execute:parser.error('--execute is required')
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state
q=QA(p/'build/qa-device/deep-audio-0.9.0');details={}
def install(path,label):
 with(q.out/(label+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(path),'--restart','--discard-session'],stdout=f,stderr=subprocess.STDOUT,check=True)
def save_load():
 q.setup();seq=state()['project_io']['sequence'];q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq+1 and s['project_io']['result']==2,timeout=20);q.button(6)
def transfer(source,target,moving):
 q.button(14 if moving else 12);q.pad(source);seq=state()['project_io']['sequence'];q.pad(target);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==(7 if moving else 6),timeout=20)
s=state();assert s['version']=='0.9.0'and s['project_storage']['session_id']is None and s['words'][9]==0 and s['words'][4]&1==0
q.setup();seq=state()['project_io']['sequence'];q.pad(7);before=q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=20);(q.out/'owner-final-before.json').write_text(json.dumps(before,indent=2))
try:
 install(q.out/'qa.so','qa-final-stage');assert state()['project_storage']['session_id']=='deep-audio-20261123-r9'
 for kind in [1,0,2]:
  q.kind(0,kind);save_load();assert state()['words'][16]&255==kind and state()['words'][787]==2;q.capture('actual-xpm-source-switch-'+str(kind))
 details['dormant_xpm_synth_audio_drum_project6_recall']=True
 q.select(0);q.button(74,True);q.capture('actual-final-loop-edit-drum');q.button(73);assert state()['words'][9]==0;q.capture('actual-edit-record-refused')
 transfer(0,2,False);assert state()['words'][787]==2;q.button(55,True);q.wait(lambda s:s['words'][787]==0);q.button(54,True);q.wait(lambda s:s['words'][787]==2);transfer(2,3,True);assert state()['words'][787]==2;q.button(111);q.select(2);assert state()['words'][787]==0;q.button(55,True);q.wait(lambda s:s['words'][787]==2);q.button(54,True);q.wait(lambda s:s['words'][787]==0);details['xpm_whole_track_copy_move_undo_redo']=True
 q.select(8);source=state();assert source['words'][20+8*12]>0 and source['words'][27+8*12]>0;q.button(74,True);transfer(8,9,False);q.capture('actual-final-audio-copy');assert state()['words'][27+9*12]==source['words'][27+8*12];transfer(9,10,True);assert state()['words'][20+9*12]==0 and state()['words'][27+10*12]==source['words'][27+8*12];q.capture('actual-final-audio-move');q.button(55,True);q.wait(lambda s:s['words'][20+9*12]>0 and s['words'][20+10*12]==0);q.button(54,True);q.wait(lambda s:s['words'][20+9*12]==0 and s['words'][20+10*12]>0)
 q.button(13);q.pad(10);assert state()['words'][20+10*12]>0;q.capture('actual-final-delete-confirm');q.pad(10);q.wait(lambda s:s['words'][20+10*12]==0);q.button(55,True);q.wait(lambda s:s['words'][20+10*12]>0);q.button(54,True);q.wait(lambda s:s['words'][20+10*12]==0);q.button(111);details['final_audio_move_delete_undo_redo']=True
 (q.out/'verification-final.json').write_text(json.dumps(details,indent=2));print('FINAL DEVICE PASS',details,flush=True)
finally:
 if state()['browser']['open']:q.button(14)
 q.button(81);install(q.out/'normal.so','normal-final-owner-restore');s=state();assert s['project_storage']['session_id']is None
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert s['words'][16+i*12+j]==before['words'][16+i*12+j],('owner',i,j)
 assert s['words'][2]==before['words'][2]and s['input_source']['selected']==before['input_source']['selected'];(q.out/'owner-final-restored.json').write_text(json.dumps(s,indent=2))
 for ext in ['so','elf','map']:shutil.copy2(q.out/f'normal.{ext}',p/f'build/device/loop-club.{ext}')
 print('FINAL OWNER16 RESTORED',flush=True)
