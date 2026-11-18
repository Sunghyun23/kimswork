// Fixed-control fixture: Dice locks preserve selected timing/style/division under test.
#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <memory>
#include <tuple>
static thread_local bool guarded=false;
void* operator new(size_t n){if(guarded)std::abort();if(auto*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
struct RuleProbe{static void events(GrooveDice&p,int sample,std::vector<std::tuple<int,int,int>>&v){for(int i=0;i<p.eventCount;++i){auto&e=p.events[i];if(sample+e.offset<88200)v.emplace_back(sample+e.offset,e.role,e.velocity);}}};
struct Host{std::unique_ptr<GrooveDice>p=std::make_unique<GrooveDice>();float audio[16][128]{},*out[16];double ppq=0;bool playing=false;
 Host(){for(int i=0;i<16;++i)out[i]=audio[i];p->prepare(44100,128);p->reset();for(int n=0;n<16;++n)p->set(kDiceLock0+n,1);block();}
 void block(){mpc::ProcessContext c;c.outputs=out;c.numOutputs=16;c.numFrames=128;c.sampleRate=44100;c.transport.valid=c.transport.tempoValid=c.transport.ppqValid=true;c.transport.bpm=120;c.transport.ppqPos=ppq;c.transport.playing=playing;for(auto&a:audio)std::fill(a,a+128,0);guarded=true;p->process(c);guarded=false;if(playing)ppq+=128.*120/(60*44100);}
 void pulse(int i){p->set(i,p->getBool(i)?0:1);block();}
 auto bar(){std::vector<std::tuple<int,int,int>>v;ppq=0;playing=true;p->reset();int sample=0;while(ppq<4){block();RuleProbe::events(*p,sample,v);sample+=128;}playing=false;block();return v;}
};
int main(){
 {Host h;auto&p=*h.p;p.set(kKitLock,1);p.set(kScope,4);p.set(kGhost,100);p.set(kVelSpread,50);p.set(kRndNudge,30);p.set(kVariation,100);for(int g=0;g<4;++g){p.set(kRuleKick+g,0);assert(p.proxyTarget(4+g)==kRulePickFullKick+g);}auto n=p.notes[4][7].read();n.chance=30;p.notes[4][7].write(n);auto expected=h.bar();auto seed=p.partSeed[0].load();for(int i=0;i<100;++i){h.pulse(kRoll);assert(p.countNotes(4)==14);assert(h.bar()==expected);assert(p.partSeed[0].load()==seed);}std::cout<<"PASS LOCK preserves actual scheduled notes/velocities/times across100 AllROLLs despite old extreme settings\n";}
 {Host h;auto&p=*h.p;p.set(kKitLock,1);p.set(kScope,1);p.set(kRandomAmt,100);p.set(kGhost,100);p.set(kVelSpread,50);for(int g=0;g<4;++g)p.set(kRuleKick+g,1);for(int i=0;i<100;++i){h.pulse(kRoll);assert(p.lengths[4].load()==3840);int snare=0;for(auto&c:p.notes[4])if(c.valid()){auto n=c.read();if(n.role==1){assert(n.tick%3840==960||n.tick%3840==2880);++snare;}if(n.role==0)assert(n.tick%3840!=960&&n.tick%3840!=2880);}assert(snare==2);}auto events=h.bar();int snare=0;for(auto&e:events)if(std::get<1>(e)==1)++snare;assert(snare==2);std::cout<<"PASS POCKET generates structured repeating one-bar backbeat; no auto-ghost flood atAmount100%\n";}

 {Host h;auto&p=*h.p;p.set(kKitLock,1);p.set(kScope,2);p.set(kRandomAmt,100);p.set(kRuleKick,0);p.set(kRuleSnare,1);p.set(kRuleHat,2);p.set(kRulePerc,3);float base[]={p.get(kSwing),p.get(kSnareLate),p.get(kHatShift),p.get(kRndNudge)};for(int i=0;i<100;++i){h.pulse(kRoll);for(int n=0;n<4;++n){assert(p.partOffset[0][n].load()==0);assert(p.get(kSwing+n)==base[n]);}}auto state=p.internalSaveState();h.pulse(kRoll);assert(p.internalLoadState(state.data(),state.size()));p.reset();for(int warm=0;warm<180;++warm)h.block();assert(p.internalSaveState()==state);auto saved=p.partSeed[1].load();h.pulse(kRoll);h.pulse(kUndo);assert(p.partSeed[1].load()==saved);std::cout<<"PASS per-role bounded groove, manual knobs unchanged, rule offsets/seeds state+Undo roundtrip\n";}
 {Host h;auto&p=*h.p;p.set(kScope,3);p.set(kKitLock,1);for(auto&n:p.notes[0])n.data.store(0);for(int g=0;g<4;++g)p.set(kRuleKick+g,0);h.pulse(kRollFill);assert(p.countNotes(0)==0);p.set(kRuleHat,2);auto seed=p.partSeed[2].load();h.pulse(kRollFill);for(auto&n:p.notes[0])if(n.valid())assert(n.read().role==2||n.read().role==3);assert(p.partSeed[2].load()==seed);std::cout<<"PASS LOCK retains empty roles; explicitROLLFILL preserves main groove seed\n";}

 {Host h;auto&p=*h.p;for(int i=0;i<128;++i)p.notes[4][i].write({i*30,i%8,100,100,80});p.set(kGhost,100);p.set(kVariation,100);p.set(kVelSpread,50);auto events=h.bar();int counts[8]{};for(auto&e:events)++counts[std::get<1>(e)];int caps[]={4,3,10,1,2,2,1,3};for(int r=0;r<8;++r)assert(counts[r]<=caps[r]);std::cout<<"PASS liveRULES constrain128-note legacy patterns before any pattern-scopeROLL\n";}

 {Host h;auto&p=*h.p;p.set(kNoteNudge,.1f);float nudge=p.getNormalised(kNoteNudge);auto state=p.internalSaveState();assert(p.internalLoadState(state.data(),state.size()));p.reset();h.block();assert(p.getNormalised(kNoteNudge)==nudge);assert(p.internalSaveState()==state);std::cout<<"PASS matching displayed note nudge retains fractional saved value and byte-identical state\n";}

}
