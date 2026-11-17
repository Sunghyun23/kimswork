#!/usr/bin/env python3
"""Compile native MPC widgets/Q-Links; artwork stays separate from interactive controls."""
from pathlib import Path
import sys,json,shutil,math,copy
from PIL import Image,ImageDraw
from skin_art import Material
P=Path(__file__).resolve().parents[1];ROOT=P.parents[2];SDK=ROOT/'third_party/mpc-plugin-builder-sdk';sys.path.insert(0,str(SDK/'tools'));import gen_ui as g
O=g.obj
params=g.load_params(str(P/'build/params.json'));meta=json.loads((P/'parameters.json').read_text());keys={p['key']:i for i,p in enumerate(meta)}
for p,m in zip(params,meta):p.name=m['name'];p.kind='choice'if'choices'in m else'float';p.choices=m.get('choices')
table=g.ParamTable(params);pages,style,_,_,kit=g.load_ui(str(P/'ui.json'),table)
out=P/'skins';out.mkdir(exist_ok=True);kitdir,assets=g.kit_dir_for(kit,str(P/'build'),str(out));tui=g.build_tui(pages,params,style,None,127,assets);ql=g.build_qlinks(pages,params)
def get(o,k):return next(v for a,v in o if a==k)
def put(o,k,v):
 for i,(a,_)in enumerate(o):
  if a==k:o[i]=(k,v);return
 o.append((k,v))
pd=get(tui,'pageData');put(pd,'replacementStrings',[O(('key','Parameter '+str(i)),('value',m['choices']))for i,m in enumerate(meta)if 'choices'in m and m['key']!='Scope']);defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions');tabs=get(pd,'tabs')
# Native Image scales; native Button does not. Generate every button at exact runtime size.
# Mechanical compile-time sizing; MPC Image widget cropped the production source on device.
Image.open(P/'assets/background-branded-source-v2.png').convert('RGB').resize((1280,628),Image.Resampling.LANCZOS).save(out/'background.png')
Image.new('RGBA',(1,1),(0,0,0,0)).save(out/'hit.png')
dice=Image.open(P/'assets/dice-icon-source-v1.png').convert('RGBA').crop((160,130,1360,870)).resize((48,32),Image.Resampling.LANCZOS);dice.save(out/'dice-icon.png')
def H(k):return 'Parameter '+str(keys[k])
def label(text,b,size=23,color='ff38261c',key=None):
 data=g.label_data('Value'if key else'Text',color,size,'horizontallyCentred verticallyCentred',None if key else text);font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold');return g.entry(text or key,'Label',data,b,'No','Show','Always',remap=[('Data',H(key)if key else'Parameter 0')])
def image(file,b):return g.entry(file,'Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',file)),b,'No','Show','Always')
def define(name,children,actions=None,bg='00000000',modal=False):
 comp=g.component_definition(copy.deepcopy(actions or[]),copy.deepcopy(children),bg,ignore_mouse=not actions)
 if modal:
  put(comp,'version',4);comp.insert(next(i for i,(k,_) in enumerate(comp)if k=='componentsData'),('hideQLinkBounds',True))
 defs.append(O(('key',name),('value',comp)))
def instance(name,b='0 0 1280 628',invalid=None):return g.entry(name,name,O(('version',1),('handleName','Data')),b,'Children','Hide'if invalid else'Show','Always',remap=[('Data','Parameter 0')],invalidating=invalid)
material=Material(P,out);material.knobs()
# Small, native-sized padlock art; the whole 44px square is the touch target.
for locked in (False,True):
 im=Image.open(out/material.plate(44,44,'orange' if locked else 'cream')).convert('RGBA');draw=ImageDraw.Draw(im)
 colour='#39271d';draw.rounded_rectangle((12,20,32,35),radius=3,fill=colour)
 if locked:draw.arc((15,9,29,29),180,360,fill=colour,width=3);draw.line((15,18,15,23),fill=colour,width=3);draw.line((29,18,29,23),fill=colour,width=3)
 else:draw.arc((19,6,33,25),180,360,fill=colour,width=3);draw.line((19,15,19,23),fill=colour,width=3)
 draw.ellipse((21,25,23,27),fill='#f4dfb8');draw.line((22,27,22,30),fill='#f4dfb8',width=2)
 im.save(out/('dice-locked.png' if locked else 'dice-unlocked.png'))
