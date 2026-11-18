#!/usr/bin/env python3
"""Generate stable parameter declarations and curated factory patch/progression data."""
import json
from pathlib import Path
P=Path(__file__).resolve().parents[1]
params=[]
def f(key,name,lo,hi,default,unit='',log=False): params.append(dict(key=key,name=name,min=lo,max=hi,default=default,unit=unit,log=log))
def c(key,name,choices,default=0): params.append(dict(key=key,name=name,choices=choices,default=default))
f('Volume','Volume',-60,0,-8,'dB')
for key,default in [('Tone',45),('Body',40),('Motion',15),('Space',20),('Drive',0)]: f(key,'Legacy Body' if key=='Body' else key,0,100,default,'%')
f('Glide','Glide',0,500,0,'ms');f('Vibrato','Vibrato',0,100,0,'%');f('Expression','Expression',0,100,70,'%')
f('Mix','Osc Mix',0,100,40,'%');f('Detune','Detune',-25,25,6,'ct');f('Cutoff','Filter Cutoff',20,18000,2800,'Hz',True);f('Resonance','Resonance',0,100,18,'%')
c('Shape','LFO Shape',['Sine','Triangle','Square','S&H']);f('Rate','LFO Rate',.05,20,1.2,'Hz',True);f('Depth','LFO Depth',0,100,12,'%');c('Target','LFO Target',['Pitch','Filter','Pulse Width','Amp'],1)
c('Wave1','Osc 1 Wave',['Sine','Triangle','Saw','Pulse'],1);c('Wave2','Osc 2 Wave',['Sine','Triangle','Saw','Pulse'],0);c('Filter','Filter Type',['Low-pass','Band-pass','High-pass','Notch']);c('DriveType','Drive Type',['Tape','Tube','Fuzz']);c('VoiceMode','Voice Mode',['Mono','Legato','Poly'],2)
f('Octave','Osc 2 Oct',-2,2,0,'oct');f('Pulse','Pulse Width',5,95,50,'%');f('Sub','Sub',0,100,10,'%');f('Noise','Noise Level',0,30,0,'%');f('FM','FM Amount',0,100,12,'%')
for prefix,label,vals in [('Amp','Amp',[8,600,55,350]),('Filt','Filter',[6,350,25,300])]:
 for stage,d in zip(['Attack','Decay','Sustain','Release'],vals):f(prefix+stage,label+' '+stage,0 if stage=='Sustain' else 1,100 if stage=='Sustain' else 5000,d,'%' if stage=='Sustain' else 'ms',stage!='Sustain')
f('Env','Filter Env',-100,100,25,'%');f('KeyTrack','Key Track',0,100,50,'%');f('Bend','Bend Range',0,24,2,'st');c('GlideLegato','Glide Mode',['Always','Legato only'],1);c('Sync','LFO Sync',['Off','On']);c('Division','LFO Division',['1/16','1/8','1/4','1/2','1 bar'],2)
f('DriveMix','Drive Mix',0,100,100,'%');f('Chorus','Chorus',0,100,15,'%');f('Delay','Delay Mix',0,100,10,'%');f('Feedback','Feedback',0,65,25,'%');c('DelayDivision','Delay Division',['1/8','1/4','1/4 dotted','1/2'],1)
from sound_bank import build as build_sounds
patches=build_sounds({p['key']:p['default'] for p in params})
c('SoundPreset','Sound Preset',[p['name'] for p in patches])
# Parameters below this boundary are intentionally excluded from sound program values.
c('InputMode','Input Mode',['Notes','Chords','Split'],2);f('Trigger','Trigger Base',0,124,36,'MIDI');f('Spread','Chord Spread',0,40,0,'ms')
qualities=['Minor 7','Major 7','Minor 9','Major 9','Dominant 7','Dominant 9','Dominant 13','Sus 2','Minor','Major','7 flat 9']
progressions=[]
recipes=[('Sunday Soul',[(50,2),(55,6),(48,3),(57,2)]),('Basement Loop',[(53,2),(46,2),(51,5),(56,1)]),('After Hours',[(50,2),(55,2),(57,10),(50,2)]),('Velvet Morning',[(51,3),(48,2),(53,2),(46,6)]),('Dusty Sunset',[(57,2),(53,3),(50,2),(52,10)]),('Warm Windows',[(55,3),(52,2),(57,2),(50,6)]),('Open Sky',[(48,7),(53,3),(57,7),(55,7)]),('Soul Turnaround',[(48,3),(57,4),(50,2),(55,5)])]
for shift,suffix in [(0,''),(2,' +2'),(-2,' -2'),(5,' +5')]:
 for name,chords in recipes:progressions.append(dict(name=name+suffix,chords=[(root+shift,q) for root,q in chords]))
