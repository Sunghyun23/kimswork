"""Left-side choice drawers; keep the current plugin page visible behind them."""
from PIL import Image,ImageDraw

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(c):return get(c,'componentData')
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(d,'key'):get(d,'value') for d in entries}
 by={p.index:p for p in params};named={p.name:p for p in params};W=426;ROW=58;PER_PAGE=8
 for state,colour in [('off','#171719'),('on','#59595e')]:
  im=Image.new('RGBA',(W,ROW),colour);d=ImageDraw.Draw(im);d.line((0,ROW-1,W,ROW-1),fill='#353538')
  if state=='on':d.line((386,29,392,35,405,21),fill='#ffffff',width=3)
  im.save(out/f'sidebar-row-{state}.png')
 for name,size,colour in [('sidebar-panel.png',(W,628),'#171719'),('sidebar-header.png',(W,58),'#303035')]:Image.new('RGBA',size,colour).save(out/name)
 def image(name,file,b):return g.entry(name,'Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',file)),b,'No','Show','Always')
 def label(text,b,size=30,align='left verticallyCentred'):
  data=g.label_data('Text','fff2f2f2',size,align,text);f=get(get(data,'textStyle'),'font');put(f,'name','Roboto');put(f,'style','Regular')
  return g.entry(text,'Label',data,b,'No','Show','Always',remap=[('Data','Parameter 0')])
 def hit(name,handle,b,index,count):return g.entry(name,'Button',g.button_data('button-hit.png','button-hit.png',index,count),b,'Yes','Show','Always',remap=[('Data',handle)])
 def text_button(text,handle,b,index,count,size=26):return [label(text,b,size,'horizontallyCentred verticallyCentred'),hit(text+' touch',handle,b,index,count)]
 def add(key,comp):entries.append(O(('key',key),('value',comp)));defs[key]=comp
 def instance(key,b='0 0 1280 628',invalid=None):return g.entry(key,key,O(('version',1),('handleName','Data')),b,'Children','Hide' if invalid else 'Show','Always',remap=[('Data','Parameter 0')],invalidating=invalid)
 def row(key,text,handle,index,count,opened=None):
  # The exact-value Button writes first; the containing row closes the drawer.
  # Both actions are native Instant/Mouse Down, with no DSP-side UI state.
  ws=[g.entry('selection','Button',g.button_data('sidebar-row-on.png','sidebar-row-off.png',index,count),f'0 0 {W} {ROW}','No','Show','Always',remap=[('Data',handle)]),label(text,f'24 0 {W-82} {ROW}'),hit('selection touch',handle,f'0 0 {W} {ROW}',index,count)]
  actions=[g.action('Mouse Down','Toggle Switch',opened,'')] if opened else []
  add(key,g.component_definition(actions,ws,'00000000'))
  return key
 def body(p,choices,opened,key,page_handle=None):
  pages=(len(choices)+PER_PAGE-1)//PER_PAGE;page_handle=page_handle or f'Local/{max(2,pages)}/sidebarPage{p.index}{key}'
  chunks=[]
  for page in range(pages):
   ws=[]
   for position,(index,text) in enumerate(choices[page*PER_PAGE:(page+1)*PER_PAGE]):
    rk=f'DTSideRow{p.index}-{index}'
    if rk not in defs:row(rk,text,p.handle,index,len(p.choices),opened)
    ws.append(instance(rk,f'0 {64+position*ROW} {W} {ROW}'))
   if pages>1:
    ws+=[image('page footer','sidebar-header.png',f'0 570 {W} 58'),label(f'{page+1} / {pages}','168 570 90 52',24,'horizontallyCentred verticallyCentred')]
    if page>0:ws+=text_button('‹ PREV',page_handle,'8 570 144 52',page-1,max(2,pages))
    if page+1<pages:ws+=text_button('NEXT ›',page_handle,'274 570 144 52',page+1,max(2,pages))
   ck=f'DTSideRows{p.index}-{key}-{page}';add(ck,g.component_definition([],ws,'00000000',ignore_mouse=True));chunks.append(instance(ck,invalid=[f'IndexedEnabling/{page}/{max(2,pages)}/{page_handle}'] if pages>1 else None))
  return chunks
 def base(title,opened,back=None):
  ws=[image('left sidebar','sidebar-panel.png',f'0 0 {W} 628'),hit('Dismiss outside',opened,f'{W} 0 {1280-W} 628',0,2),image('sidebar heading','sidebar-header.png',f'0 0 {W} 58')]
  ws.append(label(title,f'{68 if back else 24} 0 {W-(126 if back else 82)} 58',29))
  if back:ws+=text_button('‹',back[0],'0 0 58 58',0,back[1],36)
  ws+=text_button('×',opened,f'{W-58} 0 58 58',0,2,36)
  return ws
 def modal(ws):
  comp=g.component_definition([],ws,'90000000');put(comp,'version',4);pos=next(i for i,(k,_) in enumerate(comp) if k=='componentsData');comp.insert(pos,('hideQLinkBounds',True));return comp
 panels=[(key,value) for key,value in list(defs.items()) if key.startswith('DTChoicePanel')]
 for key,comp in panels:
  p=by[int(key[len('DTChoicePanel'):])];opened=f'Local/2/dtChoice{p.index}'
  if p.name=='Sound Preset':
   cat=f'Local/7/soundCategory{p.index}';ws=[];categories=['Init','Bass','Keys','Synth','Lead','Pad']
   first=base('SOUND PRESET',opened)
   for ci,name in enumerate(categories,1):
    count=sum(s.startswith(name+' / ') for s in p.choices);rk=f'DTSideCategory{ci}';row(rk,f'{name.upper()}  ({count})',cat,ci,7);first.append(instance(rk,f'0 {64+(ci-1)*ROW} {W} {ROW}'))
   firstkey='DTSideSoundCategories';add(firstkey,g.component_definition([],first,'00000000',ignore_mouse=True));ws.append(instance(firstkey,invalid=[f'IndexedEnabling/0/7/{cat}']))
   for ci,name in enumerate(categories,1):
    choices=[(i,s.split(' / ',1)[1]) for i,s in enumerate(p.choices) if s.startswith(name+' / ')]
    cw=base(name.upper(),opened,(cat,7))+body(p,choices,opened,'sound'+str(ci));ck=f'DTSideSoundCategory{ci}';add(ck,g.component_definition([],cw,'00000000',ignore_mouse=True));ws.append(instance(ck,invalid=[f'IndexedEnabling/{ci}/7/{cat}']))
   # Common pane blocks the old page, category layers themselves pass empty touches.
   comp[:]=modal([image('left sidebar','sidebar-panel.png',f'0 0 {W} 628'),hit('Dismiss outside',opened,f'{W} 0 {1280-W} 628',0,2)]+ws)
  elif p.name.startswith('Chord ') and p.name.endswith(' Voice'):
   slot=int(p.name.split()[1]);quality=named[f'Chord {slot} Type'];ws=base(f'CHORD {slot} VOICING',opened)
   for q,count in enumerate((4,4,5,5,4,5,5,3,3,3,5)):
    choices=[(0,'Root position'),(1,'1st inversion'),(3,'2nd inversion')]
    if count>=4:choices.append((4,'3rd inversion'))
    if count>=5:choices.append((5,'4th inversion'))
    choices.append((2,'Open voicing'))
    ck=f'DTSideVoicings{slot}-{q}';add(ck,g.component_definition([],body(p,choices,opened,f'voice{slot}-{q}'),'00000000',ignore_mouse=True));ws.append(instance(ck,invalid=[f'IndexedEnabling/{q}/11/{quality.handle}']))
   comp[:]=modal(ws)
  else:comp[:]=modal(base(p.name.upper(),opened)+body(p,list(enumerate(p.choices)),opened,'choices'))
 # Remove the temporary native MenuOverlay probe; retained selectors have their artwork.

 # Arrow functions never open an Off/On menu. Discrete knobs open their list
 # with a single value-plate tap as well as the existing double-tap gesture.
 for key in ('DTPrev','DTNext','DTCPrev','DTCNext'):
  if key in defs:put(defs[key],'actions',[a for a in get(defs[key],'actions') if get(a,'handler')!='Show Overlay'])
 for key,definition in list(defs.items()):
  if key.startswith('DTChoiceKnob-choice-'):
   index=int(key.rsplit('-',1)[1]);get(definition,'componentsData').append(hit('Open value list touch',f'Local/2/dtChoice{index}','0 116 146 28',1,2))

 # Every touch entry into the sound browser starts at categories, including
 # the detail page value label. The child writes category 0; its native parent
 # Mouse Down opens the drawer, using the same ordering as selection+close rows.
 sound=named['Sound Preset'];opened=f'Local/2/dtChoice{sound.index}';cat=f'Local/7/soundCategory{sound.index}'
 for key,definition in list(defs.items()):
  if not key.endswith('-choice-'+str(sound.index)):continue
  if key.startswith('DTMenu'):
   w,h=240,34;x,y=0,0
  elif key.startswith('slider'):
   x,y,w,h=15,70,248,30
  else:continue
  launch='DTSoundLaunchCategory'+key
  add(launch,g.component_definition([g.action('Mouse Down','Toggle Switch',opened,'')],[hit('Reset sound category touch',cat,f'0 0 {w} {h}',0,7)],'00000000'))
  get(definition,'componentsData').append(instance(launch,f'{x} {y} {w} {h}'))
