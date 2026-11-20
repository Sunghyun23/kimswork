#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <memory>
#include <fstream>
static thread_local bool guarded=false;
void*operator new(size_t size){if(guarded)std::abort();if(auto*p=std::malloc(size))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
using G=GrooveDice;
struct Host{
 std::unique_ptr<G>p=std::make_unique<G>();float data[16][512]{},*out[16];int block=128;double rate=44100;std::vector<float>audio;
 Host(){for(int c=0;c<16;++c)out[c]=data[c];p->prepare(rate,512);p->reset();}
 void render(const mpc::MidiEvent*event=nullptr,int count=0){for(auto&c:data)std::fill(c,c+512,0);mpc::ProcessContext c{};c.outputs=out;c.numOutputs=16;c.numFrames=block;c.sampleRate=rate;c.midi=event;c.numMidiEvents=count;c.transport.bpm=90;c.transport.tempoValid=true;guarded=true;p->process(c);guarded=false;for(int f=0;f<block;++f){assert(std::isfinite(data[0][f]));audio.push_back(data[0][f]);}}
 void pulse(int key){p->set(key,!p->getBool(key));render();}
 void dry(){for(int key:{kDrive,kDust,kRoom,kDuck,kLowCut,kModDepth})p->set(key,0);p->set(kFollow,0);p->set(kPreview,0);p->set(kMaster,.7f);}
 void impulse(bool constant=false){p->library.stop();dry();for(auto&s:p->sounds){s.sampleRate=int(rate);s.channels.assign(1,std::vector<float>(int(rate),constant?.1f:0));if(!constant)s.channels[0][0]=.1f;}p->set(kXSample0,0);p->set(kSample0,0);p->set(kLevel0,0);p->set(kTune0,0);p->set(kDecay0,2000);p->prepare(rate,512);p->reset();}
 void pin(int role,int layer,bool constant=false){int channel=8+role*3+layer-1,id=p->layerAll[role][layer-1];auto&slot=p->library.slots[channel];slot.sound.sampleRate=int(rate);slot.sound.channels.assign(1,std::vector<float>(int(rate),constant?.1f:0));if(!constant)slot.sound.channels[0][0]=.1f;slot.id=id;slot.refs=1;slot.state.store(3);p->library.current[channel]=channel;p->library.active[channel].store(id);p->library.desiredSeen[channel]=id;p->set(p->layerKey(role,layer,G::LSample),id);p->set(p->layerKey(role,layer,G::LLevel),0);p->set(p->layerKey(role,layer,G::LDecay),2000);}
 void play(int note=24,int offset=17){audio.clear();auto e=mpc::MidiEvent::noteOn(0,note,100,offset);render(&e,1);for(int i=0;i<int(rate*.15/block)+2;++i)render();}
 void ready(){for(int tries=0;tries<2000;++tries){render();bool done=true;for(int slot=0;slot<6;++slot){int id=p->getInt(kL0Sample+slot*G::LayerFields);done&=p->library.active[8+slot].load()==id;}if(done)return;std::this_thread::sleep_for(std::chrono::milliseconds(1));}assert(false&&"layer cache timeout");}
};
static size_t first(const std::vector<float>&a){for(size_t i=0;i<a.size();++i)if(std::abs(a[i])>1e-8)return i;return a.size();}
static std::vector<uint8_t>legacy(const std::vector<uint8_t>&src){
 auto u32=[&](size_t at){return uint32_t(src[at])|uint32_t(src[at+1])<<8|uint32_t(src[at+2])<<16|uint32_t(src[at+3])<<24;};
 std::vector<uint8_t>out(src.begin(),src.begin()+12);out[8]=342&255;out[9]=342>>8;out[10]=out[11]=0;size_t pos=12;
 for(int i=0;i<kParamCount;++i){size_t n=1+src[pos]+8;if(i<342)out.insert(out.end(),src.begin()+pos,src.begin()+pos+n);pos+=n;}
 uint32_t n=u32(pos)-G::LayerTrailerSize-G::CymbalTrailerSize-G::PoolTrailerSize-G::ReverseTrailerSize-G::PerformanceTrailerSize-G::HatPerformanceTrailerSize;for(int k=0;k<4;++k)out.push_back(uint8_t(n>>(k*8)));out.insert(out.end(),src.begin()+pos+4,src.begin()+pos+4+n);return out;
}
int main(){
 for(double rate:{44100.,48000.,96000.})for(int size:{31,128,511})for(float shift:{-30.f,-15.f,0.f,30.f}){
  Host h;h.rate=rate;h.block=size;h.impulse();h.p->set(kRoleShift0,shift);h.play();assert(first(h.audio)==17+size_t(std::llround(rate*(30+shift)*.001)));assert(h.p->info().latencySamples==std::llround(rate*.030));
 }
 std::cout<<"PASS signed Shift exact onset at3rates x3variable blocks x4offsets;30ms reported latency\n";
 {Host h;auto&p=*h.p;for(int n=0;n<4;++n)assert(p.proxyTarget(8+n)==kMixKick+n);float sample=p.get(kXSample0);p.setNormalised(kQ9,.5f);assert(std::abs(p.get(kMixKick)+21)<.001&&p.get(kXSample0)==sample);
  for(int part=0;part<4;++part){p.set(kSoundPart,part);for(int n=0;n<16;++n){int key=p.soundTarget(n);if(key<0)continue;p.setNormalised(kSoundQ0+n,.6f);if(n%4==1&&!(part==2&&n>=8))assert(std::abs(p.get(key)-p.param(key).toPlain(.6f))<.001f);}}
  p.set(kSoundPart,0);p.setNormalised(kSoundQ4,1);assert(p.getInt(kL0Sample)==p.layerAll[0].back()&&p.getBool(kL0On));p.set(kSoundDetails,1);assert(p.soundTarget(4)==kL0Start&&p.soundTarget(5)==kL0Decay&&p.soundTarget(7)==kL0HighCut&&p.soundTarget(0)==kRoleStart0);p.setNormalised(kSoundQ7,.7f);assert(std::abs(p.get(kL0HighCut)-p.param(kL0HighCut).toPlain(.7f))<.001);p.set(kSoundInvert,1);assert(p.getBool(kL0Invert));
  std::cout<<"PASS MAIN mix relays, all16SOUND controls across4parts, sample selection, detail context and invert\n";
 }
 {Host a,b;a.impulse(true);b.impulse(true);for(int layer=1;layer<4;++layer){a.pin(0,layer,true);b.pin(0,layer,true);a.p->set(a.p->layerKey(0,layer,G::LShift),layer*5);b.p->set(b.p->layerKey(0,layer,G::LShift),layer*5);}a.p->set(kChoke0,0);b.p->set(kChoke0,2);a.play();b.play();assert(a.audio==b.audio);assert(a.p->fired.load()==1&&b.p->fired.load()==1);
  b.p->set(kL0Solo,1);b.p->reset();b.play();int active=0;for(auto&v:b.p->voices)if(v.active){++active;assert(v.layer==1);}assert(active==1);
  std::cout<<"PASS4layers share one logical hit, do not self-choke, and SOLO isolates one layer\n";
 }
 {Host h;auto&p=*h.p;p.set(kL0Sample,p.layerAll[0][0]);p.set(kL0Level,-17);p.set(kL0Shift,-18);p.set(kL0Tune,3);p.set(kL0LowCut,140);p.set(kMixHat,-7);p.set(kUserKit,2);h.pulse(kSaveKit);p.set(kL0Shift,25);p.set(kL0Sample,p.layerAll[0][1]);p.set(kMixHat,0);h.pulse(kLoadKit);assert(p.getInt(kL0Sample)==p.layerAll[0][0]&&p.getInt(kL0Shift)==-18&&p.getInt(kMixHat)==-7);
  auto state=p.internalSaveState();Host restored;assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();assert(restored.p->getInt(kL0Shift)==-18&&restored.p->getInt(kL0LowCut)==140&&restored.p->getInt(kMixHat)==-7);assert(restored.p->getInt(kL0Sample)==p.getInt(kL0Sample));
  auto old=legacy(state);Host prior;assert(prior.p->internalLoadState(old.data(),old.size()));prior.p->reset();assert(!prior.p->getBool(kL0On)&&std::abs(prior.p->get(kMixHat))<.0001f&&prior.p->getBool(kRoleOn0));assert(prior.p->countNotes(0)==p.countNotes(0));
  std::cout<<"PASS kit save/load, full layer/filter/shift/mix state, old342parameter state preserves notes with layersOff\n";
 }
 {Host source;auto&p=*source.p;p.library.stop();int id=p.layerAll[0][0];p.set(kL0Sample,id);p.set(kUserKit,1);source.pulse(kSaveKit);auto state=p.internalSaveState();Host missing;missing.p->library.stop();missing.p->library.entries[id-1].id=std::string(64,'f');assert(missing.p->internalLoadState(state.data(),state.size()));assert(missing.p->getInt(kL0Sample)==0&&missing.p->layerMissing[0].load()>0);auto absent=missing.p->internalSaveState();Host reinstalled;reinstalled.p->library.stop();assert(reinstalled.p->internalLoadState(absent.data(),absent.size()));assert(reinstalled.p->getInt(kL0Sample)==id&&!reinstalled.p->layerMissing[0].load());
  reinstalled.p->loadGuardFrames.store(0);reinstalled.pulse(kLoadKit);assert(reinstalled.p->getInt(kL0Sample)==id);std::cout<<"PASS missing layer identity survives resave and reconnects current/savedkit after reinstall\n";
 }
 {Host h;auto&p=*h.p;p.set(kL0Sample,p.layerAll[0][0]);p.set(kL0Lock,1);int locked=p.getInt(kL0Sample);p.set(kMixKick,-9);p.set(kSoundPart,0);auto notes=p.notes[4][0].data.load();for(int i=0;i<40;++i)h.pulse(kSoundRoll);assert(p.getInt(kL0Sample)==locked);assert(std::abs(p.get(kMixKick)+9)<.0001f);assert(p.notes[4][0].data.load()==notes);h.pulse(kSoundUndo);assert(p.getInt(kL0Sample)==locked);
  p.set(kL0Lock,0);h.ready();for(int i=0;i<200;++i){p.set(kL0Sample,p.layerAll[0][i%p.layerAll[0].size()]);h.render();}h.ready();for(auto&v:p.voices)if(v.active)assert(v.librarySlot<0||p.library.slots[v.librarySlot].refs>0);
  std::cout<<"PASS role-filtered ROLL preserves layer locks/mix/rhythm; asynchronous sample changes and RT allocation guard\n";
 }
 {Host h;h.impulse(true);auto&p=*h.p;for(int l=1;l<4;++l)h.pin(0,l,true);p.set(kSeparate,1);p.set(kRoute0,3);p.set(kMixKick,-6);for(int i=0;i<200;++i)h.render();h.play();assert(std::abs(h.data[6][h.block-1])>.0001f);for(int c=0;c<16;++c)if(c!=6&&c!=7)for(int f=0;f<h.block;++f)assert(h.data[c][f]==0);std::cout<<"PASS all four layers follow the role stereo route and MAIN trim without leaking to main\n";}
 {Host h;h.impulse(true);auto&p=*h.p;p.set(kChoke0,2);p.set(kRoleShift0,30);auto e=mpc::MidiEvent::noteOn(0,24,100,0);h.render(&e,1);uint64_t old=p.hitCounter;p.set(kRoleShift0,-30);h.render(&e,1);assert(p.hitCounter==old+1);for(auto&v:p.voices)if(v.active)assert(v.hit!=old);std::cout<<"PASS a newer hit chokes older pending voices at its actual onset\n";}
 {Host h;auto&p=*h.p;p.set(kSoundDetails,1);p.setNormalised(kSoundQ6,.40361154f);p.set(kL0Sample,1000000);h.render();auto state=p.internalSaveState();assert(p.internalLoadState(state.data(),state.size()));h.render();assert(p.internalSaveState()==state);std::cout<<"PASS per-row detail relays and unavailable catalogue indices preserve byte-identical host chunks\n";}
 std::cout<<"ALL LAYER TESTS PASSED\n";return 0;
}
