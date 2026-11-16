#include <cstdlib>
#include <new>
#include <atomic>
static std::atomic<int> realtimeAllocations{0};static thread_local bool inAudio=false;
void* operator new(std::size_t n){if(inAudio)++realtimeAllocations;if(void*p=std::malloc(n))return p;throw std::bad_alloc();}
void* operator new[](std::size_t n){return ::operator new(n);}
void operator delete(void*p)noexcept{std::free(p);}void operator delete[](void*p)noexcept{std::free(p);}
#include "../src/CratePress.cpp"
#include <fstream>
#include <iostream>
#include <vector>
#include <cassert>
using K=CratePress;
static void check(bool ok,const char* msg){if(!ok){std::cerr<<"FAIL "<<msg<<"\n";std::exit(1);}}
struct Audio {
 CratePress p;float l[512]{},r[512]{},a[512]{},b[512]{};float*ins[2]{l,r};float*outs[2]{a,b};mpc::ProcessContext c{};
 Audio(){c.inputs=ins;c.outputs=outs;c.numInputs=c.numOutputs=2;c.numFrames=128;c.sampleRate=44100;c.transport.bpm=84;c.transport.playing=true;}
 void run(int frames=128){c.numFrames=frames;inAudio=true;p.process(c);inAudio=false;for(int i=0;i<frames;++i)check(std::isfinite(a[i])&&std::isfinite(b[i])&&std::fabs(a[i])<32&&std::fabs(b[i])<32,"finite bounded stereo output");c.transport.ppqPos+=frames*c.transport.bpm/(60*c.sampleRate);}
};
void snapshot(Audio&x,const char*path){std::ofstream f(path);f<<"{\"program\":\""<<x.p.programName(x.p.currentProgram())<<"\",\"params\":[";for(int i=0;i<x.p.numParams();++i){if(i)f<<',';f<<x.p.getNormalised(i);}f<<"]}";}
void wav(const std::string&path,const std::vector<int16_t>&v){std::ofstream f(path,std::ios::binary);auto u16=[&](uint16_t x){f.put(x&255);f.put(x>>8);};auto u32=[&](uint32_t x){u16(x&65535);u16(x>>16);};f.write("RIFF",4);u32(36+v.size()*2);f.write("WAVEfmt ",8);u32(16);u16(1);u16(2);u32(44100);u32(44100*4);u16(4);u16(16);f.write("data",4);u32(v.size()*2);for(auto s:v)u16(uint16_t(s));}
int main(int argc,char**argv){
 std::string out=argc>1?argv[1]:"build";
 Audio x;check(x.p.numPrograms()==32&&x.p.numParams()==380,"bank and stable telemetry schema");
 for(int k=0;k<347;++k)if(k>=K::InputMeter&&k!=K::CurveBand)check(!x.p.param(k).automatable,"display telemetry excluded from automation");
 // Real signed extrema, both channels; opposite polarity must not cancel.
 x.p.set(K::Bypass,1);x.p.set(K::PeakGuard,0);x.p.reset();
 for(int i=0;i<512;++i){x.l[i]=.6f;x.r[i]=-.25f;}
 for(int block=0;block<400;++block)x.run();
 for(int j=0;j<64;++j){check(std::fabs(x.p.get(K::WaveBase+j)-.6f)<1e-5,"input positive waveform is measured audio");check(std::fabs(x.p.get(K::WaveBase+64+j)-.25f)<1e-5,"input negative waveform");check(std::fabs(x.p.get(K::WaveBase+128+j)-.6f)<1e-5,"aligned bypass output waveform");}
 // Actual bypass impulse position must match the host latency declaration.
 x.p.reset();for(int i=0;i<512;++i)x.l[i]=x.r[i]=0;x.l[0]=x.r[0]=.5f;x.run(512);int peak=0;for(int i=1;i<512;++i)if(std::fabs(x.a[i])>std::fabs(x.a[peak]))peak=i;check(peak==x.p.info().latencySamples,"reported and measured latency match");
 // Curves use the exact same knee/ratio/threshold function as audio dynamics.
 for(int band=0;band<4;++band){x.p.set(K::CurveBand,band);x.p.set(band==0?K::Compress:K::LowComp+band-1,67);x.p.reset();for(int j=0;j<15;++j)x.run();for(int k=0;k<48;++k){float db=-48+48.f*k/47;float y=mpc::dsp::clamp((db-crate::reduction(db,.67f,band? -5.f:0.f)+48)/48.f,0.f,1.f);check(std::fabs(x.p.get(K::CurveBase+k)-y)<1e-5,"transfer graph matches DSP");}}
 //32 actual programs,3rates, variable blocks, state restore of every sound parameter.
 std::ofstream report(out+"/presets.csv");report<<"preset,sample_rate,peak,rms\n";
 for(int program=0;program<32;++program)for(int sr:{44100,48000,96000}){
  x.p.setProgram(program);x.p.prepare(sr,512);x.c.sampleRate=sr;x.c.transport.ppqPos=0;double sum=0;float peak=0;int total=0;std::vector<int16_t>audio;
  for(int block=0;block<200;++block){int n=block%3==0?31:block%3==1?128:511;
   for(int i=0;i<n;++i){double t=double(total+i)/sr;double beat=std::fmod(t,.7142857);float kick=float(.65*std::sin(2*3.14159265*61*t)*std::exp(-beat*27));float mid=float(.24*std::sin(2*3.14159265*440*t)*(.35+.65*std::exp(-std::fmod(t+.18,.357)*18)));float hat=float(.08*std::sin(2*3.14159265*8000*t)*std::exp(-std::fmod(t,.1785)*65));x.l[i]=kick+mid+hat;x.r[i]=kick+mid*.8f-hat;}
   x.run(n);for(int i=0;i<n;++i){peak=std::max(peak,std::max(std::fabs(x.a[i]),std::fabs(x.b[i])));sum+=x.a[i]*x.a[i]+x.b[i]*x.b[i];if(sr==44100){audio.push_back(int16_t(mpc::dsp::clamp(x.a[i],-1.f,1.f)*32767));audio.push_back(int16_t(mpc::dsp::clamp(x.b[i],-1.f,1.f)*32767));}}total+=n;if(program==1&&sr==44100&&x.p.get(K::CompMeter)>2)snapshot(x,(out+"/ui-state.json").c_str());
  }
  check(peak>.001&&peak<1,"all presets audible and peak bounded");report<<'"'<<x.p.programName(program)<<"\","<<sr<<','<<peak<<','<<std::sqrt(sum/(total*2))<<'\n';
  auto state=x.p.internalSaveState();float settings[33];for(int i=0;i<33;++i)settings[i]=x.p.getNormalised(i);for(int i=0;i<33;++i)x.p.setNormalised(i,.123f);check(x.p.internalLoadState(state.data(),state.size()),"valid state load");x.p.reset();for(int i=0;i<33;++i)check(std::fabs(x.p.getNormalised(i)-settings[i])<1e-5,"sound settings restored");
  if(sr==44100){wav(out+"/preset-"+std::to_string(program)+".wav",audio);}
 }
 // Input/output aliasing, mono, missing input and sweeping every audible setting.
 x.p.setProgram(1);x.p.prepare(44100,512);x.c.sampleRate=44100;
 for(int block=0;block<300;++block){for(int k=0;k<33;++k)x.p.setNormalised(k,float((block*7+k*13)%101)/100);for(int i=0;i<512;++i)x.l[i]=x.r[i]=.2f*std::sin(float(i+block*512)*.1f);x.run(512);}
 x.c.numInputs=0;x.c.inputs=nullptr;x.run();x.c.inputs=x.ins;x.c.numInputs=1;x.c.numOutputs=1;x.run();x.c.numInputs=x.c.numOutputs=2;x.outs[0]=x.l;x.outs[1]=x.r;x.run();
 check(realtimeAllocations.load()==0,"process has zero allocations");
 std::cout<<"PASS: signed stereo waveform, aligned latency,4DSP curves,32presets x3rates,variable blocks,state,parameter sweeps,mono/in-place/null inputs,zero RT allocations\n";
}
