// Compare public1.14.2 and the candidate through real VST entry points.
// No level normalization: defaults must retain the exact original samples.
#include "../src/DonutTheory.cpp"
#include <cassert>
#include <dlfcn.h>
#include <iostream>
#include <memory>
struct Native {
 void*lib;AEffect*e;float l[128]{},r[128]{};float*out[2]={l,r};
 static VstIntPtr host(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){return op==audioMasterVersion?2400:0;}
 Native(const char*file,int program){lib=dlopen(file,RTLD_NOW|RTLD_LOCAL);assert(lib);auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(lib,"VSTPluginMain"));assert(entry);e=entry(host);assert(e);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,44100);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effSetProgram,0,program,nullptr,0);e->setParameter(e,kInputMode,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);}
 ~Native(){e->dispatcher(e,effClose,0,0,nullptr,0);dlclose(lib);}
 void note(bool on,int key){VstMidiEvent midi{};midi.type=kVstMidiType;midi.byteSize=sizeof(midi);midi.midiData[0]=char(on?0x90:0x80);midi.midiData[1]=char(key);midi.midiData[2]=100;VstEvents events{};events.numEvents=1;events.events[0]=reinterpret_cast<VstEvent*>(&midi);e->dispatcher(e,effProcessEvents,0,0,&events,0);}
 void render(){std::fill(l,l+128,0);std::fill(r,r+128,0);e->processReplacing(e,nullptr,out,128);}
};
int main(int argc,char**argv){assert(argc==3);double worst=0;for(int program=0;program<66;++program){Native old(argv[1],program),now(argv[2],program);double error=0;for(int block=0;block<1400;++block){if(block==10||block==1050){int key=program>=1&&program<13?36:60;old.note(true,key);now.note(true,key);}if(block==450||block==1300){int key=program>=1&&program<13?36:60;old.note(false,key);now.note(false,key);}old.render();now.render();for(int n=0;n<128;++n){error=std::max(error,double(std::abs(old.l[n]-now.l[n])));error=std::max(error,double(std::abs(old.r[n]-now.r[n])));}}worst=std::max(worst,error);if(error>0)std::cout<<"program "<<program<<" max difference "<<error<<"\n";assert(error==0);}std::cout<<"PASS all66factory programs sample-identical to1.14.2 across attacks, tails, idle/rearticulation; max error "<<worst<<"\n";}
