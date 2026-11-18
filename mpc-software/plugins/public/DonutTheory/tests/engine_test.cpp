#include <cassert>
#include <iostream>
#include <memory>
#include <thread>
#include <new>
#include <cstdlib>
static bool forbidAlloc=false;
#if !__has_feature(address_sanitizer)
void* operator new(std::size_t n){if(forbidAlloc){std::cerr<<"allocation in audio callback\n";std::abort();}if(void* p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void* p)noexcept{std::free(p);}
void* operator new[](std::size_t n){return ::operator new(n);}
void operator delete[](void* p)noexcept{std::free(p);}
#endif
#define DONUT_VOICE_TEST
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
 // A single strum rolls low-to-high; release cancels all delayed tones.
 {Rig roll;roll.p->set(kInputMode,1);roll.p->set(kSpread,20);roll.p->set(kAmpAttack,1);roll.p->set(kAmpRelease,10);roll.block({mpc::MidiEvent::noteOn(0,36,100)});int audible=0;for(auto v:roll.p->voiceSnapshots())if(v.owner==36&&v.amplitude>0)++audible;assert(audible==1);
 for(int i=0;i<7;++i)roll.block();audible=0;for(auto v:roll.p->voiceSnapshots())if(v.owner==36&&v.amplitude>0)++audible;assert(audible==2);roll.block({mpc::MidiEvent::noteOff(0,36)});for(int i=0;i<100;++i)roll.block();assert(!roll.p->activeVoices());}
 // With chord legato, moved pitches roll in without retriggering common tones.
 for(float glide:{0.f,80.f}){Rig roll;roll.p->set(kInputMode,1);roll.p->set(kChordGlide,1);roll.p->set(kGlide,glide);roll.p->set(kAmpRelease,10);roll.p->set(kChordOctave2,1);roll.p->set(kVoicing2,3);roll.block({mpc::MidiEvent::noteOn(0,36,100)});for(int i=0;i<100;++i)roll.block();roll.p->set(kSpread,20);roll.block({mpc::MidiEvent::noteOn(0,37,100)});
 auto pitch=[&](int note){for(auto v:roll.p->voiceSnapshots())if(v.owner==37&&v.note==note&&v.held)return v.pitch;return -1000.f;};assert(pitch(55)==50&&pitch(59)==60&&pitch(64)==64);
 for(int i=0;i<7;++i)roll.block();assert(pitch(55)>50&&pitch(55)<=55&&pitch(59)==60&&pitch(64)==64);for(int i=0;i<20;++i)roll.block();assert(pitch(59)<60&&pitch(59)>=59);
 roll.block({mpc::MidiEvent::cc(0,120,0)});for(int i=0;i<100;++i)roll.block();assert(!roll.p->activeVoices());}
 std::cout<<"PASS: one-shot ascending STRUM, delayed-tone cancellation and staggered glide with fixed common tones\n";

 // All expanded selectors preserve the legacy ranges and all unrelated slots.
 {Rig h;h.p->set(kSlotCount,3);h.block();
 for(int bank=0;bank<4;++bank){h.p->set(kMainChordPage,bank);h.block();for(int row=0;row<4;++row)for(int type=0;type<32;++type){h.p->setNormalised(kHarmonyEdit1+row,type/31.f);h.block();assert(h.p->getInt(kChordQualities[bank*4+row])==type);std::array<int,6> n;assert(donut::chordNotes(48,type,0,0,n)==donut::kTypeSizes[type]);}}
 for(int bank=0;bank<4;++bank)for(int n=1;n<=96;++n){const int preset=n%96;h.p->set(kMainChordPage,bank);h.block();std::array<int,16> roots{},types{},pads{};for(int i=0;i<16;++i){roots[i]=h.p->getInt(kChordRoots[i]);types[i]=h.p->getInt(kChordQualities[i]);pads[i]=h.p->getInt(kChordPadNotes[i]);}h.p->set(kHarmonyCurrent,preset);h.block();
  for(int i=0;i<16;++i){assert(h.p->getInt(kChordPadNotes[i])==pads[i]);if(i/4==bank){assert(h.p->getInt(kChordRoots[i])==kProgressions[preset][i%4][0]&&h.p->getInt(kChordQualities[i])==kProgressions[preset][i%4][1]);}else assert(h.p->getInt(kChordRoots[i])==roots[i]&&h.p->getInt(kChordQualities[i])==types[i]);}}
 h.p->set(kHarmonyPreset1,95);h.block();h.p->set(kHarmonyType1,31);h.p->set(kRoot1,61);h.block();const float oldType=h.p->getNormalised(kQuality1),oldPreset=h.p->getNormalised(kProgression);auto state=h.p->internalSaveState();Rig r;assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();r.p->setNormalised(kQuality1,oldType);r.p->setNormalised(kProgression,oldPreset);r.block();assert(r.p->getInt(kHarmonyType1)==31&&r.p->getInt(kHarmonyPreset1)==95&&r.p->getInt(kRoot1)==61);
 for(int i=0;i<200;++i)r.block();r.p->set(kQuality1,6);r.block();assert(r.p->getInt(kHarmonyType1)==6);
 Rig old;old.p->set(kProgression,7);old.block();old.p->set(kQuality1,4);old.p->set(kRoot1,61);old.block();state=old.p->internalSaveState();size_t end=12;for(int i=0;i<242;++i)end+=1+state[end]+8;state.resize(end+4);state[8]=242;state[9]=state[10]=state[11]=0;for(int b=0;b<4;++b)state[end+b]=0;
 Rig migrated;assert(migrated.p->internalLoadState(state.data(),state.size()));migrated.p->reset();migrated.block();assert(migrated.p->getInt(kHarmonyType1)==4&&migrated.p->getInt(kHarmonyPreset1)==7&&migrated.p->getInt(kRoot1)==61&&!migrated.p->getBool(kChordGlide));
 }
 std::cout<<"PASS: 32 types across all editor banks, 96 presets across4banks, source isolation, legacy242 migration and normalized replay protection\n";

 {Rig g;g.p->set(kInputMode,1);g.p->set(kChordGlide,1);g.p->set(kGlide,80);g.p->set(kAmpAttack,100);g.p->set(kAmpRelease,15);g.p->set(kChordOctave2,1);g.p->set(kVoicing2,3);g.block({mpc::MidiEvent::noteOn(0,36,100)});for(int i=0;i<400;++i)g.block();
 auto before=g.p->voiceSnapshots();g.block({mpc::MidiEvent::noteOn(0,37,100)});auto after=g.p->voiceSnapshots();int held=0,sliding=0,common=0;
 for(int i=0;i<16;++i)if(after[i].owner==37&&after[i].held){++held;if(std::abs(after[i].pitch-after[i].note)>.1f)++sliding;if(before[i].held&&before[i].note==after[i].note){++common;assert(std::abs(after[i].pitch-before[i].pitch)<.001f&&after[i].amplitude>.1f);}}
 assert(held==5&&sliding==2&&common==3);
 g.block({mpc::MidiEvent::noteOff(0,37)});held=0;for(auto v:g.p->voiceSnapshots())if(v.held){assert(v.owner==36);++held;}assert(held==5);g.block({mpc::MidiEvent::noteOff(0,36)});for(int i=0;i<100;++i)g.block();assert(!g.p->activeVoices());
 // Non-overlapping presses must snap even with a long release and Always glide.
 g.p->set(kGlideLegato,0);g.p->set(kAmpRelease,1000);g.block({mpc::MidiEvent::noteOn(0,36,100)});g.block({mpc::MidiEvent::noteOff(0,36),mpc::MidiEvent::noteOn(0,37,100)});for(auto v:g.p->voiceSnapshots())if(v.owner==37&&v.held)assert(std::abs(v.pitch-v.note)<.001f);g.block({mpc::MidiEvent::cc(0,120,0)});assert(!g.p->activeVoices());
 // Touch-to-touch, MIDI-to-touch and native bank changes retain proper owners.
 g.p->set(kAmpRelease,15);g.p->set(kSlotCount,1);g.block();g.p->set(kTouch1,1);g.block();g.p->set(kTouch2,1);g.block();g.p->set(kTouch2,0);g.block();for(auto v:g.p->voiceSnapshots())if(v.held)assert(v.owner==2048);g.p->set(kTouch1,0);for(int i=0;i<100;++i)g.block();assert(!g.p->activeVoices());
 g.block({mpc::MidiEvent::noteOn(0,36,100)});g.p->set(kTouch2,1);g.block();g.p->set(kMainChordPage,1);g.block();held=0;for(auto v:g.p->voiceSnapshots())if(v.held){assert(v.owner==36);++held;}assert(held==5);g.block({mpc::MidiEvent::noteOff(0,36)});for(int i=0;i<100;++i)g.block();assert(!g.p->activeVoices());
 // Sustain and repeated note-on counters must finish without stranded notes.
 g.block({mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::cc(0,64,127)});g.block({mpc::MidiEvent::noteOn(0,37,100)});g.block({mpc::MidiEvent::noteOff(0,37)});g.block({mpc::MidiEvent::cc(0,64,0)});for(auto v:g.p->voiceSnapshots())if(v.held)assert(v.owner==36);g.block({mpc::MidiEvent::noteOff(0,36)});
 g.block({mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::noteOn(0,36,100),mpc::MidiEvent::noteOn(1,37,100)});g.block({mpc::MidiEvent::noteOff(0,36)});g.block({mpc::MidiEvent::noteOff(0,36),mpc::MidiEvent::noteOff(1,37)});for(int i=0;i<100;++i)g.block();assert(!g.p->activeVoices());
 }
 std::cout<<"PASS: chord glide common tones, exact nearest moves, overlap-only pitch, touch/MIDI fallback, page ownership, sustain, channels and repeated notes\n";

 // Preview is isolated from stored chords; only APPLY mutates voice/octave.
 {for(int all=0;all<2;++all){Rig m;m.p->set(kSlotCount,3);m.p->set(kMainChordPage,2);m.p->set(kSmoothScope,all);m.p->set(kAmpRelease,10);m.block();
  std::array<std::array<int,5>,16> before{};for(int slot=0;slot<16;++slot)for(int f=0;f<5;++f)before[slot][f]=m.p->getInt(f<4?(f==0?kChordRoots[slot]:f==1?kChordQualities[slot]:f==2?kChordVoicings[slot]:kChordOctaves[slot]):kChordPadNotes[slot]);
  m.p->set(kSmoothOpen,1);m.block();std::thread ui([&]{m.p->flushHostUpdates();});ui.join();assert(m.p->getInt(kSmoothStatus)==1||m.p->getInt(kSmoothStatus)==5);
  const int start=all?0:8,count=all?16:4;std::array<int,16> top{};for(int i=0;i<count;++i)top[i]=m.p->getInt(kSmoothRow1+i);
  m.p->set(kSmoothPreview,1);m.block();assert(m.p->getInt(kSmoothPreview)==1&&m.p->activeVoices()>=3);
  for(int slot=0;slot<16;++slot){assert(m.p->getInt(kChordRoots[slot])==before[slot][0]);assert(m.p->getInt(kChordVoicings[slot])==before[slot][2]&&m.p->getInt(kChordOctaves[slot])==before[slot][3]);}
  // Stop/close releases preview and cannot modify the original progression.
  m.p->set(kSmoothPreview,0);for(int b=0;b<50;++b)m.block();assert(m.p->activeVoices()==0);
  m.p->set(kSmoothApply,1);m.block();assert(!m.p->getInt(kSmoothOpen)&&m.p->getInt(kSmoothStatus)==3);
  for(int slot=0;slot<16;++slot){assert(m.p->getInt(kChordRoots[slot])==before[slot][0]&&m.p->getInt(kChordQualities[slot])==before[slot][1]&&m.p->getInt(kChordPadNotes[slot])==before[slot][4]);
   if(slot<start||slot>=start+count)assert(m.p->getInt(kChordVoicings[slot])==before[slot][2]&&m.p->getInt(kChordOctaves[slot])==before[slot][3]);
   else{std::array<int,6> notes;const int n=donut::chordNotes(before[slot][0],before[slot][1],m.p->getInt(kChordVoicings[slot]),m.p->getInt(kChordOctaves[slot])-2,notes);assert(notes[n-1]==top[slot-start]);}}
  assert(m.p->getInt(kChordVoicings[start])==before[start][2]&&m.p->getInt(kChordOctaves[start])==before[start][3]);
 }
 // A new UI result may arrive before the next audio block. Cancel the old
 // copied audition even when the newly published result is already fresh.
 {Rig quick;quick.p->set(kSmoothOpen,1);quick.block();std::thread a([&]{quick.p->flushHostUpdates();});a.join();quick.p->set(kSmoothPreview,1);quick.block();assert(quick.p->getInt(kSmoothPreview));
 quick.p->set(kRoot1,51);std::thread b([&]{quick.p->flushHostUpdates();});b.join();quick.block();assert(!quick.p->getInt(kSmoothPreview));for(int i=0;i<300;++i)quick.block();assert(quick.p->activeVoices()==0);}
 // Automatic preview end keeps a separately held MIDI note; stale plans cannot apply.
 Rig m;m.p->set(kAmpRelease,10);m.p->set(kSmoothOpen,1);m.block();std::thread ui([&]{m.p->flushHostUpdates();});ui.join();m.block({mpc::MidiEvent::noteOn(0,92,100)});
 m.p->set(kSmoothPreview,1);m.block();for(int b=0;b<1100;++b)m.block();assert(!m.p->getInt(kSmoothPreview)&&m.p->activeVoices()==1);m.block({mpc::MidiEvent::noteOff(0,92)});for(int b=0;b<50;++b)m.block();assert(m.p->activeVoices()==0);
 m.p->set(kRoot1,51);m.p->set(kSmoothApply,1);m.block();assert(m.p->getInt(kSmoothOpen)&&m.p->getInt(kVoicing1)==0&&m.p->getInt(kChordOctave1)==2&&m.p->getInt(kSmoothStatus)==6);
 std::thread ui2([&]{m.p->flushHostUpdates();});ui2.join();m.p->set(kSmoothPreview,1);m.block();m.block({mpc::MidiEvent::cc(0,123,0)});assert(!m.p->getInt(kSmoothPreview));
 auto state=m.p->internalSaveState();Rig loaded;assert(loaded.p->internalLoadState(state.data(),state.size()));loaded.p->reset();loaded.block();assert(!loaded.p->getInt(kSmoothOpen)&&!loaded.p->getInt(kSmoothPreview)&&!loaded.p->getInt(kSmoothApply));
 }
 std::cout<<"PASS: scoped smooth APPLY preserves roots/types/MIDI/inactive slots, preview ownership/end/stop, stale-plan rejection and safe state\n";

 // Key reference never changes notes. Explicit confirmation moves all active roots only.
 {static_assert(kChordKeyRoot==208&&kTransposeDelta==219&&kNumParams==272,"Append-only key controls");
  for(int count=0;count<4;++count)for(int key=0;key<12;++key)for(int from=0;from<12;++from){Rig m;m.p->set(kSlotCount,count);m.block();
   std::array<std::array<float,5>,16> before{};
   for(int slot=0;slot<16;++slot){m.p->set(kChordRoots[slot],48+slot%12);m.p->set(kChordOctaves[slot],slot%5);m.p->set(kChordPadNotes[slot],80+slot);for(int f=0;f<5;++f)before[slot][f]=m.p->get(f==0?kChordRoots[slot]:f==1?kChordQualities[slot]:f==2?kChordVoicings[slot]:f==3?kChordOctaves[slot]:kChordPadNotes[slot]);}
   m.p->set(kChordKeyRoot,key);m.p->set(kChordKeyMode,key%2);m.p->set(kTransposeFrom,from);m.block();
   for(int slot=0;slot<16;++slot)assert(m.p->get(kChordRoots[slot])==before[slot][0]);
   // An Apply write without a visible preview is inert.
   m.p->set(kTransposeApply,1);m.block();for(int slot=0;slot<16;++slot)assert(m.p->get(kChordRoots[slot])==before[slot][0]);
   m.p->set(kTransposeOpen,1);m.block();int delta=(key-from+18)%12-6;if(delta==-6)delta=6;assert(m.p->getInt(kTransposeDelta)==delta);
   m.p->set(kTransposeApply,1);m.block();assert(!m.p->getInt(kTransposeOpen)&&!m.p->getInt(kTransposeApply));assert(m.p->getInt(kTransposeFrom)==key);
   for(int slot=0;slot<16;++slot){assert(m.p->get(kChordRoots[slot])==before[slot][0]+(slot<(count+1)*4?delta:0));for(int f=1;f<5;++f)assert(m.p->get(f==1?kChordQualities[slot]:f==2?kChordVoicings[slot]:f==3?kChordOctaves[slot]:kChordPadNotes[slot])==before[slot][f]);}
   // Reopening at the applied key cannot compound the same transposition.
   m.p->set(kTransposeOpen,1);m.p->set(kTransposeApply,1);m.block();assert(m.p->getInt(kTransposeStatus)==2);
  }
  Rig m;m.p->set(kRoot1,84);m.p->set(kChordKeyRoot,2);m.p->set(kTransposeOpen,1);m.p->set(kTransposeApply,1);m.block();assert(m.p->getInt(kRoot1)==84&&m.p->getInt(kRoot2)==55&&m.p->getInt(kTransposeOpen)==1&&m.p->getInt(kTransposeStatus)==3);
  m.p->set(kRoot1,24);m.p->set(kChordKeyRoot,11);m.p->set(kTransposeApply,1);m.block();assert(m.p->getInt(kRoot1)==24&&m.p->getInt(kTransposeStatus)==3);
  m.p->set(kChordKeyRoot,0);m.p->set(kRoot1,50);m.block();assert(m.p->getInt(kKeyDegree1)==9); // ii
  m.p->set(kSlotCount,2);m.p->set(kMainChordPage,2);m.p->set(kRoot9,49);m.block();assert(m.p->getInt(kKeyDegree1)==0);
  m.p->set(kChordKeyRoot,9);m.p->set(kChordKeyMode,1);m.p->set(kRoot9,57);m.block();assert(m.p->getInt(kKeyDegree1)==8); // i
  auto state=m.p->internalSaveState();Rig restored;assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();restored.block();assert(restored.p->getInt(kChordKeyRoot)==9&&restored.p->getInt(kChordKeyMode)==1&&!restored.p->getInt(kTransposeOpen));
  // Restore actual old 208-param state, not only the new schema defaults.
  size_t end=12;for(int i=0;i<208;++i)end+=1+state[end]+8;state.resize(end+4);state[8]=208;state[9]=state[10]=state[11]=0;for(int b=0;b<4;++b)state[end+b]=0;
  assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();restored.block();assert(restored.p->getInt(kChordKeyRoot)==0&&restored.p->getInt(kChordKeyMode)==0&&!restored.p->getInt(kTransposeApply));assert(restored.p->getInt(kRoot9)==57);
 }
 std::cout<<"PASS: 576 key/source/count transposes, reference isolation, whole-operation bounds, degree readback, state and old208 migration without render allocations\n";

 // Manual position hints survive presets/LEARN/state and never alter MIDI routing.
 {static_assert(kPadPosition1==192&&kPadPosition16==207&&kChordKeyRoot==208&&kNumParams==272,"Append-only pad positions");
  Rig m;for(int slot=0;slot<16;++slot){assert(m.p->getInt(kPadPosition1+slot)==0);assert(!m.p->param(kPadPosition1+slot).automatable);m.p->set(kPadPosition1+slot,float(slot*8+1));}
  m.p->setProgram(12);m.block();assert(m.p->getInt(kPadPosition1)==1&&m.p->getInt(kPadPosition16)==121);
  m.p->set(kPadLearn,1);m.block({mpc::MidiEvent::noteOn(0,75,100)});assert(m.p->getInt(kPadPosition1)==1&&m.p->getInt(kPadNote1)==75&&m.p->activeVoices()==0);
  auto state=m.p->internalSaveState();Rig r;assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();for(int slot=0;slot<16;++slot)assert(r.p->getInt(kPadPosition1+slot)==slot*8+1);
  size_t pos=12;for(int i=0;i<192;++i)pos+=1+state[pos]+8;state.resize(pos+4);for(int j=0;j<4;++j){state[8+j]=j?0:192;state[pos+j]=0;}
  assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();for(int slot=0;slot<16;++slot)assert(r.p->getInt(kPadPosition1+slot)==0);assert(r.p->getInt(kPadNote1)==75);
 }
 std::cout<<"PASS: manual pad locations, LEARN and sound independence, 272-param round-trip and 192-param migration\n";
 // Osc Mix endpoints isolate the other oscillator independently of SUB.
 {for(float mix:{0.f,100.f}){Rig a,b;
   for(auto* m:{&a,&b}){m->p->setProgram(0);m->p->set(kBody,40);m->p->set(kMix,mix);m->p->set(kWave1,0);m->p->set(kWave2,2);m->p->set(kAmpAttack,1);m->p->set(kAmpSustain,100);m->p->set(kCutoff,18000);m->p->reset();}
   b.p->set(mix==0?kWave2:kWave1,1);a.block();b.block();
   a.block({mpc::MidiEvent::noteOn(0,60,100)});b.block({mpc::MidiEvent::noteOn(0,60,100)});
   for(int block=0;block<120;++block){a.block();b.block();for(int i=0;i<128;++i)assert(std::abs(a.l[i]-b.l[i])<1e-6f);}
  }}
 std::cout<<"PASS: OSC Mix endpoints isolate OSC1/OSC2 independent of SUB, with finite stable output\n";
 // SUB is a true -12-semitone sine; the retired Body must not alter it or OSC mix.
 {auto spectral=[](float sub,float body){Rig m;m.p->setProgram(0);m.p->set(kSub,sub);m.p->set(kBody,body);m.p->set(kWave1,0);m.p->set(kWave2,0);m.p->set(kMix,0);m.p->set(kTone,0);m.p->set(kCutoff,18000);m.p->set(kAmpAttack,1);m.p->set(kAmpSustain,100);m.p->set(kAmpDecay,1);m.p->reset();m.block({mpc::MidiEvent::noteOn(0,57,100)});for(int b=0;b<100;++b)m.block();
   std::array<double,2> projection{};std::vector<float> audio;for(int b=0;b<345;++b){m.block();audio.insert(audio.end(),m.l,m.l+128);}for(size_t n=0;n<audio.size();++n){double phase=6.283185307179586*110*n/44100.;projection[0]+=audio[n]*std::sin(phase);projection[1]+=audio[n]*std::cos(phase);}return std::make_pair(std::hypot(projection[0],projection[1])/audio.size(),audio);};
  auto off=spectral(0,0),half=spectral(50,0),full=spectral(100,0),oldBody=spectral(100,100);assert(full.first>off.first*10&&half.first>off.first*5&&full.first>half.first*1.2);assert(full.second==oldBody.second);
  std::cout<<"PASS: SUB adds110Hz beneath220Hz OSC, continuous level, retired Body inert, no RT allocations\n";
 }
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
 t.p->set(kHarmonyPreset1,0);t.block();
 t.p->set(kChordPrev,1);t.block();assert(t.p->getInt(kHarmonyPreset1)==95);
 t.p->set(kChordPrev,1);t.block();assert(t.p->getInt(kHarmonyPreset1)==94);
 t.p->set(kChordNext,1);t.block();assert(t.p->getInt(kHarmonyPreset1)==95);
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
 t.p->set(kInputMode,1);t.p->reset();for(int prog=0;prog<kProgressionCount;++prog){t.p->set(kHarmonyPreset1,prog);t.block();for(int pad=0;pad<4;++pad)t.block({mpc::MidiEvent::noteOn(0,36+pad,100)});assert(t.p->activeVoices()<=16);for(int pad=0;pad<4;++pad)t.block({mpc::MidiEvent::noteOff(0,36+pad)});t.finish();}
 std::cout<<"PASS: channel-isolated pedals, CC121 mono fallback, unified presets/volume, synced rate display;  MIDI/chord ownership, sustain, mono fallback, mode transitions, preset independence, state, arrows, spread cancellation, sample offsets, allocation-free rendering, 64 sounds and 96 progressions. Peak range "<<minPeak<<".."<<maxPeak<<"\n";
}
