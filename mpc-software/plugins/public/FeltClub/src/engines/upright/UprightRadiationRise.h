#pragma once
#include "../../VoiceTypes.h"
#include <array>
#include <cstddef>

namespace felt::upright {
// Reduced early radiation envelope. The strike force and string states are
// independent of this observation model. Lower partials build over a longer
// interval than the prompt higher partials.
struct RadiationRise {
 static constexpr std::array<float,6> periods{6.0f,4.9f,2.65f,1.45f,.85f,.55f};
 std::array<float,periods.size()> residual{};
 std::array<float,periods.size()> pole{};
 bool active=false;

 void strike(int note,float frequency,float dt) {
  // The low bass did not support the midrange buildup law in either
  // recording set. Fade the contribution in above that register.
  const float depth=.82f*clamp((note-35)/8.f,0,1);
  active=depth>0;
  if(!active)return;
  for(std::size_t h=0;h<periods.size();++h){
   const float seconds=clamp(0.90f*periods[h]/frequency,.0015f,.042f);
   pole[h]=std::exp(-dt/seconds);
   residual[h]=depth;
  }
 }
 void reset(){active=false;}
 float advance(std::size_t harmonic){
  residual[harmonic]*=pole[harmonic];
  return 1-residual[harmonic];
 }
 void settle(){if(residual[0]<1e-5f)active=false;}
};
}
