#!/usr/bin/env python3
"""Compile custom MPC skin using pinned SDK's ordered JSON writer (duplicate keys preserved)."""
import json,sys,shutil,subprocess,copy
from pathlib import Path
P=Path(__file__).resolve().parents[1]; ROOT=P.parents[2]; SDK=ROOT/'third_party/mpc-plugin-builder-sdk'
sys.path.insert(0,str(SDK/'tools'))
import gen_ui as g
O=g.obj
params=g.load_params(str(P/'build/params.json'))
# host_sim infers kinds by sampling display strings and can misclassify
# low-range floats (e.g. octave). Use our stable declarations for UI types.
declared=json.loads((P/'parameters.json').read_text())
for p,m in zip(params,declared):
 p.kind='choice' if 'choices' in m else 'float'
 p.choices=m.get('choices')
# UI aliases select appended full-range controls while legacy declarations/IDs
# remain byte-identical for existing MPC projects and automation.
by_native={p.name:p for p in params}
by_native['Sound Preset'].name='Legacy Sound Preset'
by_native['Full Sound Preset'].name='Sound Preset'
for slot in range(1,17):
 old=by_native[f'Chord {slot} Type'];new=by_native[f'Chord {slot} Harmony'];old.name='Legacy '+old.name;new.name=f'Chord {slot} Type'
for bank in range(1,5):
 name='Chord Preset' if bank==1 else f'Chord Preset Bank {bank}'
 old=by_native[name];new=by_native[f'Harmony Preset {bank}'];old.name='Legacy '+old.name;new.name=name
for row in range(1,5):
 letter=chr(64+row);old=by_native[f'Edit {letter} Type'];new=by_native[f'Harmony Edit {letter}'];old.name='Legacy '+old.name;new.name=f'Edit {letter} Type'
for p in params:
 if p.name=='Tremolo Rate':p.name='Mod Rate'
 elif p.name=='Tremolo Depth':p.name='Mod Depth'
table=g.ParamTable(params)
pages,style,_,_,kit=g.load_ui(str(P/'ui.json'),table)
out=P/'skins';out.mkdir(exist_ok=True)
kitdir,assets=g.kit_dir_for(kit,str(P/'build'),str(out))
tui=g.build_tui(pages,params,style,None,127,assets);ql=g.build_qlinks(pages,params)
def get(o,k):return next(v for a,v in o if a==k)
def setv(o,k,v):
 for i,(a,_)in enumerate(o):
  if a==k:o[i]=(k,v);return
 o.append((k,v))
pd=get(tui,'pageData');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions');tabs=get(pd,'tabs')
by={p.name:p for p in params};meta=json.loads((P/'parameters.json').read_text());bykey={m['key']:i for i,m in enumerate(meta)}
for slot in range(1,17):bykey['Quality'+str(slot)]=bykey['HarmonyType'+str(slot)]
for bank in range(1,5):bykey['Progression'+('' if bank==1 else str(bank))]=bykey['HarmonyPreset'+str(bank)]
bykey['SoundPreset']=bykey['SoundCurrent']
bykey['Progression']=bykey['HarmonyCurrent']
bykey['Rate']=bykey['LfoRateControl'];bykey['TremRate']=bykey['ModRateControl']
def handle(key):return 'Parameter '+str(bykey[key])
def adddef(name,comp):defs.append(O(('key',name),('value',comp)))
def label(text,bounds,size=20,color='ff352d21',kind='Text',binding=None):
 return g.entry(text,'Label',g.label_data(kind,color,size,'horizontallyCentred verticallyCentred',text if kind=='Text' else None),bounds,'No','Show','Always',remap=[('Data',binding or 'Parameter 0')] if kind=='Text' or binding else [])
