#pragma once
#include "dsp/adsr.h"
#include "UprightCore.h"
#include "ReciprocalContact.h"
#include "UprightBassCalibration.h"
#include "UprightOctaveCalibration.h"
#include "UprightAction.h"
#include "UprightRadiationRise.h"
#include "UprightVoicing.h"
#include <array>
#include <cstdint>
namespace felt {
struct UprightVoice : VoiceState {
 static constexpr int MaxModes=96;
 std::array<upright::Mode,MaxModes> modes{};
 mpc::dsp::ADSR amp;upright::Hammer hammer;
 upright::Action action;
 upright::ReciprocalContact<MaxModes> hammerContact;
 float contactPitchRatio=1.f;
 upright::RadiationRise radiationRise;
 VoiceSettings settings{};
 float dcLow=0,dcPole=.01f,toneLow=0,dt=1.f/44100,damperRamp=.007f;
 float boardLow=0,boardPole=0,boardAmount=0;
 float releaseProgress=0,releaseStep=0,releaseLevel=0;
 bool releasing=false;
 int damperContactSamples=0;
 std::array<float,MaxModes> settledDamper{};
 float keyNoise=0;int keyNoiseLeft=0;
 float brightnessTarget=.1f,circuitBlend=.5f,circuitBlendTarget=.5f,toneRamp=.004f;
 float sr=44100,lp=0,brightness=.1f;
 float modalMass=.02f,hammerNoise=0,keyBalance=1;
 int count=0,strings=1;
 std::array<float,MaxModes> unisonMix{};
 std::array<float,MaxModes> attackRadiation{};
 float attackEdge=0,attackEdgePole=0;
 std::array<float,MaxModes> bridgeDifferenceWeights{};
 // Retain the relative motion of upper unison strings as a separate
 // bridge drive. Their forces can cancel in the mean but excite different
 // nearby parts of the panel.
 float soundboardProfile=0,bridgeDetail=0,detailTone=0,detailLow=0;

