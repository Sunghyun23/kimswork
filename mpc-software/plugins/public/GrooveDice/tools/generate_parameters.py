from pathlib import Path
import json
P=Path(__file__).resolve().parents[1];params=[]
def f(key,name,lo,hi,default,unit='',dec=0,auto=True):params.append(dict(key=key,name=name,min=lo,max=hi,default=default,unit=unit,decimals=dec,auto=auto))
def c(key,name,choices,default=0,auto=True):params.append(dict(key=key,name=name,choices=choices,default=default,auto=auto))
def b(key,name,default=0,auto=True):c(key,name,['Off','On'],default,auto)
f('Master','Master',0,1,.7,'',2)
for i in range(16):f('Q'+str(i+1),'Screen '+str(i+1),0,1,0,'',3,False)
f('Swing','Swing',50,75,54,'%');f('SnareLate','Snare Late',-40,60,12,'ms');f('HatShift','Hat Shift',-40,40,-4,'ms');f('RndNudge','Rnd Nudge',0,30,4,'ms')
f('Density','Density',0,100,100,'%');f('Ghost','Ghost',0,100,15,'%');f('VelSpread','Vel Spread',0,50,8,'%');f('RandomAmt','Random Amt',0,100,35,'%')
c('FillSlot','Fill Select',['A','B','C','D']);f('FillDensity','Fill Density',0,100,100,'%');c('FillEvery','Fill Every',['Off','4 Bars','8 Bars','16 Bars']);f('Variation','Variation',0,100,0,'%')
f('Drive','Drive',0,100,10,'%');f('Dust','Dust',0,100,10,'%');f('Room','Room',0,100,8,'%');f('Duck','Duck',0,100,15,'%')
c('Kit','Kit',['Dusty Velvet','Basement Soul','Amber Room']);c('Pattern','Pattern',['Velvet Stoop','Basement Bounce','Sunday Steps']);b('KitLock','Kit Lock');b('PatternLock','Pattern Lock');c('Scope','Scope',['Kit Only','Kit + Pattern','Kit + Groove','Kit + Fill','All','Kit + FX'],1)
for k,n in [('Roll','Roll Dice'),('Undo','Undo'),('Keep','Keep'),('Trigger','Trigger Fill'),('Record','Rec Fill'),('Cancel','Cancel'),('SaveKit','Save Kit'),('LoadKit','Load Kit')]:b(k,n,auto=False)
b('Preview','Preview');b('Follow','Follow MPC',1);f('PreviewBpm','Preview BPM',40,180,92,'BPM');b('Editor','Fill Editor',auto=False)
c('FillLength','Fill Length',['1 Beat','2 Beats','1 Bar','2 Bars','4 Bars'],2);c('RecMode','Rec Mode',['Replace','Overdub']);c('CountIn','Count-in',['Off','1 Bar','2 Bars'],1)
f('NoteSelect','Note Select',0,127,0);f('NoteNudge','Note Nudge',-60,60,0,'ms');f('NoteVelocity','Note Velocity',1,127,100);f('NoteGate','Note Gate',5,500,80,'ms');f('NoteChance','Note Chance',0,100,100,'%');b('NoteDelete','Delete Note',auto=False)
f('Status','Status',0,6,0,auto=False);f('Position','Position',0,9999,0,auto=False);f('RecordedCount','Fill Notes',0,128,0,auto=False)
for v in range(8):
 f(f'Pad{v}',f'Pad {v+1}',0,1,0,auto=False)
 f(f'Level{v}',f'Level {v+1}',-48,6,-6 if v in (2,3) else 0,'dB')
 f(f'Tune{v}',f'Tune {v+1}',-12,12,0,'st',1)
 f(f'Decay{v}',f'Decay {v+1}',20,2000,500,'ms')
 f(f'Pan{v}',f'Pan {v+1}',-100,100,0,'%')
 c(f'Sample{v}',f'Sample {v+1}',['1','2','3','4','5','6','7','8'])
 f(f'Choke{v}',f'Choke {v+1}',0,16,1 if v in (2,3) else 0)
 c(f'Route{v}',f'Output {v+1}',['Main/Kick','Snare','Closed Hat','Open Hat','Clap','Tom','Cymbal','Perc'],v)
 f(f'PadNote{v}',f'Pad Note {v+1}',0,127,[24,26,28,29,31,33,35,36][v])
