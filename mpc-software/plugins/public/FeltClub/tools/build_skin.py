#!/usr/bin/env python3
"""Reference-derived MPC skin. Native widgets remain functional, no mock host bar."""
from pathlib import Path
import copy,json,sys,importlib.util,math
from PIL import Image,ImageDraw,ImageFont
P=Path(__file__).resolve().parents[1];SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk';sys.path.insert(0,str(SDK/'tools'));import gen_ui as g
O=g.obj
out=P/'skins';out.mkdir(exist_ok=True)
args=sys.argv;sys.argv=['artwork',str(P/'design/approved-main.png'),str(out)];spec=importlib.util.spec_from_file_location('felt_art',P/'tools/artwork.py');art=importlib.util.module_from_spec(spec);spec.loader.exec_module(art);sys.argv=args
meta=json.loads((P/'parameters.json').read_text());keys={p['key']:i for i,p in enumerate(meta)};programs=json.loads((P/'programs.json').read_text());params=g.load_params(str(P/'build/params.json'));assert len(params)==len(meta)
for p,m in zip(params,meta):p.name=m['name'];p.kind='choice'if'choices'in m else'float';p.choices=m.get('choices')
table=g.ParamTable(params);pages,style,_,_,kit=g.load_ui(str(P/'ui.json'),table);kitdir,assets=g.kit_dir_for(kit,str(P/'build'),str(out));tui=g.build_tui(pages,params,style,None,97,assets);ql=g.build_qlinks(pages,params)
def get(o,k):return next(v for a,v in o if a==k)
def put(o,k,v):
 for i,(a,_)in enumerate(o):
  if a==k:o[i]=(k,v);return
 o.append((k,v))
pd=get(tui,'pageData');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions');template=copy.deepcopy(get(pd,'tabs')[0]);put(pd,'replacementStrings',[O(('key','Parameter '+str(i)),('value',m['choices']))for i,m in enumerate(meta)if'choices'in m])
Image.new('RGBA',(1,1)).save(out/'hit.png')
# Native48px octave dials retain exactly the same approved artwork.
strip=Image.open(out/'knob.png');small=Image.new('RGBA',(48,48*97))
for n in range(97):small.paste(strip.crop((0,n*72,72,(n+1)*72)).resize((48,48),Image.Resampling.LANCZOS),(0,n*48))
small.save(out/'knob-small.png')
# Dice is a small ivory engraved die, matching the approved footer.
die=Image.new('RGBA',(38,38));dr=ImageDraw.Draw(die);dr.rounded_rectangle((5,3,33,33),5,fill='#eee4c6',outline='#183b30',width=2)
for x,y in [(12,10),(26,10),(19,18),(12,26),(26,26)]:dr.ellipse((x-2,y-2,x+2,y+2),fill='#214236')
die.save(out/'dice.png')
for size,name in [((426,628),'sidebar.png'),((1280,628),'detail-background.png')]:
 im=art.tile(art.green,size);im=im.convert('RGBA');ImageDraw.Draw(im).rectangle((0,0,size[0]-1,size[1]-1),outline='#b99959',width=2);im.save(out/name)
