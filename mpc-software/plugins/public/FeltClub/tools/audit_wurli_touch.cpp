// Dense native Wurli touch audit: velocities1..127 at6keys; no audio gain matching.
#include "mpc_plugin.h"
#include "../src/Parameters.h"
#include <dlfcn.h>
#include <filesystem>
#include <fstream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <iostream>
#include <cassert>
static VstTimeInfo timeInfo{};
static VstIntPtr host(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){return op==audioMasterVersion?2400:op==audioMasterGetTime?reinterpret_cast<VstIntPtr>(&timeInfo):0;}
struct Event{int64_t frame;int note,vel;bool on;};

int main(int argc,char**argv){assert(argc==3);void*lib=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);if(!lib){std::cerr<<dlerror()<<'\n';return 1;}auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(lib,"VSTPluginMain"));assert(entry);std::ofstream csv(argv[2]);csv<<"note,velocity,body_dbfs,h2_h1_db,h3_h1_db,adjacent_cosine,release_dbfs\n";std::vector<float>previous;
 const char*models[]={"tine","wurli","upright","felt","clav","muted","thumb"};
 for(int trial=0;trial<6*127;++trial){AEffect*e=entry(host);assert(e&&e->numParams==kParamCount);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,48000);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);e->dispatcher(e,effStartProcess,0,0,nullptr,0);
  struct Decl:mpc::Plugin{Decl():mpc::Plugin({}){declareParameters(*this);}void process(mpc::ProcessContext&)override{}}decl;
  auto set=[&](int k,float v){e->setParameter(e,k,decl.param(k).toNormalised(v));};
  constexpr int keys[]={36,48,59,60,72,84};int model=1,pitch=keys[trial/127],velocity=1+trial%127;set(kInputMode,0);set(kModel,model);set(kVelocityCurve,1);set(kMaster,-10);set(kTone,60);set(kBody,62);set(kCharacter3,36);set(kCharacter4,20);set(kHammer,32);set(kVelocity,65);set(kDetune,1.2);set(kResonance,30);set(kNoise,0);set(kKeyOff,0);set(kAttack,1);set(kDecay,1800);set(kSustain,92);set(kRelease,720);for(int k:{kDepth,kDrift,kDrive,kChorus,kDelay,kRoom})set(k,0);set(kWidth,100);set(kPedal,0);
  std::vector<Event>events;auto note=[&](double at,double duration,int pitch,int vel){events.push_back({int64_t(at*48000),pitch,vel,true});events.push_back({int64_t((at+duration)*48000),pitch,0,false});};
  note(.01,.30,pitch,velocity);
  std::sort(events.begin(),events.end(),[](auto&a,auto&b){return a.frame<b.frame;});
  int64_t total=int64_t(.60*48000);std::vector<float>audio(total*2);size_t next=0;float data[2][128]{},*out[]={data[0],data[1]};
  timeInfo={};timeInfo.sampleRate=48000;timeInfo.tempo=88;timeInfo.timeSigNumerator=timeInfo.timeSigDenominator=4;timeInfo.flags=kVstTempoValid|kVstPpqPosValid|kVstTimeSigValid|kVstTransportPlaying;
  for(int64_t pos=0;pos<total;pos+=128){int frames=std::min<int64_t>(128,total-pos);VstMidiEvent midi[32]{};struct Packet{VstInt32 count=0;VstIntPtr reserved=0;VstEvent*events[32]{};}packet;
   while(next<events.size()&&events[next].frame<pos+frames){auto&v=events[next++];assert(packet.count<32);auto&m=midi[packet.count];m.type=kVstMidiType;m.byteSize=sizeof(m);m.deltaFrames=v.frame-pos;m.midiData[0]=v.on?0x90:0x80;m.midiData[1]=v.note;m.midiData[2]=v.vel;packet.events[packet.count++]=reinterpret_cast<VstEvent*>(&m);}
   if(packet.count)e->dispatcher(e,effProcessEvents,0,0,&packet,0);timeInfo.samplePos=pos;timeInfo.ppqPos=pos*88./(60*48000);e->processReplacing(e,nullptr,out,frames);
   for(int i=0;i<frames;++i)for(int c=0;c<2;++c){float v=data[c][i];assert(std::isfinite(v));audio[(pos+i)*2+c]=v;}
  }
  constexpr int first=1440,last=12000;std::vector<float>body;double energy=0,dot=0,oldEnergy=0,release=0;
  for(int i=first;i<last;++i){float x=(audio[i*2]+audio[i*2+1])*.5f;body.push_back(x);energy+=x*x;if(velocity>1){dot+=double(x)*previous[i-first];oldEnergy+=double(previous[i-first])*previous[i-first];}}
  auto harmonic=[&](int h){double coef=2*std::cos(2*3.14159265358979323846*(440*std::exp2((pitch-69)/12.))*h/48000),s1=0,s2=0;for(size_t i=0;i<body.size();++i){double w=.5-.5*std::cos(2*3.14159265358979323846*i/(body.size()-1));double y=body[i]*w+coef*s1-s2;s2=s1;s1=y;}return std::max(1e-30,s1*s1+s2*s2-coef*s1*s2);};
  double fundamental=harmonic(1);for(int i=19200;i<26400;++i){double x=(audio[i*2]+audio[i*2+1])*.5;release+=x*x;}
  csv<<pitch<<","<<velocity<<","<<10*std::log10(energy/body.size()+1e-30)<<","<<10*std::log10(harmonic(2)/fundamental)<<","<<10*std::log10(harmonic(3)/fundamental)<<","<<(velocity>1?dot/std::sqrt(energy*oldEnergy+1e-30):1)<<","<<10*std::log10(release/7200+1e-30)<<"\n";previous=std::move(body);
  e->dispatcher(e,effStopProcess,0,0,nullptr,0);e->dispatcher(e,effMainsChanged,0,0,nullptr,0);e->dispatcher(e,effClose,0,0,nullptr,0);
 }dlclose(lib);
}
