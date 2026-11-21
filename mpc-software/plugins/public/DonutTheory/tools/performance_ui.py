"""Shared-state POLY/MONO editors and individual OSC/LFO control surfaces.
Only native tab navigation changes views. No navigation action writes DSP values.
"""
import copy,json,re
from PIL import Image

def install(g,tui,ql,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_)in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(w):return get(w,'componentData')
 def box(w):return list(map(int,get(get(w,'bounds'),'bounds').split()))
 def move(w,b):put(get(w,'bounds'),'bounds',' '.join(map(str,b)))
 pd=get(tui,'pageData');tabs=get(pd,'tabs');old={get(t,'tabName'):t for t in tabs}
 entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(e,'key'):get(e,'value')for e in entries}
 meta=json.loads((out.parent/'parameters.json').read_text());ix={p['key']:i for i,p in enumerate(meta)}
 locks=json.loads((out.parent/'dice-locks.json').read_text());targets={int(k):v for k,v in locks['targets'].items()}
 def h(k):return 'Parameter '+str(ix[k])
 def add(k,d):
  if k in defs:defs[k][:]=d;return
  entries.append(O(('key',k),('value',d)));defs[k]=d
 def label(text,b,size=22,kind='Text',handle='Parameter 0',colour='ff352d21'):
  data=g.label_data(kind,colour,size,'horizontallyCentred verticallyCentred',text if kind=='Text' else None)
  put(get(get(data,'textStyle'),'font'),'name','Roboto');put(get(get(data,'textStyle'),'font'),'style','Bold')
  return g.entry(text or kind,'Label',data,b,'No','Show','Always',remap=[] if handle=='Data' else [('Data',handle)])
 def image(n,file,b):return g.entry(n,'Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',file)),b,'No','Show','Always')
 def inst(n,k,b,handle='Parameter 0',invalid=None):return g.entry(n,k,O(('version',1),('handleName','Data')),b,'Children','Hide'if invalid else'Show','Always',remap=[('Data',handle)],invalidating=invalid or [])
 def hit(n,handle,b,index=1,count=2):return g.entry(n,'Button',g.button_data('button-hit.png','button-hit.png',index,count),b,'Yes','Show','Always',remap=[('Data',handle)])
 def scaled(src,size):
  name='perf-'+str(size[0])+'x'+str(size[1])+'-'+src
  if not(out/name).exists():Image.open(out/src).convert('RGBA').resize(size,Image.Resampling.LANCZOS).save(out/name)
  return name
 def resize_instance(z,b):
  oldx,oldy,oldw,oldh=box(z);typ=get(cd(z),'type');move(z,b)
  if typ not in defs:return z
  name='PerfResize'+str(len(defs));d=copy.deepcopy(defs[typ])
  for c in get(d,'componentsData'):
   x,y,w,hh=box(c);bb=[round(x*b[2]/oldw),round(y*b[3]/oldh),round(w*b[2]/oldw),round(hh*b[3]/oldh)];move(c,bb);data=get(cd(c),'data')
   if get(cd(c),'type')=='Button':
    for field in('onImage','offImage'):
     src=dict(data).get(field)
     if src and src!='button-hit.png':put(data,field,scaled(src,(bb[2],bb[3])))
   elif get(cd(c),'type')=='Label':put(get(get(data,'textStyle'),'font'),'height',20.0)
  add(name,d);put(cd(z),'type',name);return z
 def plate(n,b,src='value-lcd-v2.png'):
  x,y,w,hh=map(int,b.split());return image(n,scaled(src,(w,hh)),b)
 def segment(key,captions,b):
  x,y,w,hh=map(int,b.split());widgets=[];each=w//len(captions)
  for i,caption in enumerate(captions):
   bx=x+each*i;bb=f'{bx} {y} {each-2} {hh}'
   on=scaled('tempo-sync-on.png',(each-2,hh));off=scaled('tempo-sync-off.png',(each-2,hh))
   widgets.append(g.entry(caption+' state','Button',g.button_data(on,off,1-i if key=='PerfVoice' else i,len(captions)),bb,'No','Show','Always',remap=[('Data',h(key))]))
   widgets.extend([label(caption,bb,18 if key=='MonoLegato' else 20),hit(caption+' touch',h(key),bb,1-i if key=='PerfVoice' else i,len(captions))])
  return widgets
 def modal(definition):
  put(definition,'version',4);position=next(i for i,(key,_)in enumerate(definition)if key=='componentsData');definition.insert(position,('hideQLinkBounds',True));return definition
 def ensure_menu(key):
  idx=ix[key];name='DTChoicePanel'+str(idx)
  if name in defs:return
  opened=f'Local/2/dtChoice{idx}';ws=[image('sidebar','sidebar-panel.png','0 0 426 628'),label(meta[idx]['name'].upper(),'16 4 346 50',26,colour='fff2f2f2'),label('×','366 4 48 48',30,colour='fff2f2f2'),hit('close',opened,'366 4 48 48',0),hit('dismiss outside',opened,'426 0 854 628',0)]
  count=len(meta[idx]['choices']);pages=(count+7)//8;pagehandle=f'Local/{max(2,pages)}/perfChoicePage{idx}'
  for i,text in enumerate(meta[idx]['choices']):
   if key in ('Lfo1SyncDivision','Lfo2SyncDivision','MotionSyncDivision'):text=text.replace(' D',' · Dotted').replace(' T',' · Triplet')
   row='PerfChoiceRow'+str(idx)+'-'+str(i);b=f'0 {64+(i%8)*58} 426 58'
   rw=[g.entry('selected','Button',g.button_data('sidebar-row-on.png','sidebar-row-off.png',i,count),'0 0 426 58','No','Show','Always',remap=[('Data',h(key))]),label(text,'24 0 346 58',28,colour='fff2f2f2'),hit('select',h(key),'0 0 426 58',i,count)]
   add(row,g.component_definition([g.action('Mouse Down','Toggle Switch',opened,'')],rw,'00000000'))
   condition=[f'IndexedEnabling/{i//8}/{max(2,pages)}/{pagehandle}']if pages>1 else []
   if key.startswith('EditVoicing')and i>=4:
    tones=h('HarmonyTones'+key[-1]);condition+=[f'IndexedEnabling/2/3/{tones}'if i==5 else f'RangedEnabling/1/2/3/{tones}']
   ws.append(inst(row,row,b,invalid=condition))
  if pages>1:
   for page in range(pages):
    fw=[label(f'{page+1} / {pages}','168 568 90 52',24,colour='fff2f2f2')]
    if page:fw.extend([label('‹ PREV','8 568 144 52',23,colour='fff2f2f2'),hit('previous page',pagehandle,'8 568 144 52',page-1,max(2,pages))])
    if page+1<pages:fw.extend([label('NEXT ›','274 568 144 52',23,colour='fff2f2f2'),hit('next page',pagehandle,'274 568 144 52',page+1,max(2,pages))])
    fn='PerfChoiceFooter'+str(idx)+'-'+str(page);add(fn,g.component_definition([],fw,'00000000',ignore_mouse=True));ws.append(inst(fn,fn,'0 0 426 628',invalid=[f'IndexedEnabling/{page}/{max(2,pages)}/{pagehandle}']))
  add(name,modal(g.component_definition([],ws,'90000000')))
 def menu(key,b,caption=None):
  ensure_menu(key);x,y,w,hh=map(int,b.split());name='PerfMenu'+key+str(w)+'x'+str(hh);opened=f'Local/2/dtChoice{ix[key]}'
  ws=[plate('LCD',f'0 0 {w} {hh}'),label('',f'8 0 {w-36} {hh}',20 if hh<=30 else 18 if w<=180 else 22,'Value',h(key)),label('›',f'{w-24} 0 24 {hh}',24),hit('open list',opened,f'0 0 {w} {hh}')]
  if key=='DiceTarget'and w==166:
   move(ws[1],[8,14,w-36,30]);put(get(get(get(cd(ws[1]),'data'),'textStyle'),'font'),'height',18.0);ws.insert(1,label('DICE TARGET',f'8 0 {w-36} 14',12))
  actions=[g.action('Mouse Down','Q-Link',h(key),'')]
  if key=='SoundCurrent':
   ws[-1]=hit('reset sound category',f'Local/7/soundCategory{ix[key]}',f'0 0 {w} {hh}',0,7);actions.append(g.action('Mouse Down','Toggle Switch',opened,''))
  add(name,g.component_definition(actions,ws,'00000000'))
  return inst(caption or key,name,b,h(key))
 def lock(key,b,invalid=None):
  idx=targets.get(ix[key]);assert idx is not None,key
  return inst('Lock '+key,'DTValueLock'+str(idx),b,h(key),invalid)
 # New canonical lock definitions use the same 44px art/hit convention.
 for idx in set(targets.values()):
  name='DTValueLock'+str(idx)
  if name in defs:continue
  ws=[g.entry('lock art','Button',g.button_data('dt-value-lock-on.png','dt-value-lock-off.png',1,1),'0 0 44 44','No','Show','Always',remap=[('Data',f'Parameter {idx}')]),hit('lock art touch',f'Parameter {idx}','0 0 44 44')]
  add(name,g.component_definition([],ws,'00000000'))
 # Preserve actual existing art; no generated concept image enters the runtime.
 main=get(get(old['MAIN'],'componentDefinition'),'componentsData')
 header=[copy.deepcopy(w)for w in main if get(cd(w),'type')=='Image'and box(w)[1]==0]
 panels={}
 def panel(ws,title,x,y,w=612,hh=176):
  file='perf-panel-'+str(w)+'x'+str(hh)+'.png'
  if file not in panels:
   src=Image.open(out/'group-panel.png').convert('RGBA');im=Image.new('RGBA',(w,hh))
   for sx,sw,dx,dw in [(0,18,0,18),(18,576,18,w-36),(594,18,w-18,18)]:
    for sy,sh,dy,dh in [(0,30,0,30),(30,172,30,hh-46),(202,16,hh-16,16)]:im.alpha_composite(src.crop((sx,sy,sx+sw,sy+sh)).resize((dw,dh),Image.Resampling.LANCZOS),(dx,dy))
   im.save(out/file);panels[file]=True
  ws.extend([image(title+' panel',file,f'{x} {y} {w} {hh}'),label(title,f'{x+12} {y+2} {w-24} 26',24,colour='fff4dfb5')])
 def knob(ws,key,caption,x,y,locked=True,small=False,compact=False,conditions=None,qlink_key=None):
  # Normalized edit relays are Q-Link targets, not numeric-entry ranges.
  # Bind visible controls/overlays to canonical units, while the parent focus
  # action selects the matching shared Q-Link relay.
  conditions=conditions or []
  if key=='PerfCharacter':
   for focus,actual in enumerate(('Drift','OscPhase')):
    knob(ws,actual,'PHASE'if focus else caption,x,y,False,small,compact,conditions+[f'IndexedEnabling/{focus}/2/{h("OscCharacterFocus")}'],key)
   return
  if key=='PerfPitch':
   for osc in range(2):
    for fine in range(2):
     actual=('OscFine'if fine else'OscTune')+str(osc+1)
     cond=conditions+[f'IndexedEnabling/{osc}/2/{h("OscFocus")}',f'IndexedEnabling/{fine}/2/{h("PitchResolution")}']
     knob(ws,actual,caption,x,y,False,small,compact,cond,key)
   return
  if key in('PerfLfoRate','LfoRateControl','Lfo2RateControl','ModRateControl'):
   modules=range(2)if key=='PerfLfoRate'else[0 if key=='LfoRateControl'else 1 if key=='Lfo2RateControl'else 2]
   for module in modules:
    sync=['Sync','Lfo2Sync','ModSync'][module];rate=['Rate','Lfo2Rate','TremRate'][module];division=['Lfo1SyncDivision','Lfo2SyncDivision','MotionSyncDivision'][module]
    for active in range(2):
     cond=conditions+[f'IndexedEnabling/{active}/2/{h(sync)}']
     if key=='PerfLfoRate':cond+=[f'IndexedEnabling/{module}/2/{h("LfoFocus")}']
     knob(ws,division if active else rate,caption,x,y,locked,small,compact,cond,key)
   return
  size=48 if small else 64 if compact else 72;height=80 if small else 108 if compact else 116;name='PerfKnob'+key+('Small'if small else 'Compact'if compact else '')+(qlink_key or '')
  width=90 if small else 146;value_y=56 if small else 84 if compact else 92
  ww=[g.entry('knob','Knob',O(('version',4),('knobType','RotatingImage'),('foregroundImage','knob-donut.png'),('backgroundImage',''),('imageScale',1.0),('dragOrientation','Vertical'),('handleName','Data')),f'{(width-size)//2} 0 {size} {size}','Yes','Show','Always')]
  if not small:ww.append(label(caption,f'0 {64 if compact else 72} {width} 20',20))
  ww.extend([plate('value LCD',f'4 {value_y} {width-8} 24'),label('',f'8 {value_y} {width-16} 24',22,'Value','Data')])
  if key=='OscPhase':
   # Keep the same72px artwork, caption baseline and24px value band as
   # neighboring controls. Restart gets a44px hit area without a taller LCD.
   ww=ww[:2]+[plate('phase LCD','4 92 82 24'),label('','6 92 78 24',20,'Value','Data')]
   for active in range(2):
    caption_mode='RETRIG'
    mode_name='PerfPhaseMode'+str(active)
    mw=[g.entry('restart state','Button',g.button_data(scaled('tempo-sync-on.png',(56,24)),scaled('tempo-sync-off.png',(56,24)),1,2),'0 20 56 24','No','Show','Always',remap=[('Data',h('OscRetrig'))]),label(caption_mode,'0 20 56 24',14),hit('change restart',h('OscRetrig'),'0 0 56 44',1-active,2)]
    add(mode_name,g.component_definition([],mw,'00000000'));ww.append(inst(caption_mode,mode_name,'90 72 56 44',invalid=[f'IndexedEnabling/{active}/2/{h("OscRetrig")}']))
  if 'choices'in meta[ix[key]]:
   ensure_menu(key);ww.append(hit('open value list',f'Local/2/dtChoice{ix[key]}',f'0 {value_y} {width} 24'))
  actions=g.control_actions('knob overlay')
  if 'choices'in meta[ix[key]]:actions=[g.action('Mouse Down','Q-Link','Data',''),g.action('Double Click','Toggle Switch',f'Local/2/dtChoice{ix[key]}',''),g.action('Enter Pressed','Toggle Switch',f'Local/2/dtChoice{ix[key]}','')]
  if qlink_key:
   for action in actions:
    if get(action,'handler')=='Q-Link':put(action,'handleName',h(qlink_key))
  add(name,g.component_definition(actions,ww,'00000000'))
  ws.append(inst(key,name,f'{x} {y} {width} {height}',h(key),conditions))
  if locked and ix[key]in targets:ws.append(lock(key,f'{max(0,x-8)} {y} 44 44',conditions))
 def touch_value(key,b,caption):
  x,y,w,hh=map(int,b.split());name='PerfValue'+key+str(w)
  split=40 if hh<=30 else w//2
  ws=[plate('LCD',f'0 0 {w} {hh}'),label(caption,f'0 0 {split} {hh}',16 if hh<=30 else 18),label('',f'{split} 0 {w-split} {hh}',18 if hh<=30 else 21,'Value','Data')]
  if ix[key]in targets:
   move(ws[1],[44,0,w//2-44,hh]);ws.append(lock(key,'0 0 44 44'))
  # Visible field owns the normal numeric overlay; double tap allows exact input.
  add(name,g.component_definition(g.control_actions('knob overlay'),ws,'00000000'))
  return inst(key,name,b,h(key))
 def command(key,caption,b,orange=False):
  x,y,w,hh=map(int,b.split());name='PerfCommand'+key+str(w);local='Local/2/perfPress'+key
  ws=[g.entry('button art','Button',g.button_data(scaled('random-on.png'if orange else'dt-unlock-on.png',(w,hh)),scaled('random-off.png'if orange else'dt-unlock-off.png',(w,hh)),1,2),f'0 0 {w} {hh}','No','Show','Always',remap=[('Data',local)]),label(caption,f'{40 if orange else 0} 0 {w-(40 if orange else 0)} {hh}',22,colour='fff4dfb5'if orange else'ff352d21')]
  ws.append(g.entry('press release','Slider',O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+local)),f'0 0 {w} {hh}','No','Show','Always'))
  add(name,g.component_definition([g.action('Mouse Down','Toggle Switch',h(key),''),g.action('Mouse Down','Toggle Switch',local,'')],ws,'00000000'))
  return inst(caption,name,b,h(key))
 def drawer(ws,key):
  ensure_menu(key);ws.append(inst('Select '+key,'DTChoicePanel'+str(ix[key]),'0 0 1280 628',h(key),[f'IndexedEnabling/1/2/Local/2/dtChoice{ix[key]}']))
 footer=json.loads((out.parent.parents[2]/'shared/ui/action-footer.json').read_text())
 def actions_footer(ws):
  for action,key in zip(footer['actions'],['UnlockDice','Randomize','UndoDice']):
   ws.append(command(key,action['label'],f"{action['x']} {footer['y']} {action['width']} {footer['height']}",key=='Randomize'))
 def browse_footer(ws,y=526,actions_y=578,poly=False):
  # Existing categorized browser launcher resets to category root on every entry.
  sound=copy.deepcopy(next(w for w in main if get(cd(w),'name')=='SoundPreset'));move(sound,[150,y,240,44]);ws.append(sound)
  ws.extend([label('PRESET',f'18 {y} 80 44',20),command('SoundPrev','‹',f'100 {y} 44 44'),command('SoundNext','›',f'396 {y} 44 44'),label('VOICE',f'454 {y} 70 44',20)])
  ws+=segment('PerfVoice',['POLY','MONO'],f'530 {y} 184 44')
  ws.append(label('DICE TARGET',f'734 {y} 148 44',20));ws.append(menu('DiceTarget',f'886 {y} 184 44'))
  ws.append(menu('InputMode',f'1078 {y} 184 44'))
  actions_footer(ws)
  # Actual mode has one explicit selector; legacy Voice Mode remains a host parameter.
  ws.append(inst('Performance settings','PerfSettingsLauncher',f'920 {footer["y"]} 342 44'))
 def qmap(tab,groups):
  assign={i:-1 for i in range(1,17)}
  for bank,keys in enumerate(groups):
   for j,key in enumerate(keys):assign[13+bank-4*j]=ix[key]
  return O(('Tab',get(tab,'fnKeyIndex')+1),('SubTab',get(tab,'fnKeySubIndex')+1),('Bank Direction','Column'),('Q-Links',O(*[('Q-Link '+str(i),assign[i])for i in range(1,17)])))
 # Utility drawer keeps rare performance settings accessible without footer tiles.
 settings='Local/2/perfSettings';sw=[image('sidebar','sidebar-panel.png','0 0 426 628'),label('PERFORMANCE','12 4 214 48',26,colour='fff2f2f2'),label('×','366 4 48 48',28,colour='fff2f2f2'),hit('close',settings,'366 4 48 48',0),hit('outside',settings,'426 0 854 628',0)]
 for i,(key,caption)in enumerate([('Volume','VOLUME'),('Expression','EXPRESSION'),('Bend','BEND RANGE'),('Trigger','TRIGGER BASE'),('Spread','STRUM'),('Motion','MOTION')]):sw.append(touch_value(key,f'18 {76+i*78} 390 54',caption))
 sw.append(label('PAD MAP','18 526 390 22',19,colour='fff2f2f2'));sw+=segment('PadMap',['CONSECUTIVE','INDIVIDUAL'],'18 550 390 44')
 add('PerfSettings',modal(g.component_definition([],sw,'90000000')))

 add('PerfSettingsLauncher',g.component_definition([], [plate('plate','0 0 342 44','dt-unlock-off.png'),label('SETTINGS','0 0 342 44',20),hit('open',settings,'0 0 342 44')],'00000000'))
 def finish(tab,name,fn,sub,ws,groups,guides):
  put(tab,'tabName',name);put(tab,'fnKeyIndex',fn);put(tab,'fnKeySubIndex',sub);put(tab,'qlinkBoundsData',guides)
  # All used selectors have a sidebar placed above ordinary siblings.
  used=set()
  def scan(w):
   for r in get(get(w,'handle remapping'),'map'):
    m=re.search(r'Parameter (\d+)$',get(r,'value'))
    if m and 'choices'in meta[int(m[1])]:used.add(int(m[1]))
  for w in ws:scan(w)
  for i in sorted(used):
   k=meta[i]['key'];drawer(ws,k)
  # Shared browser and utility modal root, always last.
  if not any(get(cd(w),'type')=='DTChoicePanel'+str(ix['SoundCurrent'])for w in ws):drawer(ws,'SoundCurrent')
  ws.append(inst('Performance settings modal','PerfSettings','0 0 1280 628',invalid=[f'IndexedEnabling/1/2/{settings}']))
  put(get(tab,'componentDefinition'),'componentsData',ws);return qmap(tab,groups)
 # Front editors share A/B/C controls and have different D controls only.
 frontgroups=[['PerfWaveFull','PerfPitch','PerfLevel','PerfCharacter'],['Cutoff','Resonance','BodyTone','Drive'],['PerfLfoRate','PerfLfoDepth','PerfLfoDelay','PerfLfoTarget']]
 maps=[];newtabs=[]
 for mono in (False,True):
  tab=old['PLAY']if mono else old['MAIN'];ws=copy.deepcopy(header)
  titles=['A  OSC / SOUND','B  FILTER / TONE','C  LFO','D  GLIDE / AMP / DROP'if mono else'D  AMP / MOTION']
  dkeys=['Glide','AmpDecay','AmpRelease','ReleaseDropAmount']if mono else['AmpAttack','AmpRelease','ModRateControl','TremDepth']
  groups=frontgroups+[dkeys]
  captions=[['WAVE','PITCH','LEVEL','DRIFT'],['FILTER CUTOFF','RESONANCE','BODY','DRIVE'],['RATE','DEPTH','DELAY','TARGET'],['GLIDE','DECAY','RELEASE','PITCH DROP']if mono else['ATTACK','RELEASE','MOTION RATE','MOTION DEPTH']]
  for bank,keys in enumerate(groups):
   x=18+(bank%2)*632;y=94+(bank//2)*184;panel(ws,titles[bank],x,y)
   for j,key in enumerate(keys):knob(ws,key,captions[bank][j],x+8+j*150,y+57,locked=bank not in (0,2))
  ws+=segment('OscCharacterFocus',['DRIFT','PHASE'],'476 124 146 26')
  ws+=segment('OscFocus',['OSC 1','OSC 2'],'26 124 146 26');ws+=segment('PitchResolution',['TUNE','FINE'],'176 124 146 26')
  ws.extend([menu('Filter','658 124 146 26'),menu('DriveType','1108 124 146 26')])
  ws+=segment('PerfLfoSync',['FREE','SYNC'],'26 308 146 26');ws+=segment('LfoFocus',['LFO 1','LFO 2'],'176 308 146 26');ws.append(menu('PerfLfoShape','326 308 296 26'))
  # Shared/conditional lock ownership follows the currently edited canonical value.
  ws.append(lock('Wave1','18 151 44 44'));
  for focus,actual in enumerate(('Drift','OscPhase')):ws.append(lock(actual,'468 151 44 44',[f'IndexedEnabling/{focus}/2/{h("OscCharacterFocus")}']))
  for osc in (0,1):
   cond=[f'IndexedEnabling/{osc}/2/{h("OscFocus")}'];ws.append(lock('OscLevel'+str(osc+1),'318 151 44 44',cond))
   for fine in (0,1):ws.append(lock(('OscFine'if fine else'OscTune')+str(osc+1),'168 151 44 44',cond+[f'IndexedEnabling/{fine}/2/{h("PitchResolution")}']))
  for module in (0,1):
   cond=[f'IndexedEnabling/{module}/2/{h("LfoFocus")}']
   for key,x in zip(['Lfo2RateControl','Lfo2Depth','Lfo2Target']if module else['LfoRateControl','Depth','Target'],[18,168,468]):ws.append(lock(key,f'{x} 335 44 44',cond))
  if mono:
   ws+=segment('MonoLegato',['MONO','LEGATO'],'658 308 146 26');ws.append(menu('GlideLegato','808 308 296 26'));ws.append(touch_value('ReleaseDropTime','1108 308 146 26','TIME'))
  else:
   ws.extend([menu('ModMode','658 308 296 26'),menu('ModShape','1108 308 146 26')]);ws+=segment('ModSync',['FREE','SYNC'],'958 308 146 26')
  if mono:browse_footer(ws)
  else:
   # One browsing row leaves70px chord pads above the shared action bar.
   ws.extend([label('PRESET','18 456 70 44',18),command('SoundPrev','‹','90 456 44 44'),menu('SoundCurrent','138 456 180 44'),command('SoundNext','›','322 456 44 44')])
   ws.append(label('CHORD PRESET','374 456 108 44',16))
   for n,b in [('ChordPrev',[486,456,44,44]),('Progression',[534,456,180,44]),('ChordNext',[718,456,44,44])]:
    w=next(w for w in main if get(cd(w),'name')==n);ws.append(resize_instance(copy.deepcopy(w),b))
   ws+=segment('PerfVoice',['POLY','MONO'],'770 456 146 44')
   ws.extend([menu('DiceTarget','924 456 166 44'),menu('InputMode','1098 456 164 44')])
   actions_footer(ws)
   for w in main:
    n=get(cd(w),'name')
    if(n.startswith('Chord ')and n[6:].isdigit()):
     z=copy.deepcopy(w);bb=box(z);bb[1]=504;move(z,bb);ws.append(z)
    elif n in('‹','‹ touch','›','› touch')or(' / 'in n and '–'in n):
     z=copy.deepcopy(w);bb=box(z);bb[1]=578;move(z,bb);ws.append(z)
  maps.append(finish(tab,'MONO'if mono else'POLY',1 if mono else 0,0,ws,groups,['18 94 612 176','650 94 612 176','18 278 612 176','650 278 612 176']));newtabs.append(tab)
 # OSC detail, all oscillator fields stay individually visible.
 tab=old['OSC'];ws=copy.deepcopy(header);groups=[['Wave1Full','OscTune1','OscFine1','OscLevel1'],['Wave2Full','OscTune2','OscFine2','OscLevel2'],['Sub','Drift','OscPulse1','Pulse2'],['Noise','FM','Tone']]
 for bank,keys in enumerate(groups):
  x=18+(bank%2)*632;y=94+(bank//2)*208;panel(ws,['A  OSC 1','B  OSC 2','C  TEXTURE / PULSE','D  NOISE / FM / TONE'][bank],x,y,612,200)
  for j,key in enumerate(keys):knob(ws,key,{'Wave1Full':'WAVE','Wave2Full':'WAVE','OscPulse1':'PW 1','Pulse2':'PW 2'}.get(key,meta[ix[key]]['name'].replace('OSC 1 ','').replace('OSC 2 ','').upper()),x+8+j*150,y+62)
 browse_footer(ws);maps.append(finish(tab,'OSC',2,0,ws,groups,['18 94 612 200','650 94 612 200','18 302 612 200','650 302 612 200']));newtabs.append(tab)
 src_strip=Image.open(out/'env-fader.png').convert('RGBA');short_strip=Image.new('RGBA',(64,80*81))
 for frame in range(81):short_strip.paste(src_strip.crop((0,frame*96,64,(frame+1)*96)).resize((64,80),Image.Resampling.LANCZOS),(0,frame*80))
 short_strip.save(out/'perf-env-fader.png')
 # ENV retains native editable ADSR curves, compact faders and shared locks.
 tab=old['ENV'];ws=copy.deepcopy(header);original=get(get(tab,'componentDefinition'),'componentsData')
 for bank,prefix in enumerate(('Amp','Filt')):
  x=18+bank*632;panel(ws,('A  AMP ENVELOPE'if bank==0 else'B  FILTER ENVELOPE'),x,94,612,266)
  curve=next(copy.deepcopy(w)for w in original if get(cd(w),'type')=='Envelope'and get(cd(w),'name').startswith('Amp'if bank==0 else'Filter'));move(curve,[x+16,130,580,78]);ws.append(curve)
  for j,stage in enumerate(('Attack','Decay','Sustain','Release')):
   # Reuse the already validated 81-frame native fader and value-lock wrappers.
   source=next(w for w in original if get(cd(w),'name')==('Amp'if bank==0 else'Filter')+' '+stage);z=copy.deepcopy(source);move(z,[x+16+j*146,216,146,136]);ws.append(z)
   definition=defs[get(cd(z),'type')];clone=copy.deepcopy(definition);name='PerfEnv'+prefix+stage
   for child in get(clone,'componentsData'):
    xx,yy,ww,hh=box(child);typ=get(cd(child),'type')
    if typ=='Knob':
     move(child,[41,28,64,80]);put(get(cd(child),'data'),'filmStrip','perf-env-fader.png')
    elif typ=='Label':move(child,[44,0,100,24]if dict(get(cd(child),'data')).get('type')=='Name'else[8,112,130,24])
    elif typ=='Image':
     # Replace only the value plate with a native-sized LCD strip.
     put(get(cd(child),'data'),'image',scaled('value-lcd-v2.png',(130,24)));move(child,[8,112,130,24])
   add(name,clone);put(cd(z),'type',name)
 panel(ws,'C  FILTER RESPONSE',18,370,1244,142)
 for j,key in enumerate(('Cutoff','Resonance','Env','KeyTrack')):knob(ws,key,{'Cutoff':'FILTER CUTOFF','Env':'ENV AMOUNT'}.get(key,meta[ix[key]]['name'].upper()),80+j*300,400,compact=True)
 ws.append(menu('Filter','1004 372 240 26'));browse_footer(ws)
 groups=[['AmpAttack','AmpDecay','AmpSustain','AmpRelease'],['FiltAttack','FiltDecay','FiltSustain','FiltRelease'],['Cutoff','Resonance','Env','KeyTrack']]
 maps.append(finish(tab,'ENV',3,0,ws,groups,['18 214 612 142','650 214 612 142','18 400 1244 110']));newtabs.append(tab)
 # MOD exposes both LFOs; Delay deliberately has no Dice lock.
 tab=old['MOD'];ws=copy.deepcopy(header);groups=[['LfoRateControl','Depth','LfoDelay','Lfo1TargetFull'],['Lfo2RateControl','Lfo2Depth','Lfo2Delay','Lfo2Target'],['ModRateControl','TremDepth','Pan','Vibrato']]
 for bank in (0,1):
  x=18+bank*632;panel(ws,'A  LFO 1'if bank==0 else'B  LFO 2',x,94,612,200)
  ws.append(menu('Lfo1ShapeFull'if bank==0 else'Lfo2Shape',f'{x+158} 126 296 26'));ws+=segment('Sync'if bank==0 else'Lfo2Sync',['FREE','SYNC'],f'{x+8} 126 146 26')
  for j,key in enumerate(groups[bank]):knob(ws,key,['RATE','DEPTH','DELAY','TARGET'][j],x+8+j*150,166,compact=True)
 panel(ws,'C  MOTION',18,306,1244,206)
 ws.extend([menu('ModMode','392 338 296 26'),menu('ModShape','692 338 296 26')]);ws+=segment('ModSync',['FREE','SYNC'],'92 338 146 26')
 for j,key in enumerate(groups[2]):knob(ws,key,['RATE','DEPTH','PAN','VIBRATO'][j],92+j*300,380)
 browse_footer(ws);maps.append(finish(tab,'MOD',4,0,ws,groups,['18 156 612 138','650 156 612 138','18 370 1244 142']));newtabs.append(tab)
 # FX: eleven Q-Links, no master-volume Dice lock or fake fourth output control.
 tab=old['FX'];ws=copy.deepcopy(header);groups=[['Drive','DriveMix','Tone','LowCut'],['Chorus','Space','Delay','Feedback'],['OutputTrim','Volume','Pan']]
 for bank in range(3):
  y=94+bank*140;panel(ws,['A  DRIVE / COLOR','B  SPACE / ECHO','C  OUTPUT'][bank],18,y,1244,138)
  for j,key in enumerate(groups[bank]):knob(ws,key,{'Delay':'DELAY','OutputTrim':'SOUND TRIM','LowCut':'LOW CUT'}.get(key,meta[ix[key]]['name'].upper()),224+j*218,y+28,compact=True)
 ws.extend([menu('DriveType','36 142 174 44'),menu('FXPreset','1100 142 144 44'),menu('DelayDivision','36 282 174 44')]);browse_footer(ws)
 maps.append(finish(tab,'FX',5,0,ws,groups,['18 94 1244 138','18 234 1244 138','18 374 1244 138']));newtabs.append(tab)
 # CHORD: keep every existing bank/key/voicing/learn modal, reflow the body
 # around the same header and bottom actions used by the sound editors.
 chord=old['CHORD'];root=get(get(chord,'componentDefinition'),'componentsData');editor=defs['Editor_DTAllChords'];ew=get(editor,'componentsData');cw=copy.deepcopy(header)
 panel(cw,'CHORD EDITOR',18,94,1244,376)
 for w in cw:
  if get(cd(w),'name')=='CHORD EDITOR':move(w,[30,96,220,26])
 # Bank buttons / slot +/- are existing native commands with their conditions.
 for w in ew:
  n=get(cd(w),'name');x,y,width,height=box(w)
  if y==4 and n!='CHORD EDITOR':
   z=resize_instance(copy.deepcopy(w),[x,96,width,36]);cw.append(z)
  elif n=='KEY':cw.append(label('KEY','928 132 54 28',20))
  elif y==60 and get(cd(w),'type')!='Label':
   z=resize_instance(copy.deepcopy(w),[int(928+(x-898)*.72),134,int(width*.72),32]);cw.append(z)
  elif n=='Circle of fifths':
   z=copy.deepcopy(w);data=get(cd(z),'data');put(data,'image',scaled(get(data,'image'),(294,300)));move(z,[940,170,294,300]);cw.append(z)
 for caption,x,width in [('SLOT',28,118),('ROOT',150,136),('TYPE',292,270),('VOICING',570,220),('OCT',800,100)]:cw.append(label(caption,f'{x} 120 {width} 24',20))
 for row in range(4):
  y=146+row*80
  knob(cw,'EditRoot'+str(row+1),'ROOT',158,y,False,True)
  cw.append(menu('HarmonyEdit'+str(row+1),f'292 {y+15} 264 40'))
  cw.append(menu('EditVoicing'+str(row+1),f'570 {y+15} 214 40'))
  knob(cw,'EditOctave'+str(row+1),'OCT',800,y,False,True)
 # Actual 16 page-conditional audition pads retain Mouse Down and the top
 # ResetOnGestureEnd Slider. Resize their native art with their hit surface.
 for w in ew:
  n=get(cd(w),'name')
  if not n.startswith('Audition'):continue
  z=copy.deepcopy(w);slot=int(n.split('_',1)[0][8:])-1;row=slot%4;move(z,[28,150+row*80,118,64]);typ=get(cd(z),'type');name='Perf'+typ;d=copy.deepcopy(defs[typ])
  for child in get(d,'componentsData'):
   xx,yy,ww,hh=box(child);move(child,[int(xx*118/90),yy,int(ww*118/90),hh]);data=get(cd(child),'data')
   if get(cd(child),'type')=='Label'and dict(data).get('type')=='Text':put(data,'text',str(slot+1))
   if get(cd(child),'type')=='Button':
    for field in ('onImage','offImage'):
     src=dict(data).get(field)
     if src and src!='button-hit.png':put(data,field,scaled(src,(118,64)))
  add(name,d);put(cd(z),'type',name);cw.append(z)
 # Utility actions preserve their exact gesture/local handles and existing modals.
 util=[w for w in ew if box(w)[1]in(516,564)and get(cd(w),'type')not in('Button','Label')]
 positions=[(18,474,186,44),(214,474,174,44),(398,474,156,44),(564,474,214,44),(788,474,212,44),(1010,474,252,44)]
 selected=[]
 for w in util:
  typ=get(cd(w),'type');definition=defs.get(typ,[])
  text=' '.join(str(dict(get(cd(c),'data')).get('text',''))for c in dict(definition).get('componentsData',[]))
  if any(word in text for word in('CHORD GLIDE','GLIDE','STRUM','SMOOTH VOICING','TRANSPOSE','PAD SETUP')):selected.append(w)
 for w,b in zip(selected,positions):
  z=copy.deepcopy(w);_,_,oldw,oldh=box(w);move(z,b);typ=get(cd(z),'type');name='PerfChordUtility'+str(len(defs));d=copy.deepcopy(defs[typ])
  for child in get(d,'componentsData'):
   x,y,width,height=box(child);move(child,[round(x*b[2]/oldw),0,round(width*b[2]/oldw),b[3]]);data=get(cd(child),'data')
   if get(cd(child),'type')=='Image':put(data,'image',scaled(get(data,'image'),(b[2],b[3])))
   if get(cd(child),'type')=='Button':
    for field in('onImage','offImage'):
     src=dict(data).get(field)
     if src and src!='button-hit.png':put(data,field,scaled(src,(b[2],b[3])))
  add(name,d);put(cd(z),'type',name);cw.append(z)
 # One compact common row is sufficient while editing chords. Dice remains
 # sound-only and can never alter chord/voicing/key/MIDI values.
 cw.append(menu('SoundCurrent','18 526 330 44'));cw+=segment('PerfVoice',['POLY','MONO'],'368 526 190 44');cw.append(menu('InputMode','574 526 184 44'))
 actions_footer(cw)
 put(editor,'componentsData',cw)
 # Keep all modal roots; discard only the old body instance, not its drawers.
 body=copy.deepcopy(root[0]);move(body,[0,0,1280,628]);put(get(chord,'componentDefinition'),'componentsData',[body]+[w for w in root[1:]if get(cd(w),'type')in('DTKeyPadSetup','DTKeyPicker','DTKeyTranspose','DTKeySourcePicker','DTSmoothVoicing')])
 for key in ['HarmonyEdit'+str(i)for i in range(1,5)]+['EditVoicing'+str(i)for i in range(1,5)]+['EditOctave'+str(i)for i in range(1,5)]+['SoundCurrent','InputMode']:
  drawer(get(get(chord,'componentDefinition'),'componentsData'),key)
 put(chord,'fnKeyIndex',0);put(chord,'fnKeySubIndex',1);put(chord,'qlinkBoundsData',[f'18 {146+i*80} 888 80'for i in range(4)])
 maps.append(qmap(chord,[['EditRoot'+str(i),'HarmonyEdit'+str(i),'EditVoicing'+str(i),'EditOctave'+str(i)]for i in range(1,5)]));newtabs.append(chord)
 put(pd,'tabs',newtabs);put(get(ql,'Screen Mode Q-Links'),'map',maps);put(ql,'Program Mode Q-Links',copy.deepcopy(get(maps[0],'Q-Links')))
 # Choice parameters retain their Dice protection in the sidebar header,
 # keeping44px lock hits away from the compact front-panel selector row.
 for key,pindex in ix.items():
  panel_name='DTChoicePanel'+str(pindex)
  if panel_name not in defs:continue
  choices=[]
  if pindex in targets:choices=[(pindex,[])]
  elif key in('PerfWave','PerfWaveFull'):choices=[(ix['Wave1Full'],[])]
  elif key in('PerfLfoShape','PerfLfoTarget'):
   owners=['Shape','Lfo2Shape']if key=='PerfLfoShape'else['Target','Lfo2Target']
   choices=[(ix[k],[f'IndexedEnabling/{module}/2/{h("LfoFocus")}'])for module,k in enumerate(owners)]
  if not choices:continue
  panel_widgets=get(defs[panel_name],'componentsData')
  for w in panel_widgets:
   if get(cd(w),'type')=='Label'and box(w)[1]<=8 and box(w)[2]>250:
    move(w,[16,4,294,50]);put(get(get(get(cd(w),'data'),'textStyle'),'font'),'height',22.0)
  for actual,conditions in choices:
   owner_key=meta[actual]['key'];panel_widgets.append(lock(owner_key,'316 6 44 44',conditions))
 # Selected relays are shared sound editors, not separate tone memories.
 put(pd,'replacementStrings',[O(('key',p.handle),('value',list(p.choices)))for p in params if p.is_discrete])