chevron=Image.new('RGBA',(12,8));ImageDraw.Draw(chevron).polygon([(0,0),(11,0),(6,7)],fill='#201b0f');chevron.save(out/'chevron.png')
serial=0;selectors=set();panels={};plates={};legends={}
def H(k):return 'Parameter '+str(keys[k])
def label(text,b,size=21,color='ff201b0f',key=None):
 # Static legends use rendered serif artwork; live values use supported native
 # Roboto Regular. Never name an unavailable custom font in the TUI.
 if key in ('Character3Label','Character4Label'):
  names=['BELL','REED','STRING','STRING','BITE','BITE','BELL'] if key=='Character3Label' else ['BARK','BARK','BOARD','BOARD','MUTE','MUTE','MUTE']
  name='Legend'+key
  define(name,[conditional(label(t,b,size,color),'IndexedEnabling/'+str(i)+'/7/'+H('Model'))for i,t in enumerate(names)])
  return instance(name)
 if text in art.legends and key is None:
  _,_,w,h=map(int,b.split());name='sized-'+text.lower()+'.png';Image.open(out/art.legends[text]).resize((w,h),Image.Resampling.LANCZOS).save(out/name);return image(name,b)
 if text and key is None:
  _,_,w,h=map(int,b.split());name='legend-'+str(len(legends))+'.png';token=(text,w,h,size,color)
  if token not in legends:
   im=Image.new('RGBA',(w,h));fsize=min(size,19) if text.isupper() else size
   if text in ('BELL','STRING','BOARD','BITE','MUTE','NOISE')and h==20:fsize=15
   font=art.font(fsize,False)
   while font.getlength(text)>w-8 and fsize>12:fsize-=1;font=art.font(fsize,False)
   ImageDraw.Draw(im).text((w/2,h/2),text,font=font,anchor='mm',fill='#'+color[-6:]);im.save(out/name);legends[token]=name
  return image(legends[token],b)
 data=g.label_data('Value'if key else'Text',color,size,'horizontallyCentred verticallyCentred',None if key else text);font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Bold');put(font,'height',float(round(size*1.30)));return g.entry(text or key,'Label',data,b,'No','Show','Always',remap=[('Data',H(key)if key else'Parameter 0')])

def image(file,b):return g.entry(file,'Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',file)),b,'No','Show','Always')
def define(name,children,actions=None,modal=False):
 d=g.component_definition(copy.deepcopy(actions or[]),copy.deepcopy(children),'00000000',ignore_mouse=not actions)
 if modal:
  put(d,'version',4);d.insert(next(i for i,(k,_)in enumerate(d)if k=='componentsData'),('hideQLinkBounds',True))
 defs.append(O(('key',name),('value',d)))
def instance(name,b='0 0 1280 628',condition=None):return g.entry(name,name,O(('version',1),('handleName','Data')),b,'Children','Hide'if condition else'Show','Always',remap=[('Data','Parameter 0')],invalidating=[condition]if condition else[])
def conditional(e,condition):
 b=get(e,'bounds');put(b,'version',2);put(b,'additionalInvalidatingHandles',[condition]);put(b,'showWhenDataModelInvalid','Hide');return e

def plate(w,h,active=False,kind='ivory',text=None,size=21):
 token=(w,h,active,kind,text,size)
 if token in plates:return plates[token]
 name='plate-'+str(len(plates))+'.png';im=art.plate(w,h,active,kind).convert('RGBA')
 if active and kind=='pressed':im=art.plate(w,h,True).convert('RGBA');im=Image.blend(im,Image.new('RGBA',im.size,'#28180cff'),.18)
 if text:
  font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Georgia.ttf',min(size,20))
  size=min(size,20)
  while font.getlength(text)>w-10 and size>13:size-=1;font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Georgia.ttf',size)
  ImageDraw.Draw(im).text((w/2,h/2),text,font=font,anchor='mm',fill='#fff0d6'if active else'#201b0f')
 im.save(out/name);plates[token]=name;return name

def local_hit(handle,b,value=1,count=2):return g.entry('touch','Button',g.button_data('hit.png','hit.png',value,count),b,'No','Show','Always',remap=[('Data',handle)])
def select(text,key,b,value,count=None):
 global serial
 serial+=1;name='Select'+str(serial);x,y,w,h=map(int,b.split());count=count or len(meta[keys[key]].get('choices',[]))or 2
 parts=[g.entry('selection','Button',g.button_data(plate(w,h,True,text=text),plate(w,h,text=text),value,count),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))]),local_hit(H(key),f'0 0 {w} {h}',value,count)];define(name,parts);return instance(name,b)