f('ChokeTime','Choke Time',1,30,5,'ms');b('Separate','Separate Out');f('LowCut','Low Cut',0,300,0,'Hz');f('ModRate','Mod Rate',.05,8,.3,'Hz',2);f('ModDepth','Mod Depth',0,100,0,'%')
# Compact saved user kit slots, data kept in extra state.
c('UserKit','User Kit',['1','2','3','4','5','6','7','8']);c('EditDrum','Edit Drum',['Kick','Snare','Closed Hat']);b('Gated','Gated Samples')
f('GridSelect','Grid Select',0,64,0,auto=False)
for i in range(64):f(f'Grid{i}',f'Cell {i+1}',0,1,0,auto=False)
f('BankInfo','Sound Bank',0,1,0,auto=False)
c('GridBar','View Bar',['1','2','3','4'])
b('GridTap','Grid Tap',auto=False)
for key in ['KitPrev','KitNext','PatternPrev','PatternNext']:b(key,key,auto=False)
# Full private library: numeric identities do not expand native choice lists into thousands of widgets.
for r in range(8):f('LibrarySample'+str(r),'Library Sound '+str(r+1),0,11039,0,auto=False)
f('LibraryBeat','Library Beat',0,2,0,auto=False);f('GrooveTemplate','Groove Template',0,93,0,auto=False)
b('LibraryOpen','Library Browser',auto=False);c('LibraryKind','Library Type',['Kits','Chops','Loops','Beats','Grooves'],auto=False);c('LibraryRole','Destination',['Kick','Snare','Closed Hat','Open Hat','Clap','Tom','Cymbal','Perc'],auto=False)
f('LibraryPage','Library Page',0,2000,0,auto=False)
for k in ['LibraryPrev','LibraryNext','LibraryLegacy','LibraryBeatOff','GrooveOff']:b(k,k,auto=False)
for r in range(8):b('LibraryPick'+str(r),'Library Pick '+str(r+1),auto=False)
for r in range(8):f('LibraryRow'+str(r),'Library Row '+str(r+1),0,1,0,auto=False)
f('LibraryInfo','Library Info',0,1,0,auto=False)
f('LibraryFolder','Library Folder',0,2000,0,auto=False);b('LibraryBack','Library Back',auto=False)
f('LibraryCurrentSound','Current Sound',0,1,0,auto=False);f('LibraryPlaybackInfo','Library Playback',0,1,0,auto=False)
legacy=json.loads((P/'legacy-catalogue.json').read_text());factory=[{'name':n} for n in legacy['FactoryKit']['choices'][1:]]
c('FactoryKit','Drum Kit',['Custom / Legacy']+[k['name']for k in factory]);c('KitDiceMode','Dice Mode',['Whole Kit','Mix Parts'])
f('KitBrowserPage','Kit Page',0,(len(factory))//8,0,auto=False)
for key in ['KitPagePrev','KitPageNext']:b(key,key,auto=False)
f('DiceInfo','Dice Result',0,1,0,auto=False);b('FactoryReload','Reload Kit',auto=False)
pools=[range(legacy[k]['max']) for k in ['SoundKick','SoundSnare','SoundHat','SoundPerc']]
for key,name,pool in zip(['SoundKick','SoundSnare','SoundHat','SoundPerc'],['Kick','Snare','Hat','Perc Set'],pools):f(key,name,0,len(pool),0,auto=False)
c('SoundBrowserGroup','Sound Role',['Kick','Snare','Hat Pairs','Perc Sets'],auto=False);f('SoundBrowserPage','Sound Page',0,100,0,auto=False);b('SoundOpen','Role Sounds',auto=False)
for key in ['SoundPrev','SoundNext']+[f'SoundPick{i}'for i in range(8)]:b(key,key,auto=False)
for i in range(8):f(f'SoundRow{i}',f'Sound Row {i+1}',0,1,0,auto=False)
f('SoundInfo','Sound Info',0,1,0,auto=False)
for key,name,default in [('RuleKick','Kick Rule',1),('RuleSnare','Snare Rule',1),('RuleHat','Hat Rule',2),('RulePerc','Perc Rule',2)]:c(key,name,['LOCK','POCKET','LOOSE','BUSY'],default)
f('ActionInfo','Last Action',0,1,0,auto=False)
c('HatGrid','Hat Grid',['1/4','1/8','1/16','1/32 Roll'],1)
f('PhraseState','Phrase State',0,262143,0,auto=False)
c('HatRhythm','Hat Rhythm',[base+suffix for base in ['1/4','1/8','1/16','1/8 DOT','1/16 DOT'] for suffix in ['', ' + FILL',' + ROLL']],3)
f('HatPhraseState','Hat Phrase State',0,4095,0,auto=False)
c('HatRhythmPage','Hat Rhythm Page',['1 / 2','2 / 2'],auto=False)
# Expansion catalogue values are append-only; old fixed-size controls stay as compatibility aliases.
f('XKit','Expansion Kit',0,65535,0,auto=False)
for role in ['Kick','Snare','Hat','Perc']:f('XSound'+role,role+' Catalog',0,65535,0,auto=False)
for r in range(8):f('XSample'+str(r),'Expansion Sound '+str(r+1),0,1048575,0,auto=False)
for key,name in [('XBeat','Expansion Beat'),('XGroove','Expansion Groove'),('XKitPage','Kit Catalog Page'),('XSoundPage','Sound Catalog Page'),('XLibraryPage','Library Catalog Page'),('XLibraryFolder','Library Catalog Folder')]:f(key,name,0,131071,0,auto=False)
f('PackFilter','Expansion Filter',0,128,0,auto=False)
for key in ['PackPrev','PackNext','KitOpen']:b(key,key,auto=False)
f('PackInfo','Expansion Info',0,1,0,auto=False)
for i in range(8):f('KitRow'+str(i),'Kit Row '+str(i+1),0,1,0,auto=False)
for i in range(8):b('KitPick'+str(i),'Kit Pick '+str(i+1),auto=False)
for key,name in [('XQKit','Kit Select'),('XQKick','Kick Select'),('XQSnare','Snare Select'),('XQHat','Hat Select'),('XQPerc','Perc Select')]:f(key,name,0,1,0,'',4,False)
# 1.1 SOUND editor and mix trims: append, never reinterpret a persisted parameter ID.
for part in ['Kick','Snare','Hat','Perc']:f('Mix'+part,part+' Mix',-48,6,0,'dB',1)
c('SoundPart','Sound Part',['KICK','SNARE','HI-HAT','PERC'],auto=False)
c('SoundRow','Selected Layer',['1','2','3','4'],auto=False);b('SoundDetails','Sound Details',auto=False)
for i in range(16):f('SoundQ'+str(i),'Sound Control '+str(i+1),0,1,0,'',4,False)
for i in range(4):f('SoundRowInfo'+str(i),'Layer Name '+str(i+1),0,1,0,auto=False)
for field in ['On','Solo','Lock']:
 for i in range(4):b('Sound'+field+str(i),field+' '+str(i+1),auto=False)
b('SoundInvert','Invert Polarity',auto=False)
for key,name in [('SoundRoll','Roll Sound'),('SoundUndo','Undo Sound'),('SoundAudition','Audition Sound')]:b(key,name,auto=False)
# Base voices retain existing Sample/Level/Tune/Decay IDs. These fields extend every role.
for field in ['On','Solo','Lock','Shift','Start','LowCut','HighCut','Invert']:
 for role in range(8):
  key='Role'+field+str(role);name=field+' Part '+str(role+1)
  if field in ['On','Solo','Lock','Invert']:b(key,name,1 if field=='On'else 0)
  else:
   lo,hi,default,unit,dec={'Shift':(-30,30,0,'ms',1),'Start':(0,200,0,'ms',1),'LowCut':(0,1000,0,'Hz',0),'HighCut':(2000,20000,20000,'Hz',0)}[field];f(key,name,lo,hi,default,unit,dec)
# Three additional layers for Kick and Snare. Slot number=(role*3+layer-1).
for slot in range(6):
 prefix='L'+str(slot);caption=('Kick'if slot<3 else'Snare')+' L'+str(slot%3+2)
 f(prefix+'Sample',caption+' Sample',0,1048575,0,auto=False)
 f(prefix+'Level',caption+' Level',-48,6,-12,'dB',1);f(prefix+'Tune',caption+' Tune',-12,12,0,'st',1)
 f(prefix+'Shift',caption+' Shift',-30,30,0,'ms',1);f(prefix+'Decay',caption+' Decay',20,2000,500,'ms')
 b(prefix+'On',caption+' On');b(prefix+'Lock',caption+' Lock');b(prefix+'Solo',caption+' Solo')
 f(prefix+'Start',caption+' Start',0,200,0,'ms',1);f(prefix+'LowCut',caption+' Low Cut',0,1000,0,'Hz');f(prefix+'HighCut',caption+' High Cut',2000,20000,20000,'Hz');b(prefix+'Invert',caption+' Invert')
# Append rule styles: preserve the original four-choice automation IDs/ranges.
styles=[('Kick',['LOCK','BASIC','VARIED','BUSY','DOWNBEAT','OFFBEATS','SYNCOPATED','DOUBLE HIT'],1),('Snare',['LOCK','BASIC','VARIED','BUSY','SOFT','GHOST BEFORE','GHOST AFTER','DOUBLE GHOST'],1),('Hat',['LOCK','BASIC','VARIED','BUSY','SPARSE','OFFBEATS','ACCENTED','OPEN HAT'],2),('Perc',['LOCK','BASIC','VARIED','BUSY','SPARSE','OFFBEATS','ANSWER SNARE','PERC LOOP'],2)]
for role,choices,default in styles:c('Rule'+role+'Style',role+' Style',choices,default)
# Sample arrows are independent commands so a click always addresses its own row.
for row in range(4):
 b('SoundSamplePrev'+str(row),'Previous Sample '+str(row+1),auto=False)
 b('SoundSampleNext'+str(row),'Next Sample '+str(row+1),auto=False)
# Independent timekeeper/transition rules; keep all 533 previous declarations unchanged.
c('OpenHatRule','Open Hat',['OFF','OFFBEAT','BAR END','FILL END'])
c('RideRule','Ride',['OFF','1/4','1/8','1/4 + BELL','1/8 + BELL'])
c('CrashRule','Crash',['OFF','START','AFTER FILL','EVERY 4','EVERY 8','EVERY 16'])
# Append-only source browser and independent cymbal sound controls.
b('CymbalView','Cymbal Sounds',auto=False)
for slot,caption in enumerate(['Ride','Crash','Bell']):
 prefix='Cym'+str(slot)
 f(prefix+'Sample',caption+' Sample',-1,1048575,0,auto=False)
 f(prefix+'Level',caption+' Level',-48,6,0,'dB',1);f(prefix+'Tune',caption+' Tune',-12,12,0,'st',1)
 f(prefix+'Shift',caption+' Shift',-30,30,0,'ms',1);f(prefix+'Decay',caption+' Decay',20,4000,800,'ms')
 b(prefix+'On',caption+' On',1);b(prefix+'Lock',caption+' Lock');b(prefix+'Solo',caption+' Solo')
 f(prefix+'Start',caption+' Start',0,200,0,'ms',1);f(prefix+'LowCut',caption+' Low Cut',0,1000,0,'Hz');f(prefix+'HighCut',caption+' High Cut',2000,20000,20000,'Hz');b(prefix+'Invert',caption+' Invert')
for row in range(4):
 b('SoundBrowse'+str(row),'Browse Sample '+str(row+1),auto=False)
 b('SoundPreview'+str(row),'Preview Sample '+str(row+1),auto=False)
b('SampleBrowserOpen','Sample Browser',auto=False)
f('SampleBrowserPage','Sample Page',0,131071,0,auto=False)
for name in ['SamplePrev','SampleNext']:b(name,name,auto=False)
f('SampleInfo','Sample List',0,1,0,auto=False)
for row in range(8):f('SampleRow'+str(row),'Sample Row '+str(row+1),0,1,0,auto=False)
for row in range(8):b('SamplePick'+str(row),'Pick Sample '+str(row+1),auto=False)
c('FillGridPage','Fill Instruments',['SNARE / TOM / PERC / CLAP','KICK / HATS / CYMBAL'],auto=False)
c('DicePool','Use Sounds From',['THIS PACK','ALL PACKS','SELECTED KITS'],auto=False)
b('PoolOpen','Select Kits',auto=False);b('PoolClear','Clear Selected Kits',auto=False)
f('PoolInfo','Selected Kits',0,1,0,auto=False)
for row in range(8):b('PoolPick'+str(row),'Toggle Kit '+str(row+1),auto=False)
for row in range(8):f('PoolRow'+str(row),'Pool Kit '+str(row+1),0,1,0,auto=False)
# Per-control Dice protection; locks affect randomisation, not manual editing.
for n in range(16):b('DiceLock'+str(n),'Dice Lock '+str(n+1),auto=False)
# Seven playable styles; legacy eight-choice IDs stay intact for saved projects.
short_styles={'GHOST BEFORE':'PRE GHOST','GHOST AFTER':'POST GHOST','DOUBLE GHOST':'2 GHOSTS','ANSWER SNARE':'ANSWER','SYNCOPATED':'SYNCOPATE'}
for role,choices,default in styles:c('RulePick'+role,role+' Style Picker',[short_styles.get(x,x)for x in choices[1:]],default-1)
# Independent reverse flags: eight base roles, six extra layers, three cymbals.
for n in range(17):b('Reverse'+str(n),'Reverse Sound '+str(n+1))
for row in range(4):b('SoundRev'+str(row),'Reverse Row '+str(row+1),auto=False)
for row in range(4):f('SoundTune'+str(row),'Tune Row '+str(row+1),-12,12,0,'st',1,False)
for row in range(4):b('SoundPolarity'+str(row),'Polarity Row '+str(row+1),auto=False)
# Append full selectors; retain all672existing declarations and normalized ranges.
extra_styles={
 'Kick':['ONE THREE','ANTICIPATE','LATE ANSWER','PICKUP','THREE HITS','FOUR HITS','SYNC PAIR','STAGGER','END DOUBLE'],
 'Snare':['SECOND UP','FIRST UP','LOW ACCENT','POST BOTH','EARLY PAIR','LATE PAIR','TRIPLET','DRAG IN','BAR TURN'],
 'Hat':['FIRST HALF','LAST HALF','ON BEATS','UPBEATS','TWO ON ONE','ONE ON TWO','SOFT TAIL','CRESCENDO','SKIP TWO'],
 'Perc':['CLAP REPLY','TOM TURN','SHAKER','CALL REPLY','OFFBEAT TOM','CLAP PAIR','LATIN THREE','TOM CLAP','END PICKUP']}
for role,choices,default in styles:c('RulePickFull'+role,role+' Extended Style Picker',[short_styles.get(x,x)for x in choices[1:]]+extra_styles[role],default-1)
from dice_lock_catalog import append as append_dice_locks
append_dice_locks(params)
# Full selector appends Triplets without changing legacy 0..778 metadata/ranges.
c('HatRhythmFull','Full Hat Rhythm',[base+suffix for base in ['1/4','1/8','1/16','1/8 DOT','1/16 DOT','1/4 T','1/8 T','1/16 T'] for suffix in ['', ' + FILL',' + ROLL']],3)
f('HatPhraseFullState','Full Hat Phrase State',0,32767,0,auto=False)
c('HatRhythmFullPage','Full Hat Rhythm Page',['1 / 3','2 / 3','3 / 3'],auto=False)
P.joinpath('parameters.json').write_text(json.dumps(params,indent=2)+'\n')
lines=['// Generated parameter declarations. Stable index order from parameters.json.','enum Key { '+', '.join('k'+p['key'] for p in params)+', kParamCount };','inline void declareParameters(mpc::Plugin& plugin) {']
for p in params:
 if 'choices'in p:expr='mpc::choiceParam('+json.dumps(p['key'])+','+json.dumps(p['name'])+',{'+','.join(json.dumps(v)for v in p['choices'])+'},'+str(p['default'])+')'
 else:expr='mpc::floatParam('+','.join([json.dumps(p['key']),json.dumps(p['name']),str(p['min']),str(p['max']),str(p['default']),json.dumps(p['unit']),str(p['decimals'])])+')'
 lines+=['{ auto p='+expr+'; p.automatable='+str(p['auto']).lower()+'; plugin.addParam(p); }']
lines+=['}']
from dice_lock_catalog import definitions as dice_definitions
lines+=dice_definitions()
P.joinpath('src/Parameters.h').write_text('\n'.join(lines)+'\n')
p=P/'plugin.json';m=json.loads(p.read_text());m.update(version='1.6.1',numOutputs=16,category='Drum',description='Neo-soul drum instrument with original procedural sounds, modular expansions, groove generation and fill recording.',url='https://kimswork.app/',license='MIT');m.pop('uiVersion',None);p.write_text(json.dumps(m,indent=2)+'\n')
main=['Swing','Snare Late','Hat Shift','Rnd Nudge','Kick Style','Snare Style','Hat Style','Perc Style','Kick Mix','Snare Mix','Hat Mix','Perc Mix','Drive','Dust','Room','Full Hat Rhythm']
pages=[{'name':'MAIN','controls':main},{'name':'FILL','controls':['Fill Select','Fill Length','Rec Mode','Count-in','Note Nudge','Note Velocity','Note Gate','Note Chance']},{'name':'GROOVE','controls':main[:4]+['Density','Ghost','Vel Spread','Random Amt']+['Preview BPM','Note Select','Note Nudge','Note Velocity','Fill Select','Fill Density','Fill Every','Variation']},{'name':'SOUND','controls':['Sound Control '+str(i+1) for i in range(16)]},{'name':'KITS','controls':['Kit Select','Kit Lock','Pattern','Pattern Lock','User Kit','Save Kit','Load Kit','Choke Time']+[f'Choke {i}'for i in range(1,9)]},{'name':'FX','controls':['Drive','Dust','Room','Duck','Low Cut','Mod Rate','Mod Depth','Master']},{'name':'ROUTING','controls':[f'Output {i}'for i in range(1,9)]+[f'Pad Note {i}'for i in range(1,9)]}]
P.joinpath('ui.json').write_text(json.dumps({'kit':'amber','pages':pages},indent=2)+'\n')
