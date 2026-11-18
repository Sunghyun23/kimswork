"""Approved key/circle reference, explicit transpose and compact pad-setup drawer."""
import copy, math, re
from PIL import Image, ImageDraw, ImageFont

MAJOR_PCS=(0,7,2,9,4,11,6,1,8,3,10,5)
MAJOR_NAMES=('C','G','D','A','E','B','F#/Gb','Db','Ab','Eb','Bb','F')
MINOR_NAMES=('Am','Em','Bm','F#m','C#m','G#m','D#m/Ebm','Bbm','Fm','Cm','Gm','Dm')
TONICS=(('C','Db','D','Eb','E','F','F#','G','Ab','A','Bb','B'),('C','C#','D','Eb','E','F','F#','G','G#','A','Bb','B'))
SCALES=((0,2,4,5,7,9,11),(0,2,3,5,7,8,10))

def diatonic(root,mode):
 tonic=TONICS[mode][root];letters='CDEFGAB';naturals=(0,2,4,5,7,9,11);first=letters.index(tonic[0]);names=[]
 suffixes=('', 'm','m','','','m','dim') if mode==0 else ('m','dim','','m','m','','')
 for degree,interval in enumerate(SCALES[mode]):
  letter=(first+degree)%7;acc=((root+interval-naturals[letter]+6)%12)-6
  names.append(letters[letter]+('#'*acc if acc>=0 else 'b'*(-acc))+suffixes[degree])
 return names

