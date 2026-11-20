"""Performance pads follow MPC bottom-up numbering; generators stay off hit faces."""
def build_performance_skin(g,O,keys,get,put,H,label,image,define,instance,button,art,knob,conditional,base,definitions,native=False):
 children=base()+[image(art(760,40,'dark'),'16 94 760 40'),label('PAD BANK B','24 94 196 40',24,'fff7e9c7'),label('LOOP: ON/OFF   FILL / ROLL: BAR END','220 94 548 40',18,'fff7e9c7'),image(art(464,400,'card'),'800 94 464 400'),label('','812 98 440 36',27,key='PerformSelectFull'),button('SELECT TO EDIT','PerformEdit','812 138 440 44',toggle=True)]
 for row in range(4):
  for col in range(4):
   pad=(3-row)*4+col+1;x=16+col*192;y=144+row*88;w=184;h=80;slot=pad-1 if pad<=12 else -1
   if pad<=12:
    key='PerformHit'+str(slot);state='PerformState'+str(slot);name='PerfPad'+str(pad)+('Native'if native else'')
    parts=[g.entry('performance pad state','Button',g.button_data(art(w,h,'olive'if pad<=4 else'orange'if pad<=8 else'cream'),art(w,h,'orange'if pad<=4 else'cream'if pad<=8 else'orange'),2,3),'0 0 184 80','No','Show','Always',remap=[('Data',H(state))]),label('B'+str(pad),'8 4 38 18',16,'fff7e9c7'if pad<=4 else'ff38261c'),label(('ROLL 'if pad<=4 else'FILL 'if pad<=8 else'HAT ')+chr(65+(pad-1)%4),'8 22 168 38',26,'fff7e9c7'if pad<=4 else'ff38261c'),label('','10 58 164 20',18,'fff7e9c7'if pad<=4 else'ff38261c',key='PerformRate'+str(slot))]
    parts.insert(1,conditional(image('dice-locked.png','136 2 44 44'),'IndexedEnabling/1/2/'+H('PerformLock'+str(slot))))
    parts.append(conditional(label('QUEUED','48 2 82 18',14),'IndexedEnabling/1/3/'+H(state)))
    # Gesture release clears the UI press; engine playback completes its selected length.
    surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+H(key)))
    parts.append(g.entry('performance hold release','Slider',surface,'0 0 184 80','No','Show','Always'))
    define(name,parts,[g.action('Mouse Down','Toggle Switch',H(key),'')]);children.append(instance(name,f'{x} {y} {w} {h}'))
   elif pad in (13,14):
    key='PerformLoop'if pad==13 else'PerformStop';text='LOOP'if pad==13 else'STOP'
    entry=button('B'+str(pad)+'  '+text,key,f'{x} {y} {w} {h}',kind='olive'if pad==13 else'cream',toggle=True)
    if pad==13:
     definition=next(get(d,'value')for d in definitions if get(d,'key')==get(get(entry,'componentData'),'type'))
     put(get(get(definition,'componentsData')[0],'handle remapping'),'map',[O(('key','Data'),('value',H('PerformLoopState')))])
    children.append(entry)
   else:children +=[image(art(w,h,'dark'),f'{x} {y} {w} {h}'),label('B'+str(pad)+ '   —',f'{x} {y} {w} {h}',20,'ffb49b75')]
 for q,(title,x,y)in enumerate([('RATE',846,192),('VELOCITY',1050,192),('HUMANIZE',846,310),('LENGTH',1050,310)]):children.append(knob('PerformQ'+str(q),title,x,y,190,physical=None if native else q))
 children +=[button('GENERATE','PerformGenerateSelected','812 444 212 44',kind='olive',toggle=True),button('LOCK / UNLOCK','PerformLockSelected','1036 444 216 44',toggle=True),label('','16 498 760 28',21,'fff7e9c7','PerformStatus'),label('','812 498 440 28',20,'fff7e9c7','CaptureInfo'),button('LEARN PAD','PerformLearn','16 534 164 40',toggle=True),label('','188 534 328 40',18,'fff7e9c7','PerformPadInfo'),conditional(button('REC FILL','PerformRecord','524 534 120 40',kind='red',toggle=True),'RangedEnabling/4/7/12/'+H('PerformSelectFull')),*([] if native else[conditional(button('EDIT NOTES','PerformEditFill','652 534 124 40',toggle=True),'RangedEnabling/4/7/12/'+H('PerformSelectFull')),button('BACK TO MAIN','PerformOpen','812 534 212 40',toggle=True)]),button('ROLL DICE','Roll','16 578 248 48',kind='orange',toggle=True),button('UNLOCK ALL','UnlockDice','276 578 220 48',toggle=True),button('UNDO','Undo','508 578 172 48',toggle=True),button('CAPTURE','Capture','692 578 264 48',kind='red',toggle=True),button('EXPORT MIDI','ExportMidi','968 578 296 48',toggle=True)]
 return children
