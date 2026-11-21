#pragma once
#include "dsp/adsr.h"
#include "MutedCore.h"
#include "MutedVoicing.h"
#include <array>
#include <cstdint>
namespace felt {
struct MutedVoice : VoiceState {
 static constexpr int MaxModes=32;
 std::array<muted::Mode,MaxModes> modes{};
 std::array<std::array<double,MaxModes>,2> palmShape{};
 std::array<double,2> palmFactor{};
 std::array<double,MaxModes> palmCos{},palmVelocityScale{},palmStateScale{};
 float palmRelative=1;double palmCross=0;
 // Original two-point viscous palm patch. Each velocity projection is passive.
 void cachePalmPhase(float relative){
  palmRelative=relative;palmCross=0;
  for(int n=0;n<count;++n){
   const auto&m=modes[n];
   if(m.muted||m.radius<=0){palmCos[n]=palmVelocityScale[n]=palmStateScale[n]=0;continue;}
   const double cs=std::max(-.999999999999,std::min(.999999999999,m.c/(2*m.radius)));
   const double sn=std::sqrt(1-cs*cs),omega=std::max(1.,m.angle*relative*sr);
   palmCross+=palmShape[0][n]*palmShape[1][n];
   palmCos[n]=cs;palmVelocityScale[n]=omega/sn;palmStateScale[n]=sn/(m.radius*omega);
  }
 }
 double palmVelocity(int n)const{
  const auto&m=modes[n];
  if(m.muted)return 0;
  return palmVelocityScale[n]*(palmCos[n]*m.y1-m.radius*m.damping*m.y2);
 }
 void dampPalm(){
  // Algebraically identical to two passive projections, using their cross term.
  double v0=0,v1=0;
  for(int n=0;n<count;++n){double v=palmVelocity(n);v0+=palmShape[0][n]*v;v1+=palmShape[1][n]*v;}
  const double impulse0=palmFactor[0]*v0;
  const double impulse1=palmFactor[1]*(v1-palmCross*impulse0);
  for(int n=0;n<count;++n)if(!modes[n].muted){
   const double inverseDamping=modes[n].damping==1?1:1/std::max(1e-9,double(modes[n].damping));
   modes[n].y2+=(palmShape[0][n]*impulse0+palmShape[1][n]*impulse1)*palmStateScale[n]*inverseDamping;
  }
 }
 mpc::dsp::ADSR amp;muted::Hammer hammer;
 VoiceSettings settings{};
 float reedLow=0,reedPole=.01f,previousPickup=0,pickupLp=0,dt=1.f/44100,damperRamp=.007f;
 float keyNoise=0;int keyNoiseLeft=0;
 float brightnessTarget=.1f,circuitBlend=.5f,circuitBlendTarget=.5f,toneRamp=.004f;
 float sr=44100,lp=0,brightness=.1f,noiseState=0;
 float modalMass=.02f,contactPosition=0,linkSpring=0,linkDamping=0,pickupGap=.001f,hammerNoise=0,pluckGain=1,keyBalance=1;
 int count=0;

