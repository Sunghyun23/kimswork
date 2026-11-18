"""Preview voice-leading without mutating the stored progression."""
def install(g,tui,params,helpers):
 label,button,command,background,outside,modal,instance,h,get,visible=helpers
 pd=get(tui,'pageData');tab=next(t for t in get(pd,'tabs')if get(t,'tabName')=='CHORD')
 definitions=get(get(pd,'componentDefinitions'),'localComponentDefinitions')
 editor=get(next(get(d,'value')for d in definitions if get(d,'key')=='Editor_DTAllChords'),'componentsData')
 editor.append(command('SMOOTH VOICING',h('Smooth Voicing'),'620 516 210 44'))
 widgets=[background('0 0 426 628'),label('SMOOTH VOICING','12 8 340 44',25),button('X',h('Smooth Voicing'),'366 8 48 44',0,2),button('CURRENT 4',h('Smooth Scope'),'12 64 198 44',0,2),button('ALL SLOTS',h('Smooth Scope'),'216 64 198 44',1,2),label('First chord anchors the top note.','12 112 402 28',21),label('TOP NOTE: BEFORE > AFTER','12 144 402 28',22)]
 for row in range(16):
  item=label('',f'{12+(row//8)*204} {180+(row%8)*36} 198 34',24,h('Top Note '+str(row+1)))
  if row>=4:
   group=row//4;condition=f'IndexedEnabling/3/4/{h("Chord Slots")}' if group==3 else f'RangedEnabling/{group}/3/4/{h("Chord Slots")}'
   visible(item,[f'IndexedEnabling/1/2/{h("Smooth Scope")}',condition])
  widgets.append(item)
 # A native indexed Button always selects ON; use Mouse Down Toggle Switch
 # and an independent local release surface for a real start/stop toggle.
 preview=command('PREVIEW',h('Preview Smooth Voicing'),'148 564 128 48')
 kind=get(get(preview,'componentData'),'type')
 definition=next(get(d,'value')for d in definitions if get(d,'key')==kind)
 art=get(definition,'componentsData')[0]
 remap=get(get(art,'handle remapping'),'map')[0]
 for i,(key,value)in enumerate(remap):
  if key=='value':remap[i]=(key,h('Preview Smooth Voicing'))
 widgets.extend([label('','12 478 402 36',25,h('Top Note Movement')),label('','12 516 402 36',22,h('Smooth Status')),button('CANCEL',h('Smooth Voicing'),'12 564 128 48',0,2),preview,command('APPLY',h('Apply Smooth Voicing'),'284 564 128 48'),outside(h('Smooth Voicing'))])
 modal('DTSmoothVoicing',widgets)
 get(get(tab,'componentDefinition'),'componentsData').append(instance('DTSmoothVoicing','0 0 1280 628',[f'IndexedEnabling/1/2/{h("Smooth Voicing")}']))
