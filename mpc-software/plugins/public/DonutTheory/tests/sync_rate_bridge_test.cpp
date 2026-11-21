#include <cassert>
#include <atomic>
#include <iostream>
#include <thread>
#include "../src/DonutTheory.cpp"
static thread_local bool inRender=false;
static std::atomic<int>callbacks{0};
static VstIntPtr host(AEffect*e,VstInt32 op,VstInt32 index,VstIntPtr,void*,float value){
 if(op==audioMasterVersion)return 2400;
 if(op==audioMasterAutomate||op==audioMasterUpdateDisplay){assert(!inRender);++callbacks;}
 if(op==audioMasterAutomate&&e){e->getParameter(e,index);char text[64]{};e->dispatcher(e,effGetParamDisplay,index,0,text,0);if(index==kPerfLfoRate||index==kLfoRateControl||index==kLfo2RateControl||index==kModRateControl)e->setParameter(e,index,value);}
 return 0;
}
int main(){AEffect*e=VSTPluginMain(host);assert(e&&e->numParams==405);auto*p=static_cast<DonutTheory*>(e->object);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,48000);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);
 float l[128]{},r[128]{};float*out[]={l,r};std::atomic<int>requested{0},done{0};std::atomic<bool>quit{false};std::thread audio([&]{int seen=0;while(!quit.load()){int n=requested.load();if(n==seen){std::this_thread::yield();continue;}seen=n;inRender=true;e->processReplacing(e,nullptr,out,128);inRender=false;done.store(n);}});
 auto render=[&]{int n=requested.fetch_add(1)+1;while(done.load()!=n)std::this_thread::yield();};render();e->dispatcher(e,effIdle_Deprecated,0,0,nullptr,0);
 int syncs[]={kSync,kModSync,kLfo2Sync},full[]={kLfo1SyncDivision,kMotionSyncDivision,kLfo2SyncDivision},relays[]={kLfoRateControl,kModRateControl,kLfo2RateControl};
 for(int module=0;module<3;++module){p->set(syncs[module],1);p->set(full[module],0);render();e->getParameter(e,relays[module]);
  for(int direction:{1,-1}){unsigned seen=0;for(int tick=0;tick<1200;++tick){callbacks.store(0);float value=std::clamp(e->getParameter(e,relays[module])+direction*.001f,0.f,1.f);e->setParameter(e,relays[module],value);render();e->getParameter(e,relays[module]);seen|=1u<<p->getInt(full[module]);assert(callbacks.load()<=16);}assert(seen==32767);}
 }
 // Idle after the final cursor state is stable emits no repeated notifications.
 e->dispatcher(e,effIdle_Deprecated,0,0,nullptr,0);callbacks.store(0);for(int block=0;block<100;++block){render();e->getParameter(e,kPerfLfoRate);}assert(callbacks.load()==0);
 quit.store(true);audio.join();e->dispatcher(e,effClose,0,0,nullptr,0);
 std::cout<<"PASS:real VST relative setter/getter turns across all3Sync modules/all15choices, separate audio thread, reentrant rate echoes, bounded callbacks and100quiet blocks\n";
}