def art(w,h,kind):return material.plate(w,h,kind)
counter=0
def button(text,key,b,index=1,count=1,kind='cream',toggle=False):
 global counter
 counter+=1;x,y,w,h=map(int,b.split());a=art(w,h,kind);lit=art(w,h,'olive'if kind in('orange','red')else'orange');parts=[g.entry('button art','Button',g.button_data(lit,a,index,count),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))]),label(text,f'0 0 {w} {h}',min(25,max(12,int((w-8)/max(1,len(text))/.62))),'fff9e9c8'if kind in('dark','red','orange','olive')else'ff38261c')]
 # Captionless hit targets must not emit an empty fixed-Text widget.
 if not text:parts=parts[:1]
 if key.startswith('DiceLock'):put(get(parts[0],'componentData'),'data',g.button_data('dice-locked.png','dice-unlocked.png',1,1))
 if key=='KitLock':
  put(get(parts[1],'bounds'),'version',2)
  put(get(parts[1],'bounds'),'additionalInvalidatingHandles',['IndexedEnabling/0/2/'+H('KitLock')]);put(get(parts[1],'bounds'),'showWhenDataModelInvalid','Hide')
  parts.append(conditional(label('KIT FIXED',f'0 0 {w} {h}',18,'fff9e9c8'),'IndexedEnabling/1/2/'+H('KitLock')))
 if key=='Roll':
  put(get(parts[1],'bounds'),'bounds',f'52 0 {w-68} {h}');parts.append(image('dice-icon.png',f'18 {(h-32)//2} 48 32'))
 if toggle:
  if key in ('Preview','Follow','KitLock','PatternLock','Separate','SoundDetails','SoundInvert','CymbalView','SampleBrowserOpen','FillGridPage','PoolOpen')or key.startswith(('SoundOn','SoundSolo','SoundLock','DiceLock')):
   parts.append(g.entry('caption toggle hit','Button',g.button_data('hit.png','hit.png',1,1),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))]));define('Btn'+str(counter),parts);return instance('Btn'+str(counter),b)
  # Independent UI-local press gate: the continuous surface owns release.
  # The engine command still toggles exactly once on Mouse Down (never on release).
  press='Local/2/gdPress'+str(counter)
  put(get(parts[0],'handle remapping'),'map',[O(('key','Data'),('value',press))])
  put(get(get(parts[0],'componentData'),'data'),'gestureBehaviour','Hold')
  surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+press))
  parts.append(g.entry('command press surface','Slider',surface,f'0 0 {w} {h}','No','Show','Always'))
  define('Btn'+str(counter),parts,[g.action('Mouse Down','Toggle Switch',H(key),''),g.action('Mouse Down','Toggle Switch',press,'')]);return instance('Btn'+str(counter),b)
 parts.append(g.entry('caption hit','Button',g.button_data('hit.png','hit.png',index,count),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))]));define('Btn'+str(counter),parts);return instance('Btn'+str(counter),b)
def panel(title,b):
 x,y,w,h=map(int,b.split());h=138;name='panel-'+str(len(defs));badge,_,caption=title.partition(' · ')
 children=[image(art(w,h,'card'),f'0 0 {w} {h}'),image(art(44,22,'orange'),'10 3 44 22'),label(badge,'10 1 44 22',24,'fff4dfbb'),label(caption,f'64 0 {w-80} 23',22)]
 style=get(get(get(children[-1],'componentData'),'data'),'textStyle');put(style,'justification','left verticallyCentred')
 define(name,children);return instance(name,f'{x} {y} {w} {h}')
def group_guide(x,y):
 # Header/badge ends at y+25; native Q badge needs a left gutter beside labels.
 return f'{x+4} {y+26} 606 110'
