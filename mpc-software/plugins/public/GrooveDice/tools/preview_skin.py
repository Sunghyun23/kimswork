#!/usr/bin/env python3
"""Offline composition from runtime TUI/assets, not a device screenshot."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont
P=Path(__file__).resolve().parents[1];S=P/'skins';t=json.loads((S/'TUI.json').read_text())['pageData'];defs={x['key']:x['value']for x in t['componentDefinitions']['localComponentDefinitions']};meta=json.loads((P/'parameters.json').read_text());canvas=Image.new('RGBA',(1280,628),'#efd9b3')
fontpath='/System/Library/Fonts/Supplemental/Arial Bold.ttf'
def bound(entry):return tuple(map(int,entry['bounds']['bounds'].split()))
def plain(entry):
 remap=entry.get('handle remapping',{}).get('map',[]);key=next((x['value']for x in remap if x['key']=='Data'),'Parameter 0');return int(key.split()[-1])if key.startswith('Parameter ')else 0

def render(entries,ox=0,oy=0):
 for entry in entries:
  x,y,w,h=bound(entry);x+=ox;y+=oy;comp=entry['componentData'];typ=comp['type'];data=comp.get('data',{});index=plain(entry);m=meta[index];value=m['default'];norm=value/(len(m['choices'])-1)if'choices'in m and len(m['choices'])>1 else(value-m.get('min',0))/max(1e-9,m.get('max',1)-m.get('min',0))
  if typ in defs:render(defs[typ]['componentsData'],x,y)
  elif typ=='Image':
   path=S/data['image'];im=Image.open(path).convert('RGBA');canvas.alpha_composite(im,(x,y))
  elif typ=='Knob'and data.get('knobType')=='FilmStrip':
   strip=Image.open(S/data['filmStrip']);frame=round(norm*(data['numFrames']-1));im=strip.crop((0,frame*w,w,(frame+1)*w));canvas.alpha_composite(im,(x,y))
  elif typ=='Button':
   file=data.get('offImage')or data.get('imageOff');
   if file:canvas.alpha_composite(Image.open(S/file).convert('RGBA'),(x,y))
  elif typ=='Label':
   text=data.get('text','');style=data['textStyle'];color='#'+style['colour'][2:];size=int(style['font']['height']*.78)
   if data.get('type')=='Value':
    text=m['choices'][int(value)]if'choices'in m else f'{value:.{m.get("decimals",0)}f}'+(' '+m['unit']if m.get('unit')else'')
    if m['key']=='BankInfo':text='LIBRARY · 11,039 WAV / 95 MIDI'
    if m['key']=='Status':text='READY'
    if m['key']=='Position':text='BAR 1'
    if m['key']=='PreviewBpm':text='92 BPM'
   f=ImageFont.truetype(fontpath,max(10,size));d=ImageDraw.Draw(canvas);box=d.textbbox((0,0),text,font=f);d.text((x+(0 if 'left' in style['justification'] else (w-(box[2]-box[0]))/2),y+(h-(box[3]-box[1]))/2-box[1]),text,font=f,fill=color)
render(defs['GDMain']['componentsData']);P.joinpath('build/qa').mkdir(exist_ok=True,parents=True);canvas.convert('RGB').save(P/'build/qa/main-runtime-composition-v2.png')
