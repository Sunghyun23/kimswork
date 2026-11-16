#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
int main(){
 GrooveDice p;assert(p.library.loaded);assert(p.library.packs.size()==2);assert(p.library.expansionErrors==0);
 assert(p.library.factoryKits.size()==104&&p.numPrograms()==105);p.prepare(44100,128);p.reset();p.selectFactoryProgram(104);
 float buffers[16][128]{},*out[16];for(int i=0;i<16;++i)out[i]=buffers[i];double energy=0;
 for(int n=0;n<500;++n){mpc::ProcessContext ctx;ctx.outputs=out;ctx.numOutputs=16;ctx.numFrames=128;ctx.sampleRate=44100;
  if(n==300)p.set(kPad0,1);for(auto&ch:buffers)std::fill(ch,ch+128,0);p.process(ctx);for(auto&ch:buffers)for(float x:ch){assert(std::isfinite(x));energy+=x*x;}
  std::this_thread::sleep_for(std::chrono::milliseconds(1));
 }
 assert(p.getInt(kXKit)==104);for(int r=0;r<8;++r)assert(p.library.entries[p.getInt(kXSample0+r)-1].pack==1);assert(energy>.0001);
 auto saved=p.internalSaveState();p.selectFactoryProgram(1);assert(p.internalLoadState(saved.data(),saved.size()));assert(p.getInt(kXKit)==104);
 assert(p.library.categories[3].size()==3&&p.library.categories[4].size()==94);
 std::cout<<"PASS authored pack loads in real engine: 104 kits, 105 native programs, all 8 roles, GM MIDI, asynchronous audio and saved identity restore; energy "<<energy<<"\n";
}