 uint32_t random=0x46454c54;
 float noise(){random^=random<<13;random^=random>>17;random^=random<<5;return float(random>>8)*(2.f/16777216.f)-1.f;}
 void prepare(float rate){sr=rate;dt=1/rate;damperRamp=1-std::exp(-dt/.003f);toneRamp=1-std::exp(-dt/.005f);amp.setSampleRate(rate);action.prepare(rate);boardPole=1-std::exp(-2*Pi*420.f/rate);}
 // One tone law for new and already sounding voices. A variable second pole
 // preserves dark settings without imposing a permanently closed cabinet.
 void setTone(float value,bool immediate=false){
  settings.tone=clamp(value,0,1);float top=18000.f;
  float cutoff=std::min(sr*.38f,1100.f+(top-1100.f)*settings.tone);
  brightnessTarget=1-std::exp(-2*Pi*cutoff/sr);circuitBlendTarget=.85f-.70f*settings.tone;
  if(immediate){brightness=brightnessTarget;circuitBlend=circuitBlendTarget;}
 }
 void stop(){soundboardProfile=bridgeDetail=detailTone=detailLow=0;amp.reset();active=down=sustained=false;owner=-1;lp=dcLow=toneLow=boardLow=energy=bodySend=hammerNoise=keyNoise=0;releasing=false;damperContactSamples=0;attackEdge=0;keyNoiseLeft=0;hammer.active=false;action.reset();radiationRise.reset();for(auto&m:modes)m.y1=m.y2=0;}
 void configureModes(bool /*excite*/){
  strings=note<40?1:note<55?2:3;
  count=MaxModes;
  const float key=clamp((note-21)/87.f,0,1),keyVariation=1+.012f*std::sin(note*1.731f+2*.37f);
  modalMass=.006f*std::exp2((60-note)/48.f)*keyVariation;
  // Smooth register laws calibrated against isolated upright recordings.
  const float dispersion=.000217f*std::exp2((note-60)/23.f+1.12f*std::max(0,note-60)/12.f);
  const float stiffness=std::min(.020f,dispersion*(.65f+settings.character3*1.4f));
  const float upperLoss=1.65f*clamp((note-42)/30.f,0,1.4f);
  const float registerDecay=std::exp2(-.35f*(std::max(0,44-note)+std::max(0,note-70))/12.f);
  // Measured treble tails shed their upper harmonics faster than the
  // original decay law. Add a loss rate, leaving the fundamental untouched.
  const float trebleLoss=.10f*clamp((note-68)/16.f,0,1);
  const float bassWeight=upright::bassRegisterWeight(note);
  const auto bassLoss=upright::bassUnisonLoss(note);
  const float position=.115f+.024f*settings.hammer+.002f*std::sin(note*.63f)+.010f*bassWeight;
  const float contactWidth=.005f+.009f*(1-settings.hammer);
  const float stringNorm=1.f/std::sqrt(float(strings));
  // Separate bridge contacts give the second partial unequal radiation
  // weights in the middle register. Preserve the mean aperture, so equal
  // string motion keeps its gain while differential motion still radiates.
  const float secondPartialAperture=.65f*clamp((note-46)/8.f,0,1)*
                                    clamp((75-note)/9.f,0,1);
  const int calibrationKey=upright::octaveIndex(note);
  const auto* calibration=calibrationKey>=0?&upright::octaveCalibration[calibrationKey]:nullptr;
  for(int n=0;n<count;++n){
   auto&m=modes[n];float ratio=1,t60=1,input=1,output=1,initial=0;

    int h=n/strings+1,unison=n%strings;
    ratio=h*std::sqrt((1+stiffness*h*h)/(1+stiffness));
    const float partialCenterHz=frequency*ratio;
    float spread=(unison==1?1.f:unison==2?-.82f:0.f)*(settings.detune+.35f+.55f*key);
    ratio*=std::exp2(spread/1200);
    // Finite contact distributes the drive and averages the felt's view
    // of string motion with the same weight.
    const float footprint=Pi*h*contactWidth;
    input=std::sin(Pi*h*position)*stringNorm*std::sin(footprint)/footprint;
    // For a stiff-string modal coordinate q, transverse bridge force is
    // T * (-1)^h * h*pi/L * q. With modal mass mu*L/2 and
    // f0=sqrt(T/mu)/(2L), T*h*pi/L = 8*pi*m*f0^2*h.
    // String normalization matches the hammer's normalized contact coordinate.
    output=(h%2?1.f:-1.f)*8*Pi*modalMass*frequency*frequency*h*stringNorm;
    // Reduced radiation weighting. Treble partials radiate less strongly
    // relative to the fundamental than the ideal endpoint force predicts.
    output*=std::pow(ratio,-upperLoss);
    const auto* unisonProfile=calibration&&h<=int(upright::CalibratedUnisonPartials)
                             ?&upright::unisonCalibration[calibrationKey][h-1]:nullptr;
    const float aperture=unisonProfile?unisonProfile->aperture:h==2?secondPartialAperture:0;
    if(aperture!=0){
     const float offset=unison==0?float(strings-1):-1.f;
     output*=1.f+offset*aperture;
    }
    if(h==1)output/=1.f+3.f*clamp((44-note)/11.f,0,1);
    if(bassWeight>0)output*=upright::bassPressureGain(note,h,partialCenterHz,bassWeight);
    t60=(4.4f+settings.body*8.2f)*std::exp2((60-note)/34.f)/(1+.045f*h+.007f*h*h);
    t60*=registerDecay*(.90f+.45f*settings.resonance);
    if(trebleLoss>0&&h>1)t60=1.f/(1.f/t60+trebleLoss*(h-1));
    if(calibration&&h<=int(upright::OctaveCalibration::Partials)){
     output*=calibration->pressureGain[h-1];
     const float inverseT60=1.f/t60;
     float calibratedInverse=std::max(.68f*inverseT60,inverseT60+calibration->inverseT60Offset[h-1]);
     if(unisonProfile){
      const float splitLoss=std::max(0.f,unison==0?-unisonProfile->flankLossSplit:unisonProfile->flankLossSplit);
      const float minimumInverse=std::max(.025f,.45f*inverseT60);
      calibratedInverse=std::max(minimumInverse,calibratedInverse+unisonProfile->inverseT60Offset+splitLoss);
     }
     t60=1.f/calibratedInverse;
    }
    const float commonLoss=h<=int(bassLoss.size())?bassLoss[h-1]:1.f;
    unisonMix[n]=1-std::exp(-dt*(3.6f-2.f*settings.resonance)/(1+.06f*h)*commonLoss);

   m.pitchable=true;m.ratio=ratio;m.input=input;m.output=output;m.amplitude=output;
   const float partialHz=frequency*ratio;
   // The prompt radiation emphasizes the highest strike partials. Taper the
   // band edges to avoid a switch when tuning moves a partial across them.
   attackRadiation[n]=clamp((partialHz-4550.f)/1500.f,0,1)*
                      clamp((11200.f-partialHz)/2500.f,0,1);
   const float shoulder=clamp((partialCenterHz-1740.f)/670.f,0,1)*
                        clamp((5630.f-partialCenterHz)/1220.f,0,1);
   bridgeDifferenceWeights[n]=strings>1&&h>6
                             ?soundboardProfile*shoulder*(unison==0?strings-1:-1):0;
   m.configure(partialHz,t60,sr,false,initial);
   m.drive=m.muted?0:input*dt*dt/modalMass;
  }
  while(count>1&&modes[count-1].drive==0&&modes[count-1].output==0)--count;
  contactPitchRatio=1.f;
  hammerContact.prepare(modes.data(),count,sr,modalMass,hammer.mass);
 }
 
