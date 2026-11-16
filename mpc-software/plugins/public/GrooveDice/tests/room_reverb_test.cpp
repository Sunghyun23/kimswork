#include "../src/RoomReverb.h"
#include <cassert>
#include <iostream>
#include <cstdlib>
static thread_local bool guarded=false;
void*operator new(size_t n){if(guarded)std::abort();if(auto*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}
int main(){for(double rate:{44100.,48000.,96000.}){groove::RoomReverb room;room.prepare(rate);int first=-1,active=0;double early=0,late=0,ll=0,rr=0,lr=0;guarded=true;for(int n=0;n<int(2*rate);++n){float l,r;room.process(n==0?1.f:0.f,0,l,r);assert(std::isfinite(l)&&std::isfinite(r)&&std::abs(l)<4&&std::abs(r)<4);double energy=l*l+r*r;if(std::abs(l)+std::abs(r)>1.e-9f){if(first<0)first=n;++active;}if(n<rate*.3)early+=energy;if(n>rate)late+=energy;if(n>rate*.05&&n<rate*.5){ll+=l*l;rr+=r*r;lr+=l*r;}}guarded=false;assert(first>rate*.01&&first<rate*.06);assert(active>rate*.15&&early>1.e-5&&late<early*1.e-5);assert(ll>0&&rr>0&&std::abs(lr/std::sqrt(ll*rr))<.98);room.reset();for(int n=0;n<200;++n){float l,r;room.process(0,0,l,r);assert(l==0&&r==0);}std::cout<<"PASS diffuse stereo room @"<<rate<<": first="<<first<<" active="<<active<<" tail ratio="<<late/early<<" correlation="<<lr/std::sqrt(ll*rr)<<"; finite/reset/no RT allocations\n";}
 groove::RoomReverb left,right;left.prepare(44100);right.prepare(44100);for(int n=0;n<44100;++n){float a,b,c,d;left.process(n==0?1:0,0,a,b);right.process(0,0,c,d);assert(c==0&&d==0);}std::cout<<"PASS independent routed room instances do not leak into silent buses\n";
}
