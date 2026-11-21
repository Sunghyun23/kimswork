from pathlib import Path
import json
P=Path(__file__).resolve().parents[1];ps=[]
def f(k,n,lo,hi,d,u='',dec=0,auto=True):ps.append(dict(key=k,name=n,min=lo,max=hi,default=d,unit=u,decimals=dec,auto=auto))
def c(k,n,choices,d=0,auto=True):ps.append(dict(key=k,name=n,choices=choices,default=d,auto=auto))
def b(k,n,d=0,auto=True):c(k,n,['Off','On'],d,auto)
f('Master','Master',-36,0,-9,'dB',1)
for k,d in [('Tone',48),('Body',62),('Character3',28),('Character4',18)]:f(k,k,0,100,d,'%')
f('Attack','Attack',1,1500,1,'ms');f('Decay','Decay',20,5000,650,'ms');f('Sustain','Sustain',0,100,70,'%');f('Release','Release',15,6000,420,'ms')
f('Rate','Motion Rate',0,1,.5,'',3);f('Depth','Motion Depth',0,100,22,'%');f('Drift','Drift',0,100,6,'%');f('Width','Width',0,100,70,'%')
for k,d in [('Drive',0),('Chorus',0),('Delay',0),('Room',0)]:f(k,k,0,100,d,'%')
c('Model','Model',['Tine','Wurli','Upright','Felt','Clav','Muted','Thumb']);c('Family','Family',['EP','Piano','Pluck'],auto=False)
names=[['Candle Tine','Sunday Velvet','Dusty Suitcase','Afterglow','Amber Bell','Low Ceiling','Warm Current','Night Letters'],['Copper Wurli','Soft Static','Basement Waltz','Brick Dust','Wurli Bloom','Quiet Block','Evening Grain','Honey Circuit'],['Sunday Upright','Soft Floor','Loose Felt','Low Window','Paper Keys','After Hours','Dust In Light','Room Sketch'],['Felt Letters','Small Hours','Moss Chords','Warm Rain','Muted Ivory','Slow Bloom','Hush Piano','Quiet Tape'],['Dusty Clav','Copper Steps','Short Answer','Side Street','Pocket Clav','Muted Talk','Wood Circuit','Friday Bounce'],['Muted Thread','Short Velvet','Small Pick','Porch Light','Soft Sync','Dry Flowers','Night Pocket','Worn Strings'],['Thumb Notes','Little Lantern','Brass Leaf','Warm Pins','Soft Chimes','Cloud Steps','Slow Pebble','Paper Tines']]
programs=[{'name':'Init','model':0}]+[{'name':name,'model':m}for m,group in enumerate(names)for name in group]
sunday_upright=programs.index({'name':'Sunday Upright','model':2})
c('Preset','Sound Preset',['Custom']+[x['name']for x in programs],1,False)
for k,n in [('PresetPrev','Previous Sound'),('PresetNext','Next Sound')]:b(k,n,auto=False)
c('MotionMode','Motion Mode',['Tremolo','Auto Pan'],1);b('Sync','Motion Sync',1);c('Shape','Motion Shape',['Sine','Triangle','Square','S&H'])
div=['1/1','1/1 D','1/1 T','1/2','1/2 D','1/2 T','1/4','1/4 D','1/4 T','1/8','1/8 D','1/8 T','1/16','1/16 D','1/16 T','1/32','1/32 D','1/32 T']
c('DelayDivision','Delay Division',div,10);f('DelayFeedback','Delay Feedback',0,72,28,'%');f('RoomTime','Room Time',0,100,55,'%');f('RoomDamp','Room Damping',0,100,50,'%')
f('Hammer','Hammer',0,100,35,'%');f('Velocity','Velocity Sens',0,100,65,'%');f('Resonance','Resonance',0,100,32,'%');f('Noise','Mechanical',0,100,0,'%');f('Pick','Pick Position',0,100,22,'%');f('Detune','String Detune',0,12,1.5,'ct',1);f('Tuning','Tuning',430,450,440,'Hz',1);b('Pedal','Sustain Pedal',1);f('PitchRange','Bend Range',1,12,2,'st');c('VelocityCurve','Velocity Curve',['Soft','Linear','Firm'],1);f('KeyOff','Key Off',0,100,0,'%');f('MotionDelay','Motion Delay',0,3000,0,'ms');f('MotionFade','Motion Fade',0,3000,0,'ms');f('DriftRate','Drift Rate',.05,3,.24,'Hz',2);f('ChorusRate','Chorus Rate',.05,2,.31,'Hz',2);f('ChorusDepth','Chorus Depth',0,100,50,'%')
sound=[x['key']for x in ps if x['key']not in ['Master','Family','Preset','PresetPrev','PresetNext','Pedal','PitchRange','Tuning']]
# Model participates in sound-only Dice and can be locked; Sync stays On/Off as set.
for k,n in [('Roll','Roll Dice'),('Undo','Undo'),('UnlockAll','Unlock All'),('Panic','All Notes Off')]:b(k,n,auto=False)
c('InputMode','Keyboard / Pads',['Notes','Chords','Split'],2);c('Key','Key',['C','C#','D','Eb','E','F','F#','G','Ab','A','Bb','B']);c('Scale','Reference Scale',['Major','Minor']);f('Transpose','Transpose',-12,12,0,'st');f('Strum','Strum',0,160,18,'ms');c('StrumDirection','Strum Direction',['Up','Down','Alternate']);f('StrumVelocity','Strum Velocity',0,30,5,'%');b('ChordGlide','Chord Glide');f('Glide','Glide Time',0,1200,90,'ms');c('Slots','Chord Slots',['4','8','12','16']);c('Page','Slot Page',['1–4','5–8','9–12','13–16'],auto=False)
for k,n in [('PagePrev','Previous Slots'),('PageNext','Next Slots'),('SlotsAdd','Add Four Slots'),('SlotsRemove','Remove Four Slots'),('Smooth','Smooth Top Notes'),('ChordPrev','Previous Progression'),('ChordNext','Next Progression')]:b(k,n,auto=False)
progNames=['Sunday Soul','Amber Changes','Side Street','Soft Landing','Blue Hour','Quiet Turn','Warm Window','Copper Circle','Dusty Steps','Night Bloom','Little Bridge','Soft Motion','Low Lantern','Porch Chords','Last Train','Velvet Loop']
for i in range(4):c('ChordPreset'+str(i+1),'Progression '+str(i+1),progNames,auto=False)
roots=[2,7,0,9]*4;types=[2,6,3,2]*4;maps=[24,26,28,29,31,33,35,36,38,40,41,43,45,47,48,50]
for i in range(16):
 c('Root'+str(i+1),'Root '+str(i+1),['C','C#','D','Eb','E','F','F#','G','Ab','A','Bb','B'],roots[i],False)
 c('Type'+str(i+1),'Chord Type '+str(i+1),['m7','maj7','m9','maj9','7','9','13','sus2','m','Major','7b9','m6','6','m6/9','6/9','m11','m13','maj9#11','maj13','7sus4','9sus4','13sus4','7#9','7b13','7alt','m7b5','dim7','7#5','mMaj7','add9','madd9','sus4'],types[i],False)
 c('Voicing'+str(i+1),'Voicing '+str(i+1),['Root','First','Open','Second','Third','Fourth'],auto=False)
 f('Octave'+str(i+1),'Octave '+str(i+1),-2,2,0,'oct',0,False)
 f('PadNote'+str(i+1),'Pad Note '+str(i+1),-1,127,maps[i],'',0,False)
