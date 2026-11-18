"""Value locks shared across MAIN and detail pages; no Q-Link allocation."""
import copy,json,re
from PIL import Image,ImageDraw

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_)in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(e,'key'):get(e,'value')for e in entries}
 cfg=json.loads((out.parent/'dice-locks.json').read_text());targets={int(k):v for k,v in cfg['targets'].items()}
 def add(k,d):entries.append(O(('key',k),('value',d)));defs[k]=d
 def box(w):return list(map(int,get(get(w,'bounds'),'bounds').split()))
 def move(w,b):put(get(w,'bounds'),'bounds',' '.join(map(str,b)))
 def cd(w):return get(w,'componentData')
 for locked in [False,True]:
  im=Image.new('RGBA',(44,44));d=ImageDraw.Draw(im);ink='#b65a20'if locked else'#87643c'
  if locked:
   d.arc((15,5,29,23),180,360,fill=ink,width=2);d.line((15,13,15,20),fill=ink,width=2);d.line((29,13,29,20),fill=ink,width=2);d.rounded_rectangle((12,16,31,29),3,fill=ink);d.ellipse((21,20,23,22),fill='#fff0c5');d.line((22,22,22,25),fill='#fff0c5',width=2)
  else:d.arc((20,3,34,21),180,360,fill=ink,width=2);d.line((20,11,20,19),fill=ink,width=2);d.rounded_rectangle((12,16,31,29),3,outline=ink,width=2)
  im.save(out/('dt-value-lock-on.png'if locked else'dt-value-lock-off.png'))
 def lock(index,b='0 0 44 44'):
  key='DTValueLock'+str(index)
  if key not in defs:
   ws=[g.entry('lock art','Button',g.button_data('dt-value-lock-on.png','dt-value-lock-off.png',1,1),'0 0 44 44','No','Show','Always',remap=[('Data',f'Parameter {index}')]),g.entry('lock art touch','Button',g.button_data('button-hit.png','button-hit.png',1,1),'0 0 44 44','No','Show','Always',remap=[('Data',f'Parameter {index}')])]
   add(key,g.component_definition([],ws,'00000000'))
  return g.entry('Dice value lock '+str(index),key,O(('version',1),('handleName','Data')),b,'Children','Show','Always')
 main_names={'OscPair','Mix','Detune','Drift','Cutoff','Resonance','BodyTone','Sub','Rate','Depth','Lfo2RateControl','Lfo2Depth','Drive','Glide','TremRate','TremDepth'}
 for tab in get(pd,'tabs'):
  name=get(tab,'tabName');widgets=get(get(tab,'componentDefinition'),'componentsData');extra=[]
  for w in list(widgets):
   c=cd(w);typ=get(c,'type');bindings=get(get(w,'handle remapping'),'map');binding=next((get(r,'value')for r in bindings if get(r,'key')=='Data'),'');match=re.search(r'Parameter (\d+)$',binding)
   if not match or int(match[1])not in targets or typ not in defs or dict(defs[typ]).get('hideQLinkBounds'):continue
   index=targets[int(match[1])];x,y,width,height=box(w)
   if name=='MAIN':
    if get(c,'name')in main_names:extra.append(lock(index,f'{x-8} {y} 44 44'))
    continue
   if typ=='DTEnvStage':
    key=typ+'ValueLock'+str(index);d=copy.deepcopy(defs[typ]);ws=get(d,'componentsData')
    for child in ws:
     ct=get(cd(child),'type');data=get(cd(child),'data')
     if ct=='Knob':move(child,[41,44,64,96])
     elif ct=='Label'and dict(data).get('type')=='Name':move(child,[44,0,100,28]);put(get(get(data,'textStyle'),'font'),'height',22.0)
     else:a,b,cw,ch=box(child);move(child,[a+8,b,cw,ch])
    ws.append(lock(index));add(key,d);put(c,'type',key);move(w,[x,y,146,height]);continue
   if typ.startswith('DTKnob')or typ.startswith('Footer_DTKnob'):
    extra.append(lock(index,f'{max(0,x-(8 if typ.startswith("Footer_")else 14))} {y} 44 44'));continue
   if typ.startswith(('knob','radio','slider','fader')):
    key=typ+'ValueLock'+str(index)
    if key not in defs:
     d=copy.deepcopy(defs[typ]);ws=get(d,'componentsData');native=any(get(cd(v),'type')=='Knob'and box(v)[2]>=80 for v in ws)
     for child in ws:
      ct=get(cd(child),'type');data=get(cd(child),'data');cx,cy,cw,ch=box(child)
      if native and ct=='Knob':move(child,[cx+44,cy,cw,ch])
      if ct=='Label'and dict(data).get('type')=='Name':move(child,[154 if native else 48,cy,112 if native else width-60,ch]);put(get(get(data,'textStyle'),'font'),'height',20.0 if native else 22.0)
      if native and ct=='Label'and dict(data).get('type')=='Value':move(child,[154,cy,112,ch])
     ws.append(lock(index));add(key,d)
    put(c,'type',key);continue
   if typ.startswith(('DTSmallMenu','DTMenu','DTSelect','DTEnvSelect','DTEnvTypeCompact')):extra.append(lock(index,f'{max(0,x-48)} {max(0,y-6)} 44 44'))
  widgets.extend(extra)
 # The lower CREATE caption becomes a real global action; all other actions stay.
 widgets=get(get(get(pd,'tabs')[0],'componentDefinition'),'componentsData');widgets[:]=[w for w in widgets if get(cd(w),'name')!='CREATE']
 d=copy.deepcopy(defs['DTRandomSound']);command='Parameter '+str(cfg['unlock'])
 for action in get(d,'actions'):
  if get(action,'handleName')=='Parameter 288':put(action,'handleName',command)
  elif 'dtRandomPress'in get(action,'handleName'):put(action,'handleName',get(action,'handleName').replace('dtRandomPress','dtUnlockPress'))
 for child in get(d,'componentsData'):
  data=get(cd(child),'data');ct=get(cd(child),'type');move(child,[0,0,122,44])
  for r in get(get(child,'handle remapping'),'map'):
   if 'dtRandomPress'in get(r,'value'):put(r,'value',get(r,'value').replace('dtRandomPress','dtUnlockPress'))
  if ct=='Slider':put(data,'handleName',get(data,'handleName').replace('dtRandomPress','dtUnlockPress'))
  if ct=='Label':put(data,'text','UNLOCK ALL');put(get(get(data,'textStyle'),'font'),'height',18.0);put(get(data,'textStyle'),'colour','ff352d21')
  if ct=='Button':put(data,'onImage','dt-unlock-on.png');put(data,'offImage','dt-unlock-off.png')
 for active in [False,True]:
  im=Image.new('RGBA',(122,44));draw=ImageDraw.Draw(im);draw.rounded_rectangle((1,1,120,42),4,fill='#d88439'if active else'#dec497',outline='#72502c',width=2);im.save(out/('dt-unlock-on.png'if active else'dt-unlock-off.png'))
 add('DTUnlockAll',d);widgets.append(g.entry('Unlock All','DTUnlockAll',O(('version',1),('handleName','Data')),'18 502 122 44','Children','Show','Always'))
