"""Compile approved reference lettering to runtime-sized PNGs; no firmware font fallback."""
from PIL import Image,ImageOps,ImageDraw,ImageFont,ImageFilter
class ApprovedType:
 def __init__(self,path):self.source=Image.open(path).convert('RGB')
 def cut(self,box,dark=False):
  region=self.source.crop(box);grey=ImageOps.grayscale(region)
  mask=grey.point(lambda v:max(0,min(255,int((116-v)*255/75))))if dark else region.getchannel('B').point(lambda v:max(0,min(255,int((v-40)*255/45))))
  # Fractional-pixel erosion reduces weight without changing the approved letter shapes.
  large=mask.resize((mask.width*3,mask.height*3),Image.Resampling.LANCZOS);light=large.filter(ImageFilter.MinFilter(3)).resize(mask.size,Image.Resampling.LANCZOS);mask=Image.blend(mask,light,.55)
  bb=mask.getbbox()
  if bb:mask=mask.crop(bb)
  return mask
 def fit(self,mask,size,color):
  w,h=size;ratio=min((w-4)/mask.width,(h-2)/mask.height);tw=max(1,round(mask.width*ratio));th=max(1,round(mask.height*ratio));mask=mask.resize((tw,th),Image.Resampling.LANCZOS)
  image=Image.new('RGBA',size);paint=Image.new('RGBA',(tw,th),color);image.paste(paint,((w-tw)//2,(h-th)//2),mask);return image
 def control(self,name,size=(148,24)):
  A=[(62,237),(239,414),(417,588),(589,759)];B=[(838,1005),(1009,1180),(1188,1365),(1378,1549)]
  groups=[['Drive','Compress','Punch','Release'],['Low Comp','Mid Comp','High Comp','Duck'],['Tone','High Cut','Lo-Fi','Drift'],['Input','Low Cut','Mix','Output']]
  for group,names in enumerate(groups):
   if name in names:
    i=names.index(name);x1,x2=(A if group%2==0 else B)[i];y1,y2=(383,409)if group<2 else(582,602)
    return self.fit(self.cut((x1,y1,x2,y2),True),size,'#34251b')
  raise ValueError(name)
 def heading(self,group,size=(604,28)):
  badges=[(58,347,119,378),(842,347,903,378),(58,544,119,575),(842,544,903,575)]
  words=[(143,345,583,377),(926,345,1374,378),(143,543,422,573),(926,543,1283,574)]
  out=Image.new('RGBA',size);badge=self.source.crop(badges[group]).convert('RGBA').resize((38,20),Image.Resampling.LANCZOS);m=Image.new('L',badge.size);ImageDraw.Draw(m).rounded_rectangle((0,0,37,19),4,fill=255);out.paste(badge,(2,4),m)
  mask=self.cut(words[group],True);ratio=min(18/mask.height,530/mask.width);tw=round(mask.width*ratio);th=round(mask.height*ratio);mask=mask.resize((tw,th),Image.Resampling.LANCZOS);ink=Image.new('RGBA',(tw,th),'#34251b');out.paste(ink,(53,(size[1]-th)//2),mask);return out
 def performance(self,index,size=(142,42),color='#f0d5a6'):
  centers=[110,306,502,696,898,1092,1288,1482];cx=centers[index]
  mask=self.cut((cx-86,811 if index in(0,3,4)else 822,cx+86,865));height=40 if index in(0,3,4)else 24;glyph=self.fit(mask,(size[0],height),color);out=Image.new('RGBA',size);out.paste(glyph,(0,(size[1]-height)//2),glyph);return out
 def latch(self,size=(94,28),color='#f0d5a6'):
  # New word absent from the reference; rasterize a heavy serif instead of firmware fallback.
  im=Image.new('RGBA',size);draw=ImageDraw.Draw(im);font=ImageFont.truetype('/System/Library/Fonts/Supplemental/Georgia.ttf',21);draw.text((size[0]/2,size[1]/2),'LATCH',font=font,anchor='mm',fill=color);return im

 def monitor(self,text,size,color,maxheight):
  boxes={'WAVE':(40,147,135,171),'INPUT':(227,147,287,170),'OUTPUT':(373,147,452,171),'LOW':(1273,154,1320,176),'MID':(1343,154,1391,176),'HIGH':(1416,154,1470,176),'DUCK':(1496,154,1554,176)}
  mask=self.cut(boxes[text]);w,h=size;glyph=self.fit(mask,(w,min(h,maxheight)),color);out=Image.new('RGBA',size);out.paste(glyph,(0,(h-glyph.height)//2),glyph);return out
