#include "../src/TextureNoise.h"
#include <cassert>
#include <cmath>
#include <iostream>
int main(){double rms[3]{};int rateIndex=0;
 for(float sr:{44100.f,48000.f,96000.f}){
  felt::TextureNoise silent,soft,loud,copy;for(auto*n:{&silent,&soft,&loud,&copy})n->prepare(sr);
  soft.set(.1f);loud.set(1);copy.set(1);double energy=0,small=0,mean=0,side=0;int count=0;
  for(int frame=0;frame<int(sr*2);++frame){auto zero=silent.process(),a=soft.process(),b=loud.process(),c=copy.process();assert(zero[0]==0&&zero[1]==0&&b==c);for(int ch=0;ch<2;++ch){assert(std::isfinite(b[ch])&&std::abs(b[ch])<.1f);if(frame>=int(sr)){energy+=double(b[ch])*b[ch];small+=double(a[ch])*a[ch];mean+=b[ch];++count;}}if(frame>=int(sr))side+=double(b[0]-b[1])*(b[0]-b[1]);}
  assert(energy>0&&small>0&&side>0&&energy/small>99&&energy/small<101);assert(std::abs(mean/count)<std::sqrt(energy/count)*.03);rms[rateIndex++]=std::sqrt(energy/count);
  loud.set(0);for(int frame=0;frame<int(sr*.5f);++frame)loud.process();for(int i=0;i<1024;++i){auto x=loud.process();assert(x[0]==0&&x[1]==0);}
  copy.reset();copy.set(1);felt::TextureNoise fresh;fresh.prepare(sr);fresh.set(1);for(int i=0;i<8192;++i)assert(copy.process()==fresh.process());
 }
 assert(*std::max_element(rms,rms+3)/ *std::min_element(rms,rms+3)<1.2);
 std::cout<<"PASS tape-like hiss3rates: exact Off, continuous no-input output, stereo/DC/level bounds, sample-rate density, deterministic reset, smooth silent fade\n";
}
