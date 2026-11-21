#define main original_engine_main
#include "engine_test.cpp"
#undef main
int main(){Host h;auto&p=*h.p;p.applyProgram(9);p.set(kInputMode,0);p.set(kCommandPads,1);p.set(kCommandLearn,3);auto rng=p.randomState.load();for(int n:{24,25,26}){h.on(n);h.off(n);}assert(p.getInt(kPreset)==10&&p.randomState.load()==rng&&!p.haveUndo);assert(h.active()>0);std::cout<<"PASS legacy pad-command settings cannot consume instrument notes or change presets/Dice\n";}
