#include <atomic>
#include <cstdlib>
#include <new>
static std::atomic<int>allocations{0};static thread_local bool audio=false;
void* operator new(std::size_t n){if(audio)++allocations;if(auto p=std::malloc(n))return p;throw std::bad_alloc();}void*operator new[](std::size_t n){return ::operator new(n);}void operator delete(void*p)noexcept{std::free(p);}void operator delete[](void*p)noexcept{std::free(p);}
#include "../src/CratePress.cpp"
#include <iostream>
#include <fstream>
#include <vector>
static void check(bool x,const char*message){if(!x){std::cerr<<"FAIL "<<message<<'\n';std::exit(1);}}
static std::array<float,2>signal(int64_t n,double sr=44100){double t=n/sr;float a=float(.32*std::sin(6.28318530718*(137*t+23*t*t))+.16*std::sin(6.28318530718*73*t));return{a,a*.8f+float(.07*std::sin(6.28318530718*307*t))};}
static std::array<float,2>step(crate::Performance&p,std::array<float,2>x){audio=true;auto y=p.process(x);audio=false;check(std::isfinite(y[0])&&std::isfinite(y[1])&&std::fabs(y[0])<4&&std::fabs(y[1])<4,"finite bounded performance output");return y;}
int main(int argc,char**argv){
 std::string dir=argc>1?argv[1]:"build";crate::Performance p;p.prepare(44100);crate::PerformanceSettings s;std::array<bool,8>gates{};
 p.configure(s,gates,120);for(int n=0;n<44100;++n){auto x=signal(n),y=step(p,x);check(x==y,"all-off is bit-exact pass-through");}
 std::ofstream report(dir+"/performance.csv");report<<"effect,held_rms_difference,released_rms_difference\n";
 for(int effect=0;effect<8;++effect){p.reset();gates.fill(false);p.configure(s,gates,120);for(int n=0;n<44100;++n)step(p,signal(n));gates[effect]=true;p.configure(s,gates,120);double delta=0,lateEnergy=0;
  std::vector<float>held;held.reserve(52920);
  for(int n=0;n<52920;++n){auto x=signal(n+44100),y=step(p,x);held.push_back(y[0]);if(n>11025)delta+=(y[0]-x[0])*(y[0]-x[0]);if(n>48000)lateEnergy+=y[0]*y[0];}
  delta=std::sqrt(delta/(52920-11026));std::cerr<<"Effect "<<effect<<" held delta "<<delta<<"\n";check(delta>.0001,"each of8effects measurably changes real audio");
  if(effect==0)check(lateEnergy<.0001,"tape stop reaches silence while held");
  if(effect==2){double error=0;for(int n=22050;n<33075;++n)error=std::max(error,double(std::fabs(held[n]-held[n+11025])));check(error<.0001,"repeat period follows selected1/8division at120BPM");}
  gates.fill(false);p.configure(s,gates,120);double released=0;for(int n=0;n<44100;++n){auto x=signal(n+97020),y=step(p,x);if(n>22050)released+=(y[0]-x[0])*(y[0]-x[0]);}released=std::sqrt(released/22049);if(effect!=3)check(released<.00001,"release returns original audio without stuck effect");report<<effect<<','<<delta<<','<<released<<'\n';
 }
 // Echo send closes on release; a finite feedback tail continues and decays.
 p.reset();gates.fill(false);gates[3]=true;p.configure(s,gates,120);for(int n=0;n<22050;++n)step(p,signal(n));gates.fill(false);p.configure(s,gates,120);double early=0,late=0;for(int n=0;n<441000;++n){auto y=step(p,{0,0});if(n<44100)early+=y[0]*y[0];if(n>396900)late+=y[0]*y[0];}check(early>.01&&late<early*.001,"echo tail audible then decays after release");
 // Captured loops stay frozen longer than the rolling buffer duration.
 p.reset();gates.fill(false);p.configure(s,gates,120);for(int n=0;n<44100;++n)step(p,signal(n));gates[2]=true;p.configure(s,gates,120);std::array<float,256>before{},after{};for(int n=0;n<44100*10;++n){auto y=step(p,signal(n+44100));if(n>=22050&&n<22306)before[n-22050]=y[0];if(n>=441000-11025&&n<441000-11025+256)after[n-(441000-11025)]=y[0];}for(int n=0;n<256;++n)check(std::fabs(before[n]-after[n])<.0001,"long-held repeat never overwrites frozen capture");
 // Overall amount0 is neutral, even with all8buttons held.
 p.reset();gates.fill(true);s.amount=0;p.configure(s,gates,120);for(int n=0;n<22050;++n){auto x=signal(n),y=step(p,x);if(n>11025)check(std::fabs(y[0]-x[0])<.00001,"FX amount0 is neutral");}
 check(allocations==0,"performance process has zero allocations");
 // Host integration: settings survive32program changes, gates never resume on project load.
 CratePress host;check(host.info().tailSamples>=44100*240,"host tail budget covers slow-tempo echo feedback");host.set(CratePress::WobbleDepth,42);host.set(CratePress::EchoFeedback,27);for(int k=0;k<32;++k){host.setProgram(k);check(std::fabs(host.get(CratePress::WobbleDepth)-42)<.001&&std::fabs(host.get(CratePress::EchoFeedback)-27)<.001,"performance setup independent of sound preset");}
 float in[2][128]{},out[2][128]{};float*ip[2]{in[0],in[1]},*op[2]{out[0],out[1]};mpc::ProcessContext c{};c.inputs=ip;c.outputs=op;c.numInputs=c.numOutputs=2;c.numFrames=128;c.sampleRate=44100;c.transport.bpm=120;
 auto process=[&](int blocks){for(int b=0;b<blocks;++b){for(int n=0;n<128;++n){auto x=signal(b*128+n);in[0][n]=x[0];in[1][n]=x[1];}audio=true;host.process(c);audio=false;}};
 host.setProgram(1);host.reset();process(300);host.set(CratePress::Latch,1);host.set(CratePress::PerfGate+2,1);process(80);host.set(CratePress::PerfGate+1,1);process(80);check(host.get(CratePress::PerfGate+1)>0.5f&&host.get(CratePress::PerfGate+2)>0.5f,"latch holds two independently enabled effects");
 {std::ofstream f(dir+"/ui-performance-state.json");f<<"{\"program\":\"Drums / Pocket Press\",\"params\":[";for(int k=0;k<host.numParams();++k){if(k)f<<',';f<<host.getNormalised(k);}f<<"]}";}
 host.set(CratePress::PerfGate+1,0);process(60);check(host.get(CratePress::PerfGate+2)>.5f,"toggling one latched effect preserves the other");host.set(CratePress::Latch,0);process(80);for(int k=0;k<8;++k)check(host.get(CratePress::PerfGate+k)==0,"latch off releases all8gates");
 host.set(CratePress::Latch,1);host.set(CratePress::PerfGate+2,1);process(80);
 auto state=host.internalSaveState();check(host.internalLoadState(state.data(),state.size()),"performance state can load");host.reset();process(30);for(int k=0;k<8;++k)check(host.get(CratePress::PerfGate+k)==0,"loaded held gate auto-clears at initial display tick");
 check(allocations==0,"integrated performance process has zero allocations");
 std::cout<<"PASS:8audible effects,neutral off,release recovery,repeat tempo,frozen capture,echo tail,independent setup,state gate clearing,latch independent toggles/release-all,zero RT allocations\n";
}
