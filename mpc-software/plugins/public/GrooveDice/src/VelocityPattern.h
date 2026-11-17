// Beat-position accents. Generated MAIN only; recorded/imported velocities stay authored.
#pragma once
#include <algorithm>
#include <cmath>
#include <cstdint>
namespace groove {
inline int hatAccent(int tick,bool accented=false){
 static constexpr int normal[]={90,45,68,40},strong[]={100,35,66,30};
 int phase=((tick%960)+960)%960;int value=(accented?strong:normal)[phase/240];
 // Intervening 32nds are supporting taps, never an extra strong beat.
 return phase%240?std::min(value,38):value;
}
inline int kickAccent(int tick){int phase=((tick%960)+960)%960;return phase==0?105:phase==480?84:64;}
inline int percAccent(int tick){static constexpr int values[]={60,42,54,40};return values[(((tick%960)+960)%960)/240];}
inline int velocityVariation(int base,float spread,int rule,uint32_t random){
 int cap=rule<=0?0:rule==1?2:rule==2?4:6;
 int amount=std::min(cap,int(std::lround(std::clamp(spread,0.f,50.f)*.12f)));
 int delta=amount?int(random%uint32_t(2*amount+1))-amount:0;
 return std::clamp(base+delta,1,127);
}
}
