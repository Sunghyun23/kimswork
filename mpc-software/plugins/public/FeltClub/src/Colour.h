#pragma once
#include "dsp/delay_line.h"
#include "dsp/smoother.h"
#include "dsp/lfo.h"
#include "dsp/math.h"
#include "TextureNoise.h"
#include <array>
#include <cmath>
namespace felt {
struct Comb {
 mpc::dsp::DelayLine line;float filter=0;int delay=1;
 void prepare(int samples){delay=samples;line.prepare(samples+2);filter=0;}
 void reset(){line.reset();filter=0;}
 float process(float x,float feedback,float damping){float y=line.readInt(delay);filter+=(1-damping)*(y-filter);if(std::abs(filter)<1e-25f)filter=0;float next=x+filter*feedback;line.write(std::abs(next)<1e-25f?0:next);return y;}
};
struct ColourSettings {float hiss=0,motionDelay=0,motionFade=0,driftHz=.24f,chorusHz=.31f,chorusDepth=.5f;float drive=0,chorus=0,delay=0,room=0,width=.7f,motionDepth=0,motionHz=2,drift=.06f,delayFeedback=.28f,delayBeats=.75f,roomTime=.7f,roomDamp=.5f;int motionMode=1,shape=0;};
struct Colour {
 float heldTime=0;bool wasHeld=false,echoRunning=false,roomRunning=false;
 float dcPole=.99715f;
 float sr=44100,phase=0,chorusPhase=0,driftPhase=0,delaySamples=13230;
 mpc::dsp::LFO motion;TextureNoise hiss;
 std::array<mpc::dsp::DelayLine,2> chorus,echo;
 std::array<Comb,8> room;
 std::array<mpc::dsp::Smoother,9> smooth;
 std::array<float,2> echoLp{},lastIn{},dc{};
 ColourSettings settings{};
 void prepare(float sampleRate){
  sr=sampleRate;hiss.prepare(sr);dcPole=std::exp(-2*3.14159265358979323846f*20/sr);motion.setSampleRate(sr);for(auto&d:chorus)d.prepare(int(sr*.06));for(auto&d:echo)d.prepare(int(sr*4));
  const int delays[8]={1499,1601,1747,1867,1553,1669,1789,1907};for(int i=0;i<8;++i)room[i].prepare(int(delays[i]*sr/44100));
  for(auto&s:smooth)s.setTime(25,sr);reset();
 }
 void reset(){echoRunning=roomRunning=false;hiss.reset();for(auto&d:chorus)d.reset();for(auto&d:echo)d.reset();for(auto&c:room)c.reset();for(auto&s:smooth)s.snap(0);smooth[4].snap(.7f);echoLp={};lastIn={};dc={};phase=chorusPhase=driftPhase=0;heldTime=0;wasHeld=false;motion.reset();}
 void update(const ColourSettings&s,double bpm){settings=s;hiss.set(s.hiss);const float v[]={s.drive,s.chorus,s.delay,s.room,s.width,s.motionDepth,s.delayFeedback,s.roomTime,s.roomDamp};for(int i=0;i<9;++i)smooth[i].setTarget(v[i]);motion.setFrequency(s.motionHz);motion.setShape(s.shape);delaySamples=std::min(sr*3.95f,float(60/std::max(30.,bpm))*s.delayBeats*sr);}
 void gate(bool held){if(held&&!wasHeld)heldTime=0;wasHeld=held;}
 float pitchRatio(){const float x=mpc::dsp::fastSin01(driftPhase)+.23f*mpc::dsp::fastSin01(driftPhase*3.73f);return 1+x*settings.drift*.004f;}
 std::array<float,2> process(float left,float right){
  std::array<float,2> x={left,right};float v[9];for(int i=0;i<9;++i)v[i]=smooth[i].process();
  float mid=(left+right)*.5f,side=(left-right)*v[4]*.5f;x={mid+side,mid-side};
  heldTime+=1/sr;float fade=std::min(1.f,std::max(0.f,(heldTime-settings.motionDelay*.001f)/std::max(.001f,settings.motionFade*.001f)));
  v[5]*=fade;const float m=motion.process();
  for(int ch=0;ch<2;++ch){
   if(settings.motionMode==0)x[ch]*=1-v[5]*(.5f-.5f*m);
   else x[ch]*=1+v[5]*m*(ch?1.f:-1.f);
   if(v[0]>.00001f){float drive=1+v[0]*4;x[ch]=mpc::dsp::fastTanh(x[ch]*drive)/(1+v[0]*1.6f);}
   // Remove pickup/drive DC without changing a note's envelope.
   const float filtered=x[ch]-lastIn[ch]+dcPole*dc[ch];lastIn[ch]=x[ch];dc[ch]=std::abs(filtered)<1e-25f?0:filtered;x[ch]=dc[ch];
   if(v[1]>0){const float lfo=mpc::dsp::fastSin01(chorusPhase+(ch?.27f:0));const float wet=chorus[ch].readLinear(sr*(.012f+.006f*settings.chorusDepth*lfo));chorus[ch].write(x[ch]);x[ch]=x[ch]*(1-v[1]*.35f)+wet*v[1]*.35f;}
   else chorus[ch].write(x[ch]); // Preserve future chorus history while dry.
   echoRunning|=v[2]>0;
   if(echoRunning){const float delayed=echo[ch].readLinear(delaySamples*(ch?1.013f:1));echoLp[ch]+=.3f*(delayed-echoLp[ch]);if(std::abs(echoLp[ch])<1e-25f)echoLp[ch]=0;echo[ch].write(x[ch]*v[2]+echoLp[ch]*std::min(.72f,v[6]));x[ch]+=delayed*.55f;}

  }
  const float send=(x[0]+x[1])*.16f*v[3];
  roomRunning|=v[3]>0;
  if(roomRunning)for(int ch=0;ch<2;++ch){float reverb=0;for(int j=0;j<4;++j)reverb+=room[ch*4+j].process(send,.50f+v[7]*.35f,.2f+v[8]*.6f);x[ch]+=reverb*.23f;}
  chorusPhase+=settings.chorusHz/sr;if(chorusPhase>=1)chorusPhase-=1;driftPhase+=settings.driftHz/sr;if(driftPhase>=1)driftPhase-=1;
  const auto floor=hiss.process();x[0]+=floor[0];x[1]+=floor[1];
  return x;
 }
};
}
