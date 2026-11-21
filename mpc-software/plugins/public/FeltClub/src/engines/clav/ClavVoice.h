#pragma once
#include "dsp/adsr.h"
#include "ClavCore.h"
#include "ClavVoicing.h"
#include <array>
#include <cstdint>
namespace felt {
struct ClavVoice : VoiceState {
 static constexpr int MaxModes=32;
 std::array<clav::Mode,MaxModes> modes{};
 std::array<float,MaxModes> centerPickup{},bridgePickup{};
 float previousCenter=0,previousBridge=0,pickupBlend=.4f;
 float releaseRatio=1,releaseTarget=1,releaseStep=.01f,lastBend=1;int releaseCounter=0;
 mpc::dsp::ADSR amp;clav::Hammer hammer;
 VoiceSettings settings{};
 float reedLow=0,reedPole=.01f,previousPickup=0,pickupLp=0,dt=1.f/44100,damperRamp=.007f;
 float keyNoise=0;int keyNoiseLeft=0;
 float brightnessTarget=.1f,circuitBlend=.5f,circuitBlendTarget=.5f,toneRamp=.004f;
 float sr=44100,lp=0,brightness=.1f,noiseState=0;
 float modalMass=.02f,contactPosition=0,linkSpring=0,linkDamping=0,pickupGap=.001f,hammerNoise=0,pluckGain=1,keyBalance=1;
 int count=0;

 uint32_t random=0x46454c54;
 float noise(){random^=random<<13;random^=random>>17;random^=random<<5;return float(random>>8)*(2.f/16777216.f)-1.f;}
 void prepare(float rate){sr=rate;dt=1/rate;damperRamp=1-std::exp(-dt/.003f);toneRamp=1-std::exp(-dt/.005f);releaseStep=1-std::exp(-dt/.0025f);amp.setSampleRate(rate);}
 // One tone law for new and already sounding voices. A variable second pole
 // preserves dark settings without imposing a permanently closed cabinet.
 void setTone(float value,bool immediate=false){
  settings.tone=clamp(value,0,1);float top=9000.0f;
  float cutoff=std::min(sr*.38f,1100.f+(top-1100.f)*settings.tone);
  brightnessTarget=1-std::exp(-2*Pi*cutoff/sr);circuitBlendTarget=0.9200f-0.4000f*settings.tone;
  if(immediate){brightness=brightnessTarget;circuitBlend=circuitBlendTarget;}
 }
 void stop(){amp.reset();active=down=sustained=false;owner=-1;lp=reedLow=pickupLp=previousPickup=energy=bodySend=hammerNoise=keyNoise=0;keyNoiseLeft=0;previousCenter=previousBridge=0;releaseRatio=releaseTarget=lastBend=1;releaseCounter=0;hammer.active=false;for(auto&m:modes)m.y1=m.y2=0;}
 void configureModes(bool /*excite*/){
  const auto&voicing=clavVoicing;
  count=32;
  const float keyVariation=1+.012f*std::sin(note*1.731f+4*.37f);
  modalMass=.0035f*std::exp2((60-note)/48.f)*keyVariation;
  pickupGap=.0018f/(1+settings.character4*.55f);pickupGap*=voicing.gap;
  pickupBlend=0.0500f+0.5000f*settings.character3;
  const float registerDecay=clamp(std::exp2((60-note)/24.f),.35f,2.f);
  for(int n=0;n<count;++n){
   auto&m=modes[n];float ratio=1,t60=1,input=1,output=1,initial=0;

    int h=n+1;
    // Empirical register dispersion, not measured string gauges/tension.
    float stiffness=.00008f*std::exp2(clamp((52-note)/12.f,-3.f,3.f));
    ratio=h*std::sqrt((1+stiffness*h*h)/(1+stiffness));
    float position=.08f+.35f*settings.pick;input=std::sin(Pi*h*position);
    float aperture=1/(1+.008f*h*h);
    float center=std::sin(Pi*h*.20f)*aperture;
    float bridge=std::sin(Pi*h*.08f)*aperture;
    float color=ratio>2.5f?voicing.upper:1.f;
    if(voicing.tilt!=0)color*=std::pow(ratio,voicing.tilt);
    centerPickup[n]=center*color;bridgePickup[n]=bridge*color;
    output=centerPickup[n]+pickupBlend*bridgePickup[n];
    float h2=float(h*h);
    t60=(8.f+14.f*settings.body)*registerDecay/
        ((1+.010f*h+.003f*h2+.00004f*h2*h2)*(1+settings.character4*9.f));
    t60*=.90f+.45f*settings.resonance;
   m.pitchable=true;m.ratio=ratio;m.input=input;m.output=output;m.amplitude=output;
   m.configure(frequency*ratio,t60,sr,false,initial);
   if(m.muted)centerPickup[n]=bridgePickup[n]=0;
   // Finite contact compliance softens the upper-mode impulse distribution.
   m.drive=m.muted?0:input*dt*dt/(modalMass*std::sqrt(float(n+1)));
  }
  while(count>1&&modes[count-1].drive==0&&modes[count-1].output==0)--count;
  linkSpring=linkDamping=0;
  // Level balance after velocity-sensitive pickup/radiation: retain headroom.
  pluckGain=1.f;
 }
 