def action(text,key,b,orange=False):
 global serial
 serial+=1;name='Action'+str(serial);x,y,w,h=map(int,b.split())
 parts=[g.entry('command art','Button',g.button_data(plate(w,h,orange,text=None if orange else text),plate(w,h,orange,text=None if orange else text),1,1),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))])]
 if orange:parts +=[image('dice.png','12 3 38 38'),label(text,f'48 0 {w-52} {h}',21,'fff8edd8')]
 # Direct native Button owns the entire hit target. Do not rely on parent Mouse
 # Down propagation through a release Slider: it drops quick command gestures
 # on this host. Engine command toggles intentionally accept either edge.
 parts.append(local_hit(H(key),f'0 0 {w} {h}',1,1));define(name,parts);return instance(name,b)

def toggle(text,key,b):
 global serial
 serial+=1;name='Toggle'+str(serial);x,y,w,h=map(int,b.split());parts=[g.entry('toggle','Button',g.button_data(plate(w,h,True,text=text,size=18),plate(w,h,text=text,size=18),1,2),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))]),local_hit(H(key),f'0 0 {w} {h}',1,1)];define(name,parts);return instance(name,b)
def lock(key,b):
 global serial
 serial+=1;name='Lock'+str(serial);parts=[g.entry('lock art','Button',g.button_data('lock-1.png','lock-0.png',1,2),'0 0 44 44','No','Show','Always',remap=[('Data',H(key))]),local_hit(H(key),'0 0 44 44',1,1)];define(name,parts);return instance(name,b)
def choice(key,b,title=None):
 global serial
 selectors.add(key);serial+=1;name='Choice'+str(serial);x,y,w,h=map(int,b.split());children=[image(plate(w,h,kind='ivory'),f'0 0 {w} {h}'),label('',f'6 0 {w-28} {h}',21,key=key),image('chevron.png',f'{w-22} {(h-8)//2} 12 8'),local_hit('Local/2/fcChoice'+str(keys[key]),f'0 0 {w} {h}')];
 if key=='Preset':
  children[-1]=local_hit('Local/4/fcPresetCategory',f'0 0 {w} {h}',0,4)
  define(name,children,[g.action('Mouse Down','Toggle Switch','Local/2/fcChoice'+str(keys[key]),'')])
 else:define(name,children)
 return instance(name,b)
def knob(key,caption,x,y,w=155,lockable=True,qkey=None,caption_color='ff201b0f'):
 global serial
 serial+=1;name='Dial'+str(serial);lcdw=w-20;left=(w-72)//2+5
 children=[g.entry('dial','Knob',g.knob_filmstrip_data('knob.png',97,'Vertical'),f'{left} 3 72 72','Yes','Show','Always',remap=[('Data',H(key))]),label(''if caption.startswith('@')else caption,f'10 77 {w-10} 20',21,color=caption_color,key=caption[1:]if caption.startswith('@')else None),image(plate(lcdw,22,kind='sage'),f'15 101 {lcdw} 22'),label('',f'15 101 {lcdw} 22',22,key=qkey or key)]
 if key in ['Model','Shape','DelayDivision','VelocityCurve','InputMode','Key','Scale','StrumDirection','Slots']or key.startswith('Edit'):
  selectors.add(key);children.append(local_hit('Local/2/fcChoice'+str(keys[key]),f'15 101 {lcdw} 22'))
 elif 'choices'in meta[keys[key]]:
  selectors.add(key);children.append(local_hit('Local/2/fcChoice'+str(keys[key]),f'15 101 {lcdw} 22'))
 if lockable and 'Lock'+key in keys:children.append(lock('Lock'+key,'0 0 44 44'))
 define(name,children,[g.action('Mouse Down','Q-Link',H(qkey or key),'')]);return instance(name,f'{x} {y} {w} 123')
def panel(caption,x,y,tag=None):
 if tag:filename='panel-'+tag+'.png'
 else:
  filename='detail-panel-'+str(len(panels))+'.png';im=Image.open(out/'panel-A.png').convert('RGBA');head=art.tile(art.green,(570,20));im.paste(head,(24,4));d=ImageDraw.Draw(im);d.text((32,14),caption,font=art.font(20),anchor='lm',fill='#eee6ce');im.save(out/filename);panels[filename]=caption
 return image(filename,f'{x} {y} 615 153')