def knob(key,name,x,y,w=138,physical=None,dice_lock=None):
 lcdw=min(116,w-22);lcdx=(w-lcdw)//2
 bind=H(key);children=[label({'MixKick':'Kick','MixSnare':'Snare','MixHat':'Hat','MixPerc':'Perc','XQKit':'Drum Kit','XQKick':'Kick','XQSnare':'Snare','XQHat':'Hat','XQPerc':'Perc Set'}.get(key,name.removesuffix(' Picker') if key.startswith('RulePick') else name.removesuffix(' Rule')if key.startswith('Rule')else name),f'0 0 {w} 23',22),g.entry('knob','Knob',g.knob_filmstrip_data('knob-v2.png',127,'Vertical'),f'{(w-64)//2} 20 64 64','Yes','Show','Always',remap=[('Data',bind)]),image(art(lcdw,24,'olive'),f'{lcdx} 81 {lcdw} 24'),label('',f'{lcdx} 81 {lcdw} 24',18 if key.startswith('RulePick')or key in ('HatRhythm','OpenHatRule','RideRule','CrashRule')else 20 if key in ('XQKit','XQKick','XQSnare','XQHat','XQPerc')or key.startswith('Sound')else 24,'fff7e9c7',key=key)]
 if dice_lock is not None:
  put(get(children[1],'bounds'),'bounds',f'{(w-64)//2-16} 20 64 64')
  children.append(button('','DiceLock'+str(dice_lock),f'{w-44} 26 44 44',toggle=True))
 if key.startswith('Sample')and key[6:].isdigit():
  library='XSample'+key[6:];conditional(children[3],'IndexedEnabling/0/1048576/'+H(library));children.append(conditional(label('LIBRARY',f'{lcdx} 81 {lcdw} 24',22,'fff7e9c7'),'RangedEnabling/1/1048575/1048576/'+H(library)))
 if key in ['XQKick','XQSnare','XQHat','XQPerc']:
  group=['XQKick','XQSnare','XQHat','XQPerc'].index(key);popup='SoundLaunch'+key
  hit=g.entry('role list touch','Button',g.button_data('hit.png','hit.png',group,4),'0 0 '+str(lcdw)+' 24','No','Show','Always',remap=[('Data',H('SoundBrowserGroup'))])
  define(popup,[hit],[g.action('Mouse Down','Toggle Switch',H('SoundOpen'),'')]);children.append(instance(popup,f'{lcdx} 81 {lcdw} 24'))
 if key=='XQKit':children.append(g.entry('kit list touch','Button',g.button_data('hit.png','hit.png',1,1),f'{lcdx} 81 {lcdw} 24','No','Show','Always',remap=[('Data',H('KitOpen'))]))
 if 'choices'in meta[keys[key]]:children.append(g.entry('value list touch','Button',g.button_data('hit.png','hit.png',1,2),f'{lcdx} 81 {lcdw} 24','No','Show','Always',remap=[('Data','Local/2/gdChoice'+str(keys[key]))]))
 component='Knob'+key+'-'+str(x)+'-'+str(y)
 if any(get(d,'key')==component for d in defs):component+='-'+str(len(defs))
 define(component,children,[g.action('Mouse Down','Q-Link',H('Q'+str(physical+1))if physical is not None else bind,'')]);return instance(component,f'{x} {y} {w} 108')
def conditional(entry,handle):
 b=get(entry,'bounds');put(b,'version',2);put(b,'showWhenDataModelInvalid','Hide');put(b,'additionalInvalidatingHandles',[handle]);return entry
def base():return[image('background.png','0 0 1280 628'),label('','22 64 750 20',18,'fff2d8a8',key='BankInfo')]
def pads(y):
 return[button(n,'Pad'+str(i),f'{16+i*157} {y} 149 58',kind='dark',toggle=True)for i,n in enumerate(['KICK','SNARE','CL HAT','OP HAT','CLAP','TOM','CYMBAL','PERC'])]
main=base()+[button('KIT','Kit','16 92 90 42',index=0,count=3),button('<','KitPrev','110 92 44 42')if'KitPrev'in keys else label('','110 92 1 1')]
# Value sliders use standard native knob overlay; choice lists receive custom sidebars below.
def value(key,b,title=None):
 x,y,w,h=map(int,b.split());name='Value'+key+str(x)+str(y);children=[image(art(w,h,'olive'),f'0 0 {w} {h}'),label('',f'4 0 {w-8} {h}',18 if key=='Scope' and w<288 else 25,'fff8ebc9',key)]
 if key=='Pattern':
  conditional(children[1],'IndexedEnabling/0/131072/'+H('XBeat'));children.append(conditional(label('',f'4 0 {w-8} {h}',20,'fff8ebc9','LibraryBeat'),'RangedEnabling/1/131071/131072/'+H('XBeat')))
 if key=='Scope' and w>=288:
  put(get(children[1],'bounds'),'bounds',f'100 0 {w-108} {h}');children.append(label('SCOPE',f'8 0 90 {h}',20,'fff8ebc9'))
 if key=='XKit':
  define(name,children,[g.action('Mouse Down','Toggle Switch',H('KitOpen'),'')])
 elif 'choices'in meta[keys[key]]:
  opened='Local/2/gdChoice'+str(keys[key]);define(name,children,[g.action('Mouse Down','Toggle Switch',opened,'')])
 else:
  children=children[:1]
  children.append(g.entry('value edit','Knob',O(('version',4),('knobType','ValueSlider'),('textStyle',get(g.label_data('Value','fff8ebc9',25),'textStyle')),('dragOrientation','Vertical'),('handleName','Data')),f'0 0 {w} {h}','Yes','Show','Always',remap=[('Data',H(key))]));define(name,children)
 return instance(name,b)