 void strike(int pitch,int velocity127,int source,int serial,const VoiceSettings&s,float tuning=440){
  stop();this->tuning=tuning;settings=s;settings.model=2;note=pitch;owner=source;age=serial;velocity=clamp(velocity127/127.f,.01f,1);frequency=initialFrequency=targetFrequency=bendHz(note,tuning);ageSeconds=0;
  pan=clamp((pitch-60)/42.f,-.65f,.65f);random=0x46454c54u+uint32_t(serial*1664525u+pitch*1013904223u);
  dcPole=1-std::exp(-2*Pi*25.f/sr);setTone(settings.tone,true);
  boardAmount=.6f*(settings.character4-.20f);
  soundboardProfile=clamp((note-47)/6.f,0,1)*clamp((70-note)/6.f,0,1);
  const int calibrationKey=upright::octaveIndex(note);
  const auto* touchProfile=calibrationKey>=0?&upright::touchCalibration[calibrationKey]:nullptr;
  float launchExponent=.85f+.45f*settings.velocity;
  if(touchProfile){
   // Fade the light-touch correction into the original launch response.
   // A nonnegative offset that decreases with velocity preserves monotonic speed.
   const float light=clamp((.68f-velocity)/.35f,0,1);
   launchExponent+=touchProfile->launchExponentOffset*light*light*(3-2*light);
  }
  const float speed=.35f+4.2f*std::pow(velocity,launchExponent);
  float mass=.0018f*std::exp2((60-note)/28.f);
  float hardness=clamp(settings.hammer*1.f+.10f*velocity,.04f,1);
  hammer.strike(speed,mass,hardness);hammer.stiffness*=clamp(std::exp2((note-60)/24.f),.3f,4.f);hammer.stiffness*=2.f;
  // A light strike compresses the felt less. Keep the existing stiffness
  // once the felt reaches the medium/strong part of the touch curve.
  hammer.stiffness*=std::exp2(1.15f*(std::min(velocity,.62f)-.62f));
  if(touchProfile){
   // Adjust the stiffness of the felt contact for the measured touch ranges.
   const float lightContact=clamp((.57f-velocity)/.34f,0,1);
   const float firmContact=clamp((velocity-.44f)/.45f,0,1);
   const float logStiffness=-1.1f*touchProfile->lightFeltWeight*lightContact
                            +1.35f*touchProfile->strongFeltWeight*firmContact;
   hammer.stiffness*=std::exp2(logStiffness);
  }
  // Bass felt has a more compliant contact across the register break.
  const float bassFelt=clamp((note-35)/6.f,0,1)*clamp((56-note)/5.f,0,1);
  hammer.stiffness*=1.f-.30f*bassFelt;
  action.strike(note,settings.character4,mass,speed);
  radiationRise.strike(note,frequency,dt);
  const float touch=clamp((velocity-.24f)/.50f,0,1);
  attackEdge=1.25f*touch;
  attackEdgePole=std::exp(-dt/(10.0f*.001f));
  // Unilateral, velocity-dependent dissipative felt contact.
  hammer.loss=.035f+.055f*settings.hammer;
  // Register balance belongs to radiation gain, not an f0-normalized pickup.
  keyBalance=.075f*clamp(std::exp2((60-note)*.40f/12),.25f,3.f);
  amp.set(settings.attack,settings.decay,settings.sustain,settings.release);amp.noteOn();down=active=true;configureModes(true);
  
 }
 void keyOff(float amount){keyNoise=amount;keyNoiseLeft=int(sr*.012f);}
 void release(bool pedal){
  down=false;
  if(releasing||!active)return;
  if(pedal){sustained=true;return;}
  const bool pedalClosure=sustained;
  sustained=false;
  releasing=true;
  releaseLevel=amp.level();
  releaseProgress=0;
  releaseStep=1/std::max(1.f,settings.release*.001f*sr);
  // A felt damper absorbs the higher partials more strongly in this
  // register. The closure after sustain also adds fundamental loss.
  const int calibrationKey=upright::octaveIndex(note);
  const float depth=calibrationKey>=0?upright::touchCalibration[calibrationKey].damperDepth:0.f;
  const float scale=depth*.72f/std::max(.08f,settings.release*.001f);
  const float upper=pedalClosure?1.8f:.8f;
  const float fundamentalLoss=pedalClosure?.65f:0.f;
  // Capture the string quickly as the felt returns after pedal-up. A
  // quieter residual vibration survives once the initial contact settles.
  float contactWeight=0;
  if(pedalClosure){
   const float middle=clamp((note-49)/6.f,0,1)*clamp((76-note)/10.f,0,1);
   const float bass=.30f*clamp((note-25)/13.f,0,1)*clamp((55-note)/5.f,0,1);
   contactWeight=std::max(middle,bass);
   if(contactWeight>0)damperContactSamples=std::max(1,int(sr*.032f));
  }
  for(int n=0;n<count;++n){
   auto& mode=modes[n];
   const float seconds=std::max(.018f,settings.release*.001f*3.f)/(1+.045f*mode.ratio);
   const int harmonic=n/strings+1;
   const float contact=(harmonic-1)/(harmonic-.68f);
   const float extraLoss=scale*(fundamentalLoss+upper*contact);
   mode.damperTarget=extraLoss>0.f
                     ?std::exp(-6.9077553f*(1.f/seconds+extraLoss)/sr)
                     :std::exp(-6.9077553f/(seconds*sr));
   if(contactWeight>0){
    const float normalLoss=1.f/seconds+extraLoss;
    const float landingLoss=normalLoss*(1.f+contactWeight*1.15f*(1.f+.15f*contact));
    const float residualLoss=normalLoss*(1.f-contactWeight*.78f);
    mode.damperTarget=std::exp(-6.9077553f*landingLoss/sr);
    settledDamper[n]=std::exp(-6.9077553f*residualLoss/sr);
   }
  }
 }
 void setEnvelope(float a,float d,float s,float r){settings.attack=a;settings.decay=d;settings.sustain=s;settings.release=r;amp.set(a,d,s,r);if(releasing)releaseStep=1/std::max(1.f,r*.001f*sr);}
 void retune(int pitch,float milliseconds,float tuning=440){note=pitch;targetFrequency=bendHz(pitch,tuning);glide=1-std::exp(-1/(std::max(1.f,milliseconds)*.001f*sr));}
 void updatePitch(float ratio){
  if(glide>0)frequency+=(targetFrequency-frequency)*std::min(1.f,glide*32);else frequency=targetFrequency;
  float relative=std::max(.1f,std::min(8.f,frequency/initialFrequency*ratio));for(int n=0;n<count;++n)modes[n].pitch(relative);
  if(hammer.active&&relative!=contactPitchRatio){
   contactPitchRatio=relative;
   hammerContact.prepare(modes.data(),count,sr,modalMass,hammer.mass);
  }
 }
 
