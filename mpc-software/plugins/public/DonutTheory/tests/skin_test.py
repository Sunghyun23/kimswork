import json
from pathlib import Path
P=Path(__file__).resolve().parents[1];skin=P/'skins'
t=json.loads((skin/'TUI.json').read_text())['pageData'];q=json.loads((skin/'Q-Links.json').read_text());params=json.loads((P/'parameters.json').read_text())
index={p['key']:i for i,p in enumerate(params)}
expected=['OscPair','Mix','Sub','Drift','AmpAttack','AmpDecay','AmpSustain','AmpRelease','Cutoff','Resonance','Rate','Depth','Drive','Glide','TremRate','TremDepth']
main=q['Screen Mode Q-Links']['map'][0];assert main['Bank Direction']=='Column'
# SDK hardware banks are columns read top-to-bottom, not Q-Link 1..4.
physical_banks=((13,9,5,1),(14,10,6,2),(15,11,7,3),(16,12,8,4))
assert [main['Q-Links'][f'Q-Link {n}']for bank in physical_banks for n in bank]==[index[k]for k in expected]
assert q['Program Mode Q-Links']==main['Q-Links']
assert len(t['tabs'][0]['qlinkBoundsData'])==4
assert len({(x['fnKeyIndex'],x['fnKeySubIndex'])for x in t['tabs']})==len(t['tabs'])
assert max(x['fnKeyIndex']for x in t['tabs'])==5
children=t['tabs'][0]['componentDefinition']['componentsData']
for c in children:
 x,y,w,h=map(float,c['bounds']['bounds'].split());assert x>=0 and y>=0 and x+w<=1280 and y+h<=628,c['componentData']['name']
assert any(c['componentData']['name']=='CHORD PRESET'for c in children)
def walk(o):
 if isinstance(o,dict):
  for k,v in o.items():
   if k in ('image','foregroundImage','backgroundImage','filmStrip','onImage','offImage')and isinstance(v,str)and v:assert(skin/v).is_file(),v
   walk(v)
 elif isinstance(o,list):
  for v in o:walk(v)
walk(t)
print('PASS: 16 Q-Link bindings, bank geometry, 6 function keys + chord sub-tab, CHORD PRESET label, all image references')

# Device screenshot regressions: artwork shrank twice, unbound titles vanished.
defs={x['key']:x['value'] for x in t['componentDefinitions']['localComponentDefinitions']}
for key in ('DTKnob','DTChoiceKnob'):
 knob=next(c['componentData']['data'] for c in defs[key]['componentsData'] if c['componentData']['type']=='Knob')
 assert knob['imageScale']==1.0
for c in children:
 if c['componentData']['name'].startswith(('A  ','B  ','C  ','D  ')):
  assert c['bounds']['showWhenDataModelInvalid']=='Show'
  assert c['bounds']['acceptsHWFocus']=='No'
for key in ('DTPrev','DTNext'):
 assert defs[key]['componentsData'][0]['componentData']['type']=='Button'
for key in ('DTMenu','DTSmallMenu','DTSelectFilter'):
 assert {'Double Click','Enter Pressed'} <= {a['onAction'] for a in defs[key]['actions']}
print('PASS: full-size rotating artwork, visible titles, native arrow buttons, menu fallback gestures')

assert any(c['componentData']['name']=='Filter / LFO divider' for c in children)
for key,count in [('SoundPreset',64),('HarmonyCurrent',96)]:
 panel=defs['DTChoicePanel'+str(index[key])]
 assert panel['componentsData']
 rows=[defs['DTSideRow'+str(index[key])+'-'+str(i)] for i in range(count)]
 buttons=[c for row in rows for c in row['componentsData'] if c['componentData']['type']=='Button' and not c['componentData']['name'].endswith(' touch')]
 assert len(buttons)==count
 assert [c['componentData']['data']['buttonId'] for c in buttons]==list(range(count))
 assert all(c['componentData']['data']['numButtonsInGroup']==count for c in buttons)
print('PASS: explicit 64 sound / 96 chord choices and Drive/Play separator')

for name,definition in defs.items():
 if any(c['componentData']['type']=='Slider' and c['componentData']['data'].get('handleName','').startswith('ResetOnGestureEnd/') for c in definition['componentsData']):continue # continuous hold surfaces checked below
 widgets=definition['componentsData']
 for i,c in enumerate(widgets):
  if c['componentData']['type']!='Button' or c['componentData']['name'].endswith(' touch') or c['componentData']['data']['onImage']=='button-hit.png':continue
  hits=[h for h in widgets[i+1:] if h['componentData']['name']==c['componentData']['name']+' touch' and h['bounds']['bounds']==c['bounds']['bounds']]
  assert hits,c['componentData']['name']
  assert hits[0]['handle remapping']==c['handle remapping']
  assert hits[0]['componentData']['data']['onImage']=='button-hit.png'
