#pragma once
#include <algorithm>
#include <cstdint>

namespace groove {
// Rhythmic cells, not independent random hits. A selected division is retained;
// random draws choose a complete phrase, its accent contour and tiny variations.
struct PerformanceRhythm {
 static constexpr int ppq=960;
 static int interval(int rate){constexpr int ticks[]={480,320,240,160,120,240};return ticks[std::max(0,std::min(5,rate))];}
 static bool triplet(int rate){return rate==1||rate==3;}
 // Interpolate intervening32nds inside a swung pair so they cannot collide
 // with the following beat. Triplets retain their own unwarped grid.
 static double swingOffset(int tick,int rate,float swing){if(triplet(rate))return 0;int pair=rate==0?960:480;double phase=double(tick%pair)/pair,triangle=phase<=.5?phase*2:(1-phase)*2;return(std::max(50.f,std::min(75.f,swing))-50)*(pair==960?.01:.005)*triangle;}

 template<class Random,class Emit>
 static void roll(int length,int rate,int velocity,bool hat,Random&&random,Emit&&emit){
  // Short pickup, call/rest/answer, doubles, and a continuous crescendo.
  constexpr uint16_t cells[]={0xf5d5,0xb5ad,0xdd55,0xf755,0xeeee,0xbdbd,0xff55,0xffff};
  const int shape=int(random()%8);const uint16_t cell=cells[shape];const int contour=int(random()%3);
  int i=0;for(int tick=0;tick<length;){
   int stride=interval(rate);if(rate==5&&tick>=length/2)stride=120;
   bool first=tick==0,last=tick+stride>=length,accent=tick%ppq==0||last;
   bool play=first||last||((cell>>(i%16))&1);
   if(play){float position=float(tick)/std::max(1,length-1);int strong=accent?0:(i%2==0?-12:-27);
    int rise=contour==0?int(position*15)-15:contour==1?int(position*9)-9:0;
    int variation=int(random()%7)-3;
    int v=std::max(18,std::min(127,velocity+strong+rise+variation));
    // Closed hats only: existing choke ownership handles previous open hats.
    if(!hat&&last&&!first&&stride>=240&&!triplet(rate)&&(shape==2||shape==5))emit(tick-80,1,std::max(18,v-36),25);
    emit(tick,hat?2:1,v,hat?30:45);
   }
   tick+=stride;++i;
  }
 }
 template<class Random,class Emit>
 static void fill(int length,int rate,int velocity,int profile,Random&&random,Emit&&emit){
  // One instrument route per phrase: snare pickup, snare-to-tom turns, or perc reply.
  constexpr uint16_t cells[]={0xd5,0xad,0xed,0xf5,0xdd,0xbd,0xb5,0xfd};
  uint16_t cell=cells[random()%8];int route=int(random()%3),i=0;
  for(int tick=0;tick<length;){
   int stride=interval(rate);if(rate==5&&tick>=length/2)stride=120;
   bool first=tick==0,last=tick+stride>=length;
   if(first||last||((cell>>(i%8))&1)){
    float progress=float(tick)/std::max(1,length-1);int role=1;
    if(profile==1&&progress>=.4f)role=5; // coherent snare -> tom turn
    else if(profile==2)role=(i%3==1?7:1); // sparse percussion answers
    else if(profile==3)role=progress<.5f?1:(route==0?5:route==1?7:4);
    int accent=last?0:tick%ppq==0?-6:i%2?-27:-16;
    int v=std::max(20,std::min(127,velocity+accent+int(random()%7)-3));
    emit(tick,role,v,role==1?50:75);
   }
   tick+=stride;++i;
  }
 }
};
}
