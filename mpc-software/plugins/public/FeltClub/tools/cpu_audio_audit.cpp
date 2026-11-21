#ifndef FELT_CPU_SOURCE
#define FELT_CPU_SOURCE "../src/FeltClub.cpp"
#endif
#include FELT_CPU_SOURCE
#include <fstream>
#include <iostream>
#include <vector>
int main(int argc,char**argv){
 if(argc!=2){std::cerr<<"Provide an existing output folder\n";return 2;}
 for(int rate:{44100,48000,96000})for(int model=0;model<7;++model)for(int scene=0;scene<2;++scene){
  FeltClub p;p.prepare(rate,512);p.set(kModel,model);p.set(kInputMode,scene?2:0);p.set(kDrift,9);p.set(kChordGlide,1);p.set(kGlide,90);p.set(kStrum,75);p.set(kHiss,0);for(int k:{kDepth,kChorus,kDelay,kRoom})p.set(k,0);
  std::ofstream file(std::string(argv[1])+"/r"+std::to_string(rate)+"m"+std::to_string(model)+"s"+std::to_string(scene)+".f32",std::ios::binary);
  uint64_t clock=0;float samples[2][512]{},*out[]={samples[0],samples[1]};
  for(int block=0;block<1000;++block){int frames=std::array<int,4>{32,97,128,511}[block%4];std::vector<mpc::MidiEvent>events;
   if(block==0){if(scene)events.push_back(mpc::MidiEvent::noteOn(0,24,75,7));else for(int n:{48,55,60,64,67,71})events.push_back(mpc::MidiEvent::noteOn(0,n,60+n%6*10,7));}
   if(block==40){p.set(kChorus,25);p.set(kDelay,20);p.set(kRoom,15);}
   if(block==100)events.push_back(mpc::MidiEvent::cc(0,64,127,3));
   if(block==120){mpc::MidiEvent e{};e.data[0]=0xe0;e.data[1]=0;e.data[2]=80;e.sampleOffset=9;events.push_back(e);}
   if(block==180){p.set(kTone,80);if(scene)events.push_back(mpc::MidiEvent::noteOn(0,26,96,11));}
   if(block==185&&scene)events.push_back(mpc::MidiEvent::noteOff(0,24,17));
   if(block==230){mpc::MidiEvent e{};e.data[0]=0xe0;e.data[1]=0;e.data[2]=64;e.sampleOffset=9;events.push_back(e);}
   if(block==300){for(int k:{kChorus,kDelay,kRoom})p.set(k,0);if(scene)events.push_back(mpc::MidiEvent::noteOff(0,26,21));else for(int n:{48,55,60,64,67,71})events.push_back(mpc::MidiEvent::noteOff(0,n,21));}
   if(block==380)events.push_back(mpc::MidiEvent::cc(0,64,0,5));
   if(block==420){p.set(kDelay,20);p.set(kRoom,15);}
   if(block==500){p.set(kModel,(model+1)%7);if(scene){events.push_back(mpc::MidiEvent::noteOn(0,28,90,2));events.push_back(mpc::MidiEvent::noteOff(0,28,19));}else for(int n=0;n<30;++n)events.push_back(mpc::MidiEvent::noteOn(0,36+n,72,2));}
   if(block==650)events.push_back(mpc::MidiEvent::cc(0,123,0,3));
   if(block==800){p.set(kPanic,1);p.service(true);}
   mpc::ProcessContext c;c.outputs=out;c.numOutputs=2;c.numFrames=frames;c.sampleRate=rate;c.midi=events.data();c.numMidiEvents=int(events.size());c.transport.valid=c.transport.tempoValid=c.transport.ppqValid=true;c.transport.bpm=88;c.transport.ppqPos=clock*88./(60*rate);p.process(c);clock+=frames;
   file.write(reinterpret_cast<char*>(samples[0]),frames*sizeof(float));file.write(reinterpret_cast<char*>(samples[1]),frames*sizeof(float));
  }
  std::cout<<rate<<','<<model<<','<<scene<<std::endl;
 }
}
