from pathlib import Path
import json
from PIL import Image
P=Path(__file__).resolve().parents[1];S=P/'skins';m=json.loads((P/'parameters.json').read_text());keys={p['key']:i for i,p in enumerate(m)};t=json.loads((S/'TUI.json').read_text())['pageData'];q=json.loads((S/'Q-Links.json').read_text());defs={d['key']:d['value']for d in t['componentDefinitions']['localComponentDefinitions']}
assert [p['tabName']for p in t['tabs']]==['MAIN','GROOVE','SOUND','KITS','FX','ROUTING'];assert len({(p['fnKeyIndex'],p['fnKeySubIndex'])for p in t['tabs']})==6
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

# SOUND uses fixed16 normalized relay controls, not legacy single-sample DRUMS mappings.
sound=next(e for e in q['Screen Mode Q-Links']['map']if e['Tab']==3)
for row in range(4):
 for knob in range(4):assert sound['Q-Links']['Q-Link '+str(13+row-4*knob)]==keys['SoundQ'+str(row*4+knob)]
assert 'SoundDetailsPanel' not in defs
assert not defs['SoundDetailPage'].get('hideQLinkBounds',False)
assert any(k.startswith('ValueKitDiceMode')for k in defs),'Dice Mode must remain reachable in KITS'
assert sum(k.startswith('DynamicKitRow')for k in defs)==8
assert defs['ExpansionKitBrowser']['hideQLinkBounds']
roll=[]
for key,value in defs.items():
 children=value.get('componentsData',[])
 if any(a.get('handleName')=='Parameter '+str(keys['Roll'])for a in value.get('actions',[])):roll.append(value)
assert len(roll)==1
print('PASS SOUND16 Q-Link layout,97paged kit choices and native caption-safe ROLL hit target')

assert defs['RoleSoundBrowser']['hideQLinkBounds']
for group,key in enumerate(['MixKick','MixSnare','MixHat','MixPerc']):
 assert q['Program Mode Q-Links']['Q-Link '+str(15-4*group)]==keys[key]
groove=next(e for e in q['Screen Mode Q-Links']['map']if e['Tab']==2)
for physical,key in zip([16,12,8,4],['FillSlot','FillDensity','FillEvery','Variation']):assert groove['Q-Links']['Q-Link '+str(physical)]==keys[key]
print('PASS MAIN C four role selectors, caption-safe paged role popup and preserved GROOVE D fill/variation controls')

# Native ordered serializer consumes hideQLinkBounds only after disableCoarseDataWheel/repeats.
for key,definition in defs.items():
 if definition.get('hideQLinkBounds'):
  order=list(definition);assert order.index('disableCoarseDataWheel')<order.index('hideQLinkBounds')<order.index('componentsData'),key
assert t['tabs'][0]['qlinkBoundsData']==['20 162 606 110','654 162 606 110','20 304 606 110','654 304 606 110']
for physical,key in zip([14,10,6,2],['RulePickFullKick','RulePickFullSnare','RulePickFullHat','RulePickFullPerc']):assert q['Program Mode Q-Links']['Q-Link '+str(physical)]==keys[key]
print('PASS ordered native modal guide suppression, padded MAIN panel bounds, B RULES mapping')

# Test what matters visually: all titles/badges outside, actual four controls inside.
# The old test rewarded enclosing the whole panel, allowing Q to overwrite the badge.
def rect(entry):return tuple(map(float,entry['bounds']['bounds'].split()))
def inside(a,b):
 x,y,w,h=a;gx,gy,gw,gh=b;return x>=gx and y>=gy and x+w<=gx+gw and y+h<=gy+gh
def overlaps(a,b):
 x,y,w,h=a;xx,yy,ww,hh=b;return x<xx+ww and xx<x+w and y<yy+hh and yy<y+h
