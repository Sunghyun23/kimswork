// Original Tine production-style examples through the real Felt Club VST.
// No Keyscape rendering, external FX, samples or plugin-source changes.
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
struct Event{int64_t frame;int note,vel;bool on;bool controller=false;};
static void wav(const std::string&file,const std::vector<float>&x){std::ofstream f(file,std::ios::binary);auto u16=[&](uint16_t v){f.put(v&255);f.put(v>>8);};auto u32=[&](uint32_t v){u16(v&65535);u16(v>>16);};f.write("RIFF",4);u32(36+x.size()*2);f.write("WAVEfmt ",8);u32(16);u16(1);u16(2);u32(48000);u32(192000);u16(4);u16(16);f.write("data",4);u32(x.size()*2);for(float v:x)u16(uint16_t(int16_t(std::lround(std::max(-1.f,std::min(1.f,v))*32767))));}
int main(int argc,char**argv){assert(argc==4||argc==5);const int requestedModel=argc==5?std::stoi(argv[4]):0;assert(requestedModel>=0&&requestedModel<7);void*lib=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);if(!lib){std::cerr<<dlerror()<<'\n';return 1;}auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(lib,"VSTPluginMain"));assert(entry);std::filesystem::create_directories(argv[2]);const bool wurliSession=std::string(argv[3])=="wurli";const bool uprightSession=std::string(argv[3])=="upright";const bool refined=uprightSession||wurliSession||std::string(argv[3])=="refined";const bool expressive=refined||std::string(argv[3])=="expressive";
 const char*names[]={"velvet-pocket","midnight-motion","afterhours-echo","dry-reference"};
 const char*refinedNames[]={"refined-motion","refined-dry","refined-no-pedal","previous-motion"};
 for(int style=0;style<4;++style){const char*name=refined?refinedNames[style]:names[style];bool revised=refined&&style<3;AEffect*e=entry(host);assert(e&&e->numParams==kParamCount);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,48000);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);e->dispatcher(e,effStartProcess,0,0,nullptr,0);
  struct Decl:mpc::Plugin{Decl():mpc::Plugin({}){declareParameters(*this);}void process(mpc::ProcessContext&)override{}}decl;
  std::vector<std::pair<int,float>>recipe;auto set=[&](int k,float v){e->setParameter(e,k,decl.param(k).toNormalised(v));recipe.push_back({k,v});};
  int voicing=refined?1:style==3?0:style;bool dry=refined?style==1:style==3;
  set(kInputMode,0);set(kModel,requestedModel);set(kVelocityCurve,1);set(kMaster,-10);set(kTone,voicing==1?60:voicing==2?51:56);set(kBody,voicing==1?62:voicing==2?64:57);set(kCharacter3,voicing==1?36:voicing==2?18:26);set(kCharacter4,voicing==1?20:voicing==2?16:14);set(kHammer,32);set(kVelocity,65);set(kDetune,1.2);set(kResonance,30);set(kNoise,0);set(kKeyOff,0);set(kAttack,1);set(kDecay,1800);set(kSustain,92);set(kRelease,voicing==1?720:voicing==2?900:520);set(kPedal,1);
  set(kMotionMode,1);set(kSync,1);set(kShape,0);set(kRate,(voicing==0?6:9)/17.f);set(kDepth,dry?0:voicing==1?65:voicing==2?42:24);set(kWidth,voicing==1?100:voicing==2?95:85);set(kMotionDelay,0);set(kMotionFade,0);
  set(kDrive,dry?0:voicing==1?2:voicing==2?3:1.8);set(kDrift,dry?0:voicing==1?2:voicing==2?2.5:1);set(kDriftRate,.20);set(kChorus,dry?0:voicing==1?14:voicing==2?8:6);set(kChorusRate,voicing==1?.22:.28);set(kChorusDepth,voicing==1?45:35);
  set(kDelay,dry?0:voicing==1?20:voicing==2?34:12);set(kDelayDivision,voicing==2?7:10);set(kDelayFeedback,voicing==1?28:voicing==2?38:19);set(kRoom,dry?0:voicing==1?42:voicing==2?58:28);set(kRoomTime,voicing==1?62:voicing==2?78:38);set(kRoomDamp,voicing==1?52:voicing==2?68:62);
  if(revised){set(kChorus,dry?0:5);set(kDelay,dry?0:10);set(kDelayFeedback,18);set(kRoom,dry?0:27);set(kRoomTime,45);set(kRoomDamp,58);}
  if(uprightSession){set(kDepth,0);set(kDrive,0);set(kDrift,0);set(kChorus,0);set(kDelay,0);set(kRoom,dry?0:18);}
  if(wurliSession){set(kMotionMode,0);set(kSync,0);set(kRate,std::log(5.f/.05f)/std::log(240.f));set(kDepth,dry?0:30);set(kChorus,0);set(kDelay,dry?0:6);set(kRoom,dry?0:16);set(kDrive,0);set(kDrift,0);}
  std::ofstream performance(std::string(argv[2])+"/"+name+"-notes.csv");performance<<"seconds,note,velocity,duration_seconds\n";
  std::vector<Event>events;auto note=[&](double at,double duration,int pitch,int vel){performance<<at<<","<<pitch<<","<<vel<<","<<duration<<"\n";events.push_back({int64_t(at*48000),pitch,vel,true});events.push_back({int64_t((at+duration)*48000),pitch,0,false});};
  // Original pocketed Dm9/G13/Cmaj9/Am9 progression, controlled upper voices.
  constexpr int chords[4][5]={{38,53,57,60,64},{43,53,57,59,64},{48,52,55,59,62},{45,52,55,59,60}};
  const double beat=60./88,bar=beat*4,start=.15;
  constexpr int touch[4][5]={{45,42,48,47,56},{49,46,52,50,62},{63,57,65,62,80},{48,45,51,49,60}};
  constexpr int ghost[3]={30,36,33},ghostLift[4]={0,3,8,0};
  if(!revised){
  for(int b=0;b<4;++b){for(int n=0;n<5;++n)note(start+b*bar+.018+n*.011,beat*(b==2?2.15:2.55),chords[b][n],expressive?touch[b][n]:(n==0?64:76)+n*2+(b%2)*3);for(int n=2;n<5;++n)note(start+b*bar+beat*3.25+.013+(n-2)*.008,beat*.38,chords[b][n],expressive?ghost[n-2]+ghostLift[b]:60+n*3);}
  note(start+beat*2.85,beat*.3,67,expressive?62:70);note(start+bar+beat*2.8,beat*.35,66,expressive?58:67);note(start+2*bar+beat*2.8,beat*.35,64,expressive?76:70);note(start+3*bar+beat*2.8,beat*.35,62,expressive?54:65);
  }else{
   // Open inner voices; E stays on top across the four main chords.
   constexpr int open[4][5]={{38,53,57,60,64},{43,53,57,59,64},{48,55,59,62,64},{45,55,59,60,64}};
   constexpr double offsets[4][5]={{0,.004,.007,.009,.006},{0,.016,.020,.024,.028},{0,.002,.005,.004,.007},{0,.006,.012,.016,.010}};
   constexpr double push[4]={.018,-.012,.027,.009},lengths[4]={2.20,2.65,1.75,2.90};
   constexpr int velocities[4][5]={{45,43,48,46,58},{47,42,49,45,61},{57,50,56,53,70},{43,40,47,43,57}};
   for(int b=0;b<4;++b)for(int n=0;n<5;++n)note(start+b*bar+push[b]+offsets[b][n],beat*(lengths[b]-(n==0?.24:0)),open[b][n],velocities[b][n]);
   // Space in bars 2/4; light anticipations in bars 1/3 only.
   for(int n=2;n<5;++n){note(start+beat*3.45+(n-2)*.005,beat*.28,open[0][n],29+(n-2)*4);note(start+2*bar+beat*2.65+(n-2)*.003,beat*.42,open[2][n],34+(n-2)*3);}
   note(start+bar+beat*3.35,beat*.40,62,46);note(start+3*bar+beat*3.20,beat*.38,62,42);
   std::ofstream pedal(std::string(argv[2])+"/"+name+"-pedal.csv");pedal<<"seconds,value\n";
   auto cc=[&](double at,int value){events.push_back({int64_t(at*48000),64,value,false,true});pedal<<at<<","<<value<<"\n";};
   if(style!=2){for(int b=0;b<4;++b){cc(start+b*bar+push[b]+.065,100);cc(start+(b+1)*bar-.040,0);}cc(start+4*bar,0);}
  }
  std::stable_sort(events.begin(),events.end(),[](auto&a,auto&b){return a.frame<b.frame;});
  int64_t total=int64_t((start+4*bar+4.2)*48000);std::vector<float>audio(total*2);size_t next=0;float data[2][128]{},*out[]={data[0],data[1]};
  timeInfo={};timeInfo.sampleRate=48000;timeInfo.tempo=88;timeInfo.timeSigNumerator=timeInfo.timeSigDenominator=4;timeInfo.flags=kVstTempoValid|kVstPpqPosValid|kVstTimeSigValid|kVstTransportPlaying;
  for(int64_t pos=0;pos<total;pos+=128){int frames=std::min<int64_t>(128,total-pos);VstMidiEvent midi[32]{};struct Packet{VstInt32 count=0;VstIntPtr reserved=0;VstEvent*events[32]{};}packet;
   while(next<events.size()&&events[next].frame<pos+frames){auto&v=events[next++];assert(packet.count<32);auto&m=midi[packet.count];m.type=kVstMidiType;m.byteSize=sizeof(m);m.deltaFrames=v.frame-pos;m.midiData[0]=v.controller?0xB0:v.on?0x90:0x80;m.midiData[1]=v.note;m.midiData[2]=v.vel;packet.events[packet.count++]=reinterpret_cast<VstEvent*>(&m);}
   if(packet.count)e->dispatcher(e,effProcessEvents,0,0,&packet,0);timeInfo.samplePos=pos;timeInfo.ppqPos=pos*88./(60*48000);e->processReplacing(e,nullptr,out,frames);
   for(int i=0;i<frames;++i)for(int c=0;c<2;++c){float v=data[c][i];assert(std::isfinite(v));audio[(pos+i)*2+c]=v;}
  }
  std::string path=std::string(argv[2])+"/"+name+"-raw.wav";wav(path,audio);double energy=0,peak=0;for(float v:audio){energy+=v*v;peak=std::max(peak,double(std::abs(v)));}std::cout<<name<<" peak="<<20*std::log10(std::max(1e-12,peak))<<" rms="<<10*std::log10(energy/audio.size()+1e-24)<<'\n';
  std::ofstream settings(std::string(argv[2])+"/"+name+"-settings.txt");for(auto pair:recipe)settings<<decl.param(pair.first).id<<"="<<pair.second<<'\n';
  e->dispatcher(e,effStopProcess,0,0,nullptr,0);e->dispatcher(e,effMainsChanged,0,0,nullptr,0);e->dispatcher(e,effClose,0,0,nullptr,0);
 }dlclose(lib);
}
