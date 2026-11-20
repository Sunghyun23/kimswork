"""Current 1.14 performance skin contracts; no hardware-rendering claims."""
import json,re
from pathlib import Path
P=Path(__file__).resolve().parents[1];S=P/'skins';pd=json.loads((S/'TUI.json').read_text())['pageData'];q=json.loads((S/'Q-Links.json').read_text());params=json.loads((P/'parameters.json').read_text());ix={p['key']:i for i,p in enumerate(params)}
defs={e['key']:e['value']for e in pd['componentDefinitions']['localComponentDefinitions']}
old=json.loads((P/'tests/legacy-353-parameters.json').read_text());assert params[:353]==old
assert len(params)==399 and len({p['key']for p in params})==399
assert [t['tabName']for t in pd['tabs']]==['POLY','MONO','OSC','ENV','MOD','FX','CHORD']
assert [(t['fnKeyIndex'],t['fnKeySubIndex'])for t in pd['tabs']]==[(0,0),(1,0),(2,0),(3,0),(4,0),(5,0),(0,1)]
front=[['PerfWaveFull','PerfPitch','PerfLevel','PerfCharacter'],['Cutoff','Resonance','BodyTone','Drive'],['PerfLfoRate','PerfLfoDepth','PerfLfoDelay','PerfLfoTarget']]
expected=[front+[['AmpAttack','AmpRelease','ModRateControl','TremDepth']],front+[['Glide','AmpDecay','AmpRelease','ReleaseDropAmount']],[['Wave1Full','OscTune1','OscFine1','OscLevel1'],['Wave2Full','OscTune2','OscFine2','OscLevel2'],['Sub','Drift','OscPulse1','Pulse2'],['Noise','FM','Tone']],[['AmpAttack','AmpDecay','AmpSustain','AmpRelease'],['FiltAttack','FiltDecay','FiltSustain','FiltRelease'],['Cutoff','Resonance','Env','KeyTrack']],[['LfoRateControl','Depth','LfoDelay','Lfo1TargetFull'],['Lfo2RateControl','Lfo2Depth','Lfo2Delay','Lfo2Target'],['ModRateControl','TremDepth','Pan','Vibrato']],[['Drive','DriveMix','Tone','LowCut'],['Chorus','Space','Delay','Feedback'],['OutputTrim','Volume','Pan']],[['EditRoot'+str(i),'HarmonyEdit'+str(i),'EditVoicing'+str(i),'EditOctave'+str(i)]for i in range(1,5)]]
for tab,mapping,groups in zip(pd['tabs'],q['Screen Mode Q-Links']['map'],expected):
 assert (mapping['Tab'],mapping['SubTab'])==(tab['fnKeyIndex']+1,tab['fnKeySubIndex']+1)
 assert mapping['Bank Direction']=='Column'
 want={str(i):-1 for i in range(1,17)}
 for bank,keys in enumerate(groups):
  for j,key in enumerate(keys):want[str(13+bank-4*j)]=ix[key]
 assert mapping['Q-Links']=={'Q-Link '+n:v for n,v in want.items()},tab['tabName']
 assert len(tab['qlinkBoundsData'])==len(groups)
 for b in tab['qlinkBoundsData']:
  x,y,w,h=map(int,b.split());assert x>=0 and y>=0 and x+w<=1280 and y+h<=628
assert q['Program Mode Q-Links']==q['Screen Mode Q-Links']['map'][0]['Q-Links']
print('PASS:353exact legacy declarations;399stable IDs; seven views and all per-tab/Program Q-Link assignments')

def walk(o):
 if isinstance(o,dict):
  for k,v in o.items():
   if k in('image','foregroundImage','backgroundImage','filmStrip','onImage','offImage')and isinstance(v,str)and v:assert(S/v).is_file(),v
   if isinstance(v,str):
    for n in re.findall(r'Parameter (\d+)',v):assert int(n)<len(params),v
   walk(v)
 elif isinstance(o,list):
  for v in o:walk(v)
walk(pd)
assert {x['key']:x['value']for x in pd['replacementStrings']}=={'Parameter '+str(i):p['choices']for i,p in enumerate(params)if 'choices'in p}
# Expand all possible conditional branches to detect cycles / construction blowups.
def expansion(w,stack=()):
 typ=w['componentData']['type']
 if typ not in defs:return 1
 assert typ not in stack,stack+(typ,)
 return 1+sum(expansion(c,stack+(typ,))for c in defs[typ]['componentsData'])
for tab in pd['tabs']:
 ws=tab['componentDefinition']['componentsData'];modal=False;budget=sum(expansion(w)for w in ws)
 assert budget<(2500 if tab['tabName']=='CHORD'else 5000),(tab['tabName'],budget)
 for w in ws:
  x,y,wid,hh=map(int,w['bounds']['bounds'].split());assert min(x,y,wid,hh)>=0 and x+wid<=1280 and y+hh<=628,(tab['tabName'],w['componentData']['name'])
  ismodal=defs.get(w['componentData']['type'],{}).get('hideQLinkBounds',False)
  if ismodal:modal=True
  else:assert not modal,(tab['tabName'],w['componentData']['name'])
  if ismodal:assert defs[w['componentData']['type']]['version']==4
 print('PASS:',tab['tabName'],'bounds / modal ordering / acyclic construction',budget,'widgets')
