#ifndef FELT_CPU_SOURCE
#define FELT_CPU_SOURCE "../src/FeltClub.cpp"
#endif
#include FELT_CPU_SOURCE
#include <chrono>
#include <fstream>
#include <iostream>
#include <vector>
#include <iomanip>
struct Bench {
 FeltClub p;float samples[2][512]{},*out[2]={samples[0],samples[1]};uint64_t clock=0;int frames=128;
 Bench(int model,int voices,bool fx,bool pedal){p.prepare(48000,512);p.set(kModel,model);p.set(kInputMode,0);p.set(kDrift,0);p.set(kHiss,0);for(int k:{kChorus,kDelay,kRoom,kDepth})p.set(k,fx?25:0);if(pedal){auto e=mpc::MidiEvent::cc(0,64,127);block(&e,1);}for(int n=0;n<voices;++n){auto e=mpc::MidiEvent::noteOn(0,48+n*2,75+n%4*9);block(&e,1);}}
 void block(const mpc::MidiEvent*e=nullptr,int n=0){mpc::ProcessContext c;c.outputs=out;c.numOutputs=2;c.numFrames=frames;c.sampleRate=48000;c.midi=e;c.numMidiEvents=n;c.transport.valid=c.transport.tempoValid=c.transport.ppqValid=true;c.transport.bpm=88;c.transport.ppqPos=clock*88./(60*48000);p.process(c);clock+=frames;}
};
int main(int argc,char**argv){if(argc!=2){std::cerr<<"Provide an existing output folder\n";return 2;}std::string dir=argv[1];std::cout<<std::setprecision(9);
 for(int m=0;m<7;++m)for(int scene=0;scene<5;++scene){int voices=scene==0?0:scene==1?1:scene==4?12:6;bool fx=scene==3,pedal=scene==4;Bench b(m,voices,fx,pedal);std::ofstream audio(dir+"/m"+std::to_string(m)+"s"+std::to_string(scene)+".f32",std::ios::binary);for(int block=0;block<1500;++block){if(block==800)for(int n=0;n<voices;++n){auto e=mpc::MidiEvent::noteOff(0,48+n*2);b.block(&e,1);}b.block();audio.write(reinterpret_cast<char*>(b.samples[0]),128*sizeof(float));audio.write(reinterpret_cast<char*>(b.samples[1]),128*sizeof(float));}audio.close();Bench timed(m,voices,fx,pedal);for(int i=0;i<200;++i)timed.block();double best=1e9,total=0;for(int run=0;run<3;++run){auto start=std::chrono::steady_clock::now();for(int i=0;i<1200;++i)timed.block();double ms=std::chrono::duration<double,std::milli>(std::chrono::steady_clock::now()-start).count();best=std::min(best,ms);total+=ms;}std::cout<<m<<","<<scene<<","<<best<<","<<total/3<<std::endl;
 }
}
