"""QA only: generate diagnostic skin; rebuild with --define DONUT_TOUCH_DIAGNOSTICS=1.
Restore production with build_skin.py and a build without the define. Never package this skin.
"""
import runpy,sys
from pathlib import Path
tools=Path(__file__).resolve().parent
sys.path.insert(0,str(tools))
ns=runpy.run_path(str(tools/'build_skin.py'))
g=ns['g'];get=ns['get'];setv=ns['setv'];defs=ns['defs'];tui=ns['tui'];out=ns['out']
for i in range(4):
 comp=next(get(d,'value') for d in defs if get(d,'key')=='DTPad'+str(i))
 setv(comp,'actions',[])
 widgets=get(comp,'componentsData')
 widgets[:]=[c for c in widgets if get(get(c,'componentData'),'type')!='Slider']
 for c in get(comp,'componentsData'):
  cd=get(c,'componentData')
  if get(cd,'type')!='Button':continue
  data=get(cd,'data')
  if i==3:setv(get(c,'bounds'),'acceptsHWFocus','Yes')
  setv(data,'handleName',('ResetOnGestureEnd/' if i>=1 else '')+'Parameter '+str(68+i))
  setv(data,'gestureBehaviour','Hold')
  setv(data,'numButtonsInGroup',1 if i==1 else 2)
 for c in get(comp,'componentsData'):
  cd=get(c,'componentData')
  if get(cd,'type')=='Label':
   setv(c,'handle remapping',g.obj(('version',1),('map',[g.obj(('key','Data'),('value',ns['handle']('Root'+str(i+1))))])))
# Probe parent Mouse Down with child continuous gesture tracking.
for i in (1,3):
 comp=next(get(d,'value') for d in defs if get(d,'key')=='DTPad'+str(i))
 setv(comp,'actions',[g.action('Mouse Down','Toggle Switch','Parameter '+str(68+i),'')])
 widgets=get(comp,'componentsData')
 widgets[:]=[c for c in widgets if get(get(c,'componentData'),'type')!='Button']
 handle='ResetOnGestureEnd/Parameter '+str(68+i)
 if i==1:
  data=g.obj(('version',4),('knobType','RotatingImage'),('foregroundImage','button-hit.png'),('backgroundImage',''),('imageScale',1.0),('dragOrientation','Vertical'),('handleName',handle))
  widget='Knob'
 else:
  data=g.obj(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','ff6c502e'),('valueColour','55b06f37'),('isBipolar',False),('handleName',handle))
  widget='Slider'
 widgets.append(g.entry('gesture probe',widget,data,'0 0 195 48','Yes','Show','Always'))
(out/'TUI.json').write_text(g.dump_json(tui)+'\n')
print('DIAGNOSTIC pads: raw Hold radio, parent-toggle + reset Knob, reset Hold radio, parent-toggle + reset Slider. Captions show OFF count:gate duration ms. STOP clears gates.')
