"""Factory bank v2: explicit soul/hip-hop synthesis recipes, no legacy patches."""
import json
from pathlib import Path
CATEGORIES=('Init','Bass','Keys','Synth','Lead','Pad')
# wave1, wave2, mix, detune, cutoff, attack, decay, sustain, release, FM, sub, lowcut
RECIPES={
'Bass':[
('Pocket Anchor',0,1,15,0,380,3,350,85,90,0,32,18),
('Maple Thump',1,0,32,0,620,2,210,48,75,0,24,20),
('Brick Bounce',3,1,22,-2,780,4,280,65,110,0,18,22),
('Molasses',1,0,42,0,320,12,480,90,160,0,38,18),
('Crate Knock',2,1,28,2,950,2,160,32,65,0,14,25),
('Soft Cone',0,0,20,0,1100,6,430,78,130,8,20,18),
('Motor Hum',3,0,36,-3,540,8,360,84,150,0,26,20),
('Felt Punch',1,2,14,0,700,2,125,24,70,0,30,24),
('Honey Slide',2,0,45,3,430,5,300,80,100,0,20,20),
('Walnut Pluck',3,1,30,0,1200,2,190,18,85,0,12,28),
('Low Canvas',0,1,50,0,490,18,600,92,190,4,28,18),
('Dust Motor',2,3,18,-4,860,4,240,58,105,0,20,22)],
'Keys':[
('Felt Circuit',0,1,22,1,3400,3,1200,12,380,24,0,85),
('Worn Tines',0,0,38,3,4800,2,1700,8,520,48,0,100),
('Honey Reeds',1,3,18,-2,2300,7,850,28,260,6,0,110),
('Kitchen Bell',0,0,42,0,6200,2,2100,0,700,70,0,140),
('Soft Hammer',1,0,30,0,2800,2,600,4,200,18,0,95),
('Velour Clav',3,1,40,2,1800,2,260,10,100,0,0,130),
('Milk Glass',0,1,50,5,4100,12,1400,18,620,38,0,120),
('Tape Celeste',0,0,25,-3,5500,4,1800,0,850,58,0,150),
('Reed Cabinet',3,0,20,0,1600,18,700,42,310,8,0,100),
('Bronze Mallet',0,1,14,0,3900,2,550,0,220,85,0,160),
('Evening Felt',1,0,45,-4,1900,14,1500,22,680,12,0,90),
('Pocket Organ',0,3,28,0,2900,9,500,90,120,0,0,110),
('Pearl Hammer',0,1,34,2,7200,2,900,3,340,32,0,135),
('Cocoa Tines',0,0,48,-2,2400,5,1900,16,500,42,0,105),
('Dust Harp',1,0,20,0,3300,2,420,0,460,15,0,145)],
'Synth':[
('Chopped Brass',2,3,34,4,1900,12,420,32,160,0,0,100),
('Mellow Block',3,1,48,-5,1250,35,650,68,320,0,0,85),
('Velvet Stab',2,2,45,7,1700,4,280,8,120,0,0,120),
('Amber Stack',2,1,30,-8,2600,80,900,76,500,0,0,110),
('Rubber Chord',3,0,28,2,1100,3,180,20,90,8,0,130),
('Hollow Print',3,3,52,4,2200,18,550,56,300,0,0,95),
('Record Bloom',1,2,38,-6,1500,140,1200,72,680,0,0,100),
('Copper Pluck',2,0,18,0,3800,2,320,0,180,12,0,140),
('Faded Poly',2,2,50,9,2000,45,750,80,450,0,0,110),
('Small Ensemble',1,3,24,3,3100,90,850,65,550,0,0,90),
('Syncopated Ink',3,2,32,-3,1350,6,230,15,110,0,0,125),
('Glass Chord',0,2,26,5,4600,16,1300,24,700,30,0,115)],
'Lead':[
('Ribbon Whistle',1,0,20,0,2400,10,600,88,180,0,0,110),
('Bent Wire',2,2,35,5,1900,4,420,78,160,0,0,130),
('Narrow Alley',3,1,28,-3,1600,3,300,70,120,0,0,150),
('Honey Horn',2,1,42,4,1300,45,650,82,250,0,0,110),
('Syrup Line',3,0,22,0,2800,8,480,86,210,5,0,120),
('Ink Flute',0,1,32,0,3600,22,900,78,320,12,0,100),
('Frayed Saw',2,2,52,-7,2200,6,380,80,170,0,0,140),
('Pocket Squeak',3,2,18,2,1100,2,220,48,100,0,0,160),
('Golden Thread',1,2,24,3,3200,15,700,90,260,0,0,115),
('Melted Neon',2,3,45,8,1800,30,550,74,290,0,0,135),
('Muted Trumpet',2,1,36,-2,950,65,800,84,310,0,0,105),
('Round Needle',0,3,12,0,4100,4,460,65,140,20,0,125)],
'Pad':[
('Draped Velvet',2,2,48,7,1300,550,1800,85,1600,0,0,140),
('Paper Halo',1,2,35,-5,2200,900,2200,78,2100,0,0,165),
('Warm Projection',2,3,42,5,950,1200,1900,88,2300,0,0,130),
('Woven Air',3,3,50,9,3200,700,2500,82,1900,0,0,200),
('Satin Room',1,1,45,3,1700,350,1400,90,1300,0,0,120),
('Faded Choir',0,2,28,-4,2400,1000,2400,76,2500,18,0,180),
('Ember Strings',2,2,52,11,2800,450,1700,92,1700,0,0,155),
('Curtain Light',1,3,32,6,1100,1400,2600,80,2800,0,0,145),
('Soft Horizon',2,1,40,-6,1800,800,2100,86,2000,0,0,150),
('Opaque Silk',3,2,38,4,700,650,1800,84,2200,0,0,135),
('Dusk Film',0,1,50,2,3400,1100,2300,72,2400,28,0,190),
('Golden Haze',2,3,44,-9,1500,500,1600,88,1800,0,0,160)]}

