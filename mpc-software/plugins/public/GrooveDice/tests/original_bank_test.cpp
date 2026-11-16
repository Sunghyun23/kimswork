#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
static thread_local bool guarded=false;
void* operator new(size_t n){if(guarded)std::abort();if(auto*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
int main(){GrooveDice p;auto&l=p.library;assert(p.bank.loaded&&l.loaded&&l.entries.size()==223&&l.categories[0].size()==128&&l.categories[3].size()==2&&l.categories[4].size()==93);assert(l.factoryKits.size()==96);int sizes[]={440,546,376,244};for(int g=0;g<4;++g)assert(l.soundPools[g].size()==size_t(sizes[g]));
 for(int role=0;role<8;++role)for(int variant=0;variant<8;++variant){auto&s=p.bank.sounds[role*8+variant];assert(s.numFrames()>256&&s.numChannels()==1&&s.sampleRate==44100);}
 p.prepare(44100,512);p.reset();float data[16][512]{},*out[16];for(int ch=0;ch<16;++ch)out[ch]=data[ch];double energy=0,peak=0;
 auto block=[&](){for(auto&ch:data)std::fill(ch,ch+512,0);mpc::ProcessContext c;c.outputs=out;c.numOutputs=16;c.numFrames=128;c.sampleRate=44100;guarded=true;p.process(c);guarded=false;for(auto&ch:data)for(int j=0;j<128;++j){assert(std::isfinite(ch[j]));peak=std::max(peak,double(std::abs(ch[j])));energy+=ch[j]*ch[j];}};
 for(int kit=1;kit<=96;++kit){p.selectFactoryProgram(kit);bool ready=false;for(int i=0;i<1500;++i){block();ready=true;for(int r=0;r<8;++r)ready&=l.active[r].load()==p.getInt(kLibrarySample0+r);if(ready)break;std::this_thread::sleep_for(std::chrono::milliseconds(1));}assert(ready);for(int r=0;r<8;++r){assert(!p.missingSample[r]);p.set(kPad0+r*9,1);block();p.set(kPad0+r*9,0);}for(int i=0;i<30;++i)block();}
 assert(peak>0&&peak<1&&energy>1);auto state=p.internalSaveState();p.selectFactoryProgram(1);assert(p.internalLoadState(state.data(),state.size()));p.reset();for(int i=0;i<180;++i)block();auto restored=p.internalSaveState();if(restored!=state){std::ofstream a("/tmp/gd090-state-before.bin",std::ios::binary),b("/tmp/gd090-state-after.bin",std::ios::binary);a.write((char*)state.data(),state.size());b.write((char*)restored.data(),restored.size());}assert(restored==state);
 for(int g=0;g<4;++g)for(int id:{1,sizes[g]}){p.set(kSoundKick+g,id);for(int j=0;j<10;++j){block();std::this_thread::sleep_for(std::chrono::milliseconds(1));}assert(p.getInt(kSoundKick+g)==id);}
 std::cout<<"PASS original128WAV/95MIDI/96kits/1606soundchoices; prefix64; all96kit8voices finite/bounded output; no audio allocation; state byte-identical; peak="<<peak<<"\n";
}
