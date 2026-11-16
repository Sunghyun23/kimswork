"""Native-size UI material compiler. Generated knob source, code-native bevels and typography.
No interaction is baked into these assets; all hit surfaces remain SDK widgets.
"""
from PIL import Image,ImageDraw,ImageFilter
import math
class Material:
 def __init__(self,project,out):
  self.out=out
  self.cream=Image.open(project/'assets/background-branded-source.png').convert('RGB').crop((40,300,1200,850))
  self.body=Image.open(project/'assets/bakelite-knob-source-v1.png').convert('RGBA').crop((330,65,1200,935)).resize((54,54),Image.Resampling.LANCZOS)
 def plate(self,w,h,kind='cream'):
  name=f'material-v2-{w}-{h}-{kind}.png';p=self.out/name
  if p.exists():return name
  colors={'cream':((251,233,192),(208,180,131)),'orange':((211,100,27),(119,43,8)),'dark':((55,47,37),(25,23,19)),'red':((186,53,30),(101,23,14)),'olive':((109,108,47),(53,57,18)),'card':((248,232,198),(232,211,166))}
  top,bottom=colors[kind];im=Image.new('RGBA',(w,h));d=ImageDraw.Draw(im)
  radius=10 if h>70 else 7
  d.rounded_rectangle((1,3,w-1,h-1),radius,fill=(27,15,6,175))
  mask=Image.new('L',(w,h));ImageDraw.Draw(mask).rounded_rectangle((2,1,w-3,h-5),radius,fill=255)
  texture=self.cream.resize((w,h)).convert('RGB');pix=texture.load()
  for y in range(h):
   t=y/max(1,h-1)
   for x in range(w):
    noise=(sum(pix[x,y])/3-220)*.12
    pix[x,y]=tuple(max(0,min(255,int(top[c]*(1-t)+bottom[c]*t+noise)))for c in range(3))
  im.paste(texture,(0,0),mask);d=ImageDraw.Draw(im)
  d.rounded_rectangle((1,1,w-2,h-4),radius,outline='#6e4a2a',width=1)
  d.rounded_rectangle((3,3,w-4,h-6),max(3,radius-2),outline='#f9e6be'if kind in('cream','card')else'#bb8450',width=1)
  if kind=='card':d.rounded_rectangle((7,6,w-8,h-9),max(3,radius-3),outline='#b8996b',width=1)
  im.save(p);return name
 def knobs(self):
  size=64;strip=Image.new('RGBA',(size,size*127))
  for frame in range(127):
   im=Image.new('RGBA',(size,size));d=ImageDraw.Draw(im)
   for mark in range(11):
    a=math.radians(135+mark*27);d.line((32+math.cos(a)*29,32+math.sin(a)*29,32+math.cos(a)*31,32+math.sin(a)*31),fill='#4d311d',width=2)
   im.alpha_composite(self.body,(5,5));d=ImageDraw.Draw(im);a=math.radians(135+frame/126*270)
   start=(32+math.cos(a)*10,32+math.sin(a)*10);end=(32+math.cos(a)*22,32+math.sin(a)*22)
   d.line((start[0]+1,start[1]+1,end[0]+1,end[1]+1),fill='#332014',width=5)
   d.line((*start,*end),fill='#ffe9b5',width=4);d.ellipse((end[0]-1,end[1]-1,end[0]+1,end[1]+1),fill='#ffedc6')
   strip.alpha_composite(im,(0,frame*size))
  strip.save(self.out/'knob-v2.png')
