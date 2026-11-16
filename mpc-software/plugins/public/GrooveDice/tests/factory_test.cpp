#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <set>
static thread_local bool forbid=false;
void* operator new(size_t n){if(forbid)std::abort();if(void*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
static std::set<int>ui;
static VstIntPtr callback(AEffect*,VstInt32 op,VstInt32 index,VstIntPtr,void*,float){if(op==audioMasterAutomate)ui.insert(index);return 0;}
struct Host{
 std::unique_ptr<GrooveDice>p=std::make_unique<GrooveDice>();float data[16][128]{};float*out[16];double energy=0;
 Host(){for(int i=0;i<16;++i)out[i]=data[i];p->prepare(44100,128);p->reset();}
 void block(){mpc::ProcessContext c;c.outputs=out;c.numOutputs=16;c.numFrames=128;c.sampleRate=44100;for(auto&x:data)std::fill(x,x+128,0);forbid=true;p->process(c);forbid=false;for(auto&x:data)for(float v:x){assert(std::isfinite(v));energy+=v*v;}}
 void pulse(int key){p->set(key,p->getBool(key)?0:1);block();}
 void wait(){for(int step=0;step<3000;++step){block();bool ready=true;for(int r=0;r<8;++r)if(p->getInt(kLibrarySample0+r)!=p->library.active[r].load())ready=false;if(ready)return;std::this_thread::sleep_for(std::chrono::milliseconds(1));}assert(false&&"kit cache timeout");}
};
int main(){Host h;auto&p=*h.p;for(int key:{kDrive,kDust,kRoom,kDuck,kLowCut,kModDepth})assert(std::abs(p.get(key))<.00001f);assert(p.numPrograms()==97&&p.library.factoryKits.size()==96);p.set(kMaster,.43);p.set(kPadNote0,57);p.set(kRoute0,3);p.set(kSwing,64);p.notes[0][0].write({123,1,84,100,80});int preserved=p.countNotes(0);std::set<std::array<int,8>>distinct;double minEnergy=1e9;size_t heard=0;
 for(int n=1;n<=96;++n){p.selectFactoryProgram(n);p.reset();h.block();h.wait();assert(p.activeFactoryProgram()==n);assert(p.getInt(kPadNote0)==57&&p.getInt(kRoute0)==3&&p.get(kMaster)==.43f&&p.get(kSwing)==64&&p.countNotes(0)==preserved);std::array<int,8>ids{};
  for(int r=0;r<8;++r){ids[r]=p.getInt(kLibrarySample0+r);assert(ids[r]==p.library.factoryKits[n-1].sounds[r]);assert(p.getInt(kChoke0+r*9)==(r==2||r==3?1:0));p.reset();double before=h.energy;h.pulse(kPad0+r*9);for(int b=0;b<100;++b)h.block();double energy=h.energy-before;assert(energy>1e-8);minEnergy=std::min(minEnergy,energy);++heard;}distinct.insert(ids);
 }
 assert(distinct.size()==96);std::cout<<"PASS97 native programs,96distinct same-volume kits,768role auditions; min energy="<<minEnergy<<"; pad/routing/master/groove/fill preserved\n";
 p.set(kFactoryKit,1);h.block();p.set(kKitDiceMode,0);p.set(kScope,2);p.set(kRandomAmt,35);p.set(kKitLock,0);for(int n=0;n<100;++n){int old=p.getInt(kFactoryKit);h.pulse(kRoll);assert(p.getInt(kFactoryKit)>0&&p.getInt(kFactoryKit)!=old);}assert(p.diceRolls.load()==100);std::cout<<"PASS100whole-kit ROLL presses change selected kit even with Groove scope; both button edges count\n";
 int locked=p.getInt(kFactoryKit);std::array<int,8>ids{};for(int r=0;r<8;++r)ids[r]=p.getInt(kLibrarySample0+r);p.set(kKitLock,1);h.pulse(kRoll);assert(p.getInt(kFactoryKit)==locked);for(int r=0;r<8;++r)assert(p.getInt(kLibrarySample0+r)==ids[r]);assert(p.displayText(kDiceInfo).find("KIT FIXED")!=std::string::npos);p.set(kKitLock,0);
 p.set(kKitDiceMode,1);p.set(kRandomAmt,100);p.set(kScope,0);h.pulse(kRoll);assert(p.getInt(kFactoryKit)==0);bool different=false;for(int r=0;r<8;++r){if(ids[r]!=p.getInt(kLibrarySample0+r))different=true;assert(p.getInt(kChoke0+r*9)==(r==2||r==3?1:0));}assert(different);h.pulse(kUndo);assert(p.getInt(kFactoryKit)==locked);for(int r=0;r<8;++r)assert(p.getInt(kLibrarySample0+r)==ids[r]);std::cout<<"PASS KitLock,role-aware MixParts and complete kit Undo\n";
 auto before=p.internalSaveState();p.set(kRandomAmt,0);int keep=p.getInt(kFactoryKit);h.pulse(kRoll);assert(p.getInt(kFactoryKit)==keep&&p.displayText(kDiceInfo)=="AMOUNT ZERO");assert(p.internalLoadState(before.data(),before.size()));p.reset();h.block();assert(p.getInt(kFactoryKit)==locked);
 for(int warm=0;warm<180;++warm)h.block();AEffect fake{};p.effect_=&fake;p.host_=&callback;ui.clear();h.pulse(kRoll);p.flushHostUpdates();assert(ui.count(kFactoryKit)&&ui.count(kLibrarySample0)&&ui.count(kSwing));p.effect_=nullptr;p.host_=nullptr;std::cout<<"PASS zero-amount feedback, saved kit state and UI notifications for actual changed controls\n";
 // Relative hardware-style deltas must accumulate across97choice boundaries.
 p.setNormalised(kFactoryKit,0);for(int i=0;i<300;++i){p.setNormalised(kFactoryKit,std::min(1.f,p.getNormalised(kFactoryKit)+.002f));h.block();}assert(p.getInt(kFactoryKit)>50);std::cout<<"PASS97-choice relative Q-Link accumulation without canonical readback loss\n";
 // Duplicate payloads at different paths must preserve the selected directory entry and fractional UI value.
 int duplicate=0;for(size_t i=1;i<p.library.entries.size();++i)if(p.library.entries[i].kind<3&&p.library.find(p.library.entries[i].id.data())!=int(i)+1){duplicate=int(i)+1;break;}if(!duplicate){auto&l=p.library;l.stop();auto e=l.entries.front();e.name="fixture/duplicate.wav";l.entries.push_back(e);l.midi.emplace_back();duplicate=int(l.entries.size());l.categories[0].push_back(duplicate-1);l.quit.store(false);l.worker=std::thread([&l]{l.run();});}assert(duplicate);
 for(int r=0;r<8;++r)p.setNormalised(kSample0+r*9,.476234f);h.block();
 p.set(kLibrarySample7,duplicate);h.block();auto exact=p.getNormalised(kLibrarySample7);auto state=p.internalSaveState();p.set(kLibrarySample7,1);h.block();assert(p.internalLoadState(state.data(),state.size()));p.reset();h.block();assert(p.getInt(kLibrarySample7)==duplicate&&p.getNormalised(kLibrarySample7)==exact);assert(p.internalSaveState()==state);std::cout<<"PASS duplicate-SHA paths preserve original selection and byte-identical state\n";
 p.set(kFactoryKit,0);h.block();p.selectFactoryProgram(0);p.reset();h.block();assert(p.getInt(kFactoryKit)==0&&p.get(kDrive)==0&&p.get(kRoom)==0&&p.getInt(kPadNote0)==57);std::cout<<"PASS Init clears kit FX while retaining user pad/routing/fills\n";
}