 uint32_t random=0x46454c54;
 float noise(){random^=random<<13;random^=random>>17;random^=random<<5;return float(random>>8)*(2.f/16777216.f)-1.f;}
 void prepare(float rate){sr=rate;dt=1/rate;damperRamp=1-std::exp(-dt/.003f);toneRamp=1-std::exp(-dt/.005f);amp.setSampleRate(rate);}
 // One tone law for new and already sounding voices. A variable second pole
 // preserves dark settings without imposing a permanently closed cabinet.
 void setTone(float value,bool immediate=false){
  settings.tone=clamp(value,0,1);float top=18000.f;
  float cutoff=std::min(sr*.38f,1100.f+(top-1100.f)*settings.tone);
  brightnessTarget=1-std::exp(-2*Pi*cutoff/sr);circuitBlendTarget=.85f-.70f*settings.tone;
  if(immediate){brightness=brightnessTarget;circuitBlend=circuitBlendTarget;}
 }
 void stop(){amp.reset();active=down=sustained=false;owner=-1;lp=reedLow=pickupLp=previousPickup=energy=bodySend=hammerNoise=keyNoise=0;keyNoiseLeft=0;palmRelative=1;hammer.active=false;for(auto&m:modes)m.y1=m.y2=0;}
 void configureModes(bool excite){
  const auto&voicing=mutedVoicing;
  count=32;
  const float keyVariation=1+.012f*std::sin(note*1.731f+5*.37f);
  modalMass=.0035f*std::exp2((60-note)/48.f)*keyVariation;
  pickupGap=.0018f/(1+settings.character4*.55f);pickupGap*=voicing.gap;
  for(int n=0;n<count;++n){
   auto&m=modes[n];float ratio=1,t60=1,input=1,output=1,initial=0;

    int h=n+1;float stiffness=.000022f;ratio=h*std::sqrt((1+stiffness*h*h)/(1+stiffness));
    float position=.08f+.35f*settings.pick;input=std::sin(Pi*h*position);
    // One virtual magnetic pickup; its position creates the string comb.
    output=std::sin(Pi*h*.24f);
    output/=1+.018f*h*h;
    t60=2.f+settings.body*3.f;
    t60/=(1+.025f*h+.004f*h*h);
    {float height=(.00007f+.00055f*velocity)*(1-.4f*settings.character4);initial=2*height*input*(1+.05f*settings.character3*h)/((1+.002f*(1-settings.character3)*h*h)*(1+h*h*(.025f+.10f*(1-velocity))*(1-.7f*settings.hammer))*Pi*Pi*h*h*position*(1-position));}
   // A finite, velocity-sensitive pick width softens high modal excitation.
   float width=.006f+.018f*(1-settings.hammer)*(1-.5f*velocity);
   float phase=Pi*h*width;
   initial*=std::sin(phase)/phase;
   if(ratio>2.5f)output*=voicing.upper;
   if(voicing.tilt!=0)output*=std::pow(ratio,voicing.tilt);
   t60*=.90f+.45f*settings.resonance;
   m.pitchable=true;m.ratio=ratio;m.input=input;m.output=output;m.amplitude=output;
   m.configure(frequency*ratio,t60,sr,excite,initial);
   m.drive=m.muted?0:input*dt*dt/modalMass;
  }
  while(count>1&&modes[count-1].drive==0&&modes[count-1].output==0)--count;
  // Approximate a finite hand contact by two damping points near the bridge.
  const float contact=.055f+.015f*settings.character4;
  const double resistance=.9+10*settings.character4;
  for(int patch=0;patch<2;++patch){
   double norm=0;
   const float position=contact+(patch==0?-.012f:.012f);
   for(int n=0;n<count;++n){
    double a=modes[n].muted?0:std::sin(double(Pi)*(n+1)*position);
    palmShape[patch][n]=a;norm+=a*a;
   }
   palmFactor[patch]=norm>0?-std::expm1(-.5*resistance*norm/(modalMass*sr))/norm:0;
  }
  cachePalmPhase(1);
  linkSpring=linkDamping=0;
  // Level balance after velocity-sensitive pickup/radiation: retain headroom.
  pluckGain=2320.f;
 }
 
