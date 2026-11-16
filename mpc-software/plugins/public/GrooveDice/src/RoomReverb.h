// Original short-room algorithm. Four loss-damped, Hadamard-mixed delay lines
// with stereo allpass input diffusion. General FDN theory: Julius O. Smith,
// Physical Audio Signal Processing / Choice of Lossless Feedback Matrix.
#pragma once
#include <array>
#include <vector>
#include <cmath>
#include <algorithm>
namespace groove {
class RoomReverb {
 struct Delay{std::vector<float>data;size_t at=0;void prepare(int n){data.assign(std::max(1,n),0);at=0;}void reset(){std::fill(data.begin(),data.end(),0);at=0;}};
 std::array<Delay,4>tank,diffusion;std::array<float,4>feedback{},damped{};float damping=.3f;int remaining=0,tailFrames=1;
 static float clean(float x){return std::abs(x)<1.e-18f?0.f:x;}
 static float allpass(Delay&d,float x){float y=clean(d.data[d.at]-.55f*x);d.data[d.at]=clean(x+.55f*y);if(++d.at==d.data.size())d.at=0;return y;}
public:
 void prepare(double rate){rate=std::clamp(rate,8000.,192000.);static const int lengths[]={911,1237,1511,1867},diff[]={149,211,173,239};for(int i=0;i<4;++i){int n=int(std::lround(lengths[i]*rate/44100.));tank[i].prepare(n);diffusion[i].prepare(int(std::lround(diff[i]*rate/44100.)));feedback[i]=float(std::pow(10.,-3.*n/(rate*.45)));}damping=float(1.-std::exp(-6.283185307*4200./rate));tailFrames=int(rate*1.5);reset();}
 void reset(){for(auto&d:tank)d.reset();for(auto&d:diffusion)d.reset();damped.fill(0);remaining=0;}
 void process(float left,float right,float&wetL,float&wetR){
  if(tank[0].data.empty()){wetL=wetR=0;return;}
  if(std::abs(left)+std::abs(right)>1.e-9f)remaining=tailFrames;else if(remaining>0)--remaining;else{wetL=wetR=0;return;}
  left=allpass(diffusion[1],allpass(diffusion[0],left));right=allpass(diffusion[3],allpass(diffusion[2],right));
  float d[4];for(int i=0;i<4;++i)d[i]=tank[i].data[tank[i].at];
  wetL=.5f*(d[0]+d[1]-d[2]-d[3]);wetR=.5f*(d[0]-d[1]+d[2]-d[3]);
  float mixed[]={.5f*(d[0]+d[1]+d[2]+d[3]),.5f*(d[0]-d[1]+d[2]-d[3]),.5f*(d[0]+d[1]-d[2]-d[3]),.5f*(d[0]-d[1]-d[2]+d[3])};
  float input[]={.2f*(left+right),.2f*(left-right),.2f*(-left+right),-.2f*(left+right)};
  for(int i=0;i<4;++i){damped[i]=clean(damped[i]+damping*(mixed[i]-damped[i]));auto&t=tank[i];t.data[t.at]=clean(input[i]+feedback[i]*damped[i]);if(++t.at==t.data.size())t.at=0;}
 }
};
}
