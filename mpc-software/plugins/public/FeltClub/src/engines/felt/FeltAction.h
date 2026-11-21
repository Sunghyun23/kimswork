#pragma once
#include "../../VoiceTypes.h"
#include <array>

namespace felt::feltpiano {
// Short radiation of the action rail and case in response to the strike.
// Their modal frequencies stay fixed when the note or its tuning changes.
struct Action {
 struct Resonator {
  float cosine=0, sine=0, displacement=0, quadrature=0;
  void prepare(float hz,float t60,float rate) {
   const float radius=std::exp(-6.9077553f/(t60*rate));
   const float angle=2*Pi*hz/rate;
   cosine=radius*std::cos(angle);
   sine=radius*std::sin(angle);
  }
  void reset() {displacement=quadrature=0;}
  float tick(float impulse) {
   const float next=cosine*displacement+sine*quadrature;
   quadrature=cosine*quadrature-sine*displacement+impulse;
   displacement=next;
   return next;
  }
 };
 static constexpr std::array<float,6> caseFrequencies{96.f,146.f,207.f,294.f,384.f,478.f};
 static constexpr std::array<float,6> caseDecay{.140f,.128f,.145f,.137f,.108f,.094f};
 static constexpr std::array<float,6> caseRadiation{.22f,.18f,.21f,.19f,.13f,.07f};
 std::array<Resonator,caseFrequencies.size()> caseModes{};
 float remainingImpulse=0,elapsed=1;
 float caseGain=0;

 void prepare(float rate) {
  for(size_t i=0;i<caseModes.size();++i)caseModes[i].prepare(caseFrequencies[i],caseDecay[i],rate);
  reset();
 }
 void reset() {
  for(auto& mode:caseModes)mode.reset();
  remainingImpulse=0;
  elapsed=1;
 }
 void strike(int note,float board,float mass,float speed) {
  reset();
  elapsed=0;
  remainingImpulse=2*mass*speed;
  const float middle=clamp((note-48)/12.f,0,1)*clamp((86-note)/21.f,0,1);
  const float treble=clamp((note-74)/16.f,0,1);
  caseGain=(1.4f+32.f*middle+58.f*treble*treble)*(.85f+.5f*board);
 }
 float tick(float force,float dt) {
  if(elapsed>=.18f)return 0;
  elapsed+=dt;
  const float impulse=std::min(remainingImpulse,std::max(0.f,force)*dt);
  remainingImpulse-=impulse;
  float pressure=0;
  for(size_t i=0;i<caseModes.size();++i)pressure+=caseRadiation[i]*caseModes[i].tick(impulse*caseGain);
  return pressure*clamp((.18f-elapsed)/.025f,0,1);
 }
};
}
