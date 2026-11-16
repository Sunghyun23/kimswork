#!/usr/bin/env python3
"""Native1280x628 Crate Press skin. Live telemetry uses read-only Image filmstrips."""
from pathlib import Path
import sys,json,math,shutil,copy
from PIL import Image,ImageDraw,ImageFont,ImageOps
P=Path(__file__).resolve().parents[1];SDK=P.parents[2]/'third_party/mpc-plugin-builder-sdk'
sys.path.insert(0,str(SDK/'tools'));import gen_ui as g
from approved_type import ApprovedType
O=g.obj
params=g.load_params(str(P/'build/params.json'));ids={p.name:p.index for p in params}
table=g.ParamTable(params);pages,style,_,_,kit=g.load_ui(str(P/'ui.json'),table)
out=P/'skins';out.mkdir(exist_ok=True)
kitdir,assets=g.kit_dir_for(kit,str(P/'build'),str(out))
tui=g.build_tui(pages,params,style,None,127,assets);ql=g.build_qlinks(pages,params)
def get(o,k):return next(v for a,v in o if a==k)
def put(o,k,v):
 for i,(a,_)in enumerate(o):
  if a==k:o[i]=(k,v);return
 o.append((k,v))
pd=get(tui,'pageData');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions');tabs=get(pd,'tabs')
def h(name):return 'Parameter '+str(ids[name])
def label(text,b,size=22,color='fff2d9ad',handle=None):
 if not handle and text in('WAVE','INPUT','OUTPUT','LOW','MID','HIGH','DUCK'):
  x,y,w,hh=map(int,b.split());file='approved-monitor-'+text+f'-{w}-{hh}.png';save(approved.monitor(text,(w,hh),'#'+color[-6:],int(size)),file);return img(file,b)
 d=g.label_data('Value'if handle else'Text',color,size,'horizontallyCentred verticallyCentred',None if handle else text)
 if handle:
  font=get(get(d,'textStyle'),'font');put(font,'name','Roboto');put(font,'style','Regular')
 return g.entry(text or 'readout','Label',d,b,'No','Show','Always',remap=[]if handle=='Data'else[('Data',handle or'Parameter 0')])
def img(file,b,handle=None,frames=0):
 d=O(('version',2),('imageType','FilmStrip'if frames else'Regular'),('colour','0'),('image',file))
 if frames:d.extend([('numFrames',frames),('handleName',handle),('animated',False)])
 return g.entry(file,'Image',d,b,'No','Show','Always')
def define(name,children,actions=None):
 defs.append(O(('key',name),('value',g.component_definition(actions or[],copy.deepcopy(children),'00000000',ignore_mouse=not actions))))
