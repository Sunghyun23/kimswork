"""Two-step category/sound browser for every in-plugin Sound Preset entry."""
from PIL import Image,ImageDraw

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(d,'key'):get(d,'value') for d in entries}
 p=next(p for p in params if p.name=='Sound Preset');handle=p.handle;cat=f'Local/7/soundCategory{p.index}';opened=f'Local/2/dtChoice{p.index}'
 def label(text,b,size=26):
  data=g.label_data('Text','ff352d21',size,'horizontallyCentred verticallyCentred',text);font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold')
  return g.entry(text,'Label',data,b,'No','Show','Always',remap=[('Data','Parameter 0')])
 def button(text,h,b,index,count):
  x,y,w,height=map(int,b.split())
  for state,fill in [('off','#dbc594'),('on','#b06f37')]:
   file=f'bank-{w}-{height}-{state}.png';im=Image.new('RGBA',(w,height));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,w-2,height-2),6,fill=fill,outline='#79552c',width=2);d.line((7,4,w-8,4),fill='#ffe6b6');im.save(out/file)
  return [g.entry(text,'Button',g.button_data(f'bank-{w}-{height}-on.png',f'bank-{w}-{height}-off.png',index,count),b,'Yes','Show','Always',remap=[('Data',h)]),label(text,b),g.entry(text+' touch','Button',g.button_data('button-hit.png','button-hit.png',index,count),b,'Yes','Show','Always',remap=[('Data',h)])]
 categories=['Init','Bass','Keys','Synth','Lead','Pad']
 root=[label('SOUND PRESET — CATEGORY / SOUND','20 8 1000 42',30)]+button('DONE',opened,'1135 10 120 38',0,2)
 root +=[label('1 · CATEGORY','24 66 250 30',24)]
 for ci,category in enumerate(categories,1):
  choices=[(i,name.split(' / ',1)[1]) for i,name in enumerate(p.choices) if name.startswith(category+' / ')]
  root +=button(f'{category.upper()}  ({len(choices)})',cat,f'24 {108+(ci-1)*76} 250 64',ci,7)
  widgets=[label('2 · '+category.upper(),'306 66 940 30',26)]
  for j,(index,name) in enumerate(choices):widgets+=button(name,handle,f'{306+(j%3)*314} {108+(j//3)*82} 300 64',index,len(p.choices))
  key=f'DTSoundCategory{ci}';entries.append(O(('key',key),('value',g.component_definition([],widgets,'00000000',ignore_mouse=True))))
  root.append(g.entry(key,key,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Hide','Always',remap=[('Data',handle)],invalidating=[f'IndexedEnabling/{ci}/7/{cat}']))
 # No default category means the first visit asks for a category before a sound.
 key='DTSoundCategoryPrompt';entries.append(O(('key',key),('value',g.component_definition([],[label('CHOOSE A CATEGORY TO SEE ITS SOUNDS','306 220 940 80',30)],'00000000',ignore_mouse=True))))
 root.append(g.entry(key,key,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Hide','Always',remap=[('Data',handle)],invalidating=[f'IndexedEnabling/0/7/{cat}']))
 root.append(label('LOW CUT IS SET PER SOUND · INIT HAS NO LOW CUT OR EFFECTS','306 558 940 50',22))
 put(defs['DTChoicePanel'+str(p.index)],'componentsData',root)
