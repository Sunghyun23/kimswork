"""SOUND editing page. Four fixed SCREEN relay groups; no invented tab-navigation handler."""
from pathlib import Path

def build_sound_skin(g,O,tabs,keys,get,put,H,label,image,define,instance,button,art,knob,value,conditional,base):
 tab=next(t for t in tabs if get(t,'tabName')=='SOUND')
 import copy
 def not_hat(entry):return [conditional(copy.deepcopy(entry),'RangedEnabling/0/1/4/'+H('SoundPart')),conditional(entry,'IndexedEnabling/3/4/'+H('SoundPart'))]
 selected_defs=set();sample_defs=set()
 def selector(row,y):
  name='SoundRowSelect'+str(row)
  if name in selected_defs:return instance(name,f'20 {y} 160 40')
  selected_defs.add(name)
  parts=[g.entry('selected row','Button',g.button_data(art(160,40,'orange'),art(160,40,'dark'),row,4),'0 0 160 40','No','Show','Always',remap=[('Data',H('SoundRow'))]),label('', '0 0 160 40',20,'fff6e4c4','SoundRowInfo'+str(row)),g.entry('caption hit','Button',g.button_data('hit.png','hit.png',row,4),'0 0 160 40','No','Show','Always',remap=[('Data',H('SoundRow'))])]
  define(name,parts);return instance(name,f'20 {y} 160 40')
 def sample(row,y):
  key='SoundQ'+str(row*4);name='LayerSample'+str(row)
  if name in sample_defs:return instance(name,f'244 {y} 224 108')
  sample_defs.add(name)
  parts=[label('SAMPLE','0 0 224 22',20),button('','SoundBrowse'+str(row),'0 24 224 40',kind='olive',toggle=True),label('','8 24 208 40',18,'fff8e7c7',key),button('<','SoundSamplePrev'+str(row),'0 64 44 44',toggle=True),button('PREVIEW','SoundPreview'+str(row),'48 64 128 44',kind='dark',toggle=True),button('>','SoundSampleNext'+str(row),'180 64 44 44',toggle=True)]
  define(name,parts,[g.action('Mouse Down','Q-Link',H(key),'')]);return instance(name,f'244 {y} 224 108')
 main=base()
 for j,text in enumerate(['KICK','SNARE','HI-HAT','PERC']):main.append(button(text,'SoundPart',f'{16+j*192} 88 180 42',j,4,kind='dark'))
 main.extend([label('USER KIT','800 88 160 42',20,'fff2dfbd'),value('UserKit','966 88 130 42'),button('CYMBALS','CymbalView','1108 88 156 42',toggle=True)])
 guides=[]
 for row in range(4):
  y=136+row*110;main.extend([image(art(1248,108,'card'),f'16 {y} 1248 108'),selector(row,y+2)])
  for j,(text,field) in enumerate([('ON','On'),('SOLO','Solo'),('LOCK','Lock')]):
   entry=button(text,'Sound'+field+str(row),f'{20+j*54} {y+52} 50 44',kind='dark',toggle=True)
   if row>=2:main.extend(not_hat(entry))
   else:main.append(entry)
  regular=[sample(row,y)]
  for j,text in enumerate(['Level','Decay','Shift'],1):regular.append(knob('SoundQ'+str(row*4+j),text,244+j*250,y,224))
  if row>=2:
   regular=[variant for entry in regular for variant in not_hat(entry)]
   # Hat C/D are useful single-voice detail groups rather than phantom empty layers.
   for j,text in enumerate(['Start','Tune','Low Cut','High Cut']):
    entry=knob('SoundQ'+str(row*4+j),text,244+j*250,y,224);conditional(entry,'IndexedEnabling/2/4/'+H('SoundPart'));main.append(entry)
  main.extend(regular);guides.append(f'196 {y} 1068 108')
 main.extend([button('DETAILS','SoundDetails','16 580 210 40',toggle=True),button('ROLL SOUND','SoundRoll','238 580 282 40',kind='orange',toggle=True),button('UNDO','SoundUndo','532 580 164 40',toggle=True),button('AUDITION','SoundAudition','708 580 260 40',kind='dark',toggle=True),button('SAVE KIT','SaveKit','980 580 284 40',toggle=True)])
 details=base()+[label('SELECTED SOUND / DETAILS','16 90 720 38',25,'fff1dfbb')]
 for row in range(4):
  entry=instance('SoundRowSelect'+str(row),f'{16+row*192} 138 160 40')
  if row>=2:details.extend(not_hat(entry))
  else:details.append(entry)
 details.extend([label('A / SAMPLE START · TUNE · FILTER','16 202 600 32',22,'fff5e5c3'),image(art(1248,160,'card'),'16 240 1248 160')])
 for j,text in enumerate(['Start','Tune','Low Cut','High Cut']):details.append(knob('SoundQ'+str(j),text,60+j*304,260,264))
 details.extend([button('INVERT POLARITY','SoundInvert','16 434 360 60',kind='dark',toggle=True),label('Shift controls timing. Start trims the sample.','404 434 840 60',23,'fff2dfbd'),button('BACK TO SOUND','SoundDetails','16 566 350 50',0,2),button('AUDITION','SoundAudition','386 566 400 50',kind='orange',toggle=True),button('SAVE KIT','SaveKit','806 566 458 50',toggle=True)])
 cym=base()+[label('CYMBAL SOUNDS','16 88 500 42',25,'fff2dfbd'),button('BACK TO DRUMS','CymbalView','964 88 300 42',toggle=True)]
 for row in range(4):
  y=136+row*110;cym.extend([image(art(1248,108,'card'),f'16 {y} 1248 108'),selector(row,y+2) if row<3 else label('DETAILS',f'20 {y+2} 160 40',20)])
  if row<3:
   for j,(text,field) in enumerate([('ON','On'),('SOLO','Solo'),('LOCK','Lock')]):cym.append(button(text,'Sound'+field+str(row),f'{20+j*54} {y+52} 50 44',kind='dark',toggle=True))
   cym.append(sample(row,y))
   for j,text in enumerate(['Level','Decay','Shift'],1):cym.append(knob('SoundQ'+str(row*4+j),text,244+j*250,y,224))
  else:
   for j,text in enumerate(['Start','Tune','Low Cut','High Cut']):cym.append(knob('SoundQ'+str(row*4+j),text,244+j*250,y,224))
 cym.extend([button('PREVIEW SELECTED','SoundAudition','16 580 608 40',kind='dark',toggle=True),button('SAVE KIT','SaveKit','640 580 624 40',toggle=True)])
 define('SoundOverview',main);define('SoundDetailsPanel',details,modal=True);define('CymbalOverview',cym)
 normal=[instance('SoundOverview',invalid=['IndexedEnabling/0/2/'+H('SoundDetails')]),instance('SoundDetailsPanel',invalid=['IndexedEnabling/1/2/'+H('SoundDetails')])]
 define('NormalSoundView',normal)
 roots=[instance('NormalSoundView',invalid=['IndexedEnabling/0/2/'+H('CymbalView')]),instance('CymbalOverview',invalid=['IndexedEnabling/1/2/'+H('CymbalView')])]
 # Native-style left sidebar. Its modal definition suppresses background Q-Link guides.
 browser=[image(art(426,628,'dark'),'0 0 426 628'),button('<','PackPrev','6 2 48 40',toggle=True),label('','60 2 246 40',20,'fff8e8c6','PackInfo'),button('>','PackNext','310 2 48 40',toggle=True),button('X','SampleBrowserOpen','366 2 54 40',toggle=True),label('','6 48 414 40',23,'fff8e8c6','SampleInfo')]
 for row in range(8):
  y=96+row*49
  browser.extend([button('','SamplePick'+str(row),f'6 {y} 414 46',kind='olive',toggle=True),label('',f'12 {y} 402 46',21,'fff8e8c6','SampleRow'+str(row))])
 browser.extend([button('< PREV','SamplePrev','6 496 202 46',toggle=True),button('NEXT >','SampleNext','218 496 202 46',toggle=True),button('PREVIEW','SoundAudition','6 552 202 60',kind='orange',toggle=True),button('DONE','SampleBrowserOpen','218 552 202 60',toggle=True),g.entry('outside sample close','Button',g.button_data('hit.png','hit.png',0,2),'426 0 854 628','No','Show','Always',remap=[('Data',H('SampleBrowserOpen'))])])
 define('SampleBrowser',browser,modal=True);roots.append(instance('SampleBrowser',invalid=['IndexedEnabling/1/2/'+H('SampleBrowserOpen')]))
 for other in tabs:
  if get(other,'tabName')!='SOUND':get(get(other,'componentDefinition'),'componentsData').append(instance('SampleBrowser',invalid=['IndexedEnabling/1/2/'+H('SampleBrowserOpen')]))
 put(tab,'componentDefinition',g.component_definition([],roots,'ffeed9b3'));put(tab,'qlinkBoundsData',guides)
