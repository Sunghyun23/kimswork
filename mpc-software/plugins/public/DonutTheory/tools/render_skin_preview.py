#!/usr/bin/env python3
"""Approximate offline MAIN rendering from actual TUI JSON, not an MPC emulator."""
import json,sys
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
P=Path(__file__).resolve().parents[1];S=P/'skins'
tui=json.loads((S/'TUI.json').read_text())['pageData']
defs={v['key']:v['value']for v in tui['componentDefinitions']['localComponentDefinitions']}
params=json.loads((P/'build/params.json').read_text())['params']
bank_category=sys.argv[2] if len(sys.argv)>2 and sys.argv[1]=='--bank' else None
sound_index=next(i for i,p in enumerate(params) if p['name']=='Sound Preset')
local_values={f'Local/2/dtChoice{sound_index}':1 if bank_category else 0,f'Local/7/soundCategory{sound_index}':(('Init','Bass','Keys','Synth','Lead','Pad').index(bank_category)+1) if bank_category and bank_category!='Categories' else 0}
canvas=Image.new('RGBA',(1280,628));d=ImageDraw.Draw(canvas)
fontpath='/System/Library/Fonts/Supplemental/Arial.ttf'
def color(s):
 if s in ('0','00000000'):return (0,0,0,0)
 s=s.zfill(8);return tuple(int(s[i:i+2],16)for i in (2,4,6,0))
def rect(bounds):return tuple(int(float(n))for n in bounds.split())
def resolve(h,mapping):return mapping.get(h,h)
def param(h):
 if '/' in h:h=h.rsplit('/',1)[1]
 return params[int(h.split()[1])] if h.startswith('Parameter ') else None
def comp(c,x,y,w,h,mapping):
 bg=c['backgroundData']['unfocussed'];col=color(bg['colour'])
 if col[3]:d.rectangle((x,y,x+w-1,y+h-1),fill=col)
 for entry in c['componentsData']:
  b=entry['bounds'];
  if b['whenVisible']=='WhenFocussed':continue
  skip=False
  for condition in b.get('additionalInvalidatingHandles',[]):
   if condition.startswith('IndexedEnabling/'):
    _,step,count,handle=condition.split('/',3)
    if handle.startswith('Local/'):
     if local_values.get(handle,0)!=int(step):skip=True
     continue
    cp=param(handle)
    if cp:
     value=cp.get('default',0);lo=cp.get('min',0);hi=cp.get('max',1)
     if round((value-lo)/(hi-lo)*(int(count)-1))!=int(step):skip=True
  if skip:continue
  a,bb,ww,hh=rect(b['bounds']);ex,ey=x+a,y+bb
  cd=entry['componentData'];data=cd['data'];typ=cd['type'];mp=dict(mapping)
  for rem in entry['handle remapping']['map']:mp[rem['key']]=resolve(rem['value'],mapping)
  if typ in defs:comp(defs[typ],ex,ey,ww,hh,mp);continue
  p=param(resolve(data.get('handleName',''),mp))
  if typ=='Image':
   im=Image.open(S/data['image']).convert('RGBA').resize((ww,hh));canvas.alpha_composite(im,(ex,ey))
  elif typ=='Knob':
   if data['knobType']=='RotatingImage':
    im=Image.open(S/data['foregroundImage']).convert('RGBA').resize((max(1,round(ww*data.get('imageScale',1))),max(1,round(hh*data.get('imageScale',1)))));im=im.rotate(135-(p['default']if p else .5)*270,resample=Image.Resampling.BICUBIC)
   elif data['knobType']=='GenericLegacy':
    d.ellipse((ex,ey,ex+ww-1,ey+hh-1),fill='#635844',outline='#352d21',width=2);continue
   else:
    im=Image.open(S/data['filmStrip']).convert('RGBA');size=im.height//data['numFrames'];frame=int((p['default']if p else .5)*(data['numFrames']-1));im=im.crop((0,frame*size,im.width,(frame+1)*size))
   canvas.alpha_composite(im,(ex,ey))
  elif typ=='Button':
   h=resolve(data.get('handleName','Data'),mp);p=param(h)
   v=local_values.get(h,0) if h.startswith('Local/') else p.get('default',0)*(data.get('numButtonsInGroup',2)-1) if p else 0
   file=data['onImage'] if int(round(v))==data.get('buttonId',1) else data['offImage']
   im=Image.open(S/file).convert('RGBA');canvas.alpha_composite(im,(ex,ey))
  elif typ in ('Label','ComboBox'):
   kind=data.get('type','Value');txt=data.get('text','') if kind=='Text' else (p['name']if kind=='Name' else p['display'])if p else data.get('text','')
   sty=data['textStyle']
   binding=resolve(data.get('handleName',''),mp)
   if kind=='Name' and binding.startswith('WithName['):txt=binding.split(']',1)[0][9:]
   if sty.get('case')=='Upper Case':txt=txt.upper()
   font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial Bold.ttf' if sty['font'].get('style')=='Bold' else fontpath,int(sty['font']['height']*.78));box=d.textbbox((0,0),txt,font=font);tw=box[2]-box[0];th=box[3]-box[1]
   d.text((ex+(ww-tw)/2,ey+(hh-th)/2-box[1]),txt,font=font,fill=color(sty['colour']))
editor=int(sys.argv[1]) if len(sys.argv)>1 and sys.argv[1].isdigit() else 0
comp(defs['DTChoicePanel'+str(sound_index)] if bank_category else defs['DTAllChords'] if editor else tui['tabs'][0]['componentDefinition'],0,0,1280,628,{})
destination=P/'build'/('preset-browser-'+bank_category.lower()+'.png' if bank_category else 'chord-editor-preview.png' if editor else 'main-preview.png')
canvas.convert('RGB').save(destination)
print(destination)
