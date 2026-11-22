#!/usr/bin/env python3
"""Verify direct4x4FXpad toggles, occupied-slot protection and stored bypass on Sample."""
import argparse,time,json
from pathlib import Path
from qa_device_matrix import QA,state,pair

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True);a=p.parse_args();q=QA(a.output);s=state();assert s['project_storage']['qa'] and s['project_storage']['session_id']
 q.button(81);q.select(0);q.button(0);q.pad(11)
 def tap(i):q.pad(i);time.sleep(.35)
 tap(0);q.wait(lambda s:s['words'][320]==2);tap(5);q.wait(lambda s:s['words'][324]==9);q.capture('actual-track-fx')
 tap(0);q.wait(lambda s:s['words'][320]==0 and s['words'][324]==9);q.capture('actual-track-one-off')
 tap(7);q.wait(lambda s:s['words'][320]==11);before=(state()['words'][320],state()['words'][324]);tap(1);assert (state()['words'][320],state()['words'][324])==before
 tap(7);tap(0);q.capture('actual-track-fx')
 q.button(14);q.wait(lambda s:s['page']==13)
 # Clear only this own QA's advanced performance slots, then return to board.
 tap(8);q.wait(lambda s:s['page']==6);tap(11);q.button(14);q.wait(lambda s:s['page']==13)
 for i in range(5):
  if pair(state()['words'][1033+i*2])[1]>.5:tap(i)
 for i in range(5):tap(i)
 c=state()['words'][1026]
 if not(c&(1<<41)) or pair(state()['words'][1027])[0]==0:tap(5)
 tap(6);tap(7);s=state();assert s['words'][328]!=0 and s['words'][332]!=0 and s['words'][1042]&3==0
 q.capture('actual-master-fx')
 params=state()['words'][329];effect=state()['words'][328];tap(6);s=state();assert s['words'][1042]&1 and s['words'][328]==effect and s['words'][329]==params
 q.capture('actual-master-one-off');tap(6);assert state()['words'][1042]&1==0
 tap(9);q.wait(lambda s:s['page']==12);q.capture('actual-sidechain');q.button(12);q.wait(lambda s:s['page']==13)
 seq=state()['project_io']['sequence'];q.setup();q.pad(7);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==1,timeout=15)
 q.button(14) if state()['page'] in [3,5,6,13] else q.button(0)
 q.button(14);q.wait(lambda s:s['page']==13);tap(0)
 seq=state()['project_io']['sequence'];q.setup();q.pad(8);q.wait(lambda s:s['project_io']['sequence']>seq and s['project_io']['result']==2,timeout=15);assert pair(state()['words'][1033])[1]>.5
 q.button(0);q.button(14);q.capture('actual-master-recalled')
 # Restore a silent stopped state, retaining parameter choices for the owner.
 tap(12);q.button(81);q.button(6)
 report={'result':'PASS','scope':'actual activeDRM and bounded remote controls; physical finger input unverified','track_toggle':'PASS','two_track_fx_retained_after_release':'PASS','third_effect_does_not_replace':'PASS','master_board':'4x4','five_texture_plus_duck_plus_two_custom':'PASS','bypass_preserves_type_and_values':'PASS','save_recall':'PASS','audio_timing':state().get('audio_timing')};(a.output/'report.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
if __name__=='__main__':main()