def instance(name,definition,bounds,binding):return g.entry(name,definition,O(('version',1),('handleName','Data')),bounds,'Yes' if binding else 'No','Hide' if binding else 'Show','Always',remap=[('Data',binding or 'Parameter 0')])
# RotatingImage fits the entry bounds on the host; imageScale is an additional multiplier.
# Do not multiply by the source-to-control pixel ratio (it shrinks the knob twice).
shutil.copy2(P/'assets/knob-donut.png',out/'knob-donut.png')
knobchildren=[g.entry('knob','Knob',O(('version',4),('knobType','RotatingImage'),('foregroundImage','knob-donut.png'),('backgroundImage',''),('imageScale',1.0),('dragOrientation','Vertical'),('handleName','Data')),'43 0 60 60','No','Show','Always'),label('', '0 62 146 20',26,kind='Name'),label('','0 82 146 20',28,kind='Value')]
adddef('DTKnob',g.component_definition(g.control_actions('knob overlay'),knobchildren,'00000000'))
# Restoration styles the two definitions independently; sharing the child list
# causes both decorative layers to be inserted twice.
adddef('DTChoiceKnob',g.component_definition(g.control_actions('menu overlay'),copy.deepcopy(knobchildren),'00000000'))
# A native input widget owns touch/focus; labels alone are not menu hit targets.
# The standard double-tap/Enter menu remains available as a fallback.
def dropdown(bounds,size=28):
 style=get(g.label_data('Value','ff352d21',size,'horizontallyCentred verticallyCentred'),'textStyle')
 return g.entry('dropdown','ComboBox',O(('version',1),('textStyle',style),('handleName','Data')),bounds,'No','Show','Always')
adddef('DTMenu',g.component_definition(g.control_actions('menu overlay'),[dropdown('4 0 290 32')],'ffc9cd92'))
adddef('DTSmallMenu',g.component_definition(g.control_actions('menu overlay'),[dropdown('2 0 132 32',28)],'ffc9cd92'))
# Momentary-looking toggle parameters drive deterministic +/- commands in audio callback.
for name,caption in [('DTPrev','<'),('DTNext','>')]:
 adddef(name,g.component_definition(g.control_actions('menu overlay',double_click=False),[g.entry('button','Button',g.button_data('arrow.png','arrow.png',1,1),'0 0 36 34','No','Show','Always'),label(caption,'0 0 36 34',25)],'ffd6c298'))
children=[g.entry('background','Image',O(('version',2),('imageType','Regular'),('colour','0'),('image','main-background.png')),'0 0 1280 628','No','Show','Always')]
def menu(key,x,y,w=136):
 definition='DTMenu' if w>=290 else 'DTSmallMenu'
 children.append(instance(key,definition,f'{x} {y} {w} 32',handle(key)))
def preset(key,prev,next,x,y):
 children.extend([instance(prev,'DTPrev',f'{x} {y} 36 34',handle(prev)),instance(key,'DTMenu',f'{x+42} {y} 300 34',handle(key)),instance(next,'DTNext',f'{x+348} {y} 36 34',handle(next))])
preset('SoundPreset','SoundPrev','SoundNext',22,132)
children.append(label('SOUND PRESET','22 108 240 24',24))
menu('VoiceMode',440,132);menu('InputMode',596,132)
children.append(label('VOICE','440 108 136 24',24));children.append(label('INPUT','596 108 136 24',24))
groups=[('A  OSC / SOUND',['OscPair','Mix','Detune','Drift'],18,174),('B  FILTER / TONE',['Cutoff','Resonance','BodyTone','Sub'],650,174),('C  LFO 1 / 2',['Rate','Depth','Lfo2RateControl','Lfo2Depth'],18,365),('D  FX / MOTION',['Drive','Glide','TremRate','TremDepth'],650,365)]
# Real vertical filmstrip faders, one widget per ADSR parameter.
from PIL import Image,ImageDraw
frames=127;fw,fh=64,92
strip=Image.new('RGBA',(fw,fh*frames))
for frame in range(frames):
 im=Image.new('RGBA',(fw,fh));d=ImageDraw.Draw(im)
 d.rounded_rectangle((28,4,36,87),radius=4,fill='#352d21',outline='#ac7a42')
 for y in range(8,88,13):d.line((15,y,22,y),fill='#6c502e',width=1)
 cy=78-round(frame/(frames-1)*66)
 d.rounded_rectangle((13,cy-8,51,cy+8),radius=3,fill='#9c8b6c',outline='#352d21',width=2)
 for dy in [-4,-1,2,5]:d.line((17,cy+dy,47,cy+dy),fill='#e9d9b7')
 strip.alpha_composite(im,(0,frame*fh))
strip.save(out/'envelope-vertical.png')
adddef('DTEnvelope',g.component_definition(g.control_actions('knob overlay'),[label('','0 0 146 24',26,kind='Name'),g.entry('fader','Knob',g.knob_filmstrip_data('envelope-vertical.png',127,'Vertical'),'41 25 64 92','No','Show','Always'),label('','0 119 146 24',28,kind='Value')],'00000000'))
for title,keys,x,y in groups:
 name='header-'+title;adddef(name,g.component_definition([],[label(title,'8 0 590 25',28,'fff3dfb8')],'ff9c4e23'))
 children.append(instance(title,name,f'{x} {y} 612 25',''))
 for j,key in enumerate(keys):
  definition='DTEnvelope' if key.startswith('Amp') else 'DTChoiceKnob' if 'choices'in meta[bykey[key]] else 'DTKnob'
  ky=y+27 if key.startswith('Amp') else y+62 if title.startswith('D') else y+60
  height=143 if key.startswith('Amp') else 102
  children.append(instance(key,definition,f'{x+j*153} {ky} 146 {height}',('WithName['+key[3:].upper()+']/'+handle(key)) if key.startswith('Amp') else handle(key)))