def base():return[image('background.png','0 0 1280 628')]
def header():
 ws=[]
 for title,key,index,x,w in [('EP','Family',0,26,100),('PIANO','Family',1,136,110),('PLUCK','Family',2,257,120)]:ws.append(select(title,key,f'{x} 84 {w} 44',index))
 ws.append(choice('Model','392 84 224 44'));return ws
footer=json.loads((P.parents[2]/'shared/ui/action-footer.json').read_text())
def actions():return[action(a['label'],key,f"{a['x']} {footer['y']} {a['width']} {footer['height']}",key=='Roll')for a,key in zip(footer['actions'],['UnlockAll','Roll','Undo'])]+[action('<','PresetPrev','586 578 44 44'),choice('Preset','638 578 304 44'),action('>','PresetNext','950 578 44 44')]
def paging():return[action('<','PagePrev','1022 578 44 44'),label('','1074 578 112 44',19,'fff2edd7','PageInfo'),action('>','PageNext','1194 578 44 44')]
def chord_selector():
 ws=[image('label-chord.png','88 462 186 28'),action('<','ChordPrev','306 456 60 44'),action('>','ChordNext','896 456 60 44')]
 for page in range(4):ws.append(conditional(choice('ChordPreset'+str(page+1),'392 456 475 44'),'IndexedEnabling/'+str(page)+'/4/'+H('Page')))
 ws.append(toggle('EDIT','ChordEditing','1019 456 150 44'));return ws
main=base()+header()
groups=[(['Tone','Body','Character3','Hiss'],['TONE','BODY','@Character3Label','NOISE']),(['Attack','Decay','Sustain','Release'],['ATTACK','DECAY','SUSTAIN','RELEASE']),(['Rate','Depth','Drift','Width'],['RATE','DEPTH','DRIFT','WIDTH']),(['Drive','Chorus','Delay','Room'],['DRIVE','CHORUS','DELAY','ROOM'])]
for bank,((ks,labels),(x,y))in enumerate(zip(groups,[(16,139),(647,139),(16,302),(647,302)])):
 main.append(panel('',x,y,'ABCD'[bank]))
 for col,(key,title)in enumerate(zip(ks,labels)):main.append(knob(key,title,x+8+col*151,y+29,qkey='MainQ'+str(bank*4+col)))
main +=[select('TREM','MotionMode','371 304 74 26',0),select('PAN','MotionMode','453 304 74 26',1),toggle('SYNC','Sync','535 304 80 26')]+chord_selector()
for row in range(4):
 name='ChordPad'+str(row);key='Hold'+str(row+1);x=18+row*314
 children=[g.entry('held pad','Button',g.button_data(plate(304,70,True),plate(304,70),1,2),'0 0 304 70','No','Show','Always',remap=[('Data',H(key))]),label('', '8 12 288 48',29,key='ChordName'+str(row+1))]
 for page in range(4):children.append(conditional(label(str(page*4+row+1),'10 2 24 18',15),'IndexedEnabling/'+str(page)+'/4/'+H('Page')))
 surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+H(key)))
 children.append(g.entry('hold release','Slider',surface,'0 0 304 70','No','Show','Always'));define(name,children,[g.action('Mouse Down','Toggle Switch',H(key),'')]);main.append(instance(name,f'{x} 504 304 70'))
main +=actions()+paging()
# One chord editor, native first-key subtab and direct MAIN EDIT entry. MAIN's
# quick-touch editor masks sound relay writes; native CHORD owns16edit Q-Links.
editor=base()+[label('CHORDS · TOUCH','16 84 300 44',25,'fff2edd7'),choice('InputMode','328 84 220 44'),action('−','SlotsRemove','564 84 52 44'),choice('Slots','624 84 120 44'),action('+','SlotsAdd','752 84 52 44'),toggle('DONE','ChordEditing','1080 84 172 44')]
for row in range(4):
 y=142+row*82;editor.append(image(plate(900,78),'16 '+str(y)+' 900 78'))
 for page in range(4):
  cond='IndexedEnabling/'+str(page)+'/4/'+H('Page');slot=page*4+row+1
  editor +=[conditional(label(str(slot),f'20 {y+2} 84 34',25),cond),conditional(label('',f'22 {y+34} 84 24',14,key='PadNote'+str(slot)),cond)]
 for field,(title,x,w)in enumerate([('ROOT',122,112),('TYPE',246,198),('VOICING',456,204),('OCT',672,116)]):
  editor +=[label(title,f'{x} {y+2} {w} 20',16),choice('Edit'+str(row*4+field),f'{x} {y+25} {w} 46')]
 editor.append(action('LEARN','Learn'+str(row+1),f'800 {y+28} 104 44'))
