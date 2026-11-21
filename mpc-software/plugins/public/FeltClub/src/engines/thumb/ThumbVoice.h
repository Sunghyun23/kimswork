#pragma once
#include "dsp/adsr.h"
#include "ThumbCore.h"
#include "ThumbBeam.h"
#include "ThumbVoicing.h"
#include <array>
#include <cstdint>
namespace felt {
// Reference-guided coefficients are fixed; no tuning state is shared.
struct ThumbVoice : VoiceState {
 static constexpr int MaxModes=12;
 std::array<thumb::Mode,MaxModes> modes{};
 mpc::dsp::ADSR amp;thumb::Hammer hammer;
 VoiceSettings settings{};
 float reedLow=0,reedPole=.01f,previousPickup=0,pickupLp=0,dt=1.f/44100,damperRamp=.007f;
 float keyNoise=0;int keyNoiseLeft=0;
 float metalRise=0,metalRiseCoefficient=.02f,previousMetalPickup=0;
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
  settings.tone=clamp(value,0,1);float top=5000.00000000f;
  float cutoff=std::min(sr*.38f,1100.f+(top-1100.f)*settings.tone);
  brightnessTarget=1-std::exp(-2*Pi*cutoff/sr);circuitBlendTarget=.85f-.70f*settings.tone;
  if(immediate){brightness=brightnessTarget;circuitBlend=circuitBlendTarget;}
 }
 void stop(){amp.reset();active=down=sustained=false;owner=-1;lp=reedLow=pickupLp=previousPickup=energy=bodySend=hammerNoise=keyNoise=0;keyNoiseLeft=0;metalRise=previousMetalPickup=0;hammer.active=false;for(auto&m:modes)m.y1=m.y2=0;}
 void configureModes(bool excite){
  const auto&voicing=thumbVoicing;
  count=11;
  const float keyVariation=1+.012f*std::sin(note*1.731f+6*.37f);
  modalMass=.0035f*std::exp2((60-note)/48.f)*keyVariation;
  pickupGap=.0018f/(1+settings.character4*.55f);pickupGap*=voicing.gap;
  // Smooth empirical register loss: upper lamellae retain a longer ring.
  const float registerDecay=std::exp2(clamp((note-64)*.27f,-.4f,2.2f));
  for(int n=0;n<count;++n){
   auto&m=modes[n];float ratio=1,t60=1,input=1,output=1,initial=0;

    if(n<int(thumb::BeamRatios.size())){
     ratio=thumb::BeamRatios[n];
     input=thumb::PluckShape[n];
     output=thumb::TipShape[n]*(n==0?1.f:((n==1?.38f:2.2f)*(.6f+settings.character3))*std::sqrt(ratio));
     // Static bending under a fingertip load falls as inverse modal stiffness.
     // Firm release retains more upper-mode excitation.
     initial=(.00004f+.00048f*std::pow(velocity,.85f+.45f*settings.velocity))*input/
             (ratio*ratio*(1+.14f*(1-velocity)*ratio));
     t60=registerDecay*(.8f+.4f*settings.body)/(.85f+settings.character4*.75f);
     // Do not make upper metal ring longer with the fundamental on high keys.
     if(n==1)t60=(.8f+.4f*settings.body)*std::min(registerDecay,std::exp2(clamp((64-note)/24.f,-1.f,0.f)))/
                         ((.85f+settings.character4*.75f)*(1+.08f*(ratio-1)));
     if(n>1)t60=(.075f+.055f*settings.body)*std::exp2(clamp((64-note)/8.f,-1.5f,1.f))/
                       (std::sqrt(std::max(1.f,ratio/6.66664531f))*(.85f+settings.character4*.75f));
    }else if(n<8){
     // The weak adjacent resonance tapers with register and touch.
     // The strong, short metal path retains the previous soft release.
     // The original upper bending mode supplies its quieter, longer tail.
     // This is an empirical modal extension, not a measured 3D geometry.
     ratio=n==6?1.f+.075f*std::exp2(clamp((64-note)/48.f,-.5f,.35f)):6.78f;
     input=n==6?.85f:.8f;
     output=n==6?.15f*(.72f+.28f*std::sqrt(velocity)):3.8f*(.82f+.18f*velocity)*std::sqrt(ratio)*(.6f+settings.character3);
     initial=(.00004f+.00048f*std::pow(velocity,.85f+.45f*settings.velocity))*input/
             (ratio*ratio*(1+.14f*(1-velocity)*ratio));
     t60=n==6?.55f:(.065f+.05f*settings.body)*std::exp2(clamp((64-note)/8.f,-1.5f,1.f));
     t60/=.85f+settings.character4*.75f;
    }else{
     // Short, fixed-frequency body modes receive the release impulse.
     // Their level/loss and the readout are empirical reference voicing.
     constexpr float bodyHz[]={190.f,575.f,1410.f};
     constexpr float bodyGain[]={.025f,.012f,.006f};
     const int body=n-8;
     ratio=bodyHz[body]/frequency;
     input=0;output=bodyGain[body]*(.4f+settings.body)*2.f;
     initial=(.00004f+.00048f*velocity)*(.30f+.70f*settings.hammer);
     t60=.16f+.10f*settings.body;
    }

   if(ratio>2.5f)output*=voicing.upper;
   if(voicing.tilt!=0)output*=std::pow(ratio,voicing.tilt);
   t60*=.90f+.45f*settings.resonance;
   m.pitchable=n<8;m.ratio=ratio;m.input=input;m.output=output;m.amplitude=output;
   m.configure(frequency*ratio,t60,sr,excite,initial);
   m.drive=m.muted?0:input*dt*dt/modalMass;
  }
  while(count>1&&modes[count-1].drive==0&&modes[count-1].output==0)--count;
  linkSpring=linkDamping=0;
  // Level balance after velocity-sensitive pickup/radiation: retain headroom.
  pluckGain=1060.f;
 }
 
