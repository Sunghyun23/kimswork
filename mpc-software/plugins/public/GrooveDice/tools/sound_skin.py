"""SOUND editing page. Four fixed SCREEN relay groups; no invented tab-navigation handler."""
from pathlib import Path

def build_sound_skin(g,O,tabs,keys,get,put,H,label,image,define,instance,button,art,knob,value,conditional,base):
 tab=next(t for t in tabs if get(t,'tabName')=='SOUND')
 import copy
 def not_hat(entry):return [conditional(copy.deepcopy(entry),'RangedEnabling/0/1/4/'+H('SoundPart')),conditional(entry,'IndexedEnabling/3/4/'+H('SoundPart'))]
 selected_defs=set();sample_defs=set()
 def selector(row,y,compact=False):
  name='SoundRowSelect'+str(row)+('Compact'if compact else'');width=138 if compact else 188
  if name in selected_defs:return instance(name,f'20 {y} {width} 40')
  selected_defs.add(name)
  parts=[g.entry('selected row','Button',g.button_data(art(width,40,'orange'),art(width,40,'dark'),row,4),f'0 0 {width} 40','No','Show','Always',remap=[('Data',H('SoundRow'))]),label('', f'0 0 {width} 40',20,'fff6e4c4','SoundRowInfo'+str(row)),g.entry('caption hit','Button',g.button_data('hit.png','hit.png',row,4),f'0 0 {width} 40','No','Show','Always',remap=[('Data',H('SoundRow'))])]
  define(name,parts);return instance(name,f'20 {y} {width} 40')
 def sample(row,y):
  key='SoundQ'+str(row*4);name='LayerSample'+str(row)
  if name in sample_defs:return instance(name,f'256 {y} 224 108')
  sample_defs.add(name)
  parts=[label('SAMPLE','0 0 224 22',20),button('','SoundBrowse'+str(row),'0 24 224 40',kind='olive',toggle=True),label('','8 24 208 40',18,'fff8e7c7',key),button('<','SoundSamplePrev'+str(row),'0 64 44 44',toggle=True),button('PREVIEW','SoundPreview'+str(row),'48 64 128 44',kind='dark',toggle=True),button('>','SoundSampleNext'+str(row),'180 64 44 44',toggle=True)]
  define(name,parts,[g.action('Mouse Down','Q-Link',H(key),'')]);return instance(name,f'256 {y} 224 108')
 def footer():
  controls=[button('<','SoundDetails','16 574 48 48',toggle=True),conditional(label('SOUND 1 / 2','72 574 140 48',20,'fff4e3bf'),'IndexedEnabling/0/2/'+H('SoundDetails')),conditional(label('SOUND 2 / 2','72 574 140 48',20,'fff4e3bf'),'IndexedEnabling/1/2/'+H('SoundDetails')),button('>','SoundDetails','220 574 48 48',toggle=True)]
  controls.extend([button('ROLL SOUND','SoundRoll','280 574 256 48',kind='orange',toggle=True),button('UNDO','SoundUndo','548 574 132 48',toggle=True),button('AUDITION','SoundAudition','692 574 256 48',kind='dark',toggle=True),button('SAVE KIT','SaveKit','960 574 304 48',toggle=True)])
  return controls
 guides=[f'212 {132+row*110} 1052 108'for row in range(4)]
 normal_header=[]
 for j,text in enumerate(['KICK','SNARE','HI-HAT','PERC']):normal_header.append(button(text,'SoundPart',f'{16+j*192} 88 180 42',j,4,kind='dark'))
 normal_header.extend([label('USER KIT','800 88 160 42',20,'fff2dfbd'),value('UserKit','966 88 130 42'),button('CYMBALS','CymbalView','1108 88 156 42',toggle=True)])
 define('SoundHeader',normal_header)
 define('CymbalHeader',[label('CYMBAL SOUNDS','16 88 500 42',25,'fff2dfbd'),button('BACK TO DRUMS','CymbalView','964 88 300 42',toggle=True)])
 def page(cymbal,detail):
  entries=base()+[instance('CymbalHeader'if cymbal else'SoundHeader')]
  for row in range(3 if cymbal else 4):
   y=132+row*110;row_entries=[image(art(1248,108,'card'),f'16 {y} 1248 108'),selector(row,y+2,detail)]
   if detail:row_entries.append(button('Ø','SoundPolarity'+str(row),f'164 {y} 44 44',kind='dark',toggle=True))
   for j,(text,field) in enumerate([('ON','On'),('SOLO','Solo'),('LOCK','Lock'),('REV','Rev')]):row_entries.append(button(text,'Sound'+field+str(row),f'{20+j*46} {y+52} 44 44',kind='dark',toggle=True))
   if detail:
    for j,text in enumerate(['Start','Decay','Low Cut','High Cut']):row_entries.append(knob('SoundQ'+str(row*4+j),text,256 if j==0 else 244+j*250,y,224))
   else:
    row_entries.append(sample(row,y))
    for j,text in enumerate(['Level','Tune','Shift'],1):row_entries.append(knob('SoundQ'+str(row*4+j),text,244+j*250,y,224))
   if not cymbal and row>=2:row_entries=[variant for entry in row_entries for variant in not_hat(entry)]
   entries.extend(row_entries)
  return entries+footer()
 for name,cymbal,detail in [('SoundOverview',False,False),('SoundDetailPage',False,True),('CymbalOverview',True,False),('CymbalDetailPage',True,True)]:define(name,page(cymbal,detail))
 normal=[instance('SoundOverview',invalid=['IndexedEnabling/0/2/'+H('SoundDetails')]),instance('SoundDetailPage',invalid=['IndexedEnabling/1/2/'+H('SoundDetails')])]
 define('NormalSoundView',normal)
 define('PagedCymbalView',[instance('CymbalOverview',invalid=['IndexedEnabling/0/2/'+H('SoundDetails')]),instance('CymbalDetailPage',invalid=['IndexedEnabling/1/2/'+H('SoundDetails')])])
 roots=[instance('NormalSoundView',invalid=['IndexedEnabling/0/2/'+H('CymbalView')]),instance('PagedCymbalView',invalid=['IndexedEnabling/1/2/'+H('CymbalView')])]
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