editor +=[label('KEY','940 142 96 24',17,'fff2edd7'),choice('Key','1040 142 212 44'),choice('Scale','940 198 152 44'),choice('Transpose','1104 198 148 44'),choice('StrumDirection','940 254 312 44'),knob('Strum','STRUM',946,315,146,False,caption_color='fff2edd7'),knob('Glide','GLIDE',1104,315,146,False,caption_color='fff2edd7'),toggle('CHORD GLIDE','ChordGlide','940 444 312 44'),action('SMOOTH TOP','Smooth','940 500 312 44'),label('','16 480 900 40',18,'fff2edd7','LearnInfo')]+actions()+paging()
define('FeltChordEditor',editor,modal=True)
# Full editor overlay is modal so MAIN's sound guide never paints across chord rows.
main.append(instance('FeltChordEditor',condition='IndexedEnabling/1/2/'+H('ChordEditing')))
settingsPages=[('MODEL',[(['Model','Tone','Body','Pick'],['MODEL','TONE','BODY','PICK']),(['Character3','Character4','Detune','Tuning'],['@Character3Label','@Character4Label','DETUNE','TUNING'])],['A  SOURCE','B  RESONANCE']),('TOUCH',[(['Velocity','VelocityCurve','Hammer','Pedal'],['VELOCITY','CURVE','HAMMER','PEDAL']),(['Noise','KeyOff','Resonance','Release'],['MECHANICAL','KEY OFF','RESONANCE','RELEASE'])],['A  PLAYER','B  RESPONSE']),('MOTION',[(['Rate','Depth','Drift','Width'],['RATE','DEPTH','DRIFT','WIDTH']),(['Shape','MotionDelay','MotionFade','DriftRate'],['SHAPE','DELAY','FADE IN','DRIFT RATE'])],['A  MOTION','B  TIMING']),('FX',[(['Drive','Chorus','Delay','Room'],['DRIVE','CHORUS','DELAY','ROOM']),(['DelayDivision','DelayFeedback','RoomTime','RoomDamp'],['DELAY SYNC','FEEDBACK','ROOM TIME','DAMPING']),(['ChorusRate','ChorusDepth','Master','Width'],['CHORUS RATE','CHORUS DEPTH','MASTER','WIDTH'])],['A  COLOR','B  SPACE','C  STEREO']),('SETUP',[(['Master','Tuning','PitchRange','InputMode'],['MASTER','TUNING','BEND RANGE','KEYS / PADS']),(['Key','Transpose','StrumVelocity','Pedal'],['KEY','TRANSPOSE','STRUM VEL','PEDAL'])],['A  OUTPUT / INPUT','B  PERFORMANCE'])]
tabs=[];maps=[]
def qmap(tab,groups):
 assign={i:-1 for i in range(1,17)}
 for bank,ks in enumerate(groups):
  for j,k in enumerate(ks):assign[13+bank-4*j]=keys[k]
 return O(('Tab',get(tab,'fnKeyIndex')+1),('SubTab',get(tab,'fnKeySubIndex')+1),('Bank Direction','Column'),('Q-Links',O(*[('Q-Link '+str(i),assign[i])for i in range(1,17)])))
def finish(name,fn,sub,ws,groups,guides):
 tab=copy.deepcopy(template);put(tab,'tabName',name);put(tab,'fnKeyIndex',fn);put(tab,'fnKeySubIndex',sub);put(tab,'componentDefinition',g.component_definition([],ws,'ff123c30'));put(tab,'qlinkBoundsData',guides);tabs.append(tab);maps.append(qmap(tab,groups))
