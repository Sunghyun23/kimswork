#pragma once
// Independent Wurli synthesis backend; no other model dependency.
#include "dsp/adsr.h"
#include "../../VoiceTypes.h"
#include "WurliCore.h"
#include "WurliVoicing.h"
#include "WurliMechanics.h"
#include <array>
#include <cstdint>
namespace felt {
struct WurliVoice : VoiceState {
 static constexpr int MaxModes=7;
 std::array<wurli::Mode,MaxModes> modes{};
 mpc::dsp::ADSR amp;wurli::Hammer hammer;WurliPickup wurliPickup;
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
 void stop(){amp.reset();active=down=sustained=false;owner=-1;lp=reedLow=pickupLp=previousPickup=energy=bodySend=hammerNoise=keyNoise=0;keyNoiseLeft=0;hammer.active=false;wurliPickup.reset();for(auto&m:modes)m.y1=m.y2=0;}
 void configureModes(bool /*excite*/){
  const auto&voicing=wurliVoicing;
  const float keyVariation=1+.012f*std::sin(note*1.731f+1*.37f);
  modalMass=.014f*std::exp2((60-note)/48.f)*keyVariation;
  modalMass*=1.15f-.40f*settings.character3;
  pickupGap=.0013f/(1+settings.character4*.55f); pickupGap*=voicing.gap;
  
  count=7;
  for(int n=0;n<count;++n){
   auto&m=modes[n];float ratio=1,t60=1,input=1,output=1;
        constexpr float r[]={1,1.0017f,5.62f,15.35f,29.8f,48.6f,1.004f};ratio=r[n];
     const float in[]={1,0,.12f,.035f,.015f,.006f,0};input=in[n];
     const float outs[]={1,0,.085f+settings.character3*.12f,.025f,.008f,.003f,0};output=outs[n];
     // Fundamental-led reed, short weak upper bending modes. Remove the
     // independently struck near-unison modes that caused synthetic beating.
     t60=(n<2?2.2f+settings.body*4.8f:.20f+settings.body*.60f)*std::exp2((60-note)/85.f)/(1+.10f*ratio);
     t60/=1+.18f*velocity*velocity;

   if(ratio>2.5f)output*=voicing.upper;
   if(voicing.tilt!=0)output*=std::pow(ratio,voicing.tilt);
   
   t60*=.90f+.45f*settings.resonance;
   m.pitchable=true;m.ratio=ratio;m.input=input;m.output=output;m.amplitude=output;
   m.configure(frequency*ratio,t60,sr,false,0);
   m.drive=m.muted?0:input*dt*dt/modalMass;
  }
  while(count>1&&modes[count-1].drive==0&&modes[count-1].output==0)--count;
  linkSpring=linkDamping=0;pluckGain=1;pickupGap*=std::exp2((60-note)/18.f);wurliPickup.configure(pickupGap,settings.character4,sr); // Canonical local reed coordinates; no unmeasured key60 flip.
 }
 
 void strike(int pitch,int velocity127,int source,int serial,const VoiceSettings&s,float tuning=440){
  stop();damperRamp=1-std::exp(-dt/.003f);this->tuning=tuning;settings=s;settings.model=1;note=pitch;owner=source;age=serial;velocity=clamp(velocity127/127.f,.01f,1);frequency=initialFrequency=targetFrequency=bendHz(note,tuning);ageSeconds=0;
  pan=clamp((pitch-60)/42.f,-.65f,.65f);random=0x46454c54u+uint32_t(serial*1664525u+pitch*1013904223u);
  reedPole=1-std::exp(-2*Pi*45.f/sr);setTone(settings.tone,true);
  float speed=.35f+4.2f*std::pow(velocity,.85f+.45f*settings.velocity)/(1+.30f*velocity*velocity);
  float mass=.0035f*std::exp2((60-note)/28.f);
  float hardness=clamp(settings.hammer*(.85f+.15f*std::sqrt(velocity))+.08f*velocity,.04f,1);
  hammer.strike(speed,mass,hardness);hammer.stiffness*=clamp(std::exp2((note-60)/6.f),.12f,256.f);hammer.stiffness*=2.5f;keyBalance=clamp(std::exp2((note-60)*(.60f+0.f)/12),.15f,8.f);
  hammer.loss=.035f+.055f*settings.hammer;
  keyBalance*=clamp(std::exp2((note-60)*-.20f/12),.5f,3.f);
  // Musical register balance is separate from physical charge/load readout.
  keyBalance*=261.6255653f/frequency;
  amp.set(settings.attack,settings.decay,settings.sustain,settings.release);amp.noteOn();down=active=true;configureModes(true);
 }
 void keyOff(float amount){keyNoise=amount;keyNoiseLeft=int(sr*.012f);}
 void release(bool pedal){
  down=false;if(pedal){sustained=true;return;}sustained=false;amp.noteOff();
  // Progressive felt contact: stronger initial motion settles over a few ms.
  damperRamp=1-std::exp(-dt/(.002f+.003f*velocity));
  // Actual damper changes modal losses, high modes settle first. Ramp contact to
  // avoid a discontinuity; ADSR Release remains the user's maximum release gate.
  // The circuit envelope already damps the fundamental. Do not multiply it
  // by another full60dB over the same time and prematurely chop the tail.
  for(int n=0;n<count;++n){float seconds=std::max(.024f,settings.release*.001f*2.8f)/(1+.10f*std::sqrt(modes[n].ratio));modes[n].damperTarget=std::exp(-6.9077553f/(seconds*sr));}
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
  for(int n=0;n<count;++n)x+=modes[n].tick(force,damperRamp);x=wurliPickup.tick(x,frequency)*.72f;
  // Pickup circuit/cabinet low-pass and dc blocking, not fixed synthetic dirt.
  brightness+=(brightnessTarget-brightness)*toneRamp;circuitBlend+=(circuitBlendTarget-circuitBlend)*toneRamp;
  pickupLp+=brightness*(x-pickupLp);lp+=brightness*(pickupLp-lp);x=pickupLp+(lp-pickupLp)*circuitBlend;
  reedLow+=reedPole*(x-reedLow);x-=reedLow;
  hammerNoise+=.12f*(force*.0005f-hammerNoise);
  if(hammer.active)x+=settings.noise*hammerNoise*noise()*.20f;
  if(keyNoiseLeft>0){x+=keyNoise*noise()*.002f*keyNoiseLeft/(sr*.012f);--keyNoiseLeft;}
  float env=amp.process();x*=env*keyBalance*wurliVoicing.level;bodySend=0;
  ageSeconds+=dt;energy+=.002f*(std::abs(x)-energy);
  if(!amp.isActive()||(!down&&!sustained&&energy<1e-7f&&ageSeconds>.15f)||ageSeconds>45)stop();
  return x;
 }
};
}