 void strike(int pitch,int velocity127,int source,int serial,const VoiceSettings&s,float tuning=440){
  stop();this->tuning=tuning;settings=s;settings.model=6;note=pitch;owner=source;age=serial;velocity=clamp(velocity127/127.f,.01f,1);frequency=initialFrequency=targetFrequency=bendHz(note,tuning);ageSeconds=0;
  // Finite thumb release rounds only the fast metal path.
  metalRiseCoefficient=1-std::exp(-1/(sr*(.0009f+.0012f*(1-velocity))));
  pan=clamp((pitch-60)/42.f,-.65f,.65f);random=0x46454c54u+uint32_t(serial*1664525u+pitch*1013904223u);
  reedPole=1-std::exp(-2*Pi*25.f/sr);setTone(settings.tone,true);
  float speed=.35f+4.2f*std::pow(velocity,.85f+.45f*settings.velocity);
  float mass=.0015f*std::exp2((60-note)/28.f);
  float hardness=clamp(settings.hammer*1.f+.10f*velocity,.04f,1);
  hammer.strike(speed,mass,hardness);hammer.stiffness*=clamp(std::exp2((note-60)/6.f),.12f,256.f);
  keyBalance=1.f;hammer.active=false;
  keyBalance*=clamp(std::exp2((note-60)*0.f/12),.5f,3.f);
  amp.set(settings.attack,settings.decay,settings.sustain,settings.release);amp.noteOn();down=active=true;configureModes(true);
  for(int n=0;n<count;++n)if(!modes[n].muted){float initial=float(modes[n].y1)*modes[n].output;previousPickup+=initial;if(n==7)previousMetalPickup=initial;}
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
 }
 
 float tick(){
  if(!active)return 0;
  float surface=0;if(hammer.active)for(int n=0;n<count;++n)surface+=modes[n].input*float(modes[n].y1);
  float force=hammer.tick(surface,dt),x=0;
  metalRise+=(1-metalRise)*metalRiseCoefficient;
  float metal=0;
  for(int n=0;n<count;++n){float mode=modes[n].tick(force,damperRamp);x+=mode;if(n==7)metal=mode;}
  float displacement=x;const float velocityScale=sr/(2*Pi*std::max(20.f,frequency));
  float motion=(x-previousPickup)*velocityScale;
  float metalMotion=(metal-previousMetalPickup)*velocityScale;
  previousPickup=displacement;previousMetalPickup=metal;
  // Round the radiated metal path after differentiation: ramping displacement
  // itself would create an artificial onset velocity/DC step.
  x=(.85f*motion+.15f*x+(metalRise-1)*(.85f*metalMotion+.15f*metal))*1.15857f*pluckGain;

  // Pickup circuit/cabinet low-pass and dc blocking, not fixed synthetic dirt.
  brightness+=(brightnessTarget-brightness)*toneRamp;circuitBlend+=(circuitBlendTarget-circuitBlend)*toneRamp;
  pickupLp+=brightness*(x-pickupLp);lp+=brightness*(pickupLp-lp);x=pickupLp+(lp-pickupLp)*circuitBlend;
  reedLow+=reedPole*(x-reedLow);x-=reedLow;
  hammerNoise+=.12f*(force*.0005f-hammerNoise);
  if(hammer.active)x+=settings.noise*hammerNoise*noise()*.20f;
  if(keyNoiseLeft>0){x+=keyNoise*noise()*.002f*keyNoiseLeft/(sr*.012f);--keyNoiseLeft;}
  float env=amp.process();x*=env*keyBalance*thumbVoicing.level;bodySend=0;
  ageSeconds+=dt;energy+=.002f*(std::abs(x)-energy);
  if(!amp.isActive()||(!down&&!sustained&&energy<1e-7f&&ageSeconds>.15f)||ageSeconds>45)stop();
  return x;
 }
};
}
