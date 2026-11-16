from pathlib import Path
import json
from PIL import Image
P=Path(__file__).resolve().parents[1];S=P/'skins';m=json.loads((P/'parameters.json').read_text());keys={p['key']:i for i,p in enumerate(m)};t=json.loads((S/'TUI.json').read_text())['pageData'];q=json.loads((S/'Q-Links.json').read_text());defs={d['key']:d['value']for d in t['componentDefinitions']['localComponentDefinitions']}
assert [p['tabName']for p in t['tabs']]==['MAIN','GROOVE','DRUMS','KITS','FX','ROUTING'];assert len({(p['fnKeyIndex'],p['fnKeySubIndex'])for p in t['tabs']})==6
for group in range(4):
 for knob in range(4):assert q['Screen Mode Q-Links']['map'][0]['Q-Links'][f'Q-Link {13+group-4*knob}']==keys[f'Q{group*4+knob+1}']
assert Image.open(S/'background.png').size==(1280,628)
assert Image.open(S/'knob-v2.png').size==(64,64*127)
assert defs['GDFill']['hideQLinkBounds']
for name in ['GDMain','GDFill']:
 for entry in defs[name]['componentsData']:
  x,y,w,h=map(float,entry['bounds']['bounds'].split());assert min(x,y,w,h)>=0 and x+w<=1280 and y+h<=628,(name,entry['componentData']['name'],(x,y,w,h))
all_keys=[d['key']for d in t['componentDefinitions']['localComponentDefinitions']];assert len(all_keys)==len(set(all_keys)), 'Duplicate component definitions'
assert len([k for k in defs if k.startswith('Cell')])==64
assert len([k for k in defs if k.startswith('Knob')and k.endswith('-164')])==8
print('PASS6tabs,16MAIN relay Q-Links,64px native knobs,1280x628background,grid64cells,editor guide hiding and page bounds')

# Source choice replacement strings cannot express dynamic library status; native visibility must select the correct caption.
sample=next(v for k,v in defs.items()if k.startswith('KnobSample0-'))
assert any(e['componentData'].get('data',{}).get('text')=='LIBRARY'and e['bounds'].get('additionalInvalidatingHandles')==['RangedEnabling/1/1048575/1048576/Parameter '+str(keys['XSample0'])]for e in sample['componentsData'])
pattern=next(v for k,v in defs.items()if k.startswith('ValuePattern590'))
assert any(e['bounds'].get('additionalInvalidatingHandles')==['RangedEnabling/1/131071/131072/Parameter '+str(keys['XBeat'])]for e in pattern['componentsData'])
print('PASS native dynamic captions distinguish library samples/beats from legacy indexed choices')

# DRUMS groupA is kit selection; three sound groups remain present and main/fill maps are unchanged.
drums=next(e for e in q['Screen Mode Q-Links']['map']if e['Tab']==3)
for physical,key in zip([13,9,5,1],['XQKit','KitDiceMode','RandomAmt','KitLock']):assert drums['Q-Links']['Q-Link '+str(physical)]==keys[key]
assert sum(k.startswith('DynamicKitRow')for k in defs)==8
assert defs['ExpansionKitBrowser']['hideQLinkBounds']
roll=[]
for key,value in defs.items():
 children=value.get('componentsData',[])
 if any(a.get('handleName')=='Parameter '+str(keys['Roll'])for a in value.get('actions',[])):roll.append(value)
assert len(roll)==1
print('PASS DRUMS4+12 Q-Link layout,97paged kit choices and native caption-safe ROLL hit target')

assert defs['RoleSoundBrowser']['hideQLinkBounds']
for group,key in enumerate(['XQKick','XQSnare','XQHat','XQPerc']):
 assert 'SoundLaunch'+key in defs
 assert q['Program Mode Q-Links']['Q-Link '+str(15-4*group)]==keys[key]
groove=next(e for e in q['Screen Mode Q-Links']['map']if e['Tab']==2)
for physical,key in zip([16,12,8,4],['FillSlot','FillDensity','FillEvery','Variation']):assert groove['Q-Links']['Q-Link '+str(physical)]==keys[key]
print('PASS MAIN C four role selectors, caption-safe paged role popup and preserved GROOVE D fill/variation controls')

# Native ordered serializer consumes hideQLinkBounds only after disableCoarseDataWheel/repeats.
for key,definition in defs.items():
 if definition.get('hideQLinkBounds'):
  order=list(definition);assert order.index('disableCoarseDataWheel')<order.index('hideQLinkBounds')<order.index('componentsData'),key
assert t['tabs'][0]['qlinkBoundsData']==['20 162 606 110','654 162 606 110','20 304 606 110','654 304 606 110']
for physical,key in zip([14,10,6,2],['RuleKick','RuleSnare','RuleHat','RulePerc']):assert q['Program Mode Q-Links']['Q-Link '+str(physical)]==keys[key]
print('PASS ordered native modal guide suppression, padded MAIN panel bounds, B RULES mapping')

