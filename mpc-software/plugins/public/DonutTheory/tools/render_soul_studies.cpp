// SPDX-License-Identifier: MIT
// Original patches/phrase. Offline render only; no factory program changes.
#include "../src/DonutTheory.cpp"
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <vector>
struct Study {const char* slug;const char* name;std::vector<std::pair<int,float>> values;bool overlap;};
const Study studies[]={
 {"dusty-sunday","Keys / Dusty Sunday",{
 {kVolume,-6},{kOscPair,1},{kWave1,0},{kWave2,1},{kMix,22},{kDetune,-2.5f},{kFM,14},{kSub,0},{kNoise,0},{kTone,12},
 {kBodyTone,28},{kCutoff,1850},{kResonance,8},{kEnv,12},{kKeyTrack,30},{kFilter,0},
 {kAmpAttack,18},{kAmpDecay,1550},{kAmpSustain,27},{kAmpRelease,780},{kFiltAttack,5},{kFiltDecay,1000},{kFiltSustain,14},{kFiltRelease,700},
 {kExpression,62},{kDriveType,0},{kDrive,5},{kDriveMix,35},{kChorus,14},{kDelay,3},{kFeedback,12},{kDelayDivision,0},{kLowCut,85},
 {kDrift,4.5f},{kModMode,1},{kModShape,0},{kTremRate,.22f},{kTremDepth,18},{kModSync,0},
 {kShape,0},{kRate,.17f},{kDepth,1.8f},{kTarget,1},{kSync,0},
 {kLfo2Shape,1},{kLfo2Rate,.11f},{kLfo2Depth,4.5f},{kLfo2Target,4},{kLfo2Sync,0},
 {kSpread,8},{kGlide,0},{kChordGlide,0}},false},
 {"velvet-melt","Synth / Velvet Melt",{
 {kVolume,-6},{kOscPair,9},{kWave1,2},{kWave2,1},{kMix,35},{kDetune,-5.5f},{kFM,0},{kSub,3},{kNoise,0},{kTone,16},
 {kBodyTone,22},{kCutoff,1350},{kResonance,12},{kEnv,8},{kKeyTrack,28},{kFilter,0},
 {kAmpAttack,85},{kAmpDecay,1700},{kAmpSustain,52},{kAmpRelease,900},{kFiltAttack,75},{kFiltDecay,1300},{kFiltSustain,36},{kFiltRelease,950},
 {kExpression,58},{kDriveType,0},{kDrive,7},{kDriveMix,45},{kChorus,18},{kDelay,4},{kFeedback,10},{kDelayDivision,0},{kLowCut,100},
 {kDrift,6.5f},{kModMode,1},{kModShape,0},{kTremRate,.18f},{kTremDepth,24},{kModSync,0},
 {kShape,0},{kRate,.11f},{kDepth,3},{kTarget,1},{kSync,0},
 {kLfo2Shape,1},{kLfo2Rate,.19f},{kLfo2Depth,6},{kLfo2Target,4},{kLfo2Sync,0},
 {kSpread,6},{kGlide,65},{kChordGlide,1}},true}
};
struct Result {std::vector<float> audio;std::vector<uint8_t> state;std::vector<float> values;float peak=0;double rms=0;};
Result render(const Study& study,float trim,bool widePan=false){
 constexpr int sr=48000;const double bar=240./82.;const int frames=int((.25+bar*4+3.5)*sr);
 DonutTheory p;p.setProgram(0);for(auto [key,value]:study.values)p.set(key,value);p.set(kOutputTrim,trim);p.set(kInputMode,2);if(widePan)p.set(kTremDepth,std::string(study.slug)=="dusty-sunday"?70.f:80.f);
 const int roots[]={48,45,41,43},types[]={3,2,14,20},voices[]={0,3,4,3};
 for(int i=0;i<4;++i){p.set(kChordRoots[i],roots[i]);p.set(kChordQualities[i],types[i]);p.set(kChordVoicings[i],voices[i]);p.set(kChordOctaves[i],2);}
 p.set(kChordKeyRoot,0);p.set(kChordKeyMode,0);p.prepare(sr,512);p.reset();
 float left[512]{},right[512]{};float* outs[]={left,right};mpc::ProcessContext c;c.outputs=outs;c.numOutputs=2;c.sampleRate=sr;c.numFrames=512;
 c.transport.valid=c.transport.tempoValid=true;c.transport.bpm=82;
 for(int f=0;f<sr;f+=512){c.numFrames=std::min(512,sr-f);p.process(c);}
 Result result;result.state=p.internalSaveState();for(int i=0;i<kNumParams;++i)result.values.push_back(p.get(i));
 // Reopen the exact saved chunk, not a separate sound configuration.
 DonutTheory loaded;loaded.prepare(sr,512);if(!loaded.internalLoadState(result.state.data(),result.state.size()))throw std::runtime_error("State restore failed");loaded.reset();
 for(int i=0;i<kNumParams;++i)if(std::abs(loaded.get(i)-p.get(i))>.001f)throw std::runtime_error("Parameter round-trip differs");
 struct Event {int frame;mpc::MidiEvent midi;};std::vector<Event> events;
 const int velocities[]={78,69,74,66};const double offsets[]={0,.014,-.009,.019};
 for(int i=0;i<4;++i){int note=loaded.getInt(kChordPadNotes[i]);double start=.25+i*bar+offsets[i];double end=.25+(i+1)*bar+(study.overlap?.075:-.27);
  events.push_back({int(start*sr),mpc::MidiEvent::noteOn(0,note,velocities[i])});events.push_back({int(end*sr),mpc::MidiEvent::noteOff(0,note)});}
 std::stable_sort(events.begin(),events.end(),[](const Event&a,const Event&b){return a.frame<b.frame;});
 size_t next=0;result.audio.reserve(frames*2);double energy=0;
 for(int f=0;f<frames;f+=512){c.numFrames=std::min(512,frames-f);std::vector<mpc::MidiEvent> midi;
  while(next<events.size()&&events[next].frame<f+c.numFrames){auto event=events[next++];event.midi.sampleOffset=event.frame-f;midi.push_back(event.midi);}
  c.midi=midi.data();c.numMidiEvents=int(midi.size());c.transport.playing=true;c.transport.ppqValid=true;c.transport.ppqPos=f/48000.*82/60.;loaded.process(c);
  for(int i=0;i<c.numFrames;++i)for(float sample:{left[i],right[i]}){if(!std::isfinite(sample)||std::abs(sample)>1)throw std::runtime_error("Invalid audio");result.audio.push_back(sample);result.peak=std::max(result.peak,std::abs(sample));energy+=sample*sample;}
 }
 if(loaded.activeVoices())throw std::runtime_error("Voices did not release");result.rms=std::sqrt(energy/result.audio.size());return result;
}
float inputFor(float output){float lo=0,hi=3;for(int i=0;i<35;++i){float m=(lo+hi)*.5f;if(mpc::dsp::fastTanh(m)<output)lo=m;else hi=m;}return (lo+hi)*.5f;}
int main(int argc,char** argv){if(argc<2||argc>3)return 2;const bool widePan=argc==3&&std::string(argv[2])=="--wide-pan";if(argc==3&&!widePan)return 2;if(DonutTheory::metadata().version!=10800||kNumParams!=287){std::cerr<<"Soul studies require the reviewed1.8.0 engine\n";return 3;}std::filesystem::path dir=argv[1];std::filesystem::create_directories(dir);
 std::ofstream summary(dir/"levels.csv");summary<<"slug,trim_db,peak,rms,seconds\n";
 for(const auto& study:studies){auto first=render(study,0,widePan);float trim=std::clamp(20.f*std::log10(inputFor(.55f)/inputFor(first.peak)),-18.f,18.f);auto result=render(study,trim,widePan);
  trim=std::clamp(trim+float(20*std::log10(.065/result.rms)),-18.f,18.f);result=render(study,trim,widePan);
  auto raw=dir/(std::string(study.slug)+".f32");std::ofstream f(raw,std::ios::binary);f.write(reinterpret_cast<const char*>(result.audio.data()),result.audio.size()*sizeof(float));
  std::ofstream state(dir/(std::string(study.slug)+".mpcp"),std::ios::binary);state.write(reinterpret_cast<const char*>(result.state.data()),result.state.size());
  std::ofstream json(dir/(std::string(study.slug)+".json"));json<<std::setprecision(9)<<"{\n  \"name\": \""<<study.name<<(widePan?" Wide":"")<<"\",\n  \"engineVersion\": \"1.8.0\",\n  \"uid\": "<<DonutTheory::metadata().uniqueId<<",\n  \"parameterCount\": "<<kNumParams<<",\n  \"valuesById\": {\n";
  DonutTheory names;for(int i=0;i<kNumParams;++i)json<<"    \""<<names.param(i).id<<"\": "<<result.values[i]<<(i+1<kNumParams?",":"")<<"\n";json<<"  }\n}\n";
  summary<<study.slug<<","<<trim<<","<<result.peak<<","<<result.rms<<","<<result.audio.size()/96000.<<"\n";
  std::cout<<study.name<<": peak="<<result.peak<<" rms="<<result.rms<<" trim="<<trim<<"; 287parameters round-trip and all notes release\n";
 }
}
