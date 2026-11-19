"""Append-only 1.14.0 editing relays and independent oscillator controls."""
import json
from pathlib import Path
P=Path(__file__).resolve().parents[1]
def append(params):
 def f(k,n,lo,hi,d,u=''):params.append(dict(key=k,name=n,min=lo,max=hi,default=d,unit=u,log=False))
 def c(k,n,opts,d=0):params.append(dict(key=k,name=n,choices=opts,default=d))
 for osc in (1,2):
  f(f'OscTune{osc}',f'OSC {osc} Tune',-48,48,0,'st')
  f(f'OscFine{osc}',f'OSC {osc} Fine',-100,100,0,'ct')
  f(f'OscLevel{osc}',f'OSC {osc} Level',0,100,60 if osc==1 else 40,'%')
 f('Pulse2','OSC 2 Pulse Width',5,95,50,'%')
 f('LfoDelay','LFO 1 Delay',0,5000,0,'ms');f('Lfo2Delay','LFO 2 Delay',0,5000,0,'ms')
 c('OscFocus','Edit Oscillator',['OSC 1','OSC 2'])
 c('PitchResolution','Pitch Resolution',['Tune','Fine'])
 c('LfoFocus','Edit LFO',['LFO 1','LFO 2'])
 c('PerfWave','Selected OSC Wave',['Sine','Triangle','Saw','Pulse'],1)
 f('PerfPitch','Selected OSC Pitch',0,1,.5)
 f('PerfLevel','Selected OSC Level',0,100,60,'%')
 f('PerfLfoRate','Selected LFO Rate',0,1,0)
 f('PerfLfoDepth','Selected LFO Depth',0,100,12,'%')
 f('PerfLfoDelay','Selected LFO Delay',0,5000,0,'ms')
 c('PerfLfoTarget','Selected LFO Target',['Pitch','Filter','Pulse Width','Amp','OSC Mix'],1)
 c('PerfLfoShape','Selected LFO Shape',['Sine','Triangle','Saw','Square','S&H'])
 c('PerfLfoSync','Selected LFO Sync',['Off','On'])
 c('PerfVoice','Performance Voice',['Mono','Poly'],1)
 c('MonoLegato','Mono Legato',['Off','On'],1)
 c('DiceTarget','Dice Target',['Auto','Bass','Keys','Synth','Lead','Pad'])
 c('UndoDice','Undo Dice',['Off','On'])
 c('Lfo1ShapeFull','LFO 1 Shape',['Sine','Triangle','Saw','Square','S&H'])
 c('Lfo1TargetFull','LFO 1 Target',['Pitch','Filter','Pulse Width','Amp','OSC Mix'],1)
 # Extend the lock table without changing any of its first 54 locks/indices.
 d=json.loads((P/'dice-locks.json').read_text());keys={p['key']:i for i,p in enumerate(params)}
 for k in ['OscTune1','OscFine1','OscLevel1','OscTune2','OscFine2','OscLevel2','Pulse2']:
  lock=len(params);c('DiceValueLock'+k,'Dice Lock '+params[keys[k]]['name'],['Unlocked','Locked'])
  d['owners'].append(k);d['locks'][k]=lock;d['targets'][str(keys[k])]=lock
 f('OscPulse1','OSC 1 Pulse Width',5,95,50,'%')
 d['targets'][str(len(params)-1)]=d['locks']['Pulse']
 d['targets'][str(keys['Lfo1ShapeFull'])]=d['locks']['Shape']
 d['targets'][str(keys['Lfo1TargetFull'])]=d['locks']['Target']
 (P/'dice-locks.json').write_text(json.dumps(d,indent=2)+'\n')
