// Fixed-control fixture: Dice locks preserve selected timing/style/division under test.
#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <memory>
static thread_local bool guarded=false;
void* operator new(size_t n){if(guarded)std::abort();if(void*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
struct Host{std::unique_ptr<GrooveDice>p=std::make_unique<GrooveDice>();float data[16][128]{},*out[16];double ppq=0;bool playing=false;
 Host(){for(int i=0;i<16;++i)out[i]=data[i];p->prepare(44100,128);p->reset();for(int n=0;n<16;++n)p->set(kDiceLock0+n,1);block();}
 void block(){for(auto&a:data)std::fill(a,a+128,0);mpc::ProcessContext c;c.outputs=out;c.numOutputs=16;c.numFrames=128;c.sampleRate=44100;c.transport.valid=c.transport.tempoValid=c.transport.ppqValid=true;c.transport.bpm=120;c.transport.playing=playing;c.transport.ppqPos=ppq;guarded=true;p->process(c);guarded=false;for(auto&a:data)for(auto x:a)assert(std::isfinite(x));if(playing)ppq+=128.*120/(60*44100);}
 void pulse(int key){p->set(key,p->getBool(key)?0:1);block();}
 void until(double beat){while(ppq<beat)block();}
};
int main(){
 {Host h;auto&p=*h.p;p.set(kKitLock,1);p.set(kScope,1);p.set(kRandomAmt,100);std::vector<GrooveDice::Note>anchors;for(auto&n:p.notes[4])if(n.valid()){auto x=n.read();if(x.role==0&&x.tick==0||x.role==1&&(x.tick==960||x.tick==2880))anchors.push_back(x);}assert(anchors.size()==3);
  for(int roll=0;roll<1000;++roll){h.pulse(kRoll);assert(p.countNotes(4)<=72&&p.lengths[4].load()==3840);for(auto&a:anchors){bool found=false;for(auto&n:p.notes[4])if(n.valid()){auto x=n.read();if(x.role==a.role&&x.tick==a.tick&&x.velocity==a.velocity)found=true;}assert(found);}int count[4][8]{};for(auto&n:p.notes[4])if(n.valid()){auto e=n.read();++count[e.tick/3840][e.role];}int caps[]={6,5,12,2,2,4,2,4};for(int bar=0;bar<4;++bar)for(int r=0;r<8;++r)assert(count[bar][r]<=caps[r]);}
  auto state=p.internalSaveState();Host other;assert(other.p->internalLoadState(state.data(),state.size()));other.p->reset();other.block();assert(other.p->baseValid[4].load());assert(other.p->baseLengths[4].load()==3840);for(int i=0;i<128;++i)assert(other.p->bases[4][i].data.load()==p.bases[4][i].data.load());std::cout<<"PASS1000ROLLs preserve downbeat/backbeats, role caps and bounded density; original pattern saved/restored\n";
 }
 {Host h;auto&p=*h.p;p.set(kKitLock,1);p.set(kScope,2);float base[]={54,12,-4,4},span[]={3,5,3,2};for(int i=0;i<4;++i)p.set(kSwing+i,base[i]);for(float amount:{10.f,80.f}){p.set(kRandomAmt,amount);for(int n=0;n<200;++n){h.pulse(kRoll);for(int i=0;i<4;++i)assert(std::abs(p.get(kSwing+i)-base[i])<=span[i]*amount*.01+.001);}}
 p.set(kGrooveTemplate,10);p.set(kRandomAmt,100);for(int n=0;n<20;++n){h.pulse(kRoll);assert(p.get(kSwing)==54);assert(p.getInt(kGrooveTemplate)==10);}std::cout<<"PASS Amount scales correlated timing bounds without cumulative drift; imported groove does not get extra random swing\n";}
 {Host h;auto&p=*h.p;p.set(kScope,0);h.playing=true;h.ppq=1;auto before=p.diceRolls.load();h.pulse(kRoll);assert(!p.pendingRoll.load()&&p.diceRolls.load()==before+1&&p.getInt(kFactoryKit)>0);h.pulse(kRoll);assert(!p.pendingRoll.load()&&p.diceRolls.load()==before+2);h.playing=false;h.block();assert(!p.pendingRoll.load());std::cout<<"PASS liveROLL commits immediately in the current block; no next-bar queue or replay on stop\n";}

 {Host h;auto&p=*h.p;int counts[]={440,546,376,244};for(int g=0;g<4;++g){assert(p.library.soundPools[g].size()==size_t(counts[g]));assert(p.proxyTarget(8+g)==kMixKick+g);for(int choice:{1,counts[g]}){p.set(kSoundKick+g,choice);h.block();auto&c=p.library.soundPools[g][choice-1];int role=g==0?0:g==1?1:g==2?2:7;assert(p.getInt(kLibrarySample0+role)==c.sounds[0]&&p.getInt(kSoundKick+g)==choice);if(g==2){assert(p.getInt(kLibrarySample0+3)==c.sounds[1]);assert(p.getInt(kChoke2)==1&&p.getInt(kChoke3)==1);}}}
 p.setNormalised(kSoundKick,0);for(int n=0;n<400;++n){p.setNormalised(kXQKick,std::min(1.f,p.getNormalised(kXQKick)+.0005f));h.block();}assert(p.getInt(kSoundKick)>50);assert(p.getInt(kFactoryKit)==0);std::cout<<"PASS1606-entry role catalogue, MAIN C mix targets / legacy role selectors, hat pairs/choke, first/last candidates, small relative knob increments\n";}
 {Host h;auto&p=*h.p;std::atomic<bool>done{false};std::thread writer([&]{for(int i=0;i<6000;++i)p.setNormalised(kXQKick,(i%400)/440.f);done.store(true);});while(!done.load())h.block();writer.join();p.set(kSoundKick,321);h.block();assert(p.getInt(kLibrarySample0)==p.library.soundPools[0][320].sounds[0]);std::cout<<"PASS concurrent role selector writes survive audio readback\n";}

 {Host h;auto&p=*h.p;for(int i=0;i<128;++i)p.notes[4][i].write({i*30,i%8,110,100,80});auto old=p.internalSaveState();size_t pos=12;for(int i=0;i<kParamCount;++i)pos+=1+old[pos]+8;constexpr size_t extra=4+7*(8+128*8)+4+16+84+200+GrooveDice::LayerTrailerSize;uint32_t n=uint32_t(old[pos])|(uint32_t(old[pos+1])<<8)|(uint32_t(old[pos+2])<<16)|(uint32_t(old[pos+3])<<24);n-=extra;for(int k=0;k<4;++k)old[pos+k]=uint8_t(n>>(k*8));old.resize(old.size()-extra);assert(p.internalLoadState(old.data(),old.size()));p.reset();for(int warm=0;warm<180;++warm)h.block();assert(p.countNotes(4)==128);p.set(kKitLock,1);p.set(kScope,1);p.set(kRandomAmt,100);h.pulse(kRoll);assert(p.countNotes(4)<=72&&p.lengths[4].load()==3840);std::cout<<"PASS legacy over-randomised pattern restarts from immutable factory beat; recorded banks remain separate\n";}

 {Host h;auto&p=*h.p;p.set(kPattern,1);p.set(kPreview,0);for(int key:{kRoll,kRecord,kPatternNext,kKitNext})p.set(key,1);auto state=p.internalSaveState();std::array<float,kParamCount>v;for(int i=0;i<kParamCount;++i)v[i]=p.getNormalised(i);v[kQ9]=1;v[kQ5]=1;for(int key:{kRoll,kRecord,kPatternNext,kKitNext})v[key]=0;assert(p.internalLoadState(state.data(),state.size()));p.reset();for(int i=0;i<kParamCount;++i){p.setNormalised(i,v[i]);if(i%8==0)h.block();}for(int warm=0;warm<180;++warm)h.block();assert(p.getNormalised(kQ9)==p.getNormalised(kMixKick));assert(p.getNormalised(kQ5)==p.getNormalised(kRulePickKick));assert(p.diceRolls.load()==0&&!p.getBool(kPreview)&&p.getInt(kPattern)==1&&p.getInt(kFactoryKit)==0&&p.recordState.load()==0);h.pulse(kRoll);assert(p.diceRolls.load()==1);std::cout<<"PASS MPC-style late parameter replay after chunk load cannot retrigger saved commands; userROLL works after settle\n";}

}
