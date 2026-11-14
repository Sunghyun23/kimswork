#include <cassert>
#include <cstring>
#include <iostream>
#include <thread>
#include "../src/DonutTheory.cpp"
static bool rendering=false;
static int updates=0,automations=0,idleRequests=0;
static VstIntPtr host(AEffect* e,VstInt32 op,VstInt32,VstIntPtr,void*,float){
 if(op==audioMasterVersion)return 2400;
 if(op==audioMasterNeedIdle_Deprecated){assert(!rendering);++idleRequests;return 1;}
 if(op==audioMasterAutomate){assert(!rendering);++automations;}
 if(op==audioMasterUpdateDisplay){assert(!rendering);++updates;
  // Hosts may synchronously re-read controls during a refresh; no recursion.
  char text[64]{};e->dispatcher(e,effGetParamDisplay,kCutoff,0,text,0);}
 return 0;
}
int main(){
 auto* e=VSTPluginMain(host);assert(e&&idleRequests==1);auto* p=static_cast<DonutTheory*>(e->object);
 auto dispatch=[&](int op,int index=0,VstIntPtr value=0,void* ptr=nullptr,float opt=0){return e->dispatcher(e,op,index,value,ptr,opt);};
 dispatch(effOpen);assert(idleRequests==2);assert(dispatch(effCanBeAutomated,kEditRoot1)==0);
 dispatch(effSetSampleRate,0,0,nullptr,44100);dispatch(effSetBlockSize,0,128);dispatch(effMainsChanged,0,1);
 float l[128]{},r[128]{};float* out[]={l,r};
 auto render=[&]{rendering=true;e->processReplacing(e,nullptr,out,128);rendering=false;};
 dispatch(effIdle_Deprecated);assert(automations>=kNumParams-91&&updates>0);updates=automations=0;
 e->setParameter(e,kSoundNext,1);render();assert(updates==0&&automations==0);
 char display[64]{};dispatch(effGetParamDisplay,kCutoff,0,display);assert(updates==1&&automations>0);assert(dispatch(effGetProgram)==1);
 char name[64]{};dispatch(effGetProgramName,0,0,name);assert(std::strcmp(name,p->programName(1))==0);
 p->set(kRoot1,61);p->set(kVolume,-20);dispatch(effSetProgram,0,12);render();
 assert(dispatch(effGetProgram)==12&&p->getInt(kRoot1)==61&&std::abs(p->get(kVolume)+20)<.001f);
 void* data=nullptr;auto size=dispatch(effGetChunk,0,0,&data);std::vector<unsigned char> saved((unsigned char*)data,(unsigned char*)data+size);
 dispatch(effSetProgram,0,7);render();dispatch(effSetChunk,0,size,saved.data());render();
 assert(dispatch(effGetProgram)==12&&p->getInt(kRoot1)==61);
 dispatch(effIdle_Deprecated);int count=updates;dispatch(effIdle_Deprecated);assert(updates==count);
 dispatch(effClose);
 // MPC may poll values on its control thread without asking for display strings.
 // Render on a real separate thread; a value read there must never notify the host.
 e=VSTPluginMain(host);p=static_cast<DonutTheory*>(e->object);dispatch(effOpen);
 dispatch(effSetSampleRate,0,0,nullptr,44100);dispatch(effSetBlockSize,0,128);dispatch(effMainsChanged,0,1);
 p->set(kPadLearn,1);dispatch(effIdle_Deprecated);updates=automations=0;
 VstMidiEvent note{};note.type=kVstMidiType;note.byteSize=sizeof(note);note.midiData[0]=char(0x90);note.midiData[1]=60;note.midiData[2]=100;
 VstEvents events{};events.numEvents=1;events.events[0]=reinterpret_cast<VstEvent*>(&note);dispatch(effProcessEvents,0,0,&events);
 std::thread audio([&]{render();assert(e->getParameter(e,kPadLearn)==0);assert(updates==0&&automations==0);});audio.join();
 assert(p->getInt(kPadNote1)==60&&p->getInt(kPadLearn)==0&&updates==0&&automations==0);
 e->getParameter(e,kPadLearn);assert(updates>0&&automations>=3); // control-thread poll publishes the registration immediately
 dispatch(effClose);
 std::cout<<"PASS: VST preset/state identity, master volume, deferred callbacks, reentrant UI refresh, idle coalescing\n";
}
