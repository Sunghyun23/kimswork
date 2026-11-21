#define FELT_TINE_VOICING_CALIBRATION 1
#define FELT_WURLI_VOICING_CALIBRATION 1
#define FELT_UPRIGHT_VOICING_CALIBRATION 1
#define FELT_FELT_VOICING_CALIBRATION 1
#define FELT_CLAV_VOICING_CALIBRATION 1
#define FELT_MUTED_VOICING_CALIBRATION 1
#define FELT_THUMB_VOICING_CALIBRATION 1
#include "../src/PhysicalVoice.h"
#include <cassert>
#include <type_traits>
#include <cstdio>
using namespace felt;
template<class...T>struct Unique;
template<>struct Unique<>:std::true_type{};
template<class H,class...T>struct Unique<H,T...>:std::bool_constant<((!std::is_same_v<H,T>)&&...)&&Unique<T...>::value>{};
static_assert(Unique<tine::Mode,wurli::Mode,upright::Mode,feltpiano::Mode,clav::Mode,muted::Mode,thumb::Mode>::value);
static_assert(Unique<tine::Hammer,wurli::Hammer,upright::Hammer,feltpiano::Hammer,clav::Hammer,muted::Hammer,thumb::Hammer>::value);
static DemoVoicing&profile(int m){switch(m){case 0:return tineVoicing;case 1:return wurliVoicing;case 2:return uprightVoicing;case 3:return feltVoicing;case 4:return clavVoicing;case 5:return mutedVoicing;default:return thumbVoicing;}}
static std::array<float,2048>render(int m){Voice v;v.prepare(48000);VoiceSettings s;s.model=m;s.attack=1;v.strike(60,85,0,1,s);std::array<float,2048>x{};for(auto&f:x)f=v.tick();return x;}
int main(){
 std::array<std::array<float,2048>,7>original{};
 for(int m=0;m<7;++m)original[m]=render(m);
 for(int changed=0;changed<7;++changed){auto saved=profile(changed);profile(changed).level=1.7f;profile(changed).gap=.8f;profile(changed).upper=1.2f;
  for(int m=0;m<7;++m){auto x=render(m);assert((x==original[m])==(m!=changed));}
  profile(changed)=saved;
 }
 std::puts("PASS7profile mutations: only the targeted engine changes; other6outputs bit-identical");
 for(int a=0;a<7;++a)for(int b=0;b<7;++b)if(a!=b){Voice reference,interleaved,other;for(auto*v:{&reference,&interleaved,&other})v->prepare(48000);VoiceSettings s;s.model=a;reference.strike(60,85,1,2,s);interleaved.strike(60,85,1,2,s);s.model=b;other.strike(43,110,7,3,s);
  for(int i=0;i<4096;++i){if(i==512){other.setTone(.1f);other.setEnvelope(1,100,.4,50);other.retune(75,30);}if(i==1500)other.release(false);other.updatePitch(1.002f);other.tick();assert(reference.tick()==interleaved.tick());}
 }
 std::puts("PASS42engine pairs: interleaved excitation, filter, envelope, glide and release do not affect another engine");
 {UprightBody a,b;FeltBody other;a.prepare(48000);b.prepare(48000);other.prepare(48000);for(int i=0;i<4096;++i){other.process(i%31==0?.8f:0,true);assert(a.process(i==0?.5f:0,true)==b.process(i==0?.5f:0,true));}}
 std::puts("PASS upright and felt soundboard/sympathetic state is isolated");
 // Reuse a voice across all models and rates: the selected backend must be
 // correctly prepared, keep its own identity and discard a stolen old voice.
 for(float rate:{44100.f,48000.f,96000.f}){Voice reused;reused.prepare(rate);for(int m=0;m<7;++m){Voice fresh;fresh.prepare(rate);VoiceSettings s;s.model=m;reused.strike(55,90,m,5,s);fresh.strike(55,90,m,5,s);assert(reused.model()==m);for(int i=0;i<1024;++i)assert(reused.tick()==fresh.tick());reused.stop();assert(!reused.state().active&&reused.tick()==0);}}
 std::puts("PASS voice reuse across7backends/3rates matches fresh instances and resets silently");
}
