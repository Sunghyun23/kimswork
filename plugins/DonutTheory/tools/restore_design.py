"""Approved-layout restoration. Changes visuals only; retains parameter/action contracts."""
from PIL import Image, ImageDraw, ImageFont
import math

def install(g,tui,out):
 O=g.obj
 def get(o,k): return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 pd=get(tui,'pageData');defs={get(d,'key'):get(d,'value') for d in get(get(pd,'componentDefinitions'),'localComponentDefinitions')}
 main=get(get(pd,'tabs')[0],'componentDefinition');children=get(main,'componentsData')
 def bounds(c,s):put(get(c,'bounds'),'bounds',s)
 def cd(c):return get(c,'componentData')
 def image(name,file,b):return g.entry(name,'Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',file)),b,'No','Show','Always')
 def style(data,size=None):
  st=get(data,'textStyle');f=get(st,'font');put(f,'name','Roboto');put(f,'style','Bold')
  if size:put(f,'height',float(size))
  put(st,'colour','ff241b10')
 def art(file,w,h,kind='lcd'):
  im=Image.new('RGBA',(w,h));d=ImageDraw.Draw(im)
  top,bottom=((207,209,92),(161,168,41)) if kind=='lcd' else ((255,222,159),(207,151,76))
  if kind=='active':top,bottom=(220,140,65),(166,77,25)
  d.rounded_rectangle((1,3,w-2,h-1),6,fill='#634322')
  for y in range(2,h-4):
   t=y/max(1,h-5);col=tuple(round(a*(1-t)+b*t) for a,b in zip(top,bottom));d.line((3,y,w-4,y),fill=col)
  d.rounded_rectangle((1,1,w-2,h-4),5,outline='#886423',width=2)
  d.line((6,4,w-7,4),fill='#fff0b3');im.save(out/file)
 # Typography across menus/detail pages, including dynamically generated captions.
 def typography(o):
  if isinstance(o,g.Obj):
   if any(k=='textStyle' for k,v in o):style(o)
   for k,v in o:typography(v)
  elif isinstance(o,list):
   for v in o:typography(v)
 typography(tui)
 art('value-lcd-v2.png',138,28)
 # The device can omit short nested Image entries after a cold skin load.
 # Keep the artwork at its exact size, with a larger transparent image canvas.
 def plate(file,size,source,position):
  canvas=Image.new('RGBA',size)
  canvas.alpha_composite(Image.open(out/source).convert('RGBA'),position)
  canvas.save(out/file)
 plate('knob-value-panel.png',(146,144),'value-lcd-v2.png',(4,116))
 plate('envelope-value-panel.png',(146,168),'value-lcd-v2.png',(4,140))
 # Image entries stay below native widgets so input behaviour is unchanged.
 for name in [n for n in defs if n in ('DTKnob','DTChoiceKnob') or n.startswith('DTChoiceKnob-choice-')]:
  comp=defs[name];ws=get(comp,'componentsData')
  for w in ws:
   typ=get(cd(w),'type');data=get(cd(w),'data')
   if typ=='Knob':bounds(w,'31 0 84 84')
   elif typ=='Label':
    isname=get(data,'type')=='Name';bounds(w,'0 86 146 26' if isname else '12 116 122 28');style(data,26 if isname else 24)
    if isname:put(get(data,'textStyle'),'case','Upper Case')
    elif name.startswith('DTChoiceKnob'):style(data,22)
  ws.insert(0,image('value plate','knob-value-panel.png','0 0 146 144'))
  # Tick ring is code-native artwork; the real donut asset is retained.
  ring=Image.new('RGBA',(146,86));d=ImageDraw.Draw(ring)
  for j in range(13):
   a=math.radians(135+j*22.5);cx,cy=73,42
   d.line((cx+44*math.cos(a),cy+44*math.sin(a),cx+49*math.cos(a),cy+49*math.sin(a)),fill='#372511',width=2)
  ring.save(out/'knob-ticks.png');ws.insert(0,image('ticks','knob-ticks.png','0 0 146 86'))
 env=defs['DTEnvelope'];ws=get(env,'componentsData')
 for w in ws:
  data=get(cd(w),'data');typ=get(cd(w),'type')
  if typ=='Label':
   isname=get(data,'type')=='Name';bounds(w,'0 0 146 26' if isname else '12 140 122 28');style(data,27 if isname else 24)
  elif typ=='Knob':bounds(w,'41 34 64 92')
 ws.insert(0,image('value plate','envelope-value-panel.png','0 0 146 168'))
 touch=defs['DTTouchValue'];put(touch,'backgroundData',g.background_data('00000000'))
 tw=get(touch,'componentsData');art('touch-value-lcd-v2.png',104,28)
 plate('touch-value-panel.png',(286,32),'touch-value-lcd-v2.png',(182,0))
 tw.insert(0,image('touch value plate','touch-value-panel.png','0 0 286 32'))
 for w in tw:
  if get(cd(w),'type')=='Label' and get(get(cd(w),'data'),'type')=='Value':
   bounds(w,'188 0 92 28');style(get(cd(w),'data'),22)
  if get(cd(w),'type')=='Knob':
   put(cd(w),'data',O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','ffb18a48'),('outlineColour','ff6c502e'),('valueColour','ff704723'),('isBipolar',False),('handleName','Data')));put(cd(w),'type','Slider');bounds(w,'3 9 24 10')
 # Rounded group borders and title rails, not flat brown rectangles.
 panel=Image.new('RGBA',(612,218));d=ImageDraw.Draw(panel)
 d.rounded_rectangle((0,0,611,217),16,fill=(255,228,173,58))
 # Clip a full-width title rail to the same rounded panel silhouette.
 rail=Image.new('RGBA',(612,218));rd=ImageDraw.Draw(rail)
 mask=Image.new('L',(612,218));md=ImageDraw.Draw(mask)
 md.rounded_rectangle((0,0,611,217),16,fill=255)
 md.rectangle((0,29,611,217),fill=0)
 for y in range(29):
  t=y/29;rd.line((0,y,611,y),fill=(round(202-43*t),round(92-36*t),round(25-8*t),255))
 panel.paste(rail,(0,0),mask)
 d=ImageDraw.Draw(panel)
 d.rounded_rectangle((0,0,611,217),16,outline='#a5662b',width=2)
 d.rounded_rectangle((3,2,608,214),14,outline='#fff0c5',width=1)
 panel.save(out/'group-panel.png')
 placements={
 'SoundPrev':(405,38,36,34),'SoundPreset':(447,38,300,34),'SoundNext':(753,38,36,34),
 'VoiceMode':(792,38,136,32),'InputMode':(936,38,136,32),
 'SOUND PRESET':(420,9,330,26),'VOICE':(792,9,136,26),'INPUT':(936,9,136,26),
 'Filter':(108,356,136,32),'DriveType':(662,356,136,32),
 'Shape':(326,356,136,32),'Target':(476,356,136,32),'Sync':(962,356,136,32),'Division':(1112,356,136,32),
 'Vibrato':(26,126,286,28),'Expression':(332,126,286,28),
 'CHORD PRESET':(18,550,190,28),'ChordPrev':(18,586,36,34),'Progression':(60,586,220,34),'ChordNext':(286,586,36,34),
 'Stop touch chords':(1182,588,80,32),
 'Filter / LFO divider':(320,390,2,144),'FX / Tremolo divider':(952,390,2,144),
 }
 groups=[(['OscPair','Body','Drift','Glide'],26,164,144),(['AmpAttack','AmpDecay','AmpSustain','AmpRelease'],658,136,168),(['Cutoff','Resonance','Rate','Depth'],26,390,144),(['Drive','Space','TremRate','TremDepth'],658,390,144)]
 for keys,x,y,h in groups:
  for j,key in enumerate(keys):placements[key]=(x+j*150,y,146,h)
 for i in range(4):placements['Chord '+str(i+1)]=(348+i*204,586,195,40);placements['Edit chord '+str(i+1)]=(390+i*204,556,110,26)
 # Full-height chord touch surfaces retain an 8px bottom safe margin.
 for i in range(4):placements['Chord '+str(i+1)]=(348+i*204,550,195,70)
 remove={'FX overview','Touch chord instructions'}
 children[:]=[c for c in children if get(cd(c),'name') not in remove]
 for c in children:
  name=get(cd(c),'name');typ=get(cd(c),'type')
  if name in placements:bounds(c,' '.join(map(str,placements[name])))
  if name in ('TremRate','TremDepth'):
   rem=get(get(c,'handle remapping'),'map')[0];put(rem,'value','WithName['+('TREM RATE' if name=='TremRate' else 'TREM DEPTH')+']/'+get(rem,'value'))
  if name.startswith(('A  ','B  ','C  ','D  ')):
   x=18 if name[0] in 'AC' else 650;y=94 if name[0] in 'AB' else 324
   bounds(c,f'{x} {y} 612 218');comp=defs[typ];put(comp,'backgroundData',g.background_data('00000000'))
   ws=get(comp,'componentsData');ws.insert(0,image('panel','group-panel.png','0 0 612 218'))
   data=get(cd(ws[-1]),'data');style(data,30);put(get(data,'textStyle'),'colour','fff4d7a3');put(get(data,'textStyle'),'justification','left verticallyCentred');bounds(ws[-1],'50 0 545 29')
 # Narrow only chord-preset menu; identities, choice panels and bindings survive.
 for c in children:
  name=get(cd(c),'name');typ=get(cd(c),'type')
  if name=='Progression':
   comp=defs[typ]
   for w in get(comp,'componentsData'):
    typw=get(cd(w),'type');data=get(cd(w),'data');bounds(w,'0 0 220 34')
    if typw=='Button' and get(data,'offImage')!='button-hit.png':put(data,'onImage','menu-chord.png');put(data,'offImage','menu-chord.png')
   art('menu-chord.png',220,34)
 # Standard menu artwork remains the same size as its actual hit bounds.
 art('menu-wide.png',300,34);art('menu-small.png',136,32);art('arrow.png',36,34,'button')
 for edit_name in ('DTEditAll',):
  comp=defs[edit_name];put(comp,'backgroundData',g.background_data('00000000'));get(comp,'componentsData').insert(0,image('edit bevel','edit-button-panel.png','0 0 110 64'))
 art('edit-bevel-v2.png',110,28,'button')
 plate('edit-button-panel.png',(110,64),'edit-bevel-v2.png',(0,0))
 art('pad-off.png',195,70,'button');art('pad-on.png',195,70,'active')
 for i in range(4):
  for w in get(defs['DTPad'+str(i)],'componentsData'):
   bounds(w,'0 0 195 70')
 # STOP is moved beside chord pads and resized including its final transparent hit.
 comp=defs['DTStop']
 for w in get(comp,'componentsData'):
  bounds(w,'0 0 80 32');data=get(cd(w),'data')
  if get(cd(w),'type')=='Button' and get(data,'offImage')!='button-hit.png':put(data,'onImage','stop-bevel.png');put(data,'offImage','stop-bevel.png')
 art('stop-bevel.png',80,32,'button')
 put(main,'backgroundData',g.background_data('ffe1d2ae'))
 put(get(pd,'tabs')[0],'qlinkBoundsData',['18 94 612 218','650 94 612 218','18 324 612 218','650 324 612 218'])

 # Header-anchored collage: just its lower edge crosses the 94px header rail.
 from pathlib import Path
 Image.open(Path(__file__).resolve().parents[1]/'assets/header-collage-transparent.png').convert('RGBA').resize((318,126),Image.Resampling.LANCZOS).save(out/'header-donut-runtime.png')
 at=next(i for i,c in enumerate(children) if get(cd(c),'name')=='Edit all chords')
 children.insert(at,image('Header donut','header-donut-runtime.png','962 0 318 126'))

 # Compact header controls with matching artwork AND caption/hit bounds.
 header={'SoundPrev':(350,38,36,34),'SoundPreset':(392,38,240,34),'SoundNext':(638,38,36,34),'SOUND PRESET':(392,9,240,26),'VoiceMode':(686,38,104,32),'VOICE':(686,9,104,26),'InputMode':(802,38,104,32),'INPUT':(802,9,104,26)}
 for c in children:
  name=get(cd(c),'name')
  if name not in header:continue
  x,y,w,h=header[name];bounds(c,f'{x} {y} {w} {h}')
  if name not in ('SoundPreset','VoiceMode','InputMode'):continue
  file=f'header-{name}.png';art(file,w,h)
  for widget in get(defs[get(cd(c),'type')],'componentsData'):
   typ=get(cd(widget),'type');data=get(cd(widget),'data')
   bounds(widget,f'0 0 {w} {h}')
   if typ=='Label':style(data,24)
   elif typ=='Button' and get(data,'offImage')!='button-hit.png':put(data,'offImage',file);put(data,'onImage',file)

 # Reserve the protruding art area; section heading stays fully to its left.
 for c in children:
  if get(cd(c),'name').startswith('B  '):
   for widget in get(defs[get(cd(c),'type')],'componentsData'):
    if get(cd(widget),'type')=='Label':bounds(widget,'50 0 250 29')

 # Cream header labels contrast with the chocolate paper; LCD text stays dark.
 for c in children:
  if get(cd(c),'name') in ('SOUND PRESET','VOICE','INPUT'):
   put(get(get(cd(c),'data'),'textStyle'),'colour','fff4dfb5')

 # Transparent PNG corners must reveal the header, not legacy widget fills.
 for c in children:
  if get(cd(c),'name') in ('SoundPrev','SoundNext','SoundPreset','VoiceMode','InputMode'):
   put(defs[get(cd(c),'type')],'backgroundData',g.background_data('00000000'))
