#define main crate_engine_base_main
#include "engine.cpp"
#undef main
int main(){
 for(int block:{32,128,512}){
  Audio x;for(int warm=0;warm<2048;warm+=block)x.run(block);x.p.set(K::Input,-6);x.p.set(K::Output,-3);x.p.set(K::Mix,72);x.p.set(K::Latch,1);x.p.set(K::PerfGate+2,1);x.run(block);
  std::array<float,13>before{};for(int n=0;n<13;++n)before[n]=x.p.getNormalised(K::DiceTargets[n]);
  auto preserved=[&](){return std::array<float,10>{x.p.getNormalised(K::Input),x.p.getNormalised(K::Output),x.p.getNormalised(K::Mix),x.p.getNormalised(K::Latch),x.p.getNormalised(K::PerfGate+2),x.p.getNormalised(K::PeakGuard),x.p.getNormalised(K::Bypass),x.p.getNormalised(K::RepeatDiv),x.p.getNormalised(K::DuckSource),x.p.getNormalised(K::FxOutput)};};auto keep=preserved();
  x.p.set(K::RollDice,1);x.run(block);check(preserved()==keep,"Dice preserves I/O/Mix/safety/LATCH/live gates/performance setup");
  for(int n=0;n<13;++n)check(x.p.getNormalised(K::DiceTargets[n])!=before[n],"every open character value changes");
  x.p.set(K::UndoDice,1);x.run(block);for(int n=0;n<13;++n)check(x.p.getNormalised(K::DiceTargets[n])==before[n],"Undo exact normalized character values");
  for(int n=0;n<13;++n)x.p.set(K::DiceLocks+n,1);x.p.set(K::RollDice,1);x.run(block);for(int n=0;n<13;++n)check(x.p.getNormalised(K::DiceTargets[n])==before[n],"all locked is a no-op");
  x.p.set(K::UnlockDice,1);x.run(block);for(int n=0;n<13;++n){check(!x.p.getBool(K::DiceLocks+n),"Unlock All clears protection");check(x.p.getNormalised(K::DiceTargets[n])==before[n],"Unlock All never changes sound");}
  x.p.set(K::DiceLocks,1);for(int i=0;i<64;++i){x.p.set(K::RollDice,1);x.run(block);check(x.p.getNormalised(K::Drive)==before[0],"shared Drive lock");check(x.p.get(K::HighCut)>=4000&&x.p.get(K::HighCut)<=18000,"musical filter range");check(x.p.get(K::LowCut)<=65.001f,"restrained low cut");check(preserved()==keep,"repeated rolls preserve live effects");}
  x.p.set(K::RollDice,1);const auto state=x.p.internalSaveState();check(x.p.getBool(K::RollDice),"snapshot does not consume command");Audio y;check(y.p.internalLoadState(state.data(),state.size()),"state loads");y.p.reset();y.run(block);check(!y.p.getBool(K::RollDice)&&y.p.getBool(K::DiceLocks),"restore clears commands and keeps locks");for(int n=0;n<13;++n)check(y.p.getNormalised(K::DiceTargets[n])==x.p.getNormalised(K::DiceTargets[n]),"state never replays Dice");
  // A real v2 chunk truncated at the prior380declarations migrates with open locks.
  size_t pos=12;for(int n=0;n<380;++n){size_t len=state[pos++];pos+=len+8;}std::vector<uint8_t>old(state.begin(),state.begin()+pos);old[8]=380&255;old[9]=380>>8;old[10]=old[11]=0;old.insert(old.end(),4,0);Audio z;check(z.p.internalLoadState(old.data(),old.size()),"old380state accepted");z.p.reset();z.run(block);for(int n=0;n<13;++n)check(!z.p.getBool(K::DiceLocks+n),"old state seeds open locks");
 }
 // A real stopped VST instance has no audio callbacks; UI reads/idle still execute Dice.
 auto*effect=VSTPluginMain(nullptr);check(effect!=nullptr,"VST created");effect->dispatcher(effect,effOpen,0,0,nullptr,0);float originalDrive=effect->getParameter(effect,K::Drive);effect->setParameter(effect,K::RollDice,1);check(static_cast<CratePress*>(effect->object)->getNormalised(K::Drive)!=originalDrive,"setter executes command even when host sends no idle or value reads");effect->dispatcher(effect,effEditIdle,0,0,nullptr,0);check(effect->getParameter(effect,K::Drive)!=originalDrive,"Dice runs without process callbacks");effect->setParameter(effect,K::UndoDice,1);effect->dispatcher(effect,effEditIdle,0,0,nullptr,0);check(effect->getParameter(effect,K::Drive)==originalDrive,"Undo runs without process callbacks");effect->setParameter(effect,K::DiceLocks,1);effect->setParameter(effect,K::UnlockDice,1);effect->dispatcher(effect,effEditIdle,0,0,nullptr,0);check(effect->getParameter(effect,K::DiceLocks)==0,"Unlock All runs without process callbacks");effect->dispatcher(effect,effClose,0,0,nullptr,0);
 check(realtimeAllocations==0,"Dice/Undo/Unlock allocate nothing in process");std::cout<<"PASS Dice/Undo/13shared locks, I/O and live performance preservation, state snapshots/old380migration, variable blocks and zero RT allocation\n";
}
