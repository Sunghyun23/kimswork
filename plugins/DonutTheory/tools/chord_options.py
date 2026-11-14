"""Complete chord inversions and per-slot octave without expanding the 16 Q-Links."""
def install(g,tui,params,out):
 import copy
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_) in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(c):return get(c,'componentData')
 pd=get(tui,'pageData');tabs=get(pd,'tabs');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(d,'key'):get(d,'value') for d in entries};by={p.name:p for p in params}
 editor=get(defs['DTAllChords'],'componentsData');page=get(next(t for t in tabs if get(t,'tabName')=='CHORD'),'componentDefinition');children=get(page,'componentsData')
 def label(text,b,size=25,handle=None):
  data=g.label_data('Value' if handle else 'Text','ff352d21',size,'horizontallyCentred verticallyCentred',None if handle else text);font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold')
  return g.entry(text,'Label',data,b,'No','Show','Always',remap=[('Data',handle or 'Parameter 0')])
 def button(text,handle,b,index,count):
  return [g.entry(text,'Button',g.button_data('list-on.png','list-off.png',index,count),b,'Yes','Show','Always',remap=[('Data',handle)]),label(text,b),g.entry(text+' touch','Button',g.button_data('button-hit.png','button-hit.png',index,count),b,'Yes','Show','Always',remap=[('Data',handle)])]
 def panel(title,local,widgets):
  ws=[label(title,'20 8 980 42',30),g.entry('Done','Button',g.button_data('list-close.png','list-close.png',0,2),'1135 10 120 38','Yes','Show','Always',remap=[('Data',local)]),label('DONE','1135 10 120 38',24),g.entry('Done touch','Button',g.button_data('button-hit.png','button-hit.png',0,2),'1135 10 120 38','Yes','Show','Always',remap=[('Data',local)])]+widgets
  comp=g.component_definition([],ws,'ffe1d2ae');put(comp,'version',4);pos=next(i for i,(k,_) in enumerate(comp) if k=='componentsData');comp.insert(pos,('hideQLinkBounds',True));return comp
 for c in editor:
  if get(cd(c),'name')=='VOICING':put(get(c,'bounds'),'bounds','575 81 150 28')
 editor.append(label('OCT','726 81 76 28',23))
 for slot in range(1,5):
  y=133+(slot-1)*100
  # Make room within the existing voicing column, keeping root/type/pad geometry.
  field=defs[f'DTChordField{slot}Voice']
  for c in get(field,'componentsData'):
   put(get(c,'bounds'),'bounds','0 0 150 44')
   if get(cd(c),'type')=='Image':put(get(cd(c),'data'),'image','chord-150-off.png')
   if get(cd(c),'type')=='Label':put(get(get(get(cd(c),'data'),'textStyle'),'font'),'height',25.0)
  for c in editor:
   if get(cd(c),'name')==f'Chord {slot} Voice':put(get(c,'bounds'),'bounds',f'575 {y} 150 44')
  p=by[f'Chord {slot} Octave'];local=f'Local/2/dtChoice{p.index}';key=f'DTChordOctave{slot}'
  # The fourth editing control is an octave donut, matching physical Q-Link 4.
  # MIDI note remains a readout; double-tap still permits manual assignment.
  source=defs[f'DTChordNote{slot}'];note_widgets=get(source,'componentsData')
  knob=copy.deepcopy(next(c for c in note_widgets if get(cd(c),'type')=='Knob'))
  put(get(knob,'bounds'),'bounds','10 0 56 56')
  field_widgets=[knob,label('', '0 61 76 27',22,p.handle),g.entry('Open octave list touch','Button',g.button_data('button-hit.png','button-hit.png',1,2),'0 61 76 27','Yes','Show','Always',remap=[('Data',local)])]
  actions=[g.action('Mouse Down','Q-Link',p.handle,''),g.action('Double Click','Toggle Switch',local,''),g.action('Enter Pressed','Toggle Switch',local,'')]
  entries.append(O(('key',key),('value',g.component_definition(actions,field_widgets,'00000000'))))
  editor.append(g.entry(f'Chord {slot} Octave',key,O(('version',1),('handleName','Data')),f'726 {y-17} 76 90','Yes','Show','Always',remap=[('Data',p.handle)]))
  note_widgets[:]=[copy.deepcopy(c) for c in note_widgets if get(cd(c),'type')=='Label']
  for c in note_widgets:put(get(c,'bounds'),'bounds','0 17 146 44')
  ws=[]
  for index,text in enumerate(p.choices):ws+=button(text,p.handle,f'{20+(index%4)*310} {64+(index//4)*63} 300 56',index,5)
  ws.append(label('OCTAVE SHIFTS THIS CHORD ONLY. ROOT AND PAD NOTE STAY UNCHANGED.','20 530 1240 60',22))
  panel_key=f'DTChoicePanel{p.index}';entries.append(O(('key',panel_key),('value',panel(f'CHORD {slot} — OCTAVE',local,ws))))
  children.append(g.entry('Select '+p.name,panel_key,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Show','Always',remap=[('Data',p.handle)],invalidating=[f'IndexedEnabling/1/2/{local}']))
  # Show only the inversions that exist for the current chord's actual tone count.
  voice=by[f'Chord {slot} Voice'];quality=by[f'Chord {slot} Type'];rows=[]
  for q,count in enumerate((4,4,5,5,4,5,5,3,3,3,5)):
   choices=[(0,'Root position'),(1,'1st inversion'),(3,'2nd inversion')]
   if count>=4:choices.append((4,'3rd inversion'))
   if count>=5:choices.append((5,'4th inversion'))
   choices.append((2,'Open voicing'))
   ws=[]
   for j,(index,text) in enumerate(choices):ws+=button(text,voice.handle,f'{20+(j%4)*310} {64+(j//4)*63} 300 56',index,6)
   rowkey=f'DTChordVoicingRows{slot}-{q}';entries.append(O(('key',rowkey),('value',g.component_definition([],ws,'00000000',ignore_mouse=True))))
   rows.append(g.entry(rowkey,rowkey,O(('version',1),('handleName','Data')),'0 0 1280 628','Children','Hide','Always',remap=[('Data',voice.handle)],invalidating=[f'IndexedEnabling/{q}/11/{quality.handle}']))
  rows.append(label('INVERSION CHANGES THE BASS NOTE. OPEN SPREADS THE CHORD TONES.','20 530 1240 60',22))
  comp=panel(f'CHORD {slot} — VOICING',f'Local/2/dtChoice{voice.index}',rows)
  existing=defs[f'DTChoicePanel{voice.index}'];existing[:]=comp
