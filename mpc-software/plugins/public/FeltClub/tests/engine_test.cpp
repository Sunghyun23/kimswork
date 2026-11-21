#include <cstdlib>
#include <new>
#include <atomic>
static bool forbidAllocation=false;
void*operator new(std::size_t n){if(forbidAllocation)std::abort();if(void*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,std::size_t)noexcept{std::free(p);}
#include "../src/FeltClub.cpp"
#include <cassert>
#include <iostream>
#include <memory>
struct Host {
 std::unique_ptr<FeltClub>p=std::make_unique<FeltClub>();float data[2][512]{};float*out[2]={data[0],data[1]};int frames=128;double rate=44100;double bpm=88;uint64_t samples=0;
 explicit Host(double sr=44100):rate(sr){p->prepare(sr,512);}
 void block(const mpc::MidiEvent*events=nullptr,int count=0){for(auto&ch:data)std::fill(ch,ch+512,0);mpc::ProcessContext c;c.numOutputs=2;c.outputs=out;c.numFrames=frames;c.sampleRate=rate;c.midi=events;c.numMidiEvents=count;c.transport.valid=c.transport.tempoValid=c.transport.ppqValid=true;c.transport.bpm=bpm;c.transport.ppqPos=samples*bpm/(60*rate);forbidAllocation=true;p->process(c);forbidAllocation=false;samples+=frames;for(auto&ch:data)for(int i=0;i<frames;++i)assert(std::isfinite(ch[i])&&std::abs(ch[i])<=.981f);}
 void render(double seconds){for(int n=0;n<int(seconds*rate/frames)+1;++n)block();}
 void event(mpc::MidiEvent e){block(&e,1);}void on(int note=60,int vel=100,int offset=0){event(mpc::MidiEvent::noteOn(0,note,vel,offset));}void off(int note=60){event(mpc::MidiEvent::noteOff(0,note));}
 void pulse(int key){p->set(key,!p->getBool(key));p->service(true);block();}
 int active(int owner=-1){int count=0;for(auto&v:p->voices)count+=v.state().active&&(owner<0||v.state().owner==owner);return count;}
 double energy(double seconds){double energy=0;for(int n=0;n<int(seconds*rate/frames)+1;++n){block();for(auto&ch:data)for(int i=0;i<frames;++i)energy+=ch[i]*ch[i];}return energy;}
};
int main(){
 {Host h;h.p->set(kCharacter4,31);h.p->setNormalised(kMainQ0+3,.52f);assert(std::abs(h.p->get(kHiss)-52)<.0001f&&h.p->get(kCharacter4)==31);h.p->set(kChordEditing,1);h.p->setNormalised(kMainQ0+3,.8f);assert(std::abs(h.p->get(kHiss)-52)<.0001f);}
 {Host h;auto rng=h.p->randomState.load();h.p->set(kHiss,60);assert(h.energy(.5)>1e-6);assert(h.p->randomState.load()==rng&&h.active()==0);h.p->set(kHiss,0);h.render(.5);assert(h.energy(.05)==0);}
 std::cout<<"PASS MAIN A4 controls Noise without altering Bark; chord overlay masks relay; idle hiss independent of notes and Dice RNG\n";

 // Dice must stay inside the selected instrument, honor locks and retain
 // playing/chord choices through a normal command pulse and exact Undo.
 for(int model=0;model<7;++model){
  Host h;h.p->set(kModel,model);h.p->set(kVelocityCurve,2);
  h.p->set(kSync,1);h.p->set(kBody,58);h.p->set(kLockBody,1);
  h.p->set(kDrive,6);h.p->set(kLockDrive,1);
  const float root=h.p->get(kRoot1),mapping=h.p->get(kPadNote1);
  for(int roll=0;roll<48;++roll){
   std::array<float,FeltClub::SoundCount> previous{};
   for(int i=0;i<FeltClub::SoundCount;++i)previous[i]=h.p->get(kSoundKeys[i]);
   h.pulse(kRoll);
   assert(h.p->getInt(kModel)==model&&h.p->getInt(kVelocityCurve)==2);
   assert(h.p->get(kBody)==58&&h.p->get(kDrive)==6&&h.p->getBool(kSync));
   assert(h.p->get(kRoot1)==root&&h.p->get(kPadNote1)==mapping);
   assert(h.p->get(kAttack)<=7&&h.p->get(kVelocity)>=60);
   assert(h.p->get(kChorus)==0&&h.p->get(kDelay)==0&&h.p->get(kRoom)==0);
   assert(std::abs(h.p->get(kRate)*17-std::round(h.p->get(kRate)*17))<1e-5);
   h.pulse(kUndo);
   for(int i=0;i<FeltClub::SoundCount;++i)assert(h.p->get(kSoundKeys[i])==previous[i]);
  }
  h.p->set(kLockBody,0);h.p->set(kLockDrive,0);
  for(int roll=0;roll<12;++roll){
   h.pulse(kRoll);
   int colors=0;for(int key:{kDrive,kChorus,kDelay,kRoom})colors+=h.p->get(key)>0;
   assert(colors<=1&&h.p->getInt(kModel)==model);
   h.p->set(kInputMode,0);h.on(48,40);h.on(60,75);h.on(72,105);
   assert(h.energy(.08)>1e-9);
   h.event(mpc::MidiEvent::cc(0,120,0));
  }
 }
 std::cout<<"PASS engine-preserving Dice across 7 models: 336 locked rolls, exact Undo, touch/sync/chord retention and 84 audible unlocked rolls\\n";

 // Exercise every factory sound through normal program selection and MIDI,
 // including the physical attack and wet tails, under the RT allocation guard.
 for(int program=0;program<felt::kProgramCount;++program){Host h;h.p->applyProgram(program);h.p->set(kInputMode,0);h.on(48,38);h.on(60,80);h.on(72,118);assert(h.energy(.25)>1e-8);h.off(48);h.off(60);h.off(72);h.render(.1);}
 std::cout<<"PASS all57factory programs: low/mid/high velocity strikes, non-silent finite output and RT allocation guard\n";
 for(double sr:{44100.,48000.,96000.})for(int model=0;model<7;++model){Host h(sr);h.p->set(kModel,model);h.p->set(kRelease,80);h.on(60,100,31);for(int c=0;c<2;++c)for(int i=0;i<31;++i)assert(h.data[c][i]==0);double energy=h.energy(.4);assert(energy>1e-5);h.off();h.render(.7);assert(h.active()==0);}
 std::cout<<"PASS7modal models x3rates, exact MIDI offset, bounded/non-silent output and release termination; RT allocation guard\n";
 for(int model=0;model<7;++model){Host soft,hard;soft.p->set(kModel,model);hard.p->set(kModel,model);soft.on(60,30);hard.on(60,110);assert(hard.energy(.3)>soft.energy(.3)*2);}
 std::cout<<"PASS velocity-dependent excitation across all7models\n";
 {Host h;h.p->set(kRelease,80);h.on();h.event(mpc::MidiEvent::cc(0,64,127));h.off();h.render(.7);assert(h.active()>0);h.event(mpc::MidiEvent::cc(0,64,0));h.render(.5);assert(h.active()==0);h.on();h.event(mpc::MidiEvent::noteOn(0,60,0));h.render(.5);assert(!h.active());}
 std::cout<<"PASS sustain, pedal release and velocity-zero note-off\n";
 {Host h;h.p->set(kRelease,80);h.p->set(kStrum,160);h.event(mpc::MidiEvent::cc(0,64,127));h.p->set(kHold1,1);h.block();assert(h.active()==1);h.p->set(kHold1,0);h.block();for(auto&e:h.p->pending)assert(!e.active);h.render(.6);assert(h.active()==0);}
 std::cout<<"PASS touch hold release ignores sustain and cancels unplayed strum notes\n";
 {Host h;h.p->set(kStrum,0);h.p->set(kChordGlide,1);h.p->set(kHold1,1);h.block();int before=h.active();h.p->set(kHold2,1);h.block();assert(h.active()==before);int gliding=0;for(auto&v:h.p->voices)if(v.state().active){assert(v.state().owner==6001);gliding+=v.state().frequency!=v.state().targetFrequency;}assert(gliding>0);h.p->set(kHold1,0);h.block();assert(h.active()==before);h.p->set(kHold2,0);h.render(1.2);assert(!h.active());}
 std::cout<<"PASS overlapping chord glide reuses voices and releases by current owner\n";
 {Host h;h.p->set(kStrum,0);h.p->set(kHold1,1);h.block();auto state=h.p->snapshot();assert(h.p->getBool(kHold1)&&h.active()>0);Host other;assert(other.p->restore(state.data(),state.size()));other.block();assert(!other.p->getBool(kHold1)&&!other.active());assert(other.p->snapshot()==state);h.p->set(kHold1,0);h.render(1.2);assert(!h.active());}
 std::cout<<"PASS state snapshots never change live holds; restore disarms gates and round-trips exactly\n";
 {Host h;h.p->set(kKey,5);h.p->set(kTranspose,-2);h.p->set(kSlots,3);h.p->set(FeltClub::rootKey(11),8);h.p->set(FeltClub::rootKey(11,4),82);h.p->set(kSync,0);h.p->set(kLockTone,1);h.p->set(kLockModel,1);h.p->set(kTone,63);auto state=h.p->snapshot();std::array<float,FeltClub::SoundCount>before;for(int i=0;i<FeltClub::SoundCount;++i)before[i]=h.p->get(kSoundKeys[i]);h.pulse(kRoll);assert(h.p->get(kTone)==63&&h.p->getInt(kModel)==0&&!h.p->getBool(kSync));assert(h.p->getInt(kKey)==5&&h.p->getInt(kTranspose)==-2&&h.p->slots()==16&&h.p->getInt(FeltClub::rootKey(11))==8&&h.p->getInt(FeltClub::rootKey(11,4))==82);int changes=0;for(int i=0;i<FeltClub::SoundCount;++i)changes+=before[i]!=h.p->get(kSoundKeys[i]);assert(changes>10);h.pulse(kUndo);for(int i=0;i<FeltClub::SoundCount;++i)assert(before[i]==h.p->get(kSoundKeys[i]));h.pulse(kUnlockAll);for(int k:kSoundLocks)assert(!h.p->getBool(k));}
 std::cout<<"PASS sound-only Dice, per-value locks, Sync preservation, exact Undo and UnlockAll; chords/key/maps retained\n";
 {Host h;h.p->set(kSlots,3);h.p->set(kPage,2);h.p->set(kKey,5);h.p->set(FeltClub::rootKey(8,4),83);for(int i=0;i<felt::kProgramCount;++i){h.p->applyProgram(i);h.on(64);h.block();assert(h.p->getInt(kPreset)==i+1&&h.p->getInt(kKey)==5&&h.p->getInt(kPage)==2&&h.p->getInt(FeltClub::rootKey(8,4))==83);h.off(64);h.p->reset();}assert(h.p->numPrograms()==57);}
 std::cout<<"PASS57original native programs preserve chord/key/page/pad state\n";
 {Host h;h.p->set(kChordEditing,1);h.p->set(kKey,3);for(int page=0;page<4;++page){h.p->set(kSlots,3);h.p->set(kPage,page);for(int col=0;col<16;++col){int key=FeltClub::editKey(col/4,col%4);h.p->setNormalised(key,.5f);assert(std::abs(h.p->readParameter(key)-.5f)<.0001f);float tone=h.p->get(kTone);h.p->setNormalised(kMainQ0+col,.2f);assert(h.p->get(kTone)==tone);}}h.p->set(kChordEditing,0);float root=h.p->get(FeltClub::rootKey(12));h.p->setNormalised(kMainQ0,.25f);assert(h.p->get(kTone)==25&&h.p->get(FeltClub::rootKey(12))==root);}
 std::cout<<"PASS16native CHORD relays/four pages/key-relative roots; quick-touch modal blocks hidden MAIN Q-Link sound changes\n";
 {Host h;h.p->set(kSlots,1);h.p->set(kPage,1);h.p->set(kHold1,1);h.block();assert(h.active()>0);h.p->set(kPage,0);h.block();h.render(1.2);assert(!h.active());h.p->set(kInputMode,2);int pad=h.p->getInt(kPadNote1);h.on(pad);assert(h.active()>0);h.p->set(FeltClub::rootKey(0,4),91);h.off(pad);h.render(1.2);assert(!h.active());}
 std::cout<<"PASS page change clears hidden touch; changing pad assignment cannot strand the old physical note owner\n";
 {Host h;h.pulse(kLearn1);h.on(80);assert(h.p->getInt(kPadNote1)==80&&h.p->learnSlot.load()<0&&!h.active());h.on(80);assert(h.active()>0);h.off(80);h.render(1.2);assert(!h.active());}
 {Host h;h.p->set(kStrum,0);h.p->set(kRelease,60);int pad=h.p->getInt(kPadNote1);h.on(pad);h.p->set(kPadNote1,80);h.on(80);h.off(pad);h.render(.15);assert(h.active(8192+80)>0);h.off(80);h.render(.7);assert(!h.active());}
 std::cout<<"PASS pad remapping while held keeps old/new MIDI-note owners independent\n";
 std::cout<<"PASS first pad LEARN hit assigns silently; next hit plays mapped chord\n";
 {Host h;for(int i=0;i<16;++i){h.p->progression(0,i);felt::SmoothInput before;before.active=4;for(int n=0;n<4;++n)before.chords[n]={48+h.p->getInt(FeltClub::rootKey(n)),h.p->getInt(FeltClub::rootKey(n,1)),h.p->getInt(FeltClub::rootKey(n,2)),h.p->getInt(FeltClub::rootKey(n,3))+2};auto plan=felt::findSmoothVoicing(before);assert(plan.valid&&plan.movementAfter<=plan.movementBefore);h.p->smoothTop();}}
 std::cout<<"PASS16neo-soul chord progressions and bounded smooth-top voicing planner\n";
 {Host h;for(int i=0;i<500;++i){float v=i/499.f;h.p->setNormalised(kRate,v);assert(h.p->readParameter(kRate)==v);}for(int k=0;k<18;++k){h.p->set(kRate,k/17.f);std::string text;assert(h.p->formatValue(kRate,0,text)&&text==h.p->param(kDelayDivision).choices[k]);}}
 std::cout<<"PASS straight/dotted/triplet motion divisions and fractional Rate cursor\n";
 std::cout<<"ALL FELT CLUB ENGINE CHECKS PASSED\n";
 {Host h;h.p->set(kModel,0);h.p->set(kTone,63);h.p->set(kAttack,2);h.on(60,75);h.render(.03);
  h.p->set(kModel,1);h.p->set(kTone,19);h.p->set(kAttack,47);h.on(67,75);h.block();int tine=0,wurli=0;
  for(auto&v:h.p->voices)if(v.state().active){v.visit([&](const auto&e){if(v.model()==0){++tine;assert(std::abs(e.settings.tone-.63f)<1e-6&&e.settings.attack==2);}if(v.model()==1){++wurli;assert(std::abs(e.settings.tone-.19f)<1e-6&&e.settings.attack==47);}});}
  assert(tine&&wurli);h.off(60);h.off(67);h.render(2);assert(!h.active());
 }
 std::cout<<"PASS selected-model tone/envelope edits do not modify another held engine; both MIDI owners release\n";

}