# Above the instruments: read-only context. Every MAIN action lives below the four panels.
main=base()+[image(art(1248,40,'dark'),'16 90 1248 40'),label('KIT','24 92 64 36',20,'fff7e9c7'),label('','92 92 300 36',23,'fff7e9c7','XKit'),label('PATTERN','400 92 112 36',20,'fff7e9c7'),label('','514 92 280 36',23,'fff7e9c7','Pattern'),label('','808 92 120 36',21,'fff7e9c7','Position'),label('','934 92 130 36',21,'fff7e9c7','Status'),label('','1070 92 180 36',22,'fff7e9c7','PreviewBpm')]

groups=[('A · TIMING',['Swing','SnareLate','HatShift','RndNudge'],16,136),('B · RULES',['RulePickKick','RulePickSnare','RulePickHat','RulePickPerc'],650,136),('C · DRUM MIX',['MixKick','MixSnare','MixHat','MixPerc'],16,278),('D · HATS / CYMBALS',['HatRhythm','OpenHatRule','RideRule','CrashRule'],650,278)]
for group,(title,groupkeys,x,y)in enumerate(groups):
 main.append(panel(title,f'{x} {y} 614 112'))
 for j,key in enumerate(groupkeys):main.append(knob(key,meta[keys[key]]['name'],x+44+j*140,y+28,138,group*4+j,dice_lock=group*4+j))
main.append(conditional(label('HATS REST ON RIDE','1036 279 216 22',16,'ff6a4930'),'RangedEnabling/1/4/5/'+H('RideRule')))
# Total bounds must fit628; knob panes finish425. Action row starts432.
# Three spacious action lanes; physical pads own direct drum performance.
define('SelectPoolButton',[g.entry('selected pool','Button',g.button_data(art(192,40,'orange'),art(192,40,'cream'),2,3),'0 0 192 40','No','Show','Always',remap=[('Data',H('DicePool'))]),label('SELECT KITS','0 0 192 40',22)], [g.action('Mouse Down','Toggle Switch',H('PoolOpen'),'')])
main +=[label('SOUNDS','16 420 88 60',17,'fff7e9c7'),button('<','KitPrev','108 420 48 60',toggle=True),value('XKit','164 420 252 60'),button('>','KitNext','424 420 48 60',toggle=True),button('LOCK KIT','KitLock','484 420 130 60',toggle=True),button('<','PackPrev','626 420 48 60',toggle=True),label('','682 420 290 60',21,'fff7e9c7','PackInfo'),button('>','PackNext','980 420 48 60',toggle=True),button('SAVE KIT','SaveKit','1040 420 224 60',kind='dark',toggle=True),
label('DICE','16 488 88 60',17,'fff7e9c7'),button('ROLL DICE','Roll','108 488 264 60',kind='orange',toggle=True),label('USE SOUNDS FROM','384 484 560 22',17,'fff7e9c7'),button('THIS PACK','DicePool','384 508 176 40',0,3),button('ALL PACKS','DicePool','568 508 176 40',1,3),instance('SelectPoolButton','752 508 192 40'),button('UNDO','Undo','956 488 112 60',toggle=True),value('Scope','1080 488 184 60'),
label('PLAY','16 556 88 60',17,'fff7e9c7'),button('MPC SYNC','Follow','108 556 156 60',toggle=True),button('PLAY LOOP','Preview','272 556 168 60',toggle=True),value('FillSlot','448 556 72 60'),button('PLAY FILL','Trigger','528 556 156 60',kind='orange',toggle=True),button('EDIT FILL','Editor','692 556 176 60',toggle=True),label('','880 556 384 60',20,'fff7e9c7','ActionInfo')]


# Fill editor is an engine-backed context panel. Fixed SCREEN relay params map to its real parameters.
editor=base()+[label('FILL EDIT / REC · Q-LINK SCREEN','16 92 1000 42',26,'fff5e5c3')]
for i in range(4):editor.append(button(chr(65+i),'FillSlot',f'{16+i*132} 142 120 42',i,4))
for i in range(4):editor.append(button('BAR '+str(i+1),'GridBar',f'{560+i*108} 142 100 40',i,4))
editor +=[label('','1008 142 256 42',24,'fff5e5c3',key='Status')]
for title,ks,x in [('A · RECORD SETUP',['FillSlot','FillLength','RecMode','CountIn'],16),('B · SELECTED NOTE',['NoteNudge','NoteVelocity','NoteGate','NoteChance'],650)]:
 editor.append(panel(title,f'{x} 186 614 128'))
 for j,key in enumerate(ks):editor.append(knob(key,meta[keys[key]]['name'],x+44+j*140,214,138,([ 'FillSlot','FillLength','RecMode','CountIn','NoteNudge','NoteVelocity','NoteGate','NoteChance'].index(key))))
