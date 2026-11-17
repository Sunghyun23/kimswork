#define main priorOneBarMain
#include "one_bar_loop_test.cpp"
#undef main
int main(){
 {GrooveDice fresh;for(auto&c:fresh.notes[4])if(c.valid()){auto n=c.read();if(n.role==2)assert(n.velocity==groove::hatAccent(n.tick));}}
 for(int b=0;b<4;++b){int t=b*960;assert(groove::hatAccent(t)==90);assert(groove::hatAccent(t+240)==45);assert(groove::hatAccent(t+480)==68);assert(groove::hatAccent(t+720)==40);}
 for(int rule=0;rule<4;++rule)for(int spread=0;spread<=50;++spread)for(unsigned seed=0;seed<1000;++seed){int strong=groove::velocityVariation(90,spread,rule,seed),mid=groove::velocityVariation(68,spread,rule,seed+173),weak=groove::velocityVariation(45,spread,rule,seed+771);assert(strong>mid&&mid>weak);assert(std::abs(strong-90)<=6);assert(rule||strong==90);assert(spread||strong==90);}
 Host h;auto&p=h.p;p.set(kHatRhythm,6);p.set(kRuleHatStyle,1);h.block();h.pulse(kRoll);
 auto hats=h.notes(4,2);assert(hats.size()==16);for(auto [tick,r,v,c,g]:hats)assert(v==groove::hatAccent(tick));
 for(int rhythm=0;rhythm<15;++rhythm)for(int style=1;style<8;++style){p.set(kHatRhythm,rhythm);p.set(kRuleHatStyle,style);h.block();for(auto [t,r,v,c,g]:h.notes(4,2))assert(v==groove::hatAccent(t,style==6));}
 for(int i=0;i<100;++i){h.pulse(kRoll);for(auto [t,r,v,c,g]:h.notes(4,0))assert(v==groove::kickAccent(t));}
 // Actual playback is stable over repeated bars even with maximum humanization.
 p.set(kHatRhythm,6);p.set(kRuleHatStyle,3);p.set(kVelSpread,50);h.block();h.events.clear();h.playing=true;h.until(16);std::vector<std::tuple<int,int,int>>reference;
 for(auto [bar,t,r,v]:h.events)if(bar==1)reference.emplace_back(t,r,v);
 for(int b=2;b<4;++b){std::vector<std::tuple<int,int,int>>actual;for(auto [bar,t,r,v]:h.events)if(bar==b)actual.emplace_back(t,r,v);assert(actual==reference);}
 // Recorded fills keep arbitrary authored velocities even at max Spread.
 Host f;f.p.set(kVelSpread,50);for(auto&n:f.p.notes[0])n.data.store(0);f.p.notes[0][0].write({0,2,19,100,80});f.p.notes[0][1].write({480,2,122,100,80});f.playing=true;f.until(.2);f.pulse(kTrigger);f.until(8);bool low=false,high=false;for(auto [bar,t,r,v]:f.events)if(bar==1&&r==2){if(t==0&&v==19)low=true;if(t==480&&v==122)high=true;}assert(low&&high);
 std::cout<<"PASS beat-position accents, all15divisions x7styles,100kick rolls, bounded spread preserving strong>medium>weak, repeated scheduler and authored fill velocities; allocation guard enabled\n";
}