for i in range(4):
 b('Hold'+str(i+1),'Touch Chord '+str(i+1),auto=False);b('Learn'+str(i+1),'Learn Pad '+str(i+1),auto=False)
 f('ChordName'+str(i+1),'Chord Name '+str(i+1),0,1,0,auto=False)
 for j in range(4):f('Edit'+str(i*4+j),'Chord Edit '+str(i*4+j+1),0,1,0,'',4,False)
for k in ['Character3Label','Character4Label','CurrentChordPreset','PageInfo','LearnInfo']:f(k,k,0,1,0,auto=False)
for target in sound:b('Lock'+target,'Lock '+target,auto=False)
b('ChordEditing','Chord Editor',auto=False)
for i in range(16):f('MainQ'+str(i),'Main Screen '+str(i+1),0,1,0,'',4,False)
# Preserve the original238parameter declarations; append the new texture pair.
f('Hiss','Noise',0,100,0,'%',0);b('LockHiss','Lock Texture Noise',auto=False);sound.append('Hiss')

# Shared A-bank action pads: append only, never randomize these mappings.
b('CommandPads','Pad Commands',0,auto=False)
for key,label,note in [('CmdPrevNote','Previous Preset Pad',25),('CmdNextNote','Next Preset Pad',26),('CmdDiceNote','Dice Pad',24)]:f(key,label,-1,127,note,auto=False)
c('CommandLearn','Learn Action',['Off','PRESET PREV','PRESET NEXT','ROLL DICE'],auto=False)
f('CommandInfo','Pad Actions',0,1,0,auto=False)
lines=['// Generated before first release. Append IDs after release; never reorder.','enum Key { '+', '.join('k'+p['key']for p in ps)+', kParamCount };','inline void declareParameters(mpc::Plugin& p){']
for v in ps:
 if 'choices'in v:expr='mpc::choiceParam('+json.dumps(v['key'])+','+json.dumps(v['name'])+',{'+','.join(json.dumps(x)for x in v['choices'])+'},'+str(v['default'])+')'
 else:expr=('mpc::logParam('if v['key']in ('Attack','Decay','Release')else'mpc::floatParam(')+','.join(map(str,[json.dumps(v['key']),json.dumps(v['name']),v['min'],v['max'],v['default'],json.dumps(v['unit']),v['decimals']]))+')'
 lines+=['{auto v='+expr+';v.automatable='+str(v['auto']).lower()+';p.addParam(v);}']
