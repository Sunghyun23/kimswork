"""Main rate controls share their Q-Link between free Hz and synced division."""
import copy
from PIL import Image,ImageDraw

def install(g,tui,ql,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_)in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(w):return get(w,'componentData')
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(e,'key'):get(e,'value')for e in entries}
 by={p.name:p for p in params};widgets=get(get(get(pd,'tabs')[0],'componentDefinition'),'componentsData')
 def add(key,value):entries.append(O(('key',key),('value',value)));defs[key]=value
 for state,fill in [('off','#dbc594'),('on','#b06f37')]:
  im=Image.new('RGBA',(74,30));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,72,28),4,fill=fill,outline='#79552c',width=2);im.save(out/f'tempo-sync-{state}.png')
 for key,sync_name in [('Rate','LFO Sync'),('TremRate','Motion Sync')]:
  knob=next(w for w in widgets if get(cd(w),'name')==key);definition=copy.deepcopy(defs[get(cd(knob),'type')]);unique='DTTempoRate'+key
  for w in get(definition,'componentsData'):
   data=get(cd(w),'data')
   if get(cd(w),'type')=='Label' and dict(data).get('type')=='Name':
    title=g.label_data('Text','ff241b10',26,'horizontallyCentred verticallyCentred','RATE');put(get(get(title,'textStyle'),'font'),'name','Roboto');put(get(get(title,'textStyle'),'font'),'style','Bold');put(cd(w),'data',title);put(get(w,'bounds'),'bounds','0 86 66 28')
  add(unique,definition);put(cd(knob),'type',unique)
  sync=by[sync_name].handle;local='Local/2/tempoPress'+key;name='DTTempoSync'+key
  label=g.entry('SYNC','Label',g.label_data('Text','ff241b10',21,'horizontallyCentred verticallyCentred','SYNC'),'0 0 74 30','No','Show','Always',remap=[('Data','Parameter 0')])
  put(get(get(get(cd(label),'data'),'textStyle'),'font'),'name','Roboto');put(get(get(get(cd(label),'data'),'textStyle'),'font'),'style','Bold')
  art=g.entry('sync state','Button',g.button_data('tempo-sync-on.png','tempo-sync-off.png',1,2),'0 0 74 30','No','Show','Always',remap=[('Data',sync)])
  surface=g.entry('sync gesture','Slider',O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+local)),'0 0 74 30','No','Show','Always')
  actions=[g.action('Mouse Down','Toggle Switch',sync,''),g.action('Mouse Down','Toggle Switch',local,'')]
  add(name,g.component_definition(actions,[art,label,surface],'00000000'))
  x,y,_,_=map(int,get(get(knob,'bounds'),'bounds').split())
  widgets.append(g.entry(name,name,O(('version',1),('handleName','Data')),f'{x+68} {y+85} 74 30','Children','Show','Always',remap=[('Data','Parameter 0')]))
 # Detail rate captions retain their public names, with the same mode-aware relay.
 for tab in get(pd,'tabs'):
  if get(tab,'tabName') not in ('MOD','FX'):continue
  for w in get(get(tab,'componentDefinition'),'componentsData'):
   if get(cd(w),'name')=='LFO Rate Control':
    rem=get(get(w,'handle remapping'),'map')[0];put(rem,'value','WithName[LFO Rate]/'+get(rem,'value'))