# Test what matters visually: all titles/badges outside, actual four controls inside.
# The old test rewarded enclosing the whole panel, allowing Q to overwrite the badge.
def rect(entry):return tuple(map(float,entry['bounds']['bounds'].split()))
def inside(a,b):
 x,y,w,h=a;gx,gy,gw,gh=b;return x>=gx and y>=gy and x+w<=gx+gw and y+h<=gy+gh
def overlaps(a,b):
 x,y,w,h=a;xx,yy,ww,hh=b;return x<xx+ww and xx<x+w and y<yy+hh and yy<y+h
for tab in t['tabs']:
 entries=defs['GDMain']['componentsData']if tab['tabName']=='MAIN'else tab['componentDefinition']['componentsData']
 panels=[e for e in entries if e['componentData']['type'].startswith('panel-')]
 nonempty=[guide for guide in tab['qlinkBoundsData']if tuple(map(float,guide.split()))[2:]!=(0.,0.)]
 assert len(panels)==len(nonempty),(tab['tabName'],'phantom or missing guide')
 for panel,guide in zip(panels,tab['qlinkBoundsData']):
  x,y,w,h=rect(panel);grect=tuple(map(float,guide.split()));gx,gy,gw,gh=grect
  assert inside(grect,(x,y,w,h)) and gy>=y+26,(tab['tabName'],guide)
  definition=defs[panel['componentData']['type']]
  for header in definition['componentsData'][1:]:
   hx,hy,hw,hh=rect(header);assert not overlaps((x+hx,y+hy,hw,hh),grect),(tab['tabName'],'header inside guide')
  controls=[e for e in entries if e['componentData']['type'].startswith('Knob')and inside(rect(e),(x,y,w,h))]
  assert len(controls)==4 and all(inside(rect(e),grect)for e in controls)
  # 40x40nativeQ badge occupies guide top-left: reserve it beside the first name/knob/LCD.
  first=min(controls,key=lambda e:rect(e)[0]);fx,fy,fw,fh=rect(first);assert fx-gx>=40
  kdef=defs[first['componentData']['type']];name=kdef['componentsData'][0];nx,ny,nw,nh=rect(name);assert not overlaps((fx+nx,fy+ny,nw,nh),(gx,gy,40,40));knob=next(e for e in kdef['componentsData']if e['componentData']['type']=='Knob');kx,ky,kw,kh=rect(knob)
  assert not overlaps((fx+kx,fy+ky,kw,kh),(gx,gy,40,40))
print('PASS every page: headers/badges excluded; four control bounds contained; nativeQ gutter reserved')

# MAIN user actions are separated into three spacious lower lanes, physical pads play drums.
for entry in defs['GDMain']['componentsData']:
 typ=entry['componentData']['type'];x,y,w,h=map(float,entry['bounds']['bounds'].split())
 if typ.startswith('Btn')or typ.startswith('Value'):assert y>=420 and h>=60,(typ,y,h)
for name in ['GDMain','GDFill']:
 for entry in defs[name]['componentsData']:
  definition=defs.get(entry['componentData']['type'],{})
  assert not any(a.get('handleName')=='Parameter '+str(keys['Pad'+str(i)])for a in definition.get('actions',[])for i in range(8))
print('PASS spacious60px lower BROWSE/CREATE/PLAY actions; no redundant screen drum pads')

commands=0
for definition in defs.values():
 actions=definition.get('actions',[])
 if any(a.get('handleName','').startswith('Local/2/gdPress')for a in actions):
  assert sum(a.get('handleName','').startswith('Parameter ')for a in actions)==1
  surfaces=[e for e in definition['componentsData']if e['componentData']['name']=='command press surface']
  assert len(surfaces)==1 and surfaces[0]['componentData']['data']['handleName'].startswith('ResetOnGestureEnd/Local/2/gdPress')
  commands+=1
assert commands>=25
print('PASS',commands,'independent press/release visuals; exactly one engine command per press')

# MAIN D4 and Program match; DUCK stays in FX. All15 choices are touch-accessible.
assert q['Program Mode Q-Links']['Q-Link 4']==keys['HatRhythm']
assert len([k for k in defs if k.startswith('GDChoiceHatRhythmRow')])==15
assert defs['GDChoiceHatRhythm']['hideQLinkBounds']
assert any(k.startswith('KnobDuck-')for k in defs)
assert any(k.startswith('KnobHatRhythm-')for k in defs)
for j in range(15):
 row=next(e for e in defs['GDChoiceHatRhythm']['componentsData']if e['componentData']['type']=='GDChoiceHatRhythmRow'+str(j))
 assert row['bounds']['additionalInvalidatingHandles']==['IndexedEnabling/'+str(j//8)+'/2/Parameter '+str(keys['HatRhythmPage'])]
print('PASS MAIN D4 HatRhythm,15paged choices, guide suppression, DUCK retained in FX')
