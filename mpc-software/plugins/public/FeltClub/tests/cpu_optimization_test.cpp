#include "../src/PhysicalVoice.h"
#include "../src/Colour.h"
#include <cassert>
#include <iostream>
// Virgin-body fast path must preserve the pedal history before first sound.
template<class Body>void bodyWake(float rate){
 Body idle,reference;idle.prepare(rate);reference.prepare(rate);reference.sleeping=false;
 for(int n=0;n<20000;++n){bool pedal=(n>=100&&n<1500)||(n>=5000&&n<11000);float input=n==2000||n==8000?.001f:0;
  if(n==1000){idle.setTuning(432);reference.setTuning(432);}
  const auto a=idle.render(input,pedal,input*.1f,input*.05f,{}),b=reference.render(input,pedal,input*.1f,input*.05f,{});assert(a==b);
 }
 idle.reset();assert(idle.sleeping);for(int i=0;i<1024;++i)assert((idle.render(0,false,0,0,{})==std::array<float,2>{}));
}
template<class Mode>void pitchCache(float rate){
 Mode cached,reference;cached.configure(220,1.2f,rate,false,0);reference.configure(220,1.2f,rate,false,0);cached.drive=reference.drive=.001f;cached.output=reference.output=1;
 for(float ratio:{1.f,1.f,.8f,.8f,2.f,2.f,1.f}){cached.pitch(ratio);reference.lastPitch=-1;reference.pitch(ratio);assert(cached.c==reference.c&&cached.rr==reference.rr);for(int i=0;i<10;++i)assert(cached.tick(i==0?.0001f:0)==reference.tick(i==0?.0001f:0));}
 cached.configure(330,1.8f,rate,false,0);reference.configure(330,1.8f,rate,false,0);cached.pitch(1);reference.pitch(1);assert(cached.c==reference.c);
}
int main(){
 for(float sr:{44100.f,48000.f,96000.f}){bodyWake<felt::UprightBody>(sr);bodyWake<felt::FeltBody>(sr);pitchCache<felt::tine::Mode>(sr);pitchCache<felt::wurli::Mode>(sr);pitchCache<felt::upright::Mode>(sr);pitchCache<felt::feltpiano::Mode>(sr);pitchCache<felt::clav::Mode>(sr);pitchCache<felt::muted::Mode>(sr);pitchCache<felt::thumb::Mode>(sr);}
 std::cout<<"PASS silent-body wake preserves exact pedal/tuning history; all7pitch caches invalidate correctly at3rates\n";
 // Chorus must keep recording while bypassed. Delay/room may sleep only
 // before first use, and must continue rendering their tails after mix off.
 for(float sr:{44100.f,48000.f,96000.f}){felt::Colour a,b;a.prepare(sr);b.prepare(sr);b.echoRunning=b.roomRunning=true;felt::ColourSettings s;s.chorus=s.delay=s.room=s.drive=s.motionDepth=s.drift=0;
  for(int n=0;n<30000;++n){if(n==2048)s.chorus=s.delay=s.room=.25f;if(n==10000)s.chorus=s.delay=s.room=0;if(n==18000)s.chorus=s.delay=s.room=.3f;if(n%32==0){a.update(s,88);b.update(s,88);}float x=(n%997==0)?.1f:0;assert(a.process(x,-x*.3f)==b.process(x,-x*.3f));}
 }
 std::cout<<"PASS dry Chorus history, delayed FX enable and off/on tails remain sample-identical at3rates\n";
}