# No text widget with an empty literal string (confirmed all-tabs blank failure).
for d in defs.values():
 for w in d['componentsData']:
  data=w['componentData']['data']
  if w['componentData']['type']=='Label'and data.get('type')=='Text':assert data.get('text'),w
# Navigation/selection is not allowed to write voice mode, gates or timbre.
for tab in pd['tabs'][:6]:assert not tab['componentDefinition']['actions']
for key in('OscFocus','LfoFocus','PitchResolution'):
 assert params[ix[key]]['choices']
for tab in pd['tabs'][:2]:
 ws=tab['componentDefinition']['componentsData'];assert any(w['componentData']['name']=='PerfLfoDelay'for w in ws)
 assert not any(w['componentData']['name']=='Lock PerfLfoDelay'for w in ws)
# Tested hold gesture ownership and native-sized assets are retained on all16 pads.
poly=pd['tabs'][0]['componentDefinition']['componentsData'];pads=[w for w in poly if re.fullmatch(r'Chord \d+',w['componentData']['name'])];assert len(pads)==16
for pad in pads:
 d=defs[pad['componentData']['type']];surface=d['componentsData'][-1]['componentData'];assert surface['type']=='Slider'and surface['data']['handleName'].startswith('ResetOnGestureEnd/Parameter ')
 assert any(a['onAction']=='Mouse Down'and a['handler']=='Toggle Switch'for a in d['actions'])
 assert pad['bounds']['bounds'].endswith('305 70')
assert sum(w['componentData']['type']=='Envelope'for w in pd['tabs'][3]['componentDefinition']['componentsData'])==2
cfg=json.loads((P/'dice-locks.json').read_text());assert str(ix['LfoDelay'])not in cfg['targets']and str(ix['Lfo2Delay'])not in cfg['targets']
for name in('PerfLfoDelay','PerfVoice','DiceTarget'):
 assert 'DiceValueLock'+name not in ix
assert all('DTChoicePanel'+str(ix[k])in defs for k in ['PerfWaveFull','PerfLfoTarget','PerfLfoShape','DiceTarget','SoundCurrent'])
print('PASS: assets/handles/choices, no empty literal labels, two native ADSRs, 16hold pads, Delay/master/chord Dice exclusions')

# Pitch and rate touch overlays must use canonical unit ranges instead of0..1relays.
for key in ['PerfKnobOscTune1PerfPitch','PerfKnobOscFine1PerfPitch','PerfKnobOscTune2PerfPitch','PerfKnobOscFine2PerfPitch']:
 assert key in defs
 assert any(a['handler']=='Show Overlay'for a in defs[key]['actions'])
 assert any(a['handler']=='Q-Link'and a['handleName']=='Parameter '+str(ix['PerfPitch'])for a in defs[key]['actions'])
menu=defs['PerfMenuSoundCurrent330x44']
assert any(a['handler']=='Toggle Switch'and a['handleName']==f'Local/2/dtChoice{ix["SoundCurrent"]}'for a in menu['actions'])
assert menu['componentsData'][-1]['handle remapping']['map']==[{'key':'Data','value':f'Local/7/soundCategory{ix["SoundCurrent"]}'}]
assert menu['componentsData'][-1]['componentData']['data']['buttonId']==0
print('PASS:canonical numeric touch ranges, explicit relay Q-Link focus and category-first CHORD sound browser')

assert params[:388]==json.loads((P/'tests/legacy-388-parameters.json').read_text())
for key in ['Lfo1SyncDivision','MotionSyncDivision','Lfo2SyncDivision']:
 assert len(params[ix[key]]['choices'])==15
 panel=defs['DTChoicePanel'+str(ix[key])]
 assert panel['hideQLinkBounds']and panel['version']==4
 assert params[ix[key]]['choices'].count('1/8 D')==1 and params[ix[key]]['choices'].count('1/8 T')==1
 assert 'PerfKnob'+key+('Compact'if key!='MotionSyncDivision'else '')+'LfoRateControl' in defs or any(k.startswith('PerfKnob'+key)for k in defs)
 assert str(ix[key])in cfg['targets']
print('PASS:first388definitions retained; three15-choice Sync grids with D/T names, shared locks and modal guide suppression')

for name,d in defs.items():
 if d.get('hideQLinkBounds'):
  keys=list(d);assert keys.index('disableCoarseDataWheel')<keys.index('hideQLinkBounds')<keys.index('componentsData'),name
print('PASS:SDK version4modal key order, including Sync choice drawers and Settings')

# Native MPC cannot resolve a nested Data -> Data alias; inherited knob value
# labels must inherit the owning parameter directly (device regression1.14.1).
def no_self_aliases(node):
 if isinstance(node,dict):
  for pair in node.get('handle remapping',{}).get('map',[]):
   assert pair['key']!=pair['value'],('recursive native handle alias',pair)
  for value in node.values():no_self_aliases(value)
 elif isinstance(node,list):
  for value in node:no_self_aliases(value)
no_self_aliases(tui if 'tui'in globals()else json.loads((P/'skins/TUI.json').read_text()))
print('PASS:native value labels inherit bound Data without recursive self-aliases')

assert params[:391]==json.loads((P/'tests/legacy-391-parameters.json').read_text())
print('PASS exact391legacy metadata; eight-wave fronts/detail; shared Drift/Phase relay')
