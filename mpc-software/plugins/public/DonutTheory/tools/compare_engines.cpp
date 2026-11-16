// Same serialized parameters and MIDI, two native VST libraries. No normalization.
#include "../src/DonutTheory.cpp"
#include <dlfcn.h>
#include <chrono>
#include <fstream>
#include <iostream>
#include <cassert>
static VstIntPtr hostCompare(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){return op==audioMasterVersion?2400:0;}
int main(int argc,char** argv){
 if(argc!=4)return 2;
 std::ofstream audio(std::string(argv[3])+".f32",std::ios::binary),report(std::string(argv[3])+".csv");
 report<<"test,engine,voices,peak,rms,cpu_percent,worst_block_percent\n";
 for(int test=0;test<4;++test){
  DonutTheory recipe;recipe.setProgram(0);recipe.set(kInputMode,0);recipe.set(kBody,40);recipe.set(kWave1,2);recipe.set(kWave2,3);recipe.set(kMix,35);recipe.set(kTone,35);recipe.set(kVolume,-12);recipe.set(kAmpAttack,5);recipe.set(kAmpSustain,80);recipe.set(kAmpRelease,100);
  recipe.set(kCutoff,test==0?500:test==1?1400:2400);recipe.set(kResonance,test==1?70:30);
  auto state=recipe.internalSaveState();
  for(int engine=0;engine<2;++engine){
   void* library=dlopen(argv[engine+1],RTLD_NOW|RTLD_LOCAL);if(!library){std::cerr<<dlerror()<<"\n";return 3;}
   auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(library,"VSTPluginMain"));assert(entry);AEffect* e=entry(hostCompare);
   auto dispatch=[&](int op,int index=0,VstIntPtr value=0,void* ptr=nullptr,float opt=0){return e->dispatcher(e,op,index,value,ptr,opt);};
   dispatch(effOpen);dispatch(effSetSampleRate,0,0,nullptr,44100);dispatch(effSetBlockSize,0,128);dispatch(effMainsChanged,0,1);dispatch(effSetChunk,0,state.size(),state.data());
   const int count=test==3?16:test==2?4:1;const int root=test==0?36:60;
   float l[128]{},r[128]{};float* out[]={l,r};float peak=0;double energy=0,elapsed=0,worst=0;
   for(int block=0;block<1378;++block){
    if(block==0||block==1034){for(int note=0;note<count;++note){VstMidiEvent event{};event.type=kVstMidiType;event.byteSize=sizeof(event);event.midiData[0]=char(block==0?0x90:0x80);event.midiData[1]=char(root+note*2);event.midiData[2]=100;VstEvents events{};events.numEvents=1;events.events[0]=reinterpret_cast<VstEvent*>(&event);dispatch(effProcessEvents,0,0,&events);}}
    auto start=std::chrono::steady_clock::now();e->processReplacing(e,nullptr,out,128);const double time=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count();elapsed+=time;worst=std::max(worst,time);
    for(int n=0;n<128;++n){assert(std::isfinite(l[n])&&std::isfinite(r[n]));peak=std::max(peak,std::max(std::abs(l[n]),std::abs(r[n])));energy+=(double(l[n])*l[n]+double(r[n])*r[n])*.5;if(test<3){float sample=(l[n]+r[n])*.5f;audio.write(reinterpret_cast<char*>(&sample),4);}}
   }
   assert(peak>1.e-5f&&peak<1);report<<test<<","<<engine<<","<<count<<","<<peak<<","<<std::sqrt(energy/(1378*128))<<","<<elapsed/(1378.*128/44100)*100<<","<<worst/(128./44100)*100<<"\n";
   dispatch(effClose);dlclose(library);
  }
 }
 std::cout<<"PASS: identical-state engine A/B and 16-voice timing rendered\n";
}