print('PASS: button hit surfaces above captions, with identical bounds and bindings')

replacements={r['key']:r['value'] for r in t['replacementStrings']}
assert replacements=={'Parameter '+str(i):p['choices'] for i,p in enumerate(params) if 'choices' in p}
for tab in t['tabs']:
 assert not any(c['componentData']['type']=='Label' and ' seg ' in c['componentData']['name'] for c in tab['componentDefinition']['componentsData'])
assert not any(c['componentData']['name'] in ('Edit all chords','Chord navigation') for c in children)
assert not any(c['componentData']['name'].startswith('Edit chord ') for c in children)
assert 'DTChordNavigation' not in defs
editor=defs['DTAllChords']
for i in range(1,5):
 bindings=[r['value'] for c in editor['componentsData'] for r in c['handle remapping']['map']]
 for key in ('Root','PadNote'):assert 'Parameter '+str(index[('HarmonyType' if key=='Quality' else key)+str(i)]) in bindings
 for suffix,key in [('Type','Quality'),('Voice','Voicing')]:
  field=defs[f'DTChordField{i}{suffix}']
  assert field['actions'][0]['handler']=='Q-Link' and field['actions'][0]['handleName']=='Parameter '+str(index[('HarmonyType' if key=='Quality' else key)+str(i)])
  assert field['actions'][1]['handleName']=='Local/2/dtChoice'+str(index[('HarmonyType' if key=='Quality' else key)+str(i)])
for i in range(4):
 pad=next(c for c in children if c['componentData']['name']=='Chord '+str(i+1))
 assert pad['handle remapping']['map']==[{'key':'Data','value':'Parameter '+str(index['Touch'+str(i+1)])}]
 definition=defs['DTPad'+str(i)]
 gate='Parameter '+str(index['Touch'+str(i+1)])
 assert definition['actions']==[{'version':1,'onAction':'Mouse Down','handler':'Toggle Switch','handleName':gate,'additionalData':''}]
 widgets=definition['componentsData']
 # The caption must not swallow touches; only a continuous gesture can own release.
 surface=widgets[-1]['componentData']
 assert surface['type']=='Slider'
 assert surface['data']['handleName']=='ResetOnGestureEnd/'+gate
 assert all(surface['data'][key]=='00000000' for key in ('backgroundColour','outlineColour','valueColour'))
 buttons=[c['componentData']['data'] for c in widgets if c['componentData']['type']=='Button']
 assert len(buttons)==1 and buttons[0]['handleName']==gate
assert index['Touch1']==68
print('PASS: stable old IDs, touch gates, separate root editors, declared parameter types')

# Self-remapping Data hides the label on the device; inherit the parent binding.
for name,definition in defs.items():
 if not name.startswith('slider'):continue
 labels=[c for c in definition['componentsData'] if c['componentData']['type']=='Label']
 assert {c['componentData']['data']['type'] for c in labels}>={'Name','Value'}
 assert all(not c['handle remapping']['map'] for c in labels)
print('PASS: readable slider labels inherit parameter bindings')

# Menus must end before the actual knob image begins, not overlap it.
byname={c['componentData']['name']:c for c in children}
for menu,knob in [('DriveType','Drive'),('Filter','Resonance'),('Shape','Rate'),('Target','Depth')]:
 mx,my,mw,mh=map(float,byname[menu]['bounds']['bounds'].split())
 kx,ky,kw,kh=map(float,byname[knob]['bounds']['bounds'].split())
 assert my+mh<=ky,(menu,knob)
print('PASS: selectors do not overlap knob artwork')

assert index["TremRate"]==74 and index["TremDepth"]==75 and index["OscPair"]==76
assert not any(c["componentData"]["name"] in ("Wave1","Wave2","Tone","Motion") for c in children)
print("PASS: append-only new parameters, OSC Pair only, momentary touch bindings")

