"""Runtime material atlas extracted from the user's approved vintage MAIN.
Only live content is reconstructed; the approved illustration/trim stays intact.
"""
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont,ImageChops,ImageStat
import math,sys,random
REF=Path(sys.argv[1]).with_name('approved-vintage-main.png');OUT=Path(sys.argv[2]);OUT.mkdir(parents=True,exist_ok=True)
im=Image.open(REF).convert('RGB')
def save(a,name):a.save(OUT/name);return name
def tile(patch,size):
 a=Image.new('RGB',size)
 for y in range(0,size[1],patch.height):
  for x in range(0,size[0],patch.width):a.paste(patch,(x,y))
 return a
paper=im.crop((225,275,250,318))
# Continuous reproducible finish avoids obvious repeated seams from narrow gutters.
rng=random.Random(0x46434c42);green=Image.new('RGB',(1280,628));base=ImageStat.Stat(im.crop((891,240,901,400))).mean
pixels=[]
for _ in range(1280*628):
 n=rng.gauss(0,4);pixels.append(tuple(max(0,min(255,round(c+n)))for c in base))
green.putdata(pixels)
body=tile(green,(1280,628));header=im.crop((0,0,1790,111)).resize((1280,80),Image.Resampling.LANCZOS)
body.paste(header,(0,0));save(header,'brand.png');save(body,'background.png')
font_path='/System/Library/Fonts/Supplemental/Georgia.ttf'
font_regular='/System/Library/Fonts/Supplemental/Georgia.ttf'
def font(size,bold=True):return ImageFont.truetype(font_path,size+3)
# Copy source panel edges/header precisely; erase all fixed live control values.
for key,box in [('A',(20,190,885,414)),('B',(905,190,1771,414)),('C',(20,425,885,646)),('D',(905,425,1771,646))]:
 p=im.crop(box).resize((615,153),Image.Resampling.LANCZOS)
 p.paste(tile(paper,(599,117)),(8,27))
 if key=='C':p.paste(tile(im.crop((259,435,280,457)),(252,22)),(358,4))
 p=p.convert('RGBA');mask=Image.new('L',p.size);ImageDraw.Draw(mask).rounded_rectangle((0,0,614,152),10,fill=255);p.putalpha(mask);save(p,'panel-'+key+'.png')
for name,box,size in [('label-sound',(62,126,258,165),(152,30)),('label-chord',(175,653,379,694),(186,28))]:
 a=im.crop(box).resize(size,Image.Resampling.LANCZOS).convert('RGBA');a.putalpha(a.convert('L').point(lambda v:max(0,min(255,(v-110)*3))));save(a,name+'.png')
# Dedicated generated knob asset, not a crop from the screen concept. The face,
# highlight and contact shadow are stationary across97frames; only the pointer
# overlay rotates. Transparent native72px output has no old panel-colour fringe.
source=Image.open(REF.parents[1]/'assets/knob/bakelite-face-source.png').convert('RGBA')
bbox=source.getchannel('A').point(lambda v:255 if v>=160 else 0).getbbox();assert bbox
face=Image.new('RGBA',(72,72));asset=source.crop(bbox);asset.thumbnail((70,70),Image.Resampling.LANCZOS);face.alpha_composite(asset,((72-asset.width)//2,(72-asset.height)//2))
strip=Image.new('RGBA',(72,72*97))
for n in range(97):
 frame=face.copy();pointer=Image.new('RGBA',(288,288));d=ImageDraw.Draw(pointer);theta=math.radians(-225+n*270/96)
 points=[(144+math.cos(theta)*r*4,144+math.sin(theta)*r*4)for r in (11,27)]
 d.line(points,fill='#9f793d',width=13);d.line(points,fill='#f8ecc9',width=7)
 frame.alpha_composite(pointer.resize((72,72),Image.Resampling.LANCZOS));strip.alpha_composite(frame,(0,n*72))
save(strip,'knob.png');save(face,'knob-face.png')

def nine(a,size):
 w,h=size;ow,oh=a.size;edge=min(8,w//3,h//3,ow//3,oh//3);out=Image.new('RGB',size)
 for left,right,dl,dr in [(0,edge,0,edge),(edge,ow-edge,edge,w-edge),(ow-edge,ow,w-edge,w)]:
  for top,bottom,dt,db in [(0,edge,0,edge),(edge,oh-edge,edge,h-edge),(oh-edge,oh,h-edge,h)]:
   out.paste(a.crop((left,top,right,bottom)).resize((dr-dl,db-dt),Image.Resampling.LANCZOS),(dl,dt))
 return out
# Use an actual source button and clear its caption, preserving all four edges.
button=im.crop((296,121,358,175));button.paste(tile(im.crop((307,131,318,141)),(42,34)),(10,10))
orange=im.crop((940,122,1067,174));orange.paste(tile(im.crop((951,134,965,147)),(107,32)),(10,10))
readout=im.crop((72,370,218,400));readout.paste(tile(im.crop((78,375,87,391)),(140,24)),(3,3))
def plate(w,h,active=False,kind='ivory'):
 src=orange if active else readout if kind=='sage' else button
 a=nine(src,(w,h)).convert('RGBA');mask=Image.new('L',(w,h));ImageDraw.Draw(mask).rounded_rectangle((0,0,w-1,h-1),min(6,h//3),fill=255);a.putalpha(mask);return a
for name,w,h in [('lcd',136,24),('lcd-large',220,30)]:save(plate(w,h,kind='sage'),name+'.png')
for locked in (0,1):
 a=Image.new('RGBA',(44,44));d=ImageDraw.Draw(a);ink='#9c4c1b'if locked else'#302a18'
 if locked:
  d.arc((15,5,29,23),180,360,fill=ink,width=2);d.line((15,13,15,20),fill=ink,width=2);d.line((29,13,29,20),fill=ink,width=2);d.rounded_rectangle((12,16,31,29),3,fill=ink);d.ellipse((21,20,23,22),fill='#fff0c5');d.line((22,22,22,25),fill='#fff0c5',width=2)
 else:d.arc((20,3,34,21),180,360,fill=ink,width=2);d.line((20,11,20,19),fill=ink,width=2);d.rounded_rectangle((12,16,31,29),3,outline=ink,width=2)
 save(a,'lock-'+str(locked)+'.png')
# Approved printed dial legends at their actual display size.
legends={};names=['TONE','BODY','REED','BARK','ATTACK','DECAY','SUSTAIN','RELEASE','RATE','DEPTH','DRIFT','WIDTH','DRIVE','CHORUS','DELAY','ROOM']
centres=[150,359,568,777,1032,1242,1452,1660]*2
for n,(name,cx)in enumerate(zip(names,centres)):
 y=340 if n<8 else 568
 a=im.crop((cx-90,y,cx+90,y+28)).resize((129,20),Image.Resampling.LANCZOS).convert('RGBA');alpha=a.convert('L').point(lambda v:max(0,min(255,(140-v)*4)));a.putalpha(alpha);filename='caption-'+name.lower()+'.png';save(a,filename);legends[name]=filename
print('Vintage approval atlas: exact header/rails/material/button/knob/printed legends; no baked live values')
