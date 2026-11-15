"""Factory bank v3: warm subtractive voicing; stable names/order, no new engine mode."""
import json
from pathlib import Path
CATEGORIES=('Init','Bass','Keys','Synth','Lead','Pad')
# Per sound: name, wave1, wave2, mix, detune, cutoff, amp A/D/S/R, FM, sub, lowcut, body, resonance, filter envelope, pulse width.
# Each row is voiced explicitly. Existing SVF/Tape algorithms and parameter identities are unchanged.
RECIPES={
'Bass':[
('Pocket Anchor',1,2,28,0,310,3,320,82,100,0,30,18,46,19,29,50),
('Maple Thump',2,1,34,1,460,2,200,44,80,0,24,20,40,23,36,50),
('Brick Bounce',3,2,35,-1,620,4,260,62,110,0,18,22,44,27,33,42),
('Molasses',1,3,32,0,250,10,520,91,170,0,36,18,50,17,21,46),
('Crate Knock',2,3,38,2,740,2,170,28,70,0,16,25,39,31,42,38),
('Soft Cone',1,0,30,0,720,6,400,78,130,0,22,18,42,15,18,50),
('Motor Hum',3,1,40,-1,420,7,370,83,150,0,28,20,47,22,27,48),
('Felt Punch',2,1,24,0,510,2,140,22,75,0,28,24,45,25,45,50),
('Honey Slide',2,1,40,1,350,5,310,81,110,0,22,20,43,21,31,50),
('Walnut Pluck',3,2,36,0,830,2,185,16,90,0,15,28,37,32,39,34),
('Low Canvas',1,2,46,0,390,16,620,92,200,0,32,18,48,16,23,50),
('Dust Motor',2,3,32,-2,650,4,250,57,115,0,22,22,42,28,35,44)],
'Keys':[
('Felt Circuit',1,2,26,2,2300,4,1050,12,340,0,0,85,26,20,25,50),
('Worn Tines',0,1,36,2,3100,3,1500,9,480,12,0,100,22,16,18,50),
('Honey Reeds',3,1,32,-2,1700,8,800,30,280,0,0,110,30,24,27,43),
('Kitchen Bell',0,1,42,1,3900,3,1800,0,640,18,0,140,18,19,14,50),
('Soft Hammer',1,2,32,1,1900,3,540,5,210,0,0,95,29,23,32,50),
('Velour Clav',3,2,38,2,1300,2,245,9,110,0,0,130,31,30,40,30),
('Milk Glass',1,0,44,3,2800,12,1250,18,550,4,0,120,24,18,21,50),
('Tape Celeste',0,1,30,-2,3500,5,1650,0,760,14,0,150,20,17,16,50),
('Reed Cabinet',3,1,28,1,1150,17,680,44,320,0,0,100,34,25,23,46),
('Bronze Mallet',0,1,24,0,2600,2,480,0,230,16,0,160,23,27,34,50),
('Evening Felt',1,2,38,-2,1350,15,1350,24,620,0,0,90,32,19,22,50),
('Pocket Organ',3,1,42,0,2100,8,480,91,130,0,0,110,28,21,12,50),
('Pearl Hammer',1,0,32,2,4100,3,820,4,320,6,0,135,22,22,28,50),
('Cocoa Tines',0,1,40,-2,1700,6,1700,17,480,10,0,105,27,18,19,50),
('Dust Harp',1,3,28,1,2200,2,390,0,420,0,0,145,25,26,37,38)],
'Synth':[
('Chopped Brass',2,3,40,3,1450,16,400,34,175,0,0,100,34,25,36,43),
('Mellow Block',3,1,44,-3,900,38,680,70,350,0,0,85,39,20,24,48),
('Velvet Stab',2,2,47,4,1250,4,260,8,125,0,0,120,33,29,44,50),
('Amber Stack',2,1,38,-4,1900,90,920,77,520,0,0,110,35,22,27,50),
('Rubber Chord',3,2,34,2,780,3,175,19,100,0,0,130,37,33,45,36),
('Hollow Print',3,3,48,3,1600,22,580,57,320,0,0,95,32,27,29,35),
('Record Bloom',1,2,42,-3,1100,155,1250,74,720,0,0,100,36,18,25,50),
('Copper Pluck',2,1,28,1,2500,2,300,0,190,0,0,140,30,28,40,50),
('Faded Poly',2,2,49,5,1450,50,780,81,480,0,0,110,34,21,25,50),
('Small Ensemble',1,3,36,3,2200,100,900,67,580,0,0,90,31,19,23,44),
('Syncopated Ink',3,2,38,-2,950,6,225,14,120,0,0,125,35,31,42,32),
('Glass Chord',1,2,32,3,3000,20,1200,25,650,0,0,115,27,23,28,50)],
'Lead':[
('Ribbon Whistle',1,3,30,1,1750,11,620,89,190,0,0,110,30,23,24,46),
('Bent Wire',2,2,43,4,1400,4,430,79,175,0,0,130,34,28,34,50),
('Narrow Alley',3,2,35,-2,1150,3,310,71,135,0,0,150,32,32,39,37),
('Honey Horn',2,1,40,3,950,48,680,84,270,0,0,110,38,24,32,50),
('Syrup Line',3,1,34,0,2000,8,500,88,230,0,0,120,35,27,29,42),
('Ink Flute',1,0,36,1,2500,24,920,80,340,0,0,100,27,19,19,50),
('Frayed Saw',2,2,49,-5,1600,6,400,82,190,0,0,140,36,30,36,50),
('Pocket Squeak',3,2,28,2,760,2,230,49,110,0,0,160,30,38,47,29),
('Golden Thread',1,2,34,2,2250,17,730,91,280,0,0,115,31,21,26,50),
('Melted Neon',2,3,44,5,1250,32,580,76,310,0,0,135,37,29,33,41),
('Muted Trumpet',2,1,38,-1,680,70,850,86,340,0,0,105,40,25,31,50),
('Round Needle',1,3,26,1,2800,4,480,67,155,0,0,125,29,26,30,40)],
'Pad':[
('Draped Velvet',2,2,48,5,950,600,1850,86,1700,0,0,140,31,21,20,50),
('Paper Halo',1,2,40,-4,1600,950,2250,80,2200,0,0,165,28,18,18,50),
('Warm Projection',2,3,44,4,720,1250,2000,89,2400,0,0,130,36,24,23,46),
('Woven Air',3,3,49,6,2300,750,2550,83,2000,0,0,200,25,17,16,40),
('Satin Room',1,2,40,3,1200,380,1500,91,1400,0,0,120,33,19,18,50),
('Faded Choir',1,3,36,-3,1750,1050,2450,78,2600,0,0,180,29,22,21,47),
('Ember Strings',2,2,50,7,2050,500,1800,93,1800,0,0,155,30,20,22,50),
('Curtain Light',1,3,38,4,800,1450,2650,82,2900,0,0,145,35,23,24,43),
('Soft Horizon',2,1,42,-4,1250,850,2200,87,2100,0,0,150,32,18,19,50),
('Opaque Silk',3,2,42,3,510,700,1900,86,2300,0,0,135,38,25,25,45),
('Dusk Film',1,2,43,2,2450,1150,2350,75,2500,0,0,190,26,17,17,50),
('Golden Haze',2,3,46,-6,1050,550,1700,89,1900,0,0,160,34,22,22,41)]}