def inst(name,b,handle=None):return g.entry(name,name,O(('version',1),('handleName','Data')),b,'Yes'if handle else'Children','Show','Always',remap=[]if handle=='Data'else[('Data',handle or'Parameter 0')])
def save(im,name):im.save(out/name);return name
# Header-only artwork. The concept's waveform, meters and controls are never baked into runtime.
approved=ApprovedType(P/'design/main-concept-v6-performance-eight.png')
concept=Image.open(P/'design/main-concept-v3.png').convert('RGB')
header=Image.open(P/'design/main-concept-v4.png').convert('RGB')
save(header.crop((0,76,1586,151)).resize((1280,52),Image.Resampling.LANCZOS),'header.png')
texture=concept.crop((35,529,70,580)).convert('RGB')
paper=Image.new('RGB',(628,158))
for yy in range(0,158,texture.height):
 for xx in range(0,628,texture.width):
  tile=texture.transpose(Image.Transpose.FLIP_LEFT_RIGHT)if (xx//texture.width)%2 else texture
  if (yy//texture.height)%2:tile=tile.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
  paper.paste(tile,(xx,yy))
panel=paper.copy();d=ImageDraw.Draw(panel);d.rounded_rectangle((1,1,626,156),9,outline='#997349',width=2);d.line((16,30,612,30),fill='#98764c',width=1);save(panel,'panel.png')
base=Image.new('RGB',(1280,628),'#24170f');base.paste(Image.open(out/'header.png'),(0,0));draw=ImageDraw.Draw(base)
draw.rounded_rectangle((8,70,1271,224),8,fill='#160f0b',outline='#a16b38',width=2)
for x in (616,944):draw.line((x,82,x,213),fill='#79522e')
draw.line((24,146,600,146),fill='#69543b')
# Input/Output waveform signed extrema: positive and negative bins share the zero line.
for x in range(24,601,72):draw.line((x,104,x,188),fill='#2c2118')
for x in range(720,914,48):draw.line((x,112,x,202),fill='#3b2b1c')
for y in range(112,203,18):draw.line((720,y,912,y),fill='#3b2b1c')
draw.line((720,202,912,112),fill='#786348',width=1)
compact=Image.new('RGB',(1280,628),'#24170f');compact.paste(Image.open(out/'header.png'),(0,0));compact.paste(base.crop((0,70,1280,226)).resize((1280,140),Image.Resampling.LANCZOS),(0,60));save(compact,'background.png')
# Small filmstrips use actual pixel sizes and remain below8192px texture height.
for color,name,width in [('#d2c5a7','in',9),('#ee974b','out',5)]:
 for sign in ('pos','neg'):
  im=Image.new('RGBA',(width,42*43));dr=ImageDraw.Draw(im)
  for frame in range(43):
   n=frame
   if n:dr.rectangle((0,frame*42+(42-n if sign=='pos' else 0),width-1,frame*42+(41 if sign=='pos' else n-1)),fill=color)
  save(im,f'wave-{name}-{sign}.png')
curve=Image.new('RGBA',(4,90*49));dr=ImageDraw.Draw(curve)
for frame in range(49):
 y=frame*90+round((1-frame/48)*87);dr.rectangle((0,y,3,y+2),fill='#ffa350')
save(curve,'curve.png')
Image.new('RGBA',(1,1),(0,0,0,0)).save(out/'hit.png')
puck=Image.new('RGBA',(7,7));ImageDraw.Draw(puck).ellipse((0,0,6,6),fill='#fff3d3');save(puck,'puck.png')
font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Georgia.ttf',17)
for text in ['FULL','LOW','MID','HIGH']:
 for on in (False,True):
  im=Image.new('RGB',(58,30),'#b66b2c'if on else'#332318');d=ImageDraw.Draw(im);d.rounded_rectangle((0,0,57,29),4,outline='#b68a54');
  if text=='FULL':d.text((29,15),text,font=font,anchor='mm',fill='#fff0ca')
  else:
   caption=approved.monitor(text,(58,30),'#fff0ca',17);im.paste(caption,(0,0),caption)
  save(im,f'{text}-{int(on)}.png')
# One physical-sized knob strip, with quiet metal and copper pointer.64frames fit78px texture.
knob=Image.new('RGBA',(64,64*97))
face=concept.crop((88,489,184,585)).convert('RGBA').resize((64,64),Image.Resampling.LANCZOS)
mask=Image.new('L',(64,64));ImageDraw.Draw(mask).ellipse((2,2,62,62),fill=255);face.putalpha(mask)
for frame in range(97):
 rotated=face.rotate(-(135+frame/96*270-141),resample=Image.Resampling.BICUBIC)
 knob.paste(rotated,(0,frame*64))
save(knob,'knob.png')
fader=Image.new('RGBA',(44,64*97))
for frame in range(97):
 im=Image.new('RGBA',(44,64));dr=ImageDraw.Draw(im);dr.rounded_rectangle((18,2,26,62),4,fill='#35271b',outline='#ac8350');y=round(52-frame/96*42);dr.rounded_rectangle((4,y-5,40,y+7),3,fill='#bda782',outline='#3a2919',width=2);dr.line((8,y+1,36,y+1),fill='#f1dfbd',width=2);fader.paste(im,(0,frame*64))
save(fader,'fader.png')
lcd=Image.new('RGBA',(140,28));ld=ImageDraw.Draw(lcd);ld.rounded_rectangle((0,0,139,27),5,fill='#3e2b1c',outline='#ad8650',width=1);save(lcd,'lcd.png')
cursor=Image.new('RGBA',(192,90*65));dr=ImageDraw.Draw(cursor)
for frame in range(65):
 x=round(frame/64*191);dr.line((x,frame*90,x,frame*90+89),fill='#fff3ce70',width=1)
save(cursor,'cursor.png')
main=[img('background.png','0 0 1280 628'),label('WAVE','24 76 76 26',23),label('INPUT','110 76 90 26',18,'ffd2c5a7'),label('OUTPUT','212 76 100 26',18,'ffee974b'),label('0.77 s','506 76 94 26',18)]
for k in range(4):
 name='in'if k<2 else'out';sign='pos'if k%2==0 else'neg';w=9 if k<2 else 5;y=104 if sign=='pos'else 146
 for j in range(64):main.append(img(f'wave-{name}-{sign}.png',f'{24+j*9+(2 if k>=2 else 0)} {y} {w} 42','Parameter '+str(41+k*64+j),43))
main +=[label('IN','24 192 32 24',17),label('','60 192 136 24',19,handle=h('Input Peak')),label('OUT','338 192 44 24',17),label('','386 192 136 24',19,handle=h('Output Peak')),label('IN dB','772 213 90 12',12),label('OUT','630 138 44 24',14)]
for i,name in enumerate(['FULL','LOW','MID','HIGH']):main.append(g.entry('curve '+name,'Button',g.button_data(f'{name}-1.png',f'{name}-0.png',i,4),f'{684+i*62} 76 58 30','Yes','Show','Always',remap=[('Data',h('Curve Band'))]))
main.extend([label('-48','678 192 32 20',14),label('-24','678 146 32 20',14),label('0','678 102 32 20',14),label('-48','720 198 28 16',14),label('-24','804 198 28 16',14),label('0','894 198 24 16',14),label('COMP GR','332 76 100 26',17),label('','432 76 100 26',18,handle=h('Comp GR'))])
for k in range(48):main.append(img('curve.png',f'{720+k*4} 112 4 90','Parameter '+str(297+k),49))
main.append(img('cursor.png','720 112 192 90','Parameter 345',65))
main.append(g.entry('signal operating point','XYPad',O(('version',1),('xPositionHandle','Parameter 345'),('yPositionHandle','Invert/Parameter 346'),('image','puck.png')),'720 112 192 90','No','Show','Always'))
# Cover the optional XY editor with a read-only parent. No touch control uses monitor handles.
for j,name in enumerate(['Low GR','Mid GR','High GR','Duck GR']):
 x=952+j*78;main.append(label(['LOW','MID','HIGH','DUCK'][j],f'{x} 78 76 24',18))
 im=Image.new('RGBA',(24,78*65));d=ImageDraw.Draw(im)
 for frame in range(65):
  value=max(0,min(1,((frame/64)*144-72)/24));height=round(value*76)
  d.rectangle((1,frame*78+1,22,frame*78+76),fill='#3d2b1e')
  if height:d.rectangle((2,frame*78+76-height,21,frame*78+76),fill='#e79342')
 save(im,f'gr{j}.png');main.extend([img(f'gr{j}.png',f'{x+26} 106 24 78',h(name),65),label('',f'{x} 186 76 28',18,handle=h(name))])
# Protect ALL monitor components from edits, including XYPad, without covering controls.
resized=set()
for e in main[1:]:
 b=get(e,'bounds');x,y,w,hh=map(int,get(b,'bounds').split());newy=round(60+(y-70)*140/156);newh=max(1,round(hh*140/156));put(b,'bounds',f'{x} {newy} {w} {newh}')
 c=get(e,'componentData');kind=get(c,'type');data=get(c,'data')
 if kind=='Image':files=[(get(data,'image'),get(data,'numFrames')if any(k=='numFrames'for k,_ in data)else 1)]
 elif kind=='Button':files=[(get(data,'onImage'),1),(get(data,'offImage'),1)]
 else:files=[]
 for name,frames in files:
  if name in resized:continue
  source=Image.open(out/name).convert('RGBA');fh=source.height//frames;result=Image.new('RGBA',(w,newh*frames))
  for frame in range(frames):result.paste(source.crop((0,frame*fh,source.width,(frame+1)*fh)).resize((w,newh),Image.Resampling.LANCZOS),(0,frame*newh))
  save(result,name);resized.add(name)
monitor=main[1:];define('ReadOnlyMonitor',monitor);main=main[:1]+[inst('ReadOnlyMonitor','0 0 1280 202')]
# Transparent no-op parent prevents child gestures; read-only Image widgets never write.
comp=get(defs[-1],'value');put(comp,'ignoreMousePresses',True)
# Curve selection buttons must remain interactive; put them outside read-only monitor.
children=get(comp,'componentsData');selectors=[e for e in children if get(get(e,'componentData'),'name').startswith('curve ')];put(comp,'componentsData',[e for e in children if e not in selectors]);main.extend(selectors)
# XYPad has no safe documented read-only flag. Replace with Image filmstrip dot column instead.
put(comp,'componentsData',[e for e in get(comp,'componentsData')if get(get(e,'componentData'),'type')!='XYPad'])
# Native controls retained on all detail pages. Main widgets are newly laid out2x2.
groups=[('A · COMP / DRIVE',['Drive','Compress','Punch','Release'],8,206),('B · BAND / DUCK',['Low Comp','Mid Comp','High Comp','Duck'],644,206),('C · COLOR',['Tone','High Cut','Lo-Fi','Drift'],8,372),('D · I/O / MIX',['Input','Low Cut','Mix','Output'],644,372)]
for group,(title,names,x,y)in enumerate(groups):
 save(approved.heading(group),f'approved-title-{group}.png')
 main.extend([img('panel.png',f'{x} {y} 628 158'),img(f'approved-title-{group}.png',f'{x+12} {y+2} 604 28')])
 for j,name in enumerate(names):
  xx=x+12+j*153;control='CPControl'+str(ids[name]);save(approved.control(name),f'approved-control-{ids[name]}.png');parts=[img(f'approved-control-{ids[name]}.png','0 0 148 24'),g.entry('knob','Knob',g.knob_filmstrip_data('fader.png'if name in ('Input','Mix','Output')else'knob.png',97,'Vertical'),'52 24 44 64'if name in ('Input','Mix','Output')else'42 24 64 64','No','Show','Always'),img('lcd.png','4 90 140 28'),label('','4 90 140 28',23,'fff3dfb6',handle='Data')];define(control,parts,g.control_actions('knob overlay'));main.append(inst(control,f'{xx} {y+32} 148 120',h(name)))

# Eight touch-only performance buttons. Program/Screen MAIN mapping is unchanged.
performance=Image.open(P/'design/main-concept-v6-performance-eight.png').convert('RGB')
# Captions come directly from the approved artwork, not a replacement desktop font.
labels=['TAPE STOP','REVERSE','REPEAT','DUB ECHO','FILTER DIP','CRUSH','WOBBLE','DROPOUT']
gateNames=['Tape Stop','Reverse','Repeat','Dub Echo','Filter Dip Hold','Crush','Wobble','Dropout']
centers=[110,306,502,696,898,1092,1288,1482]
for i,(text,gate,cx)in enumerate(zip(labels,gateNames,centers)):
 x=8+i*158+(8 if i>=4 else 0);w=150;hh=82
 for on in(False,True):
  plate=Image.new('RGB',(w,hh),'#86501f'if on else'#332115');dd=ImageDraw.Draw(plate);dd.rounded_rectangle((1,1,w-2,hh-2),9,outline='#edb36c'if on else'#ae7740',width=2);dd.rounded_rectangle((5,5,w-6,hh-6),6,outline='#9a5d2d'if on else'#54331e')
  icon=performance.crop((cx-43,766,cx+43,808)).resize((48,24),Image.Resampling.LANCZOS);iconMask=ImageOps.grayscale(icon).point(lambda v:255 if v>165 else 0);cleanIcon=Image.new('RGB',(48,24),'#ffe1a7'if on else'#eed2a0');plate.paste(cleanIcon,(51,7),iconMask)
  caption=approved.performance(i,color='#ffe1a7'if on else'#f0d5a6');plate.paste(caption,(4,34),caption)
  save(plate,f'perform-{i}-{int(on)}.png')
 raw=h(gate);state=img(f'perform-{i}-0.png',f'0 0 {w} {hh}')
 # Artwork only; a Button must not own the gesture. The caption is baked into the image.
 strip=Image.new('RGBA',(w,hh*2));strip.paste(Image.open(out/f'perform-{i}-0.png'),(0,0));strip.paste(Image.open(out/f'perform-{i}-1.png'),(0,hh));save(strip,f'perform-{i}.png')
 progress=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','ffffc16a'),('isBipolar',False),('handleName',h('FX Activity '+str(i+1))))
 surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+raw))
 name='PerformHold'+str(i);define(name,[img(f'perform-{i}.png',f'0 0 {w} {hh}',raw,2),g.entry('effect progress','Slider',progress,f'8 {hh-7} {w-16} 3','No','Show','Always'),g.entry('hold touch surface','Slider',surface,f'0 0 {w} {hh}','Yes','Show','Always')],[g.action('Mouse Down','Toggle Switch',raw,'')]);entry=inst(name,f'{x} 538 {w} {hh}');bounds=get(entry,'bounds');put(bounds,'version',2);bounds.append(('additionalInvalidatingHandles',['IndexedEnabling/0/2/'+h('LATCH')]));main.append(entry)
 # LATCH uses an ordinary toggle hit surface. It never receives the momentary reset handler.
 hit=Image.new('RGBA',(w,hh));save(hit,f'perform-hit-{i}.png')
 latchName='PerformLatch'+str(i);latchChildren=[img(f'perform-{i}.png',f'0 0 {w} {hh}',raw,2),g.entry('latched effect progress','Slider',copy.deepcopy(progress),f'8 {hh-7} {w-16} 3','No','Show','Always'),g.entry('latch tap','Button',g.button_data(f'perform-hit-{i}.png',f'perform-hit-{i}.png',1,1),f'0 0 {w} {hh}','Yes','Show','Always',remap=[('Data',raw)])];define(latchName,latchChildren);entry=inst(latchName,f'{x} 538 {w} {hh}');bounds=get(entry,'bounds');put(bounds,'version',2);bounds.append(('additionalInvalidatingHandles',['IndexedEnabling/1/2/'+h('LATCH')]));main.append(entry)
