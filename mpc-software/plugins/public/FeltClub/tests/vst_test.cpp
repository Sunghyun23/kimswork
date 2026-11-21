#include "mpc_plugin.h"
#include "../src/Parameters.h"
#include "../src/FactoryNames.h"
#include <dlfcn.h>
#include <cassert>
#include <cmath>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>
#include <set>
#include <thread>
#include <chrono>
static bool inRender=false;static int badCallbacks=0,uiUpdates=0;
static VstIntPtr callback(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){if(op==audioMasterUpdateDisplay)++uiUpdates;if(inRender&&(op==audioMasterAutomate||op==audioMasterUpdateDisplay))++badCallbacks;return op==audioMasterVersion?2400:0;}
int main(int argc,char**argv){assert(argc==2);void*lib=dlopen(argv[1],RTLD_NOW|RTLD_LOCAL);assert(lib);auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(lib,"VSTPluginMain"));assert(entry);AEffect*e=entry(callback);assert(e&&e->numParams==kParamCount&&e->numPrograms==57);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,48000);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);
 float data[2][128]{},*out[]={data[0],data[1]};auto block=[&]{inRender=true;e->processReplacing(e,nullptr,out,128);inRender=false;};block();
 // Reuse the render thread for a later UI poll, as some hosts do. Pending
 // display updates must flush outside render, without requiring a new thread.
 int sharedThreadUpdates=uiUpdates;
 e->getParameter(e,kTone);
 assert(uiUpdates>sharedThreadUpdates);
 std::cout<<"PASS render-thread reuse permits subsequent UI display notifications\n";
 char noiseName[64]{};e->dispatcher(e,effGetParamName,kMainQ0+3,0,noiseName,0);assert(std::string(noiseName)=="Noise");float barkBefore=e->getParameter(e,kCharacter4);e->setParameter(e,kMainQ0+3,.5f);assert(e->getParameter(e,kHiss)==.5f&&e->getParameter(e,kCharacter4)==barkBefore);e->setParameter(e,kHiss,0);
 std::set<std::string>names;bool wurli=false;for(int i=0;i<57;++i){char text[64]{};assert(e->dispatcher(e,effGetProgramNameIndexed,i,0,text,0)==1&&text[0]);assert(names.insert(text).second);wurli|=std::string(text).find("Wurli")!=std::string::npos;}assert(wurli);
 struct Parameters : mpc::Plugin {
  Parameters():mpc::Plugin({}){declareParameters(*this);}
  void process(mpc::ProcessContext&) override {}
 } parameters;
 const auto plain=[&](int key){return parameters.param(key).toPlain(e->getParameter(e,key));};
 assert(std::abs(plain(kBody)-62)<.01f&&std::abs(plain(kRelease)-420)<.01f);
 e->dispatcher(e,effSetProgram,0,felt::kSundayUprightProgram,nullptr,0);
 block();
 assert(std::lround(plain(kModel))==2);
 assert(std::abs(plain(kBody)-50)<.01f&&std::abs(plain(kRelease)-520)<.01f);
 assert(plain(kRoom)==0&&plain(kDelay)==0&&std::lround(plain(kVelocityCurve))==1);
 std::cout<<"PASS Sunday Upright loads the approved voicing through the native preset selector\n";
 e->dispatcher(e,effSetProgram,0,9,nullptr,0);block();assert(std::lround(e->getParameter(e,kModel)*6)==1);char text[64]{};e->dispatcher(e,effGetProgramName,0,0,text,0);assert(std::string(text)=="Copper Wurli");e->dispatcher(e,effGetParamName,kMainQ0+2,0,text,0);assert(std::string(text)=="Reed");
 std::cout<<"PASS actual VST57unique native program names and WURLI selection/semantic Q-Link captions\n";
 int updatesBefore=uiUpdates;std::thread ui([&]{e->setParameter(e,kRoll,1);});ui.join();assert(uiUpdates>updatesBefore);block();assert(e->getParameter(e,kPreset)==0);e->dispatcher(e,effMainsChanged,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,48000);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);e->setParameter(e,kUndo,1);block();assert(e->dispatcher(e,effGetProgram,0,0,nullptr,0)==9);
 std::cout<<"PASS stopped-instance UI Dice command service and exact native preset Undo\n";
 void*chunk=nullptr;auto size=e->dispatcher(e,effGetChunk,0,0,&chunk,0);std::vector<uint8_t>state(static_cast<uint8_t*>(chunk),static_cast<uint8_t*>(chunk)+size);std::vector<float>values(kParamCount);for(int i=0;i<kParamCount;++i)values[i]=e->getParameter(e,i);assert(e->dispatcher(e,effSetChunk,0,size,state.data(),0));for(int i=0;i<kParamCount;++i)e->setParameter(e,i,values[i]);for(int i=0;i<220;++i)block();assert(e->dispatcher(e,effGetProgram,0,0,nullptr,0)==9);
 std::cout<<"PASS MPC late full-parameter replay does not retrigger commands or wipe factory identity\n";
 auto bad=state;bad.pop_back();float before=e->getParameter(e,kTone);assert(!e->dispatcher(e,effSetChunk,0,bad.size(),bad.data(),0)&&e->getParameter(e,kTone)==before);bad=state;size_t pos=13+bad[12];bad[pos]=0;bad[pos+1]=0;bad[pos+2]=0xc0;bad[pos+3]=0x7f;assert(!e->dispatcher(e,effSetChunk,0,bad.size(),bad.data(),0)&&e->getParameter(e,kTone)==before);
 e->setParameter(e,kTone,NAN);assert(std::isfinite(e->getParameter(e,kTone)));e->dispatcher(e,effGetParamName,-1,0,text,0);assert(!text[0]);
 std::cout<<"PASS truncated/NaN state is transactional; invalid values/indices safe\n";
 for(int direction:{1,-1}){e->setParameter(e,kRate,direction>0?0:1);std::set<std::string>seen;for(int i=0;i<1100;++i){float next=std::max(0.f,std::min(1.f,e->getParameter(e,kRate)+direction*.001f));e->setParameter(e,kRate,next);e->dispatcher(e,effGetParamDisplay,kMainQ0+8,0,text,0);seen.insert(text);}assert(seen.size()==18);}
 // UI edit mode is transient: a preset/project restore must not open it.
 e->setParameter(e,kChordEditing,1);
 assert(e->getParameter(e,kChordEditing)==1);
 void*editChunk=nullptr;auto editSize=e->dispatcher(e,effGetChunk,0,0,&editChunk,0);
 std::vector<uint8_t>editState(static_cast<uint8_t*>(editChunk),static_cast<uint8_t*>(editChunk)+editSize);
 assert(e->dispatcher(e,effSetChunk,0,editState.size(),editState.data(),0));
 assert(e->getParameter(e,kChordEditing)==0);
 std::cout<<"PASS restored instrument clears transient inline chord editor; native tab selection is host-owned\n";
 // A stopped MPC may suspend processReplacing for an unbounded time.
 assert(e->dispatcher(e,effSetChunk,0,size,state.data(),0));
 std::this_thread::sleep_for(std::chrono::milliseconds(550));
 std::thread stoppedUi([&]{e->setParameter(e,kRoll,1);e->getParameter(e,kTone);});stoppedUi.join();
 assert(e->getParameter(e,kPreset)==0);
 std::cout<<"PASS state-load guard expires while audio processing is suspended; UI Dice works without rendering\n";
 assert(!badCallbacks);e->dispatcher(e,effMainsChanged,0,0,nullptr,0);e->dispatcher(e,effClose,0,0,nullptr,0);dlclose(lib);std::cout<<"PASS fractional synced Rate visits18divisions both ways, readable Main Q-Link values and no render host callbacks\n";
}
