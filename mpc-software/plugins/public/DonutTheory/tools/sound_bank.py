"""Factory bank v4: voiced for the nonlinear four-pole ladder engine; stable names/order."""
import json
from pathlib import Path
CATEGORIES=('Init','Bass','Keys','Synth','Lead','Pad')
# Per sound: name, wave1, wave2, mix, detune, cutoff, amp A/D/S/R, FM, sub, lowcut, body, resonance, filter envelope, pulse width.
# Each row is voiced for the ladder engine. Parameter identities/order are unchanged.
RECIPES={
'Bass':[
('Pocket Anchor', 1, 2, 28, 0, 460, 3, 320, 82, 100, 0, 30, 18, 46, 25, 23, 50),
('Maple Thump', 2, 1, 34, 1, 680, 2, 200, 44, 80, 0, 24, 20, 40, 32, 29, 50),
('Brick Bounce', 3, 2, 35, -1, 920, 4, 260, 62, 110, 0, 18, 22, 44, 36, 26, 42),
('Molasses', 1, 3, 32, 0, 380, 10, 520, 91, 170, 0, 36, 18, 50, 23, 17, 46),
('Crate Knock', 2, 3, 38, 2, 1120, 2, 170, 28, 70, 0, 16, 25, 39, 40, 34, 38),
('Soft Cone', 1, 0, 30, 0, 980, 6, 400, 78, 130, 0, 22, 18, 42, 21, 14, 50),
('Motor Hum', 3, 1, 40, -1, 640, 7, 370, 83, 150, 0, 28, 20, 47, 29, 22, 48),
('Felt Punch', 2, 1, 24, 0, 780, 2, 140, 22, 75, 0, 28, 24, 45, 34, 36, 50),
('Honey Slide', 2, 1, 40, 1, 530, 5, 310, 81, 110, 0, 22, 20, 43, 28, 25, 50),
('Walnut Pluck', 3, 2, 36, 0, 1280, 2, 185, 16, 90, 0, 15, 28, 37, 43, 31, 34),
('Low Canvas', 1, 2, 46, 0, 580, 16, 620, 92, 200, 0, 32, 18, 48, 22, 18, 50),
('Dust Motor', 2, 3, 32, -2, 980, 4, 250, 57, 115, 0, 22, 22, 42, 37, 28, 44)],
'Keys':[
('Felt Circuit', 1, 2, 26, 2, 3400, 4, 1050, 12, 340, 0, 0, 85, 26, 24, 20, 50),
('Worn Tines', 0, 1, 36, 2, 4700, 3, 1500, 9, 480, 12, 0, 100, 22, 18, 14, 50),
('Honey Reeds', 3, 1, 32, -2, 2600, 8, 800, 30, 280, 0, 0, 110, 30, 30, 22, 43),
('Kitchen Bell', 0, 1, 42, 1, 5900, 3, 1800, 0, 640, 18, 0, 140, 18, 21, 11, 50),
('Soft Hammer', 1, 2, 32, 1, 2850, 3, 540, 5, 210, 0, 0, 95, 29, 28, 26, 50),
('Velour Clav', 3, 2, 38, 2, 2100, 2, 245, 9, 110, 0, 0, 130, 31, 37, 32, 30),
('Milk Glass', 1, 0, 44, 3, 4200, 12, 1250, 18, 550, 4, 0, 120, 24, 22, 17, 50),
('Tape Celeste', 0, 1, 30, -2, 5300, 5, 1650, 0, 760, 14, 0, 150, 20, 19, 13, 50),
('Reed Cabinet', 3, 1, 28, 1, 1800, 17, 680, 44, 320, 0, 0, 100, 34, 31, 18, 46),
('Bronze Mallet', 0, 1, 24, 0, 4100, 2, 480, 0, 230, 16, 0, 160, 23, 33, 27, 50),
('Evening Felt', 1, 2, 38, -2, 2050, 15, 1350, 24, 620, 0, 0, 90, 32, 23, 18, 50),
('Pocket Organ', 3, 1, 42, 0, 3200, 8, 480, 91, 130, 0, 0, 110, 28, 26, 10, 50),
('Pearl Hammer', 1, 0, 32, 2, 6100, 3, 820, 4, 320, 6, 0, 135, 22, 27, 22, 50),
('Cocoa Tines', 0, 1, 40, -2, 2600, 6, 1700, 17, 480, 10, 0, 105, 27, 21, 15, 50),
('Dust Harp', 1, 3, 28, 1, 3500, 2, 390, 0, 420, 0, 0, 145, 25, 32, 30, 38)],
'Synth':[
('Chopped Brass', 2, 3, 40, 3, 2300, 16, 400, 34, 175, 0, 0, 100, 34, 32, 29, 43),
('Mellow Block', 3, 1, 44, -3, 1450, 38, 680, 70, 350, 0, 0, 85, 39, 27, 19, 48),
('Velvet Stab', 2, 2, 47, 4, 2050, 4, 260, 8, 125, 0, 0, 120, 33, 38, 35, 50),
('Amber Stack', 2, 1, 38, -4, 2900, 90, 920, 77, 520, 0, 0, 110, 35, 29, 22, 50),
('Rubber Chord', 3, 2, 34, 2, 1280, 3, 175, 19, 100, 0, 0, 130, 37, 43, 36, 36),
('Hollow Print', 3, 3, 48, 3, 2500, 22, 580, 57, 320, 0, 0, 95, 32, 35, 23, 35),
('Record Bloom', 1, 2, 42, -3, 1750, 155, 1250, 74, 720, 0, 0, 100, 36, 24, 20, 50),
('Copper Pluck', 2, 1, 28, 1, 3900, 2, 300, 0, 190, 0, 0, 140, 30, 36, 32, 50),
('Faded Poly', 2, 2, 49, 5, 2250, 50, 780, 81, 480, 0, 0, 110, 34, 28, 20, 50),
('Small Ensemble', 1, 3, 36, 3, 3350, 100, 900, 67, 580, 0, 0, 90, 31, 25, 18, 44),
('Syncopated Ink', 3, 2, 38, -2, 1550, 6, 225, 14, 120, 0, 0, 125, 35, 41, 34, 32),
('Glass Chord', 1, 2, 32, 3, 4600, 20, 1200, 25, 650, 0, 0, 115, 27, 30, 22, 50)],
'Lead':[
('Ribbon Whistle', 1, 3, 30, 1, 2600, 11, 620, 89, 190, 0, 0, 110, 30, 31, 19, 46),
('Bent Wire', 2, 2, 43, 4, 2250, 4, 430, 79, 175, 0, 0, 130, 34, 38, 27, 50),
('Narrow Alley', 3, 2, 35, -2, 1850, 3, 310, 71, 135, 0, 0, 150, 32, 43, 31, 37),
('Honey Horn', 2, 1, 40, 3, 1500, 48, 680, 84, 270, 0, 0, 110, 38, 33, 26, 50),
('Syrup Line', 3, 1, 34, 0, 3000, 8, 500, 88, 230, 0, 0, 120, 35, 36, 23, 42),
('Ink Flute', 1, 0, 36, 1, 3700, 24, 920, 80, 340, 0, 0, 100, 27, 26, 15, 50),
('Frayed Saw', 2, 2, 49, -5, 2550, 6, 400, 82, 190, 0, 0, 140, 36, 41, 29, 50),
('Pocket Squeak', 3, 2, 28, 2, 1250, 2, 230, 49, 110, 0, 0, 160, 30, 51, 38, 29),
('Golden Thread', 1, 2, 34, 2, 3400, 17, 730, 91, 280, 0, 0, 115, 31, 28, 21, 50),
('Melted Neon', 2, 3, 44, 5, 2000, 32, 580, 76, 310, 0, 0, 135, 37, 39, 26, 41),
('Muted Trumpet', 2, 1, 38, -1, 1100, 70, 850, 86, 340, 0, 0, 105, 40, 34, 25, 50),
('Round Needle', 1, 3, 26, 1, 4200, 4, 480, 67, 155, 0, 0, 125, 29, 35, 24, 40)],
'Pad':[
('Draped Velvet', 2, 2, 48, 5, 1600, 600, 1850, 86, 1700, 0, 0, 140, 31, 25, 16, 50),
('Paper Halo', 1, 2, 40, -4, 2450, 950, 2250, 80, 2200, 0, 0, 165, 28, 22, 14, 50),
('Warm Projection', 2, 3, 44, 4, 1250, 1250, 2000, 89, 2400, 0, 0, 130, 36, 29, 18, 46),
('Woven Air', 3, 3, 49, 6, 3500, 750, 2550, 83, 2000, 0, 0, 200, 25, 21, 13, 40),
('Satin Room', 1, 2, 40, 3, 1850, 380, 1500, 91, 1400, 0, 0, 120, 33, 24, 14, 50),
('Faded Choir', 1, 3, 36, -3, 2700, 1050, 2450, 78, 2600, 0, 0, 180, 29, 27, 17, 47),
('Ember Strings', 2, 2, 50, 7, 3100, 500, 1800, 93, 1800, 0, 0, 155, 30, 25, 18, 50),
('Curtain Light', 1, 3, 38, 4, 1400, 1450, 2650, 82, 2900, 0, 0, 145, 35, 28, 19, 43),
('Soft Horizon', 2, 1, 42, -4, 1950, 850, 2200, 87, 2100, 0, 0, 150, 32, 22, 15, 50),
('Opaque Silk', 3, 2, 42, 3, 920, 700, 1900, 86, 2300, 0, 0, 135, 38, 31, 20, 45),
('Dusk Film', 1, 2, 43, 2, 3650, 1150, 2350, 75, 2500, 0, 0, 190, 26, 20, 14, 50),
('Golden Haze', 2, 3, 46, -6, 1750, 550, 1700, 89, 1900, 0, 0, 160, 34, 26, 18, 41)]}

