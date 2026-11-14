#include <cassert>
#include <iostream>
#include <memory>
#include <new>
#include <cstdlib>
static bool forbidAlloc=false;
#if !__has_feature(address_sanitizer)
void* operator new(std::size_t n){if(forbidAlloc){std::cerr<<"allocation in audio callback\n";std::abort();}if(void* p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void* p)noexcept{std::free(p);}
void* operator new[](std::size_t n){return ::operator new(n);}
void operator delete[](void* p)noexcept{std::free(p);}
#endif
#include "../src/DonutTheory.cpp"
struct Rig {
 std::unique_ptr<DonutTheory> p=std::make_unique<DonutTheory>();
 float l[512]{},r[512]{};float* out[2]={l,r};
 Rig(){p->set(kInputMode,0);p->set(kPadMap,0);p->prepare(44100,512);}
 float block(std::initializer_list<mpc::MidiEvent> events={},int size=128){
  std::fill(l,l+512,0);std::fill(r,r+512,0);mpc::ProcessContext c;c.outputs=out;c.numOutputs=2;c.numFrames=size;c.sampleRate=44100;c.midi=events.begin();c.numMidiEvents=int(events.size());
  forbidAlloc=true;p->process(c);forbidAlloc=false;
  float peak=0;for(int i=0;i<size;++i){assert(std::isfinite(l[i])&&std::isfinite(r[i]));assert(std::abs(l[i])<=1&&std::abs(r[i])<=1);peak=std::max(peak,std::abs(l[i]));}return peak;
 }
 void finish(){for(int i=0;i<2100;++i)block();assert(p->activeVoices()==0);}
};
int main(){
 // Sixteen slots, view-independent MIDI, count/page safety and old-state migration.
 {Rig m;m.p->set(kSlotCount,3);m.p->set(kInputMode,1);m.p->set(kPadMap,1);m.p->set(kAmpRelease,1);
  for(int slot=0;slot<16;++slot)m.p->set(kChordPadNotes[slot],80+slot);m.block();
  for(int slot=0;slot<16;++slot){m.block({mpc::MidiEvent::noteOn(0,80+slot,100)});assert(m.p->activeVoices()>=3);assert(m.p->getInt(kChordPlayingPage1+slot/4)==1);
   m.block({mpc::MidiEvent::noteOff(0,80+slot)});for(int b=0;b<40;++b)m.block();assert(m.p->activeVoices()==0);
   m.p->set(kChordTouches[slot],1);m.block();assert(m.p->activeVoices()>=3);m.p->set(kChordTouches[slot],0);for(int b=0;b<40;++b)m.block();assert(m.p->activeVoices()==0);}
  m.p->set(kChordRoots[15],62);m.p->set(kChordOctaves[15],4);m.p->set(kChordVoicings[15],4);m.p->set(kMainChordPage,3);m.block();
  m.block({mpc::MidiEvent::noteOn(0,95,100)});m.p->set(kMainChordPage,0);m.block();assert(m.p->activeVoices()==5&&m.p->getInt(kChordPlayingPage4)==1);
  m.p->set(kTouch1,1);m.block();assert(m.p->activeVoices()==10);m.p->set(kMainChordPage,1);m.block();for(int b=0;b<40;++b)m.block();assert(m.p->activeVoices()==5&&m.p->getInt(kTouch1)==0);
  m.p->set(kSlotsMinus,1);m.block();assert(m.p->getInt(kSlotCount)==2&&m.p->activeVoices()==0);m.p->set(kSlotCount,0);m.p->set(kMainChordPage,3);m.block();assert(m.p->getInt(kMainChordPage)==0);
  m.block({mpc::MidiEvent::noteOn(0,95,100)});assert(m.p->activeVoices()==0);m.p->set(kChordTouches[15],1);m.block();assert(m.p->activeVoices()==0&&m.p->getInt(kChordTouches[15])==0);
  m.p->set(kSlotCount,3);m.block();assert(m.p->getInt(kChordRoots[15])==62&&m.p->getInt(kChordOctaves[15])==4&&m.p->getInt(kChordVoicings[15])==4&&m.p->getInt(kChordPadNotes[15])==95);
  m.p->set(kPadLearn,16);m.block({mpc::MidiEvent::noteOn(0,81,100)});assert(m.p->getInt(kPadNote16)==81&&m.p->getInt(kPadNote2)==95&&m.p->getInt(kPadLearn)==0&&m.p->getInt(kLearnStatus)==16&&m.p->activeVoices()==0);
  m.p->set(kMainChordPage,3);m.block();auto state=m.p->internalSaveState();Rig restored;assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();restored.block();assert(restored.p->getInt(kSlotCount)==3&&restored.p->getInt(kMainChordPage)==3&&restored.p->getInt(kPadNote16)==81&&restored.p->getInt(kRoot16)==62);
  // Loading a genuine 91-parameter prefix into an already edited instance resets all additions.
  size_t pos=12;for(int i=0;i<91;++i)pos+=1+state[pos]+8;state.resize(pos+4);for(int j=0;j<4;++j){state[8+j]=j?0:91;state[pos+j]=0;}
  assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();assert(restored.p->getInt(kSlotCount)==0&&restored.p->getInt(kMainChordPage)==0&&restored.p->getInt(kChordOctave16)==2&&restored.p->getInt(kRoot16)==57);
 }
 std::cout<<"PASS: 16 independent slots, physical/touch ownership, hidden-bank feedback, shrink/expand, learn16 and 91-param migration\n";
 // Changing a bank's preset only replaces its own four roots/types.
 {Rig m;m.p->set(kSlotCount,3);m.p->set(kRoot1,61);m.p->set(kRoot5,62);m.p->set(kRoot13,63);m.p->set(kVoicing9,4);m.p->set(kChordOctave9,3);m.p->set(kPadNote9,99);m.block();
  m.p->set(kMainChordPage,2);m.p->set(kChordNext,1);m.block();assert(m.p->getInt(kProgression3)==1&&m.p->getInt(kProgression)==0&&m.p->getInt(kProgression2)==0&&m.p->getInt(kProgression4)==0);
  assert(m.p->getInt(kRoot9)==kProgressions[1][0][0]&&m.p->getInt(kQuality9)==kProgressions[1][0][1]);assert(m.p->getInt(kRoot1)==61&&m.p->getInt(kRoot5)==62&&m.p->getInt(kRoot13)==63);
  assert(m.p->getInt(kVoicing9)==4&&m.p->getInt(kChordOctave9)==3&&m.p->getInt(kPadNote9)==99&&m.p->getInt(kSlotCount)==3);
  auto state=m.p->internalSaveState();Rig r;assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();assert(r.p->getInt(kProgression3)==1&&r.p->getInt(kRoot1)==61&&r.p->getInt(kRoot9)==kProgressions[1][0][0]);
 }
 std::cout<<"PASS: independent bank presets, visible-page arrows, musical settings preservation and state round-trip\n";
 // Saving removes transient UI gates from state, releases screen auditions on
 // the audio thread, and leaves a held physical MIDI chord sounding.
 {Rig m;m.p->set(kSlotCount,3);m.p->set(kInputMode,1);m.p->set(kPadMap,1);m.p->set(kAmpRelease,1);m.block();
  m.block({mpc::MidiEvent::noteOn(0,m.p->getInt(kPadNote16),100)});m.p->set(kTouch1,1);m.block();assert(m.p->activeVoices()==10);
  auto state=m.p->internalSaveState();assert(m.p->getInt(kTouch1)==0&&m.p->getInt(kChordPlayingPage4)==0);for(int b=0;b<40;++b)m.block();assert(m.p->activeVoices()==5);
  Rig r;assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();assert(r.p->activeVoices()==0&&r.p->getInt(kTouch1)==0);
 }
 // Editor Q-Links mirror the visible four-slot bank; direct slot parameters
 // remain independent, and restored mirror values cannot overwrite real chords.
 {Rig m;m.p->set(kSlotCount,3);m.p->set(kMainChordPage,2);m.block();assert(m.p->getInt(kEditRoot1)==m.p->getInt(kRoot9));
  m.p->set(kEditRoot1,64);m.p->set(kEditQuality1,3);m.p->set(kEditVoicing1,4);m.p->set(kEditOctave1,4);m.block();
  assert(m.p->getInt(kRoot9)==64&&m.p->getInt(kQuality9)==3&&m.p->getInt(kVoicing9)==4&&m.p->getInt(kChordOctave9)==4&&m.p->getInt(kRoot1)==50);
  m.p->set(kRoot9,65);m.block();assert(m.p->getInt(kEditRoot1)==65);
  m.p->set(kMainChordPage,3);m.block();m.p->set(kEditRoot4,66);m.block();assert(m.p->getInt(kRoot16)==66&&m.p->getInt(kRoot9)==65);
  auto state=m.p->internalSaveState();Rig r;assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();assert(r.p->getInt(kRoot9)==65&&r.p->getInt(kRoot16)==66&&r.p->getInt(kEditRoot4)==66);
  m.p->set(kSlotCount,0);m.block();assert(m.p->getInt(kMainChordPage)==0&&m.p->getInt(kEditRoot1)==m.p->getInt(kRoot1));
 }
 std::cout<<"PASS: static editor mirrors follow selected bank, edit all four fields, read back slot edits and preserve state\n";
 // Exact pitches: all Cmaj9 inversions put the named chord member in the bass.
 {const int expected[5][5]={{48,52,55,59,62},{52,55,59,60,62},{55,59,60,62,64},{59,60,62,64,67},{62,64,67,71,72}};
  for(int inversion=0;inversion<5;++inversion)for(int octave=-2;octave<=2;++octave){std::array<int,6> notes{};const int v=inversion<2?inversion:inversion+1;
   assert(donut::chordNotes(48,3,v,octave,notes)==5);for(int n=0;n<5;++n)assert(notes[n]==expected[inversion][n]+12*octave);}
  std::array<int,6> notes{};assert(donut::chordNotes(48,3,2,0,notes)==5);assert((notes==std::array<int,6>{48,55,62,64,71,0}));
 }
 // Only real inversions remain selectable when changing to a shorter chord.
 for(int quality=0;quality<11;++quality){Rig m;m.p->set(kQuality1,quality);m.p->set(kVoicing1,5);m.block();assert(m.p->getInt(kVoicing1)==donut::chordToneCount(quality));}
 // v0.1.16 chunks had 87 parameters and voicing normalised to a three-choice range.
 {Rig old;old.p->set(kVoicing1,1);old.p->set(kVoicing2,2);std::vector<uint8_t> state={'M','P','C','P'};
  auto u32=[&](uint32_t v){for(int n=0;n<4;++n)state.push_back(uint8_t(v>>(8*n)));};auto f32=[&](float f){uint32_t v;std::memcpy(&v,&f,4);u32(v);};u32(2);u32(87);
  for(int index=0;index<87;++index){const auto& id=old.p->param(index).id;state.push_back(uint8_t(id.size()));state.insert(state.end(),id.begin(),id.end());f32(old.p->get(index));
   const bool voice=index==kVoicing1||index==kVoicing2||index==kVoicing3||index==kVoicing4;f32(voice?old.p->get(index)/2.f:old.p->getNormalised(index));}u32(0);
  Rig restored;for(int slot=0;slot<4;++slot)restored.p->set(kChordOctave1+slot,0);assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();assert(restored.p->getInt(kVoicing1)==1&&restored.p->getInt(kVoicing2)==2);
  for(int slot=0;slot<4;++slot)assert(restored.p->getInt(kChordOctave1+slot)==2);
 }
 // DRIFT is neutral at zero, bounded, and contains slow and fast pitch movement.
 {WowFlutter off,full,half;off.prepare(3000);full.prepare(3000);half.prepare(3000);
  double slow=0,fast=0;float peak=0;
  for(int i=0;i<60000;++i){float a=full.process(100),b=half.process(50);assert(off.process(0)==0);assert(std::isfinite(a)&&std::abs(a)<=.431f);assert(std::abs(b-a*.25f)<1e-6f);peak=std::max(peak,std::abs(a));slow+=a*std::sin(2*3.141592653589793*.43*i/3000);fast+=a*std::sin(2*3.141592653589793*6.73*i/3000+2*3.141592653589793*.61);}
  assert(peak>.3f&&std::abs(slow)>1000&&std::abs(fast)>500);
 }
 {Rig a,b;b.p->set(kDrift,100);b.p->reset();float difference=0;
  a.block({mpc::MidiEvent::noteOn(0,69,100)});b.block({mpc::MidiEvent::noteOn(0,69,100)});
  for(int n=0;n<100;++n){a.block({},127);b.block({},127);for(int i=0;i<127;++i)difference+=std::abs(a.l[i]-b.l[i]);}
  assert(difference>1);auto state=b.p->internalSaveState();b.p->set(kDrift,0);assert(b.p->internalLoadState(state.data(),state.size()));assert(b.p->get(kDrift)==100);
 }

 // Nonconsecutive pad notes work with stock-scale layouts; learn captures silently.
 {Rig m;m.p->set(kInputMode,2);m.p->set(kPadMap,1);m.block();
  for(int note:{24,26,28,29}){m.block({mpc::MidiEvent::noteOn(0,note,100)});assert(m.p->activeVoices()==5);m.block({mpc::MidiEvent::noteOff(0,note)});m.finish();}
  m.p->set(kPadLearn,2);m.block({mpc::MidiEvent::noteOn(0,65,100)});assert(m.p->getInt(kPadNote2)==65&&m.p->getInt(kPadLearn)==0&&m.p->activeVoices()==0);
  m.block({mpc::MidiEvent::noteOff(0,65)});m.block({mpc::MidiEvent::noteOn(0,65,100)});assert(m.p->activeVoices()==5);
  m.p->set(kPadNote2,66);m.block();assert(m.p->activeVoices()==0);
  m.p->set(kPadLearn,2);m.block({mpc::MidiEvent::noteOn(0,24,100)});assert(m.p->getInt(kPadNote2)==24&&m.p->getInt(kPadNote1)==66);
  auto state=m.p->internalSaveState();m.p->set(kPadNote1,70);assert(m.p->internalLoadState(state.data(),state.size()));m.p->reset();assert(m.p->getInt(kPadNote1)==66&&m.p->getInt(kPadMap)==1);
  // Build a real old 77-parameter SDK chunk to verify migration keeps legacy routing.
  size_t pos=12;for(int i=0;i<77;++i)pos+=1+state[pos]+8;
  state.resize(pos+4);for(int j=0;j<4;++j){state[8+j]=j?0:77;state[pos+j]=0;}
  assert(m.p->internalLoadState(state.data(),state.size()));m.p->reset();assert(m.p->getInt(kPadMap)==0);
 }
 // Each edited voicing must be shared by touch and mapped MIDI, and survive state restore.
 for(int slot=0;slot<4;++slot)for(int voicing=0;voicing<6;++voicing){
  Rig touch,midi;
  for(auto* r:{&touch,&midi}){r->p->set(kInputMode,2);r->p->set(kTrigger,48);r->p->set(kVoicing1+slot*3,float(voicing));r->p->set(kChordOctave1+slot,float(voicing%5));r->p->reset();r->block();}
  auto state=midi.p->internalSaveState();midi.p->set(kVoicing1+slot*3,float((voicing+1)%6));
  assert(midi.p->internalLoadState(state.data(),state.size()));midi.p->reset();midi.block();
  assert(midi.p->getInt(kVoicing1+slot*3)==voicing&&midi.p->getInt(kChordOctave1+slot)==voicing%5);
  touch.p->set(kTouch1+slot,1);touch.block();midi.block({mpc::MidiEvent::noteOn(0,48+slot,100)});
  assert(touch.p->activeVoices()==midi.p->activeVoices());
  for(int b=0;b<20;++b){touch.block();midi.block();}
  touch.p->set(kTouch1+slot,0);touch.block();midi.block({mpc::MidiEvent::noteOff(0,48+slot)});
  touch.finish();midi.finish();
 }
 // Pair selection preserves musical settings, tracks legacy wave edits and host programs.
 {Rig m;m.p->set(kCutoff,1234);m.p->set(kDetune,9);m.p->set(kOscPair,11);m.block();
  assert(m.p->getInt(kWave1)==2&&m.p->getInt(kWave2)==3);
  assert(std::abs(m.p->get(kCutoff)-1234)<.1f&&std::abs(m.p->get(kDetune)-9)<.01f);
  m.p->set(kWave1,1);m.block();assert(m.p->getInt(kOscPair)==7);
  m.p->setProgram(40);m.p->reset();m.block();assert(m.p->getInt(kOscPair)==m.p->getInt(kWave1)*4+m.p->getInt(kWave2));
  m.p->set(kTremDepth,65);m.p->set(kTremRate,6);auto state=m.p->internalSaveState();
  m.p->set(kTremDepth,0);assert(m.p->internalLoadState(state.data(),state.size()));m.p->reset();m.block();assert(std::abs(m.p->get(kTremDepth)-65)<.01f&&std::abs(m.p->get(kTremRate)-6)<.01f);
  m.p->set(kTouch1,1);m.block();assert(m.p->activeVoices()==5);m.p->set(kTouch1,0);m.block();m.finish();}
 // At zero depth independent tremolo is exactly bypassed; at full depth it attenuates
 // a sustained tone while preserving the existing filter LFO parameter values.
 {Rig dry,wet;
  for(auto* r:{&dry,&wet}){r->p->set(kAmpSustain,100);r->p->set(kChorus,0);r->p->set(kMotion,0);r->p->set(kSpace,0);r->p->set(kDelay,0);r->p->set(kTarget,1);r->p->set(kDepth,10);r->p->set(kTremRate,4);r->p->reset();}
  dry.block({mpc::MidiEvent::noteOn(0,60,100)});wet.block({mpc::MidiEvent::noteOn(0,60,100)});
  for(int j=0;j<100;++j){dry.block();wet.block();for(int i=0;i<128;++i)assert(dry.l[i]==wet.l[i]);}
  wet.p->set(kTremDepth,100);double a=0,b=0;
  for(int j=0;j<400;++j){dry.block();wet.block();if(j>50)for(int i=0;i<128;++i){a+=dry.l[i]*dry.l[i];b+=wet.l[i]*wet.l[i];}}
  assert(b>a*.1&&b<a*.8);assert(wet.p->getInt(kTarget)==1&&wet.p->get(kDepth)==10);
  wet.p->set(kFXPreset,1);wet.block();assert(wet.p->get(kTremDepth)==0);}
 // Touch chords work in Notes mode and are independent of MIDI ownership.
 {Rig m;m.p->set(kTouch1,1);m.block();assert(m.p->activeVoices()==5);
  m.block({mpc::MidiEvent::noteOn(0,60,100)});assert(m.p->activeVoices()==6);
  m.block({mpc::MidiEvent::noteOff(0,60)});for(int i=0;i<1800;++i)m.block();assert(m.p->activeVoices()==5);
  m.p->set(kTouchStop,1);m.block();m.finish();assert(m.p->getInt(kTouch1)==0);
  m.p->set(kTouch2,1);m.block();assert(m.p->activeVoices()==5);
  m.block({mpc::MidiEvent::cc(0,123,0)});m.finish();assert(m.p->getInt(kTouch2)==0);
  m.p->set(kTouch1,1);m.block();m.p->reset();m.block();assert(m.p->activeVoices()==0);
  m.p->set(kFXPreset,5);m.block();assert(m.p->getInt(kDelayDivision)==2&&m.p->get(kFeedback)>40);
  m.p->set(kFXPreset,1);m.block();assert(m.p->get(kChorus)==0&&m.p->get(kDelay)==0&&m.p->get(kMotion)==0);}
 {Rig panic;panic.p->set(kTouch1,1);panic.block({mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::cc(0,64,127)});assert(panic.p->activeVoices()>0);panic.p->set(kTouchStop,1);panic.block();assert(panic.p->activeVoices()==0);assert(panic.p->getInt(kTouch1)==0);}
 // Factory v2 low-cut is audible, bypassable and owned by each sound.
 {Rig off,bass,cut;for(auto* m:{&off,&bass,&cut}){m->p->setProgram(0);m->p->set(kWave1,0);m->p->set(kAmpAttack,1);m->p->set(kAmpSustain,100);m->p->set(kTone,0);m->p->set(kCutoff,18000);m->p->reset();}
 bass.p->set(kLowCut,18);cut.p->set(kLowCut,150);double e0=0,e1=0,e2=0;
 off.block({mpc::MidiEvent::noteOn(0,33,100)});bass.block({mpc::MidiEvent::noteOn(0,33,100)});cut.block({mpc::MidiEvent::noteOn(0,33,100)});
 for(int j=0;j<800;++j){off.block();bass.block();cut.block();if(j>300)for(int i=0;i<128;++i){e0+=off.l[i]*off.l[i];e1+=bass.l[i]*bass.l[i];e2+=cut.l[i]*cut.l[i];}}
 assert(e1>e0*.85&&e2<e0*.04);
 for(int i=0;i<64;++i){off.p->setProgram(i);assert(off.p->get(kLowCut)==kPatchExtras[i][0]);assert(off.p->get(kDrift)==kPatchExtras[i][1]);}
 off.p->setProgram(0);assert(off.p->get(kLowCut)==0&&off.p->get(kTremDepth)==0&&off.p->get(kChorus)==0&&off.p->get(kDelay)==0);}
 // Held editor audition follows Root/Type/Voicing without adding duplicate voices.
 {Rig m;m.p->setProgram(0);m.p->set(kWave1,0);m.p->set(kAmpAttack,1);m.p->set(kQuality1,9);m.p->set(kRoot1,48);m.p->reset();m.p->set(kTouch1,1);m.block();
 for(int i=0;i<100;++i)m.block();
 auto crossings=[&](){int n=0;float previous=0;for(int b=0;b<256;++b){m.block();for(int i=0;i<128;++i){if(previous<=0&&m.l[i]>0)++n;previous=m.l[i];}}return n;};
 const int low=crossings();m.p->set(kRoot1,60);m.block();assert(m.p->getInt(kTouch1)==1&&m.p->activeVoices()==3);const int high=crossings();assert(high>low*1.7&&high<low*2.3);
 m.p->set(kChordOctave1,3);m.block();const int octaveHigh=crossings();assert(octaveHigh>high*1.7&&octaveHigh<high*2.3);m.p->set(kChordOctave1,2);m.block();
 m.p->set(kQuality1,2);m.block();assert(m.p->activeVoices()==5&&m.p->getInt(kChordSlot)==0);
 m.p->set(kVoicing1,2);m.block();assert(m.p->activeVoices()==5);
 m.block({mpc::MidiEvent::noteOn(2,72,90)});assert(m.p->activeVoices()==6);
 m.p->set(kPadNote1,25);m.block();assert(m.p->getInt(kTouch1)==1&&m.p->activeVoices()==5);
 m.p->set(kTouch1,0);m.block();m.finish();
 for(int slot=0;slot<4;++slot){m.p->set(kRoot1+slot*3,55+slot);m.block();assert(m.p->getInt(kChordSlot)==slot);}}
 Rig t;
 t.p->set(kInputMode,1);t.block();t.block({mpc::MidiEvent::noteOn(0,36,100)});assert(t.p->activeVoices()==5);
 t.block({mpc::MidiEvent::noteOn(0,37,100)});assert(t.p->activeVoices()==10);
 t.block({mpc::MidiEvent::noteOff(0,36)});for(int i=0;i<1800;++i)t.block();assert(t.p->activeVoices()==5);
 t.block({mpc::MidiEvent::noteOff(0,37)});t.finish();
 t.p->reset();t.block({mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::cc(0,64,127)});
 t.block({mpc::MidiEvent::noteOff(0,36)});assert(t.p->activeVoices()==5);t.block({mpc::MidiEvent::cc(0,64,0)});t.finish();
 // Mode changes kill stale owners, and later note-offs cannot leave stuck voices.
 t.block({mpc::MidiEvent::noteOn(0,36,100)});t.p->set(kInputMode,0);t.block();assert(t.p->activeVoices()==0);
 t.p->set(kVoiceMode,1);t.block();t.block({mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::noteOn(0,64,90)});assert(t.p->activeVoices()==1);
 t.block({mpc::MidiEvent::noteOff(0,64)});assert(t.p->activeVoices()==1);t.block({mpc::MidiEvent::noteOff(0,60)});t.finish();
 // SPLIT keeps a single melodic voice without collapsing or trapping chords.
 for(int voiceMode:{0,1})for(int individual:{0,1}){
  Rig m;m.p->set(kInputMode,2);m.p->set(kVoiceMode,voiceMode);m.p->set(kPadMap,individual);m.p->set(kAmpRelease,5);m.block();
  const int pad=individual?24:36;
  m.block({mpc::MidiEvent::noteOn(0,pad,100),mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::noteOn(0,64,100)});
  assert(m.p->activeVoices()==6); // five chord tones plus one melody, not seven
  m.block({mpc::MidiEvent::noteOff(0,64)});assert(m.p->activeVoices()==6);
  m.block({mpc::MidiEvent::noteOff(0,60)});for(int n=0;n<100;++n)m.block();
  assert(m.p->activeVoices()==5); // fallback must not turn the held pad note into melody
  m.block({mpc::MidiEvent::noteOn(0,67,100),mpc::MidiEvent::noteOff(0,pad)});
  for(int n=0;n<100;++n)m.block();assert(m.p->activeVoices()==1);
  m.block({mpc::MidiEvent::noteOff(0,67)});m.finish();
 }
 // Poly SPLIT remains polyphonic, while CHORDS accepts complete chords in every voice mode.
 {Rig m;m.p->set(kInputMode,2);m.p->set(kVoiceMode,2);m.block();
  m.block({mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::noteOn(0,64,100)});assert(m.p->activeVoices()==7);
  m.block({mpc::MidiEvent::cc(0,120,0)});assert(m.p->activeVoices()==0);}
 for(int voiceMode:{0,1,2}){
  Rig m;m.p->set(kInputMode,1);m.p->set(kVoiceMode,voiceMode);m.block();m.block({mpc::MidiEvent::noteOn(0,36,100)});assert(m.p->activeVoices()==5);
  m.block({mpc::MidiEvent::noteOff(0,36)});m.finish();
 }
 // Pedal release/reset frees MIDI chords and melody, but not a held screen pad.
 for(int controller:{64,121}){
  Rig m;m.p->set(kInputMode,2);m.p->set(kVoiceMode,1);m.p->set(kAmpRelease,5);m.block();m.p->set(kTouch2,1);m.block();
  m.block({mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::cc(0,64,127)});assert(m.p->activeVoices()==11);
  m.block({mpc::MidiEvent::noteOff(0,36),mpc::MidiEvent::noteOff(0,60)});assert(m.p->activeVoices()==11);
  m.block({mpc::MidiEvent::cc(1,64,0)});for(int n=0;n<100;++n)m.block();assert(m.p->activeVoices()==11);
  m.block({mpc::MidiEvent::cc(0,controller,0)});for(int n=0;n<100;++n)m.block();assert(m.p->activeVoices()==5);
  m.p->set(kTouch2,0);m.block();m.finish();
 }
 // Heavy chord overlap cannot steal SPLIT's reserved monophonic melody voice.
 {Rig m;m.p->set(kInputMode,2);m.p->set(kVoiceMode,1);m.p->set(kAmpRelease,5);m.block();
  m.block({mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::noteOn(0,37,100),mpc::MidiEvent::noteOn(0,38,100),mpc::MidiEvent::noteOn(0,39,100)});assert(m.p->activeVoices()==16);
  m.block({mpc::MidiEvent::noteOff(0,36),mpc::MidiEvent::noteOff(0,37),mpc::MidiEvent::noteOff(0,38),mpc::MidiEvent::noteOff(0,39)});for(int n=0;n<100;++n)m.block();assert(m.p->activeVoices()==1);
  m.block({mpc::MidiEvent::noteOff(0,60)});m.finish();}
 // A separated legato note retriggers after a gap, even while the previous release is audible.
 for(int input:{0,2}){
  Rig m;m.p->set(kInputMode,input);m.p->set(kVoiceMode,1);m.p->set(kAmpRelease,20);m.block();
  m.block({mpc::MidiEvent::noteOn(0,60,100)});for(int n=0;n<20;++n)m.block();
  m.block({mpc::MidiEvent::noteOff(0,60)});assert(m.p->activeVoices()==1);
  m.block({mpc::MidiEvent::noteOn(0,64,100)});float peak=0;for(int n=0;n<100;++n)peak=m.block();
  assert(m.p->activeVoices()==1&&peak>1e-5f);m.block({mpc::MidiEvent::noteOff(0,64)});m.finish();
 }
 std::cout<<"PASS: SPLIT mono/legato melody, independent chords, pedal ownership, stealing and detached-note retrigger\n";
 // Sound choice preserves a customised progression and chord root; host programs too.
 t.p->set(kProgression,5);t.block();t.p->set(kRoot1,61);t.p->set(kChordOctave1,4);t.p->set(kSoundPreset,9);t.block();assert(t.p->getInt(kProgression)==5&&t.p->getInt(kRoot1)==61&&t.p->getInt(kChordOctave1)==4);
 t.p->setProgram(12);t.p->reset();assert(t.p->getInt(kProgression)==5&&t.p->getInt(kRoot1)==61&&t.p->getInt(kChordOctave1)==4);
 // Snapshot restoration preserves custom edits rather than reapplying the factory patch.
 t.p->set(kCutoff,1234);auto snapshot=t.p->internalSaveState();t.p->set(kCutoff,50);t.p->set(kRoot1,25);assert(t.p->internalLoadState(snapshot.data(),snapshot.size()));t.p->reset();t.block();assert(std::abs(t.p->get(kCutoff)-1234)<.1f&&t.p->getInt(kRoot1)==61);
 // Momentary arrows reset themselves: repeated taps advance once each and wrap.
 t.p->set(kSoundPreset,0);t.block();t.p->set(kSoundPrev,1);t.block();assert(t.p->getInt(kSoundPreset)==63);t.block();assert(t.p->getInt(kSoundPreset)==63);
 assert(t.p->getInt(kSoundPrev)==0);
 t.p->set(kSoundPrev,1);t.block();assert(t.p->getInt(kSoundPreset)==62);
 t.p->set(kSoundNext,1);t.block();assert(t.p->getInt(kSoundPreset)==63);
 t.p->set(kSoundNext,1);t.block();assert(t.p->getInt(kSoundPreset)==0);
 t.p->set(kProgression,0);t.block();
 t.p->set(kChordPrev,1);t.block();assert(t.p->getInt(kProgression)==31);
 t.p->set(kChordPrev,1);t.block();assert(t.p->getInt(kProgression)==30);
 t.p->set(kChordNext,1);t.block();assert(t.p->getInt(kProgression)==31);
 t.p->set(kChordNext,1);t.block();assert(t.p->getInt(kProgression)==0);
 assert(t.p->getInt(kSoundPreset)==0&&t.p->getInt(kChordNext)==0);
 // Spread cancellation: release before the last chord tone starts.
 t.p->set(kInputMode,1);t.p->set(kSpread,40);t.block();t.block({mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::noteOff(0,36,30)});t.finish();
 // MIDI offsets: silence before note-on, audio after attack.
 t.p->set(kInputMode,0);t.p->setProgram(0);t.p->reset();t.block({mpc::MidiEvent::noteOn(0,60,110,64)});for(int i=0;i<64;++i)assert(t.l[i]==0&&t.r[i]==0);
 // Pedals are channel-local, including mono fallback.
 {Rig m;m.p->set(kVoiceMode,1);m.block();
  m.block({mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::cc(0,64,127),mpc::MidiEvent::noteOff(0,60)});
  m.block({mpc::MidiEvent::cc(1,64,0)});for(int i=0;i<1000;++i)m.block();assert(m.p->activeVoices()==1);
  m.block({mpc::MidiEvent::cc(0,64,0)});m.finish();}
 {Rig m;m.p->set(kVoiceMode,1);m.block();
  m.block({mpc::MidiEvent::noteOn(0,60,100),mpc::MidiEvent::noteOn(0,64,100),mpc::MidiEvent::cc(0,64,127),mpc::MidiEvent::noteOff(0,64),mpc::MidiEvent::cc(0,121,0)});
  for(int i=0;i<1000;++i)m.block();assert(m.p->activeVoices()==1);
  m.block({mpc::MidiEvent::noteOff(0,60)});m.finish();}
 // Both preset routes preserve the user's master volume and share one program index.
 t.p->set(kVolume,-20);t.p->set(kSoundPreset,1);t.block();assert(t.p->currentProgram()==1);
 assert(std::abs(t.p->get(kVolume)+20)<.001f);t.p->setProgram(2);t.p->reset();t.block();
 assert(t.p->currentProgram()==2&&std::abs(t.p->get(kVolume)+20)<.001f);
 t.p->set(kSync,1);t.p->set(kDivision,2);assert(t.p->displayText(kRate)=="Sync 1/4");
 t.p->set(kSync,0);assert(t.p->displayText(kRate).find("Hz")!=std::string::npos);
 t.p->set(kOctave,1.5f);assert(t.p->displayText(kOctave)=="2 oct");
 t.p->set(kTrigger,56.5f);assert(t.p->displayText(kTrigger)=="57 MIDI");
 // All presets render finite non-silent audio. This is not artistic listening approval.
 float minPeak=1,maxPeak=0;
 for(int patch=0;patch<64;++patch){t.p->setProgram(patch);t.p->set(kInputMode,0);t.p->reset();float peak=t.block({mpc::MidiEvent::noteOn(0,60,110)});for(int i=0;i<400;++i)peak=std::max(peak,t.block({},(i%2)?32:512));assert(peak>1e-5f);minPeak=std::min(minPeak,peak);maxPeak=std::max(maxPeak,peak);}
 // All progressions support four overlapping pads with bounded voice allocation.
 t.p->set(kInputMode,1);t.p->reset();for(int prog=0;prog<32;++prog){t.p->set(kProgression,prog);t.block();for(int pad=0;pad<4;++pad)t.block({mpc::MidiEvent::noteOn(0,36+pad,100)});assert(t.p->activeVoices()<=16);for(int pad=0;pad<4;++pad)t.block({mpc::MidiEvent::noteOff(0,36+pad)});t.finish();}
 std::cout<<"PASS: channel-isolated pedals, CC121 mono fallback, unified presets/volume, synced rate display;  MIDI/chord ownership, sustain, mono fallback, mode transitions, preset independence, state, arrows, spread cancellation, sample offsets, allocation-free rendering, 64 sounds and 32 progressions. Peak range "<<minPeak<<".."<<maxPeak<<"\n";
}
