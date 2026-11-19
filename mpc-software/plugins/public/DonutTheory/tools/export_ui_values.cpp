// Runtime values for approximate skin previews: never claim a hardware capture.
#include "../src/DonutTheory.cpp"
#include <iostream>
int main(){DonutTheory p;p.prepare(48000,128);p.setProgram(64);float l[128]{},r[128]{};float* out[]={l,r};mpc::ProcessContext ctx;ctx.sampleRate=48000;ctx.outputs=out;ctx.numOutputs=2;ctx.numFrames=128;for(int i=0;i<4;++i)p.process(ctx);for(int i=0;i<kNumParams;++i){std::string text;if(!p.formatValue(i,p.get(i),text))text=p.param(i).display(p.get(i));std::cout<<i<<'\t'<<p.getNormalised(i)<<'\t'<<text<<'\n';}}
