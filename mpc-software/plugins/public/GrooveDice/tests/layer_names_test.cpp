// Exercise the actual VST dispatch path used for MPC Q-Link captions.
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <cstring>
#include <iostream>
static VstIntPtr host(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){return op==audioMasterVersion?2400:0;}
int main(){auto*e=VSTPluginMain(host);assert(e);auto*p=static_cast<GrooveDice*>(e->object);char text[128]{};
 for(int part=0;part<4;++part){p->set(kSoundPart,part);for(int detail=0;detail<2;++detail){p->set(kSoundDetails,detail);for(int row=0;row<(part==2?2:4);++row){p->set(kSoundRow,row);for(int q=0;q<16;++q){std::memset(text,0,sizeof(text));e->dispatcher(e,effGetParamName,kSoundQ0+q,0,text,0);int key=p->soundTarget(q);assert(std::strcmp(text,key<0?"Unused":p->param(key).name.c_str())==0);}}}}
 e->dispatcher(e,effClose,0,0,nullptr,0);std::cout<<"PASS VST Q-Link caption dispatch for all four parts, rows and overview/detail contexts\n";
}