 void strike(int pitch,int velocity127,int source,int serial,const VoiceSettings&s,float tuning=440){
  stop();this->tuning=tuning;settings=s;settings.model=5;note=pitch;owner=source;age=serial;velocity=clamp(velocity127/127.f,.01f,1);frequency=initialFrequency=targetFrequency=bendHz(note,tuning);ageSeconds=0;
  pan=clamp((pitch-60)/42.f,-.65f,.65f);random=0x46454c54u+uint32_t(serial*1664525u+pitch*1013904223u);
  reedPole=1-std::exp(-2*Pi*25.f/sr);setTone(settings.tone,true);
  float speed=.35f+4.2f*std::pow(velocity,.85f+.45f*settings.velocity);
  float mass=.0018f*std::exp2((60-note)/28.f);
  float hardness=clamp(settings.hammer*1.f+.10f*velocity,.04f,1);
  hammer.strike(speed,mass,hardness);hammer.stiffness*=clamp(std::exp2((note-60)/6.f),.12f,256.f);
  keyBalance=1.f;hammer.active=false;
  keyBalance*=clamp(std::exp2((note-60)*0.f/12),.5f,3.f);
  amp.set(settings.attack,settings.decay,settings.sustain,settings.release);amp.noteOn();down=active=true;configureModes(true);
  for(int n=0;n<count;++n)if(!modes[n].muted)previousPickup+=float(modes[n].y1)*modes[n].output;
 }
 void keyOff(float amount){keyNoise=amount;keyNoiseLeft=int(sr*.012f);}
 void release(bool pedal){
  down=false;if(pedal){sustained=true;return;}sustained=false;amp.noteOff();
  // Actual damper changes modal losses, high modes settle first. Ramp contact to
  // avoid a discontinuity; ADSR Release remains the user's maximum release gate.
  // The circuit envelope already damps the fundamental. Do not multiply it
  // by another full60dB over the same time and prematurely chop the tail.
  for(int n=0;n<count;++n){float seconds=std::max(.018f,settings.release*.001f*3.f)/(1+.045f*modes[n].ratio);modes[n].damperTarget=std::exp(-6.9077553f/(seconds*sr));}
 }
 void setEnvelope(float a,float d,float s,float r){settings.attack=a;settings.decay=d;settings.sustain=s;settings.release=r;amp.set(a,d,s,r);}
 void retune(int pitch,float milliseconds,float tuning=440){note=pitch;targetFrequency=bendHz(pitch,tuning);glide=1-std::exp(-1/(std::max(1.f,milliseconds)*.001f*sr));}
 void updatePitch(float ratio){
  if(glide>0)frequency+=(targetFrequency-frequency)*std::min(1.f,glide*32);else frequency=targetFrequency;
  float relative=std::max(.1f,std::min(8.f,frequency/initialFrequency*ratio));for(int n=0;n<count;++n)modes[n].pitch(relative);
  if(relative!=palmRelative)cachePalmPhase(relative);
 }
 
 float tick(){
  if(!active)return 0;
  float surface=0;if(hammer.active)for(int n=0;n<count;++n)surface+=modes[n].input*float(modes[n].y1);
  float force=hammer.tick(surface,dt),x=0;
  for(int n=0;n<count;++n)x+=modes[n].tick(force,damperRamp);
  dampPalm();
  float displacement=x;float motion=(x-previousPickup)*sr/(2*Pi*std::max(20.f,frequency));previousPickup=displacement;
  x=motion*pluckGain;
  // Pickup circuit/cabinet low-pass and dc blocking, not fixed synthetic dirt.
  brightness+=(brightnessTarget-brightness)*toneRamp;circuitBlend+=(circuitBlendTarget-circuitBlend)*toneRamp;
  pickupLp+=brightness*(x-pickupLp);lp+=brightness*(pickupLp-lp);x=pickupLp+(lp-pickupLp)*circuitBlend;
  reedLow+=reedPole*(x-reedLow);x-=reedLow;
  hammerNoise+=.12f*(force*.0005f-hammerNoise);
  if(hammer.active)x+=settings.noise*hammerNoise*noise()*.20f;
  if(keyNoiseLeft>0){x+=keyNoise*noise()*.002f*keyNoiseLeft/(sr*.012f);--keyNoiseLeft;}
  float env=amp.process();x*=env*keyBalance*mutedVoicing.level;bodySend=0;
  ageSeconds+=dt;energy+=.002f*(std::abs(x)-energy);
  if(!amp.isActive()||(!down&&!sustained&&energy<1e-7f&&ageSeconds>.15f)||ageSeconds>45)stop();
  return x;
 }
};
}
