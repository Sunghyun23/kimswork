#include <cassert>
#include <cmath>
#include <iostream>
#include <memory>
#include <array>
#include <string>
#define DONUT_VOICE_TEST
#include "../src/DonutTheory.cpp"
struct Rig{std::unique_ptr<DonutTheory>p=std::make_unique<DonutTheory>();float l[512]{},r[512]{};float*out[2]={l,r};Rig(double sr=48000){p->prepare(sr,512);block();}void block(int frames=128,mpc::Transport tr={}){mpc::ProcessContext c;c.sampleRate=48000;c.outputs=out;c.numOutputs=2;c.numFrames=frames;c.transport=tr;p->process(c);for(int i=0;i<frames;++i)assert(std::isfinite(l[i])&&std::isfinite(r[i]));}};
static constexpr int syncs[]={kSync,kModSync,kLfo2Sync},rates[]={kRate,kTremRate,kLfo2Rate},relays[]={kLfoRateControl,kModRateControl,kLfo2RateControl},full[]={kLfo1SyncDivision,kMotionSyncDivision,kLfo2SyncDivision},legacy[]={kDivision,kModDivision,kLfo2Division};
static std::vector<uint8_t> prefix(DonutTheory&p,int count){auto s=p.internalSaveState();size_t end=12;for(int i=0;i<count;++i)end+=1+s[end]+8;s.resize(end+4);for(int b=0;b<4;++b){s[end+b]=0;s[8+b]=uint8_t(count>>(8*b));}return s;}
int main(){
 static_assert(kLfo1SyncDivision==388&&kMotionSyncDivision==389&&kLfo2SyncDivision==390&&kNumParams==391,"append-only sync grid");
 for(int old=0;old<5;++old){const char*base[]={"1/16","1/8","1/4","1/2","1 bar"};float beats[]={.25f,.5f,1,2,4};for(auto suffix:{" D"," T"}){std::string name=std::string(base[old])+suffix;bool found=false;for(int i=0;i<15;++i)if(name==donut::kSyncNames[i]){assert(std::abs(donut::kSyncBeats[i]-beats[old]*(suffix[1]=='D'?1.5f:2.f/3))<.000001f);found=true;}assert(found);}}
 for(int i=1;i<15;++i)assert(donut::kSyncBeats[i]<donut::kSyncBeats[i-1]);
 for(int module=0;module<3;++module){Rig h;auto&p=*h.p;p.set(rates[module],1.37f);p.set(syncs[module],1);p.set(full[module],0);h.block();const float hz=p.getNormalised(rates[module]);std::array<bool,15>seen{};
  for(int n=0;n<1200;++n){p.setNormalised(relays[module],std::min(1.f,p.getNormalised(relays[module])+.001f));h.block(n%2?32:127);seen[p.getInt(full[module])]=true;assert(p.getNormalised(rates[module])==hz);}
  for(bool found:seen)assert(found);assert(p.getInt(full[module])==14);
  seen.fill(false);for(int n=0;n<1200;++n){p.setNormalised(relays[module],std::max(0.f,p.getNormalised(relays[module])-.001f));h.block(97);seen[p.getInt(full[module])]=true;}
  for(bool found:seen)assert(found);assert(p.getInt(full[module])==0);
  // Direct touch-menu changes and Sync toggles rebind the cursor correctly.
  p.set(full[module],8);h.block();assert(donut::syncIndex(p.get(relays[module]))==8);
  p.set(syncs[module],0);h.block();assert(std::abs(p.get(relays[module])-hz)<1.e-5f);
  p.setNormalised(relays[module],.42f);h.block();assert(std::abs(p.getNormalised(rates[module])-.42f)<1.e-5f);
  p.set(syncs[module],1);h.block();assert(p.getInt(full[module])==8&&donut::syncIndex(p.get(relays[module]))==8);
  for(int old=0;old<5;++old){p.set(legacy[module],old);h.block();assert(p.getInt(full[module])==donut::kLegacySyncIndex[old]);}
 }
 std::cout<<"PASS:three Sync controls visit all15choices clockwise/counterclockwise using0.001relative turns across blocks; Hz memory, touch selection and mode switching\n";
 {Rig h;auto&p=*h.p;p.set(kSync,1);p.set(kLfo2Sync,1);p.set(kLfo1SyncDivision,0);p.set(kLfo2SyncDivision,14);p.set(kLfoFocus,0);h.block();for(int n=0;n<1200;++n){p.setNormalised(kPerfLfoRate,std::min(1.f,p.getNormalised(kPerfLfoRate)+.001f));h.block();}assert(p.getInt(kLfo1SyncDivision)==14&&p.getInt(kLfo2SyncDivision)==14);p.set(kLfoFocus,1);h.block();for(int n=0;n<1200;++n){p.setNormalised(kPerfLfoRate,std::max(0.f,p.getNormalised(kPerfLfoRate)-.001f));h.block();}assert(p.getInt(kLfo2SyncDivision)==0&&p.getInt(kLfo1SyncDivision)==14);}
 std::cout<<"PASS:selected-LFO front relay accumulates slow turns and isolates LFO1/LFO2\n";
 for(double bpm:{60.,88.,120.,200.})for(int mode=0;mode<15;++mode){Rig h;auto&p=*h.p;for(int module=0;module<3;++module){p.set(syncs[module],1);p.set(full[module],mode);}mpc::Transport tr;tr.valid=tr.playing=tr.tempoValid=tr.ppqValid=true;tr.bpm=bpm;tr.ppqPos=7.3125;h.block(97,tr);auto phases=p.modulationPhases();double cycles=(tr.ppqPos+97*bpm/(60.*48000))/donut::kSyncBeats[mode];double expected=cycles-std::floor(cycles);for(auto phase:phases)assert(std::abs(phase-expected)<.0001);}
 std::cout<<"PASS:all15beat periods x4tempos x3modules render correct PPQ phases, including dotted3/2 and triplet2/3\n";
 {Rig h;auto&p=*h.p;for(int module=0;module<3;++module){p.set(syncs[module],1);p.set(full[module],module*3);}auto state=p.internalSaveState();Rig copy;assert(copy.p->internalLoadState(state.data(),state.size()));copy.p->reset();copy.block();for(int module=0;module<3;++module)assert(copy.p->getInt(full[module])==module*3);
  for(int old=0;old<5;++old){for(int module=0;module<3;++module)p.set(legacy[module],old);auto s=prefix(p,388);Rig loaded;assert(loaded.p->internalLoadState(s.data(),s.size()));loaded.p->reset();loaded.block();for(int module=0;module<3;++module)assert(loaded.p->getInt(full[module])==donut::kLegacySyncIndex[old]);}
  for(int module=0;module<3;++module){p.set(full[module],3+module);p.set(diceValueLock(full[module]),1);}for(int n=0;n<30;++n){p.set(kRandomize,1);h.block();for(int module=0;module<3;++module)assert(p.getInt(full[module])==3+module);}
  p.set(kUnlockDice,1);h.block();std::array<int,3>before{};for(int module=0;module<3;++module)before[module]=p.getInt(full[module]);p.set(kRandomize,1);h.block();p.set(kUndoDice,1);h.block();for(int module=0;module<3;++module)assert(p.getInt(full[module])==before[module]);
 }
 std::cout<<"PASS:full dotted/triplet state restore, all5legacy divisions migrate, shared rate locks protect new choices and Undo restores them\n";
}
