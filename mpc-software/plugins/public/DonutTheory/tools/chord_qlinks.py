"""A real CHORD sub-tab owns the four slot banks; overlays cannot remap Q-Links."""
import copy

def install(g,tui,ql,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(c):return get(c,'componentData')
 pd=get(tui,'pageData');tabs=get(pd,'tabs');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(d,'key'):get(d,'value') for d in entries};by={p.name:p for p in params}
 children=get(get(tabs[0],'componentDefinition'),'componentsData')
 def label(text,b,size=23):
  data=g.label_data('Text','ff352d21',size,'horizontallyCentred verticallyCentred',text);font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold')
  return g.entry(text,'Label',data,b,'No','Show','Always',remap=[('Data','Parameter 0')])
 editor=defs['DTAllChords'];widgets=get(editor,'componentsData')
 widgets[:]=[w for w in widgets if get(cd(w),'name') not in ('Done','Done touch','DONE')]
 for w in widgets:
  if get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('text','').startswith('ALL CHORDS'):
   put(get(cd(w),'data'),'text','CHORDS — Q-LINK SCREEN: A / B / C / D');put(get(get(get(cd(w),'data'),'textStyle'),'font'),'height',27.0)
 for w in widgets:
  if get(cd(w),'name')=='Cancel learn' and get(cd(w),'type')=='Button':
   data=get(cd(w),'data');put(data,'onImage',get(data,'offImage'))
 widgets.extend([label('FIRST TAB','1070 8 180 22',19),label('BACK TO MAIN','1070 30 180 22',19)])
 for i in range(4):
  for w in get(defs['DTAudition'+str(i+1)],'componentsData'):
   if get(cd(w),'type')=='Label' and get(get(cd(w),'data'),'type')=='Text':put(get(cd(w),'data'),'text',chr(65+i)+' · '+str(i+1))
 # Native choice replacement strings override dynamic formatValue text.
 # Waiting cues therefore depend directly on the actual Pad Learn parameter.
 for w in widgets:
  if get(cd(w),'type')=='Label' and dict(get(cd(w),'data')).get('type')=='Value':
   remaps=get(get(w,'handle remapping'),'map')
   if any(get(r,'value')==by['Pad Learn Status'].handle for r in remaps):
    put(get(w,'bounds'),'version',2);put(get(w,'bounds'),'showWhenDataModelInvalid','Hide');put(get(w,'bounds'),'additionalInvalidatingHandles',[f'IndexedEnabling/0/5/{by["Pad Learn"].handle}'])
 for slot in range(1,5):
  w=label(f'PRESS THE PHYSICAL PAD TO ASSIGN TO CHORD {slot}','20 49 970 28',23)
  put(get(w,'bounds'),'version',2);put(get(w,'bounds'),'showWhenDataModelInvalid','Hide');put(get(w,'bounds'),'additionalInvalidatingHandles',[f'IndexedEnabling/{slot}/5/{by["Pad Learn"].handle}']);widgets.append(w)
 # All four fields bank the matching slot before opening their controls.
 for slot in range(1,5):
  y=116+(slot-1)*100
  for suffix,x,w in [('Type',260,300),('Voice',575,220)]:
   param=by[f'Chord {slot} {suffix}'];b=f'{x} {y+17} {w} 44';key=f'DTChordField{slot}{suffix}'
   widgets[:]=[v for v in widgets if get(get(v,'bounds'),'bounds')!=b]
   text=g.label_data('Value','ff352d21',27,'horizontallyCentred verticallyCentred');font=get(get(text,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold')
   field=[g.entry('field art','Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',f'chord-{w}-off.png')),f'0 0 {w} 44','No','Show','Always'),g.entry('value','Label',text,f'0 0 {w} 44','No','Show','Always',remap=[('Data',param.handle)])]
   actions=[g.action('Mouse Down','Q-Link',param.handle,''),g.action('Mouse Down','Toggle Switch',f'Local/2/dtChoice{param.index}','')]
   entries.append(O(('key',key),('value',g.component_definition(actions,field,'00000000'))))
   widgets.append(g.entry(f'Chord {slot} {suffix}',key,O(('version',1),('handleName','Data')),b,'Yes','Show','Always',remap=[('Data',param.handle)]))
  get(defs['DTAudition'+str(slot)],'actions').append(g.action('Mouse Down','Q-Link',by[f'Chord {slot} Root'].handle,''))
 for slot in range(1,5):
  note=defs['DTChordNote'+str(slot)];put(note,'actions',[a for a in get(note,'actions') if get(a,'handler')!='Q-Link'])
 # The native guide is the single bank indicator. A second persistent row
 # outline can disagree with it when re-entering the tab, so do not draw one.
 # Reuse the full editor on the native sub-tab, with its existing choice overlays.
 extra_names={'DTChordAdvanced'}|{f'Select Chord {i} {suffix}' for i in range(1,5) for suffix in ('Type','Voice')}
 extra=[copy.deepcopy(c) for c in children if get(cd(c),'name') in extra_names]
 children[:]=[c for c in children if get(cd(c),'name')!='DTAllChords' and get(cd(c),'name') not in extra_names]
 page=g.component_definition([], [g.entry('Chord editor','DTAllChords',O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data','Parameter 0')])]+extra,'ffe1d2ae')
 tab=next(t for t in tabs if get(t,'tabName')=='CHORD');put(tab,'componentDefinition',page);put(tab,'fnKeyIndex',0);put(tab,'fnKeySubIndex',1);put(tab,'qlinkBoundsData',[f'96 {112+i*100} 708 96' for i in range(4)])
 qmap=get(get(ql,'Screen Mode Q-Links'),'map');mapping=next(m for m in qmap if get(m,'SubTab')==2);put(mapping,'Tab',1)
 assignment={}
 for slot in range(4):
  names=[f'Chord {slot+1} Root',f'Chord {slot+1} Type',f'Chord {slot+1} Voice',f'Chord {slot+1} Octave']
  for knob,name in enumerate(names):assignment[13+slot-4*knob]=by[name].index
 put(mapping,'Bank Direction','Column');put(mapping,'Q-Links',O(*[('Q-Link '+str(n),assignment[n]) for n in range(1,17)]))
 # No startup/help modal. CHORD is reached through the native first tab.
 children[:]=[c for c in children if get(cd(c),'name')=='background' or get(cd(c),'name') not in ('Edit all chords','Chord navigation')]
 # Hide bank rectangles over modal menus, restoring them on the real page.
 for entry in entries:
  name=get(entry,'key')
  if name.startswith('DTChoicePanel') or name in ('DTChordNavigation','DTChordAdvanced','DTLfoSettings'):
   comp=get(entry,'value');put(comp,'version',4)
   if 'hideQLinkBounds' not in dict(comp):
    pos=next(i for i,(k,_) in enumerate(comp) if k=='componentsData');comp.insert(pos,('hideQLinkBounds',True))

 # Fit detail bank guides to their actual columns, including only populated rows.
 # The SDK defaults extend every guide to the bottom, even on short pages.
 for detail in tabs:
  if get(detail,'tabName') not in ('OSC','FILTER','AMP','MOD','FX'):continue
  columns=[[] for _ in range(4)]
  for c in get(get(detail,'componentDefinition'),'componentsData'):
   typ=get(cd(c),'type')
   if not typ.startswith(('knob','fader','radio','slider')):continue
   x,y,w,h=map(int,get(get(c,'bounds'),'bounds').split())
   columns[min(3,x//320)].append((x,y,w,h))
  put(detail,'qlinkBoundsData',[f'{18+320*i} 0 288 {max(y+h for x,y,w,h in col)+8}' for i,col in enumerate(columns)])
 # restore_design normalises text colours globally; restore radio contrast last.
 for name,definition in defs.items():
  if not name.startswith('radio'):continue
  for c in get(definition,'componentsData'):
   if get(cd(c),'type')=='Label' and ' seg ' in get(cd(c),'name'):put(get(get(cd(c),'data'),'textStyle'),'colour','fff3dfb8')
