// Isolated Tine velocity audit: fresh native instance per note; no FX or pedal.
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
static void wav(const std::string&file,const std::vector<float>&x){std::ofstream f(file,std::ios::binary);auto u16=[&](uint16_t v){f.put(v&255);f.put(v>>8);};auto u32=[&](uint32_t v){u16(v&65535);u16(v>>16);};f.write("RIFF",4);u32(36+x.size()*2);f.write("WAVEfmt ",8);u32(16);u16(1);u16(2);u32(48000);u32(192000);u16(4);u16(16);f.write("data",4);u32(x.size()*2);for(float v:x)u16(uint16_t(int16_t(std::lround(std::max(-1.f,std::min(1.f,v))*32767))));}
int main(int argc,char**argv){assert(argc==4||argc==5);const int requestedModel=argc==5?std::stoi(argv[4]):0;assert(requestedModel>=0&&requestedModel<7);void*lib=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);if(!lib){std::cerr<<dlerror()<<'\n';return 1;}auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(lib,"VSTPluginMain"));assert(entry);std::filesystem::create_directories(argv[2]);
 const char*models[]={"tine","wurli","upright","felt","clav","muted","thumb"};
 for(int trial=0;trial<9;++trial){AEffect*e=entry(host);assert(e&&e->numParams==kParamCount);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,48000);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);e->dispatcher(e,effStartProcess,0,0,nullptr,0);
  struct Decl:mpc::Plugin{Decl():mpc::Plugin({}){declareParameters(*this);}void process(mpc::ProcessContext&)override{}}decl;
  auto set=[&](int k,float v){e->setParameter(e,k,decl.param(k).toNormalised(v));};
  int model=requestedModel,pitch=48+(trial/3)*12,velocity=35+(trial%3)*30;set(kInputMode,0);set(kModel,model);set(kVelocityCurve,1);set(kMaster,-10);set(kTone,60);set(kBody,62);set(kCharacter3,36);set(kCharacter4,20);set(kHammer,32);set(kVelocity,65);set(kDetune,1.2);set(kResonance,30);set(kNoise,0);set(kKeyOff,0);set(kAttack,1);set(kDecay,1800);set(kSustain,92);set(kRelease,720);for(int k:{kDepth,kDrift,kDrive,kChorus,kDelay,kRoom})set(k,0);set(kWidth,100);set(kPedal,0);
  std::vector<Event>events;auto note=[&](double at,double duration,int pitch,int vel){events.push_back({int64_t(at*48000),pitch,vel,true});events.push_back({int64_t((at+duration)*48000),pitch,0,false});};
  note(.15,1.30,pitch,velocity);
  std::sort(events.begin(),events.end(),[](auto&a,auto&b){return a.frame<b.frame;});
  int64_t total=int64_t(3.20*48000);std::vector<float>audio(total*2);size_t next=0;float data[2][128]{},*out[]={data[0],data[1]};
  timeInfo={};timeInfo.sampleRate=48000;timeInfo.tempo=88;timeInfo.timeSigNumerator=timeInfo.timeSigDenominator=4;timeInfo.flags=kVstTempoValid|kVstPpqPosValid|kVstTimeSigValid|kVstTransportPlaying;
  for(int64_t pos=0;pos<total;pos+=128){int frames=std::min<int64_t>(128,total-pos);VstMidiEvent midi[32]{};struct Packet{VstInt32 count=0;VstIntPtr reserved=0;VstEvent*events[32]{};}packet;
   while(next<events.size()&&events[next].frame<pos+frames){auto&v=events[next++];assert(packet.count<32);auto&m=midi[packet.count];m.type=kVstMidiType;m.byteSize=sizeof(m);m.deltaFrames=v.frame-pos;m.midiData[0]=v.on?0x90:0x80;m.midiData[1]=v.note;m.midiData[2]=v.vel;packet.events[packet.count++]=reinterpret_cast<VstEvent*>(&m);}
   if(packet.count)e->dispatcher(e,effProcessEvents,0,0,&packet,0);timeInfo.samplePos=pos;timeInfo.ppqPos=pos*88./(60*48000);e->processReplacing(e,nullptr,out,frames);
   for(int i=0;i<frames;++i)for(int c=0;c<2;++c){float v=data[c][i];assert(std::isfinite(v));audio[(pos+i)*2+c]=v;}
  }
  std::string path=std::string(argv[2])+"/"+argv[3]+"-n"+std::to_string(pitch)+"-v"+std::to_string(velocity)+".wav";wav(path,audio);double energy=0,peak=0;for(float v:audio){energy+=v*v;peak=std::max(peak,double(std::abs(v)));}std::cout<<pitch<<"/"<<velocity<<" peak="<<20*std::log10(std::max(1e-12,peak))<<" rms="<<10*std::log10(energy/audio.size()+1e-24)<<'\n';e->dispatcher(e,effStopProcess,0,0,nullptr,0);e->dispatcher(e,effMainsChanged,0,0,nullptr,0);e->dispatcher(e,effClose,0,0,nullptr,0);
 }dlclose(lib);
}
