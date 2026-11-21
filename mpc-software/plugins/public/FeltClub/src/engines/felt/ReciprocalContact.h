#pragma once
#include "FeltCore.h"
#include <array>

namespace felt::feltpiano {
// Exact free modal rotations on either side of a reciprocal contact
// impulse. This is only used while the hammer can touch the strings.
template<std::size_t ModeCount>
struct ReciprocalContact {
 struct Coefficients {
  double halfCosine=0,halfSine=0;
  double phaseToQuadrature=0,previousToQuadrature=0;
  double forwardImpulse=0,backwardImpulse=0;
  double surfaceVelocityWeight=0;
 };
 std::array<Coefficients,ModeCount> coefficients{};
 double inverseEffectiveMass=0;

 void prepare(const Mode* modes,int count,double rate,double modalMass,double hammerMass) {
  inverseEffectiveMass=1/hammerMass;
  for(int i=0;i<count;++i){
   const auto& mode=modes[i];
   if(mode.muted)continue;
   auto& c=coefficients[i];
   const double theta=mode.phaseCos==mode.cosine?mode.angle:std::atan2(mode.phaseSin,mode.phaseCos);
   const double omega=theta*rate;
   const double halfRadius=std::sqrt(mode.radius);
   const double halfSine=std::sqrt(std::max(0.,.5*(1-mode.phaseCos)));
   c.halfCosine=halfRadius*std::sqrt(std::max(0.,.5*(1+mode.phaseCos)));
   c.halfSine=halfRadius*halfSine;
   c.phaseToQuadrature=mode.phaseCos/mode.phaseSin;
   c.previousToQuadrature=-mode.radius/mode.phaseSin;
   const double kick=mode.input/(modalMass*omega);
   c.forwardImpulse=halfSine*halfRadius*kick;
   c.backwardImpulse=halfSine/halfRadius*kick;
   c.surfaceVelocityWeight=mode.input*omega;
   inverseEffectiveMass+=double(mode.input)*mode.input/modalMass;
  }
 }

 float tick(Hammer& hammer,Mode* modes,int count,double dt,float damperRamp) const {
  double velocity=hammer.velocity;
  double position=hammer.position+velocity*dt*.5;
  double surface=0,surfaceVelocity=0;
  for(int i=0;i<count;++i){
   auto& mode=modes[i];
   if(mode.muted){mode.tick(0,damperRamp);continue;}
   const auto& c=coefficients[i];
   const double before=mode.damping;
   mode.damping+=(mode.damperTarget-mode.damping)*damperRamp;
   const double halfDamping=mode.damping==1.f?1.:std::sqrt(double(mode.damping));
   const double q=mode.y1;
   const double p=c.phaseToQuadrature*q+c.previousToQuadrature*before*mode.y2;
   surface+=mode.input*(c.halfCosine*q+c.halfSine*p)*halfDamping;
   surfaceVelocity+=c.surfaceVelocityWeight*(c.halfCosine*p-c.halfSine*q)*halfDamping;

   // The no-contact full step. The reciprocal impulse is added below once
   // the shared surface displacement and velocity are known.
   mode.y1=mode.radius*mode.damping*(2*mode.phaseCos*q-mode.radius*before*mode.y2);
   mode.y2=q;
  }
  const double gap=std::max(0.,position-surface);
  const double elastic=std::min(600.,hammer.stiffness*gap*gap*(1+std::min(2.,gap*500)));
  const double relative=velocity-surfaceVelocity;
  // Solve the velocity-dependent term at the midpoint of the impulse.
  const double force=std::clamp(elastic*(1+hammer.loss*relative)/
                                (1+.5*dt*elastic*hammer.loss*inverseEffectiveMass),0.,600.);
  const double impulse=dt*force;
  const double relativeMidpoint=relative-.5*impulse*inverseEffectiveMass;
  hammer.dissipated+=std::max(0.,(force-elastic)*relativeMidpoint)*dt;
  velocity-=impulse/hammer.mass;
  position+=velocity*dt*.5;

  surface=0;
  for(int i=0;i<count;++i){
   auto& mode=modes[i];
   if(mode.muted)continue;
   const auto& c=coefficients[i];
   const double halfDamping=mode.damping==1.f?1.:std::sqrt(double(mode.damping));
   mode.y1+=c.forwardImpulse*impulse*halfDamping;
   // y2 encodes quadrature at the current angle. The backward half-step
   // gives the kick's contribution to that state with the opposite sign.
   mode.y2-=c.backwardImpulse*impulse/halfDamping;
   surface+=mode.input*mode.y1;
  }

  hammer.position=float(position);
  hammer.velocity=float(velocity);
  hammer.age+=float(dt);
  hammer.lastForce=float(force);
  if((hammer.age>.00015f&&position<=surface&&velocity<0)||hammer.age>.012f){
   hammer.active=false;
   hammer.lastForce=0;
  }
  return float(force);
 }
};
}
