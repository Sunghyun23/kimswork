// Independent lo-fi performance processing. All storage allocated only in prepare().
#pragma once
#include <vector>
#include <array>
#include <cmath>
#include <algorithm>
#include <cstdint>
#include "dsp/smoother.h"
#include "dsp/math.h"
namespace crate {
struct StereoHistory {
 std::array<std::vector<float>,2> data;int head=0,filled=0,size=1;
 void prepare(double sr){size=std::max(16,int(sr*8));for(auto&d:data)d.assign(size,0);head=filled=0;}
 void reset(){head=filled=0;}
 void push(const std::array<float,2>&x){for(int c=0;c<2;++c)data[c][head]=x[c];if(++head==size)head=0;filled=std::min(size,filled+1);}
 float at(int c,double position)const{while(position<0)position+=size;while(position>=size)position-=size;int i=int(position),j=(i+1)%size;float f=float(position-i);return data[c][i]+(data[c][j]-data[c][i])*f;}
 float delay(int c,float samples)const{if(samples>=filled-1)return 0;return at(c,double(head)-1-std::max(0.f,samples));}
};
struct PerformanceSettings {
 float tapeMs=700;int reverse=1,repeat=1,echoDivision=2;
 float feedback=.55f,echoMix=.35f,filterHz=900,amount=1;
 float bits=8,crushRate=.6f,wobbleRate=2,wobbleDepth=.55f;
 float dropoutRate=8,dropoutDepth=1,releaseMs=15,output=1;
};
class Performance {
public:
 enum Effect {Tape,Reverse,Repeat,Echo,Filter,Crush,Wobble,Dropout,N};
 void prepare(double sr){sr_=sr;for(auto*h:{&live_,&reverse_,&repeat_,&echo_})h->prepare(sr);
  for(auto&s:mix_)s.setTime(5,sr);for(auto&s:control_)s.setTime(20,sr);delay_.setTime(30,sr);amount_.setTime(15,sr);output_.setTime(15,sr);reset();}
 void reset(){const float defaults[10]={.55f,.35f,.12f,256,.6f,2,.55f,8,1,700};for(int i=0;i<10;++i)control_[i].snap(defaults[i]);for(auto*h:{&live_,&reverse_,&repeat_,&echo_})h->reset();for(auto&s:mix_)s.snap(0);gate_.fill(false);last_.fill(false);progress.fill(0);filter1_.fill(0);filter2_.fill(0);echoLow_.fill(0);held_.fill(0);for(auto&d:wobble_)d.fill(0);wobbleHead_=0;tapeAge_=tapeLag_=reversePhase_=repeatPhase_=wobblePhase_=holdPhase_=dropPhase_=0;reverseStart_=repeatStart_=reverseLength_=repeatLength_=0;dropGain_=1;dropTarget_=1;dropCut_=false;dropDuration_=.3f;tail_=0;random_=0x43525046;delay_.snap(float(sr_*.4));amount_.snap(1);output_.snap(1);}
 void configure(const PerformanceSettings&s,const std::array<bool,N>&g,double bpm){
  settings_=s;gate_=g;bpm=std::isfinite(bpm)?std::max(20.,std::min(300.,bpm)):120.;
  static const double division[4]={1,.5,.25,.125},rev[4]={.5,1,2,4},echoBeats[4]={1,.5,.75,1.5};
  reverseWanted_=int(sr_*60/bpm*rev[std::max(0,std::min(3,s.reverse))]);repeatWanted_=int(sr_*60/bpm*division[std::max(0,std::min(3,s.repeat))]);
  delay_.setTarget(std::min(float(echo_.size-3),float(sr_*60/bpm*echoBeats[std::max(0,std::min(3,s.echoDivision))])));
  amount_.setTarget(s.amount);output_.setTarget(s.output);
  filterCoeff_=1-std::exp(-float(6.28318530718*std::min(double(s.filterHz),sr_*.4)/sr_));
  quant_=float(1u<<std::max(3,std::min(16,int(std::round(s.bits)))));dropSpeed_=1-std::exp(-1.f/float(sr_*.003));
  const float values[10]={s.feedback,s.echoMix,filterCoeff_,quant_,s.crushRate,s.wobbleRate,s.wobbleDepth,s.dropoutRate,s.dropoutDepth,s.tapeMs};for(int i=0;i<10;++i)control_[i].setTarget(values[i]);
  for(int k=0;k<N;++k){mix_[k].setTime(g[k]?5.f:std::max(5.f,s.releaseMs),sr_);mix_[k].setTarget(g[k]?1.f:0.f);}
 }
 std::array<float,2> process(std::array<float,2>x){
  float control[10];for(int i=0;i<10;++i)control[i]=control_[i].process();
  const auto original=x;float blend[N];for(int k=0;k<N;++k)blend[k]=mix_[k].process();
  if(gate_[Tape]&&!last_[Tape])tapeAge_=tapeLag_=0;
  live_.push(x);
  float tapeSpeed=1,tapeFade=1;
  if(blend[Tape]>1e-6f){float duration=std::max(1.f,control[9]*.001f*float(sr_));if(gate_[Tape]){tapeAge_+=1;tapeLag_+=std::min(1.,tapeAge_/duration);}tapeSpeed=1-float(std::min(1.,tapeAge_/duration));float fade=1-float(std::max(0.,std::min(1.,(tapeAge_/duration-.8)/.2)));tapeFade=fade;for(int c=0;c<2;++c)x[c]+=(live_.delay(c,float(tapeLag_))*fade-x[c])*blend[Tape];progress[Tape]=float(std::min(1.,tapeAge_/duration));}
  else progress[Tape]=0;
  if(gate_[Reverse]&&!last_[Reverse]){reverseLength_=std::min(reverseWanted_,reverse_.filled);reverseStart_=reverse_.head-reverseLength_;reversePhase_=0;}
  if(blend[Reverse]<1e-6f)reverse_.push(x);
  if(blend[Reverse]>1e-6f&&reverseLength_>2){for(int c=0;c<2;++c){double pos=reverseLength_-1-reversePhase_;float v=reverse_.at(c,reverseStart_+pos)*edge(reversePhase_,reverseLength_)*(1+blend[Tape]*(tapeFade-1));x[c]+=(v-x[c])*blend[Reverse];}reversePhase_+=1+blend[Tape]*(tapeSpeed-1);if(reversePhase_>=reverseLength_)reversePhase_=0;progress[Reverse]=float(reversePhase_/reverseLength_);}else progress[Reverse]=0;
  if(gate_[Repeat]&&!last_[Repeat]){repeatLength_=std::min(repeatWanted_,repeat_.filled);repeatStart_=repeat_.head-repeatLength_;repeatPhase_=0;}
  if(blend[Repeat]<1e-6f)repeat_.push(x);
  if(blend[Repeat]>1e-6f&&repeatLength_>2){for(int c=0;c<2;++c){float forward=repeat_.at(c,repeatStart_+repeatPhase_),backward=repeat_.at(c,repeatStart_+repeatLength_-1-repeatPhase_);float v=(forward+(backward-forward)*blend[Reverse])*edge(repeatPhase_,repeatLength_)*(1+blend[Tape]*(tapeFade-1));x[c]+=(v-x[c])*blend[Repeat];}repeatPhase_+=1+blend[Tape]*(tapeSpeed-1);if(repeatPhase_>=repeatLength_)repeatPhase_=0;progress[Repeat]=float(repeatPhase_/repeatLength_);if(blend[Reverse]>.5f)progress[Reverse]=progress[Repeat];}else progress[Repeat]=0;
  for(int c=0;c<2;++c){filter1_[c]+=control[2]*(x[c]-filter1_[c]);filter2_[c]+=control[2]*(filter1_[c]-filter2_[c]);x[c]+=(filter2_[c]-x[c])*blend[Filter];}
  holdPhase_+=std::max(.02f,1-control[4]*.97f);bool update=holdPhase_>=1;if(update)holdPhase_-=1;
  for(int c=0;c<2;++c){if(update)held_[c]=std::round(x[c]*control[3])/control[3];x[c]+=(held_[c]-x[c])*blend[Crush];}
  wobblePhase_+=control[5]/sr_;if(wobblePhase_>=1)wobblePhase_-=std::floor(wobblePhase_);
  float delay=float(sr_)*(.006f+mpc::dsp::fastSin01(float(wobblePhase_))*.003f*control[6]);
  for(int c=0;c<2;++c){wobble_[c][wobbleHead_]=x[c];float rp=float(wobbleHead_)-delay;while(rp<0)rp+=8192;int a=int(rp)&8191,b=(a+1)&8191;float f=rp-std::floor(rp);float v=wobble_[c][a]+(wobble_[c][b]-wobble_[c][a])*f;x[c]+=(v-x[c])*blend[Wobble];}wobbleHead_=(wobbleHead_+1)&8191;
  if(gate_[Dropout]&&!last_[Dropout]){dropPhase_=0;dropGain_=1;dropCut_=true;dropDuration_=.3f;}
  dropPhase_+=control[7]/sr_;if(dropPhase_>=dropDuration_){dropPhase_-=dropDuration_;dropCut_=!dropCut_;random_^=random_<<13;random_^=random_>>17;random_^=random_<<5;float jitter=float(random_&65535)/65535;dropDuration_=dropCut_?.18f+jitter*.32f:.55f+jitter*.65f;}
  dropTarget_=dropCut_?1-control[8]:1;
  dropGain_+=dropSpeed_*(dropTarget_-dropGain_);for(auto&v:x)v*=1+blend[Dropout]*(dropGain_-1);
  // Echo is a momentary send. Feedback continues after release and never exceeds80%.
  std::array<float,2>feed{};float echoDelay=delay_.process();
  for(int c=0;c<2;++c){float v=echo_.delay(c,echoDelay);echoLow_[c]+=.24f*(v-echoLow_[c]);feed[c]=mpc::dsp::fastTanh(x[c]*blend[Echo]+echoLow_[c]*control[0]);x[c]+=v*control[1];}
  echo_.push(feed);tail_=std::max(std::max(std::fabs(feed[0]),std::fabs(feed[1])),tail_*.9995f);
  float amount=amount_.process(),out=output_.process(),active=std::min(1.f,tail_*4);for(int k=0;k<N;++k)active=std::max(active,blend[k]);
  for(int c=0;c<2;++c){x[c]=original[c]+(x[c]-original[c])*amount;x[c]*=1+(out-1)*active*amount;}
  for(int k=0;k<N;++k)if(k!=Tape&&k!=Reverse&&k!=Repeat)progress[k]=blend[k];last_=gate_;return x;
 }
 std::array<float,N>progress{};
private:
 float edge(double phase,int length)const{double fade=std::min(sr_*.002,length*.1);return float(std::max(0.,std::min(1.,std::min(phase/fade,(length-1-phase)/fade))));}
 double sr_=44100,tapeAge_=0,tapeLag_=0,reversePhase_=0,repeatPhase_=0,wobblePhase_=0,dropPhase_=0;
 StereoHistory live_,reverse_,repeat_,echo_;std::array<mpc::dsp::Smoother,N>mix_;std::array<mpc::dsp::Smoother,10>control_;mpc::dsp::Smoother delay_,amount_,output_;
 PerformanceSettings settings_;std::array<bool,N>gate_{},last_{};
 std::array<float,2>filter1_{},filter2_{},echoLow_{},held_{};std::array<std::array<float,8192>,2>wobble_{};int wobbleHead_=0,reverseWanted_=1,repeatWanted_=1,reverseStart_=0,repeatStart_=0,reverseLength_=0,repeatLength_=0;
 float filterCoeff_=.1f,quant_=256,holdPhase_=0,dropSpeed_=.01f,dropGain_=1,dropTarget_=1,tail_=0,dropDuration_=.3f;bool dropCut_=false;uint32_t random_=0x43525046;
};
}