adddef('DTDivider',g.component_definition([],[],'ff9c4e23',ignore_mouse=True))
children.append(instance('Filter / LFO divider','DTDivider','952 204 2 137',''))
children.append(instance('FX / Tremolo divider','DTDivider','952 426 2 103',''))
menu('Filter',740,201);menu('DriveType',962,201)
for key,x in [('Shape',654),('Target',807),('Sync',960),('Division',1113)]:menu(key,x,393)
for key,title in [('Filter','FLT'),('Shape','WAV'),('Target','TO'),('Sync','SYNC'),('Division','DIV')]:
 name='DTSelect'+key
 adddef(name,g.component_definition(g.control_actions('menu overlay'),[label(title,'2 0 22 30' if key=='Target' else '2 0 40 30',18),dropdown('24 0 110 32' if key=='Target' else '42 0 92 32',24)],'ffc9cd92'))
 for child in children:
  cd=get(child,'componentData')
  if get(cd,'name')==key:setv(cd,'type',name)
# Editable numeric touch fields retain Vibrato and Expression without consuming Q-Links.
adddef('DTTouchValue',g.component_definition(g.control_actions('knob overlay'),[g.entry('value','Knob',O(('version',5),('knobType','GenericLegacy'),('polarity','UniPolar'),('dragOrientation','Vertical'),('handleName','Data')),'0 0 32 28','No','Show','Always'),label('','36 0 146 28',24,kind='Name'),label('','182 0 104 28',26,kind='Value')],'ffe1d2ae'))
for key,x in [('Vibrato',24),('Expression',330)]:children.append(instance(key,'DTTouchValue',f'{x} 337 286 28',handle(key)))
children.append(label('CHORD PRESET','22 540 220 28',24))
preset('Progression','ChordPrev','ChordNext',22,574)
# Four touch chord gates; separate EDIT controls open the root editor.
for i in range(4):
 name='DTPad'+str(i)
 overlay=copy.deepcopy(next(get(d,'value') for d in defs if get(d,'key')=='knob overlay'))
 overlay_widget=get(overlay,'componentsData')[0]
 setv(overlay_widget,'handle remapping',O(('version',1),('map',[O(('key','Data'),('value',handle('Root'+str(i+1))))])))
 adddef(name+' root overlay',overlay)
 gate_button=g.button_data('pad-on.png','pad-off.png',1,2)
 # Artwork only: the final continuous gesture surface handles input.
 # Native Button + ResetOnGestureEnd ends immediately on MPC 3.10.1.
 setv(gate_button,'gestureBehaviour','Hold')
 setv(gate_button,'handleName',handle('Touch'+str(i+1)))
 comp=g.component_definition([g.action('Mouse Down','Toggle Switch',handle('Touch'+str(i+1)),'')],[g.entry('button','Button',gate_button,'0 0 195 48','No','Show','Always'),label('', '0 2 195 44',34,kind='Value',binding=handle('Root'+str(i+1)))],'00000000')
 adddef(name,comp);children.append(instance('Chord '+str(i+1),name,f'{438+i*204} 566 195 48',handle('Touch'+str(i+1))))
# Separate editing row cannot accidentally sound a chord.
for i in range(4):
 name='DTEdit'+str(i)
 adddef(name,g.component_definition([g.action('Mouse Down','Show Overlay','Data','DTPad'+str(i)+' root overlay')],[label('EDIT '+str(i+1),'0 0 110 28',22)],'ffd6c298'))
 children.append(instance('Edit chord '+str(i+1),name,f'{438+i*204} 536 110 28',handle('Root'+str(i+1))))
