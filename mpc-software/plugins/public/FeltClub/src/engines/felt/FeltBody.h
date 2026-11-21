#pragma once
#include "FeltCore.h"
#include "dsp/biquad.h"
#include <array>
#include <algorithm>
namespace felt {
// Original reduced force-to-radiation model. Low modes use an effective
// orthotropic plate, upper modes inter-rib strips. Geometry/losses are voicing
// assumptions, not measurements of a named piano or commercial impulse response.
struct FeltBody {
 struct RadiationMode {
  double a=0,b=0,z1=0,z2=0;float gain=0,drive=0;
  void prepare(float hz,float bandwidth,float rate,float weight){
   double r=std::exp(-Pi*bandwidth/rate);a=2*r*std::cos(2*Pi*hz/rate);b=r*r;
   drive=float((1-b)*.5);gain=weight;z1=z2=0;
   if(hz>rate*.40f)gain=drive=0;
  }
  float process(float difference){double y=a*z1-b*z2+drive*difference;z2=z1;z1=std::abs(y)<1e-24?0:y;return float(z1)*gain;}
  void reset(){z1=z2=0;}
 };
 // A short difference of two acoustic paths at the rim. Return opposite
 // signed pressure to the two channels so the main mono radiation is retained.
 struct RimRadiation {
  static constexpr unsigned BufferSize=256;
  std::array<float,BufferSize> history{};
  unsigned cursor=0;
  float nearDelay=0,farDelay=0;

