"""SOUND editing page. Four fixed SCREEN relay groups; no invented tab-navigation handler."""
from pathlib import Path

def build_sound_skin(g,O,tabs,keys,get,put,H,label,image,define,instance,button,art,knob,value,conditional,base):
 tab=next(t for t in tabs if get(t,'tabName')=='SOUND')
 import copy
 def not_hat(entry):return [conditional(copy.deepcopy(entry),'RangedEnabling/0/1/4/'+H('SoundPart')),conditional(entry,'IndexedEnabling/3/4/'+H('SoundPart'))]
 def selector(row,y):
  name='SoundRowSelect'+str(row)
  parts=[g.entry('selected row','Button',g.button_data(art(160,40,'orange'),art(160,40,'dark'),row,4),'0 0 160 40','No','Show','Always',remap=[('Data',H('SoundRow'))]),label('', '0 0 160 40',20,'fff6e4c4','SoundRowInfo'+str(row)),g.entry('caption hit','Button',g.button_data('hit.png','hit.png',row,4),'0 0 160 40','No','Show','Always',remap=[('Data',H('SoundRow'))])]
  define(name,parts);return instance(name,f'20 {y} 160 40')
 # A transparent filmstrip supplies drag/Q-Link input without native ValueSlider text painting over the name.
 from PIL import Image
 Image.new('RGBA',(64,64*127),(0,0,0,0)).save(Path(__file__).resolve().parents[1]/'skins/sample-touch.png')
 def sample(row,y):
  key='SoundQ'+str(row*4);name='LayerSample'+str(row)
  data=g.knob_filmstrip_data('sample-touch.png',127,'Vertical')
  parts=[label('SAMPLE','0 0 224 22',20),image(art(224,40,'olive'),'0 24 224 40'),label('','8 24 208 40',18,'fff8e7c7',key),g.entry('sample select','Knob',data,'0 24 224 40','Yes','Show','Always',remap=[('Data',H(key))]),button('<','SoundSamplePrev'+str(row),'0 64 44 44',toggle=True),label('DRAG / Q-LINK','48 64 128 44',15,'ff5a4030'),button('>','SoundSampleNext'+str(row),'180 64 44 44',toggle=True)]
  define(name,parts,[g.action('Mouse Down','Q-Link',H(key),'')]);return instance(name,f'244 {y} 224 108')
 main=base()
 for j,text in enumerate(['KICK','SNARE','HI-HAT','PERC']):main.append(button(text,'SoundPart',f'{16+j*192} 88 180 42',j,4,kind='dark'))
 main.extend([label('USER KIT','800 88 160 42',20,'fff2dfbd'),value('UserKit','966 88 130 42'),label('SOUND EDIT','1108 88 156 42',20,'fff2dfbd')])
 guides=[]
 for row in range(4):
  y=136+row*110;main.extend([image(art(1248,108,'card'),f'16 {y} 1248 108'),selector(row,y+2)])
  for j,(text,field) in enumerate([('ON','On'),('SOLO','Solo'),('LOCK','Lock')]):
   entry=button(text,'Sound'+field+str(row),f'{20+j*54} {y+52} 50 44',kind='dark',toggle=True)
   if row>=2:main.extend(not_hat(entry))
   else:main.append(entry)
  regular=[sample(row,y)]
  for j,text in enumerate(['Level','Tune','Shift'],1):regular.append(knob('SoundQ'+str(row*4+j),text,244+j*250,y,224))
  if row>=2:
   regular=[variant for entry in regular for variant in not_hat(entry)]
   # Hat C/D are useful single-voice detail groups rather than phantom empty layers.
   for j,text in enumerate(['Start','Low Cut','High Cut','Decay']):
    entry=knob('SoundQ'+str(row*4+j),text,244+j*250,y,224);conditional(entry,'IndexedEnabling/2/4/'+H('SoundPart'));main.append(entry)
  main.extend(regular);guides.append(f'196 {y} 1068 108')
 main.extend([button('DETAILS','SoundDetails','16 580 210 40',toggle=True),button('ROLL SOUND','SoundRoll','238 580 282 40',kind='orange',toggle=True),button('UNDO','SoundUndo','532 580 164 40',toggle=True),button('AUDITION','SoundAudition','708 580 260 40',kind='dark',toggle=True),button('SAVE KIT','SaveKit','980 580 284 40',toggle=True)])
 details=base()+[label('SELECTED SOUND / DETAILS','16 90 720 38',25,'fff1dfbb')]
 for row in range(4):
  entry=instance('SoundRowSelect'+str(row),f'{16+row*192} 138 160 40')
  if row>=2:details.extend(not_hat(entry))
  else:details.append(entry)
 details.extend([label('A / SAMPLE START · FILTER · ENVELOPE','16 202 600 32',22,'fff5e5c3'),image(art(1248,160,'card'),'16 240 1248 160')])
 for j,text in enumerate(['Start','Low Cut','High Cut','Decay']):details.append(knob('SoundQ'+str(j),text,60+j*304,260,264))
 details.extend([button('INVERT POLARITY','SoundInvert','16 434 360 60',kind='dark',toggle=True),label('Shift controls timing. Start trims the sample.','404 434 840 60',23,'fff2dfbd'),button('BACK TO SOUND','SoundDetails','16 566 350 50',0,2),button('AUDITION','SoundAudition','386 566 400 50',kind='orange',toggle=True),button('SAVE KIT','SaveKit','806 566 458 50',toggle=True)])
 define('SoundOverview',main);define('SoundDetailsPanel',details,modal=True)
 roots=[instance('SoundOverview',invalid=['IndexedEnabling/0/2/'+H('SoundDetails')]),instance('SoundDetailsPanel',invalid=['IndexedEnabling/1/2/'+H('SoundDetails')])]
 put(tab,'componentDefinition',g.component_definition([],roots,'ffeed9b3'));put(tab,'qlinkBoundsData',guides)
