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
 for(double sr:{44100.,48000.,96000.})for(int program=0;program<57;++program){
  Host h(sr);h.p->applyProgram(program);h.p->set(kInputMode,0);
  assert(h.p->getInt(kModel)==felt::kProgramModels[program]);
  for(int note:{48,60,72})for(int velocity:{35,65,100}){
   h.p->reset();h.on(note,velocity);assert(h.energy(.09)>1e-10);h.off(note);
  }
 }
 std::cout<<"PASS57factory programs x3rates x3registers x3velocities:1539finite/non-silent strikes under RT guard\n";
 // Identical names/indices and a dry reference for every engine. Init stays neutral.
 {Host h;assert(h.p->numPrograms()==57);h.p->applyProgram(0);for(int k:{kDepth,kDrift,kDrive,kChorus,kDelay,kRoom,kNoise,kKeyOff})assert(h.p->get(k)==0);
 for(int model=0;model<7;++model){h.p->applyProgram(1+model*8);assert(h.p->getInt(kModel)==model);for(int k:{kDepth,kDrift,kDrive,kChorus,kDelay,kRoom,kNoise,kKeyOff})assert(h.p->get(k)==0);assert(h.p->get(kVelocity)==65);}}
 for(int model=0;model<7;++model){
  Host h;h.p->set(kModel,model);h.p->set(kVelocityCurve,2);h.p->set(kKey,8);h.p->set(kTranspose,-3);h.p->set(kSlots,3);h.p->set(kPage,2);
  for(int lock=0;lock<FeltClub::SoundCount;++lock){
   h.p->set(kSync,lock%2);int key=kSoundKeys[lock];float value=h.p->param(key).max;
   if(key==kModel)value=model;
   h.p->set(key,value);h.p->set(kSoundLocks[lock],1);
   std::array<float,kParamCount> before{};for(int k=0;k<kParamCount;++k)before[k]=h.p->get(k);
   h.pulse(kRoll);assert(h.p->get(key)==value);assert(h.p->getInt(kModel)==model);assert(h.p->get(kSync)==before[kSync]);assert(h.p->get(kVelocityCurve)==before[kVelocityCurve]);
   for(int k=kInputMode;k<=kPadNote16;++k)assert(h.p->get(k)==before[k]);
   h.pulse(kUndo);for(int k:kSoundKeys)assert(h.p->get(k)==before[k]);assert(h.p->getBool(kSoundLocks[lock]));
   h.p->set(kSoundLocks[lock],0);h.p->applyProgram(1+model*8);
  }
 }
 std::cout<<"PASS266individual lock/Undo cases; MIDI/chords/key/slots/engine/touch/sync preserved\n";
 for(int model=0;model<7;++model){Host h;h.p->set(kModel,model);const auto& profile=felt::kDiceProfiles[model];
  for(int i=0;i<64;++i){h.pulse(kRoll);assert(h.p->getInt(kModel)==model);assert(h.p->get(kAttack)<=4);assert(h.p->get(kNoise)==0&&h.p->get(kKeyOff)==0);assert(h.p->get(kDrift)<=profile.drift);assert(h.p->get(kDepth)<=profile.motion);assert(h.p->get(kSustain)>=92);assert(h.p->getInt(kShape)<=1);int colors=0;for(int k:{kDrive,kChorus,kDelay,kRoom})colors+=h.p->get(k)>0;assert(colors<=1);
   if(model==5){assert(h.p->get(kCharacter4)>=18&&h.p->get(kCharacter4)<=38);assert(h.p->get(kRelease)>=100&&h.p->get(kRelease)<=220);}
  }
 }
 std::cout<<"PASS448unlocked rolls: acoustic attacks, bounded drift/motion, no random noise, one color; Muted keeps pitched guitar body\n";
}