# Native touch grid: each visible step selects or creates a real recorded event.
editor +=[button('PERC / DRUMS','FillGridPage','16 326 260 28',toggle=True),value('NoteSelect','290 326 400 30'),label('','710 326 180 30',23,'fff5e5c3',key='RecordedCount')]
for row in range(4):
 y=358+row*40
 for page,captions in enumerate([['SNARE','TOM','PERC','CLAP'],['KICK','CLOSED HAT','OPEN HAT','CYMBAL']]):editor.append(conditional(label(captions[row],f'16 {y} 180 40',20,'fff5e5c3'),'IndexedEnabling/'+str(page)+'/2/'+H('FillGridPage')))
 for step in range(16):
  x=200+step*66;cell=row*16+step;children=[g.entry('cell state','Button',g.button_data(art(60,38,'orange'),art(60,38,'dark'),1,1),'0 0 60 38','No','Show','Always',remap=[('Data',H('Grid'+str(cell)))]),g.entry('select note','Button',g.button_data('hit.png','hit.png',cell+1,65),'0 0 60 38','No','Show','Always',remap=[('Data',H('GridSelect'))])];define('Cell'+str(cell),children,[g.action('Mouse Down','Toggle Switch',H('GridTap'),'')]);editor.append(instance('Cell'+str(cell),f'{x} {y} 60 38'))
editor +=[button('MAIN','Editor','16 520 176 42',index=0,count=2,toggle=True),button('REC FILL','Record','202 520 216 42',kind='red',toggle=True),button('AUDITION','Trigger','428 520 180 42',kind='orange',toggle=True),button('CANCEL','Cancel','618 520 130 42',toggle=True),button('DELETE','NoteDelete','758 520 130 42',toggle=True),button('UNDO','Undo','898 520 110 42',toggle=True),label('','1018 520 246 42',25,'fff5e5c3',key='Status')]+[image(art(1248,48,'dark'),'16 570 1248 48'),label('','28 570 1224 48',23,'fff7e9c7','ActionInfo')]
define('GDMain',main);define('GDFill',editor,modal=True)
mainRoot=[instance('GDMain',invalid=['IndexedEnabling/0/2/'+H('Editor')]),instance('GDFill',invalid=['IndexedEnabling/1/2/'+H('Editor')])]
put(tabs[0],'componentDefinition',g.component_definition([],mainRoot,'ffeed9b3'));put(tabs[0],'qlinkBoundsData',[group_guide(x,y)for _,_,x,y in groups])
# Retain native first-key FILL fallback, with direct fixed mapping; main editor relay works without invented tab actions.
put(tabs[1],'fnKeyIndex',0);put(tabs[1],'fnKeySubIndex',1);put(tabs[1],'componentDefinition',g.component_definition([],editor,'ffeed9b3'));put(tabs[1],'qlinkBoundsData',[group_guide(16,186),group_guide(650,186),'0 0 0 0','0 0 0 0'])
for ti,tab in enumerate(tabs[2:],1):put(tab,'fnKeyIndex',ti)
# All detail pages use the same real native controls and approved material palette.
for ti,tab in enumerate(tabs[2:],2):
 page=pages[ti];children=base()+[label(page.name+' · SCREEN Q-LINK','16 92 900 36',27,'fff5e5c3')]
 if page.name=='GROOVE':children.append(label('1-BAR LOOP / LIVE EDIT / SEPARATE FILL A-D','16 126 900 18',18,'fff5e5c3'))
 bounds=[]
 for group in range(4):
  controls=[c for c in page.controls[group*4:group*4+4] if c.param is not None]
  if not controls:bounds.append('0 0 0 0');continue
  x=16 if group%2==0 else 650;y=146 if group<2 else 302;section=['KIT / DICE','KICK','SNARE','CLOSED HAT'][group]if page.name=='DRUMS'else'FILL / VARIATION'if page.name=='GROOVE'and group==3 else page.name;children.append(panel(chr(65+group)+' · '+section,f'{x} {y} 614 25'));bounds.append(group_guide(x,y))
  for j,ctl in enumerate(controls):
   key=meta[ctl.param.index]['key'];children.append(knob(key,meta[ctl.param.index]['name'],x+44+j*140,y+28,138))
 if page.name=='GROOVE':
  children +=[label('HAT RHYTHM · LIVE','16 526 664 28',21,'fff5e5c3'),value('HatRhythm','16 566 664 50')]
 if page.name=='KITS':children +=[value('KitDiceMode','16 446 600 50'),button('<','PackPrev','650 526 54 54',toggle=True),label('','710 526 494 54',21,'fff7e8c7','PackInfo'),button('>','PackNext','1210 526 54 54',toggle=True),button('SAVE KIT','SaveKit','16 526 300 54',kind='orange',toggle=True),button('LOAD KIT','LoadKit','334 526 300 54',toggle=True)]
 if page.name=='ROUTING':children.append(button('SEPARATE OUT','Separate','1000 574 264 42',toggle=True))
 put(tab,'componentDefinition',g.component_definition([],children,'ffeed9b3'));put(tab,'qlinkBoundsData',bounds)

