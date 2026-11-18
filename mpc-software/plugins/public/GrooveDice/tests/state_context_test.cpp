#define main oldLayers
#include "layers_test.cpp"
#undef main
#include <random>
int main(){Host h;auto&p=*h.p;std::mt19937 rng(1234);std::uniform_real_distribution<float>uni(0,1);for(int n=0;n<200;++n){int id=rng()%kParamCount;float v=uni(rng);if(n%7==0)v=n%14==0?0:1;p.setNormalised(id,v);h.render();}for(int n=0;n<5;++n)h.render();auto state=p.internalSaveState();std::array<float,kParamCount>values;for(int n=0;n<kParamCount;++n)values[n]=p.getNormalised(n);for(int n=0;n<kParamCount;++n)p.setNormalised(n,uni(rng));h.render();h.render();assert(p.internalLoadState(state.data(),state.size()));p.reset();h.render();h.render();auto after=p.internalSaveState();assert(after==state);std::cout<<"PASS seeded200parameter sweep restores browser folder/context and relative Q-Link accumulators byte-identically\n";}
