"""Shared action geometry with Crate Press artwork and scoped Dice locks."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont

def install(g,tui,ql,out,params):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_)in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(w):return get(w,'componentData')
 def box(w):return list(map(int,get(get(w,'bounds'),'bounds').split()))
 def move(w,b):put(get(w,'bounds'),'bounds',' '.join(map(str,b)))
 P=out.parent;policy=json.loads((P.parents[2]/'shared/ui/action-footer.json').read_text());pd=get(tui,'pageData');tabs=get(pd,'tabs');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions');ids={p.name:p.index for p in params}
 Image.new('RGBA',(1,1),(0,0,0,0)).save(out/'cp-action-hit.png')
 targets=['Drive','Compress','Punch','Release','Low Comp','Mid Comp','High Comp','Duck','Tone','High Cut','Lo-Fi','Drift','Low Cut']
 def button(text,key,b,lock=False):
  x,y,w,h=b;files=[]
  for active in (0,1):
   file='dice-'+str(ids[key])+'-'+str(w)+'-'+str(h)+'-'+str(active)+'.png';im=Image.new('RGBA',(w,h),(0,0,0,0)) if lock else Image.new('RGB',(w,h),'#86501f'if active or text=='ROLL DICE'else'#332115');d=ImageDraw.Draw(im)
   if not lock:d.rounded_rectangle((1,1,w-2,h-2),6,outline='#edb36c'if active else'#ae7740',width=2)
   if lock:
    # Match Donut Theory's native glyph geometry within its44px touch surface.
    ink='#edb36c'if active else'#ae8856'
    if active:
     d.arc((15,5,29,23),180,360,fill=ink,width=2)
     d.line((15,13,15,20),fill=ink,width=2)
     d.line((29,13,29,20),fill=ink,width=2)
     d.rounded_rectangle((12,16,31,29),3,fill=ink)
     d.ellipse((21,20,23,22),fill='#fff0c5')
     d.line((22,22,22,25),fill='#fff0c5',width=2)
    else:
     d.arc((20,3,34,21),180,360,fill=ink,width=2)
     d.line((20,11,20,19),fill=ink,width=2)
     d.rounded_rectangle((12,16,31,29),3,outline=ink,width=2)
   else:
    size=18;font=ImageFont.truetype('/System/Library/Fonts/Helvetica.ttc',size)
    while font.getlength(text)>w-12:size-=1;font=ImageFont.truetype('/System/Library/Fonts/Helvetica.ttc',size)
    d.text((w/2,h/2),text,font=font,anchor='mm',fill='#ffe1a7')
   im.save(out/file);files.append(file)
  if not lock:
   hit_file=f'cp-action-hit-{w}x{h}.png';Image.new('RGBA',(w,h),(0,0,0,0)).save(out/hit_file)
   name='CPAction'+str(ids[key]);handle='Parameter '+str(ids[key]);press='Local/2/cpActionPress'+str(ids[key])
   children=[g.entry('action art','Button',g.button_data(files[1],files[0],1,1),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',handle)]),g.entry('action hit','Button',g.button_data(hit_file,hit_file,1,1),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',press)])]
   defs.append(O(('key',name),('value',g.component_definition([g.action('Mouse Down','Toggle Switch',handle,''),g.action('Mouse Down','Toggle Switch',press,'')],children,'00000000'))))
   return g.entry(text,name,O(('version',1),('handleName','Data')),' '.join(map(str,b)),'Children','Show','Always',remap=[('Data',handle)])
  return g.entry(text or key,'Button',g.button_data(files[1],files[0],1,1),' '.join(map(str,b)),'Yes','Show','Always',remap=[('Data','Parameter '+str(ids[key]))])
 for ti,tab in enumerate(tabs):
  ws=get(get(tab,'componentDefinition'),'componentsData')
  if ti==0:
   for w in ws:
    name=get(cd(w),'name');b=box(w)
    if name.startswith(('PerformHold','PerformLatch')):b[1]=488;move(w,b)
    elif name=='LATCH':move(w,[584,578,140,44])
    elif name=='CURVE':move(w,[736,578,88,44])
    elif name.startswith('curve '):i=['FULL','LOW','MID','HIGH'].index(name[6:]);move(w,[832+i*66,578,58,44])
    # Main controls retain their original size and Q-Link locations.
    binding={get(r,'key'):get(r,'value')for r in get(get(w,'handle remapping'),'map')}.get('Data','')
    bound=next((p.name for p in params if binding=='Parameter '+str(p.index)),None)
    if bound in targets:
     name=bound;x,y,_,_=box(w);ws.append(button('', 'Lock '+name,[x,y+24,44,44],True))
  else:
   for w in ws:
    b=box(w);row=max(0,min(3,round((b[1]-45)/151)));b[1]=45+row*128;move(w,b)
    name=get(cd(w),'name')
    if name in targets:ws.append(button('','Lock '+name,[b[0],b[1]+24,44,44],True))
   put(tab,'qlinkBoundsData',[f'{38+320*i} 0 248 560'for i in range(4)])
  ws.extend(button(a['label'],a['label'],[a['x'],policy['y'],a['width'],policy['height']])for a in policy['actions'])
