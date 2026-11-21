#include "../../src/engines/upright/UprightVoice.h"
#include "../../src/engines/upright/UprightBody.h"
#include <cstdio>
int main(){puts("[");int row=0;for(int key:{36,48,60,72,84})for(int vel:{35,65,95})for(bool pedal:{false,true}){
 felt::UprightVoice v;felt::UprightBody b;v.prepare(48000);b.prepare(48000);felt::VoiceSettings s;s.model=2;s.attack=1;s.decay=1800;s.sustain=.92;s.release=720;s.tone=.60;s.body=.62;s.character3=.36;s.character4=.20;s.hammer=.32;s.velocity=.65;s.detune=1.2;s.resonance=.30;v.strike(key,vel,0,1,s);
 double direct=0,body=0,peak=0,forcePeak=0;for(int i=0;i<48000;++i){float x=v.tick();auto y=b.process(v.bodySend,pedal);double l=x*(.707106-.35*v.pan),r=x*(.707106+.35*v.pan);direct+=l*l+r*r;body+=y[0]*y[0]+y[1]*y[1];peak=std::max(peak,double(std::abs(x)));forcePeak=std::max(forcePeak,double(v.hammer.lastForce));}
 printf("%s{\"note\":%d,\"velocity\":%d,\"pedal\":%s,\"contact_ms\":%.6f,\"force_peak_N\":%.6f,\"body_to_direct_db\":%.6f,\"raw_peak\":%.8f,\"modes\":%d}",row++?",\n":"",key,vel,pedal?"true":"false",v.hammer.age*1000,forcePeak,10*std::log10(body/direct),peak,v.count);
 }puts("\n]");}
