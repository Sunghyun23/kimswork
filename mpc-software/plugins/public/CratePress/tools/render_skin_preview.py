#!/usr/bin/env python3
"""Offline layout proof from the runtime TUI and measured engine telemetry, not an MPC capture."""
from pathlib import Path
import json,math,sys
from PIL import Image,ImageDraw,ImageFont
P=Path(__file__).resolve().parents[1];skin=P/'skins'
tui=json.loads((skin/'TUI.json').read_text());pd=tui['pageData'];defs={d['key']:d['value']for d in pd['componentDefinitions']['localComponentDefinitions']}
params=json.loads((P/'build/params.json').read_text())['params'];performance='--performance'in sys.argv;state='ui-performance-state.json'if performance else'ui-state.json';values=json.loads((P/'build'/state).read_text())['params'];values += [p['default']for p in params[len(values):]]
# plain-domain ranges must follow the declared engine schema.
ranges=[(-24,6),(-18,18),(0,100),(0,100),(30,800),(0,100),(0,100),(0,100),(0,100),(0,100),(-100,100),(0,100),(0,100),(0,250),(2000,20000),(0,100)]
fontfile='/System/Library/Fonts/Supplemental/Arial.ttf'
cache={}
def font(n):return ImageFont.truetype(fontfile,max(10,int(n)))
def color(c):
 if c=='0':return(0,0,0,0)
 c=c.zfill(8);return tuple(int(c[i:i+2],16)for i in (2,4,6,0))
def norm(h):
 if h.startswith('Invert/'):return 1-norm(h[7:])
 return values[int(h.split()[-1])]if h.startswith('Parameter ')else 0

def text_value(h):
 i=int(h.split()[-1]);v=norm(h)
 if i==40:return['FULL','LOW','MID','HIGH'][round(v*3)]
 if i>=33 and i<40:return f'{v*144-72:.1f} dB'
 if i>=len(ranges):return str(round(v,2))
 lo,hi=ranges[i];plain=lo*(hi/lo)**v if i in(4,14)else lo+(hi-lo)*v
 if i==13 and plain<1:return'Off'
 if i==14 and plain>=19500:return'Open'
 if i in(13,14):return f'{plain/1000:.1f} kHz'if plain>=1000 else f'{plain:.0f} Hz'
 return f'{plain:.1f} dB'if i in(0,1)else f'{plain:.0f} ms'if i==4 else f'{plain:.0f}%'
canvas=Image.new('RGBA',(1280,628))
def render(comp,offset=(0,0),bindings=None):
 bindings=bindings or{}
 for e in comp['componentsData']:
  hidden=False
  for conditional in e['bounds'].get('additionalInvalidatingHandles',[]):
   if conditional.startswith('IndexedEnabling/'):
    _,step,count,handle=conditional.split('/',3)
    if round(norm(handle)*(int(count)-1))!=int(step):hidden=True
  if hidden:continue
  x,y,w,h=map(float,e['bounds']['bounds'].split());x,y=int(x+offset[0]),int(y+offset[1]);w,h=int(w),int(h)
  c=e['componentData'];t=c['type'];d=c['data'];binding=dict(bindings)
  for r in e['handle remapping']['map']:binding[r['key']]=bindings.get(r['value'],r['value'])
  handle=binding.get(d.get('handleName','Data'),d.get('handleName','Data'))
  if t in defs:render(defs[t],(x,y),binding);continue
  if t in('Image','Knob','Button'):
   if t=='Image':file=d['image'];frames=d.get('numFrames',0)
   elif t=='Knob':
    if d.get('knobType')!='FilmStrip':continue
    file=d['filmStrip'];frames=d['numFrames']
   else:
    count=d['numButtonsInGroup'];on=(round(norm(handle)*max(1,count-1))==d['buttonId']);file=d['onImage']if on else d['offImage'];frames=0
   if file not in cache:cache[file]=Image.open(skin/file).convert('RGBA')
   im=cache[file]
   if frames:
    height=im.height//frames;index=min(frames-1,max(0,round(norm(handle)*(frames-1))));im=im.crop((0,index*height,im.width,(index+1)*height))
   #Native Image assets are sized by generator; no silent scaling in this proof.
   if im.size!=(w,h):raise AssertionError((file,im.size,(w,h)))
   canvas.alpha_composite(im,(x,y))
  elif t=='Label':
   ts=d['textStyle'];txt=text_value(handle)if d['type']=='Value'else d.get('text','');draw=ImageDraw.Draw(canvas);draw.text((x+w/2,y+h/2),txt,font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial Bold.ttf'if ts['font'].get('style')=='Bold'else fontfile,max(10,int(ts['font']['height']))),anchor='mm',fill=color(ts['colour']))
render(pd['tabs'][0]['componentDefinition'])
prefix='main-performance-preview'if performance else'main-runtime-preview';canvas.convert('RGB').save(P/'build'/(prefix+'.png'))
full=Image.new('RGB',(1280,800),'#111111');full.paste(canvas.convert('RGB'),(0,110));d=ImageDraw.Draw(full)
for title,text,x in [('TRACK','Audio 001',24),('PLUGIN','Crate Press',380),('PRESET','Drums / Pocket Press',752)]:
 d.text((x,10),title,font=font(17),fill='#99958e');d.text((x,36),text,font=font(27),fill='#eee5d6')
for i,t in enumerate(['MAIN','DUCK','COLOR','PERFORM','MORE']):d.text((128+i*256,759),t,font=font(27),fill='#efc28d'if i==0 else'#aaa49c',anchor='mm')
d.line((38,786,218,786),fill='#e18a38',width=4)
full.save(P/'build'/(prefix+'-screen.png'))
print('Rendered runtime TUI using measured audio data (offline proof, not hardware screenshot)')