  void prepare(float rate,float nearSeconds=.00030f,float farSeconds=.00148f) {
   nearDelay=clamp(nearSeconds*rate,1.f,float(BufferSize-2));
   farDelay=clamp(farSeconds*rate,1.f,float(BufferSize-2));
   reset();
  }
  void reset() {history.fill(0);cursor=0;}
  float tap(float delay) const {
   const auto whole=static_cast<unsigned>(delay);
   const float fraction=delay-float(whole);
   const float a=history[(cursor-whole)&(BufferSize-1)];
   const float b=history[(cursor-whole-1)&(BufferSize-1)];
   return a+(b-a)*fraction;
  }
  std::array<float,2> paths(float pressure) {
   history[cursor]=pressure;
   const std::array<float,2> result{tap(nearDelay),tap(farDelay)};
   cursor=(cursor+1)&(BufferSize-1);
   return result;
  }
  float tick(float pressure) {
   const auto pathsOut=paths(pressure);
   return .19f*(pathsOut[0]-pathsOut[1]);
  }
 };
 RimRadiation rim,detailRadiation;
 mpc::dsp::StereoBiquad voicingEq;
 std::array<RadiationMode,64> board{};
 std::array<feltpiano::Mode,48> sympathy{};
 // A silent, never-excited soundboard has zero acoustic state. Advance its
 // shared damper recurrence once, without rendering zero resonators.
 bool sleeping=true;float sleepingDamper=1;
 float sympatheticTuning=440;
 float pedal=0,sr=44100,pedalRamp=.002f,damperRamp=.007f,closedDamper=.995f;
 float previous=0,previous2=0,low=0,high=0,lowPole=0,highPole=0;
 void prepare(float rate){
  rim.prepare(rate);
  detailRadiation.prepare(rate,.00062f,.00141f);
  voicingEq.setCoeffs(mpc::dsp::BiquadCoeffs::peak(rate,330.f,.76f,-1.8f));
  sr=rate;pedalRamp=1-std::exp(-1/(.01133f*rate));damperRamp=1-std::exp(-1/(.003f*rate));closedDamper=std::exp(-6.9077553f/(.032f*rate));
  lowPole=1-std::exp(-2*Pi*42/rate);highPole=1-std::exp(-2*Pi*8500/rate);
  struct Candidate {float hz,weight;};std::array<Candidate,64> candidates{};int at=0;
  // Effective clamped plate: w^2=(Dx*kx^4+2Dxy*kx^2ky^2+Dy*ky^4)/rhoA.
  // Positive stiffnesses ensure real modes. Ribs/case constrain the lowest mode.
  constexpr float dx=607.f,dy=40.f,dxy=39.f,rhoA=3.87f;
  for(int m=1;m<=8;++m)for(int n=1;n<=8;++n){float kx=Pi*(m+.5f)/1.15f,ky=Pi*(n+.5f)/.85f;
   float f=std::sqrt((dx*kx*kx*kx*kx+2*dxy*kx*kx*ky*ky+dy*ky*ky*ky*ky)/rhoA)/(2*Pi);
   float shape=std::sin(Pi*m*.43f)*std::sin(Pi*n*.57f);
   candidates[at++]={std::sqrt(42*42+f*f),.20f+.8f*shape*shape};
  }
  std::sort(candidates.begin(),candidates.end(),[](const auto&a,const auto&b){return a.hz<b.hz;});
  for(int i=0;i<24;++i){auto c=candidates[i];board[i].prepare(c.hz,22+c.hz*.055f,rate,c.weight*.18f);}
  // Above the plate regime, three nearby inter-rib strip families. This keeps
  // the main radiation path present in the upper register without a dry bypass.
  for(int i=0;i<40;++i){int strip=i%3,n=i/3+1;float width=.118f+.012f*strip;
   float cross=1100*std::pow(.13f/width,2);float length=45*n*n;
   float hz=std::sqrt(cross*cross+length*length);
   board[24+i].prepare(hz,45+hz*.09f,rate,.14f/(1+.035f*n));
  }
  sympatheticTuning=440;
  // One string-register resonator per semitone, A1 through G#5.
  for(size_t i=0;i<sympathy.size();++i){
   auto& mode=sympathy[i];
   mode.configure(55.f*std::exp2(float(i)/12.f),1.2f,rate,false,0);
   mode.output=1;
   normalizeSympatheticDrive(mode);
  }
  reset();
 }
 static void normalizeSympatheticDrive(feltpiano::Mode& mode){
  // Normalize the pole-pair response at its angular frequency. This keeps
  // the steady sympathetic gain consistent when the sample rate changes.
  constexpr double resonantGain=.188;
  const double loss=1-mode.radius;
  const double sine=mode.phaseSin;
  mode.drive=float(resonantGain*loss*std::sqrt(loss*loss+4*mode.radius*sine*sine));
 }
 void setTuning(float referenceHz){
  if(sympatheticTuning==referenceHz)return;
  sympatheticTuning=referenceHz;
  for(auto& mode:sympathy){
   mode.pitch(referenceHz/440.f);
   normalizeSympatheticDrive(mode);
  }
 }
 void resetSympathetic(){
  for(auto& mode:sympathy){
   mode.y1=mode.y2=0;
   mode.damping=mode.damperTarget=closedDamper;
  }
 }
 void reset(){
  detailRadiation.reset();rim.reset();voicingEq.reset();pedal=previous=previous2=low=high=0;
  for(auto& mode:board)mode.reset();
  resetSympathetic();sleeping=true;sleepingDamper=closedDamper;
 }
 std::array<float,2> process(float input,bool open,float profiledForce=0,float detail=0){
  sleeping=false;
  pedal+=pedalRamp*((open?1.f:0)-pedal);
  // The calibrated register transfers more force to the resolved plate
  // modes and less to the unresolved broadband component.
  const float plateDrive=input+.30f*profiledForce;
  const float floorDrive=input-.20f*profiledForce;
  float difference=plateDrive-previous2;previous2=previous;previous=plateDrive;
  float l=0,r=0;
  for(int i=0;i<64;++i){float v=board[i].process(difference);float pan=(i%3-1)*.12f;l+=v*(.5f-pan);r+=v*(.5f+pan);}
  // Smooth unresolved high-density mobility floor within the body model, not
  // the old full-level string/pickup output. No delay/reverb is used here.
  low+=lowPole*(floorDrive-low);high+=highPole*(floorDrive-low-high);
  l+=high*.30f;r+=high*.30f;
  if(open||pedal>=1e-6f){
   const float sympatheticInput=input*pedal;
   const float target=open?1.f:closedDamper;
   for(auto& mode:sympathy){
    mode.damperTarget=target;
    const float v=mode.tick(sympatheticInput,damperRamp)*.10f;
    l+=v;r+=v;
   }
  }else if(pedal!=0){
   pedal=0;
   resetSympathetic();
  }
  const float rimSide=rim.tick(.5f*(l+r));
  // Nearby panel paths receive higher differential pressure at slightly
  // different times; a small prompt component retains the local attack.
  const auto paths=detailRadiation.paths(detail);
  const float detailLeft=.36f*(.2f*detail+.8f*paths[0]);
  const float detailRight=.36f*(.2f*detail+.8f*paths[1]);
  return {l+rimSide+detailLeft,r-rimSide+detailRight};
 }
 std::array<float,2> render(float input,bool open,float profiledForce,float detail,
                            const std::array<float,2>& direct){
  if(sleeping){
   if(input==0&&profiledForce==0&&detail==0&&direct[0]==0&&direct[1]==0){
    pedal+=pedalRamp*((open?1.f:0)-pedal);
    if(open||pedal>=1e-6f){const float target=open?1.f:closedDamper;sleepingDamper+=(target-sleepingDamper)*damperRamp;}
    else if(pedal!=0){pedal=0;sleepingDamper=closedDamper;}
    return {};
   }
   // Match the control history of a continuously rendered silent body.
   for(auto&mode:sympathy)mode.damping=sleepingDamper;
   sleeping=false;
  }

  auto pressure=process(input,open,profiledForce,detail);
  pressure[0]+=direct[0];
  pressure[1]+=direct[1];
  // Shape the whole Felt sound before mixing other engines and effects.
  voicingEq.process(pressure[0],pressure[1]);
  return pressure;
 }
};
}
