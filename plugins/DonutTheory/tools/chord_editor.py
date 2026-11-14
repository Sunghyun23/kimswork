"""One all-slot editor; explicit MIDI notes and learn, stable existing parameter IDs."""
from PIL import Image, ImageDraw

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 pd=get(tui,'pageData');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions')
 library={get(d,'key'):get(d,'value') for d in defs};by={p.name:p for p in params}
 def h(name):return by[name].handle
 def label(s,b,size=28,kind='Text',handle='Parameter 0'):
  return g.entry(s,'Label',g.label_data(kind,'ff352d21',size,'horizontallyCentred verticallyCentred',s if kind=='Text' else None),b,'No','Show','Always',remap=[] if handle=='Data' else [('Data',handle)])
 def art(file,w,on):
  im=Image.new('RGBA',(w,44));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,w-2,42),6,fill='#b06f37' if on else '#dbc594',outline='#79552c',width=2);d.line((7,4,w-8,4),fill='#ffe6b6');im.save(out/file)
 for w in (60,150,180,220,300):
  for suffix,on in [('off',False),('on',True)]:art(f'chord-{w}-{suffix}.png',w,on)
 def button(name,handle,x,y,w=180,index=1,count=2):
  return g.entry(name,'Button',g.button_data(f'chord-{w}-on.png',f'chord-{w}-off.png',index,count),f'{x} {y} {w} 44','Yes','Show','Always',remap=[('Data',handle)])
 def rotary(name,handle,x,y):
  data=O(('version',4),('knobType','RotatingImage'),('foregroundImage','knob-donut.png'),('backgroundImage',''),('imageScale',1.0),('dragOrientation','Vertical'),('handleName','Data'))
  definition=g.component_definition(g.control_actions('knob overlay'),[g.entry('knob','Knob',data,'45 0 56 56','No','Show','Always'),label('','0 61 146 27',26,'Value','Data')],'00000000')
  defs.append(O(('key',name),('value',definition)))
  return g.entry(name,name,O(('version',1),('handleName','Data')),f'{x} {y} 146 90','Yes','Hide','Always',remap=[('Data',handle)])
 local='Local/2/dtAllChords'
 # MAIN has one editor entry; remove separate EDIT buttons while retaining play pads.
 children=get(get(get(pd,'tabs')[0],'componentDefinition'),'componentsData')
 children[:]=[c for c in children if not get(get(c,'componentData'),'name').startswith('Edit chord ')]
 definition=g.component_definition([g.action('Mouse Down','Toggle Switch',local,'')],[label('EDIT ALL','0 0 110 28',22)],'ffd6c298')
 defs.append(O(('key','DTEditAll'),('value',definition)))
 children.append(g.entry('Edit all chords','DTEditAll',O(('version',1),('handleName','Data')),'214 550 110 28','Yes','Show','Always',remap=[('Data','Parameter 0')]))
 ws=[label('ALL CHORDS — ROOT / TYPE / VOICING / PAD','20 8 970 40',30),button('Done',local,1070,8,index=0),label('DONE','1070 8 180 44'),label('LEARN: tap a row, press its physical pad, then press again to play.','20 49 970 28',23)]
 for title,x,w in [('SLOT',16,74),('ROOT',100,146),('CHORD TYPE',260,300),('VOICING',575,220),('MIDI NOTE',815,146),('PAD LEARN',1010,180)]:ws.append(label(title,f'{x} 81 {w} 28',25))
 for i in range(4):
  slot=i+1;y=116+i*100
  ws.extend([label(str(slot),f'16 {y+17} 74 44',36),rotary('DTChordRoot'+str(slot),h(f'Chord {slot} Root'),100,y),rotary('DTChordNote'+str(slot),h(f'Pad {slot} Note'),815,y)])
  for name,x,w in [(f'Chord {slot} Type',260,300),(f'Chord {slot} Voice',575,220)]:
   param=by[name];key=f'Local/2/dtChoice{param.index}'
   ws.extend([button(name,key,x,y+17,w),label('',f'{x} {y+17} {w} 44',27,'Value',param.handle)])
  ws.extend([button('Learn '+str(slot),h('Pad Learn'),1010,y+17,index=slot,count=5),label('LEARN '+str(slot),f'1010 {y+17} 180 44',27)])
 ws.append(label('INPUT MODE','20 528 568 30',24))
 for j,title in enumerate(('NOTES','CHORDS','SPLIT')):
  x=20+j*194;ws.extend([button(title,h('Input Mode'),x,564,index=j,count=3),label(title,f'{x} 564 180 44',25)])
 ws.append(label('PAD ROUTING','610 528 375 30',24))
 for j,title in enumerate(('LEGACY BASE','PAD NOTES')):
  x=610+j*194;ws.extend([button(title,h('Pad Mapping'),x,564,index=j,count=2),label(title,f'{x} 564 180 44',25)])
 ws.extend([button('Cancel learn',h('Pad Learn'),1010,564,index=0,count=5),label('CANCEL LEARN','1010 564 180 44',24)])
 defs.append(O(('key','DTAllChords'),('value',g.component_definition([],ws,'ffe1d2ae'))))
 children.append(g.entry('DTAllChords','DTAllChords',O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data','Parameter 0')],invalidating=[f'IndexedEnabling/1/2/{local}']))

def finish(g,tui,params):
 """Expose the existing choice-list definitions above the all-slot MAIN editor."""
 def get(o,k):return next(v for a,v in o if a==k)
 pd=get(tui,'pageData');children=get(get(get(pd,'tabs')[0],'componentDefinition'),'componentsData')
 for p in params:
  if p.name not in {f'Chord {i} {suffix}' for i in range(1,5) for suffix in ('Type','Voice')}:continue
  local=f'Local/2/dtChoice{p.index}'
  children.append(g.entry('Select '+p.name,'DTChoicePanel'+str(p.index),g.obj(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data',p.handle)],invalidating=[f'IndexedEnabling/1/2/{local}']))
