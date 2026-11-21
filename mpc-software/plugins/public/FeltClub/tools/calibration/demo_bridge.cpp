#define FELT_TINE_VOICING_CALIBRATION 1
#define FELT_WURLI_VOICING_CALIBRATION 1
#define FELT_UPRIGHT_VOICING_CALIBRATION 1
#define FELT_FELT_VOICING_CALIBRATION 1
#define FELT_CLAV_VOICING_CALIBRATION 1
#define FELT_MUTED_VOICING_CALIBRATION 1
#define FELT_THUMB_VOICING_CALIBRATION 1
#include "../../src/PhysicalVoice.h"
extern "C" void fc_demo(const float*p,int model,int note,int velocity,int frames,float tuning,float*out){
 felt::DemoVoicing profile{p[0],p[1],p[2],1,p[3]};if(model==0)felt::tineVoicing=profile;else if(model==1)felt::wurliVoicing=profile;else if(model==2)felt::uprightVoicing=profile;else if(model==3)felt::feltVoicing=profile;else if(model==4)felt::clavVoicing=profile;else if(model==5)felt::mutedVoicing=profile;else if(model==6)felt::thumbVoicing=profile;
 felt::Voice v;v.prepare(44100);felt::VoiceSettings s;s.model=model;s.tone=.50;s.body=.60;s.character3=.32;s.character4=model==5?.65:.25;s.attack=1;s.decay=1000;s.sustain=.90;s.release=350;s.hammer=.35;s.velocity=.65;s.resonance=.35;s.detune=1.5;v.strike(note,velocity,0,1,s,tuning);
 for(int i=0;i<frames;++i){if((i&31)==0)v.updatePitch(1);out[i]=v.tick();}
}