# Equal outside gutters and aligned rows; catch the observed A-value/extra overlap.
def box(name): return tuple(map(int,byname[name]['bounds']['bounds'].split()))
assert box('Cutoff')[1] == box('Rate')[1] == box('Drive')[1]
assert box('AmpAttack')[1] < box('Cutoff')[1]
assert box('OscPair')[0] < box('AmpAttack')[0] and box('Cutoff')[0] < box('Drive')[0]
assert box('Vibrato')[1]+box('Vibrato')[3] <= box('OscPair')[1]
assert box('Expression')[1]+box('Expression')[3] <= box('Drift')[1]
assert [box(k)[0] for k in ('OscPair','Mix','Sub','Drift')]==sorted(box(k)[0] for k in ('OscPair','Mix','Sub','Drift'))
assert box('Glide')[1]==box('Drive')[1] and 'Space' not in byname
assert any(c['componentData']['name']=='Space' for c in t['tabs'][5]['componentDefinition']['componentsData'])
for i in range(4):
 x,y,w,h=box('Chord '+str(i+1))
 assert h==70 and y+h<=620
 assert all(v['bounds']['bounds']=='0 0 222 70' for v in defs['DTPad'+str(i)]['componentsData'] if v['componentData']['type'] in ('Button','Slider'))
assert t['tabs'][0]['tabName']=='MAIN' and (t['tabs'][0]['fnKeyIndex'],t['tabs'][0]['fnKeySubIndex'])==(0,0)
assert 'Stop touch chords' not in byname
print('PASS: aligned knob rows, no auxiliary overlap, equal outer gutters and tall chord hit bounds')

assert index['Drift']==83 and index['Detune']==10
assert 'Drift' in byname and 'Detune' not in byname
assert any(c['param']=='Detune' for page in json.loads((P/'ui.json').read_text())['pages'] if page['name']=='OSC' for c in page['controls'])
print('PASS: DRIFT replaces MAIN Detune only; OSC Detune and stable IDs retained')

# Header regression: transparent corners must not expose the previous light fill.
for name in ('SoundPrev','SoundNext','SoundPreset','VoiceMode','InputMode'):
 component=defs[byname[name]['componentData']['type']]
 assert all(component['backgroundData'][state]['colour']=='00000000' for state in ('focussed','unfocussed'))
 if name in ('SoundPreset','VoiceMode','InputMode'):
  width,height=box(name)[2:]
  assert all(w['bounds']['bounds']==f'0 0 {width} {height}' for w in component['componentsData'] if w['componentData']['type']=='Button')
assert box('Header donut')==(962,0,318,126)
from PIL import Image
im=Image.open(skin/'main-background-runtime.png').convert('RGB')
assert im.size==(1280,628) and im.getpixel((640,93))==(56,34,24)
assert min(im.getpixel((640,94)))>100
rail=Image.open(skin/'group-panel.png').convert('RGBA')
assert rail.getpixel((5,20))[3]==255 and rail.getpixel((606,20))[3]==255
print('PASS: transparent header fills, matching compact hit bounds, full-bleed art and exact header boundary')

# Shared source child lists previously caused double insertion of both layers.
for key,component in defs.items():
 if key in ('DTKnob','DTChoiceKnob') or key.startswith('DTChoiceKnob-choice-'):
  names=[w['componentData']['name'] for w in component['componentsData']]
  assert names.count('ticks')==1 and names.count('value plate')==1, key
print('PASS: independent knob definitions contain exactly one tick and value plate')

assert index['LearnStatus']==84 and len(params)==275
assert index['PadPosition1']==192 and index['PadPosition16']==207
chord_root=next(tab for tab in t['tabs'] if tab['tabName']=='CHORD')['componentDefinition']['componentsData']
assert not any(w['componentData']['type'].startswith('DTPadPosition') for w in chord_root)
assert not any(key.startswith('DTPadPosition') for key in defs)
for slot in range(1,17):
 p=params[index[f'PadPosition{slot}']]
 assert p['default']==0 and p['choices'][0]=='Unassigned'
 assert p['choices'][1]=='A1' and p['choices'][16]=='A16' and p['choices'][-1]=='H16'
assert 'Position grid' not in json.dumps(t)
from PIL import Image
print('PASS: manual pad-location UI retired, saved parameter IDs retained and MIDI/LEARN independent')
assert 'Stop touch chords' not in byname and 'Sync' not in byname and 'Division' not in byname
assert 'DTLfoSettings' not in defs and 'DTChordAdvanced' in defs
assert 'LFO settings' not in byname
mod=next(tab for tab in t['tabs'] if tab['tabName']=='MOD')
assert all(any(c['componentData']['name']==name for c in mod['componentDefinition']['componentsData']) for name in ('LFO Sync','LFO Division'))
for slot in range(1,5):
 widgets=defs['DTAudition'+str(slot)]['componentsData']
 assert any(w['componentData']['type']=='Slider' and w['componentData']['data']['handleName']=='ResetOnGestureEnd/Parameter '+str(index['Touch'+str(slot)]) for w in widgets)
 roots=[w for w in defs['DTChordRoot'+str(slot)]['componentsData'] if w['componentData']['type']=='Label']
 assert len(roots)==61 and all(w['bounds']['showWhenDataModelInvalid']=='Hide' for w in roots)
