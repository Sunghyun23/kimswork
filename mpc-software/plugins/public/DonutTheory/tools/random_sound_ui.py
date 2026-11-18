"""Compact global sound generator; no extra Q-Link or moved controls."""
from PIL import Image,ImageDraw
import copy

def install(g,tui,params,out):
 O=g.obj
 def get(o,k):return next(v for a,v in o if a==k)
 pd=get(tui,'pageData');defs=get(get(pd,'componentDefinitions'),'localComponentDefinitions')
 root=get(get(get(pd,'tabs')[0],'componentDefinition'),'componentsData')
 index=next(p.index for p in params if p.name=='Random Sound');command=f'Parameter {index}';press='Local/2/dtRandomPress'
 for active in [False,True]:
  im=Image.new('RGBA',(48,52));d=ImageDraw.Draw(im)
  d.rounded_rectangle((1,1,46,50),4,fill='#b06f37'if active else'#d8c18e',outline='#72502c',width=2)
  d.rounded_rectangle((14,5,34,25),3,fill='#f3dfb4',outline='#39291c',width=1)
  for x,y in [(18,9),(30,9),(24,15),(18,21),(30,21)]:d.ellipse((x-1,y-1,x+1,y+1),fill='#39291c')
  im.save(out/('random-on.png'if active else'random-off.png'))
 art=g.entry('random art','Button',g.button_data('random-on.png','random-off.png',1,1),'0 0 48 52','No','Show','Always',remap=[('Data',press)])
 caption=g.entry('RND','Label',g.label_data('Text','ff352d21',19,'horizontallyCentred verticallyCentred','RND'),'0 28 48 22','No','Show','Always',remap=[('Data','Parameter 0')])
 data=O(('version',2),('sliderType','ColourBar'),('direction','Horizontal'),('backgroundColour','00000000'),('outlineColour','00000000'),('valueColour','00000000'),('isBipolar',False),('handleName','ResetOnGestureEnd/'+press))
 hit=g.entry('random press','Slider',data,'0 0 48 52','No','Show','Always')
 definition=g.component_definition([g.action('Mouse Down','Toggle Switch',command,''),g.action('Mouse Down','Toggle Switch',press,'')],[art,caption,hit],'00000000')
 defs.append(O(('key','DTRandomSound'),('value',definition)))
 root.append(g.entry('Random sound','DTRandomSound',O(('version',1),('handleName','Data')),'910 26 48 52','No','Show','Always',remap=[('Data','Parameter 0')]))

 # Choice replacement strings can bypass formatValue; show custom name explicitly.
 by={get(d,'key'):get(d,'value')for d in defs}
 for name in ['DTMenu-choice-287','sliderWithFocusMenu-choice-287']:
  widgets=get(by[name],'componentsData')
  for i,w in list(enumerate(widgets)):
   cd=get(w,'componentData');data=get(cd,'data')
   if get(cd,'type')!='Label' or dict(data).get('type')!='Value':continue
   bounds=get(w,'bounds')
   def put(o,k,v):
    for j,(a,_)in enumerate(o):
     if a==k:o[j]=(k,v);return
    o.append((k,v))
   put(bounds,'version',2);put(bounds,'showWhenDataModelInvalid','Hide');put(bounds,'additionalInvalidatingHandles',['IndexedEnabling/0/2/Parameter 289'])
   custom=copy.deepcopy(w);cc=get(custom,'componentData');put(cc,'name','Random sound name');value=g.label_data('Text','ff352d21',24,'horizontallyCentred verticallyCentred','Random Sound');put(value,'textStyle',copy.deepcopy(get(data,'textStyle')));put(cc,'data',value)
   put(get(custom,'bounds'),'additionalInvalidatingHandles',['IndexedEnabling/1/2/Parameter 289']);put(custom,'handle remapping',O(('version',1),('map',[])))
   widgets.insert(i+1,custom)

 # Explicit reload fires even when selecting the already-active factory row.
 for i in range(66):
  row=by['DTSideRow287-'+str(i)]
  get(row,'actions').append(g.action('Mouse Down','Toggle Switch','Parameter 290',''))
