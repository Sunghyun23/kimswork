#pragma once
#include <algorithm>
#include <cmath>
namespace felt {
// Interface/data only. No model coefficients or synthesis implementation here.
constexpr float Pi=3.14159265358979323846f;
inline float clamp(float v,float lo,float hi){return std::max(lo,std::min(v,hi));}
inline float bendHz(int note,float tuning=440){return tuning*std::exp2((note-69)/12.f);}
struct VoiceSettings {
 int model=0;
 float tone=.48f,body=.62f,character3=.28f,character4=.18f;
 float attack=5,decay=650,sustain=.7f,release=420;
 float hammer=.35f,velocity=.65f,resonance=.32f,pick=.22f,detune=1.5f,noise=0;
};
struct VoiceState {
 float tuning=440,frequency=440,initialFrequency=440,targetFrequency=440,glide=0;
 float energy=0,pan=0,velocity=0,ageSeconds=0,bodySend=0;
 int note=-1,owner=-1,age=0;
 bool down=false,sustained=false,active=false;
};
}