# Global LATCH occupies header space, preserving all eight performance buttons.
for on in(False,True):
 im=Image.new('RGB',(140,44),'#97591f'if on else'#302014');dd=ImageDraw.Draw(im);dd.rounded_rectangle((1,1,138,42),6,outline='#efb365'if on else'#997348',width=2);dd.ellipse((13,17,22,26),fill='#ffd073'if on else'#70533a');caption=approved.latch(color='#ffe4ae'if on else'#d8bd91');im.paste(caption,(33,8),caption);save(im,f'latch-{int(on)}.png')
main.append(g.entry('LATCH','Button',g.button_data('latch-1.png','latch-0.png',1,1),'624 4 140 44','Yes','Show','Always',remap=[('Data',h('LATCH'))]))

put(tabs[0],'componentDefinition',g.component_definition([],main,'ff24170f'))
put(tabs[0],'qlinkBoundsData',[f'{x} {y} 628 158'for _,_,x,y in groups])
# Touch-readout uses native overlay; function-key detail pages preserve generated controls.
for e in get(get(ql,'Screen Mode Q-Links'),'map'):
 tab=get(e,'Tab')-1;assignment={}
 for group in range(4):
  for knobindex in range(4):
   i=group*4+knobindex
   if i<len(pages[tab].controls)and pages[tab].controls[i].param:assignment[13+group-4*knobindex]=pages[tab].controls[i].param.index
 put(e,'Q-Links',O(*[('Q-Link '+str(n),assignment.get(n,-1))for n in range(1,17)]))
