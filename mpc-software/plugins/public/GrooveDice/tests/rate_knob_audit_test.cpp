// Audit existing rhythm/FX controls; no engine or metadata changes.
#define main priorLoopMain
#include "one_bar_loop_test.cpp"
#undef main
#include <dlfcn.h>
static VstIntPtr auditHost(AEffect*,VstInt32 op,VstInt32,VstIntPtr,void*,float){return op==audioMasterVersion?2400:0;}
int main(){
 for(int q:{4,5,6,7,12,13,14,15}){Host h;auto&p=h.p;int relay=kQ1+q,target=p.proxyTarget(q);assert(target>=0);int choices=int(p.param(target).choices.size());assert(choices>=2);p.setNormalised(target,0);h.block();auto rolls=p.diceRolls.load();std::vector<bool>seen(choices);
  for(int n=0;n<1200;++n){p.setNormalised(relay,std::min(1.f,p.getNormalised(relay)+.001f));h.block();seen[p.getInt(target)]=true;}
  for(bool found:seen)assert(found);assert(p.getInt(target)==choices-1);seen.assign(choices,false);
  for(int n=0;n<1200;++n){p.setNormalised(relay,std::max(0.f,p.getNormalised(relay)-.001f));h.block();seen[p.getInt(target)]=true;}
  for(bool found:seen)assert(found);assert(p.getInt(target)==0&&p.diceRolls.load()==rolls);
 }
 std::cout<<"PASS:Groove Dice relative0.001Q-Link turns visit every Rule/Hat/Open/Ride/Crash choice both ways without rolling Dice; RT allocation guard enabled\n";
 {Host h;auto&p=h.p;for(int rhythm:{9,12}){p.set(kHatRhythmFull,rhythm);h.block();int stride=rhythm==9?720:360;auto notes=h.notes(4,2);assert(!notes.empty());for(auto [tick,role,vel,chance,gate]:notes)assert(tick%stride==0);}
  p.setNormalised(kModRate,0);h.block();float before=p.get(kModRate);for(int n=0;n<100;++n){p.setNormalised(kModRate,p.getNormalised(kModRate)+.001f);h.block();}assert(p.get(kModRate)>before);std::string text;assert(!p.param(kModRate).unit.empty());
 }
 std::cout<<"PASS:existing hat dotted grids have720/360tick spacing; free-Hz FX ModRate retains incremental input\n";
 {const char* folder=std::getenv("MPC_PLUGIN_FOLDER");assert(folder);std::string path=std::string(folder)+"/build/native/groove-dice.dylib";void*lib=dlopen(path.c_str(),RTLD_NOW|RTLD_LOCAL);assert(lib);auto entry=reinterpret_cast<AEffect*(*)(audioMasterCallback)>(dlsym(lib,"VSTPluginMain"));assert(entry);AEffect* e=entry(auditHost);assert(e&&e->numParams==kParamCount);e->dispatcher(e,effOpen,0,0,nullptr,0);e->dispatcher(e,effSetSampleRate,0,0,nullptr,44100);e->dispatcher(e,effSetBlockSize,0,128,nullptr,0);e->dispatcher(e,effMainsChanged,0,1,nullptr,0);float data[16][128]{},*out[16];for(int c=0;c<16;++c)out[c]=data[c];auto render=[&]{guarded=true;e->processReplacing(e,nullptr,out,128);guarded=false;};render();
  for(int item=0;item<8;++item){int qs[]={4,5,6,7,12,13,14,15},targets[]={kRulePickFullKick,kRulePickFullSnare,kRulePickFullHat,kRulePickFullPerc,kHatRhythmFull,kOpenHatRule,kRideRule,kCrashRule},counts[]={16,16,16,16,24,4,5,6};int relay=kQ1+qs[item],target=targets[item],choices=counts[item];e->setParameter(e,target,0);render();std::vector<bool>seen(choices);
   for(int n=0;n<1200;++n){e->setParameter(e,relay,std::min(1.f,e->getParameter(e,relay)+.001f));render();seen[std::lround(e->getParameter(e,target)*(choices-1))]=true;char display[128]{};e->dispatcher(e,effGetParamDisplay,relay,0,display,0);assert(display[0]);}
   for(bool value:seen)assert(value);assert(std::lround(e->getParameter(e,target)*(choices-1))==choices-1);seen.assign(choices,false);
   for(int n=0;n<1200;++n){e->setParameter(e,relay,std::max(0.f,e->getParameter(e,relay)-.001f));render();seen[std::lround(e->getParameter(e,target)*(choices-1))]=true;}
   for(bool value:seen)assert(value);assert(std::lround(e->getParameter(e,target)*(choices-1))==0);
  }
  assert(e->getParameter(e,kRoll)==0);e->dispatcher(e,effClose,0,0,nullptr,0);dlclose(lib);
 }
 std::cout<<"PASS:real VST setters/getters/display/rendering for all8rhythm selectors sweep every choice both directions without triggering Dice or allocating in render\n";

}
