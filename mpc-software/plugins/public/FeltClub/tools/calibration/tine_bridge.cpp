#define FELT_TINE_CALIBRATION 1
#include "../../src/PhysicalVoice.h"
extern "C" void fc_tine(const float*values,int note,int velocity,int frames,float*out){
 felt::tineProfile={values[0],values[1],values[2],values[3],values[4],values[5],values[6],values[7],values[8],values[9],values[10],values[11]};
 felt::Voice v;v.prepare(44100);felt::VoiceSettings s;s.model=0;s.tone=.50;s.body=.60;s.character3=.32;s.character4=.25;s.attack=1;s.decay=1000;s.sustain=.90;s.release=350;s.hammer=.35;s.velocity=.65;s.resonance=.35;s.detune=1.5;
 v.strike(note,velocity,0,1,s);
 for(int i=0;i<frames;++i){if((i&31)==0)v.updatePitch(1);out[i]=v.tick();}
}