print('PASS: momentary editor auditions, root-only labels, advanced safety and appended learn feedback')

# Category-first bank replaces the flat picker in MAIN and detail pages.
from collections import Counter
bank=json.loads((P/'factory-presets.json').read_text())['sounds']
assert Counter(p['category'] for p in bank)=={'Init':1,'Bass':12,'Keys':15,'Synth':12,'Lead':12,'Pad':12}
assert index['LowCut']==85
assert bank[0]['values']['LowCut']==0 and all(0<p['values']['LowCut']<=220 for p in bank[1:])
assert len({json.dumps(p['values'],sort_keys=True) for p in bank})==64
panel=defs['DTChoicePanel'+str(index['SoundPreset'])]
assert 'DTSideSoundCategories' in defs
for ci,category in enumerate(('Init','Bass','Keys','Synth','Lead','Pad'),1):
 assert f'DTSideSoundCategory{ci}' in defs
 choices=[i for i,preset in enumerate(bank) if preset['category']==category]
 assert all(f'DTSideRow{index["SoundPreset"]}-{i}' in defs for i in choices)
print('PASS: category-first 64-new-sound bank, Init, unique recipes and per-sound low cut')

# Bank voicing contract: warm subtractive programs without new modes/UI choices.
for sound in bank[1:]:
 v=sound['values'];assert v['Filter']==0 and v['DriveType']==0,sound['name']
 assert v['Drift']<=5 and v['FM']<=18 and abs(v['Detune'])<=7,sound['name']
 if sound['category']!='Keys':assert v['FM']==0,sound['name']
 if sound['category']=='Bass':assert v['Chorus']==v['Delay']==v['Space']==0 and v['LowCut']<=28,sound['name']
assert bank[0]['values']['FM']==bank[0]['values']['Drive']==bank[0]['values']['Drift']==0
print('PASS: warm subtractive factory voicing, restrained drift/FM and dry bass/Init')


import re
manifest=json.loads((P/'plugin.json').read_text())
a,b,c=map(int,manifest['version'].split('.'))
assert int(re.search(r'i.version=(\d+)',(P/'src/DonutTheory.cpp').read_text()).group(1))==a*10000+b*100+c
print('PASS: generator preserves manifest/binary version agreement')

chord_tab=next(tab for tab in t['tabs'] if tab['tabName']=='CHORD')
assert (chord_tab['fnKeyIndex'],chord_tab['fnKeySubIndex'])==(0,1)
chord_map=next(m for m in q['Screen Mode Q-Links']['map'] if (m['Tab'],m['SubTab'])==(1,2))
expected_chords=[key+str(slot) for slot in range(1,5) for key in ('EditRoot','HarmonyEdit','EditVoicing','EditOctave')]
assert [chord_map['Q-Links'][f'Q-Link {n}'] for bank in physical_banks for n in bank]==[index[k] for k in expected_chords]
assert chord_map['Bank Direction']=='Column'
assert chord_tab['qlinkBoundsData']==[f'96 {112+i*100} 708 96' for i in range(4)]
assert not any(c['componentData']['type']=='DTAllChords' for c in children)
assert not any(c['componentData']['name'].startswith('Active chord ') for c in editor['componentsData'])
print('PASS: dedicated CHORD sub-tab slot banks, row geometry and unchanged MAIN/Program mapping')

for key,value in defs.items():
 if key.startswith('DTChoicePanel') or key in ('DTChordAdvanced','DTLfoSettings'):
  assert value['version']==4 and value['hideQLinkBounds'] is True,key
print('PASS: MAIN first, no navigation popup and modal Q-Link guides disabled')

# Full-page category layers must not swallow sibling category/Done touches.
for key, definition in defs.items():
 if key.startswith("DTSoundCategory"):
  assert definition["ignoreMousePresses"] is True, key

