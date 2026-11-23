from pathlib import Path
import sys,json,time,subprocess,shutil
p=Path(__file__).resolve().parents[1];sys.path.insert(0,str(p/'tools'))
from qa_device_matrix import QA,state
import argparse
parser=argparse.ArgumentParser(description='Isolated ERASE/COPY QA. Requires authorized app restarts; prepare qa.so and previous.so in the evidence directory.')
parser.add_argument('--execute',action='store_true')
if not parser.parse_args().execute:parser.error('--execute is required')
out=p/'build/qa-device/erase-copy-0.11.2';q=QA(out);before=state();assert before['words'][9]==0 and before['words'][4]&1==0 and before['project_storage']['session_id']is None
(out/'saved-owner-before.json').write_text(json.dumps(before,indent=2))
def install(lib,label):
 with(out/(label+'.log')).open('w')as f:subprocess.run(['python3',str(p/'tools/qa_manage.py'),'--stage',str(lib),'--restart','--discard-session'],check=True,stdout=f,stderr=subprocess.STDOUT)
def button(code,shift=False):
 for on in [True,False]:q.event(kind='transport_button',code=code,on=on,shift=shift)
 # Callback queues are drained on the next paint, not synchronously.
 time.sleep(.25)
def transfer(a,b,move=False):
 button(9,True)
 if move:q.button(14)
 q.pad(a);seq=state()['project_io']['sequence'];q.pad(b)
 q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==(7 if move else 6),timeout=15)
try:
 install(out/'qa.so','qa-install');assert state()['project_storage']['session_id']=='erase-copy-20261123'
 q.kind(0,2);q.button(40);q.pad(0,shift=True);q.pad(1,shift=True);q.wait(lambda s:s['words'][22]>0);q.button(81);q.button(6)
 source=state()['words'][22];transfer(0,1);q.wait(lambda s:s['words'][34]==source);q.capture('actual-copy')
 button(9);q.pad(1);assert state()['words'][34]==source;q.capture('actual-delete-confirm');q.pad(1);q.wait(lambda s:s['words'][34]==0)
 q.button(55,True);q.wait(lambda s:s['words'][34]==source);q.button(54,True);q.wait(lambda s:s['words'][34]==0)
 transfer(0,1);transfer(1,2,True);q.wait(lambda s:s['words'][34]==0 and s['words'][46]==source);q.capture('actual-move')
 button(9,True);q.pad(0);seq=state()['project_io']['sequence'];q.pad(2);q.wait(lambda s:s['project_io']['sequence']>seq);assert state()['words'][22]==source and state()['words'][46]==source
 q.button(111);q.button(82);q.wait(lambda s:s['words'][4]&1!=0);button(9);q.capture('actual-running-refused');q.button(81);q.button(6)
 button(9,True);q.button(6);q.select(0);q.button(40);q.pad(3,shift=True);q.wait(lambda s:s['words'][481]&8!=0);q.pad(3,shift=True,erase=True);q.wait(lambda s:s['words'][481]&8==0)
 q.button(6);q.capture('actual-restored-loop')
 (out/'verification.json').write_text(json.dumps({'version':'0.11.2','result':'PASS','scope':'Actual Sample: injected independent button callback, controller pads and active DRM; not a physical finger press','cases':['SHIFT+ERASE copy with independent content','ERASE delete requires confirmation','delete undo/redo','move and occupied target protection','running refusal','SAMPLE exits edit','Drum SHIFT+ERASE+pad generation undo retained'],'remote_transport_edges':state()['remote_transport_edges']},indent=2))
 print('ERASE/COPY DEVICE PASS',flush=True)
finally:
 try:q.button(81)
 except Exception:pass
 install(out/'previous.so','owner-restore');s=state()
 for i in range(16):
  for j in [0,1,2,4,6,7,8,9,10,11]:assert before['words'][16+i*12+j]==s['words'][16+i*12+j],(i,j)
 assert before['words'][2]==s['words'][2] and before['input_source']['selected']==s['input_source']['selected']
 (out/'owner-restored.json').write_text(json.dumps(s,indent=2));print('OWNER16 RESTORED',flush=True)
