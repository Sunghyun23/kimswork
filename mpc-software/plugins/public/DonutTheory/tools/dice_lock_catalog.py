"""Canonical ownership for Dice-only locks. Relays/compound rates share a lock."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
TARGETS='AmpAttack AmpDecay AmpRelease AmpSustain BodyTone Chorus Cutoff Delay DelayDivision Depth Detune Division Drift Drive DriveMix DriveType Env Expression FM FXPreset Feedback FiltAttack FiltDecay FiltRelease FiltSustain Filter Glide GlideLegato KeyTrack Lfo2Depth Lfo2Division Lfo2Rate Lfo2Shape Lfo2Sync Lfo2Target LowCut Mix ModDivision ModMode ModShape ModSync Motion Noise Octave OscPair OutputTrim Pan Pulse Rate Resonance Shape Space Sub Sync Target Tone TremDepth TremRate Vibrato VoiceMode Wave1 Wave2'.split()
ALIASES={k:'OscPair'for k in ['Wave1','Wave2']}
for owner,keys in [('LfoRateControl',['Rate','Sync','Division']),('Lfo2RateControl',['Lfo2Rate','Lfo2Sync','Lfo2Division']),('ModRateControl',['TremRate','ModSync','ModDivision'])]:ALIASES.update({k:owner for k in keys})
def append(params):
 lookup={p['key']:i for i,p in enumerate(params)};owners=sorted({ALIASES.get(k,k)for k in TARGETS},key=lookup.get)
 first=len(params)
 for key in owners:params.append(dict(key='DiceValueLock'+key,name='Dice Lock '+params[lookup[key]]['name'],choices=['Unlocked','Locked'],default=0))
 params.append(dict(key='UnlockDice',name='Unlock All Dice',choices=['Off','On'],default=0))
 locks={owner:first+i for i,owner in enumerate(owners)}
 mapping={lookup[k]:locks[ALIASES.get(k,k)]for k in set(TARGETS)|set(owners)}
 (ROOT/'dice-locks.json').write_text(json.dumps(dict(owners=owners,first=first,locks=locks,targets={str(k):v for k,v in mapping.items()},unlock=len(params)-1),indent=2)+'\n')
def definitions(params):
 d=json.loads((ROOT/'dice-locks.json').read_text());keys={i:p['key']for i,p in enumerate(params)}
 return ['static constexpr int kDiceValueLocks[]={'+','.join('kDiceValueLock'+k for k in d['owners'])+'};','static constexpr int kDiceValueOwners[]={'+','.join('k'+k for k in d['owners'])+'};','static int diceValueLock(int key){switch(key){'+''.join('case k'+keys[int(k)]+':return '+str(v)+';'for k,v in d['targets'].items())+'default:return -1;}}']