# Guides cover each populated detail column and stop after its last control.
for tab in t['tabs']:
 if tab['tabName'] not in ('OSC','FILTER','AMP','MOD','FX'):continue
 for col,rect in enumerate(tab['qlinkBoundsData']):
  x,y,w,h=map(int,rect.split())
  controls=[c for c in tab['componentDefinition']['componentsData'] if c['componentData']['type'].startswith(('knob','fader','radio','slider')) and int(c['bounds']['bounds'].split()[0])//320==col]
  assert controls
  bottom=0
  for c in controls:
   cx,cy,cw,ch=map(int,c['bounds']['bounds'].split());bottom=max(bottom,cy+ch)
   assert x<=cx and cx+cw<=x+w and y<=cy and cy+ch<=y+h
  assert y+h==bottom+8
for key,definition in defs.items():
 if key.startswith('radio'):
  assert all(c['componentData']['data']['textStyle']['colour']=='fff3dfb8' for c in definition['componentsData'] if c['componentData']['type']=='Label' and ' seg ' in c['componentData']['name'])
print('PASS: detail guide coverage without empty trailing rows and readable radio captions')

# Chord octaves are append-only, mapped to Q-Link 4 and drawn as donuts.
assert [index['ChordOctave'+str(slot)] for slot in range(1,5)]==[87,88,89,90]
for slot in range(1,5):
 assert params[index['Voicing'+str(slot)]]['choices']==['Root','1st inv','Open','2nd inv','3rd inv','4th inv']
 assert params[index['ChordOctave'+str(slot)]]['default']==2
 assert any(c['componentData']['name']==f'Chord {slot} Octave' and c['bounds']['bounds']==f'726 {116+(slot-1)*100} 76 90' for c in editor['componentsData'])
 assert [c['componentData']['type'] for c in defs[f'DTChordOctave{slot}']['componentsData']].count('Knob')==1
 assert all(c['componentData']['type']=='Label' for c in defs[f'DTChordNote{slot}']['componentsData'])
 assert defs[f'DTChordOctave{slot}']['actions'][0]['handler']=='Q-Link'
 for quality,count in enumerate((3,4,5)):
  row=defs[f'DTSideVoicings{slot}-{quality}']
  assert row['ignoreMousePresses'] is True
  group=defs[row['componentsData'][0]['componentData']['type']]
  assert len(group['componentsData'])==count+1
 assert all(a['handler']!='Q-Link' for a in defs[f'DTChordNote{slot}']['actions'])
print('PASS: complete quality-aware inversions, four appended octave selectors and preserved Q-Link banks')

for key,definition in defs.items():
 if key.startswith('DTChoicePanel'):
  assert definition['backgroundData']['focussed']['colour']=='90000000'
  assert any(c['componentData']['name']=='Dismiss outside' and c['bounds']['bounds']=='426 0 854 628' for c in definition['componentsData'])
 if re.fullmatch(r'DTSideRow\d+-\d+',key):
  assert definition['actions'][0]['handler']=='Toggle Switch'
  assert all(int(c['bounds']['bounds'].split()[0])+int(c['bounds']['bounds'].split()[2])<=426 for c in definition['componentsData'])
print('PASS: left drawers preserve the page, show bounded rows and dismiss outside')

# Performance order and numeric plate padding are user-facing layout contracts.
for title in ('A  SOUND / PLAY','B  AMP ENVELOPE','C  FILTER / LFO','D  FX / MOTION'):
 assert title in byname
for name in ('DTKnob','DTEnvelope'):
 value=next(c for c in defs[name]['componentsData'] if c['componentData']['type']=='Label' and c['componentData']['data']['type']=='Value')
 assert value['componentData']['data']['textStyle']['font']['height']==24
 assert value['bounds']['bounds'].split()[0]=='12' and value['bounds']['bounds'].split()[2]=='122'
assert box('Filter')[0]<320 and box('Shape')[0]<box('Target')[0]<640
assert box('DriveType')[0]>640
print('PASS: A Sound/B Envelope/C Filter-LFO/D FX-Tremolo and padded numeric displays')

# One native CHORD tab owns static editor mirrors for the currently selected bank.
assert len(t['tabs'])==7
assert sorted(tab['fnKeySubIndex'] for tab in t['tabs'] if tab['fnKeyIndex']==0)==[0,1]
assert len([m for m in q['Screen Mode Q-Links']['map'] if m['Tab']==1])==2
for slot in range(5,17):
 pad=next(c for c in children if c['componentData']['name']==f'Chord {slot}')
 definition=defs[pad['componentData']['type']]
 assert definition['actions'][0]['handleName']==f'Parameter {index["Touch"+str(slot)]}'
 assert definition['componentsData'][-1]['componentData']['data']['handleName']==f'ResetOnGestureEnd/Parameter {index["Touch"+str(slot)]}'
 assert pad['bounds']['additionalInvalidatingHandles']==[f'IndexedEnabling/{(slot-1)//4}/4/Parameter {index["MainChordPage"]}']
 assert any(w['componentData']['type']=='Label' and w['componentData']['data'].get('text')==str(slot) for w in definition['componentsData'])
 assert index['Root'+str(slot)]==91+(slot-5)*6
assert params[index['SlotCount']]['default']==0
print('PASS: 16-slot append-only graph, four large MAIN pad pages, one static editor mirror map and release surfaces')

# Degenerate RangedEnabling is invalid on the hardware; singleton uses IndexedEnabling.
def no_degenerate_range(o):
 if isinstance(o,str) and o.startswith('RangedEnabling/'):
  tokens=o.split('/');assert tokens[1]!=tokens[2],o
 elif isinstance(o,dict):
  for v in o.values():no_degenerate_range(v)
 elif isinstance(o,list):
  for v in o:no_degenerate_range(v)
no_degenerate_range(t)
assert (skin/'slot-36-28-off.png').is_file() and (skin/'slot-44-44-off.png').is_file()

# Arrow shadows/frames have a transparent bottom row inside the actual hit height.
for file in ('arrow.png','slot-36-28-off.png','slot-44-44-off.png'):
 im=Image.open(skin/file).convert('RGBA');assert im.getbbox()[3]<im.height,file

selector=next(c for c in children if c['componentData']['name']=='Progression')
assert any(v['value']==f"Parameter {index['HarmonyCurrent']}"for v in selector['handle remapping']['map'])
assert len([c for c in children if c['componentData']['name']=='Progression'])==1
print('PASS: one current-bank preset relay with four independent stored banks')

# Preset arrows and bank navigation must each provide a 44x44 hit surface.
for name in ('ChordPrev','ChordNext'):
 c=byname[name];assert tuple(map(int,c['bounds']['bounds'].split()[2:]))==(44,44)
 definition=defs[c['componentData']['type']]
 assert all(w['bounds']['bounds']=='0 0 44 44' for w in definition['componentsData'])
for caption in ('‹','›'):
 hit=next(c for c in children if c['componentData']['name']==caption+' touch')
 assert tuple(map(int,hit['bounds']['bounds'].split()[2:]))==(44,44)
 x,y,w,h=map(int,hit['bounds']['bounds'].split());assert y>=534 and y+h<=580
selector=next(c for c in children if c['componentData']['name']=='Progression')
assert selector['bounds']['bounds']=='68 580 220 44'
for w in defs[selector['componentData']['type']]['componentsData']:assert int(w['bounds']['bounds'].split()[3])==44
assert byname['ChordPrev']['bounds']['bounds']=='18 580 44 44'
assert byname['ChordNext']['bounds']['bounds']=='294 580 44 44'
print('PASS: four navigation arrows have 44x44 hit targets, safe gaps, native-size art and full-height selectors')


editor_widgets=defs['Editor_DTAllChords']['componentsData']
for page in range(4):
 assert any(any(c['componentData']['type']=='Button' and c['componentData']['name']==f'{page*4+1}–{page*4+4}' and c['componentData']['data']['buttonId']==page for c in defs.get(w['componentData']['type'],{}).get('componentsData',[])) for w in editor_widgets)
for slot in range(1,17):
 buttons=[w for w in defs['DTKeyPadSetup']['componentsData'] if w['componentData']['type']=='Button' and w['componentData']['name']==f'LEARN {slot}']
 assert len(buttons)==1 and buttons[0]['componentData']['data']['buttonId']==slot and buttons[0]['componentData']['data']['numButtonsInGroup']==17
print('PASS: in-editor bank buttons, 16 independent learn buttons and MAIN/CHORD two-tab navigation')

editor_widgets=defs['Editor_DTAllChords']['componentsData']
titles=[c for c in editor_widgets if c['componentData']['type']=='Label' and c['componentData']['data'].get('text','').startswith('CHORD EDITOR')]
assert len(titles)==1
assert any(c['componentData']['type']=='Label' and c['componentData']['data'].get('text')=='CHORDS' and c['bounds']['bounds']=='214 564 180 44' for c in editor_widgets)
for key in ('DTMenu-choice-46','sliderWithFocusMenu-choice-46'):
 launcher=defs[defs[key]['componentsData'][-1]['componentData']['type']]
 assert launcher['actions'][0]['handleName']=='Local/2/dtChoice46'
 assert launcher['componentsData'][0]['handle remapping']['map']==[{'key':'Data','value':'Local/7/soundCategory46'}]
 assert launcher['componentsData'][0]['componentData']['data']['buttonId']==0
print('PASS: single editor title, intact CHORDS caption and category-first touch launchers')

assert all("HOLD TO PLAY" not in json.dumps(defs[c["componentData"]["type"]]) for c in children if c["componentData"]["name"].startswith("Chord ") and c["componentData"]["type"].startswith("DTPad"))

# Real-device regression: MAIN bank-arrow art/hits were above selection drawers.
for tab in t['tabs']:
 seen_modal=False
 for widget in tab['componentDefinition']['componentsData']:
  modal=defs.get(widget['componentData']['type'],{}).get('hideQLinkBounds',False)
  if modal:seen_modal=True
  else:assert not seen_modal, f"{tab['tabName']}: ordinary widget above modal: {widget['componentData']['name']}"
print('PASS: all tab modal drawers render and intercept above ordinary widgets and bank arrows')

# Hidden children are still constructed by the MPC. A 10,889-widget CHORD
# tree is too costly; retiring pad-location pickers leaves 1,913 widgets.
def expanded_widgets(component, stack=()):
 total=0
 for widget in component['componentsData']:
  total+=1
  kind=widget['componentData']['type']
  if kind in defs:
   assert kind not in stack, f'Component cycle: {stack} -> {kind}'
   total+=expanded_widgets(defs[kind],stack+(kind,))
 return total
chord_definition=next(tab for tab in t['tabs'] if tab['tabName']=='CHORD')['componentDefinition']
assert expanded_widgets(chord_definition)<=2500
print('PASS: CHORD expanded widget budget <=2500, no recursive component cycles')

# Key reference adds no hardware banks and retains all existing slot controls.
assert index['ChordKeyRoot']==208 and index['TransposeDelta']==219
assert not any(w['componentData']['name'].startswith(('MIDI ', 'LEARN '))for w in editor_widgets)
charts=[w for w in editor_widgets if w['componentData']['name']=='Circle of fifths']
assert len(charts)==24
assert all(w['bounds']['bounds']=='832 108 432 440' and len(w['bounds']['additionalInvalidatingHandles'])==2 for w in charts)
assert all(Image.open(skin/w['componentData']['data']['image']).size==(432,440)for w in charts)
for key in ('DTKeyPicker','DTKeyPadSetup','DTKeyTranspose','DTKeySourcePicker'):
 assert defs[key]['version']==4 and defs[key]['hideQLinkBounds']
 assert any(w['componentData']['name']=='Dismiss outside' and w['componentData']['type']=='Button' and w['componentData']['data']['onImage']=='button-hit.png'for w in defs[key]['componentsData'])
assert not any(w['componentData']['type']=='Label' and w['componentData']['data'].get('type')=='Text' and not w['componentData']['data'].get('text')for definition in defs.values()for w in definition['componentsData'])
# Three shared condition fields cannot overlap a guide: entire right chart is outside x804.
assert all(int(w['bounds']['bounds'].split()[0])>=832 for w in charts)
import sys
sys.path.insert(0,str(P/'tools'))
from key_reference import diatonic
assert diatonic(0,0)==['C','Dm','Em','F','G','Am','Bdim']
assert diatonic(7,0)==['G','Am','Bm','C','D','Em','F#dim']
assert diatonic(9,1)==['Am','Bdim','C','Dm','Em','F','G']
assert diatonic(1,0)==['Db','Ebm','Fm','Gb','Ab','Bbm','Cdim']
print('PASS: 24 accurate key charts, diatonic spelling, reference-only touch controls, PAD SETUP and confirmation drawers')

assert params[:208]==json.loads((P/'tests/legacy-208-parameters.json').read_text())
print('PASS: first208 full parameter declarations match1.2.3 fixture exactly')

assert params[:220]==json.loads((P/'tests/legacy-220-parameters.json').read_text())
assert index['SmoothOpen']==220 and index['SmoothRow16']==241
smooth=defs['DTSmoothVoicing'];assert smooth['version']==4 and smooth['hideQLinkBounds']
assert any(w['componentData']['name']=='Dismiss outside' and w['componentData']['data']['onImage']=='button-hit.png'for w in smooth['componentsData'])
assert len([w for w in smooth['componentsData']if w['componentData']['type']=='Label'and any(v['value']==f"Parameter {index['SmoothRow'+str(row)]}"for row in range(1,17)for v in w['handle remapping']['map'])])==16
for w in smooth['componentsData']:
 x,y,wid,hei=map(int,w['bounds']['bounds'].split());assert x>=0 and y>=0 and x+wid<=1280 and y+hei<=628
print('PASS: 220 declarations retained, SMOOTH scope/preview/apply modal, sixteen top-note rows and unchanged Q-Links')

preview=next(defs[w['componentData']['type']]for w in defs['DTSmoothVoicing']['componentsData']if any(a.get('handleName')==f"Parameter {index['SmoothPreview']}"for a in defs.get(w['componentData']['type'],{}).get('actions',[])))
assert sum(a.get('handler')=='Toggle Switch' and a.get('handleName')==f"Parameter {index['SmoothPreview']}"for a in preview['actions'])==1
assert preview['componentsData'][0]['handle remapping']['map']==[{'key':'Data','value':f"Parameter {index['SmoothPreview']}"}]
assert preview['componentsData'][-1]['componentData']['type']=='Slider' and preview['componentsData'][-1]['componentData']['data']['handleName'].startswith('ResetOnGestureEnd/Local/')
print('PASS: PREVIEW is a persistent start/stop command, with local-only gesture release')

assert params[:242]==json.loads((P/'tests/legacy-242-parameters.json').read_text())
assert all(len(params[index['HarmonyType'+str(i)]]['choices'])==32 for i in range(1,17))
assert all(len(params[index['HarmonyPreset'+str(i)]]['choices'])==96 for i in range(1,5))
assert all(len(params[index['Quality'+str(i)]]['choices'])==11 for i in range(1,17))
print('PASS: first242 declarations/ranges retained; 32 types/96 recipes and full-range Q-Link proxies')

from collections import Counter
progressions=json.loads((P/'factory-presets.json').read_text())['progressions']
assert len(progressions)==96 and len({p['name']for p in progressions})==96
assert Counter(p['category']for p in progressions[32:])=={k:8 for k in ('Neo Soul','Hip-hop','Vamps','R&B','Modal','Gospel','Borrowed','Turnarounds')}
fingerprint=lambda p:tuple(((note-p['chords'][0][0])%12,quality)for note,quality in p['chords'])
assert len({fingerprint(p)for p in progressions[32:]})==64
for tab in t['tabs']:assert expanded_widgets(tab['componentDefinition'])<=3000,tab['tabName']
print('PASS: 64 distinct new harmonic forms/eight styles and all page construction budgets')

for title,param in [('GLIDE','Glide'),('STRUM','Spread')]:
 definition=next(d for d in defs.values()if any(w['componentData']['type']=='Label'and w['componentData']['data'].get('text')==title for w in d['componentsData'])and any(w['componentData']['name']=='time drag'for w in d['componentsData']))
 assert definition['componentsData'][-1]['componentData']['type']=='Slider'
 assert definition['componentsData'][-1]['bounds']['acceptsHWFocus']=='No'
 assert all(a['handler']!='Q-Link'for a in definition['actions'])
print('PASS: touch GLIDE/STRUM use existing values without stealing the sixteen chord Q-Links')

assert params[:272]==json.loads((P/'tests/legacy-272-parameters.json').read_text())
assert [p['key']for p in params[272:]]==['ModMode','ModShape','Pan']
assert params[index['ModMode']]['choices']==['Tremolo','Auto Pan']
for key in ['ModMode','ModShape']:
 assert 'DTChoicePanel'+str(index[key]) in defs
 assert any(w['componentData']['name']==key for w in children)
assert len(next(tab for tab in t['tabs']if tab['tabName']=='FX')['qlinkBoundsData'])==4
print('PASS: first272 declarations retained; shared motion selectors, Pan detail and unchanged MAIN sixteen Q-Links')

fx=next(tab['componentDefinition']['componentsData']for tab in t['tabs']if tab['tabName']=='FX')
assert not any(w['componentData']['name'].startswith('SPACE:')for w in fx)
for name in ['Mod Rate','Mod Depth']:
 w=next(w for w in fx if w['componentData']['name']==name)
 assert w['handle remapping']['map'][0]['value'].startswith('WithName['+name+']/')
assert next(w for w in fx if w['componentData']['name']=='Motion Shape' and w['componentData']['type']!='Label')['componentData']['type'].startswith('DTSmallMenu')
print('PASS: FX motion labels override legacy names; shape has a direct dropdown and no footer overlaps the sixteen cells')
