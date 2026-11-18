// SPDX-License-Identifier: MIT
// Original broad low-mid density stage. No sub oscillator or sub-octave synthesis.
#pragma once
#include <algorithm>
#include <cmath>
namespace donut {
class BodyTone {
public:
 void prepare(float sr){loCoeff_=1.f-std::exp(-6.28318530718f*140.f/sr);hiCoeff_=1.f-std::exp(-6.28318530718f*650.f/sr);reset();}
 void reset(){lo_=hi_=0;}
 float process(float x,float amount){
  amount=std::clamp(amount,0.f,1.f);lo_+=loCoeff_*(x-lo_);hi_+=hiCoeff_*(x-hi_);
  if(std::abs(lo_)<1e-25f)lo_=0;if(std::abs(hi_)<1e-25f)hi_=0;
  const float band=1.6f*(hi_-lo_);const float dense=band/(1.f+.3f*std::abs(band));
  return (x+.65f*amount*dense)/(1.f+.18f*amount);
 }
private:float lo_=0,hi_=0,loCoeff_=.02f,hiCoeff_=.09f;
};
}
