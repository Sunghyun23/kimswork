// Crate Press — independent character bus processor, not a model of proprietary code.
// Stereo input -> character compression -> LR4 three-band dynamics/duck -> colour
// -> wow/flutter -> tone cuts -> phase/latency-aligned dry/wet -> output/peak guard.
// All memory is fixed-size. No allocation, locking, logging, I/O or host calls in process.
#include "mpc_plugin.h"
#include "dsp/biquad.h"
#include "dsp/smoother.h"
#include "dsp/math.h"
#include "Performance.h"
#include <array>
#include <atomic>
#include <cmath>
#include <algorithm>
using namespace mpc::dsp;

namespace crate {
// Shared by DSP and the transfer display: soft-knee reduction before makeup.
inline float reduction(float inputDb,float amount,float offset=0){
 if(amount<.00001f)return 0;
 float over=inputDb-(-8.f-30.f*amount+offset),slope=1.f-1.f/(1.f+7.f*amount*amount);
 return clamp(over<=-3?0:over<3?slope*(over+3)*(over+3)/12.f:slope*over,0.f,48.f);
}
// Audio-owned fixed history. Each bin retains signed extrema from BOTH stereo channels.
// Host receives snapshots at30Hz; the skin never samples an instantaneous audio value.
struct Scope {
 static constexpr int bins=64;
 std::array<std::array<float,bins>,4> history{};
 std::array<float,4> extrema{};int head=0,samples=0,binSamples=529,publishSamples=0,publishPeriod=1470;
 void prepare(double sr){binSamples=std::max(1,int(sr*.012));publishPeriod=std::max(1,int(sr/30));reset();}
 void reset(){for(auto&v:history)v.fill(0);extrema.fill(0);head=samples=publishSamples=0;}
 bool push(const float* input,const float* output){
  for(int ch=0;ch<2;++ch){extrema[0]=std::max(extrema[0],input[ch]);extrema[1]=std::max(extrema[1],-input[ch]);extrema[2]=std::max(extrema[2],output[ch]);extrema[3]=std::max(extrema[3],-output[ch]);}
  if(++samples>=binSamples){for(int k=0;k<4;++k){history[k][head]=clamp(extrema[k],0.f,1.f);extrema[k]=0;}head=(head+1)%bins;samples=0;}
  if(++publishSamples>=publishPeriod){publishSamples=0;return true;}return false;
 }
};
struct Ring {
 std::array<float,8192> data{};unsigned pos=0;
 void reset(){data.fill(0);pos=0;}
 float read(float samples)const{samples=clamp(samples,1.f,8190.f);int n=int(samples);float f=samples-n;float a=data[(pos-n)&8191],b=data[(pos-n-1)&8191];return a+(b-a)*f;}
 void write(float x){data[pos]=x;pos=(pos+1)&8191;}
};
//20ms coefficient morph avoids hard filter jumps during UI automation.
struct MorphBiquad {
 Biquad filter;BiquadCoeffs current,target,step;int remaining=0,length=882;bool ready=false;
 void prepare(double sr){length=std::max(1,int(sr*.02));ready=false;remaining=0;}
 void setCoeffs(const BiquadCoeffs&c){target=c;if(!ready){current=c;filter.setCoeffs(c);ready=true;remaining=0;return;}remaining=length;
  step.b0=(c.b0-current.b0)/length;step.b1=(c.b1-current.b1)/length;step.b2=(c.b2-current.b2)/length;step.a1=(c.a1-current.a1)/length;step.a2=(c.a2-current.a2)/length;}
 void reset(){filter.reset();current=target;filter.setCoeffs(current);remaining=0;}
 float process(float x){if(remaining){current.b0+=step.b0;current.b1+=step.b1;current.b2+=step.b2;current.a1+=step.a1;current.a2+=step.a2;if(!--remaining)current=target;filter.setCoeffs(current);}return filter.process(x);}
};
struct Split {
 MorphBiquad low1,low2,high1,high2;
 void prepare(double sr){low1.prepare(sr);low2.prepare(sr);high1.prepare(sr);high2.prepare(sr);}
 void setup(double sr,float hz){auto l=BiquadCoeffs::lowpass(sr,hz),h=BiquadCoeffs::highpass(sr,hz);low1.setCoeffs(l);low2.setCoeffs(l);high1.setCoeffs(h);high2.setCoeffs(h);}
 void reset(){low1.reset();low2.reset();high1.reset();high2.reset();}
 void run(float x,float&lo,float&hi){lo=low2.process(low1.process(x));hi=high2.process(high1.process(x));}
};
struct Bands {
 Split a,b;MorphBiquad alignLow,dryA,dryB;
 void prepare(double sr){a.prepare(sr);b.prepare(sr);alignLow.prepare(sr);dryA.prepare(sr);dryB.prepare(sr);}
 void setup(double sr,float lo,float hi){a.setup(sr,lo);b.setup(sr,hi);alignLow.setCoeffs(BiquadCoeffs::allpass(sr,hi));dryA.setCoeffs(BiquadCoeffs::allpass(sr,lo));dryB.setCoeffs(BiquadCoeffs::allpass(sr,hi));}
 void reset(){a.reset();b.reset();alignLow.reset();dryA.reset();dryB.reset();}
 std::array<float,3> run(float x){float low,upper,mid,high;a.run(x,low,upper);b.run(upper,mid,high);return {alignLow.process(low),mid,high};}
 float dry(float x){return dryB.process(dryA.process(x));}
};
struct Dynamics {
 float envelope=0,gain=1,gr=0;
 void reset(){envelope=0;gain=1;gr=0;}
 void detect(float peak,float attack,float release){float recovery=release/(1.f+envelope*.65f);envelope+=(peak>envelope?attack:recovery)*(peak-envelope);}
 void target(float amount,bool makeup,float thresholdOffset=0){
  if(amount<.00001f){gr=0;desired=1;return;}
  gr=reduction(gainToDb(envelope),amount,thresholdOffset);
  // This is restrained automatic makeup, not an LUFS matching claim.
  desired=dbToGain(-gr+(makeup?std::min(4.f,gr*.28f):0.f));
 }
 float desired=1;
 float next(float speed){gain+=speed*(desired-gain);return gain;}
};
}

