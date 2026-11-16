#include "../src/CratePress.cpp"
#include <thread>
#include <iostream>
#include <cstdlib>
static thread_local bool audio=false;static std::atomic<int> calls{0},bad{0};static std::array<float,CratePress::Count>cache{};
static VstIntPtr host(AEffect*,VstInt32 op,VstInt32 index,VstIntPtr,void*,float value){
 if(op==audioMasterVersion)return 2400;
 if(op==audioMasterAutomate){if(audio)++bad;cache[index]=value;++calls;}
 if(op==audioMasterUpdateDisplay&&audio)++bad;return 0;
}
static void check(bool ok,const char*m){if(!ok){std::cerr<<m<<'\n';std::exit(1);}}
int main(){
 auto*e=VSTPluginMain(host);e->dispatcher(e,effOpen,0,0,nullptr,0);
 e->dispatcher(e,effSetProgram,0,1,nullptr,0);
 auto*p=static_cast<CratePress*>(e->object);
 check(calls>=CratePress::LegacyCount,"program must notify cached skin values");
 check(std::abs(cache[CratePress::Compress]-.55f)<1e-6,"Pocket Press cache reflects program");
 auto render=[&](){audio=true;float l[128],r[128],a[128],b[128];for(int i=0;i<128;++i)l[i]=r[i]=.6f;float*in[]{l,r};float*out[]{a,b};mpc::ProcessContext c{};c.inputs=in;c.outputs=out;c.numInputs=c.numOutputs=2;c.numFrames=128;c.sampleRate=44100;
 for(int j=0;j<60;++j){p->process(c);e->getParameter(e,CratePress::InputMeter);}audio=false;};
 const int before=calls;std::thread t(render);t.join();check(calls==before&&bad==0,"render must never call host, including parameter reads");
 e->getParameter(e,CratePress::InputMeter);check(calls>before,"UI poll drains telemetry notifications");check(cache[CratePress::InputMeter]>.3f,"live input meter cache refreshed");
 check(cache[CratePress::CurveBase+47]<1,"live compression curve cache refreshed");
 std::cout<<"PASS: program refresh, deferred telemetry, no render-thread host callbacks\n";
 e->dispatcher(e,effClose,0,0,nullptr,0);
}
