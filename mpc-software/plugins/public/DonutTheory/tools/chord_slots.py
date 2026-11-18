"""Sixteen stored chords with one editor, shared pad page and static editor Q-Links."""
import copy,re
from PIL import Image,ImageDraw

def install(g,tui,ql,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(c):return get(c,'componentData')
 pd=get(tui,'pageData');tabs=get(pd,'tabs');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(d,'key'):get(d,'value') for d in entries};by={p.name:p for p in params}
 def h(name):return by[name].handle
 def add(key,value):entries.append(O(('key',key),('value',value)));defs[key]=value
 def label(text,b,size=22,handle=None,invalid=None):
  data=g.label_data('Value' if handle else 'Text','ff352d21',size,'horizontallyCentred verticallyCentred',None if handle else text)
  font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold')
  return g.entry(text or 'slot value','Label',data,b,'No','Hide' if invalid else 'Show','Always',remap=[('Data',handle or 'Parameter 0')],invalidating=invalid)
 def visible(c,conditions):
  b=get(c,'bounds');put(b,'version',2);put(b,'showWhenDataModelInvalid','Hide');put(b,'additionalInvalidatingHandles',dict(b).get('additionalInvalidatingHandles',[])+conditions);return c
 def instance(key,b='0 0 1280 628',conditions=None):return g.entry(key,key,O(('version',1),('handleName','Data')),b,'Children','Hide' if conditions else 'Show','Always',remap=[('Data','Parameter 0')],invalidating=conditions)
 def image(file,w,height,on=False):
  im=Image.new('RGBA',(w,height));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,w-2,height-2),5,fill='#b06f37' if on else '#dbc594',outline='#79552c',width=2);d.line((7,4,w-8,4),fill='#ffe6b6');im.save(out/file)
 for w,height in ((36,28),(44,32),(44,44),(132,32)):
  for suffix,on in (('off',False),('on',True)):image(f'slot-{w}-{height}-{suffix}.png',w,height,on)
 # Transparent safe bottom edge for the existing sound/chord preset arrows too.
 image('arrow.png',36,34)
 def button(text,param,b,index=1,count=2):
  w,height=map(int,b.split()[2:]);base=f'slot-{w}-{height}-'
  return [g.entry(text,'Button',g.button_data(base+'on.png',base+'off.png',index,count),b,'No','Show','Always',remap=[('Data',param)]),label(text,b,22),g.entry(text+' touch','Button',g.button_data('button-hit.png','button-hit.png',index,count),b,'Yes','Show','Always',remap=[('Data',param)])]
 def clone_graph(obj,mapping,prefix):
  seen=set();ordered=[]
  def find(o):
   if isinstance(o,g.Obj):
    for k,v in o:
     if k in ('type','additionalData') and isinstance(v,str) and v in defs and v not in seen:seen.add(v);find(defs[v]);ordered.append(v)
     else:find(v)
   elif isinstance(o,list):
    for v in o:find(v)
  find(obj);rename={key:prefix+key for key in ordered}
  def transform(o):
   if isinstance(o,str):
    if o in rename:return rename[o]
    o=re.sub(r'Parameter (\d+)',lambda m:'Parameter '+str(mapping.get(int(m[1]),int(m[1]))),o)
    for stem in ('dtChoice','sidebarPage'):o=re.sub(stem+r'(\d+)',lambda m:stem+str(mapping.get(int(m[1]),int(m[1]))),o)
    return o
   if isinstance(o,g.Obj):return O(*[(k,transform(v)) for k,v in o])
   if isinstance(o,list):return [transform(v) for v in o]
   return o
  for key in ordered:add(rename[key],transform(copy.deepcopy(defs[key])))
  return transform(copy.deepcopy(obj)),rename
 main=get(tabs[0],'componentDefinition');children=get(main,'componentsData')
 editor_source=copy.deepcopy(defs['DTAllChords']);ew=get(editor_source,'componentsData')
 # Keep Root/Type/Voice/Octave visuals/actions; rebuild physical routing and
 # audition rows against the selected real slots, not the editor mirrors.
 def is_learn(c):return get(cd(c),'name').startswith(('Learn ','LEARN ','PRESS THE PHYSICAL'))
 def is_status(c):return get(cd(c),'type')=='Label' and any(get(r,'value')==h('Pad Learn Status') for r in get(get(c,'handle remapping'),'map'))
 ew[:]=[c for c in ew if not is_learn(c) and not is_status(c) and not get(cd(c),'name').startswith(('Audition ','DTChordNote','FIRST TAB','BACK TO MAIN'))]
 for c in ew:
  if get(cd(c),'type')=='Label' and get(cd(c),'name').startswith('ALL CHORDS'):
   put(get(cd(c),'data'),'text','CHORD EDITOR · Q-LINK A / B / C / D');put(get(c,'bounds'),'bounds','20 4 720 34')
 mapping={}
 for row in range(1,5):
  for source,target in [('Root','Root'),('Type','Type'),('Voice','Voice'),('Octave','Octave')]:mapping[by[f'Chord {row} {source}'].index]=by[f'Edit {chr(64+row)} {target}'].index
 add('DTEditorTemplate',editor_source)
 chord=next(t for t in tabs if get(t,'tabName')=='CHORD');page=get(chord,'componentDefinition');page_children=get(page,'componentsData')
 # Clone the editor with all its existing choice panels, retaining one real tab.
 cloned,_=clone_graph(page,mapping,'Editor_');put(chord,'componentDefinition',cloned)
 editor=defs['Editor_DTAllChords'] if 'Editor_DTAllChords' in defs else None
 # The page still points at DTAllChords; substitute the filtered source before cloning.
 # Replace only its body, then clone its referenced field definitions above.
 if editor:
  filtered,_=clone_graph(editor_source,mapping,'EditorBody_');editor[:]=filtered
 else:raise ValueError('editor graph missing')
 widgets=get(editor,'componentsData')
 widgets+=button('−',h('SlotsMinus'),'994 4 44 44')+[label('','1044 4 160 44',23,h('Chord Slots'))]+button('+',h('SlotsPlus'),'1210 4 44 44')
 for bank in range(4):
  available=f'IndexedEnabling/3/4/{h("Chord Slots")}' if bank==3 else f'RangedEnabling/{bank}/3/4/{h("Chord Slots")}'
  for c in button(f'{bank*4+1}–{bank*4+4}',h('Chord Pad Page'),f'{20+bank*140} 42 132 32',bank,4):widgets.append(visible(c,[available]))
 widgets.append(label('','590 42 670 30',19,h('Pad Learn Status'),[f'IndexedEnabling/0/17/{h("Pad Learn")}']))
 for slot in range(1,17):
  bank=(slot-1)//4;row=(slot-1)%4;y=116+row*100;condition=[f'IndexedEnabling/{bank}/4/{h("Chord Pad Page")}']
  widgets.append(label(f'PRESS PHYSICAL PAD FOR SLOT {slot}','590 42 670 30',19,invalid=[f'IndexedEnabling/{slot}/17/{h("Pad Learn")}']))
  actual={}
  for old,new in ((f'Chord {row+1} Root',f'Chord {slot} Root'),(f'Play Chord {row+1}',f'Play Chord {slot}')):actual[by[old].index]=by[new].index
  source=instance('DTAudition'+str(row+1),f'5 {y+8} 90 64')
  audition,names=clone_graph(source,actual,f'Audition{slot}_')
  for c in get(defs[names['DTAudition'+str(row+1)]],'componentsData'):
   if get(cd(c),'type')=='Label' and dict(get(cd(c),'data')).get('type')=='Text':put(get(cd(c),'data'),'text',chr(65+row)+' · '+str(slot))
  widgets.append(visible(audition,condition))
  source=g.entry(f'MIDI {slot}','DTChordNote'+str(row+1),O(('version',1),('handleName','Data')),f'815 {y} 146 90','Yes','Show','Always',remap=[('Data',h(f'Pad {slot} Note'))])
  widgets.append(visible(source,condition))
  for c in button(f'LEARN {slot}',h('Pad Learn'),f'1010 {y+17} 180 44',slot,17):
   # These proven 180px existing assets are shared with the original editor.
   if get(cd(c),'type')=='Button' and get(get(cd(c),'data'),'offImage')!='button-hit.png':
    data=get(cd(c),'data');put(data,'onImage','chord-180-on.png');put(data,'offImage','chord-180-off.png')
   widgets.append(visible(c,condition))
 # Editor fields address rows A–D, rather than showing a stale slot-1
 # title while row A is editing slot 5, 9 or 13.
 for key,definition in list(defs.items()):
  if not key.startswith(('Editor_','EditorBody_')):continue
  for c in get(definition,'componentsData'):
   if get(cd(c),'type')!='Label':continue
   data=get(cd(c),'data');text=dict(data).get('text','');match=re.fullmatch(r'CHORD ([1-4]) (TYPE|VOICING|OCTAVE)',text)
   if match:put(data,'text',chr(64+int(match[1]))+' · '+match[2])
 # Fixed Q-Link parameters are editing mirrors; their target bank follows the
 # same saved page parameter used by MAIN. Actual slot IDs never move.
 qmap=next(m for m in get(get(ql,'Screen Mode Q-Links'),'map') if (get(m,'Tab'),get(m,'SubTab'))==(1,2));assign={}
 for row in range(4):
  for knob,name in enumerate(('Root','Type','Voice','Octave')):assign[13+row-4*knob]=by[f'Edit {chr(65+row)} {name}'].index
 put(qmap,'Q-Links',O(*[(f'Q-Link {n}',assign[n]) for n in range(1,17)]))
 # MAIN: retain four full-size hold pads per visible bank.
 pads=[c for c in children if get(cd(c),'name').startswith('Chord ') and get(cd(c),'type').startswith('DTPad')]
 for c in pads:visible(c,[f'IndexedEnabling/0/4/{h("Chord Pad Page")}'])
 for bank in range(1,4):
  for row,source in enumerate(pads):
   slot=bank*4+row+1;actual={by[f'Chord {row+1} Root'].index:by[f'Chord {slot} Root'].index,by[f'Play Chord {row+1}'].index:by[f'Play Chord {slot}'].index}
   c,names=clone_graph(source,actual,f'Main{slot}_');put(cd(c),'name',f'Chord {slot}')
   definition=defs[names[f'DTPad{row}']]
   for w in get(definition,'componentsData'):
    if get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('type')=='Text':put(get(cd(w),'data'),'text',str(slot))
   put(get(c,'bounds'),'additionalInvalidatingHandles',[f'IndexedEnabling/{bank}/4/{h("Chord Pad Page")}']);children.append(c)
 # Larger chord navigation: preset arrows are 44x44; bank arrows have a
 # 44x44 invisible hit area around a 44x32 face. The top target begins below
 # the filter LCDs (ending at y534); targets meet at y580 without overlap.
 for key in ('DTPrev','DTNext'):
  comp=copy.deepcopy(defs[key]);large=key+'ChordLarge'
  for w in get(comp,'componentsData'):
   put(get(w,'bounds'),'bounds','0 0 44 44')
   data=get(cd(w),'data')
   if get(cd(w),'type')=='Button' and get(data,'offImage')!='button-hit.png':
    put(data,'onImage','slot-44-44-on.png');put(data,'offImage','slot-44-44-off.png')
   if get(cd(w),'type')=='Label':put(get(get(data,'textStyle'),'font'),'height',30.)
  add(large,comp)
 for c in children:
  name=get(cd(c),'name')
  if name in ('ChordPrev','ChordNext'):
   put(cd(c),'type',('DTPrev' if name=='ChordPrev' else 'DTNext')+'ChordLarge')
   put(get(c,'bounds'),'bounds',f'{18 if name=="ChordPrev" else 294} 580 44 44')
  elif name=='Progression':
   put(get(c,'bounds'),'bounds','68 580 220 44')
   # This selector definition is unique to the chord bank; sound UI stays intact.
   for w in get(defs[get(cd(c),'type')],'componentsData'):
    x,y,wid,hei=map(int,get(get(w,'bounds'),'bounds').split())
    put(get(w,'bounds'),'bounds',f'{x} 0 {wid} 44')
 menu=Image.open(out/'menu-chord.png').convert('RGBA')
 menu.resize((220,44),Image.Resampling.BICUBIC).save(out/'menu-chord.png')
 # One current-bank relay avoids constructing four hidden96-row browsers.
 # Bank identities remain separate in the engine; MOD still selects bank1.
 progression=next(c for c in children if get(cd(c),'name')=='Progression')
 for c in children:
  if get(cd(c),'name')=='CHORD PRESET':put(get(c,'bounds'),'bounds','18 548 146 28');put(get(get(get(cd(c),'data'),'textStyle'),'font'),'height',18.)
 for caption,key,x in [('‹','PadPagePrev',172),('›','PadPageNext',294)]:
  ws=button(caption,h(key),f'{x} 542 44 32')
  # Only the final, topmost transparent surface receives the extended hit box.
  put(get(ws[-1],'bounds'),'bounds',f'{x} 536 44 44')
  children+=ws
 for count in range(4):
  for bank in range(count+1):children.append(label(f'{bank*4+1}–{bank*4+4} / {(count+1)*4}','218 548 72 28',18,invalid=[f'IndexedEnabling/{bank}/4/{h("Chord Pad Page")}',f'IndexedEnabling/{count}/4/{h("Chord Slots")}']))
 for bank in range(4):children.append(label(str(bank+1),f'{220+bank*17} 534 16 12',12,invalid=[f'IndexedEnabling/1/2/{h("Playing Chord Page "+str(bank+1))}']))
 mod=next(t for t in tabs if get(t,'tabName')=='MOD')
 for c in get(get(mod,'componentDefinition'),'componentsData'):
  if get(cd(c),'name')=='Chord Preset':
   for r in get(get(c,'handle remapping'),'map'):
    if get(r,'key')=='Data':put(r,'value','WithName[PRESET 1–4]/'+h('Chord Preset'))
