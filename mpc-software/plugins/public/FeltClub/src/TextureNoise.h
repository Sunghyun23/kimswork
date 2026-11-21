#pragma once
#include "dsp/smoother.h"
#include <array>
#include <algorithm>
#include <cmath>
#include <cstdint>
namespace felt {
// Independent, continuous, gently band-limited tape-like hiss. Not mechanical
// strike noise, sampled tape, crackle or a claim of a measured machine model.
struct TextureNoise {
 mpc::dsp::Smoother amount;
 uint32_t rng=0x48535331u;
 float hp=0,lp=0,density=1;
 std::array<float,2> high{},previous{},low1{},low2{};
 bool running=false;
 void prepare(float sr){hp=std::exp(-2*3.14159265358979323846f*90/sr);lp=1-std::exp(-2*3.14159265358979323846f*8500/sr);density=std::sqrt(sr/48000.f);amount.setTime(25,sr);reset();}
 void reset(){rng=0x48535331u;amount.snap(0);high=previous=low1=low2={};running=false;}
 void set(float value){amount.setTarget(std::max(0.f,std::min(1.f,value)));}
 float white(){rng^=rng<<13;rng^=rng>>17;rng^=rng<<5;return float(rng>>8)*(2.f/16777215.f)-1;}
 std::array<float,2> process(){
  const float gain=amount.process()*.05f*density;
  if(gain==0){if(running){high=previous=low1=low2={};running=false;}return{};}
  running=true;const float common=white();std::array<float,2>out{};
  for(int c=0;c<2;++c){const float x=.78f*common+.22f*white();high[c]=x-previous[c]+hp*high[c];previous[c]=x;low1[c]+=lp*(high[c]-low1[c]);low2[c]+=lp*(low1[c]-low2[c]);out[c]=low2[c]*gain;}
  return out;
 }
};
}
