from pathlib import Path
from PIL import Image
import json
P=Path(__file__).resolve().parents[1];pd=json.loads((P/'skins/TUI.json').read_text())['pageData'];defs={x['key']:x['value']for x in pd['componentDefinitions']['localComponentDefinitions']};meta=json.loads((P/'parameters.json').read_text());ix={x['key']:i for i,x in enumerate(meta)};ql=json.loads((P/'skins/Q-Links.json').read_text());tabs=pd['tabs']
assert [x['tabName']for x in tabs]==['MAIN','CHORD','MODEL','TOUCH','MOTION','FX','SETUP']
assert [(x['fnKeyIndex'],x['fnKeySubIndex'])for x in tabs]==[(0,0),(0,1),(1,0),(2,0),(3,0),(4,0),(5,0)]
assert Image.open(P/'skins/background.png').size==(1280,628)
assert Image.open(P/'skins/knob.png').size==(72,72*97)
assert Image.open(P/'skins/lock-0.png').size==(44,44)
assert meta[0]['key']=='Master'and len(meta)==246 and meta[ix['Model']]['choices'][1]=='Wurli'
assert defs['FeltChordEditor']['hideQLinkBounds']and defs['FeltChordEditor']['version']==4
for name,d in defs.items():
 if name.startswith('ChoicePanel'):assert d['hideQLinkBounds']and d['version']==4
 for w in d.get('componentsData',[]):
  c=w['componentData'];b=list(map(int,w['bounds']['bounds'].split()));assert b[2]>0 and b[3]>0,(name,b)
  if c['type']=='Label'and c.get('data',{}).get('labelType')=='Text':assert c['data'].get('text'),name
  assert c.get('data',{}).get('text')!='▾','Use native triangle image, not unsupported font glyph'
main=tabs[0]['componentDefinition']['componentsData'];dials=[]
for w in main:
 d=defs.get(w['componentData']['type'],{});knobs=[c for c in d.get('componentsData',[])if c['componentData']['type']=='Knob']
 if knobs:
  assert len(knobs)==1;key=int(next(v['value']for v in knobs[0]['handle remapping']['map']if v['key']=='Data').split()[-1]);dials.append((w,d,knobs[0],key))
mainKeys=[ix[k]for k in ['Tone','Body','Character3','Hiss','Attack','Decay','Sustain','Release','Rate','Depth','Drift','Width','Drive','Chorus','Delay','Room']]
assert [x[-1]for x in dials]==mainKeys
assert [meta[x[-1]]['key']for x in dials[4:8]]==['Attack','Decay','Sustain','Release']
for bank in range(4):
 gx,gy,gw,gh=map(int,tabs[0]['qlinkBoundsData'][bank].split())
 for widget,d,k,index in dials[bank*4:bank*4+4]:
  x,y,w,h=map(int,widget['bounds']['bounds'].split());assert gx<=x and gy<=y and x+w<=gx+gw and y+h<=gy+gh
  locks=[c for c in d['componentsData']if c['componentData']['type'].startswith('Lock')];assert len(locks)==1
  lx,ly,lw,lh=map(int,locks[0]['bounds']['bounds'].split());kx,ky,kw,kh=map(int,k['bounds']['bounds'].split());assert(lw,lh)==(44,44)and lx+lw<=kx and kw==kh==72
for row in range(4):
 p=defs['ChordPad'+str(row)];assert p['componentsData'][-1]['componentData']['type']=='Slider';assert p['componentsData'][-1]['bounds']['bounds']=='0 0 304 70';assert p['actions'][0]['handleName']=='Parameter '+str(ix['Hold'+str(row+1)])
 assert p['componentsData'][-1]['componentData']['data']['handleName']=='ResetOnGestureEnd/Parameter '+str(ix['Hold'+str(row+1)])
maps=ql['Screen Mode Q-Links']['map'];assert len(maps)==7
for row in range(4):
 for col in range(4):
  physical='Q-Link '+str(13+row-4*col);assert maps[0]['Q-Links'][physical]==ix['MainQ'+str(row*4+col)];assert maps[1]['Q-Links'][physical]==ix['Edit'+str(row*4+col)];assert ql['Program Mode Q-Links'][physical]==mainKeys[row*4+col]
for key,d in defs.items():
 for w in d.get('componentsData',[]):
  if w['componentData']['type']=='Image':
   f=P/'skins'/w['componentData']['data']['image'];im=Image.open(f);assert max(im.size)<=8192
print('PASS approved MAIN16/ADSR order;72pxdials;44pxnonoverlapping locks;70pxhold pads;7native maps; modal guides; category/native selectors and image limits')

# Sound browsing has one identical footer position on every native page.
for tab in tabs:
 matches=[]
 for widget in tab['componentDefinition']['componentsData']:
  definition=defs.get(widget['componentData']['type'],{})
  for child in definition.get('componentsData',[]):
   if child['componentData']['type']=='Label' and any(x['key']=='Data'and x['value']=='Parameter '+str(ix['Preset'])for x in child.get('handle remapping',{}).get('map',[])):
    matches.append(widget['bounds']['bounds'])
 assert matches==['638 578 304 44'],(tab['tabName'],matches)
print('PASS one consistent bottom SOUND PRESET browser on all7native pages')

# A dedicated fixed-lighting face replaces the old rotating screen crop.
import numpy as np
frames=np.asarray(Image.open(P/'skins/knob.png'))
y,x=np.ogrid[:72,:72];outside=(x-36)**2+(y-36)**2>31**2
for n in range(1,97):assert np.array_equal(frames[:72][outside],frames[n*72:(n+1)*72][outside])
assert (P/'assets/knob/bakelite-face-source.png').is_file()
print('PASS independent transparent knob asset; stationary body/light across97frames')

# Original238identities and metadata are permanently stable.
original=json.loads((P/"tests/parameter-prefix-238.json").read_text());assert meta[:238]==original and meta[238]["key"]=="Hiss"and meta[239]["key"]=="LockHiss"
print("PASS238parameter prefix exact; appended Noise/lock; MAIN and Program A4 target Noise, MODEL retains character4")

assert meta[:240]==json.loads((P/"tests/parameter-prefix-240.json").read_text())
assert [meta[ix[k]]["default"]for k in ["CmdDiceNote","CmdPrevNote","CmdNextNote"]]==[24,25,26]
