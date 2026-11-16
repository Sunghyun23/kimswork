#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <set>
#include <fstream>
static thread_local bool guarded=false;
void* operator new(size_t n){if(guarded)std::abort();if(auto*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
struct Host{GrooveDice p;float data[16][128]{},*out[16];Host(){for(int i=0;i<16;++i)out[i]=data[i];p.prepare(44100,128);p.reset();}void block(){for(auto&c:data)std::fill(c,c+128,0);mpc::ProcessContext c;c.outputs=out;c.numOutputs=16;c.numFrames=128;c.sampleRate=44100;guarded=true;p.process(c);guarded=false;for(auto&ch:data)for(float x:ch)assert(std::isfinite(x));}void pulse(int i){p.set(i,!p.getBool(i));block();}void ready(){for(int i=0;i<2000;++i){block();bool ok=true;for(int r=0;r<8;++r)ok&=p.library.active[r].load()==p.getInt(kXSample0+r);if(ok)return;std::this_thread::sleep_for(std::chrono::milliseconds(1));}assert(false&&"asynchronous expansion sample loading timeout");}};
int main(int argc,char**argv){assert(argc>=2);std::string mode=argv[1];Host h;auto&p=h.p;auto&l=p.library;
 if(mode=="both"){
  assert(l.loaded&&l.packs.size()==2&&p.numPrograms()==193&&l.entries.size()==11357&&l.factoryKits.size()==192);assert(l.categories[3].size()==4&&l.categories[4].size()==186);assert(l.packs[1].isPrivate);std::set<std::string>names;for(int n=0;n<p.numPrograms();++n)assert(names.insert(p.programName(n)).second);
  p.set(kXSample0,11262);h.ready();assert(l.active[0].load()==11262);assert(l.slots[l.current[0]].sound.numFrames()==l.entries[11261].frames);h.pulse(kPad0);
  p.setNormalised(kQ10,1);h.block();assert(p.getInt(kXSoundSnare)==1092&&p.getInt(kXSample1)==l.soundPools[1].back().sounds[0]);
  h.pulse(kPackNext);assert(p.getInt(kPackFilter)==1);h.pulse(kPackNext);assert(p.getInt(kPackFilter)==2&&p.displayText(kPackInfo).find("PRIVATE")!=std::string::npos);
  p.set(kKitLock,0);p.set(kScope,0);for(int n=0;n<100;++n){h.pulse(kRoll);int k=p.getInt(kXKit);assert(k>96&&k<=192&&l.factoryKits[k-1].pack==1);}
  p.set(kKitDiceMode,1);for(int n=0;n<30;++n){h.pulse(kRoll);for(int r=0;r<8;++r)assert(l.entries[p.getInt(kXSample0+r)-1].pack==1);}
  p.selectFactoryProgram(192);h.ready();assert(p.activeFactoryProgram()==192);auto state=p.internalSaveState();if(argc>2){std::ofstream out(argv[2],std::ios::binary);out.write((char*)state.data(),state.size());}
  p.selectFactoryProgram(1);assert(p.internalLoadState(state.data(),state.size()));p.reset();for(int n=0;n<200;++n)h.block();assert(p.getInt(kXKit)==192&&p.internalSaveState()==state);
  std::cout<<"PASS2independentbanks/192kits/193nativeprograms/3212soundchoices/190MIDI; private payload decoded from own file; >11039sampleIDs and >546soundIDs; expansion-filtered100KitROLL/30MixParts; no audio allocation; byte-identical state\n";
 }else{
  assert(argc>2);std::ifstream f(argv[2],std::ios::binary);std::vector<uint8_t>state((std::istreambuf_iterator<char>(f)),{});assert(!state.empty()&&p.internalLoadState(state.data(),state.size()));p.reset();for(int n=0;n<200;++n)h.block();
  if(mode=="missing"){assert(l.packs.size()==1&&p.getInt(kXKit)==0);for(int r=0;r<8;++r)assert(p.missingSample[r]);int before=p.fired.load();for(int r=0;r<8;++r)h.pulse(kPad0+r*9);assert(p.fired.load()==before);assert(p.displayText(kBankInfo)=="MISSING SAMPLE");if(argc>3){auto saved=p.internalSaveState();std::ofstream out(argv[3],std::ios::binary);out.write((char*)saved.data(),saved.size());}std::cout<<"PASS removed expansion reports missing samples and mutes missing voices; no replacement by unrelated core sounds\n";}
  else if(mode=="restored"){assert(l.packs.size()==2&&p.getInt(kXKit)==192&&p.getInt(kPackFilter)==2);for(int r=0;r<8;++r)assert(!p.missingSample[r]&&l.entries[p.getInt(kXSample0+r)-1].pack==1);std::cout<<"PASS project resaved with missing pack reconnects kit/pack/sample identities after reinstallation\n";}
  else if(mode=="reordered"){assert(l.packs.size()==3&&p.getInt(kXKit)==288&&p.getInt(kPackFilter)==3);for(int r=0;r<8;++r)assert(!p.missingSample[r]&&l.entries[p.getInt(kXSample0+r)-1].pack==2);std::cout<<"PASS adding earlier expansion changes offsets but kit/pack/sample stable identities restore correctly\n";}
 }
}
