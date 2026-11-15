// Offline musical-pattern audit; emits unnormalised audio and measured levels.
#include "../src/DonutTheory.cpp"
#include <fstream>
#include <iomanip>
#include <iostream>
int main(int argc,char** argv){
 if(argc!=2&&argc!=3)return 2;
 const bool untrimmed=argc==3&&std::string(argv[2])=="--untrimmed";
 std::ofstream raw(std::string(argv[1])+"/factory-audition.f32",std::ios::binary);
 std::ofstream report(std::string(argv[1])+"/factory-audit.csv");report<<"index,name,low_cut_hz,peak,rms\n";
 for(int patch=0;patch<64;++patch){
  DonutTheory synth;synth.prepare(44100,128);synth.setProgram(patch);synth.set(kInputMode,0);if(untrimmed)synth.set(kOutputTrim,0);synth.reset();
  const std::string name=kPatchNames[patch];const bool bass=name.rfind("Bass",0)==0,lead=name.rfind("Lead",0)==0;
  const int root=bass?36:lead?72:60;
  float l[128]{},r[128]{};float* output[2]={l,r};mpc::ProcessContext ctx;ctx.outputs=output;ctx.numOutputs=2;ctx.sampleRate=44100;ctx.numFrames=128;
  mpc::MidiEvent on[4]={mpc::MidiEvent::noteOn(0,root,96),mpc::MidiEvent::noteOn(0,root+3,80),mpc::MidiEvent::noteOn(0,root+7,86),mpc::MidiEvent::noteOn(0,root+10,78)};
  mpc::MidiEvent off[4]={mpc::MidiEvent::noteOff(0,root),mpc::MidiEvent::noteOff(0,root+3),mpc::MidiEvent::noteOff(0,root+7),mpc::MidiEvent::noteOff(0,root+10)};
  double energy=0;float peak=0;int count=0;
  for(int block=0;block<1723;++block){ // 5 seconds per program: 2s hold, 3s release.
   ctx.midi=block==0?on:block==689?off:nullptr;ctx.numMidiEvents=(block==0||block==689)?(bass||lead?1:4):0;
   synth.process(ctx);
   for(int i=0;i<128;++i){const float sample=(l[i]+r[i])*.5f;if(!std::isfinite(l[i])||!std::isfinite(r[i])||std::abs(l[i])>1||std::abs(r[i])>1)return 3;raw.write(reinterpret_cast<const char*>(&sample),sizeof sample);peak=std::max(peak,std::max(std::abs(l[i]),std::abs(r[i])));if(block<689){energy+=(double(l[i])*l[i]+double(r[i])*r[i])*.5;++count;}}
  }
  if(peak<1e-5)return 4;
  report<<patch<<",\""<<name<<"\","<<kPatchExtras[patch][0]<<","<<peak<<","<<std::sqrt(energy/count)<<"\n";
 }
 std::cout<<"PASS: all 64 factory sounds rendered; finite audio, non-silence, full pattern levels recorded\n";
}
