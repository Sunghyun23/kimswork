"""Performance pads follow MPC bottom-up numbering; generators stay off hit faces."""
def build_performance_skin(g,O,keys,get,put,H,label,image,define,instance,button,art,knob,conditional,base,definitions,native=False):
 slots=[4,5,0,1,8,9]
 children=base()+[image(art(760,40,'dark'),'16 94 760 40'),label('PAD BANK A','24 94 196 40',24,'fff7e9c7'),label('C MAJOR / A1-6 / ONE OCTAVE','220 94 548 40',18,'fff7e9c7'),image(art(464,400,'card'),'800 94 464 400'),*[conditional(label(('SNARE ROLL 'if n<4 else'FILL 'if n<8 else'HAT ROLL ')+chr(65+n%4),'812 98 440 36',27),'IndexedEnabling/'+str(n)+'/12/'+H('PerformSelectFull'))for n in range(12)],button('SELECT TO EDIT','PerformEdit','812 138 440 44',toggle=True)]
 for index,slot in enumerate(slots):
  pad=index+1;x=16+(index%2)*384;y=144+(index//2)*116;w=376;h=108
  key='PerformHit'+str(slot);state='PerformState'+str(slot);name='PerfPad'+str(pad)+('Native'if native else'')
  title=('FILL 'if slot<8 and slot>=4 else'SNARE ROLL 'if slot<4 else'HAT ROLL ')+chr(65+slot%4)
  parts=[g.entry('performance pad state','Button',g.button_data(art(w,h,'orange'),art(w,h,'cream'),2,3),'0 0 376 108','No','Show','Always',remap=[('Data',H(state))]),label('A'+str(pad),'10 6 50 22',18,'ff38261c'),label(title,'12 32 352 40',26,'ff38261c'),label('','12 78 352 22',19,'ff38261c',key='PerformRate'+str(slot))]
  parts.insert(1,conditional(image('dice-locked.png','322 2 44 44'),'IndexedEnabling/1/2/'+H('PerformLock'+str(slot))))
  parts.append(conditional(label('QUEUED','96 6 156 22',17),'IndexedEnabling/1/3/'+H(state)))
  surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+H(key)))
  parts.append(g.entry('performance hold release','Slider',surface,'0 0 376 108','No','Show','Always'))
  define(name,parts,[g.action('Mouse Down','Toggle Switch',H(key),'')]);children.append(instance(name,f'{x} {y} {w} {h}'))
 for q,(title,x,y)in enumerate([('RATE',846,192),('VELOCITY',1050,192),('HUMANIZE',846,310),('LENGTH',1050,310)]):children.append(knob('PerformQ'+str(q),title,x,y,190,physical=None if native else q))
 children +=[button('GENERATE','PerformGenerateSelected','812 444 212 44',kind='olive',toggle=True),button('LOCK / UNLOCK','PerformLockSelected','1036 444 216 44',toggle=True),label('','16 498 760 28',21,'fff7e9c7','PerformStatus'),label('','812 498 440 28',20,'fff7e9c7','CaptureInfo'),button('LEARN PAD','PerformLearn','16 534 164 40',toggle=True),label('','188 534 328 40',18,'fff7e9c7','PerformPadInfo'),conditional(button('REC FILL','PerformRecord','524 534 120 40',kind='red',toggle=True),'RangedEnabling/4/7/12/'+H('PerformSelectFull')),*([] if native else[conditional(button('EDIT NOTES','PerformEditFill','652 534 124 40',toggle=True),'RangedEnabling/4/7/12/'+H('PerformSelectFull')),button('BACK TO MAIN','PerformOpen','812 534 212 40',toggle=True)]),button('UNLOCK ALL','PerformUnlock','16 578 160 44',toggle=True),button('ROLL DICE','PerformGenerateAll','188 578 240 44',kind='orange',toggle=True),button('UNDO','PerformUndo','440 578 128 44',toggle=True),button('CAPTURE','Capture','584 578 328 44',kind='red',toggle=True),button('EXPORT MIDI','ExportMidi','924 578 340 44',toggle=True)]
 return children