class CratePress:public mpc::Plugin {
public:
 enum Key { Output,Input,Compress,Punch,Release,LowComp,MidComp,HighComp,Duck,Drive,Tone,Lofi,Drift,LowCut,HighCut,Mix,DuckSource,Division,DuckLow,DuckMid,DuckHigh,Detector,Sensitivity,WowRate,Flutter,Noise,Width,PeakGuard,Bypass,LofiType,SplitLow,SplitHigh,AutoMakeup,InputMeter,OutputMeter,CompMeter,LowMeter,MidMeter,HighMeter,DuckMeter,BaseCount,CurveBand=BaseCount,WaveBase,CurveBase=WaveBase+256,CurveX=CurveBase+48,CurveY,LegacyCount,PerfGate=LegacyCount,PerfSettings=PerfGate+8,TapeTime=PerfSettings,ReverseLength,RepeatDiv,EchoDiv,EchoFeedback,EchoMix,FilterDip,FxAmount,CrushBits,CrushRate,WobbleRate,WobbleDepth,DropoutRate,DropoutDepth,FxRelease,FxOutput,PerfProgress,Latch=PerfProgress+8,Count };
 static mpc::PluginInfo description(){mpc::PluginInfo i;i.name="Crate Press";i.vendor="kim";i.uniqueId=MPC_FOURCC('K','3','6','Z');i.version=202;i.isInstrument=false;i.numInputs=2;i.numOutputs=2;i.wantsMidi=false;i.tailSamples=44100*240;return i;}
 CratePress():mpc::Plugin(description()){
  using namespace mpc;
  addParam(floatParam("output","Output",-24,6,0,"dB",1));
  addParam(floatParam("input","Input",-18,18,0,"dB",1));
  addParam(floatParam("compress","Compress",0,100,0,"%",0));
  addParam(floatParam("punch","Punch",0,100,50,"%",0));
  addParam(logParam("release","Release",30,800,160,"ms",0));
  addParam(floatParam("lowComp","Low Comp",0,100,0,"%",0));
  addParam(floatParam("midComp","Mid Comp",0,100,0,"%",0));
  addParam(floatParam("highComp","High Comp",0,100,0,"%",0));
  addParam(floatParam("duck","Duck",0,100,0,"%",0));
  addParam(floatParam("drive","Drive",0,100,0,"%",0));
  addParam(floatParam("tone","Tone",-100,100,0,"%",0));
  addParam(floatParam("lofi","Lo-Fi",0,100,0,"%",0));
  addParam(floatParam("drift","Drift",0,100,0,"%",0));
  addParam(floatParam("lowCut","Low Cut",0,250,0,"Hz",0));
  addParam(logParam("highCut","High Cut",2000,20000,20000,"Hz",0));
  addParam(floatParam("mix","Mix",0,100,100,"%",0));
  addParam(choiceParam("duckSource","Duck Source",{"Bass Sense","Tempo"},0));
  addParam(choiceParam("division","Division",{"1/4","1/8","1/8 DOT","1/16"},0));
  addParam(boolParam("duckLow","Duck Low",false));addParam(boolParam("duckMid","Duck Mid",true));addParam(boolParam("duckHigh","Duck High",true));
  addParam(logParam("detector","Detector",40,250,110,"Hz",0));
  addParam(floatParam("sensitivity","Sensitivity",-36,0,-18,"dB",0));
  addParam(logParam("wowRate","Wow Rate",.1f,2,.55f,"Hz",2));
  addParam(floatParam("flutter","Flutter",0,100,25,"%",0));
  addParam(floatParam("noise","Noise",0,100,0,"%",0));
  addParam(floatParam("width","Stereo Width",50,150,100,"%",0));
  addParam(boolParam("peakGuard","Peak Guard",true));addParam(boolParam("bypass","Bypass",false));
  addParam(choiceParam("lofiType","Lo-Fi Type",{"Warm","Sampler","Crunch"},0));
  addParam(logParam("splitLow","Low Split",80,400,160,"Hz",0));
  addParam(logParam("splitHigh","High Split",1200,6000,3000,"Hz",0));
  addParam(boolParam("autoMakeup","Auto Makeup",true));
  for(const auto&name:std::array<const char*,7>{"Input Peak","Output Peak","Comp GR","Low GR","Mid GR","High GR","Duck GR"}){auto p=floatParam(name,name,-72,72,0,"dB",1);p.automatable=false;addParam(p);}
  addParam(choiceParam("curveBand","Curve Band",{"FULL","LOW","MID","HIGH"},0));
  for(int k=0;k<256;++k){std::string id="scope"+std::to_string(k);auto p=floatParam(id.c_str(),id.c_str(),0,1,0,"",3);p.automatable=false;addParam(p);}
  for(int k=0;k<48;++k){std::string id="curve"+std::to_string(k);auto p=floatParam(id.c_str(),id.c_str(),0,1,float(k)/47,"",3);p.automatable=false;addParam(p);}
  for(auto name:{"Curve X","Curve Y"}){auto p=floatParam(name,name,0,1,0,"",3);p.automatable=false;addParam(p);}
  for(auto name:{"Tape Stop","Reverse","Repeat","Dub Echo","Filter Dip Hold","Crush","Wobble","Dropout"}){auto p=boolParam(name,name,false);p.automatable=false;addParam(p);}
  addParam(floatParam("tapeTime","Tape Time",80,2000,700,"ms",0));
  addParam(choiceParam("reverseLength","Reverse Length",{"1/8","1/4","1/2","1 BAR"},1));
  addParam(choiceParam("repeatDivision","Repeat Division",{"1/4","1/8","1/16","1/32"},1));
  addParam(choiceParam("echoDivision","Echo Division",{"1/4","1/8","1/8 DOT","1/4 DOT"},2));
  addParam(floatParam("echoFeedback","Echo Feedback",0,80,55,"%",0));addParam(floatParam("echoMix","Echo Mix",0,100,35,"%",0));
  addParam(logParam("filterDip","Filter Dip",200,6000,900,"Hz",0));addParam(floatParam("fxAmount","FX Amount",0,100,100,"%",0));
  addParam(floatParam("crushBits","Crush Bits",3,16,8,"bit",0));addParam(floatParam("crushRate","Crush Rate",0,100,60,"%",0));
  addParam(logParam("wobbleRate","Wobble Rate",.1f,12,2,"Hz",1));addParam(floatParam("wobbleDepth","Wobble Depth",0,100,55,"%",0));
  addParam(logParam("dropoutRate","Dropout Rate",1,32,8,"Hz",1));addParam(floatParam("dropoutDepth","Dropout Depth",0,100,100,"%",0));
  addParam(floatParam("fxRelease","FX Release",5,200,15,"ms",0));addParam(floatParam("fxOutput","FX Output",-12,6,0,"dB",1));
  for(int k=0;k<8;++k){std::string name="FX Activity "+std::to_string(k+1);auto p=floatParam(name.c_str(),name.c_str(),0,1,0,"",2);p.automatable=false;addParam(p);}
  {auto p=boolParam("performanceLatch","LATCH",false);p.automatable=false;addParam(p);}
  makePrograms();prepare(44100,128);
 }
 // MPC caches native skin values. Queue engine-owned changes and drain only on non-render reads.
 static std::uintptr_t threadToken(){static thread_local unsigned char token;return reinterpret_cast<std::uintptr_t>(&token);}
 void markRenderThread(){
  const auto token=threadToken();for(auto&seen:renderThreads_)if(seen.load(std::memory_order_relaxed)==token)return;
  for(auto&seen:renderThreads_){std::uintptr_t empty=0;if(seen.compare_exchange_strong(empty,token,std::memory_order_relaxed))return;}
  renderOverflow_.store(true,std::memory_order_relaxed);
 }
 void publish(int index,float value){
  const float norm=param(index).toNormalised(value);if(norm==getNormalised(index))return;
  setNormalised(index,norm);changed_[index].store(true,std::memory_order_release);
 }
 void queueAll(){for(auto&flag:changed_)flag.store(true,std::memory_order_release);displayPending_.store(true,std::memory_order_release);}
 void flushFromValueRead(){
  if(renderOverflow_.load(std::memory_order_relaxed))return;
  const auto token=threadToken();for(auto&seen:renderThreads_)if(seen.load(std::memory_order_relaxed)==token)return;
  if(flushing_.test_and_set(std::memory_order_acquire))return;
  for(int i=0;i<Count;++i)if(changed_[i].exchange(false,std::memory_order_acq_rel))notifyHostParameterChanged(i);
  if(displayPending_.exchange(false,std::memory_order_acq_rel))requestDisplayUpdate();
  flushing_.clear(std::memory_order_release);
 }
 void requestHostIdle(){if(host_&&effect_)host_(effect_,audioMasterNeedIdle_Deprecated,0,0,nullptr,0.f);}
 void makePrograms(){
  #include "FactoryPrograms.inc"
 }
 void prepare(double sr,int)override{
  sr_=std::isfinite(sr)?clamp(sr,8000.,192000.):44100.;
  info_.tailSamples=int(sr_*240);performance_.prepare(sr_);
  for(auto&b:bands_)b.prepare(sr_);for(auto&b:detHp_)b.prepare(sr_);for(auto&b:detLp_)b.prepare(sr_);for(auto&b:toneShelf_)b.prepare(sr_);for(auto&b:outHp_)b.prepare(sr_);for(auto&b:outLp_)b.prepare(sr_);latency_=int(std::round(sr_*.006));setLatency(latency_);
  for(int i=0;i<Count;++i){smooth_[i].setTime(i==SplitLow||i==SplitHigh?100.f:20.f,sr_);smooth_[i].snap(get(i));}
  for(auto*s:{&attackCoef_,&bandAttackCoef_,&releaseCoef_})s->setTime(20,sr_);
  inputGain_.setTime(20,sr_);outputGain_.setTime(20,sr_);inputGain_.snap(dbToGain(get(Input)));outputGain_.snap(dbToGain(get(Output)));
  wetBlend_.setTime(10,sr_);wetBlend_.snap(getBool(Bypass)?0.f:1.f);compGainSpeed_=onePoleCoeff(3,sr_);duckAttack_=onePoleCoeff(1.5f,sr_);meterDecay_=std::exp(-1.f/(.3f*float(sr_)));configuredLo_=configuredHi_=configuredDetect_=configuredLC_=configuredHC_=-1;configuredTone_=999;scope_.prepare(sr_);reset();
 }
 void reset()override{
  performance_.reset();clearPerformance_=true;latchWasOn_=getBool(Latch);
  for(auto&b:bands_)b.reset();for(auto&d:wetDelay_)d.reset();for(auto&d:dryDelay_)d.reset();for(auto&d:rawDelay_)d.reset();
  for(auto&d:dyn_){d.reset();d.desired=1;}
  for(auto&b:detHp_)b.reset();for(auto&b:detLp_)b.reset();for(auto&b:toneShelf_)b.reset();for(auto&b:outHp_)b.reset();for(auto&b:outLp_)b.reset();
  for(int i=0;i<Count;++i)smooth_[i].snap(get(i));inputGain_.snap(dbToGain(get(Input)));outputGain_.snap(dbToGain(get(Output)));wetBlend_.snap(getBool(Bypass)?0.f:1.f);
  attackCoef_.snap(onePoleCoeff(1.f+get(Punch)*.38f,sr_));bandAttackCoef_.snap(onePoleCoeff(3.f+get(Punch)*.12f,sr_));releaseCoef_.snap(onePoleCoeff(get(Release),sr_));
  scope_.reset();heldGR_.fill(0);curveAmount_=-1;curveBand_=-1;duckDesired_=duckGain_=1;
  peakIn_=peakOut_=detEnv_=detSlow_=duckDb_=0;hold_={0,0};colourLp_={0,0};phaseWow_=phaseFlutter_=holdPhase_=0;tick_=0;random_=0x4b435250;
 }
 void process(mpc::ProcessContext&c)override{
  markRenderThread();
  if(!c.outputs||c.numOutputs<=0||c.numFrames<=0)return;
  std::array<float,BaseCount> setting{};for(int i=0;i<BaseCount;++i)setting[i]=get(i);
  for(int i=1;i<=Width;++i)smooth_[i].setTarget(get(i));smooth_[AutoMakeup].setTarget(get(AutoMakeup));
  inputGain_.setTarget(dbToGain(get(Input)));outputGain_.setTarget(dbToGain(get(Output)));wetBlend_.setTarget(getBool(Bypass)?0.f:1.f);
  const float low=get(SplitLow),high=std::min(get(SplitHigh),float(sr_*.4)),detect=std::min(get(Detector),float(sr_*.25));
  if(low!=configuredLo_||high!=configuredHi_){for(auto&b:bands_)b.setup(sr_,low,high);configuredLo_=low;configuredHi_=high;}
  if(detect!=configuredDetect_){for(auto&b:detHp_)b.setCoeffs(BiquadCoeffs::highpass(sr_,28));for(auto&b:detLp_)b.setCoeffs(BiquadCoeffs::lowpass(sr_,detect));configuredDetect_=detect;}
  float tone=get(Tone);
  if(tone!=configuredTone_){for(auto&b:toneShelf_)b.setCoeffs(BiquadCoeffs::highShelf(sr_,1800,.707f,tone*.065f));configuredTone_=tone;}
  float lc=get(LowCut),hc=std::min(get(HighCut),float(sr_*.45));
  if(lc!=configuredLC_){for(auto&b:outHp_)b.setCoeffs(lc<1?BiquadCoeffs::identity():BiquadCoeffs::highpass(sr_,std::max(10.f,lc)));configuredLC_=lc;}
  if(hc!=configuredHC_){for(auto&b:outLp_)b.setCoeffs(get(HighCut)>=19500?BiquadCoeffs::identity():BiquadCoeffs::lowpass(sr_,hc));configuredHC_=hc;}
  float targetAttack=onePoleCoeff(1.f+get(Punch)*.38f,sr_),targetBandAttack=onePoleCoeff(3.f+get(Punch)*.12f,sr_),targetRelease=onePoleCoeff(get(Release),sr_);
  attackCoef_.setTarget(targetAttack);bandAttackCoef_.setTarget(targetBandAttack);releaseCoef_.setTarget(targetRelease);
  const float duckThreshold=dbToGain(get(Sensitivity)),makeup=getBool(AutoMakeup);
  const bool guard=getBool(PeakGuard);
  const int selected=clamp(getInt(CurveBand),0,3);const float curveAmount=setting[selected==0?Compress:LowComp+selected-1]*.01f;

  const bool tempo=getInt(DuckSource)==1;const int type=getInt(LofiType);
  static const double beats[]={1,.5,.75,.25};double duration=beats[clamp(getInt(Division),0,3)];double bpm=std::isfinite(c.transport.bpm)?clamp(c.transport.bpm,20.,300.):120.;
  const bool latch=getBool(Latch);if(latchWasOn_&&!latch)for(int k=0;k<8;++k)publish(PerfGate+k,0);latchWasOn_=latch;
  crate::PerformanceSettings fx;
  fx.tapeMs=get(TapeTime);fx.reverse=getInt(ReverseLength);fx.repeat=getInt(RepeatDiv);fx.echoDivision=getInt(EchoDiv);
  fx.feedback=get(EchoFeedback)*.01f;fx.echoMix=get(EchoMix)*.01f;fx.filterHz=get(FilterDip);fx.amount=get(FxAmount)*.01f;
  fx.bits=get(CrushBits);fx.crushRate=get(CrushRate)*.01f;fx.wobbleRate=get(WobbleRate);fx.wobbleDepth=get(WobbleDepth)*.01f;
  fx.dropoutRate=get(DropoutRate);fx.dropoutDepth=get(DropoutDepth)*.01f;fx.releaseMs=get(FxRelease);fx.output=dbToGain(get(FxOutput));
  std::array<bool,8> gates{};if(!clearPerformance_)for(int k=0;k<8;++k)gates[k]=getBool(PerfGate+k);
  performance_.configure(fx,gates,bpm);
  for(int n=0;n<c.numFrames;++n){
   const float attack=attackCoef_.process(),bandAttack=bandAttackCoef_.process(),release=releaseCoef_.process();
   float finalOut[2]={0,0},alignedIn[2]={0,0};float in[2]={0,0};for(int ch=0;ch<2;++ch){int src=c.numInputs>1?ch:0;if(c.inputs&&c.numInputs>0&&c.inputs[src])in[ch]=clamp(sanitize(c.inputs[src][n]),-8.f,8.f);}
   float inputGain=inputGain_.process(),outputGain=outputGain_.process();float vals[16];for(int i=1;i<=Mix;++i)vals[i]=smooth_[i].process();float duckAmount=vals[Duck]*.01f;
   float detector=0;for(int ch=0;ch<2;++ch)detector=std::max(detector,std::fabs(detLp_[ch].process(detHp_[ch].process(in[ch]*inputGain))));
   detEnv_+=(detector>detEnv_?duckAttack_:release)*(detector-detEnv_);detSlow_+=(.00003f*44100.f/float(sr_))*(detector-detSlow_);
   float trigger=clamp((detEnv_-.35f*detSlow_)/std::max(.001f,duckThreshold),0.f,1.f);
   if(tempo){
    double ppq=c.transport.ppqPos+double(n)*bpm/(60.*sr_);if(!std::isfinite(ppq))ppq=0;
    double phase=ppq/duration-std::floor(ppq/duration);double recovery=std::min(.95,double(vals[Release])*.001*bpm/(60.*duration));
    trigger=c.transport.playing?float(std::max(0.,1.-phase/std::max(.05,recovery))):0.f;
   }
   float wantedDuck=duckAmount*18.f*trigger;duckDb_+=(wantedDuck>duckDb_?duckAttack_:release)*(wantedDuck-duckDb_);
   // Four envelope followers, one shared gain per stereo pair. Expensive curve evaluation only every16frames.
   dyn_[0].detect(std::max(std::fabs(in[0]),std::fabs(in[1]))*inputGain,attack,release);
   if((tick_&15)==0){dyn_[0].target(vals[Compress]*.01f,makeup);if(vals[Compress]<.00001f)dyn_[0].desired=1;}
   float fullGain=dyn_[0].next(compGainSpeed_);std::array<float,3>split[2];float dry[2];
   for(int ch=0;ch<2;++ch){split[ch]=bands_[ch].run(in[ch]*inputGain*fullGain);dry[ch]=bands_[ch].dry(in[ch]*inputGain);}
   for(int b=0;b<3;++b){dyn_[b+1].detect(std::max(std::fabs(split[0][b]),std::fabs(split[1][b])),bandAttack,release);
    if((tick_&15)==0){dyn_[b+1].target(vals[LowComp+b]*.01f,makeup,-5.f);if(vals[LowComp+b]<.00001f)dyn_[b+1].desired=1;}}
   float bandGain[3];for(int b=0;b<3;++b){bandGain[b]=dyn_[b+1].next(compGainSpeed_);}
   float weights[3];for(int b=0;b<3;++b)weights[b]=smooth_[DuckLow+b].process();
   float duckGain=1;
   if((tick_&15)==0)duckDesired_=dbToGain(-duckDb_);duckGain=duckGain_+=(compGainSpeed_)*(duckDesired_-duckGain_);
   const float wowRate=smooth_[WowRate].process(),flutter=smooth_[Flutter].process(),noise=smooth_[Noise].process();const float stereoWidth=smooth_[Width].process()*.01f;
   phaseWow_=wrapPhase(phaseWow_+wowRate/float(sr_));phaseFlutter_=wrapPhase(phaseFlutter_+(5.8f+flutter*.025f)/float(sr_));
   float wobble=(fastSin01(phaseWow_)*.0012f+fastSin01(phaseFlutter_)*flutter*.000002f)*float(sr_)*(vals[Drift]*.01f);
   float crush=vals[Lofi]*.01f,drive=vals[Drive]*.01f;holdPhase_+=1.f-crush*(type==0?.40f:.94f);bool update=holdPhase_>=1;if(update)holdPhase_-=1;
   float wet[2],delayedDry[2],raw[2];
   for(int ch=0;ch<2;++ch){
    float x=0;for(int b=0;b<3;++b)x+=split[ch][b]*bandGain[b]*(1.f+weights[b]*(duckGain-1.f));
    if(drive>.00001f)x=(1-drive)*x+drive*fastTanh(x*(1.f+7.f*drive))/std::max(1.f,1.f+2.f*drive);
    if(update)hold_[ch]=x;float bits=16.f-crush*(type==2?12.f:type==1?10.f:5.f);float quant=float(1u<<clamp(int(bits),1,16));float degraded=std::round(hold_[ch]*quant)/quant;
    colourLp_[ch]+=(.85f-.65f*crush)*(degraded-colourLp_[ch]);x+=(colourLp_[ch]-x)*crush;
    if(noise>0){random_^=random_<<13;random_^=random_>>17;random_^=random_<<5;x+=(float(int32_t(random_))*(1.f/2147483648.f))*(noise*.00002f);}
    wetDelay_[ch].write(x);dryDelay_[ch].write(dry[ch]);rawDelay_[ch].write(in[ch]);
    wet[ch]=outLp_[ch].process(outHp_[ch].process(toneShelf_[ch].process(wetDelay_[ch].read(float(latency_+1)+wobble))));
    delayedDry[ch]=dryDelay_[ch].read(float(latency_+1));raw[ch]=rawDelay_[ch].read(float(latency_+1));
   }
   float m=(wet[0]+wet[1])*.5f,s=(wet[0]-wet[1])*.5f*stereoWidth;wet[0]=m+s;wet[1]=m-s;float blend=wetBlend_.process();
   std::array<float,2>bus{};for(int ch=0;ch<2;++ch)bus[ch]=delayedDry[ch]+(wet[ch]-delayedDry[ch])*(vals[Mix]*.01f);
   bus=performance_.process(bus);
   for(int ch=0;ch<2;++ch){float x=raw[ch]+(bus[ch]-raw[ch])*blend;x*=outputGain;
    if(guard){float a=std::fabs(x);if(a>.95f)x=std::copysign(.95f+.04f*(a-.95f)/(.04f+a-.95f),x);}
    if(!std::isfinite(x))x=0;finalOut[ch]=x;alignedIn[ch]=raw[ch];peakIn_=std::max(std::fabs(in[ch]),peakIn_*meterDecay_);peakOut_=std::max(std::fabs(x),peakOut_*meterDecay_);if(ch<c.numOutputs&&c.outputs[ch])c.outputs[ch][n]=x;
   }
   heldGR_[0]=std::max(heldGR_[0],dyn_[0].gr);for(int b=0;b<3;++b)heldGR_[b+1]=std::max(heldGR_[b+1],dyn_[b+1].gr);heldGR_[4]=std::max(heldGR_[4],duckDb_);
   if(scope_.push(alignedIn,finalOut)){
    // Transient touch gates are disarmed on reset and cleared at the first display tick.
    // Restoring a project/preset never resumes a held performance effect.
    if(clearPerformance_){for(int k=0;k<8;++k)publish(PerfGate+k,0);clearPerformance_=false;}
    for(int k=0;k<8;++k)publish(PerfProgress+k,performance_.progress[k]);
    if(selected!=curveBand_||curveAmount!=curveAmount_){for(int k=0;k<48;++k){float db=-48.f+48.f*k/47.f;float y=db-crate::reduction(db,curveAmount,selected==0?0.f:-5.f);publish(CurveBase+k,clamp((y+48.f)/48.f,0.f,1.f));}curveBand_=selected;curveAmount_=curveAmount;}
    publish(InputMeter,clamp(gainToDb(peakIn_),-72.f,12.f));publish(OutputMeter,clamp(gainToDb(peakOut_),-72.f,12.f));publish(CompMeter,heldGR_[0]);for(int b=0;b<3;++b)publish(LowMeter+b,heldGR_[b+1]);publish(DuckMeter,heldGR_[4]);heldGR_.fill(0);
    for(int k=0;k<4;++k)for(int j=0;j<64;++j)publish(WaveBase+k*64+j,scope_.history[k][(scope_.head+j)%64]);
    float db=gainToDb(dyn_[selected].envelope);publish(CurveX,clamp((db+48.f)/48.f,0.f,1.f));publish(CurveY,clamp((db-crate::reduction(db,curveAmount,selected==0?0.f:-5.f)+48.f)/48.f,0.f,1.f));
   }
   ++tick_;
  }
  for(int ch=2;ch<c.numOutputs;++ch)if(c.outputs[ch])std::fill(c.outputs[ch],c.outputs[ch]+c.numFrames,0.f);
  // Value storage only; no notifyHostParameterChanged/requestDisplayUpdate from the render thread.

 }
 bool formatValue(int i,float v,std::string&out)override{
  if((i==LowCut)&&v<1){out="Off";return true;}
  if(i==HighCut&&v>=19500){out="Open";return true;}
  if(i>=InputMeter&&i<BaseCount){char b[32];std::snprintf(b,sizeof(b),"%.1f dB",double(v));out=b;return true;}
  return false;
 }
private:
 std::array<std::atomic<bool>,Count>changed_{};std::atomic<bool>displayPending_{false},renderOverflow_{false};
 std::array<std::atomic<std::uintptr_t>,8>renderThreads_{};std::atomic_flag flushing_=ATOMIC_FLAG_INIT;
 double sr_=44100;int latency_=265;unsigned tick_=0;uint32_t random_=0x4b435250;
 crate::Performance performance_;bool clearPerformance_=true,latchWasOn_=false;
 std::array<float,5>heldGR_{};crate::Scope scope_;float curveAmount_=-1;int curveBand_=-1;
 std::array<Smoother,Count>smooth_;Smoother inputGain_,outputGain_,wetBlend_,attackCoef_,bandAttackCoef_,releaseCoef_;
 std::array<crate::Bands,2>bands_;std::array<crate::Ring,2>wetDelay_,dryDelay_,rawDelay_;std::array<crate::Dynamics,4>dyn_;
 std::array<crate::MorphBiquad,2>detHp_,detLp_,toneShelf_,outHp_,outLp_;std::array<float,2>hold_{},colourLp_{};
 float peakIn_=0,peakOut_=0,detEnv_=0,detSlow_=0,duckDb_=0,duckDesired_=1,duckGain_=1,phaseWow_=0,phaseFlutter_=0,holdPhase_=0;
 float configuredLo_=-1,configuredHi_=-1,configuredDetect_=-1,configuredTone_=999,configuredLC_=-1,configuredHC_=-1,compGainSpeed_=1,duckAttack_=1,meterDecay_=.999f;
};
// Plugin-local bridge: shared SDK stays unmodified.
static VstIntPtr crateDispatch(AEffect*e,VstInt32 op,VstInt32 index,VstIntPtr value,void*ptr,float opt){
 auto*p=static_cast<CratePress*>(e->object);
 const auto result=mpc::detail::dispatcher(e,op,index,value,ptr,opt);
 if(op==effOpen)p->requestHostIdle();
 if(op==effSetProgram||op==effSetChunk)p->queueAll();
 if(op==effSetProgram||op==effSetChunk||op==effIdle_Deprecated||op==effEditIdle||op==effGetParamDisplay||op==effGetProgram)p->flushFromValueRead();
 return result;
}
static float crateGetParameter(AEffect*e,VstInt32 index){static_cast<CratePress*>(e->object)->flushFromValueRead();return mpc::detail::getParameter(e,index);}
extern "C" MPC_EXPORT AEffect* VSTPluginMain(audioMasterCallback host){
 if(host&&host(nullptr,audioMasterVersion,0,0,nullptr,0)==0)return nullptr;
 auto*e=mpc::detail::createEffect(new CratePress(),host);
 if(e){e->dispatcher=&crateDispatch;e->getParameter=&crateGetParameter;}
 return e;
}