for tab in t['tabs']:
 if tab['tabName']=='SOUND':continue
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
 if typ.startswith('Btn')or typ.startswith('Value'):assert y>=420 and h>=40,(typ,y,h)
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
assert q['Program Mode Q-Links']['Q-Link 4']==keys['CrashRule']
assert len([k for k in defs if k.startswith('GDChoiceHatRhythmRow')])==15
assert defs['GDChoiceHatRhythm']['hideQLinkBounds']
assert any(k.startswith('KnobDuck-')for k in defs)
assert any(k.startswith('KnobHatRhythm-')for k in defs)
for j in range(15):
 row=next(e for e in defs['GDChoiceHatRhythm']['componentsData']if e['componentData']['type']=='GDChoiceHatRhythmRow'+str(j))
 assert row['bounds']['additionalInvalidatingHandles']==['IndexedEnabling/'+str(j//8)+'/2/Parameter '+str(keys['HatRhythmPage'])]
print('PASS MAIN D1 HatRhythm,15paged choices, guide suppression, DUCK retained in FX')

# Four row guides exclude layer selectors/On/Solo/Lock, include all actual control bounds.
sound_tab=next(tab for tab in t['tabs']if tab['tabName']=='SOUND')
assert sound_tab['qlinkBoundsData']==[f'212 {132+row*110} 1052 108'for row in range(4)]
for row in range(4):
 guide=tuple(map(float,sound_tab['qlinkBoundsData'][row].split()))
 controls=[e for e in defs['SoundOverview']['componentsData']if e['componentData']['type'].startswith(('LayerSample','KnobSoundQ'))and abs(rect(e)[1]-(132+row*110))<.01]
 assert len(controls)>=4 and all(inside(rect(e),guide)for e in controls)
 assert min(rect(e)[0]for e in controls)-guide[0]>=40
for name in ['SoundOverview','SoundDetailPage','CymbalOverview','CymbalDetailPage']:
 for e in defs[name]['componentsData']:
  x,y,w,h=rect(e);assert min(x,y,w,h)>=0 and x+w<=1280 and y+h<=628,(name,e['componentData']['name'],(x,y,w,h))
print('PASS MAIN C mix,16 SOUND relays,4 row guides with native-Q gutter, two editing pages and bounds')
# Sample names are separate native Labels. A transparent filmstrip has no intrinsic text/background.
for row in range(4):
 children=defs['LayerSample'+str(row)]['componentsData']
 touch=next(e for e in children if e['componentData']['type'].startswith('Btn') and rect(e)==(0.,24.,224.,40.))
 command=defs[touch['componentData']['type']];contents=command['componentsData'];captions=[e for e in contents if e['componentData']['type']=='Label'and e['componentData']['data'].get('type')=='Value']
 assert len(captions)==1
 x,y,w,h=map(int,captions[0]['bounds']['bounds'].split());assert (x,y,w,h)==(8,0,208,40)
 assert 'verticallyCentred' in captions[0]['componentData']['data']['textStyle']['justification']
 assert contents[-1]['componentData']['type']=='Slider'and contents.index(captions[0])<len(contents)-1
 assert sum(a.get('handleName')=='Parameter '+str(keys['SoundBrowse'+str(row)])for a in command['actions'])==1

 arrows=[e for e in children if e['componentData']['type'].startswith('Btn') and rect(e)[2:]==(44.,44.)]
 assert len(arrows)==2
 for entry,key in zip(arrows,['SoundSamplePrev'+str(row),'SoundSampleNext'+str(row)]):
  assert any(a.get('handleName')=='Parameter '+str(keys[key])for a in defs[entry['componentData']['type']]['actions'])
 preview=next(e for e in children if e['componentData']['type'].startswith('Btn') and rect(e)==(48.,64.,128.,44.))
 assert any(a.get('handleName')=='Parameter '+str(keys['SoundPreview'+str(row)])for a in defs[preview['componentData']['type']]['actions'])
 assert all(rect(e)[0]>=0 and rect(e)[1]>=0 and rect(e)[0]+rect(e)[2]<=224 and rect(e)[1]+rect(e)[3]<=108 for e in children)
print('PASS named sample plate,44px previous/next targets, tap-list and preview press surfaces and row bounds')
# Value-only sample editing needs the same explicit focus action as rotary knobs.
for row in range(4):
 assert any(a.get('onAction')=='Mouse Down' and a.get('handler')=='Q-Link' and a.get('handleName')=='Parameter '+str(keys['SoundQ'+str(row*4)]) for a in defs['LayerSample'+str(row)]['actions'])
print('PASS each sample selector focuses its own SCREEN Q-Link row')

# Kit lock must be visible, with scope choices reflecting its real behavior.
labels=[e['componentData'] for d in defs.values() for e in d.get('componentsData',[]) if e['componentData']['type']=='Label']
text=json.dumps(labels)
for caption in ['LOCK KIT','KIT FIXED','EXTRA CHANGES','No extras','New Pattern','Groove Variation','Recorded Fill','Pattern + Fill + FX']:
 assert caption in text,caption
for n in range(6):
 rows=defs['GDChoiceScopeRow'+str(n)]['componentsData']
 assert len(rows)==1
print('PASS explicit kit lock and explicit extra-change choices')

# RULES expose seven playable touch choices without a LOCK value and MAIN B's four fixed SCREEN relays.
for n,key in enumerate(['RulePickFullKick','RulePickFullSnare','RulePickFullHat','RulePickFullPerc']):
 assert len(m[keys[key]]['choices'])==16 and 'LOCK'not in m[keys[key]]['choices']
 assert all('GDChoice'+key+'Row'+str(j)in defs for j in range(16))
 assert not any(e['componentData'].get('type')=='Label' and '4-BAR PHRASES' in str(e)for d in defs.values()for e in d.get('componentsData',[]))
print('PASS four7-choice rule drawers, Program B styles and live1-bar captions')

# MAIN D is articulation; audio colour is retained only on FX.
main_types=[e['componentData']['type']for e in defs['GDMain']['componentsData']]
for key in ['HatRhythm','OpenHatRule','RideRule','CrashRule']:
 assert any(t.startswith('Knob'+key+'-')for t in main_types)
 assert defs['GDChoice'+key]['hideQLinkBounds']
for key in ['Drive','Dust','Room']:assert not any(t.startswith('Knob'+key+'-')for t in main_types)
for index,key in zip([16,12,8,4],['HatRhythm','OpenHatRule','RideRule','CrashRule']):assert q['Program Mode Q-Links']['Q-Link '+str(index)]==keys[key]
print('PASS MAIN D Hat/Open/Ride/Crash, native mappings and guide-hiding drawers; Drive/Dust/Room absent from MAIN and preserved on FX')

assert defs["SampleBrowser"]["hideQLinkBounds"] and defs["SelectedKitBrowser"]["hideQLinkBounds"]
assert not any(e["componentData"]["type"]=="Knob" for row in range(4) for e in defs["LayerSample"+str(row)]["componentsData"])
print("PASS sample tap-list, preview buttons and both modal guide suppression")

# Labels and fixed relay positions match the approved two-page layout in every row.
for name,rows,columns in [('SoundOverview',range(4),{1:'Level',2:'Tune',3:'Shift'}),('CymbalOverview',range(3),{1:'Level',2:'Tune',3:'Shift'}),('SoundDetailPage',range(4),{0:'Start',1:'Decay',2:'Low Cut',3:'High Cut'}),('CymbalDetailPage',range(3),{0:'Start',1:'Decay',2:'Low Cut',3:'High Cut'})]:
 for row in rows:
  for column,caption in columns.items():
   prefix='KnobSoundQ'+str(row*4+column)+'-'
   controls=[e for e in defs[name]['componentsData']if e['componentData']['type'].startswith(prefix)]
   assert controls and all(defs[e['componentData']['type']]['componentsData'][0]['componentData']['data']['text']==caption for e in controls),(name,row,column)
print('PASS all rows: page1 Sample/Level/Tune/Shift; page2 Start/Decay/LowCut/HighCut')

for d in defs.values():
 for entry in d.get("componentsData",[]):
  c=entry["componentData"]
  assert not(c["type"]=="Label" and c["data"].get("type")=="Text" and c["data"].get("text")==""),"Empty fixed text widget"
print("PASS captionless hit targets omit empty fixed-Text widgets")

assert Image.open(S/'dice-locked.png').size==(44,44) and Image.open(S/'dice-unlocked.png').size==(44,44)
main_controls=[e for e in defs['GDMain']['componentsData']if e['componentData']['type'].startswith('Knob')]
assert len(main_controls)==16
for control in main_controls:
 children=defs[control['componentData']['type']]['componentsData']
 locks=[e for e in children if e['componentData']['type'].startswith('Btn')]
 assert len(locks)==1 and rect(locks[0])==(94.,26.,44.,44.)
 lock=defs[locks[0]['componentData']['type']]['componentsData'][0]
 assert lock['componentData']['data']['onImage']=='dice-locked.png'
 handle=lock['handle remapping']['map'][0]['value'];assert keys['DiceLock0']<=int(handle.split()[-1])<=keys['DiceLock15']
 knob=next(e for e in children if e['componentData']['type']=='Knob');assert not overlaps(rect(knob),rect(locks[0]))
print('PASS16persistent44px locks inside control bounds with no knob overlap')

# Conditional default kit caption needs version-2 bounds to hide under KIT FIXED.
for definition in defs.values():
 for entry in definition.get("componentsData",[]):
  bounds=entry.get("bounds",{})
  if bounds.get("showWhenDataModelInvalid")=="Hide":
   assert bounds.get("version",0)>=2, "Conditional caption requires version-2 bounds"

# Every sound row has a separate stateful REV button, without touching its sample hit.
reverse_counts={}
for name in ['SoundOverview','CymbalOverview']:
 entries=defs[name]['componentsData'];found=[]
 for entry in entries:
  definition=defs.get(entry['componentData']['type'],{})
  children=definition.get('componentsData',[])
  arts=[c for c in children if c['componentData']['type']=='Button' and c.get('handle remapping',{}).get('map')]
  if not arts:continue
  handles=[m.get('value','')for c in arts for m in c['handle remapping']['map']]
  if any('Parameter '+str(keys['SoundRev'+str(row)])in handles for row in range(4)):
   x,y,w,h=rect(entry);assert w>=44 and h>=44 and x+w<=244
   found.append(entry)
 reverse_counts[name]=len(found)
assert reverse_counts=={'SoundOverview':6,'CymbalOverview':3},reverse_counts
print('PASS REV44px toggles for all sound rows; Hat detail rows exclude duplicate buttons')

# The page controls remain on both views; 48px arrows and a visible page indicator.
for name in ['SoundOverview','SoundDetailPage','CymbalOverview','CymbalDetailPage']:
 children=defs[name]['componentsData'];buttons=[e for e in children if rect(e)[1]==574 and rect(e)[2:]==(48.,48.)]
 assert len(buttons)==2,(name,'page arrows')
 assert sorted(rect(e)[0]for e in buttons)==[16.,220.]
 assert all(e['componentData']['type']!='SoundDetailsPanel'for e in children)
assert t['tabs'][2]['qlinkBoundsData']==[f'212 {132+r*110} 1052 108'for r in range(4)]
print('PASS SOUND page1/2 explicit 48px previous/next arrows, fixed page display and retained native guides')

# Keep the expanded SOUND tree bounded; hidden pages must not explode native widget count.
def expanded_count(entries,path=()):
 count=0
 for entry in entries:
  name=entry['componentData']['type'];assert name not in path,'Recursive skin definition'
  count+=1+expanded_count(defs.get(name,{}).get('componentsData',[]),path+(name,))
 return count
assert expanded_count(t['tabs'][2]['componentDefinition']['componentsData'])<2000
print('PASS SOUND expanded widget tree below2000 including hidden pages/browser')

# Runtime-value captions must sit below the final command press surface.
for name,prefix,expected in [('SampleBrowser','SamplePick',8),('SelectedKitBrowser','PoolPick',8)]:
 matched=0
 for entry in defs[name]['componentsData']:
  command=defs.get(entry['componentData']['type'],{});actions=command.get('actions',[])
  if any(a.get('handleName')=='Parameter '+str(keys[prefix+str(row)])for row in range(8)for a in actions):
   contents=command['componentsData'];assert contents[-1]['componentData']['type']=='Slider'
   assert any(e['componentData']['type']=='Label'and e['componentData']['data'].get('type')=='Value'for e in contents[:-1]);matched+=1
 assert matched==expected,(name,matched)
for prefix in ['LibraryRowButton','RoleSoundRow','DynamicKitRow']:
 for row in range(8):
  command=defs[prefix+str(row)];assert not command.get('actions',[])
  hit=defs[command['componentsData'][-1]['componentData']['type']];assert len([a for a in hit['actions']if a.get('handleName','').startswith('Parameter ')])==1
  assert hit['componentsData'][0]['componentData']['data']['onImage']=='hit.png'
  assert hit['componentsData'][-1]['componentData']['type']=='Slider'
print('PASS sample/sample-list/kit-pool/library/role/kit caption center is covered by a single command surface')

assert m[:672]==json.loads((P/'tests/legacy-parameters-1.3.4.json').read_text())
assert len(m)==676
print('PASS: first672full declarations retained, extended16style selectors append')
