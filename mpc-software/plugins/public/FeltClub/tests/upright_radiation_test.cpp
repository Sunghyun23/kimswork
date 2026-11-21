#include "../src/engines/upright/UprightVoice.h"
#include "../src/engines/upright/UprightBody.h"
#include <cassert>
#include <cmath>
#include <cstdio>
int main(){double peak=0;int cases=0;
 for(float rate:{44100.f,48000.f,96000.f})for(int key=21;key<=108;++key)for(int velocity:{35,127}){
  felt::UprightVoice voice;felt::UprightBody body;voice.prepare(rate);body.prepare(rate);felt::VoiceSettings s;s.model=2;s.attack=1;s.tone=.9f;s.hammer=.85f;s.body=.9f;s.resonance=.8f;s.detune=5;voice.strike(key,velocity,0,1,s);double energy=0;
  for(int i=0;i<int(rate*.08f);++i){if((i&31)==0)voice.updatePitch(1);float x=voice.tick();auto y=body.render(voice.bodySend,true,voice.bodySend*voice.soundboardProfile,voice.bridgeDetail,{x,x});for(float a:y){assert(std::isfinite(a)&&std::abs(a)<32);peak=std::max(peak,double(std::abs(a)));energy+=a*a;}}
  assert(energy>1e-20);++cases;
 }
 // A middle-register voice can use the extra soundboard inputs while a
 // simultaneous bass voice retains its own drive and radiation.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::UprightBody mixed,bass,mid;
  for(auto* plate:{&mixed,&bass,&mid})plate->prepare(rate);
  double sideEnergy=0;
  for(int n=0;n<int(rate*.045f);++n){
   const float first=n==0?.1f:0.f;
   const float second=n==300?.08f:0.f;
   const float difference=n==300?.06f:0.f;
   const auto a=mixed.process(first+second,false,second,difference);
   const auto b=bass.process(first,false);
   const auto c=mid.process(second,false,second,difference);
   for(int channel=0;channel<2;++channel){
    assert(std::abs(a[channel]-b[channel]-c[channel])<1e-6f);
   }
   sideEnergy+=(c[0]-c[1])*(c[0]-c[1]);
  }
  assert(sideEnergy>1e-7);
  mid.reset();
  const auto silent=mid.process(0,false);
  assert(silent[0]==0&&silent[1]==0);
 }
 std::puts("PASS Upright focused bridge and differential inputs mix linearly and reset silently at three rates");
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::UprightBody body;body.prepare(rate);
  double before=0,after=0;
  for(int i=0;i<int(rate*.15f);++i){
   const float note=.25f*std::sin(float(i)*felt::Pi*660.f/rate);
   const auto pressure=body.render(0,false,0,0,{note,0});
   assert(pressure[1]==0);
   if(i>=int(rate*.05f)){
    before+=double(note)*note;
    after+=double(pressure[0])*pressure[0];
   }
  }
  assert(std::abs(10*std::log10(after/before)+1.8)<.02);
  body.reset();
  const auto silent=body.render(0,false,0,0,{});
  assert(silent[0]==0&&silent[1]==0);
 }
 std::puts("PASS Upright direct output has the approved 330 Hz dip and silent reset at three rates");
 std::printf("PASS%dUpright full-radiation88-key/velocity/rate cases; finite nonzero, peak%.6f before host gain/limiter\n",cases,peak);
}
