#include "mpc_plugin.h"
#include "../src/Parameters.h"
#include <dlfcn.h>
#include <vector>
#include <cassert>
#include <cmath>
#include <iostream>
static VstIntPtr host(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){return op==audioMasterVersion?2400:0;}
int main(int argc,char**argv){assert(argc==3);void*libs[2]{};AEffect*e[2]{};
for(int i=0;i<2;++i){libs[i]=dlopen(argv[i+1],RTLD_NOW|RTLD_LOCAL);assert(libs[i]);auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(libs[i],"VSTPluginMain"));assert(entry);e[i]=entry(host);assert(e[i]&&(i==0?e[i]->numParams>=238:e[i]->numParams==kParamCount)&&e[i]->numPrograms==57);e[i]->dispatcher(e[i],effOpen,0,0,nullptr,0);}
for(int program=0;program<57;++program){
 e[0]->dispatcher(e[0],effSetProgram,0,program,nullptr,0);
 e[0]->setParameter(e[0],kTone,.634f);e[0]->setParameter(e[0],kKey,8.f/11);e[0]->setParameter(e[0],kTranspose,9.f/24);e[0]->setParameter(e[0],kSlots,1);e[0]->setParameter(e[0],kRoot1,5.f/11);e[0]->setParameter(e[0],kPadNote16,83.f/128);e[0]->setParameter(e[0],kLockTone,1);e[0]->setParameter(e[0],kSync,program%2);
 void*ptr=nullptr;const int bytes=e[0]->dispatcher(e[0],effGetChunk,0,0,&ptr,0);assert(bytes>0&&ptr);std::vector<unsigned char>state(static_cast<unsigned char*>(ptr),static_cast<unsigned char*>(ptr)+bytes);
 e[1]->setParameter(e[1],kHiss,.8f);e[1]->setParameter(e[1],kLockHiss,1);
 assert(e[1]->dispatcher(e[1],effSetChunk,0,state.size(),state.data(),0));
 for(int key:kSoundKeys)assert(e[0]->getParameter(e[0],key)==e[1]->getParameter(e[1],key));
 for(int key=kKey;key<=kPadNote16;++key)assert(e[0]->getParameter(e[0],key)==e[1]->getParameter(e[1],key));
 for(int key:kSoundLocks)assert(e[0]->getParameter(e[0],key)==e[1]->getParameter(e[1],key));
 assert(e[1]->getParameter(e[1],kTone)==.634f);assert(e[1]->getParameter(e[1],kHiss)==0&&e[1]->getParameter(e[1],kLockHiss)==0);
}
for(int i=0;i<2;++i){e[i]->dispatcher(e[i],effClose,0,0,nullptr,0);dlclose(libs[i]);}
std::cout<<"PASS57old0.2.37chunks restored into0.3.0:37sound values/37locks/chord/key/slot/MIDI values exact; no factory recipe overwrite\n";
}