def draw_chart(path,root,mode):
 scale=3;im=Image.new('RGBA',(432*scale,440*scale),(0,0,0,0));d=ImageDraw.Draw(im)
 def font(size,bold=False):return ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial'+(' Bold' if bold else '')+'.ttf',size*scale)
 def text(x,y,s,size=20,bold=False,fill='#352d21'):
  d.text((x*scale,y*scale),s,font=font(size,bold),fill=fill,anchor='mm')
 cx,cy=216,180;outer,inner,core=174,122,70
 selected=(root+(3 if mode else 0))%12
 for i,pc in enumerate(MAJOR_PCS):
  start=-105+i*30;end=start+30
  box=((cx-outer)*scale,(cy-outer)*scale,(cx+outer)*scale,(cy+outer)*scale)
  d.pieslice(box,start,end,fill='#bc8045' if pc==selected else '#e7d8b0',outline='#79552c',width=scale)
 d.ellipse(((cx-inner)*scale,(cy-inner)*scale,(cx+inner)*scale,(cy+inner)*scale),outline='#79552c',width=scale)
 d.ellipse(((cx-core)*scale,(cy-core)*scale,(cx+core)*scale,(cy+core)*scale),fill='#e7d8b0',outline='#79552c',width=scale)
 for i in range(12):
  angle=math.radians(-90+i*30)
  for radius,label,size in ((148,MAJOR_NAMES[i],20),(96,MINOR_NAMES[i],17)):
   text(cx+math.cos(angle)*radius,cy+math.sin(angle)*radius,label.replace('/', '\n') if radius==96 else label,13 if radius==96 and '/' in label else size,True)
 text(cx,cy-19,TONICS[mode][root],28,True);text(cx,cy+11,'MINOR' if mode else 'MAJOR',20,True);text(cx,cy+37,'REFERENCE',11)
 d.rounded_rectangle((2*scale,369*scale,430*scale,436*scale),radius=7*scale,fill='#dbc594',outline='#79552c',width=scale)
 names=diatonic(root,mode);roman=('I','ii','iii','IV','V','vi','vii°') if not mode else ('i','ii°','III','iv','v','VI','VII')
 for row,indices in enumerate((range(4),range(4,7))):
  width=428/len(indices)
  for col,i in enumerate(indices):text(2+width*(col+.5),388+row*28,roman[i]+' '+names[i],17,True)
 im.resize((432,440),Image.Resampling.LANCZOS).save(path)

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 def put(o,k,v):
  for i,(a,_)in enumerate(o):
   if a==k:o[i]=(k,v);return
  o.append((k,v))
 def cd(c):return get(c,'componentData')
 pd=get(tui,'pageData');entries=get(get(pd,'componentDefinitions'),'localComponentDefinitions');defs={get(e,'key'):get(e,'value')for e in entries};by={p.name:p for p in params}
 def h(name):return by[name].handle
 def add(key,value):entries.append(O(('key',key),('value',value)));defs[key]=value
 def label(text,b,size=22,handle=None):
  data=g.label_data('Value' if handle else 'Text','ff352d21',size,'horizontallyCentred verticallyCentred',None if handle else text)
  font=get(get(data,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Regular')
  return g.entry(text or 'key value','Label',data,b,'No','Show','Always',remap=[('Data',handle or 'Parameter 0')])
 def visible(c,conditions):
  put(get(c,'bounds'),'version',2);put(get(c,'bounds'),'showWhenDataModelInvalid','Hide');put(get(c,'bounds'),'additionalInvalidatingHandles',conditions);return c
 def instance(name,b,conditions=None):
  e=g.entry(name,name,O(('version',1),('handleName','Data')),b,'Children','Show','Always',remap=[('Data','Parameter 0')]);return visible(e,conditions)if conditions else e
 def asset(w,height):
  for suffix,fill in [('off','#dbc594'),('on','#b06f37')]:
   file=f'key-{w}-{height}-{suffix}.png'
   if (out/file).exists():continue
   im=Image.new('RGBA',(w,height));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,w-2,height-2),5,fill=fill,outline='#79552c',width=2);d.line((7,4,w-8,4),fill='#ffe6b6');im.save(out/file)
 def button(text,handle,b,index=1,count=2,value=False):
  x,y,w,height=map(int,b.split());asset(w,height);name='DTKeyButton'+str(len(defs))
  items=[g.entry(text or 'key','Button',g.button_data(f'key-{w}-{height}-on.png',f'key-{w}-{height}-off.png',index,count),f'0 0 {w} {height}','No','Show','Always',remap=[('Data',handle)]),label(text,f'4 0 {w-8} {height}',21,handle if value else None),g.entry((text or 'key')+' touch','Button',g.button_data('button-hit.png','button-hit.png',index,count),f'0 0 {w} {height}','Yes','Show','Always',remap=[('Data',handle)])]
  add(name,g.component_definition([],items,'00000000'));return instance(name,b)
 def command(text,handle,b):
  # Mouse Down once; ResetOnGestureEnd only controls the local pressed artwork.
  x,y,w,height=map(int,b.split());asset(w,height);name='DTKeyCommand'+str(len(defs));pressed='Local/2/keyPress'+str(len(defs))
  parts=[g.entry(text,'Button',g.button_data(f'key-{w}-{height}-on.png',f'key-{w}-{height}-off.png',1,2),f'0 0 {w} {height}','No','Show','Always',remap=[('Data',pressed)]),label(text,f'0 0 {w} {height}',21)]
  surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+pressed))
  parts.append(g.entry('command press surface','Slider',surface,f'0 0 {w} {height}','Yes','Show','Always'))
  add(name,g.component_definition([g.action('Mouse Down','Toggle Switch',handle,''),g.action('Mouse Down','Toggle Switch',pressed,'')],parts,'00000000'));return instance(name,b)
 def outside(handle):return g.entry('Dismiss outside','Button',g.button_data('button-hit.png','button-hit.png',0,2),'426 0 854 628','Yes','Show','Always',remap=[('Data',handle)])
 def modal(key,widgets):
  comp=g.component_definition([],widgets,'90000000');put(comp,'version',4);pos=next(i for i,(k,_)in enumerate(comp)if k=='componentsData');comp.insert(pos,('hideQLinkBounds',True));add(key,comp)
 def background(b):
  name='DTKeyBackdrop'+str(len(defs));add(name,g.component_definition([],[],'ffe1d2ae',ignore_mouse=True));return instance(name,b)
 tab=next(t for t in get(pd,'tabs')if get(t,'tabName')=='CHORD');page=get(get(tab,'componentDefinition'),'componentsData');editor=get(defs['Editor_DTAllChords'],'componentsData')
 padlocal='Local/2/dtPadSetup';keylocal='Local/2/dtKeyPicker';sourcelocal='Local/2/dtSourcePicker'
 advanced=[copy.deepcopy(w)for w in editor if get(cd(w),'name')in ('ADVANCED','ADVANCED touch')]
 routing=[];keep=[]
 for w in editor:
  name=get(cd(w),'name');x,y,width,height=map(int,get(get(w,'bounds'),'bounds').split())
  if re.fullmatch(r'MIDI \d+',name)or name.startswith('LEARN ')or name.startswith('PRESS PHYSICAL')or (name=='slot value'and y==42):routing.append(copy.deepcopy(w));continue
  if y<81 or name in ('MIDI NOTE','PAD LEARN','Cancel learn','Cancel learn touch','CANCEL LEARN','ADVANCED','ADVANCED touch'):continue
  keep.append(w)
 editor[:]=keep
 editor.append(label('CHORD EDITOR','20 4 240 44',25))
 for bank in range(4):
  cond=[(f'IndexedEnabling/3/4/{h("Chord Slots")}'if bank==3 else f'RangedEnabling/{bank}/3/4/{h("Chord Slots")}')]
  editor.append(visible(button(f'{bank*4+1}–{bank*4+4}',h('Chord Pad Page'),f'{270+bank*130} 4 120 44',bank,4),cond))
 editor.extend([command('−',h('SlotsMinus'),'994 4 44 44'),label('','1044 4 160 44',22,h('Chord Slots')),command('+',h('SlotsPlus'),'1210 4 44 44')])
 editor.extend([label('KEY','832 60 60 44',22),button('',keylocal,'898 60 110 44',value=False),label('','902 60 102 44',22,h('Chord Key')),button('MAJOR',h('Chord Key Mode'),'1020 60 116 44',0,2),button('MINOR',h('Chord Key Mode'),'1140 60 116 44',1,2)])
 # Key caption must be below its transparent press surface, not an external sibling.
 keybutton=editor[-4];keydef=defs[get(cd(keybutton),'type')];parts=get(keydef,'componentsData');parts[:]=[w for w in parts if not(get(cd(w),'type')=='Label')];parts.insert(1,label('','4 0 102 44',22,h('Chord Key')));editor.pop(-3)
 for mode in range(2):
  for root in range(12):
   file=f'circle-key-{mode}-{root}.png';draw_chart(out/file,root,mode)
   chart=g.entry('Circle of fifths','Image',O(('version',2),('imageType','Regular'),('colour','0'),('image',file)),'832 108 432 440','No','Show','Always')
   editor.append(visible(chart,[f'IndexedEnabling/{root}/12/{h("Chord Key")}',f'IndexedEnabling/{mode}/2/{h("Chord Key Mode")}']))
 for row in range(4):editor.append(label('',f'8 {189+row*100} 80 20',18,h('Chord Degree '+str(row+1))))
 editor.extend([command('TRANSPOSE',h('Transpose Preview'),'620 564 210 44'),command('PAD SETUP',padlocal,'850 564 200 44')])
 # Preserve the existing advanced action handle and modal contract.
 if advanced:
  handle=get(get(advanced[-1],'handle remapping'),'map')[0];editor.append(button('ADVANCED',get(handle,'value'),'1070 564 190 44'))
 # PAD SETUP: existing real-slot MIDI and LEARN controls, same bank conditions.
 pad=[background('0 0 426 628'),label('PAD SETUP','12 8 340 44',25),button('X',padlocal,'366 8 48 44',0,2),label('MIDI NOTE','12 65 166 30',20),label('LEARN','200 65 206 30',20)]
 for w in routing:
  name=get(cd(w),'name');x,y,width,height=map(int,get(get(w,'bounds'),'bounds').split())
  if name.startswith('MIDI '):put(get(w,'bounds'),'bounds',f'12 {y} 166 90');pad.append(w)
  elif name.startswith('LEARN '):put(get(w,'bounds'),'bounds',f'210 {y} 180 44');pad.append(w)
  else:put(get(w,'bounds'),'bounds','12 98 402 28');pad.append(w)
 cancel=button('CANCEL LEARN',h('Pad Learn'),'12 564 198 44',0,17);art=get(get(defs[get(cd(cancel),'type')],'componentsData')[0],'componentData');put(get(art,'data'),'onImage','key-198-44-off.png')
 pad.extend([cancel,button('DONE',padlocal,'222 564 192 44',0,2),outside(padlocal)])
 modal('DTKeyPadSetup',pad);page.append(instance('DTKeyPadSetup','0 0 1280 628',[f'IndexedEnabling/1/2/{padlocal}']))
 def picker(key,local,param,title):
  rows=[background('0 0 426 628'),label(title,'12 8 340 44',25),button('X',local,'366 8 48 44',0,2)]
  for i,note in enumerate(TONICS[0]):
   item=button(note,h(param),f'{12+(i%2)*204} {68+(i//2)*76} 198 60',i,12);put(defs[get(cd(item),'type')],'actions',[g.action('Mouse Down','Toggle Switch',local,'')]);rows.append(item)
  rows.extend([button('DONE',local,'12 564 402 44',0,2),outside(local)])
  modal(key,rows);page.append(instance(key,'0 0 1280 628',[f'IndexedEnabling/1/2/{local}']))
 picker('DTKeyPicker',keylocal,'Chord Key','KEY REFERENCE')
 # Transpose confirmation drawer; source root remains adjustable for existing songs.
 trans=[background('0 0 426 628'),label('TRANSPOSE ALL SLOTS','12 8 402 44',25),label('FROM','12 76 80 44',21),button('',sourcelocal,'104 76 156 44'),label('','108 76 148 44',22,h('Transpose From')),button('MAJ',h('Transpose From Mode'),'268 76 70 44',0,2),button('MIN',h('Transpose From Mode'),'342 76 70 44',1,2),label('TO KEY','12 148 110 44',21),label('','124 148 144 44',25,h('Chord Key')),label('','268 148 144 44',21,h('Chord Key Mode')),label('','12 218 402 52',28,h('Transpose Interval')),label('ACTIVE SLOTS','12 280 196 40',20),label('','212 280 200 40',22,h('Chord Slots')),label('Root moves. Type, voicing, octave','12 344 402 30',19),label('and MIDI mapping stay unchanged.','12 374 402 30',19),label('','12 440 402 76',19,h('Transpose Status')),button('CANCEL',h('Transpose Preview'),'12 564 192 44',0,2),command('APPLY',h('Apply Transpose'),'216 564 198 44'),outside(h('Transpose Preview'))]
 # Put dynamic FROM caption inside its existing picker command, below hit.
 src=trans[3];parts=get(defs[get(cd(src),'type')],'componentsData');parts[:]=[w for w in parts if get(cd(w),'type')!='Label'];parts.insert(1,label('','4 0 148 44',22,h('Transpose From')));trans.pop(4)
 modal('DTKeyTranspose',trans);page.append(instance('DTKeyTranspose','0 0 1280 628',[f'IndexedEnabling/1/2/{h("Transpose Preview")}']))
 picker('DTKeySourcePicker',sourcelocal,'Transpose From','SOURCE KEY')

 from smooth_voicing_ui import install as smooth_voicing
 smooth_voicing(g,tui,params,(label,button,command,background,outside,modal,instance,h,get,visible))
