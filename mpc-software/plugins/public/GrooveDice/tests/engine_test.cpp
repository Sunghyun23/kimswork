#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <memory>
#include <atomic>
#include <thread>
static thread_local bool forbidAlloc=false;
void* operator new(std::size_t n){if(forbidAlloc)std::abort();if(void*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p) noexcept{std::free(p);}void operator delete(void*p,std::size_t) noexcept{std::free(p);}
struct RuleProbe {static void clear(GrooveDice&p){p.clearBank(4);}static bool downbeat(GrooveDice&p){return p.eventCount==1&&p.events[0].offset==17&&p.events[0].role==0;}};
struct Host {
 std::unique_ptr<GrooveDice> p=std::make_unique<GrooveDice>();
 float audio[16][512]{};float* output[16];double ppq=0;int frames=128;double bpm=120;bool playing=false,cycling=false,recording=false;double energy[16]{};
 Host(){for(int i=0;i<16;++i)output[i]=audio[i];p->prepare(44100,512);p->reset();block();}
 void block(const mpc::MidiEvent* midi=nullptr,int count=0){for(auto& ch:audio)std::fill(ch,ch+512,0);mpc::ProcessContext c;c.outputs=output;c.numOutputs=16;c.numFrames=frames;c.sampleRate=44100;c.midi=midi;c.numMidiEvents=count;c.transport.valid=c.transport.tempoValid=c.transport.ppqValid=true;c.transport.bpm=bpm;c.transport.playing=playing;c.transport.recording=recording;c.transport.ppqPos=ppq;c.transport.cycling=cycling;c.transport.cycleStartPpq=0;c.transport.cycleEndPpq=4;forbidAlloc=true;p->process(c);forbidAlloc=false;for(int ch=0;ch<16;++ch)for(int n=0;n<frames;++n){assert(std::isfinite(audio[ch][n]));energy[ch]+=audio[ch][n]*audio[ch][n];}if(playing||p->getBool(kPreview))ppq+=frames*bpm/(60*44100);if(cycling&&ppq>=4)ppq-=4;}
 void until(double beat){while(ppq<beat)block();}
 void pulse(int key){p->set(key,p->getBool(key)?0:1);block();}
 void note(int note,int offset=0,int velocity=100){auto e=mpc::MidiEvent::noteOn(0,note,velocity,offset);block(&e,1);}
};
static std::atomic<int> notified{-1};
static VstIntPtr callback(AEffect*,VstInt32 op,VstInt32 index,VstIntPtr,void*,float){if(op==audioMasterAutomate)notified.store(index);return 0;}
int main(){
 {Host h;auto&p=*h.p;p.set(kSwing,50);p.set(kRndNudge,0);p.set(kGhost,0);
  // All16 SCREEN relays control exactly the intendedMAIN parameter.
  for(int n=0;n<16;++n){int t=p.proxyTarget(n);p.setNormalised(kQ1+n,.4f);assert(std::abs(p.get(t)-p.param(t).toPlain(.4f))<.001f);}
  p.set(kEditor,1);for(int n=0;n<8;++n){int t=p.proxyTarget(n);p.setNormalised(kQ1+n,.7f);assert(std::abs(p.get(t)-p.param(t).toPlain(.7f))<.001f);}float before=p.get(kDrive);p.set(kQ13,.9);assert(p.get(kDrive)==before);std::cout<<"PASS16MAIN and8FILL relays; inactive banks do not alter synth controls\n";
 }
 {Host h;auto&p=*h.p;h.playing=true;p.set(kCountIn,0);p.set(kFillLength,2);p.set(kRecMode,0);h.pulse(kRecord);h.until(.1);h.note(h.p->getInt(kPadNote0),64,111);h.note(h.p->getInt(kPadNote1),17,84);h.note(h.p->getInt(kPadNote2),90,63);h.until(4.05);assert(p.recordState.load()==3);assert(p.countNotes(0)==3);int count=0;for(auto&n:p.notes[0])if(n.valid()){auto v=n.read();assert(v.velocity==111||v.velocity==84||v.velocity==63);++count;}assert(count==3);std::cout<<"PASS replace fill records only3physical MIDI hits, correct velocities, auto finish\n";
  auto saved=p.internalSaveState();Host other;assert(other.p->internalLoadState(saved.data(),saved.size()));other.p->reset();assert(other.p->countNotes(0)==3);for(int i=0;i<GrooveDice::MaxNotes;++i)assert(other.p->notes[0][i].data.load()==p.notes[0][i].data.load());assert(other.p->getInt(kEditor)==0);std::cout<<"PASS fill notes/velocity/timing state round-trip; initialMAIN\n";
  p.set(kRecMode,1);h.pulse(kRecord);double next=std::ceil(h.ppq/4)*4;h.until(next+.2);h.note(h.p->getInt(kPadNote3));h.until(next+4.05);assert(p.countNotes(0)==4);std::cout<<"PASS overdub preserves old notes and appends hit\n";
  h.pulse(kTrigger);assert(p.recordState.load()==4);double start=std::ceil(h.ppq/4)*4;h.until(start+.2);assert(p.recordState.load()==5);h.until(start+4.05);assert(p.recordState.load()==0);std::cout<<"PASS queued fill -> playing -> main pattern\n";
 }
 {Host h;auto&p=*h.p;p.set(kCountIn,1);h.playing=true;int original=p.countNotes(0);h.pulse(kRecord);h.note(h.p->getInt(kPadNote0));assert(p.recorded.load()==0);assert(p.countNotes(0)==original);h.pulse(kCancel);h.until(9);assert(p.recorded.load()==0);std::cout<<"PASS count-in excludes hits and cancel preserves original fill\n";
 }
 {Host h;auto&p=*h.p;p.set(kScope,0);p.set(kKitLock,1);p.set(kSample0,3);h.pulse(kRoll);assert(p.getInt(kSample0)==3);p.set(kKitLock,0);h.pulse(kRoll);assert(p.getInt(kChoke2)==1&&p.getInt(kChoke3)==1&&p.getInt(kChoke0)==0&&p.getInt(kChoke1)==0);p.set(kUserKit,3);p.set(kSample0,6);h.pulse(kSaveKit);p.set(kSample0,1);h.pulse(kLoadKit);assert(p.getInt(kSample0)==6);std::cout<<"PASS kit lock, role-aware random/choke defaults, user kit save/load\n";
 }
 {Host h;auto&p=*h.p;p.set(kEditor,1);p.set(kNoteSelect,0);h.block();int oldTick=p.notes[0][0].read().tick;p.setNormalised(kQ5,p.param(kNoteNudge).toNormalised(20));h.block();assert(p.notes[0][0].read().tick!=oldTick);p.setNormalised(kQ6,p.param(kNoteVelocity).toNormalised(55));h.block();assert(p.notes[0][0].read().velocity==55);std::cout<<"PASS note Nudge/velocity edited through SCREEN relays\n";
 }
 {Host h;auto&p=*h.p;p.set(kSeparate,1);p.set(kRoute0,2);h.note(h.p->getInt(kPadNote0),64);for(int i=0;i<20;++i)h.block();assert(h.energy[4]>0&&h.energy[0]==0);p.set(kSeparate,0);h.note(h.p->getInt(kPadNote0));for(int i=0;i<20;++i)h.block();assert(h.energy[0]>0);std::cout<<"PASS16channel routing and main/separate mode\n";
 }
 {Host h;auto&p=*h.p;p.set(kGhost,0);p.set(kRndNudge,0);p.set(kSwing,50);h.playing=true;h.until(1);int n=p.fired.load();assert(n>0);h.ppq=0;h.block();assert(p.fired.load()>n);h.frames=32;h.until(.2);h.frames=512;h.until(.4);std::cout<<"PASS host transport/seek,32/128/512block rendering; no process allocations\n";
 }
 {Host h;auto&p=*h.p;h.playing=true;h.cycling=true;h.until(1);p.set(kCountIn,0);h.pulse(kRecord);bool sent=false;for(int i=0;i<2000&&p.recordState.load()!=3;++i){if(p.recordState.load()==2&&!sent){h.note(h.p->getInt(kPadNote0));sent=true;}else h.block();}assert(sent&&p.recordState.load()==3&&p.countNotes(0)==1);std::cout<<"PASS fill recording crosses host loop boundaries without cancellation\n";}
 {Host h;h.p->set(kSample0,3);auto saved=h.p->internalSaveState();Host other;std::swap(other.p->bank.ids[3],other.p->bank.ids[6]);assert(other.p->internalLoadState(saved.data(),saved.size()));other.p->reset();other.block();assert(other.p->getInt(kSample0)==6);std::cout<<"PASS saved sample identity reconnects after bank directory reorder\n";}
 {Host h;auto&p=*h.p;p.set(kScope,3);for(auto&n:p.notes[0])n.data.store(0);h.pulse(kRoll);assert(p.countNotes(0)>0);h.pulse(kUndo);assert(p.countNotes(0)==0);std::cout<<"PASS random fill generates from empty slot and Undo restores empty slot\n";}
 {Host h;auto&p=*h.p;std::atomic<bool>done{false};std::thread writer([&]{for(int i=0;i<10000;++i)p.setNormalised(kQ1,(i%100)*.01f);done.store(true);});while(!done.load())h.block();writer.join();p.setNormalised(kQ1,.64f);h.block();assert(std::abs(p.get(kSwing)-66)<.001f);AEffect fake{};p.effect_=&fake;p.host_=&callback;p.flushHostUpdates();assert(notified.load()>=0);p.effect_=nullptr;p.host_=nullptr;std::cout<<"PASS concurrent relay input survives audio readback and queues target UI notification\n";}
 {Host h;auto&p=*h.p;p.set(kEditor,1);h.block();p.set(kFillSlot,0);h.block();for(int i=0;i<200;++i){p.setNormalised(kQ1,p.getNormalised(kQ1)+.01f);h.block();}assert(p.getInt(kFillSlot)==3);std::cout<<"PASS small relative Q-Link increments accumulate across discrete Fill Select steps\n";}
 // Host count-in uses negative PPQ. Never loop a pattern into that region.
 for(int frames:{32,128,512})for(bool preview:{false,true}){
  Host h;auto&p=*h.p;h.frames=frames;h.playing=h.recording=true;h.ppq=-8;
  p.set(kPreview,preview);p.set(kGhost,0);p.set(kRndNudge,0);p.set(kHatShift,0);p.set(kSnareLate,0);p.set(kSwing,50);p.set(kXBeat,0);p.set(kPattern,0);RuleProbe::clear(p);p.notes[4][0].write({0,0,110,100,80});
  double step=h.bpm/(60*44100.);
  while(h.ppq+frames*step<=0){h.block();assert(p.fired.load()==0);assert(h.energy[0]==0);}
  // Crossing block keeps the true frame offset of the downbeat.
  h.ppq=-17*step;h.block();assert(RuleProbe::downbeat(p));assert(p.fired.load()==1);
  h.block();assert(p.fired.load()==1);
 }
 std::cout<<"PASS host count-in8beats silent, Preview cannot bypass it, sample-exact downbeat at32/128/512frames\n";
 {Host h;auto&p=*h.p;h.playing=h.recording=true;h.ppq=-4;h.note(p.getInt(kPadNote0));assert(p.fired.load()==1);std::cout<<"PASS physical MIDI monitoring remains playable during host count-in\n";}
 // Kit lock protects factory / custom samples, role settings and all layers in every scope.
 for(int mode:{0,1})for(int scope=0;scope<6;++scope){
  Host h;auto&p=*h.p;p.set(kKitLock,1);p.set(kKitDiceMode,mode);p.set(kScope,scope);p.set(kRandomAmt,100);p.set(kL0Sample,p.layerAll[0][0]);p.set(kL0Level,-12);p.set(kL0Shift,-7);h.block();
  std::vector<std::pair<int,float>> saved;auto remember=[&](int k){saved.emplace_back(k,p.get(k));};remember(kXKit);
  for(int r=0;r<8;++r){remember(kXSample0+r);for(int n=1;n<=6;++n)remember(kPad0+r*9+n);}
  for(int k=kRoleOn0;k<=kL5Invert;++k)remember(k);for(int k=kCym0Sample;k<=kCym2Invert;++k)remember(k);for(int k=kMixKick;k<=kMixPerc;++k)remember(k);
  for(int n=0;n<10;++n){h.pulse(kRoll);for(auto kv:saved)assert(p.get(kv.first)==kv.second);}
  static const char* titles[]={"Extra: None","Extra: Pattern","Extra: Groove","Extra: Fill","Extra: All","Extra: FX"};assert(p.displayText(kScope)==titles[scope]);
  if(scope==1)assert(p.countNotes(4)>0);
  auto state=p.internalSaveState();Host restored;assert(restored.p->internalLoadState(state.data(),state.size()));assert(restored.p->getBool(kKitLock));assert(restored.p->displayText(kScope)==titles[scope]);
 }
 std::cout<<"PASS KIT FIXED preserves samples/roles/layers/mix in6scopes x2dice modes x10rolls; effective labels and saved lock\n";
 std::cout<<"ALL ENGINE TESTS PASSED\n";
}