def build(defaults):
 init=dict(defaults);init.update(Wave1=2,Wave2=0,Mix=0,Body=0,Sub=0,FM=0,Noise=0,Tone=0,Cutoff=18000,Resonance=0,Env=0,KeyTrack=0,AmpAttack=5,AmpDecay=500,AmpSustain=100,AmpRelease=120,VoiceMode=2,Detune=0,Octave=0,Motion=0,Space=0,Chorus=0,Delay=0,Drive=0,Depth=0,Vibrato=0,Glide=0,LowCut=0,Drift=0,TremRate=4,TremDepth=0)
 bank=[dict(name='Init / Init',category='Init',values=init)]
 for family in CATEGORIES[1:]:
  for row in RECIPES[family]:
   name,a,b,mix,det,cut,att,dec,sus,rel,fm,sub,low,body,res,env,pulse=row
   v=dict(init);v.update(Wave1=a,Wave2=b,Mix=mix,Body=body,Sub=sub,Detune=det,Cutoff=cut,Resonance=res,Env=env,KeyTrack=35,AmpAttack=att,AmpDecay=dec,AmpSustain=sus,AmpRelease=rel,FiltAttack=max(2,att),FiltDecay=dec*.65,FiltSustain=16,FiltRelease=rel,FM=fm,Tone=30,LowCut=low,Drive=7,DriveMix=65,Pulse=pulse,Expression=55,Drift=2 if family=='Bass' else 4)
   if family=='Bass':v.update(VoiceMode=1,Chorus=0,Delay=0,Space=0,Glide=65 if name=='Honey Slide' else 0,Depth=0,FiltSustain=8,Drive=8,Tone=38,KeyTrack=25)
   if family=='Keys':v.update(Chorus=4,Delay=2,Feedback=10,Space=0,TremRate=3.8,TremDepth=8 if name in ('Worn Tines','Cocoa Tines') else 0,Tone=27,Drive=5,KeyTrack=40)
   if family=='Synth':v.update(Chorus=7,Delay=3,Feedback=12,Depth=2,Target=1,Rate=.28,FiltSustain=22,Tone=34)
   if family=='Lead':v.update(VoiceMode=1,Glide=65 if name in ('Syrup Line','Melted Neon') else 45,Chorus=0,Delay=4,Feedback=14,Depth=0,FiltSustain=28,Tone=36,Drive=8)
   if family=='Pad':v.update(Chorus=12,Delay=6,Feedback=17,Space=4,Depth=3,Target=1,Rate=.12,FiltSustain=48,Tone=27,Drive=5,Drift=5)
   bank.append(dict(name=family+' / '+name,category=family,values=v))
 levels=json.loads((Path(__file__).resolve().parents[1]/'presets/factory-levels.json').read_text())['trim_db']
 for patch in bank:patch['values']['OutputTrim']=levels[patch['name']]
 assert len(bank)==64
 return bank