finish('MAIN',0,0,main,[[f'MainQ{i*4+j}'for j in range(4)]for i in range(4)],['24 167 608 126','655 167 608 126','24 330 608 126','655 330 608 126'])
# Native CHORD draws native row guides; its direct EDIT relays do not depend on MAIN modal state.
nativeEditor=copy.deepcopy(editor);nativeEditor.pop(6);nativeEditor[1]=label('CHORDS · Q-LINK','16 84 300 44',25,'fff2edd7')
finish('CHORD',0,1,nativeEditor,[[f'Edit{i*4+j}'for j in range(4)]for i in range(4)],[f'116 {142+i*82} 680 78'for i in range(4)])
for fn,(name,gs,titles)in enumerate(settingsPages,1):
 ws=base()+header();guides=[]
 for bank,((ks,labels),(x,y))in enumerate(zip(gs,[(16,139),(647,139),(16,302),(647,302)])):
  ws.append(panel(titles[bank],x,y));guides.append(f'{x+8} {y+28} 608 126')
  for col,(k,title)in enumerate(zip(ks,labels)):ws.append(knob(k,title,x+8+col*151,y+29))
 if name=='MOTION':ws +=[select('TREM','MotionMode','371 304 74 44',0),select('PAN','MotionMode','453 304 74 44',1),toggle('SYNC','Sync','535 304 80 44')]
 if name=='SETUP':ws +=[action('ALL NOTES OFF','Panic','24 474 292 44')]
 ws +=actions();finish(name,fn,0,ws,[x[0]for x in gs],guides)
# All selectors are sidebars above ordinary content, including every detail/native
# page and the inline chord editor. Choice state uses UI-local handles only.
def options_for(key):
 if key.startswith('Edit'):
  field=int(key[4:])%4
  return (['C','C#','D','Eb','E','F','F#','G','Ab','A','Bb','B']if field==0 else meta[keys['Type1']]['choices']if field==1 else meta[keys['Voicing1']]['choices']if field==2 else ['−2 oct','−1 oct','0 oct','+1 oct','+2 oct'])
 m=meta[keys[key]]
 if'choices'in m:return m['choices']
 if key=='Transpose':return [str(x)for x in range(-12,13)]
 return [str(x)for x in range(int(m['min']),int(m['max'])+1)]
def option(text,key,index,count,b,opened):
 global serial
 serial+=1;name='Option'+str(serial);x,y,w,h=map(int,b.split());parts=[g.entry('choice state','Button',g.button_data(plate(w,h,True,text=text),plate(w,h,text=text),index,count),f'0 0 {w} {h}','No','Show','Always',remap=[('Data',H(key))]),local_hit(H(key),f'0 0 {w} {h}',index,count)];define(name,parts,[g.action('Mouse Down','Toggle Switch',opened,'')]);return instance(name,b)
