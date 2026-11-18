"""Dice-owned values only; existing MAIN locks retain their IDs."""
import json
from pathlib import Path
P=Path(__file__).resolve().parents[1]
def append(params):
 index={p['key']:i for i,p in enumerate(params)}
 front=['Swing','SnareLate','HatShift','RndNudge','RulePickFullKick','RulePickFullSnare','RulePickFullHat','RulePickFullPerc','MixKick','MixSnare','MixHat','MixPerc','HatRhythm','OpenHatRule','RideRule','CrashRule']
 targets={index[k]:index['DiceLock'+str(i)]for i,k in enumerate(front)}
 keys=['Drive','Dust','Room','Duck','LowCut','ModRate','ModDepth']
 for r in range(8):keys+=['XSample'+str(r)]+[name+str(r)for name in ['Level','Tune','Decay','Pan','Choke']]
 keys+=['L'+str(s)+'Sample'for s in range(6)]+['Cym'+str(s)+'Sample'for s in range(3)]
 new=[]
 for k in keys:
  if index[k]in targets:continue
  lock=len(params);new.append(lock);targets[index[k]]=lock
  params.append(dict(key='DiceValueLock'+k,name='Dice Lock '+params[index[k]]['name'],choices=['Off','On'],default=0,auto=False))
 for i in range(4):params.append(dict(key='FillDiceLock'+str(i),name='Fill '+chr(65+i)+' Lock',choices=['Off','On'],default=0,auto=False))
 for i in range(16):params.append(dict(key='SoundValueLock'+str(i),name='Sound Value Lock '+str(i+1),choices=['Off','On'],default=0,auto=False))
 for i in range(16):params.append(dict(key='SoundValueLockEnabled'+str(i),name='Sound Lock Available '+str(i+1),choices=['Off','On'],default=0,auto=False))
 params.append(dict(key='UnlockDice',name='Unlock All Dice',choices=['Off','On'],default=0,auto=False))
 unlock=len(params)-1
 for key,name in [('RollFill','Roll Fill'),('RollFx','Roll FX')]:params.append(dict(key=key,name=name,choices=['Off','On'],default=0,auto=False))
 (P/'dice-locks.json').write_text(json.dumps(dict(targets={str(k):v for k,v in targets.items()},new=new,unlock=unlock),indent=2)+'\n')
def definitions():
 c=json.loads((P/'dice-locks.json').read_text())
 return ['static constexpr int kExtraDiceValueLocks[]={'+','.join(map(str,c['new']))+'};','static int diceValueLock(int key){switch(key){'+''.join(f'case {k}:return {v};'for k,v in c['targets'].items())+'default:return -1;}}']