legacy_progressions=list(progressions)
from harmony_bank import TYPES,MINOR_TYPES,extend
progressions=extend(progressions)
c('Progression','Chord Preset',[p['name'] for p in legacy_progressions])
for i,(root,q) in enumerate(progressions[0]['chords']):
 f('Root'+str(i+1),'Chord '+str(i+1)+' Root',24,84,root,'MIDI');c('Quality'+str(i+1),'Chord '+str(i+1)+' Type',qualities,q);c('Voicing'+str(i+1),'Chord '+str(i+1)+' Voice',['Root','1st inv','Open','2nd inv','3rd inv','4th inv'])
c('ChordSlot','Chord Slot',['Pad '+str(i) for i in range(1,17)])
for key in ['SoundPrev','SoundNext','ChordPrev','ChordNext']: c(key,key,['Off','On'])
# Append-only additions preserve the 68 shipped parameter indices.
for i in range(1,5):c('Touch'+str(i),'Play Chord '+str(i),['Off','On'])
c('TouchStop','Stop Chords',['Off','On'])
c('FXPreset','FX Preset',['Custom','Dry','Warm Tape','Wide Chorus','Slap Delay','Space Echo','Fuzz Box','Dark Dub'])
f('TremRate','Tremolo Rate',.1,15,4,'Hz',True)
f('TremDepth','Tremolo Depth',0,100,0,'%')
c('OscPair','OSC Pair',[a+' + '+b for a in ['Sine','Triangle','Saw','Pulse'] for b in ['Sine','Triangle','Saw','Pulse']],1)
# Append-only per-slot MIDI mapping; legacy Trigger Base remains available.
for i,note in enumerate([24,26,28,29],1):f('PadNote'+str(i),'Pad '+str(i)+' Note',0,127,note,'MIDI')
c('PadMap','Pad Mapping',['Consecutive','Individual'],1)
c('PadLearn','Pad Learn',['Off']+['Slot '+str(i) for i in range(1,17)])
f('Drift','Drift',0,100,0,'%')
c('LearnStatus','Pad Learn Status',['Ready — LEARN assigns a physical pad']+['Pad '+str(i)+' registered — press again to play' for i in range(1,17)])
f('LowCut','Low Cut',0,250,0,'Hz')
f('OutputTrim','Sound Trim',-18,18,0,'dB')
# Append per-chord octave offsets; existing voicing indices 0/1/2 stay unchanged.
for i in range(1,5):c('ChordOctave'+str(i),'Chord '+str(i)+' Octave',['-2 oct','-1 oct','0 oct','+1 oct','+2 oct'],2)
# 0.1.20: append slots 5..16; preserve the first 91 parameter identities.
for slot in range(5,17):
 root,q=progressions[0]['chords'][(slot-1)%4]
 f('Root'+str(slot),'Chord '+str(slot)+' Root',24,84,root,'MIDI')
 c('Quality'+str(slot),'Chord '+str(slot)+' Type',qualities,q)
 c('Voicing'+str(slot),'Chord '+str(slot)+' Voice',['Root','1st inv','Open','2nd inv','3rd inv','4th inv'])
 c('ChordOctave'+str(slot),'Chord '+str(slot)+' Octave',['-2 oct','-1 oct','0 oct','+1 oct','+2 oct'],2)
 f('PadNote'+str(slot),'Pad '+str(slot)+' Note',0,127,24+((slot-1)//7)*12+[0,2,4,5,7,9,11][(slot-1)%7],'MIDI')
 c('Touch'+str(slot),'Play Chord '+str(slot),['Off','On'])
c('SlotCount','Chord Slots',['4 SLOTS','8 SLOTS','12 SLOTS','16 SLOTS'])
c('MainChordPage','Chord Pad Page',['1–4','5–8','9–12','13–16'])
for key in ('SlotsMinus','SlotsPlus','PadPagePrev','PadPageNext'):c(key,key,['Off','On'])
for page in range(1,5):c('ChordPlayingPage'+str(page),'Playing Chord Page '+str(page),['Off','On'])
# Independent progression selection for each four-slot bank; legacy bank 1 keeps its ID.
for bank in range(2,5):c('Progression'+str(bank),'Chord Preset Bank '+str(bank),[p['name'] for p in legacy_progressions])
# Static editor Q-Links mirror the four slots selected by the shared pad page.
for row in range(1,5):
 f('EditRoot'+str(row),'Edit '+chr(64+row)+' Root',24,84,progressions[0]['chords'][row-1][0],'MIDI')
 c('EditQuality'+str(row),'Edit '+chr(64+row)+' Type',qualities,progressions[0]['chords'][row-1][1])
 c('EditVoicing'+str(row),'Edit '+chr(64+row)+' Voice',['Root','1st inv','Open','2nd inv','3rd inv','4th inv'])
 c('EditOctave'+str(row),'Edit '+chr(64+row)+' Octave',['-2 oct','-1 oct','0 oct','+1 oct','+2 oct'],2)
# Manual physical-position hints. MIDI never provides a pad bank/index.
for slot in range(1,17):c('PadPosition'+str(slot),'Pad '+str(slot)+' Position',['Unassigned']+[f'{bank}{pad}' for bank in 'ABCDEFGH' for pad in range(1,17)])
# 1.3.0 append-only key reference and explicit all-active-slot transpose.
notes=['C','C# / Db','D','Eb','E','F','F# / Gb','G','Ab','A','Bb','B']
c('ChordKeyRoot','Chord Key',notes)
c('ChordKeyMode','Chord Key Mode',['Major','Minor'])
c('TransposeFrom','Transpose From',notes)
c('TransposeFromMode','Transpose From Mode',['Major','Minor'])
c('TransposeOpen','Transpose Preview',['Off','On'])
c('TransposeApply','Apply Transpose',['Off','On'])
c('TransposeStatus','Transpose Status',['Ready','Applied','No pitch change','OUT OF RANGE — adjust source key or chord roots'])
for row in range(1,5):c('KeyDegree'+str(row),'Chord Degree '+str(row),['—','I','II','III','IV','V','VI','VII','i','ii','iii','iv','v','vi','vii'])
f('TransposeDelta','Transpose Interval',-5,6,0,'st')
# 1.4.0: explicit preview/apply for loop-aware top-note voice leading.
c('SmoothOpen','Smooth Voicing',['Off','On'])
c('SmoothScope','Smooth Scope',['Current 4','All active slots'])
c('SmoothApply','Apply Smooth Voicing',['Off','On'])
c('SmoothPreview','Preview Smooth Voicing',['Off','On'])
c('SmoothStatus','Smooth Status',['Calculating','Ready','Preview playing','Applied','ADJUST CHORD REGISTER','Already smooth','Recalculating'])
f('SmoothMovement','Top Note Movement',0,2048,0,'st')
for row in range(1,17):f('SmoothRow'+str(row),'Top Note '+str(row),0,127,0,'MIDI')
# Stable legacy ranges remain aliases; append full-range canonical selectors.
for slot in range(1,17):c('HarmonyType'+str(slot),'Chord '+str(slot)+' Harmony',[t[0]for t in TYPES],progressions[0]['chords'][(slot-1)%4][1])
for bank in range(1,5):c('HarmonyPreset'+str(bank),'Harmony Preset '+str(bank),[p['name']for p in progressions])
for row in range(1,5):c('HarmonyEdit'+str(row),'Harmony Edit '+chr(64+row),[t[0]for t in TYPES],progressions[0]['chords'][row-1][1])
for row in range(1,5):c('HarmonyTones'+str(row),'Harmony Tones '+chr(64+row),['Triad','Seventh','Extended'],2)
c('ChordGlide','Chord Glide',['Off','On'])
c('HarmonyCurrent','Current Chord Preset',[p['name']for p in progressions])
# Shared motion controls: old tremolo rate/depth IDs and defaults stay exact.
c('ModMode','Motion Mode',['Tremolo','Auto Pan'])
c('ModShape','Motion Shape',['Sine','Triangle','Saw','Square','S&H'])
f('Pan','Pan',-100,100,0,'%')
c('ModSync','Motion Sync',['Off','On'])
c('ModDivision','Motion Division',['1/16','1/8','1/4','1/2','1 bar'],2)
f('LfoRateControl','LFO Rate Control',0,1,0)
f('ModRateControl','Mod Rate Control',0,1,0)
# Body is a new active control; legacy Body remains inert for old projects.
f('BodyTone','Body',0,100,0,'%')
c('Lfo2Shape','LFO 2 Shape',['Sine','Triangle','Saw','Square','S&H'])
f('Lfo2Rate','LFO 2 Rate',.05,20,1.2,'Hz',True)
f('Lfo2Depth','LFO 2 Depth',0,100,0,'%')
c('Lfo2Target','LFO 2 Target',['Pitch','Filter','Pulse Width','Amp','OSC Mix'],1)
c('Lfo2Sync','LFO 2 Sync',['Off','On'])
c('Lfo2Division','LFO 2 Division',['1/16','1/8','1/4','1/2','1 bar'],2)
f('Lfo2RateControl','LFO 2 Rate Control',0,1,0)
# Append the reviewed Soul Studies without changing any legacy choice range.
soul_sounds=[]
extras=('TremRate','TremDepth','OscPair','Drift','LowCut','OutputTrim','ModMode','ModShape','Pan','ModSync','ModDivision','BodyTone','Lfo2Shape','Lfo2Rate','Lfo2Depth','Lfo2Target','Lfo2Sync','Lfo2Division')
timbral_keys=[p['key'] for p in params[1:46]]+list(extras)
for slug,category,name in [('dusty-sunday','Keys','Dusty Sunday'),('velvet-melt','Synth','Velvet Melt')]:
 recipe=json.loads((P/'presets/soul-studies/sync-88-eighth'/f'{slug}.json').read_text())['valuesById']
 values={key:recipe[key.lower()] for key in timbral_keys}
 # Keep master volume user-owned; retain the approved level at default -8dB.
 values['OutputTrim']+=recipe['volume']-params[0]['default']
 soul_sounds.append(dict(name=f'{category} / {name}',category=category,values=values))
factory_sounds=patches+soul_sounds
c('SoundCurrent','Full Sound Preset',[p['name']for p in factory_sounds])
c('Randomize','Random Sound',['Off','On'])
c('Randomized','Random Sound Status',['Factory','Random'])
c('FactoryReload','Reload Sound',['Off','On'])
(P/'src/ChordTypes.h').write_text('#pragma once\nnamespace donut {\nstatic constexpr int kTypeCount=32;\nstatic constexpr int kTypeSizes[32]={'+','.join(str(len(t[2]))for t in TYPES)+'};\nstatic constexpr int kTypeIntervals[32][5]={' + ','.join('{'+','.join(map(str,list(t[2])+[0]*(5-len(t[2]))))+'}'for t in TYPES)+'};\nstatic const char*const kTypeSuffixes[32]={'+','.join(json.dumps(t[1])for t in TYPES)+'};\ninline bool minorType(int q){switch(q){'+''.join('case '+str(q)+':'for q in sorted(MINOR_TYPES))+'return true;default:return false;}}\n}\n')
(P/'parameters.json').write_text(json.dumps(params,indent=2)+'\n')
(P/'factory-presets.json').write_text(json.dumps(dict(sounds=factory_sounds,progressions=progressions),indent=2)+'\n')
lines=['// Generated by tools/generate_catalog.py; stable IDs must not be reordered after release.','#pragma once','enum Params { '+', '.join('k'+p['key'] for p in params)+', kNumParams };','inline void declareParameters(mpc::Plugin& p) {']
for p in params:
 key=p['key'];nm=p['name'];d=p['default']
 if 'choices'in p:expr='mpc::choiceParam('+json.dumps(key.lower())+','+json.dumps(nm)+', {'+','.join(json.dumps(x) for x in p['choices'])+'},'+str(d)+')'
 else:expr='mpc::'+('logParam' if p['log'] else 'floatParam')+'('+json.dumps(key.lower())+','+json.dumps(nm)+','+','.join(str(p[x])+'f' if '.' in str(p[x]) else str(p[x])+'.0f' for x in ['min','max','default'])+','+json.dumps(p['unit'])+', 1)'
 lines.append('{auto editor='+expr+';editor.automatable=false;p.addParam(std::move(editor));}' if key.startswith(('Edit','PadPosition','ChordKey','Transpose','KeyDegree','Smooth','HarmonyEdit','HarmonyTones','HarmonyCurrent','LfoRateControl','ModRateControl','Lfo2RateControl','Randomize','Randomized','FactoryReload')) else 'p.addParam('+expr+');')
lines+=['}']
for name,prefix in [('Roots','Root'),('LegacyQualities','Quality'),('Qualities','HarmonyType'),('Voicings','Voicing'),('Octaves','ChordOctave'),('PadNotes','PadNote'),('Touches','Touch')]:lines.append('static constexpr int kChord'+name+'[16] = {'+','.join('k'+prefix+str(slot) for slot in range(1,17))+'};')
lines.append('static constexpr int kLegacyBankProgressions[4] = {kProgression,kProgression2,kProgression3,kProgression4};')
lines.append('static constexpr int kBankProgressions[4] = {kHarmonyPreset1,kHarmonyPreset2,kHarmonyPreset3,kHarmonyPreset4};')
lines+=['static const char* const kPatchNames[] = {'+','.join(json.dumps(p['name']) for p in factory_sounds)+'};','static constexpr int kFactoryProgramCount=66;','static constexpr float kPatches[64][kSoundPreset+1] = {']
for i,p in enumerate(patches):lines.append('{'+','.join(str(float(p['values'][q['key']]))+'f' for q in params[:next(i for i,p in enumerate(params) if p['key']=='SoundPreset')])+','+str(float(i))+'f},')
lines+=['};','static constexpr int kProgressionCount=96;\nstatic constexpr int kProgressions[96][4][2] = {']
for p in progressions:lines.append('{'+','.join('{'+str(a)+','+str(b)+'}' for a,b in p['chords'])+'},')
lines+=['};','static constexpr float kPatchExtras[64][5] = {']
for patch in patches:lines.append('{'+','.join(str(float(patch['values'][key]))+'f' for key in ('LowCut','Drift','TremRate','TremDepth','OutputTrim'))+'},')
lines+=['};']
lines+=['struct SoulValue {int parameter;float value;};','static constexpr SoulValue kSoulValues[2]['+str(len(timbral_keys))+'] = {']
for sound in soul_sounds:lines.append('{'+','.join('{k'+key+','+str(float(sound['values'][key]))+'f}'for key in timbral_keys)+'},')
lines+=['};']
(P/'src/Catalog.h').write_text('\n'.join(lines)+'\n')
# Main uses four horizontal banks; custom skin generator remaps their 2x2 geometry.
pages=[('MAIN',['OscPair','Mix','Detune','Drift','Cutoff','Resonance','BodyTone','Sub','Rate','Depth','Lfo2RateControl','Lfo2Depth','Drive','Glide','TremRate','TremDepth']),('OSC',['Wave1','Wave2','Mix','Detune','Octave','Pulse','Sub','Noise','FM','Tone','VoiceMode','SoundPreset']),('FILTER',['Filter','Cutoff','Resonance','Env','FiltAttack','FiltDecay','FiltSustain','FiltRelease','KeyTrack','BodyTone','Sub']),('AMP',['AmpAttack','AmpDecay','AmpSustain','AmpRelease','Volume','Expression','VoiceMode','Glide','GlideLegato','Bend','InputMode','Trigger','Spread','ChordSlot']),('MOD',['Shape','Rate','Depth','Target','Sync','Division','Vibrato','Motion','Lfo2Shape','Lfo2RateControl','Lfo2Depth','Lfo2Target','Lfo2Sync','Lfo2Division','Progression']),('FX',['DriveType','Drive','DriveMix','Chorus','Delay','Feedback','DelayDivision','Space','FXPreset','TremRate','TremDepth','LowCut','OutputTrim','ModMode','ModShape','Pan'])]
pages.append(('CHORD',[x+str(i) for i in range(1,5) for x in ['Root','Quality','Voicing']]+['Progression','Spread','InputMode','Trigger']))
lookup={p['key']:p for p in params}
ui={'kit':'amber','style':{'background':'ffe1d2ae','text':'ff312b20','muted':'ff514634','accent':'ffac5b2a'},'pages':[]}
for name,keys in pages:
 controls=[]
 for key in keys:
  p=lookup[key];widget='fader' if key.startswith(('AmpAttack','AmpDecay','AmpSustain','AmpRelease','FiltAttack','FiltDecay','FiltSustain','FiltRelease')) else 'knob'
  if 'choices'in p:widget='switch' if len(p['choices'])<=4 else 'slider'
  controls.append(dict(param={'Rate':'LFO Rate Control','TremRate':'Mod Rate Control','TremDepth':'Mod Depth'}.get(key,p['name']),widget=widget,**({'overlay':'menu'} if 'choices'in p else {})))
 ui['pages'].append(dict(name=name,controls=controls))
(P/'ui.json').write_text(json.dumps(ui,indent=2)+'\n')
manifest=json.loads((P/'plugin.json').read_text());manifest.update(description='Soulful polyphonic synth with up to sixteen editable chord slots and independent sound/progression presets.',url='https://github.com/Sunghyun23/kimswork')
(P/'plugin.json').write_text(json.dumps(manifest,indent=2)+'\n')