adddef('DTStop',g.component_definition([],[g.entry('stop','Button',g.button_data('menu-small.png','menu-small.png',1,1),'0 0 136 32','No','Show','Always'),label('STOP','0 0 136 32',28)],'ffd6c298'))
children.append(instance('Stop touch chords','DTStop','1110 134 136 32',handle('TouchStop')))
adddef('DTPlayHint',g.component_definition([],[label('CHORDS: HOLD TO PLAY','0 0 360 32',24)],'ffe1d2ae'))
children.append(instance('Touch chord instructions','DTPlayHint','746 134 360 32',''))
adddef('DTFXHint',g.component_definition([],[label('TAPE / CHORUS / DELAY   FX','0 0 500 30',26)],'ffe1d2ae'))
children.append(instance('FX overview','DTFXHint','746 100 500 30',''))
main=get(tabs[0],'componentDefinition');setv(main,'componentsData',children);setv(main,'backgroundData',g.background_data('ffe1d2ae'))
setv(tabs[0],'qlinkBoundsData',['18 174 612 190','650 174 612 190','18 365 612 170','650 365 612 170'])
# Q-Link numbers are a bottom-left-origin hardware grid, not sequential
# bank positions. Use the normal four-knob column order in both modes.
qmap=get(get(ql,'Screen Mode Q-Links'),'map');setv(qmap[0],'Bank Direction','Column')
ordered=[bykey[k]for _,keys,_,_ in groups for k in keys]
hardware_order=[13+bank-4*knob for bank in range(4) for knob in range(4)]
assignments=dict(zip(hardware_order,ordered))
setv(qmap[0],'Q-Links',g.Obj(('Q-Link '+str(n),assignments[n])for n in range(1,17)))
setv(ql,'Program Mode Q-Links',g.Obj(('Q-Link '+str(n),assignments[n])for n in range(1,17)))
# CHORD is AMP's second sub-tab, not an unsupported seventh function key.
setv(tabs[-1],'fnKeyIndex',3);setv(tabs[-1],'fnKeySubIndex',1);setv(qmap[-1],'Tab',4);setv(qmap[-1],'SubTab',2)
# Include all discrete replacement strings, including custom controls.
setv(pd,'replacementStrings',[O(('key',p.handle),('value',list(p.choices)))for p in params if p.is_discrete])
for img in g.referenced_images(tui,set()):
 src=Path(kitdir)/img
 if src.exists():shutil.copy2(src,out/img)
shutil.copy2(P/'assets/main-background-v2.png',out/'main-background.png')
# New control artwork, not a modification of the generated background.
from PIL import Image,ImageDraw
Image.new('RGBA',(36,34),(214,194,152,255)).save(out/'arrow.png')
for filename,fill in [('pad-off.png',(215,194,148)),('pad-on.png',(176,111,55))]:
 im=Image.new('RGBA',(195,48));d=ImageDraw.Draw(im);d.rounded_rectangle((1,1,193,46),radius=6,fill=fill,outline=(108,80,46),width=2);d.line((7,4,188,4),fill=(240,219,175),width=1);im.save(out/filename)
from chord_editor import install as install_chord_editor
install_chord_editor(g,tui,params,out)
# Touch-only selectors; the existing sixteen MAIN Q-Links stay fixed.
menu('ModMode',964,356);menu('ModShape',1114,356)
menu('Lfo2Shape',326,356);menu('Lfo2Target',476,356)
for key,caption in [('Lfo2Shape','WAV'),('Lfo2Target','TO')]:
 name='DTSelect'+key;adddef(name,g.component_definition(g.control_actions('menu overlay'),[label(caption,'2 0 22 30' if key=='Lfo2Target' else '2 0 40 30',18),dropdown('24 0 110 32' if key=='Lfo2Target' else '42 0 92 32',24)],'ffc9cd92'))
 for w in children:
  if get(get(w,'componentData'),'name')==key:setv(get(w,'componentData'),'type',name)
# Native Name labels use legacy host metadata unless explicitly overridden.
fx_widgets=get(get(next(t for t in tabs if get(t,'tabName')=='FX'),'componentDefinition'),'componentsData')
for w in list(fx_widgets):
 name=get(get(w,'componentData'),'name')
 if name in ('Mod Rate Control','Mod Depth'):
  caption='Mod Rate' if name=='Mod Rate Control' else name
  mapping=get(get(w,'handle remapping'),'map')[0];setv(mapping,'value','WithName['+caption+']/'+get(mapping,'value'))
 if name=='Motion Shape':
  setv(get(w,'componentData'),'type','DTSmallMenu');setv(get(w,'bounds'),'bounds','734 542 136 32')
  fx_widgets.append(label('Motion Shape','663 497 278 32',24))
