"""Interaction cues and performance-oriented MAIN/editor refinements."""
import copy
from PIL import Image,ImageDraw

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(c):return get(c,'componentData')
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(d,'key'):get(d,'value') for d in entries};by={p.name:p for p in params}
 main=get(get(pd,'tabs')[0],'componentDefinition');children=get(main,'componentsData')
 def label(text,b,size=23,handle=None,invalidating=None):
  return g.entry(text,'Label',g.label_data('Value' if handle else 'Text','ff352d21',size,'horizontallyCentred verticallyCentred',None if handle else text),b,'No','Hide' if invalidating else 'Show','Always',remap=[('Data',handle or 'Parameter 0')],invalidating=invalidating)
 def define(name,widgets,actions=None):
  value=g.component_definition(actions or [],widgets,'ffe1d2ae');entries.append(O(('key',name),('value',value)));defs[name]=value
 def overlay(name,widgets,local):
  define(name,widgets)
  children.append(g.entry(name,name,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data','Parameter 0')],invalidating=[f'IndexedEnabling/1/2/{local}']))
 def button(text,handle,b,index=1,count=2):
  x,y,w,h=map(int,b.split());file=f'usability-{w}-{h}.png'
  im=Image.new('RGBA',(w,h));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,w-2,h-2),5,fill='#dbc594',outline='#79552c',width=2)
  if text in ('D · LFO SETTINGS','ADVANCED','CHANGE'):d.polygon([(w-17,h//2-3),(w-7,h//2-3),(w-12,h//2+3)],fill='#352d21')
  im.save(out/file)
  active='active-'+file;d.rounded_rectangle((1,1,w-2,h-2),5,fill='#b06f37',outline='#79552c',width=2);im.save(out/active)
  return [g.entry(text,'Button',g.button_data(active,file,index,count),b,'Yes','Show','Always',remap=[('Data',handle)]),label(text,b),g.entry(text+' touch','Button',g.button_data('button-hit.png','button-hit.png',index,count),b,'Yes','Show','Always',remap=[('Data',handle)])]
 def open_control(name,text,b,local):
  define(name,button(text,local,'0 0 '+ ' '.join(b.split()[2:])),[])
  children.append(g.entry(name,name,O(('version',1),('handleName','Data')),b,'Yes','Show','Always',remap=[('Data','Parameter 0')]))
 # Dropdown cues are part of menu artwork, so no caption can intercept touches.
 for file in ['menu-chord.png','menu-small.png','menu-wide.png','header-SoundPreset.png','header-VoiceMode.png','header-InputMode.png']:
  im=Image.open(out/file).convert('RGBA');d=ImageDraw.Draw(im);w,h=im.size
  d.polygon([(w-15,h//2-2),(w-7,h//2-2),(w-11,h//2+3)],fill='#352d21');im.save(out/file)
 # Reserve a little room for the dropdown cue without changing any hit surface.
 for name,comp in list(defs.items()):
  if name.startswith(('DTMenu-choice','DTSmallMenu-choice','DTSelect')):
   for c in get(comp,'componentsData'):
    if get(cd(c),'type')=='Label' and get(get(cd(c),'data'),'type')=='Value':
     x,y,w,h=map(int,get(get(c,'bounds'),'bounds').split());put(get(c,'bounds'),'bounds',f'{x} {y} {w-18} {h}')
 for name,comp in list(defs.items()):
  if name.startswith('DTSelect'):
   ws=get(comp,'componentsData');is_filter=any(get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('text')=='FLT' for w in ws)
   if is_filter:ws[:]=[w for w in ws if not(get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('text')=='FLT')]
   for w in ws:
    if get(cd(w),'type')=='Label' and get(get(cd(w),'data'),'type')=='Value':
     put(get(get(get(cd(w),'data'),'textStyle'),'font'),'height',24.0)
     if is_filter:put(get(w,'bounds'),'bounds','2 0 116 32')
 # One clear performance strip; safety control lives in Advanced.
 children[:]=[c for c in children if get(cd(c),'name')!='Stop touch chords']
 for c in children:
  if get(cd(c),'name')=='CHORD PRESET':put(get(c,'bounds'),'bounds','18 548 190 28')
  if get(cd(c),'name') in ('Sync','Division'):children.remove(c);break
 children[:]=[c for c in children if get(cd(c),'name') not in ('Sync','Division')]
 for w in get(defs['DTEditAll'],'componentsData'):
  if get(cd(w),'type')=='Label':put(get(cd(w),'data'),'text','EDIT CHORDS');put(get(get(get(cd(w),'data'),'textStyle'),'font'),'height',18.0)
 # Hold cues live inside each pad, below its transparent gesture surface.
 for i in range(4):
  for c in children:
   if get(cd(c),'name')=='Chord '+str(i+1):put(get(c,'bounds'),'bounds',f'{348+i*230} 550 222 70')
  pad=defs['DTPad'+str(i)]
  for w in get(pad,'componentsData'):
   if get(cd(w),'type') in ('Button','Slider'):put(get(w,'bounds'),'bounds','0 0 222 70')
   if get(cd(w),'type')=='Button':
    data=get(cd(w),'data')
    if get(data,'offImage')!='button-hit.png':
     for k in ('onImage','offImage'):
      old=get(data,k);new='wide-'+old;Image.open(out/old).resize((222,70),Image.Resampling.LANCZOS).save(out/new);put(data,k,new)
  # Numbers are part of the existing caption; hold gesture remains topmost.
  for w in get(defs['DTPad'+str(i)],'componentsData'):
   if get(cd(w),'type')=='Label':put(get(w,'bounds'),'bounds','0 14 222 56')
  get(defs['DTPad'+str(i)],'componentsData').insert(1,label(str(i+1),'8 0 28 20',17))
 # Filter/LFO and FX/Tremolo use the same four-control grouping as Q-Link.
 # Shape/Target stay with the filter; Sync/Division remain on MOD.
 # Mode and waveform sit above the two motion knobs on the right.
 # Their values identify them; avoid cramped captions between dropdowns.
 # Explanations stay inside the relevant selection screens.
 hints={'Voice Mode':'MONO / LEGATO: single-note melody in NOTES and SPLIT. POLY: multiple notes. Chord pads always play full chords.',
 'Input Mode':'NOTES: play notes. CHORDS: assigned notes play chords. SPLIT: assigned chords plus melody notes.'}
 for name,text in hints.items():get(defs['DTChoicePanel'+str(by[name].index)],'componentsData').append(label(text,'30 540 1220 60',22))
 # Code-native root labels: existing chord-name parameters and state stay intact.
 editor=get(defs['DTAllChords'],'componentsData')
 editor[:]=[w for w in editor if get(cd(w),'name') not in ('1','2','3','4','PAD ROUTING','LEGACY BASE','PAD NOTES') and not(get(cd(w),'type')=='Button' and get(cd(w),'name') in ('LEGACY BASE touch','PAD NOTES touch'))]
 for w in editor:
  if get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('text')=='SLOT':put(get(cd(w),'data'),'text','PLAY')
  if get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('text')=='LEARN: tap a row, press its physical pad, then press again to play.':
   put(cd(w),'data',g.label_data('Value','ff352d21',23,'horizontallyCentred verticallyCentred'));put(w,'handle remapping',O(('version',1),('map',[O(('key','Data'),('value',by['Pad Learn Status'].handle))])))
 for i in range(4):
  slot=i+1;y=116+i*100;root=by[f'Chord {slot} Root']
  comp=defs['DTChordRoot'+str(slot)];ws=get(comp,'componentsData');ws[:]=[w for w in ws if get(cd(w),'type')!='Label']
  for n in range(24,85):
   note=('C','C#','D','Eb','E','F','F#','G','Ab','A','Bb','B')[n%12]+str(n//12-2)
   ws.append(label(note,'0 61 146 27',26,invalidating=[f'IndexedEnabling/{n-24}/61/{root.handle}']))
  # A miniature version of the proven hold pad provides per-slot audition.
  pad=copy.deepcopy(defs['DTPad'+str(i)]);put(pad,'backgroundData',g.background_data('00000000'))
  for w in get(pad,'componentsData'):
   put(get(w,'bounds'),'bounds','0 0 90 64')
   data=get(cd(w),'data')
   if get(cd(w),'type')=='Button' and get(data,'offImage')!='button-hit.png':
    for k in ('onImage','offImage'):
     f=get(data,k);new='mini-'+f;Image.open(out/f).resize((90,64),Image.Resampling.LANCZOS).save(out/new);put(data,k,new)
   if get(cd(w),'type')=='Label':
    if get(data,'type')=='Text':put(data,'text',str(slot));put(get(w,'bounds'),'bounds','0 0 90 18')
    else:put(get(w,'bounds'),'bounds','0 18 90 46');put(get(get(data,'textStyle'),'font'),'height',22.0)
  name='DTAudition'+str(slot);entries.append(O(('key',name),('value',pad)))
  editor.append(g.entry('Audition '+str(slot),name,O(('version',1),('handleName','Data')),f'5 {y+8} 90 64','Yes','Show','Always',remap=[('Data','Parameter 0')]))
 advanced='Local/2/dtChordAdvanced'
 editor.extend(button('ADVANCED',advanced,'610 564 375 44'))
 aw=[label('ADVANCED — PAD ROUTING & SAFETY','20 20 950 44',30)]+button('DONE',advanced,'1070 20 180 44',0)
 aw += [label('INDIVIDUAL: assign each pad with LEARN. CONSECUTIVE: compatibility with older projects.','30 110 1220 60',24)]
 for j,text in enumerate(('CONSECUTIVE','INDIVIDUAL')):aw+=button(text,by['Pad Mapping'].handle,f'{100+j*560} 220 460 64',j,2)
 aw+=button('ALL NOTES OFF',by['Stop Chords'].handle,'400 400 460 64')
 aw.append(label('Stops keyboard, physical pad and screen notes, including sustain.','30 500 1220 50',25))
 overlay('DTChordAdvanced',aw,advanced)
 # Explicit relationship between Space and the available detailed FX controls.
 fx=get(get(pd,'tabs')[5],'componentDefinition')
 # FX now uses all sixteen cells; keep explanatory text in the manual.
 # Use the same typeface as the restored interface.
 def bold(o):
  if isinstance(o,g.Obj):
   if 'textStyle' in dict(o):
    font=get(get(o,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold')
   for _,v in o:bold(v)
  elif isinstance(o,list):
   for v in o:bold(v)
 bold(tui)
 for c in children:
  if get(cd(c),'name') in ('D · LFO','TREMOLO'):put(get(get(cd(c),'data'),'textStyle'),'colour','fff4d7a3')
 # Selection panels must remain on top of new settings overlays.
 panels=[c for c in children if get(cd(c),'name').startswith('Select ')]
 children[:]=[c for c in children if c not in panels]+panels