for key in sorted(selectors,key=lambda k:keys[k]):
 opened='Local/2/fcChoice'+str(keys[key]);pagehandle='Local/64/fcChoicePage'+str(keys[key]);options=options_for(key);n=len(options);pages=max(1,math.ceil(n/8));ws=[image('sidebar.png','0 0 426 628'),label('CHOOSE SOUND'if key=='Preset'else meta[keys[key]]['name'].upper(),'12 8 340 44',23,'fff5edd7'),label('×','366 8 48 44',26,'fff5edd7'),local_hit(opened,'366 8 48 44',0),local_hit(opened,'426 0 854 628',0)]
 if key=='Preset':
  category='Local/4/fcPresetCategory';groups=[('INIT',[(1,'Init')]),('EP',[(i+1,x['name'])for i,x in enumerate(programs)if i and x['model']<2]),('PIANO',[(i+1,x['name'])for i,x in enumerate(programs)if i and 2<=x['model']<4]),('PLUCK',[(i+1,x['name'])for i,x in enumerate(programs)if i and x['model']>=4])]
  for cat,(title,items)in enumerate(groups):
   if cat==0:
    root=[option('INIT','Preset',1,n,'12 76 402 52',opened)]
    for j,text in enumerate(['EP','PIANO','PLUCK'],1):
     global_name='Cat'+str(j);define(global_name,[image(plate(402,52,text=text),'0 0 402 52'),local_hit(category,'0 0 402 52',j,4),image('hit.png','0 0 1 1')]);root.append(instance(global_name,f'12 {76+j*64} 402 52'))
    define('PresetCategoryRoot',root);ws.append(instance('PresetCategoryRoot',condition='IndexedEnabling/0/4/'+category));continue
   for page in range(math.ceil(len(items)/8)):
    pagehandle='Local/64/fcPresetPage'+str(cat)
    children=[]
    for row,(index,text)in enumerate(items[page*8:(page+1)*8]):children.append(option(text,key,index,n,f'12 {66+row*60} 402 52',opened))
    for value,x,title in [(0,12,'BACK'),(max(0,page-1),152,'<'),(min(math.ceil(len(items)/8)-1,page+1),286,'>')]:
     handle=category if title=='BACK'else pagehandle;count=4 if title=='BACK'else 64;children +=[image(plate(128,44,text=title),f'{x} 566 128 44'),local_hit(handle,f'{x} 566 128 44',value,count)]
    name=f'PresetCat{cat}Page{page}';define(name,children)
    name2=name+'WhenPage';define(name2,[instance(name,condition=f'IndexedEnabling/{page}/64/'+pagehandle)]);ws.append(instance(name2,condition=f'IndexedEnabling/{cat}/4/'+category))
 else:
  for page in range(pages):
   children=[option(text,key,index,n,f'12 {64+row*60} 402 52',opened)for row,(index,text)in enumerate(list(enumerate(options))[page*8:(page+1)*8])]
   if pages>1:
    for x,value,title in [(12,max(0,page-1),'<'),(286,min(pages-1,page+1),'>')]:children +=[image(plate(128,44,text=title),f'{x} 566 128 44'),local_hit(pagehandle,f'{x} 566 128 44',value,64)]
   name='ChoicePage'+str(keys[key])+'-'+str(page);define(name,children);ws.append(instance(name,condition=f'IndexedEnabling/{page}/64/'+pagehandle))
 define('ChoicePanel'+str(keys[key]),ws,modal=True)
# Sidebars are topmost siblings. INLINE editor also gets its own roots to remain
# fully functional while the native MAIN root is behind it.
modalRoots=[instance('ChoicePanel'+str(keys[k]),condition='IndexedEnabling/1/2/Local/2/fcChoice'+str(keys[k]))for k in sorted(selectors,key=lambda k:keys[k])]
for tab in tabs:get(get(tab,'componentDefinition'),'componentsData').extend(copy.deepcopy(modalRoots))

lookup={get(d,'key'):d for d in defs};reachable=set()
def references(node):
 for w in get(node,'componentsData'):
  typ=get(get(w,'componentData'),'type')
  if typ in lookup and typ not in reachable:
   reachable.add(typ);references(get(lookup[typ],'value'))
for tab in tabs:references(get(tab,'componentDefinition'))
for name in ['knob overlay','knob overlay bipolar','menu overlay']:
 if name in lookup and name not in reachable:reachable.add(name);references(get(lookup[name],'value'))
defs[:]=[d for d in defs if get(d,'key')in reachable]
put(pd,'tabs',tabs);put(get(ql,'Screen Mode Q-Links'),'map',maps)
realMain=[['Tone','Body','Character3','Hiss'],['Attack','Decay','Sustain','Release'],['Rate','Depth','Drift','Width'],['Drive','Chorus','Delay','Room']];put(ql,'Program Mode Q-Links',copy.deepcopy(get(qmap(tabs[0],realMain),'Q-Links')))
(out/'TUI.json').write_text(g.dump_json(tui)+'\n');(out/'Q-Links.json').write_text(g.dump_json(ql)+'\n')
print('Felt Club:7native pages; approved green MAIN, ADSR,72pxdials,70pxpads; actual Screen maps and choice sidebars')
