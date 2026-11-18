#define main priorOneBarMain
#include "one_bar_loop_test.cpp"
#undef main
#include <set>
int main(){
 static_assert(kRulePickFullKick==672&&kParamCount==779,"append-only extended rules");
 for(int group=0;group<4;++group){Host h;auto&p=h.p;p.set(kDensity,100);p.set(kHatRhythm,6);h.block();
  std::set<std::vector<Row>>raw;std::set<std::vector<std::tuple<int,int,int>>>scheduled;
  auto fill=h.notes(0);auto other=h.notes(4,(group+1)%4);
  for(int choice=7;choice<16;++choice){p.set(kRulePickFullKick+group,choice);h.block();assert(p.ruleStyle(group)==choice+1);
   auto n=h.notes(4,group);assert(!n.empty());raw.insert(n);assert(h.notes(0)==fill);assert(h.notes(4,(group+1)%4)==other);
   int backbeats=0;for(auto [tick,role,vel,chance,gate]:h.notes(4)){assert(tick>=0&&tick<3840);if(role==0)assert(tick!=960&&tick!=2880);if(role==1&&(tick==960||tick==2880))++backbeats;}assert(backbeats==2);
   h.ppq=0;h.events.clear();p.reset();h.playing=true;h.frames=choice%2?32:512;h.until(8);h.playing=false;h.block();
   std::vector<std::tuple<int,int,int>>heard;for(auto [bar,tick,role,vel]:h.events)if(bar==1&&(role==0?0:role==1?1:role<4?2:3)==group)heard.emplace_back(tick,role,vel);
   assert(!heard.empty());scheduled.insert(heard);
  }
  assert(raw.size()==9&&scheduled.size()==9);
  auto state=p.internalSaveState();Host restored;assert(restored.p.internalLoadState(state.data(),state.size()));restored.p.reset();restored.block();assert(restored.p.ruleStyle(group)==16);
  for(int i=0;i<200;++i)restored.block();restored.p.set(kRulePickKick+group,2);restored.block();assert(restored.p.ruleStyle(group)==3);
 }
 // Every hat style remains playable for every independent division/ornament.
 {Host h;h.p.set(kDensity,100);for(int rhythm=0;rhythm<15;++rhythm)for(int choice=0;choice<16;++choice){h.p.set(kHatRhythm,rhythm);h.p.set(kRulePickFullHat,choice);h.block();assert(!h.notes(4,2).empty());}}
 // Legacy672chunks seed appended selectors, retaining exact old meanings.
 {Host h;h.p.set(kRuleKickStyle,6);h.block();auto state=h.p.internalSaveState();size_t pos=12;for(int i=0;i<672;++i)pos+=1+state[pos]+8;
  std::vector<uint8_t>old;
  old.assign(state.begin(),state.begin()+pos);size_t end=pos;for(int i=672;i<676;++i)end+=1+state[end]+8;old.insert(old.end(),state.begin()+end,state.end());old[8]=672&255;old[9]=672>>8;
  Host copy;assert(copy.p.internalLoadState(old.data(),old.size()));copy.p.reset();copy.block();assert(copy.p.ruleStyle(0)==6);
 }
 std::cout<<"PASS36new rule fingerprints and36distinct scheduled patterns; actual variable-block events, backbeats2/4, kick exclusions, other-part/fill preservation, full state and legacy672migration\n";
}