def build(defaults):
 init=dict(defaults);init.update(Wave1=2,Wave2=0,Mix=0,Body=0,Sub=0,FM=0,Noise=0,Tone=0,Cutoff=18000,Resonance=0,Env=0,KeyTrack=0,AmpAttack=5,AmpDecay=500,AmpSustain=100,AmpRelease=120,VoiceMode=2,Detune=0,Octave=0,Motion=0,Space=0,Chorus=0,Delay=0,Drive=0,Depth=0,Vibrato=0,Glide=0,LowCut=0,Drift=0,TremRate=4,TremDepth=0)
 bank=[dict(name='Init / Init',category='Init',values=init)]
 for family in CATEGORIES[1:]:
  for row in RECIPES[family]:
   name,a,b,mix,det,cut,att,dec,sus,rel,fm,sub,low,body,res,env,pulse=row
   v=dict(init);v.update(Wave1=a,Wave2=b,Mix=mix,Body=body,Sub=sub,Detune=det,Cutoff=cut,Resonance=res,Env=env,KeyTrack=35,AmpAttack=att,AmpDecay=dec,AmpSustain=sus,AmpRelease=rel,FiltAttack=max(2,att),FiltDecay=dec*.65,FiltSustain=16,FiltRelease=rel,FM=fm,Tone=42,LowCut=low,Drive=12,DriveMix=75,Pulse=pulse,Expression=55,Drift=2 if family=='Bass' else 4)
   if family=='Bass':v.update(VoiceMode=1,Chorus=0,Delay=0,Space=0,Glide=65 if name=='Honey Slide' else 0,Depth=0,FiltSustain=8,Drive=14,Tone=48,KeyTrack=25)
   if family=='Keys':v.update(Chorus=4,Delay=2,Feedback=10,Space=0,TremRate=3.8,TremDepth=8 if name in ('Worn Tines','Cocoa Tines') else 0,Tone=38,Drive=8,KeyTrack=40)
   if family=='Synth':v.update(Chorus=7,Delay=3,Feedback=12,Depth=2,Target=1,Rate=.28,FiltSustain=18,Tone=44)
   if family=='Lead':v.update(VoiceMode=1,Glide=65 if name in ('Syrup Line','Melted Neon') else 45,Chorus=0,Delay=4,Feedback=14,Depth=0,FiltSustain=22,Tone=46,Drive=13)
   if family=='Pad':v.update(Chorus=12,Delay=6,Feedback=17,Space=4,Depth=3,Target=1,Rate=.12,FiltSustain=48,Tone=38,Drive=8,Drift=5)
   bank.append(dict(name=family+' / '+name,category=family,values=v))
 levels=json.loads((Path(__file__).resolve().parents[1]/'presets/factory-levels.json').read_text())['trim_db']
 for patch in bank:patch['values']['OutputTrim']=levels[patch['name']]
 assert len(bank)==64
 return bank
