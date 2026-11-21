#pragma once
#include "../../VoiceTypes.h"
namespace felt::clav {
struct Mode {
 bool pitchable=true,muted=false;float lastPitch=-1;
 double c=0,rr=0,y1=0,y2=0;
 float ratio=1,amplitude=0;double radius=0,angle=0,cosine=1,sine=0;
 float input=0,output=0,drive=0,damping=1,damperTarget=1;
 void configure(float hz,float t60,float sr,bool excite,float a){
  lastPitch=-1;y1=y2=0;damping=damperTarget=1;muted=hz>=sr*.40f||hz<8;
  if(muted){c=rr=drive=output=0;return;}
  const double r=std::exp(-6.907755278982/(std::max(.012f,t60)*double(sr)));
  radius=r;angle=double(2*Pi)*hz/sr;cosine=std::cos(angle);sine=std::sin(angle);c=2*r*std::cos(double(2*Pi)*hz/sr);rr=r*r;
  if(excite){y1=a;y2=a*std::cos(double(angle))/r;}
 }
 void pitch(float relative){
  if(relative==lastPitch)return;
  if(!pitchable||(drive==0&&output==0))return;
  lastPitch=relative;
  muted=angle*relative>=Pi*.80f;
  const double d=std::min(double(Pi*.80f),double(angle)*relative),delta=d-angle,d2=delta*delta;
  const double cs=std::abs(delta)<.25?double(cosine)*(1-.5*d2+d2*d2/24)-double(sine)*(delta-delta*d2/6+delta*d2*d2/120):std::cos(d);
  c=2*radius*cs;
 }
 float tick(float force=0,float ramp=.007f){
  double y;
  if(damperTarget==1&&damping==1)y=c*y1-rr*y2+double(drive)*force;
  else{damping+=(damperTarget-damping)*ramp;y=c*damping*y1-rr*damping*damping*y2+double(drive)*force;}
  y2=y1;y1=std::abs(y)<1e-24?0:y;
  return muted?0:float(y1)*output;
 }
};
// Original finite-duration tangent/anvil excitation surrogate. It supplies a
// bounded external impulse; this is not reciprocal hammer/string integration.
struct Tangent {
 float position=0,velocity=0,mass=.0012f,stiffness=1,age=0,lastForce=0;
 float impulse=0,duration=.0006f;bool active=false;
 void strike(float speed,float weight,float hard){
  position=age=lastForce=0;velocity=speed;mass=weight;
  impulse=mass*speed;stiffness=hard;active=true;
 }
 float tick(float /*surface*/,float dt){
  if(!active){lastForce=0;return 0;}
  // Integral of 30*u^2*(1-u)^2 over [0,1] is one.
  float u=clamp((age+.5f*dt)/duration,0.f,1.f),q=u*(1-u);
  lastForce=(30*impulse/duration)*q*q;age+=dt;
  if(age>=duration)active=false;
  return lastForce;
 }
};
using Hammer=Tangent; // Keep the private engine inspection interface stable.
}