first=get(get(get(ql,'Screen Mode Q-Links'),'map')[0],'Q-Links');put(ql,'Program Mode Q-Links',copy.deepcopy(first))
for definition in defs:
 if 'overlay' in get(definition,'key'):
  component=get(definition,'value');put(component,'version',4);component.insert(next(i for i,(k,_) in enumerate(component)if k=='componentsData'),('hideQLinkBounds',True))
# Bake switch captions into their own buttons: decorative siblings intercept touches on this host.
caption_font_path='/System/Library/Fonts/Helvetica.ttc'
for tab_index,tab in enumerate(tabs[1:],1):
 children=get(get(tab,'componentDefinition'),'componentsData');remove=[]
 for index,entry in enumerate(children):
  spec=get(entry,'componentData');kind=get(spec,'type')
  if not kind.startswith('radio'):continue
  param_name=get(spec,'name');choices=params[ids[param_name]].choices
  original=next(get(d,'value')for d in defs if get(d,'key')==kind);custom=copy.deepcopy(original)
  put(custom,'actions',[]);put(custom,'ignoreMousePresses',True)
  for button in get(custom,'componentsData'):
   bd=get(button,'componentData')
   if get(bd,'type')!='Button':continue
   data=get(bd,'data');choice=get(data,'buttonId');text=choices[choice].replace(' DOT','·')
   _,_,w,hh=map(int,get(get(button,'bounds'),'bounds').split());font_size=18;font=ImageFont.truetype(caption_font_path,font_size)
   while font.getlength(text)>w-6 and font_size>10:
    font_size-=1;font=ImageFont.truetype(caption_font_path,font_size)
   for state in ('onImage','offImage'):
    source=get(data,state);art=Image.open(Path(kitdir)/source).convert('RGBA')
    ImageDraw.Draw(art).text((w/2,hh/2),text,font=font,anchor='mm',fill='#fff0d2')
    file=f'choice-{tab_index}-{index}-{choice}-{state}.png';save(art,file);put(data,state,file)
  name=f'CPChoice{tab_index}_{index}';defs.append(O(('key',name),('value',custom)));put(spec,'type',name)
  remove.extend(e for e in children if get(get(e,'componentData'),'name').startswith(param_name+' seg '))
 put(get(tab,'componentDefinition'),'componentsData',[e for e in children if e not in remove])
(out/'TUI.json').write_text(g.dump_json(tui)+'\n');(out/'Q-Links.json').write_text(g.dump_json(ql)+'\n')
# Runtime includes only referenced assets, not unused kit images or design sources.
import re
refs=set(re.findall(r'"([^"\n]+\.(?:png|jpg|jpeg))"',(out/'TUI.json').read_text()))
for ref in refs:
 if not(out/ref).exists() and (Path(kitdir)/ref).exists():shutil.copy2(Path(kitdir)/ref,out/ref)
for f in out.glob('*.png'):
 if f.name not in refs:f.unlink()
print('Crate Press native MAIN skin: real signed extrema, shared DSP transfer curve,4x4Q-Link')