 void strike(int pitch,int velocity127,int source,int serial,const VoiceSettings&s,float tuning=440){
  stop();this->tuning=tuning;settings=s;settings.model=4;note=pitch;owner=source;age=serial;velocity=clamp(velocity127/127.f,.01f,1);frequency=initialFrequency=targetFrequency=bendHz(note,tuning);ageSeconds=0;
  pan=clamp((pitch-60)/42.f,-.65f,.65f);random=0x46454c54u+uint32_t(serial*1664525u+pitch*1013904223u);
  reedPole=1-std::exp(-2*Pi*25.f/sr);setTone(settings.tone,true);
  float speed=.35f+4.2f*std::pow(velocity,.85f+.45f*settings.velocity);
  float mass=.0012f*std::exp2((60-note)/28.f);
  float hardness=clamp(settings.hammer*1.f+.10f*velocity,.04f,1);
  hammer.strike(speed,mass,hardness);
  hammer.duration=clamp((.00045f+.00035f*(1-hardness))*std::exp2((60-note)/36.f),.00015f,.0016f);
  keyBalance=clamp(std::exp2((note-60)*.18f/12),.65f,1.7f);
  amp.set(settings.attack,settings.decay,settings.sustain,settings.release);amp.noteOn();down=active=true;configureModes(true);
  
 }
 void keyOff(float amount){keyNoise=amount;keyNoiseLeft=int(sr*.012f);}
 void release(bool pedal){
  down=false;if(pedal){sustained=true;return;}sustained=false;amp.noteOff();
  hammer.active=false;hammer.lastForce=0;releaseTarget=.8408964153f;releaseCounter=0;
  // Actual damper changes modal losses, high modes settle first. Ramp contact to
  // avoid a discontinuity; ADSR Release remains the user's maximum release gate.
  // The circuit envelope already damps the fundamental. Do not multiply it
  // by another full60dB over the same time and prematurely chop the tail.
  for(int n=0;n<count;++n){float seconds=(.04f+.12f*(1-settings.character4))*clamp(settings.release/420.f,.25f,4.f)/(1+.06f*modes[n].ratio);modes[n].damperTarget=std::exp(-6.9077553f/(seconds*sr));}
 }
 void setEnvelope(float a,float d,float s,float r){settings.attack=a;settings.decay=d;settings.sustain=s;settings.release=r;amp.set(a,d,s,r);}
 void retune(int pitch,float milliseconds,float tuning=440){note=pitch;targetFrequency=bendHz(pitch,tuning);glide=1-std::exp(-1/(std::max(1.f,milliseconds)*.001f*sr));}
 void updatePitch(float ratio){
  lastBend=ratio;
  if(glide>0)frequency+=(targetFrequency-frequency)*std::min(1.f,glide*32);else frequency=targetFrequency;
  float relative=std::max(.1f,std::min(8.f,frequency/initialFrequency*ratio*releaseRatio));for(int n=0;n<count;++n)modes[n].pitch(relative);
 }
 
 float tick(){
  if(!active)return 0;
  if(releaseTarget<1&&releaseRatio>releaseTarget){
   releaseRatio+=(releaseTarget-releaseRatio)*releaseStep;
   bool settled=releaseRatio-releaseTarget<.00002f;
   if(settled)releaseRatio=releaseTarget;
   if(!(releaseCounter++&15)||settled){
    float relative=clamp(frequency/initialFrequency*lastBend*releaseRatio,.1f,8.f);
    for(int n=0;n<count;++n)modes[n].pitch(relative);
   }
  }
  float force=hammer.tick(0,dt),center=0,bridge=0;
  for(int n=0;n<count;++n){
   modes[n].tick(force,damperRamp);
   if(!modes[n].muted){float y=float(modes[n].y1);center+=y*centerPickup[n];bridge+=y*bridgePickup[n];}
  }
  // Bounded, opposite pickup curvature; original approximation, no field solve.
  float centerFlux=center+.20f*center*center/(pickupGap+std::abs(center));
  float bridgeFlux=bridge-.20f*bridge*bridge/(pickupGap+std::abs(bridge));
  float x=((centerFlux-previousCenter)+pickupBlend*(bridgeFlux-previousBridge))*sr*.12f;
  previousCenter=centerFlux;previousBridge=bridgeFlux;previousPickup=center+pickupBlend*bridge;
  // Pickup circuit/cabinet low-pass and dc blocking, not fixed synthetic dirt.
  brightness+=(brightnessTarget-brightness)*toneRamp;circuitBlend+=(circuitBlendTarget-circuitBlend)*toneRamp;
  pickupLp+=brightness*(x-pickupLp);lp+=brightness*(pickupLp-lp);x=pickupLp+(lp-pickupLp)*circuitBlend;
  reedLow+=reedPole*(x-reedLow);x-=reedLow;
  hammerNoise+=.12f*(force*.0005f-hammerNoise);
  if(hammer.active)x+=settings.noise*hammerNoise*noise()*.20f;
  if(keyNoiseLeft>0){x+=keyNoise*noise()*.002f*keyNoiseLeft/(sr*.012f);--keyNoiseLeft;}
  float env=amp.process();x*=env*keyBalance*clavVoicing.level;bodySend=0;
  ageSeconds+=dt;energy+=.002f*(std::abs(x)-energy);
  if(!amp.isActive()||(!down&&!sustained&&energy<1e-7f&&ageSeconds>.15f)||ageSeconds>45)stop();
  return x;
 }
};
}
