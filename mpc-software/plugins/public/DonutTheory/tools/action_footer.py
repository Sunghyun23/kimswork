"""MAIN-only action reflow; keep the approved art, parameter bindings and detail pages."""
import copy,math
from PIL import Image,ImageDraw

def install(g,tui,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_)in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(w):return get(w,'componentData')
 def bounds(w,b):put(get(w,'bounds'),'bounds',b)
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(e,'key'):get(e,'value')for e in entries};tab=get(pd,'tabs')[0];widgets=get(get(tab,'componentDefinition'),'componentsData')
 def clone(w,prefix):
  original=get(cd(w),'type');key=prefix+original
  if key not in defs:
   defs[key]=copy.deepcopy(defs[original]);entries.append(O(('key',key),('value',defs[key])))
  put(cd(w),'type',key);return defs[key]
 def resize_asset(old,new,size):
  Image.open(out/old).convert('RGBA').resize(size,Image.Resampling.LANCZOS).save(out/new)
 # Preserve the entire original header collage, font families and untouched detail controls.
 panel=Image.open(out/'group-panel.png').convert('RGBA');compact=Image.new('RGBA',(612,176))
 compact.paste(panel.crop((0,0,612,30)),(0,0));compact.paste(panel.crop((0,30,612,202)).resize((612,130),Image.Resampling.LANCZOS),(0,30));compact.paste(panel.crop((0,202,612,218)),(0,160));compact.save(out/'footer-group-panel.png')
 lcd=Image.open(out/'value-lcd-v2.png').convert('RGBA').resize((138,24),Image.Resampling.LANCZOS);plate=Image.new('RGBA',(146,116));plate.alpha_composite(lcd,(4,92));plate.save(out/'footer-value-panel.png')
 ticks=Image.new('RGBA',(146,74));draw=ImageDraw.Draw(ticks)
 for j in range(13):
  a=math.radians(135+j*22.5);cx,cy=73,36;draw.line((cx+38*math.cos(a),cy+38*math.sin(a),cx+43*math.cos(a),cy+43*math.sin(a)),fill='#372511',width=2)
 ticks.save(out/'footer-knob-ticks.png')
 keys=[['OscPair','Mix','Detune','Drift'],['Cutoff','Resonance','BodyTone','Sub'],['Rate','Depth','Lfo2RateControl','Lfo2Depth'],['Drive','Glide','TremRate','TremDepth']]
 controls={n:(26+(632 if i%2 else 0)+j*150,151+(184 if i>=2 else 0))for i,group in enumerate(keys)for j,n in enumerate(group)}
 for w in widgets:
  name=get(cd(w),'name')
  if name.startswith(('A  ','B  ','C  ','D  ')):
   letter=name[0];x=18 if letter in'AC'else 650;y=94 if letter in'AB'else 278;bounds(w,f'{x} {y} 612 176');d=clone(w,'Footer_')
   for child in get(d,'componentsData'):
    if get(cd(child),'type')=='Image':put(get(cd(child),'data'),'image','footer-group-panel.png');bounds(child,'0 0 612 176')
  if name in controls:
   x,y=controls[name];bounds(w,f'{x} {y} 146 116');key='Footer_'+get(cd(w),'type');fresh=key not in defs;d=clone(w,'Footer_')
   if not fresh:continue
   for child in get(d,'componentsData'):
    typ=get(cd(child),'type');data=get(cd(child),'data');n=get(cd(child),'name')
    if typ=='Knob':bounds(child,'37 0 72 72')
    elif typ=='Image':
     if n=='ticks':bounds(child,'0 0 146 74');put(data,'image','footer-knob-ticks.png')
     elif n=='value plate':bounds(child,'0 0 146 116');put(data,'image','footer-value-panel.png')
    elif typ=='Label':
     isname=dict(data).get('type')=='Name' or dict(data).get('text')=='RATE'
     bounds(child,('0 72 66 20'if dict(data).get('text')=='RATE'else'0 72 146 20')if isname else'12 92 122 24')
     put(get(get(data,'textStyle'),'font'),'height',22.0 if isname else 24.0)
 # Local parameter selectors remain next to their knobs; only global browse/actions move.
 positions={'Filter':(662,124,136,26),'Vibrato':(26,124,286,26),'Expression':(332,124,286,26),'DriveType':(662,308,136,26),'Shape':(26,308,136,26),'Target':(176,308,136,26),'Lfo2Shape':(326,308,136,26),'Lfo2Target':(476,308,136,26),'ModMode':(964,308,136,26),'ModShape':(1114,308,136,26),
 'SoundPrev':(304,456,44,44),'SoundPreset':(354,456,240,44),'SoundNext':(600,456,44,44),'SOUND PRESET':(142,456,156,44),'VOICE':(658,456,86,44),'VoiceMode':(750,456,180,44),'INPUT':(942,456,96,44),'InputMode':(1044,456,218,44),
 'CHORD PRESET':(406,502,154,44),'ChordPrev':(568,502,44,44),'Progression':(618,502,244,44),'ChordNext':(868,502,44,44),'Random sound':(148,502,240,44),
 'Filter / LFO divider':(320,335,2,116),'FX / Tremolo divider':(952,335,2,116)}
 for w in widgets:
  name=get(cd(w),'name')
  if name not in positions:continue
  x,y,width,height=positions[name];bounds(w,f'{x} {y} {width} {height}');typ=get(cd(w),'type')
  if typ=='Label':
   data=get(cd(w),'data');put(get(data,'textStyle'),'colour','ff352d21');put(get(get(data,'textStyle'),'font'),'height',20.0 if name=='CHORD PRESET'else 22.0);continue
  if name=='Random sound':continue
  if typ not in defs or name.endswith('divider'):continue
  d=clone(w,'Footer_'+name+'_')
  for child in get(d,'componentsData'):
   data=get(cd(child),'data');kind=get(cd(child),'type');bx,by,bw,bh=map(int,get(get(child,'bounds'),'bounds').split())
   if height==26:
    childh=max(1,round(bh*26/32));bounds(child,f'{bx} {round(by*26/32)} {bw} {childh}')
    if kind=='Label'and dict(data).get('type')!='Text':put(get(get(data,'textStyle'),'font'),'height',22.0)
   else:
    childh=height;bounds(child,f'{bx} 0 {width-bx*2 if kind=="Label"and bx else width-(18 if kind=="Label"and name in("SoundPreset","VoiceMode","InputMode","Progression")else 0)} {height}')
   if kind in('Image','Button'):
    for field in (['image']if kind=='Image'else['onImage','offImage']):
     old=get(data,field)
     if old=='button-hit.png':continue
     new=f'footer-{name}-{field}-{old}';resize_asset(old,new,(bw if height==26 else width,childh));put(data,field,new)
 # SYNC is still adjacent to Rate, with its original shape/colour and proven gesture.
 for name,key in [('DTTempoSyncRate','Rate'),('DTTempoSyncLfo2RateControl','Lfo2RateControl'),('DTTempoSyncTremRate','TremRate')]:
  w=next(w for w in widgets if get(cd(w),'name')==name);x,y=controls[key];bounds(w,f'{x+68} {y+71} 74 24');d=clone(w,'Footer_')
  for child in get(d,'componentsData'):
   bounds(child,'0 0 74 24');data=get(cd(child),'data');typ=get(cd(child),'type')
   if typ=='Label':put(get(get(data,'textStyle'),'font'),'height',18.0)
   if typ=='Button':
    for field in ['onImage','offImage']:
     old=get(data,field);new='footer-'+old;resize_asset(old,new,(74,24));put(data,field,new)
 # Same 16 active/page-conditional hold pads; preserve Mouse Down and top release surface.
 for w in widgets:
  name=get(cd(w),'name')
  if name.startswith('Chord ')and name[6:].isdigit():
   slot=int(name[6:])-1;bounds(w,f'{18+slot%4*313} 550 305 70');d=clone(w,'Footer_')
   for child in get(d,'componentsData'):
    kind=get(cd(child),'type');data=get(cd(child),'data');bx,by,bw,bh=map(int,get(get(child,'bounds'),'bounds').split())
    if kind in('Button','Slider'):bounds(child,'0 0 305 70')
    elif kind=='Label'and dict(data).get('type')!='Text':bounds(child,f'0 {by} 305 {bh}')
    if kind=='Button':
     for field in ['onImage','offImage']:
      old=get(data,field)
      if old=='button-hit.png':continue
      new='footer-'+old;resize_asset(old,new,(305,70));put(data,field,new)
  elif name in('‹','‹ touch','›','› touch'):
   x=1032 if name.startswith('‹')else 1172;bounds(w,f'{x} 502 44 44')
   if get(cd(w),'type')=='Button':
    data=get(cd(w),'data')
    for field in ['onImage','offImage']:
     old=get(data,field)
     if old=='button-hit.png':continue
     new='footer-'+old;resize_asset(old,new,(44,44));put(data,field,new)
  elif' / 'in name and '–'in name:bounds(w,'1082 502 84 44')
 # Retire the redundant miniature bank numbers, keeping the live page caption and arrows.
 widgets[:]=[w for w in widgets if not(get(cd(w),'name')in('1','2','3','4')and get(get(w,'bounds'),'bounds').endswith('16 12'))]
 for text,b in [('BROWSE','18 456 116 44'),('CREATE','18 502 116 44'),('SLOTS','940 502 84 44')]:
  data=g.label_data('Text','ff352d21',22,'horizontallyCentred verticallyCentred',text);put(get(get(data,'textStyle'),'font'),'name','Roboto');put(get(get(data,'textStyle'),'font'),'style','Bold');widgets.append(g.entry(text,'Label',data,b,'No','Show','Always',remap=[('Data','Parameter 0')]))
 # Shared generator command, original local press/release behaviour; no new parameter.
 w=next(w for w in widgets if get(cd(w),'name')=='Random sound');d=defs[get(cd(w),'type')]
 for active in [False,True]:
  im=Image.new('RGBA',(240,44));dr=ImageDraw.Draw(im)
  dr.rounded_rectangle((1,1,238,42),5,fill='#d88439'if active else'#b65a20',outline='#7b401e',width=2);dr.line((6,4,233,4),fill='#f1bc72');dr.line((6,39,233,39),fill='#713719');dr.rounded_rectangle((17,9,41,33),3,fill='#f3dfb4',outline='#39291c',width=1)
  for px,py in [(22,14),(36,14),(29,21),(22,28),(36,28)]:dr.ellipse((px-1,py-1,px+1,py+1),fill='#39291c')
  im.save(out/('random-on.png'if active else'random-off.png'))
 for child in get(d,'componentsData'):
  data=get(cd(child),'data');kind=get(cd(child),'type');bounds(child,'0 0 240 44')
  if kind=='Label':
   bounds(child,'50 0 182 44');put(data,'text','ROLL DICE');put(get(data,'textStyle'),'colour','fff4dfb5');f=get(get(data,'textStyle'),'font');put(f,'name','Roboto');put(f,'style','Bold');put(f,'height',26.0)
 put(tab,'qlinkBoundsData',['18 94 612 176','650 94 612 176','18 278 612 176','650 278 612 176'])