lines+=['}','static constexpr int kMainKeys[]={kTone,kBody,kCharacter3,kHiss,kAttack,kDecay,kSustain,kRelease,kRate,kDepth,kDrift,kWidth,kDrive,kChorus,kDelay,kRoom};','static constexpr int kSoundKeys[]={'+','.join('k'+x for x in sound)+'};','static constexpr int kSoundLocks[]={'+','.join('kLock'+x for x in sound)+'};']
(P/'src/Parameters.h').write_text('\n'.join(lines)+'\n');(P/'parameters.json').write_text(json.dumps(ps,indent=2)+'\n');(P/'programs.json').write_text(json.dumps(programs,indent=2)+'\n')
h=['#pragma once','namespace felt {','static const char*const kProgramNames[]={'+','.join(json.dumps(x['name'])for x in programs)+'};','static constexpr int kProgramModels[]={'+','.join(str(x['model'])for x in programs)+'};','static constexpr int kProgramCount='+str(len(programs))+';','static constexpr int kSundayUprightProgram='+str(sunday_upright)+';','static const char*const kModelNames[]={"Tine","Wurli","Upright","Felt","Clav","Muted","Thumb"};','static const char*const kProgressionNames[]={'+','.join(json.dumps(x)for x in progNames)+'};','}'];(P/'src/FactoryNames.h').write_text('\n'.join(h)+'\n')
meta=json.loads((P/'plugin.json').read_text());meta.setdefault('version','0.1.0');meta.update(category='Piano',description='Sample-free modal electric/acoustic keys and plucked instruments with held chords, strum and sound Dice.',url='https://kimswork.app/');(P/'plugin.json').write_text(json.dumps(meta,indent=2)+'\n')
main=[next(v['name']for v in ps if v['key']==key)for key in ['Tone','Body','Character3','Hiss','Attack','Decay','Sustain','Release','Rate','Depth','Drift','Width','Drive','Chorus','Delay','Room']];pages=[{'name':name,'controls':main}for name in ['MAIN','MODEL','TOUCH','MOTION','FX','SETUP']];(P/'ui.json').write_text(json.dumps({'kit':'amber','pages':pages},indent=2)+'\n')
print(len(ps),'parameters;',len(programs),'original programs;',len(sound),'sound locks')

# Curated factory bank is separate from stable parameter identities and names.
bank=json.loads((P/'factory-sounds.json').read_text())
assert [(v['name'],v['model']) for v in bank]==[(v['name'],v['model']) for v in programs]
assert len(bank)==57
for entry in bank:
 assert set(entry['sound'])==set(sound)
 for key,value in entry['sound'].items():
  param=next(x for x in ps if x['key']==key)
  low,high=(0,len(param['choices'])-1)if 'choices'in param else(param['min'],param['max'])
  assert low<=value<=high,(entry['name'],key,value)
(P/'programs.json').write_text(json.dumps(bank,indent=2)+'\n')
fmt=lambda value:format(float(value),'.9g')+('f'if '.'in format(float(value),'.9g')or'e'in format(float(value),'.9g')else'.0f')
h=['#pragma once','// Generated from factory-sounds.json. Stable57program names/order.','namespace felt {',
   'static constexpr int kFactoryKeys[]={'+','.join('k'+x for x in sound)+'};',
   'static constexpr float kFactoryValues[57]['+str(len(sound))+']={']
for entry in bank:h.append(' {'+','.join(fmt(entry['sound'][key])for key in sound)+'}, // '+entry['name'])
h+=['};','template<class Values> inline void applyFactorySound(int index,Values& values){for(unsigned j=0;j<sizeof(kFactoryKeys)/sizeof(int);++j)values[kFactoryKeys[j]]=kFactoryValues[index][j];}','}']
(P/'src/FactorySounds.h').write_text('\n'.join(h)+'\n')