from sound_skin import build_sound_skin
build_sound_skin(g,O,tabs,keys,get,put,H,label,image,define,instance,button,art,knob,value,conditional,base)
# Custom sidebars for every choice selector, placed above current content and hiding background guides.
for tab in tabs:
 children=get(get(tab,'componentDefinition'),'componentsData')
 allowed={'XKit','Kit','Pattern','Scope','FillSlot','FillEvery','FillLength','RecMode','CountIn','GridBar','RulePickKick','RulePickSnare','RulePickHat','RulePickPerc','HatRhythm','OpenHatRule','RideRule','CrashRule'} if get(tab,'tabName')=='MAIN' else {meta[c.param.index]['key']for page in pages if page.name==get(tab,'tabName')for c in page.controls if c.param is not None}
 if get(tab,'tabName')=='GROOVE':allowed.add('HatRhythm')
 if get(tab,'tabName')=='SOUND':allowed.add('UserKit')
 if get(tab,'tabName')=='KITS':allowed.add('KitDiceMode')
 for m in meta:
  if 'choices'not in m or m['key']not in allowed:continue
  key=m['key'];opened='Local/2/gdChoice'+str(keys[key]);name='GDChoice'+key
  if not any(get(d,'key')==name for d in defs):
   rows=[image(art(426,628,'dark'),'0 0 426 628'),label('EXTRA CHANGES' if key=='Scope' else m['name'].upper(),'12 8 340 44',28,'fff3dfba')]
   rows.append(g.entry('outside dismiss','Button',g.button_data('hit.png','hit.png',0,2),'426 0 854 628','No','Show','Always',remap=[('Data',opened)]))
   for j,text in enumerate(m['choices']):
    if key not in ('XKit','HatRhythm')and j>=8:break
    rowName=name+'Row'+str(j)
    if key=='Scope':text=['No extras','New Pattern','Groove Variation','Recorded Fill','Pattern + Fill + FX','FX'][j]
    parts=[button(text,key,'0 0 414 56',j,len(m['choices']),kind='dark')]
    actions=[g.action('Mouse Down','Toggle Switch',opened,'')]
    if key=='XKit':actions.append(g.action('Mouse Down','Toggle Switch',H('FactoryReload'),''))
    define(rowName,parts,actions);entry=instance(rowName,f'6 {62+(j%8)*60} 414 56')
    if key in ('XKit','HatRhythm'):conditional(entry,'IndexedEnabling/'+str(j//8)+'/'+str((len(m['choices'])+7)//8)+'/'+H('KitBrowserPage'if key=='XKit'else'HatRhythmPage'))
    rows.append(entry)
   if key=='XKit':rows +=[label('','6 538 414 24',20,'fff7e8c7','KitBrowserPage'),button('< PREV','KitPagePrev','6 568 202 48',kind='cream',toggle=True),button('NEXT >','KitPageNext','218 568 202 48',kind='cream',toggle=True)]
   if key=='HatRhythm':rows +=[button('PAGE 1 / 2','HatRhythmPage','6 568 202 48',0,2),button('PAGE 2 / 2','HatRhythmPage','218 568 202 48',1,2)]
   if key.startswith('Rule'):
    rows.append(label('Live changes / LOCK holds rhythm','12 548 402 30',19,'fff3dfba'))
   # X uses a genuine native toggle on the local handle.
   hit=g.entry('Close sidebar','Button',g.button_data('hit.png','hit.png',0,2),'360 0 66 56','No','Show','Always',remap=[('Data',opened)]);rows +=[label('X','360 0 66 56',28,'fff8e4bd'),hit];define(name,rows,modal=True)
  children.append(instance(name,invalid=['IndexedEnabling/1/2/'+opened]))
# Library contents are immutable runtime catalogue entries; page captions update via formatValue.
def library_row(index,b):
 x,y,w,h=map(int,b.split());name='LibraryRowButton'+str(index)
 children=[conditional(image(art(w,h,'cream'),f'0 0 {w} {h}'),'IndexedEnabling/0/2/'+H('LibraryPick'+str(index))),conditional(image(art(w,h,'orange'),f'0 0 {w} {h}'),'IndexedEnabling/1/2/'+H('LibraryPick'+str(index))),label('',f'8 0 {w-16} {h}',20,'ff38261c','LibraryRow'+str(index))]
 define(name,children,[g.action('Mouse Down','Toggle Switch',H('LibraryPick'+str(index)),'')]);return instance(name,b)
lib=[image(art(426,628,'dark'),'0 0 426 628'),button('<','PackPrev','6 2 48 40',toggle=True),label('','60 2 246 40',20,'fff8e8c6','PackInfo'),button('>','PackNext','310 2 48 40',toggle=True),button('X','LibraryOpen','366 2 54 40',toggle=True)]
for i,title in enumerate(['KITS','CHOPS','LOOPS','BEATS','GROOVE']):lib.append(button(title,'LibraryKind',f'{6+i*83} 44 80 36',i,5,kind='olive'))
for i,title in enumerate(['KICK','SNARE','CL HAT','OP HAT','CLAP','TOM','CYMBAL','PERC']):lib.append(button(title,'LibraryRole',f'{6+(i%4)*104} {86+(i//4)*31} 100 29',i,8,kind='dark'))
lib.append(label('','6 150 304 24',20,'fff7e8c7','LibraryInfo'));lib.append(button('FOLDERS','LibraryBack','314 148 106 28',kind='cream',toggle=True))
for i in range(8):lib.append(library_row(i,f'6 {178+i*43} 414 41'))
lib +=[button('< PREV','LibraryPrev','6 526 202 40',toggle=True),button('NEXT >','LibraryNext','216 526 204 40',toggle=True),button('CORE SOUNDS','LibraryLegacy','6 574 132 38',toggle=True),button('BEAT OFF','LibraryBeatOff','144 574 132 38',toggle=True),button('GRV OFF','GrooveOff','282 574 138 38',toggle=True)]
# Outside dismissal is above the entire previous tab; captions retain raw parent command hit surfaces.
lib.append(g.entry('outside close','Button',g.button_data('hit.png','hit.png',0,2),'426 0 854 628','No','Show','Always',remap=[('Data',H('LibraryOpen'))]))
define('LibraryBrowser',lib,modal=True)
for tab in tabs:
 children=get(get(tab,'componentDefinition'),'componentsData')
 if get(tab,'tabName')in ('DRUMS','KITS','GROOVE'):
  children.append(button('LIBRARY','LibraryOpen','1000 578 264 40',kind='orange',toggle=True));children.append(label('','650 446 614 32',21,'fff5e5c3','LibraryCurrentSound'));children.append(label('','650 484 614 32',19,'fff5e5c3','LibraryPlaybackInfo'))
 children.append(instance('LibraryBrowser',invalid=['IndexedEnabling/1/2/'+H('LibraryOpen')]))
# Classified role browser: eight rows, stable values; never creates thousands of native choice widgets.
role=[image(art(426,628,'dark'),'0 0 426 628'),button('<','PackPrev','6 2 48 40',toggle=True),label('','60 2 246 40',20,'fff8e8c6','PackInfo'),button('>','PackNext','310 2 48 40',toggle=True),button('X','SoundOpen','366 2 54 40',toggle=True)]
for group,title in enumerate(['KICK','SNARE','HAT PAIRS','PERC SET']):role.append(button(title,'SoundBrowserGroup',f'{6+group*104} 48 100 40',group,4,kind='olive'))
role.append(label('','6 98 414 36',22,'fff7e8c7','SoundInfo'))
for row in range(8):
 name='RoleSoundRow'+str(row);children=[image(art(414,48,'cream'),'0 0 414 48'),label('','8 0 398 48',21,key='SoundRow'+str(row))]
 define(name,children,[g.action('Mouse Down','Toggle Switch',H('SoundPick'+str(row)),'')]);role.append(instance(name,f'6 {142+row*50} 414 48'))
role +=[button('< PREV','SoundPrev','6 560 202 48',toggle=True),button('NEXT >','SoundNext','218 560 202 48',toggle=True),g.entry('outside role close','Button',g.button_data('hit.png','hit.png',0,2),'426 0 854 628','No','Show','Always',remap=[('Data',H('SoundOpen'))])]
define('RoleSoundBrowser',role,modal=True)
for tab in tabs:get(get(tab,'componentDefinition'),'componentsData').append(instance('RoleSoundBrowser',invalid=['IndexedEnabling/1/2/'+H('SoundOpen')]))
kitrows=[image(art(426,628,'dark'),'0 0 426 628'),button('<','PackPrev','6 2 48 40',toggle=True),label('','60 2 246 40',20,'fff8e8c6','PackInfo'),button('>','PackNext','310 2 48 40',toggle=True),button('X','KitOpen','366 2 54 40',toggle=True),label('DRUM KITS','6 48 414 38',25,'fff8e8c6'),label('','6 94 414 34',21,'fff7e8c7','XKitPage')]
for row in range(8):
 name='DynamicKitRow'+str(row);parts=[g.entry('selected kit art','Button',g.button_data(art(414,48,'orange'),art(414,48,'cream'),1,1),'0 0 414 48','No','Show','Always',remap=[('Data',H('KitRow'+str(row)))]),label('','8 0 398 48',21,key='KitRow'+str(row))]
 define(name,parts,[g.action('Mouse Down','Toggle Switch',H('KitPick'+str(row)),'')]);kitrows.append(instance(name,f'6 {138+row*50} 414 48'))
kitrows +=[button('< PREV','KitPagePrev','6 560 202 48',toggle=True),button('NEXT >','KitPageNext','218 560 202 48',toggle=True),g.entry('outside kit close','Button',g.button_data('hit.png','hit.png',0,2),'426 0 854 628','No','Show','Always',remap=[('Data',H('KitOpen'))])]
define('ExpansionKitBrowser',kitrows,modal=True)
for tab in tabs:get(get(tab,'componentDefinition'),'componentsData').append(instance('ExpansionKitBrowser',invalid=['IndexedEnabling/1/2/'+H('KitOpen')]))
pool=[image(art(426,628,'dark'),'0 0 426 628'),button('<','PackPrev','6 2 48 40',toggle=True),label('','60 2 246 40',20,'fff8e8c6','PackInfo'),button('>','PackNext','310 2 48 40',toggle=True),button('X','PoolOpen','366 2 54 40',toggle=True),label('','6 48 414 40',23,'fff8e8c6','PoolInfo')]
for row in range(8):
 y=96+row*49;pool.extend([button('','PoolPick'+str(row),f'6 {y} 414 46',kind='olive',toggle=True),label('',f'12 {y} 402 46',21,'fff8e8c6','PoolRow'+str(row))])
pool.extend([button('< PREV','KitPagePrev','6 496 202 46',toggle=True),button('NEXT >','KitPageNext','218 496 202 46',toggle=True),button('CLEAR','PoolClear','6 552 202 60',toggle=True),button('DONE','PoolOpen','218 552 202 60',toggle=True),g.entry('outside pool close','Button',g.button_data('hit.png','hit.png',0,2),'426 0 854 628','No','Show','Always',remap=[('Data',H('PoolOpen'))])])
define('SelectedKitBrowser',pool,modal=True)
for tab in tabs:get(get(tab,'componentDefinition'),'componentsData').append(instance('SelectedKitBrowser',invalid=['IndexedEnabling/1/2/'+H('PoolOpen')]))
# SCREEN MAIN uses fixed16 relay parameters; native FILL uses actual note/record parameters.
entries=get(get(ql,'Screen Mode Q-Links'),'map')
for ti,e in enumerate(entries):
 if ti==0:
  assignment={13+group-4*knob:keys['Q'+str(group*4+knob+1)]for group in range(4)for knob in range(4)};put(e,'Q-Links',O(*[('Q-Link '+str(n),assignment[n])for n in range(1,17)]))
 elif ti==1:put(e,'Tab',1);put(e,'SubTab',2)
 else:
  put(e,'Tab',ti)
  assignment={}
  for group in range(4):
   for knobindex in range(4):
    ci=group*4+knobindex
    if ci<len(pages[ti].controls)and pages[ti].controls[ci].param is not None:assignment[13+group-4*knobindex]=pages[ti].controls[ci].param.index
  put(e,'Q-Links',O(*[('Q-Link '+str(n),assignment.get(n,-1))for n in range(1,17)]))
# Program/Track always retains actualMAIN controls, independent of editor relay context.
assignment={13+group-4*knob:keys[groups[group][1][knob]]for group in range(4)for knob in range(4)}
put(ql,'Program Mode Q-Links',O(*[('Q-Link '+str(n),assignment[n])for n in range(1,17)]))
# EDIT/REC is the single context panel onMAIN; no duplicate nativeFILL subtab.
tabs.pop(1);entries.pop(1)
(out/'TUI.json').write_text(g.dump_json(tui)+'\n');(out/'Q-Links.json').write_text(g.dump_json(ql)+'\n')
print('Groove Dice native skin compiled: MAIN + fill editor,6function keys,SCREEN relay + independent Program maps')
