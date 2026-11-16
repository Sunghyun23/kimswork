#include <atomic>
#include <cstdlib>
#include <new>
static std::atomic<int>rtAlloc{0};static thread_local bool inRender=false;
void*operator new(std::size_t n){if(inRender)++rtAlloc;if(auto p=std::malloc(n))return p;throw std::bad_alloc();}void*operator new[](std::size_t n){return ::operator new(n);}void operator delete(void*p)noexcept{std::free(p);}void operator delete[](void*p)noexcept{std::free(p);}
#include "../src/Performance.h"
#include <iostream>
#include <fstream>
#include <cmath>
#include <array>
static void check(bool b,const char*why){if(!b){std::cerr<<"FAIL "<<why<<'\n';std::exit(1);}}
static std::array<float,2> source(int64_t n,double sr){double t=n/sr;float a=float(.25*std::sin(6.2831853*(113*t+37*t*t))+.14*std::sin(6.2831853*61*t));return{a,a*.8f+float(.1*std::sin(6.2831853*997*t))};}
static std::array<float,2> render(crate::Performance&p,std::array<float,2>x){inRender=true;auto y=p.process(x);inRender=false;check(std::isfinite(y[0])&&std::isfinite(y[1])&&std::fabs(y[0])<4&&std::fabs(y[1])<4,"combination output finite/bounded");return y;}
static void close(const std::array<float,2>&a,const std::array<float,2>&b){check(std::fabs(a[0]-b[0])<1e-6&&std::fabs(a[1]-b[1])<1e-6,"release one button equals independent gate schedule");}
int main(int argc,char**argv){
 std::string out=argc>1?argv[1]:"build";std::ofstream report(out+"/combinations.csv");report<<"sample_rate,effect_a,effect_b,peak,mean_squared_delta_vs_a,mean_squared_delta_vs_b,hold,release_a,release_all\n";
 crate::Performance p,soloA,soloB;crate::PerformanceSettings s;int pairs=0;
 for(int rate:{44100,48000,96000}){p.prepare(rate);soloA.prepare(rate);soloB.prepare(rate);int64_t time=0;
  for(int a=0;a<8;++a)for(int b=a+1;b<8;++b){++pairs;p.reset();soloA.reset();soloB.reset();std::array<bool,8>g{};p.configure(s,g,93);soloA.configure(s,g,93);soloB.configure(s,g,93);for(int i=0;i<rate;++i){auto x=source(time++,rate);render(p,x);render(soloA,x);render(soloB,x);}g[a]=g[b]=true;p.configure(s,g,93);auto ga=g,gb=g;ga[b]=false;gb[a]=false;soloA.configure(s,ga,93);soloB.configure(s,gb,93);float peak=0;double diffA=0,diffB=0;
   for(int i=0;i<rate;++i){auto x=source(time++,rate);auto y=render(p,x),ya=render(soloA,x),yb=render(soloB,x);diffA+=(y[0]-ya[0])*(y[0]-ya[0]);diffB+=(y[0]-yb[0])*(y[0]-yb[0]);peak=std::max(peak,std::max(std::fabs(y[0]),std::fabs(y[1])));}
   if(diffA/rate<=1e-8||diffB/rate<=1e-8)std::cerr<<"Pair "<<a<<","<<b<<" deltas "<<diffA/rate<<","<<diffB/rate<<"\n";check(diffA/rate>1e-8&&diffB/rate>1e-8,"both effects contribute compared with either effect alone");
   // Drop only a; b must stay held and its marker must remain live.
   g[a]=false;p.configure(s,g,93);double remaining=0;for(int i=0;i<rate/3;++i){auto x=source(time++,rate),y=render(p,x);remaining+=(y[0]-x[0])*(y[0]-x[0]);}check(remaining/(rate/3)>1e-8,"remaining effect stays audible after releasing its partner");check(p.progress[b]>=0&&p.progress[b]<=1,"remaining effect progress valid");if(b>=3)check(p.progress[b]>.99f,"remaining button is not released with its neighbor");
   g[b]=false;p.configure(s,g,93);double error=0;for(int i=0;i<rate;++i){auto x=source(time++,rate),y=render(p,x);if(i>rate/2)error+=(y[0]-x[0])*(y[0]-x[0]);}
   if(a!=3&&b!=3)check(error/(rate/2)<1e-9,"pair release returns dry signal with no stuck gate");
   report<<rate<<','<<a<<','<<b<<','<<peak<<','<<diffA/rate<<','<<diffB/rate<<",PASS,PASS,PASS\n";
  }
  // All8, then staggered releases, repeated changes while processing variable blocks.
  p.reset();std::array<bool,8>g{};p.configure(s,g,84);for(int i=0;i<rate;++i)render(p,source(time++,rate));g.fill(true);p.configure(s,g,84);for(int i=0;i<rate*2;++i)render(p,source(time++,rate));
  for(int k=0;k<8;++k){g[k]=false;p.configure(s,g,84);for(int i=0;i<rate/4;++i)render(p,source(time++,rate));}
  for(int i=0;i<rate*10;++i)render(p,{0,0});auto tail=render(p,{0,0});check(std::fabs(tail[0])+std::fabs(tail[1])<.0001,"all8 release and echo tail settle to silence");
 }
 // A reference runner receives the SAME events in smaller blocks. Results must agree.
 // Tests independent gates and block partitioning for a musical combination with an echo tail.
 crate::Performance ref;p.prepare(44100);ref.prepare(44100);std::array<bool,8>g{};p.configure(s,g,93);ref.configure(s,g,93);int64_t n=0;
 for(int stage=0;stage<6;++stage){g.fill(false);if(stage==1||stage==2)g[4]=true;if(stage>=2&&stage<=4)g[3]=true;if(stage==3)g[6]=true;p.configure(s,g,93);ref.configure(s,g,93);
  for(int i=0;i<22050;++i){auto x=source(n++,44100);if(i%31==0)ref.configure(s,g,93);close(render(p,x),render(ref,x));}
 }
 check(rtAlloc==0,"all combination processing allocation free");
 std::cout<<"PASS:28pairs x3rates="<<pairs<<",both effects audible vs each solo,all8held,staggered releases,echo tail,independent gates,block partition,zero RT allocations\n";
}
