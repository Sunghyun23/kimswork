#include "../src/engines/wurli/WurliVoice.h"
#include <cassert>
#include <cstdio>
#include <complex>
int main(){
 felt::WurliPickup p;p.configure(.001f,.3f,48000);double minC=2,maxC=-1;
 for(int i=0;i<=28800;++i){double z=-36+i*.0025,c=p.field(z);assert(std::isfinite(c)&&c>=-1e-6&&c<=1.000001);minC=std::min(minC,c);maxC=std::max(maxC,c);}
 assert(maxC-minC>.99);for(int i=1;i<288;++i){double z=-36+i*.25,h=1e-5;double left=(p.field(z)-p.field(z-h))/h,right=(p.field(z+h)-p.field(z))/h;assert(std::abs(left-right)<.0002);}
 std::puts("PASS bounded geometric capacitance and continuous interpolated slopes");
 {felt::WurliPickup bass,treble;bass.configure(.001f,.3f,48000,false);treble.configure(.001f,.3f,48000,true);for(int i=-100;i<=100;++i){float x=i*.00001f;assert(bass.capacitance(x)==treble.capacitance(-x));}}
 std::puts("PASS opposite solder orientations mirror the geometric readout");
 double maxField=0,maxPeak=0;int maxKey=0;float maxRate=0;
 for(float sr:{44100.f,48000.f,96000.f})for(int key=21;key<=108;++key)for(int vel:{32,80,127})for(float bark:{0.f,1.f})for(float hammer:{0.f,1.f})for(float reed:{0.f,1.f})for(float body:{0.f,1.f}){
  felt::WurliVoice v;v.prepare(sr);felt::VoiceSettings s;s.model=1;s.attack=1;s.character4=bark;s.hammer=hammer;s.character3=reed;s.body=body;v.strike(key,vel,0,1,s);double e=0;
  for(int i=0;i<int(sr*.06f);++i){float x=v.tick();assert(std::isfinite(x)&&std::abs(x)<32);e+=x*x;maxPeak=std::max(maxPeak,double(std::abs(x)));}
  assert(e>1e-20);if(v.wurliPickup.maxExcursion>maxField){maxField=v.wurliPickup.maxExcursion;maxKey=key;maxRate=sr;}
 }
 // The finite lookup boundary must not be reached in this stated stress grid.
 std::fprintf(stderr,"field max %.3f key%d rate%.0f\n",maxField,maxKey,maxRate);assert(maxField<35.5);std::printf("PASS88keys/3rates/3velocities/Bark+Hammer+Reed+Body extremes stay inside field grid:%.3f; raw peak%.3f\n",maxField,maxPeak);
 // Verify the component-derived input approximation against its own analog
 // transfer, without treating it as a measured full transistor preamplifier.
 for(float rate:{44100.f,48000.f,96000.f})for(double hz:{20.,100.,1000.,5000.}){
  felt::WurliInputCircuit circuit;circuit.prepare(4*rate);double in=0,out=0;int n=int(rate*4*.8);
  for(int i=0;i<n;++i){double x=std::sin(2*3.14159265358979323846*hz*i/(4*rate)),y=circuit.process(x);if(i>=int(rate*4*.6)){in+=x*x;out+=y*y;}}
  std::complex<double>z(0,2*3.14159265358979323846*hz);constexpr double R=470000,Rs=22000,Cc=22e-9,Cp=220e-12;
  auto h=z*(R*Cc)/(1.+z*(R*Cp+(Rs+R)*Cc)+z*z*(R*Cp*Rs*Cc))/(1.+z*(150000*100e-12));double error=20*std::log10(std::sqrt(out/in)/std::abs(h));assert(std::abs(error)<.3);
  circuit.reset();assert(circuit.process(0)==0);for(int i=0;i<int(rate*4);++i)circuit.process(1);assert(std::abs(circuit.process(1))<1e-10);
 }
 std::puts("PASS component-derived input transfer within0.3dB across3rates, DC rejection and silent reset");
 double previousSlope=0;for(int key:{58,59,60,61,62}){felt::WurliVoice v;v.prepare(48000);felt::VoiceSettings s;s.model=1;v.strike(key,80,0,1,s);double slope=v.wurliPickup.capacitance(.000001)-v.wurliPickup.capacitance(-.000001);assert(std::abs(slope)>1e-8);if(previousSlope!=0)assert(previousSlope*slope>0);previousSlope=slope;}
 std::puts("PASS canonical reed coordinates have no arbitrary MIDI60direction switch");
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::WurliVoice v;v.prepare(rate);felt::VoiceSettings s;s.model=1;s.attack=1;s.release=200;v.strike(60,100,0,1,s);for(int i=0;i<int(rate*.1);++i)v.tick();v.release(true);assert(v.sustained&&v.modes[0].damperTarget==1);v.release(false);float damp=v.modes[0].damping;for(int i=0;i<int(rate*.05);++i){v.tick();assert(v.modes[0].damping<=damp+1e-7);damp=v.modes[0].damping;}for(int i=0;i<int(rate*.5);++i)v.tick();assert(!v.active);
 }
 std::puts("PASS Wurli pedal preserves vibration and released felt contact damps progressively to silence at3rates");

}