mod_widgets=get(get(next(t for t in tabs if get(t,'tabName')=='MOD'),'componentDefinition'),'componentsData')
for w in list(mod_widgets):
 name=get(get(w,'componentData'),'name')
 if name in ('LFO 2 Shape','LFO 2 Target'):
  x,y,_,_=map(int,get(get(w,'bounds'),'bounds').split());setv(get(w,'componentData'),'type','DTSmallMenu');setv(get(w,'bounds'),'bounds',f'{x+71} {y+45} 136 32');mod_widgets.append(label(name,f'{x} {y} 278 32',24))
from choice_panels import install
install(g,tui,params,out)
from chord_editor import finish as finish_chord_editor
finish_chord_editor(g,tui,params)
# Mouse Down opens the gate; a continuous Slider gesture owns its release.
# Place it above both artwork and caption, replacing automatic Button hit copies.
# Verified on MPC Live III 3.10.1.12 with remote holds and physical touch/audio.
for i in range(4):
 comp=next(get(d,'value') for d in defs if get(d,'key')=='DTPad'+str(i))
 widgets=get(comp,'componentsData')
 widgets[:]=[c for c in widgets if not (get(get(c,'componentData'),'type')=='Button' and get(get(c,'componentData'),'name').endswith(' touch'))]
 surface=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+handle('Touch'+str(i+1))))
 widgets.append(g.entry('hold touch surface','Slider',surface,'0 0 195 48','Yes','Show','Always'))
# Compile the source artwork at the exact device pixel dimensions.
# MPC 3.10.1 displayed the original Image at native size despite its bounds.
# Lay out header and body as separate texture regions, not one approximate band.
# The generated source's brown band ends at row156 of887; map it to y0..93.
header_source=Image.open(P/'assets/main-background-brown-header.png').convert('RGBA')
body_source=Image.open(P/'assets/main-background-v4.png').convert('RGBA')
runtime=body_source.resize((1280,628),Image.Resampling.LANCZOS)
header_height=round(header_source.height*156/887)
header=header_source.crop((0,0,header_source.width,header_height)).resize((1280,94),Image.Resampling.LANCZOS)
runtime.paste(header,(0,0))
ImageDraw.Draw(runtime).line((0,93,1279,93),fill='#382218',width=1)
runtime.save(out/'main-background-runtime.png')
setv(get(children[0],'componentData'),'data',O(('version',2),('imageType','Regular'),('colour','0'),('image','main-background-runtime.png')))
from restore_design import install as restore_design
restore_design(g,tui,out)
from usability import install as usability
usability(g,tui,params,out)
from preset_browser import install as preset_browser
preset_browser(g,tui,params,out)
from chord_qlinks import install as chord_qlinks
chord_qlinks(g,tui,ql,params,out)
from chord_options import install as chord_options
chord_options(g,tui,params,out)
from sidebars import install as sidebars
sidebars(g,tui,params,out)
from chord_slots import install as chord_slots
chord_slots(g,tui,ql,params,out)
from key_reference import install as key_reference
key_reference(g,tui,params,out)
from tempo_controls import install as tempo_controls
tempo_controls(g,tui,ql,params,out)
from random_sound_ui import install as random_sound_ui
random_sound_ui(g,tui,params,out)
# Apply display-only names after all choice/browser bindings have been generated.
osc_widgets=get(get(next(t for t in tabs if get(t,'tabName')=='OSC'),'componentDefinition'),'componentsData')
for w in osc_widgets:
 if get(get(w,'componentData'),'name')=='Sound Preset':
  mapping=get(get(w,'handle remapping'),'map')[0];setv(mapping,'value','WithName[Sound Preset]/'+get(mapping,'value'))


# Manual pad-location hints retired: their per-slot 8x16 selection trees
# inflate CHORD construction on MPC. MIDI/LEARN and stored IDs remain intact.
# Later additions (bank arrows, pad hints) must never render or receive input
# above a modal sidebar. Keep every tab's ordinary siblings below its modals.
modal_defs={get(e,'key'):get(e,'value') for e in get(get(pd,'componentDefinitions'),'localComponentDefinitions')}
for tab in get(pd,'tabs'):
 widgets=get(get(tab,'componentDefinition'),'componentsData')
 def is_modal(w):
  definition=modal_defs.get(get(get(w,'componentData'),'type'),[])
  return any(k=='hideQLinkBounds' and v for k,v in definition)
 widgets[:]=[w for w in widgets if not is_modal(w)]+[w for w in widgets if is_modal(w)]
(out/'TUI.json').write_text(g.dump_json(tui)+'\n');(out/'Q-Links.json').write_text(g.dump_json(ql)+'\n')
print('Custom skins generated: MAIN, 5 detail tabs and one bank-selectable CHORD editor')
