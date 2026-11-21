#pragma once
#include "../../VoiceTypes.h"
namespace felt::muted {
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
struct Hammer {
 float position=0,velocity=0,mass=.004f,stiffness=8e7f,age=0,lastForce=0,loss=0,previousSurface=0;double dissipated=0;bool active=false;
 void strike(float speed,float weight,float hard){position=0;velocity=speed;mass=weight;stiffness=3e7f+hard*hard*2.7e8f;age=lastForce=loss=previousSurface=0;dissipated=0;active=true;}
 float tick(float surface,float dt){
  if(!active)return 0;
  float force=0;
  if(loss>0){
   float surfaceVelocity=(surface-previousSurface)/dt;
   for(int i=0;i<4;++i){float plane=previousSurface+(surface-previousSurface)*(i+.5f)*.25f;float compression=std::max(0.f,position-plane);float elastic=std::min(600.f,stiffness*compression*compression*(1+std::min(2.f,compression*500)));float relative=velocity-surfaceVelocity;float f=std::max(0.f,std::min(600.f,elastic*(1+loss*relative)));dissipated+=std::max(0.f,(f-elastic)*relative)*dt*.25;velocity-=f/mass*(dt*.25f);position+=velocity*(dt*.25f);force+=f*.25f;}
   previousSurface=surface;
  }else for(int i=0;i<4;++i){float compression=std::max(0.f,position-surface);float f=std::min(600.f,stiffness*compression*compression*(1+std::min(2.f,compression*500)));
   velocity-=f/mass*(dt*.25f);position+=velocity*(dt*.25f);force+=f*.25f;
  }
  age+=dt;lastForce=force;
  if((age>.00015f&&position<=surface&&velocity<0)||age>.012f){active=false;lastForce=0;}
  return force;
 }
};
}
