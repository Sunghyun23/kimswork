"""ENV/PLAY detail reflow. Existing handles, original art and MAIN stay intact."""
import copy
from PIL import Image, ImageDraw


def install(g, tui, ql, params, out):
 O = g.obj
 def get(o, k): return next(v for a, v in o if a == k)
 def put(o, k, v):
  for i, (a, _) in enumerate(o):
   if a == k: o[i] = (k, v); return
  o.append((k, v))
 pd = get(tui, 'pageData'); tabs = get(pd, 'tabs')
 entries = get(get(pd, 'componentDefinitions'), 'localComponentDefinitions')
 defs = {get(e, 'key'): get(e, 'value') for e in entries}
 by = {p.name: p for p in params}
 def add(key, d):
  entries.append(O(('key', key), ('value', d))); defs[key] = d
 def label(text, b, size=24, kind='Text', handle='Parameter 0', colour='ff352d21'):
  d = g.label_data(kind, colour, size, 'horizontallyCentred verticallyCentred', text if kind == 'Text' else None)
  f = get(get(d, 'textStyle'), 'font'); put(f, 'name', 'Roboto'); put(f, 'style', 'Bold')
  return g.entry(text or kind, 'Label', d, b, 'No', 'Show', 'Always', remap=[] if handle == 'Data' else [('Data', handle)])
 def image(name, file, b):
  return g.entry(name, 'Image', O(('version', 2), ('imageType', 'Regular'), ('colour', '0'), ('image', file)), b, 'No', 'Show', 'Always')
 def instance(name, key, b, handle, focus='Yes'):
  return g.entry(name, key, O(('version', 1), ('handleName', 'Data')), b, focus, 'Show', 'Always', remap=[] if handle == 'Data' else [('Data', handle)])
 def panel(file, w, h):
  src = Image.open(out/'group-panel.png').convert('RGBA'); im = Image.new('RGBA', (w, h))
  # Nine-slice: preserve rounded rail/corners instead of stretching their radii.
  for sx, sw, dx, dw in [(0, 18, 0, 18), (18, 576, 18, w-36), (594, 18, w-18, 18)]:
   for sy, sh, dy, dh in [(0, 30, 0, 30), (30, 172, 30, h-46), (202, 16, h-16, 16)]:
    im.alpha_composite(src.crop((sx, sy, sx+sw, sy+sh)).resize((dw, dh), Image.Resampling.LANCZOS), (dx, dy))
  im.save(out/file)
 def group(widgets, title, b, file):
  x, y, w, h = b; widgets.extend([image(title+' panel', file, f'{x} {y} {w} {h}'), label(title, f'{x+16} {y+2} {w-32} 26', 25, colour='fff4dfb5')])
 main = get(get(tabs[0], 'componentDefinition'), 'componentsData')
 header = [copy.deepcopy(w) for w in main if get(get(w, 'componentData'), 'name') in ('background', 'header donut')]
 # Header art uses a different name in earlier builds; copy every header-only Image.
 for w in main:
  if get(get(w, 'componentData'), 'type') == 'Image' and w not in header:
   x,y,wid,h = map(int, get(get(w,'bounds'),'bounds').split())
   if y == 0 and wid < 1280: header.append(copy.deepcopy(w))
 panel('env-panel.png', 612, 360); panel('env-response-panel.png', 1244, 152)
 panel('play-panel.png', 612, 218); panel('play-mode-panel.png', 612, 294)
 # 81 native-size frames fit within the GPU's 8192px texture limit.
 strip = Image.new('RGBA', (64, 96*81)); dr = ImageDraw.Draw(strip)
 for i in range(81):
  y=i*96; cy=y+82-round(i/80*68)
  dr.rounded_rectangle((27,y+4,36,y+92),4,fill='#4a3020',outline='#b18a48',width=1)
  dr.line((31,y+8,31,y+88),fill='#211911',width=2)
  dr.rounded_rectangle((9,cy-10,54,cy+10),4,fill='#65402b')
  dr.rounded_rectangle((10,cy-12,53,cy+7),3,fill='#dfbd80',outline='#684522',width=2)
  dr.line((14,cy-9,49,cy-9),fill='#fff0be',width=2)
  dr.line((15,cy-2,48,cy-2),fill='#885627',width=2)
 strip.save(out/'env-fader.png')
 lcd=Image.open(out/'value-lcd-v2.png').convert('RGBA').resize((122,28),Image.Resampling.LANCZOS)
 plate=Image.new('RGBA',(130,172)); plate.alpha_composite(lcd,(4,140)); plate.save(out/'env-fader-value.png')
 stage_widgets=[image('value plate','env-fader-value.png','0 0 130 172'),
  label('', '0 0 130 28',24,'Name','Data'),
  g.entry('vertical stage','Knob',g.knob_filmstrip_data('env-fader.png',81,'Vertical'),'33 34 64 96','Yes','Show','Always'),
  label('', '8 140 114 28',24,'Value','Data')]
 add('DTEnvStage',g.component_definition(g.control_actions('knob overlay'),stage_widgets,'00000000'))
 def curve(name, handles, b):
  d=O(('version',1),('envelopeType','ADSR'),('envelopeStyle','WithoutHeader'),('envelopeName',name),
   ('backgroundColour','ff322218'),('scaleColour','ff5b4430'),('lineColour','fff2cf86'),('fillColour','30d89c46'),('highlightColour','ffffe6b6'),
   ('lineWidth',3),('handleWidth',18),('handleHeight',14),('handleShape','Circle'),('handleLineWidth',2),
   ('handleLineColour','fff4dfb5'),('handleFillColour','ff70401e'),('handleTextColour','fff4dfb5'),
   ('handleLineWidthOnPressed',2),('handleLineColourOnPressed','ffffe6b6'),('handleFillColourOnPressed','ffb65a20'),('handleTextColourOnPressed','ffffe6b6'),
   ('handleArrowLineColourOnPressed','ffffe6b6'),('handleArrowFillColourOnPressed','ffb65a20'),
   *[(k+'Handle',h) for k,h in zip(['attack','decay','sustain','release'],handles)],
   ('attackShapeHandle','Invalid'),('decayShapeHandle','Invalid'),('releaseShapeHandle','Invalid'),
   ('forceHideShapeHandles',True),('forceCurveShape',0.5),('curveType','Exponential'))
  return g.entry(name,'Envelope',d,b,'Children','Show','Always')
 env=next(t for t in tabs if get(t,'tabName')=='FILTER'); play=next(t for t in tabs if get(t,'tabName')=='AMP')
 ew=copy.deepcopy(header)
 for bank,(prefix,x) in enumerate([('Amp',18),('Filter',650)]):
  group(ew,chr(65+bank)+'  '+prefix.upper()+' ENVELOPE',(x,94,612,360),'env-panel.png')
  names=[prefix+' '+s for s in ['Attack','Decay','Sustain','Release']]
  ew.append(curve(prefix+' curve',[by[n].handle for n in names],f'{x+16} 132 580 120'))
  for j,n in enumerate(names):
   h='WithName['+n.split()[-1].upper()+']/'+by[n].handle
   ew.append(instance(n,'DTEnvStage',f'{x+22+j*146} 274 130 172',h))
 group(ew,'C  FILTER RESPONSE',(18,470,1244,152),'env-response-panel.png')
 for j,n in enumerate(['Filter Cutoff','Resonance','Filter Env','Key Track']):
  caption={'Filter Env':'ENV AMOUNT','Filter Cutoff':'CUTOFF'}.get(n,n.upper())
  ew.append(instance(n,'Footer_DTKnob',f'{95+300*j} 502 146 116','WithName['+caption+']/'+by[n].handle))
 # Dedicated native-size selector; the final transparent hit is above the value caption.
 Image.open(out/'value-lcd-v2.png').resize((250,44),Image.Resampling.LANCZOS).save(out/'env-select.png')
 def selector(n,b):
  p=by[n]; key='DTEnvSelect'+str(p.index); local=f'Local/2/dtChoice{p.index}'
  if key not in defs:
   ws=[image('LCD','env-select.png','0 0 250 44'),label('', '8 0 216 44',24,'Value',p.handle),label('›','224 0 26 44',26)]
   ws.append(g.entry('open list','Button',g.button_data('button-hit.png','button-hit.png',1,2),'0 0 250 44','Yes','Show','Always',remap=[('Data',local)]))
   add(key,g.component_definition([g.action('Mouse Down','Q-Link',p.handle,'')],ws,'00000000'))
  return instance(n,key,b,p.handle)
 # Compact type selector in C's title rail, clear of graph and faders.
 ew.append(selector('Filter Type','994 472 250 26'))
 compact=copy.deepcopy(defs['DTEnvSelect'+str(by['Filter Type'].index)])
 Image.open(out/'env-select.png').resize((250,26),Image.Resampling.LANCZOS).save(out/'env-select-compact.png')
 for w in get(compact,'componentsData'):
  x,y,width,height=map(int,get(get(w,'bounds'),'bounds').split())
  put(get(w,'bounds'),'bounds',f'{x} 0 {width} 26')
  if get(get(w,'componentData'),'type')=='Image':put(get(get(w,'componentData'),'data'),'image','env-select-compact.png')
  elif get(get(w,'componentData'),'type')=='Label':put(get(get(get(get(w,'componentData'),'data'),'textStyle'),'font'),'height',22.0)
 add('DTEnvTypeCompact',compact);put(get(ew[-1],'componentData'),'type','DTEnvTypeCompact')
 pw=copy.deepcopy(header)
 group(pw,'A  PERFORMANCE',(18,94,612,218),'play-panel.png')
 for j,n in enumerate(['Volume','Expression','Glide','Bend Range']):pw.append(instance(n,'DTKnob',f'{26+150*j} 140 146 144',by[n].handle))
 group(pw,'B  CHORD PLAY',(650,94,612,218),'play-panel.png')
 for j,n in enumerate(['Trigger Base','Chord Spread','Chord Slot']):
  h=by[n].handle; key='DTKnob'
  if n=='Chord Slot':
   key='DTEnvSlot'; d=copy.deepcopy(defs['DTKnob'])
   get(d,'actions')[:]=[g.action('Mouse Down','Q-Link','Data',''),g.action('Double Click','Toggle Switch','Local/2/dtChoice63',''),g.action('Enter Pressed','Toggle Switch','Local/2/dtChoice63','')]
   get(d,'componentsData').append(g.entry('open slot list','Button',g.button_data('button-hit.png','button-hit.png',1,2),'0 116 146 28','Yes','Show','Always',remap=[('Data','Local/2/dtChoice63')]))
   add(key,d)
  pw.append(instance(n,key,f'{662+150*j} 140 146 144',h))
 pw.append(label('TRIGGER BASE','1114 170 138 28',18))
 pw.append(label('CONSECUTIVE','1114 206 138 28',16))
 group(pw,'C  VOICE / INPUT',(18,324,612,294),'play-mode-panel.png')
 for j,n in enumerate(['Voice Mode','Glide Mode','Input Mode']):
  pw.extend([label(n.upper(),f'34 {378+74*j} 250 44',22),selector(n,f'338 {378+74*j} 250 44')])
 group(pw,'D  RELEASE / PITCH DROP',(650,324,612,294),'play-mode-panel.png')
 for j,n in enumerate(['Amp Release','Drop Amount','Drop Time']):
  caption={'Amp Release':'RELEASE','Drop Amount':'AMOUNT','Drop Time':'TIME'}[n]
  pw.append(instance(n,'DTKnob',f'{674+196*j} 408 146 144','WithName['+caption+']/'+by[n].handle))
 def finish(tab,name,widgets,groups,guides):
  # Reuse existing sidebar definitions including choice-state and close behaviour.
  # All controls on these pages with discrete values get their existing drawer.
  needed={by[n].index for group in groups for n in group if by[n].is_discrete}
  if name=='ENV':needed.add(by['Filter Type'].index)
  for i in sorted(needed):
   assert 'DTChoicePanel'+str(i) in defs, i
   widgets.append(g.entry('Select '+str(i),'DTChoicePanel'+str(i),O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data',f'Parameter {i}')],invalidating=[f'IndexedEnabling/1/2/Local/2/dtChoice{i}']))
  put(get(tab,'componentDefinition'),'componentsData',widgets);put(tab,'tabName',name);put(tab,'qlinkBoundsData',guides)
  qmap=get(get(ql,'Screen Mode Q-Links'),'map');m=next(m for m in qmap if get(m,'Tab')==get(tab,'fnKeyIndex')+1 and get(m,'SubTab')==1)
  assignment={i:-1 for i in range(1,17)}
  for bank,group in enumerate(groups):
   for j,n in enumerate(group):assignment[13+bank-4*j]=by[n].index
  put(m,'Bank Direction','Column');put(m,'Q-Links',O(*[('Q-Link '+str(i),assignment[i])for i in range(1,17)]))
 finish(env,'ENV',ew,[['Amp '+s for s in ['Attack','Decay','Sustain','Release']],['Filter '+s for s in ['Attack','Decay','Sustain','Release']],['Filter Cutoff','Resonance','Filter Env','Key Track']],['18 264 612 184','650 264 612 184','18 498 1244 122'])
 finish(play,'PLAY',pw,[['Volume','Expression','Glide','Bend Range'],['Trigger Base','Chord Spread','Chord Slot'],['Voice Mode','Glide Mode','Input Mode'],['Amp Release','Drop Amount','Drop Time']],['18 134 612 160','650 134 612 160','18 366 612 226','650 396 612 168'])