 // Passive rank-one velocity loss at the shared bridge. In normalized
 // quadrature, velocity = omega*p. Contract its common component only, leaving
 // differential unison motion to decay more slowly (prompt + aftersound).
 // This is a reduced dissipative coupling, not a measured bridge admittance.
 void coupleUnisons(){
  if(strings<2)return;
  for(int first=0;first<count;first+=strings){
   double mean=0;int live=0;
   for(int j=0;j<strings&&first+j<count;++j){auto&m=modes[first+j];if(m.muted||m.phaseSin<1e-8)continue;double r=m.radius*m.damping;mean+=(m.phaseCos*m.y1-r*m.y2)*m.bridgeVelocity;++live;}
   if(!live)continue;mean*=unisonMix[first]/live;
   for(int j=0;j<strings&&first+j<count;++j){auto&m=modes[first+j];if(m.muted||m.phaseSin<1e-8)continue;// Algebraically eliminate reconstructing and then cancelling the
    // same quadrature state. Held modes use only cached multiplications.
    double correction=mean*m.bridgeReturn;
    m.y2+=m.damping==1?correction:correction/m.damping;}
  }
 }
 // After hammer contact/attack build-up, process one unison group at a
 // time. This preserves modal order while keeping its hot state in cache.
 template<int S>void freeGroup(int first,float& x,float& difference){
  double mean=0;int live=0;
  if constexpr(S>1){
   for(int j=0;j<S;++j){auto&m=modes[first+j];if(m.muted||m.phaseSin<1e-8)continue;double r=m.radius*m.damping;mean+=(m.phaseCos*m.y1-r*m.y2)*m.bridgeVelocity;++live;}
   if(live)mean*=unisonMix[first]/live;
  }
  for(int j=0;j<S;++j){int index=first+j;auto&m=modes[index];
   if constexpr(S>1)if(!m.muted&&m.phaseSin>=1e-8&&live){double correction=mean*m.bridgeReturn;m.y2+=m.damping==1?correction:correction/m.damping;}
   float sample=m.tick(0,damperRamp);difference+=bridgeDifferenceWeights[index]*sample;x+=sample;
  }
 }
 template<int S>void freeModes(float&x,float&difference){
  int first=0;for(;first+S<=count;first+=S)freeGroup<S>(first,x,difference);
  if(first==count)return;
  // A Nyquist-trimmed final group can contain fewer than S strings.
  double mean=0;int live=0;
  for(int n=first;n<count;++n){auto&m=modes[n];if(m.muted||m.phaseSin<1e-8)continue;mean+=(m.phaseCos*m.y1-m.radius*m.damping*m.y2)*m.bridgeVelocity;++live;}
  if(live)mean*=unisonMix[first]/live;
  for(int n=first;n<count;++n){auto&m=modes[n];if(!m.muted&&m.phaseSin>=1e-8&&live){double correction=mean*m.bridgeReturn;m.y2+=m.damping==1?correction:correction/m.damping;}float sample=m.tick(0,damperRamp);difference+=bridgeDifferenceWeights[n]*sample;x+=sample;}
 }
 float tick(){
  if(!active)return 0;
  if(damperContactSamples>0&&--damperContactSamples==0)
   for(int n=0;n<count;++n)modes[n].damperTarget=settledDamper[n];
  const bool fastFree=!hammer.active&&!radiationRise.active&&attackEdge==0;
  if(!fastFree)coupleUnisons();
  const bool advancedByContact=hammer.active;
  const float force=advancedByContact?hammerContact.tick(hammer,modes.data(),count,dt,damperRamp):0.f;
  const float actionPressure=action.tick(force,dt);
  float x=0,difference=0;
  const float edge=attackEdge;
  if(edge>1e-5f)attackEdge*=attackEdgePole;
  else attackEdge=0;
  const auto radiatedMode=[&](int index){
   auto& mode=modes[index];
   const float sample=advancedByContact?(mode.muted?0.f:float(mode.y1)*mode.output)
                                       :mode.tick(0,damperRamp);
   difference+=bridgeDifferenceWeights[index]*sample;
   return edge>0?sample*(1+edge*attackRadiation[index]):sample;
  };
  if(fastFree){if(strings==1)freeModes<1>(x,difference);else if(strings==2)freeModes<2>(x,difference);else freeModes<3>(x,difference);}
  else{
  int n=0;
  if(radiationRise.active){
   for(std::size_t h=0;h<upright::RadiationRise::periods.size()&&n<count;++h){
    const int end=std::min(n+strings,count);
    float group=0;
    for(;n<end;++n)group+=radiatedMode(n);
    x+=group*radiationRise.advance(h);
   }
   radiationRise.settle();
  }
  for(;n<count;++n)x+=radiatedMode(n);
  }
  // User Tone remains a smoothed spectral control on the bridge-force signal.
  brightness+=(brightnessTarget-brightness)*toneRamp;circuitBlend+=(circuitBlendTarget-circuitBlend)*toneRamp;
  toneLow+=brightness*(x-toneLow);lp+=brightness*(toneLow-lp);x=toneLow+(lp-toneLow)*circuitBlend;
  dcLow+=dcPole*(x-dcLow);x-=dcLow;
  hammerNoise+=.12f*(force*.0005f-hammerNoise);
  if(hammer.active)x+=settings.noise*hammerNoise*noise()*.20f;
  if(keyNoiseLeft>0){x+=keyNoise*noise()*.002f*keyNoiseLeft/(sr*.012f);--keyNoiseLeft;}
  float env;
  if(releasing){
   releaseProgress=std::min(1.f,releaseProgress+releaseStep);
   const float remaining=1-releaseProgress;
   env=releaseLevel*remaining*remaining;
  }else env=amp.process();
  x*=env*keyBalance*uprightVoicing.level;
  boardLow+=boardPole*(x-boardLow);
  bodySend=x+boardAmount*boardLow+actionPressure*env;
  if(soundboardProfile>0){
   detailTone+=brightness*(difference-detailTone);
   detailLow+=brightness*(detailTone-detailLow);
   bridgeDetail=(detailTone+(detailLow-detailTone)*circuitBlend)*env*keyBalance*uprightVoicing.level;
  }
  // Small direct leakage only; main acoustic output comes from UprightBody.
  // Other backends retain their existing host mixer contracts.
  const float leakage=.025f;
  ageSeconds+=dt;energy+=.002f*(std::abs(x)-energy);
  if((releasing?releaseProgress>=1:!amp.isActive())||(!down&&!sustained&&energy<1e-7f&&ageSeconds>.15f)||ageSeconds>45)stop();
  return x*leakage;
 }
};
}