def build(defaults):
 init=dict(defaults);init.update(Wave1=2,Wave2=0,Mix=0,Body=0,Sub=0,FM=0,Noise=0,Tone=0,Cutoff=18000,Resonance=0,Env=0,KeyTrack=0,AmpAttack=5,AmpDecay=500,AmpSustain=100,AmpRelease=120,VoiceMode=2,Detune=0,Octave=0,Motion=0,Space=0,Chorus=0,Delay=0,Drive=0,Depth=0,Vibrato=0,Glide=0,LowCut=0,Drift=0,TremRate=4,TremDepth=0)
 bank=[dict(name='Init / Init',category='Init',values=init)]
 for family in CATEGORIES[1:]:
  for row in RECIPES[family]:
   name,a,b,mix,det,cut,att,dec,sus,rel,fm,sub,low=row
   v=dict(init);v.update(Wave1=a,Wave2=b,Mix=mix,Body=30 if family=='Bass' else 12,Sub=sub,Detune=det,Cutoff=cut,Resonance=10,Env=18,KeyTrack=35,AmpAttack=att,AmpDecay=dec,AmpSustain=sus,AmpRelease=rel,FiltAttack=max(2,att),FiltDecay=dec*.7,FiltSustain=20,FiltRelease=rel,FM=fm,Tone=28,LowCut=low,Drive=6,DriveMix=60,Expression=55,Drift=5 if family=='Bass' else 12)
   if family=='Bass':v.update(VoiceMode=1,Chorus=0,Delay=0,Space=0,Glide=45 if name=='Honey Slide' else 0,Depth=0,Env=24)
   if family=='Keys':v.update(Chorus=10,Delay=4,Feedback=12,Space=0,Env=8,TremRate=3.8,TremDepth=12 if name in ('Worn Tines','Cocoa Tines') else 0)
   if family=='Synth':v.update(Chorus=14,Delay=6,Feedback=15,Depth=4,Target=1,Rate=.35,Env=30,Pulse=35)
   if family=='Lead':v.update(VoiceMode=1,Glide=55,Chorus=0,Delay=8,Feedback=18,Depth=0,Pulse=32,Env=22)
   if family=='Pad':v.update(Chorus=24,Delay=10,Feedback=20,Space=8,Depth=5,Target=1,Rate=.16,FiltSustain=55,Env=16)
   bank.append(dict(name=family+' / '+name,category=family,values=v))
 levels=json.loads((Path(__file__).resolve().parents[1]/'presets/factory-levels.json').read_text())['trim_db']
 for patch in bank:patch['values']['OutputTrim']=levels[patch['name']]
 assert len(bank)==64
 return bank
