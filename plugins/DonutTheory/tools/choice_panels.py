"""Explicit choice panels for MPC 3.10.1 (native MenuOverlay is empty on device)."""
import copy

def install(g,tui,params,skin):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 pd=get(tui,'pageData');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions')
 library={get(v,'key'):get(v,'value') for v in defs}
 choices={p.handle:p for p in params if p.is_discrete and p.name not in ('SoundPrev','SoundNext','ChordPrev','ChordNext','Stop Chords') and not p.name.startswith('Play Chord')}
 def add(name,value):
  if name not in library:defs.append(O(('key',name),('value',value)));library[name]=value
 def caption(text,bounds,size=28,kind='Text',handle='Parameter 0'):
  return g.entry(text,'Label',g.label_data(kind,'ff352d21',size,'horizontallyCentred verticallyCentred',text if kind=='Text' else None),bounds,'No','Show','Always',remap=[] if handle=='Data' else [('Data',handle)])
 from PIL import Image,ImageDraw
 def art(name,w,h,colour):
  im=Image.new('RGBA',(w,h),colour);d=ImageDraw.Draw(im);d.rectangle((0,0,w-1,h-1),outline=(108,80,46),width=2);im.save(skin/name)
 for name,w,h,col in [('list-off.png',300,56,(215,194,148)),('list-on.png',300,56,(176,111,55)),('list-close.png',120,38,(215,194,148)),('menu-wide.png',300,34,(201,205,146)),('menu-small.png',136,32,(201,205,146))]:art(name,w,h,col)
 def button(name,handle,bounds,image,on=None,index=1,count=2):
  return g.entry(name,'Button',g.button_data(on or image,image,index,count),bounds,'Yes','Show','Always',remap=[('Data',handle)])
 for tab in get(pd,'tabs'):
  root=get(tab,'componentDefinition');children=get(root,'componentsData');panels=[];seen=set()
  for child in list(children):
   cd=get(child,'componentData');typ=get(cd,'type')
   remap=dict((get(r,'key'),get(r,'value')) for r in get(get(child,'handle remapping'),'map'))
   handle=remap.get('Data');p=choices.get(handle)
   if not p or typ not in library or typ.startswith('DTPad'):continue
   local=f'Local/2/dtChoice{p.index}';page=f'Local/2/dtChoicePage{p.index}'
   modified=copy.deepcopy(library[typ]);name=f'{typ}-choice-{p.index}'
   actions=[a for a in get(modified,'actions') if get(a,'handler')!='Show Overlay']
   actions += [g.action('Double Click','Toggle Switch',local,''),g.action('Enter Pressed','Toggle Switch',local,'')]
   put(modified,'actions',actions)
   if typ in ('DTMenu','DTSmallMenu') or typ.startswith('DTSelect'):
    w=300 if typ=='DTMenu' else 136;h=34 if w==300 else 32
    widgets=[button('open choices',local,f'0 0 {w} {h}','menu-wide.png' if w==300 else 'menu-small.png')]
    for item in get(modified,'componentsData'):
     data=get(item,'componentData')
     if get(data,'type')=='ComboBox':
      bounds=get(get(item,'bounds'),'bounds');widgets.append(caption('',bounds,28,'Value',handle))
     else:widgets.append(item)
    put(modified,'componentsData',widgets)
   add(name,modified);put(cd,'type',name)
   if handle in seen:continue
   seen.add(handle)
   panelName=f'DTChoicePanel{p.index}'
   panel=[caption(p.name.upper(),'20 8 950 42',30),button('Done',local,'1135 10 120 38','list-close.png',index=0),caption('DONE','1135 10 120 38',24)]
   n=len(p.choices)
   for part in range((n+31)//32):
    rows=[]
    for i in range(part*32,min(n,(part+1)*32)):
     j=i-part*32;x=20+(j%4)*310;y=64+(j//4)*63
     rows += [button(p.choices[i],handle,f'{x} {y} 300 56','list-off.png','list-on.png',i,n),caption(p.choices[i],f'{x+4} {y+4} 292 48',28)]
    sub=f'DTChoiceRows{p.index}-{part}';add(sub,g.component_definition([],rows,'00000000',ignore_mouse=True))
    panel.append(g.entry('choices',sub,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data',handle)],invalidating=[f'IndexedEnabling/{part}/2/{page}'] if n>32 else None))
   if n>32:
    for part,text in [(0,'1 – 32'),(1,'33 – 64')]:
     x=450+part*150;panel += [button('page',page,f'{x} 584 120 38','list-close.png',index=part),caption(text,f'{x} 584 120 38',22)]
   add(panelName,g.component_definition([],panel,'ffe1d2ae'))
   panels.append(g.entry('Select '+p.name,panelName,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data',handle)],invalidating=[f'IndexedEnabling/1/2/{local}']))
  # SDK radio captions are siblings of the radio component, so an internal
  # touch surface cannot cover them. Move captions into the per-parameter
  # definition, then the final hit surfaces will be above them too.
  for child in list(children):
   cd=get(child,'componentData');typ=get(cd,'type')
   if not typ.startswith('radio') or typ not in library:continue
   prefix=get(cd,'name')+' seg '
   x,y,_,_=map(int,get(get(child,'bounds'),'bounds').split())
   widgets=get(library[typ],'componentsData')
   for cap in list(children):
    data=get(cap,'componentData')
    if get(data,'type')!='Label' or not get(data,'name').startswith(prefix):continue
    cx,cy,w,h=map(int,get(get(cap,'bounds'),'bounds').split())
    put(get(cap,'bounds'),'bounds',f'{cx-x} {cy-y} {w} {h}')
    put(get(get(data,'data'),'textStyle'),'colour','fff3dfb8')
    widgets.append(cap);children.remove(cap)
  children.extend(panels)
 # Static labels in SDK-generated detail tabs were hidden because root Data is unbound.
 def repair(o):
  if isinstance(o,g.Obj):
   keys=dict(o)
   if keys.get('componentData'):
    cd=keys['componentData'];data=get(cd,'data')
    if get(cd,'type')=='Label' and dict(data).get('type')=='Text':
     put(get(o,'bounds'),'showWhenDataModelInvalid','Show')
     put(o,'handle remapping',O(('version',1),('map',[O(('key','Data'),('value','Parameter 0'))])))
    if get(cd,'type')=='Label' and dict(data).get('type') in ('Value','Name'):
     put(get(data,'textStyle'),'colour','ff352d21')
   for _,v in o:repair(v)
  elif isinstance(o,list):
   for v in o:repair(v)
 repair(tui)
 # GenericLegacy draws its own white value, ignoring Label textStyle.
 # Use a ColourBar plus explicit labels so the cream detail pages stay legible.
 for item in defs:
  if not get(item,'key').startswith('slider'):continue
  widgets=get(get(item,'value'),'componentsData')
  for widget in list(widgets):
   cd=get(widget,'componentData')
   if get(cd,'type')!='Slider':continue
   put(cd,'data',O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','ffcfbd95'),('outlineColour','ff6c502e'),('valueColour','ffb06f37'),('isBipolar',False),('handleName','Data')))
   put(get(widget,'bounds'),'bounds','15 43 248 20')
   widgets.extend([caption('','15 4 248 30',28,'Name','Data'),caption('','15 70 248 30',30,'Value','Data')])
 # On MPC Live III 3.10.1, a visible Label intercepts taps over its bounds.
 # Keep artwork and captions below a transparent native Button hit surface.
 # The same rule covers MAIN arrows, chord pads, open/close/page buttons,
 # and the SDK-generated detail-tab radio switches.
 Image.new('RGBA',(1,1),(0,0,0,0)).save(skin/'button-hit.png')
 for item in defs:
  component=get(item,'value');widgets=get(component,'componentsData')
  hits=[]
  for widget in widgets:
   if get(get(widget,'componentData'),'type')!='Button':continue
   hit=copy.deepcopy(widget);cd=get(hit,'componentData');data=get(cd,'data')
   put(cd,'name',get(cd,'name')+' touch')
   put(data,'onImage','button-hit.png');put(data,'offImage','button-hit.png')
   hits.append(hit)
  widgets.extend(hits)
