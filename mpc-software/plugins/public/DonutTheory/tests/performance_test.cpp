#include <cassert>
#include <cmath>
#include <iostream>
#include <memory>
#include <array>
#define DONUT_VOICE_TEST
#include "../src/DonutTheory.cpp"
struct Rig{
 std::unique_ptr<DonutTheory> p=std::make_unique<DonutTheory>();float l[512]{},r[512]{};float* outputs[2]={l,r};
 Rig(){p->set(kInputMode,0);p->prepare(48000,512);block();}
 void block(std::initializer_list<mpc::MidiEvent> midi={},int frames=127){mpc::ProcessContext c;c.sampleRate=48000;c.outputs=outputs;c.numOutputs=2;c.numFrames=frames;c.midi=midi.begin();c.numMidiEvents=int(midi.size());p->process(c);for(int i=0;i<frames;++i)assert(std::isfinite(l[i])&&std::abs(l[i])<=1&&std::isfinite(r[i]));}
};
static std::vector<uint8_t> old353(DonutTheory& p){auto state=p.internalSaveState();size_t end=12;for(int i=0;i<353;++i)end+=1+state[end]+8;state.resize(end+4);for(int b=0;b<4;++b)state[end+b]=0;state[8]=353&255;state[9]=353>>8;state[10]=state[11]=0;return state;}
int main(){
 static_assert(kOscTune1==353&&kNumParams==391,"append-only performance surface");
 {Rig m;m.p->set(kMix,32);m.p->set(kDetune,-7);m.p->set(kOctave,-1);m.p->set(kPulse,34);auto old=old353(*m.p);Rig r;assert(r.p->internalLoadState(old.data(),old.size()));r.block();assert(std::abs(r.p->get(kOscLevel1)-68)<.001&&std::abs(r.p->get(kOscLevel2)-32)<.001);assert(std::abs(r.p->get(kOscFine2)+7)<.001&&r.p->getInt(kOscTune2)==-12);assert(std::abs(r.p->get(kPulse2)-34)<.001);assert(r.p->get(kLfoDelay)==0&&r.p->get(kLfo2Delay)==0);}
 std::cout<<"PASS: legacy353 migration preserves oscillator ratio, tune, fine and pulse; Delay defaults zero\n";
 {Rig m;m.p->set(kOscFine1,5);m.p->set(kOscFine2,-3);m.p->set(kOscTune1,12);m.p->set(kOscTune2,-12);m.p->set(kOscLevel1,70);m.p->set(kOscLevel2,30);m.p->set(kLfoDelay,150);m.p->set(kLfo2Delay,420);m.p->set(kVoiceMode,2);m.block({mpc::MidiEvent::noteOn(0,60,100)});
  const auto before=m.p->voiceSnapshots();const auto phases=m.p->modulationPhases();
  for(int osc=0;osc<2;++osc)for(int fine=0;fine<2;++fine)for(int lfo=0;lfo<2;++lfo){m.p->set(kOscFocus,osc);m.p->set(kPitchResolution,fine);m.p->set(kLfoFocus,lfo);m.block();assert(m.p->getInt(kVoiceMode)==2);assert(m.p->getInt(kPerfWave)==m.p->getInt(osc?kWave2:kWave1));assert(std::abs(m.p->get(kPerfLevel)-(osc?30:70))<.001);assert(std::abs(m.p->get(kPerfLfoDelay)-(lfo?420:150))<.001);const auto now=m.p->voiceSnapshots();assert(now[0].owner==before[0].owner&&now[0].held);}
  (void)phases;assert(std::abs(m.p->get(kOscFine1)-5)<.001&&std::abs(m.p->get(kOscFine2)+3)<.001);assert(m.p->getInt(kOscTune1)==12&&m.p->getInt(kOscTune2)==-12);
  m.p->set(kOscPulse1,24);m.p->set(kPulse2,71);m.p->set(kOscPulse1,39);assert(std::abs(m.p->get(kPulse2)-71)<.001);
  m.p->set(kOscFocus,1);m.p->set(kPitchResolution,1);m.p->set(kPerfPitch,m.p->param(kOscFine2).toNormalised(17));assert(std::abs(m.p->get(kOscFine2)-17)<.001&&m.p->getInt(kOscTune2)==-12&&std::abs(m.p->get(kOscFine1)-5)<.001);
  m.p->set(kPerfLevel,42);assert(std::abs(m.p->get(kOscLevel2)-42)<.001&&m.p->get(kOscLevel1)==70);
  m.p->set(kLfoFocus,1);m.p->set(kPerfLfoDelay,700);assert(std::abs(m.p->get(kLfo2Delay)-700)<.001&&std::abs(m.p->get(kLfoDelay)-150)<.001);
  m.p->set(kPerfLfoTarget,4);assert(m.p->getInt(kLfo2Target)==4);m.p->set(kLfoFocus,0);m.p->set(kPerfLfoTarget,4);m.p->set(kPerfLfoShape,2);assert(m.p->getInt(kLfo1TargetFull)==4&&m.p->getInt(kLfo1ShapeFull)==2);
  auto state=m.p->internalSaveState();Rig r;assert(r.p->internalLoadState(state.data(),state.size()));r.p->reset();r.block();assert(std::abs(r.p->get(kOscFine2)-17)<.001&&r.p->getInt(kOscTune2)==-12);assert(std::abs(r.p->get(kLfo2Delay)-700)<.001&&r.p->getInt(kLfo1ShapeFull)==2);
 }
 std::cout<<"PASS: OSC/LFO focus and Tune/Fine selection preserve sound/mode/held voices; relays route independently and state restores\n";
 {Rig m;m.p->set(kVoiceMode,2);m.p->set(kMonoLegato,1);m.p->set(kPerfVoice,0);assert(m.p->getInt(kVoiceMode)==1);m.p->set(kPerfVoice,1);assert(m.p->getInt(kVoiceMode)==2);m.p->set(kMonoLegato,0);assert(m.p->getInt(kVoiceMode)==2);m.p->set(kPerfVoice,0);assert(m.p->getInt(kVoiceMode)==0);
  for(int mode=0;mode<3;++mode){m.p->set(kVoiceMode,mode);m.p->set(kLfoDelay,650);m.p->set(kLfo2Delay,320);m.p->set(kDiceValueLockOscLevel1,1);m.p->set(kOscLevel1,73);m.p->set(kDiceValueLockOscFine2,1);m.p->set(kOscFine2,-13);
   for(int roll=0;roll<20;++roll){m.p->set(kRandomize,1);m.block();assert(m.p->getInt(kVoiceMode)==mode);assert(std::abs(m.p->get(kOscLevel1)-73)<.001&&std::abs(m.p->get(kOscFine2)+13)<.001);assert(std::abs(m.p->get(kLfoDelay)-650)<.001&&std::abs(m.p->get(kLfo2Delay)-320)<.001);}
  }
  std::array<float,kNumParams> previous{},normalised{};for(int i=0;i<kNumParams;++i){previous[i]=m.p->get(i);normalised[i]=m.p->getNormalised(i);}m.p->set(kRandomize,1);m.block();m.p->set(kUndoDice,1);m.block();for(int i=0;i<kNumParams;++i)if(diceValueLock(i)>=0){if(std::abs(previous[i]-m.p->get(i))>=.001)std::cerr<<"Undo mismatch "<<i<<" "<<m.p->param(i).name<<" "<<previous[i]<<" -> "<<m.p->get(i)<<"\n";assert(previous[i]==m.p->get(i));assert(normalised[i]==m.p->getNormalised(i));}assert(m.p->getInt(kVoiceMode)==2);
  m.p->set(kVoiceMode,0);m.p->setProgram(64);m.block();assert(m.p->getInt(kVoiceMode)==0&&m.p->get(kLfoDelay)==0&&m.p->get(kLfo2Delay)==0);
 }
 std::cout<<"PASS: explicit mode toggle,60roll mode/Delay retention, new lock protection through legacy aliases, one-step sound Undo and factory mode retention\n";
 {Rig delayed,dry;for(auto r:{&delayed,&dry}){r->p->setProgram(0);r->p->set(kLfo2Depth,0);r->p->set(kMotion,0);r->p->set(kVibrato,0);r->p->set(kNoise,0);r->p->set(kLfo1TargetFull,0);r->p->set(kDepth,0);r->block();}delayed.p->set(kDepth,95);delayed.p->set(kLfoDelay,300);for(int n=0;n<100;++n){delayed.block();dry.block();}
  bool laterDiff=false;for(int n=0;n<220;++n){if(n==0){delayed.block({mpc::MidiEvent::noteOn(0,60,100)});dry.block({mpc::MidiEvent::noteOn(0,60,100)});}else{delayed.block();dry.block();}float difference=0;for(int i=0;i<127;++i)difference+=std::abs(delayed.l[i]-dry.l[i]);if(n<110)assert(difference<.00001);if(n>140&&difference>.0001)laterDiff=true;}assert(laterDiff);
  delayed.block({mpc::MidiEvent::noteOff(0,60)});for(int n=0;n<220;++n)delayed.block();assert(delayed.p->activeVoices()==0);
 }
 std::cout<<"PASS: note-on LFO delay keeps pitch unmodulated for300ms, fades modulation afterward and releases normally\n";
 {Rig changing,reference;for(auto r:{&changing,&reference}){r->p->setProgram(64);r->block();r->block({mpc::MidiEvent::noteOn(0,60,100)});}
  for(int n=0;n<64;++n){changing.p->set(kOscFocus,n%2);changing.p->set(kPitchResolution,(n/2)%2);changing.p->set(kLfoFocus,(n/4)%2);changing.block();reference.block();for(int i=0;i<127;++i)assert(changing.l[i]==reference.l[i]&&changing.r[i]==reference.r[i]);}
  changing.p->set(kOscLevel1,63);changing.p->set(kOscLevel2,42);changing.p->set(kOscFine2,17);changing.p->set(kPulse2,71);auto state=changing.p->internalSaveState();Rig restored;assert(restored.p->internalLoadState(state.data(),state.size()));restored.p->reset();for(int key:{kMix,kDetune,kOctave,kPulse})restored.p->setNormalised(key,restored.p->getNormalised(key));restored.block();assert(std::abs(restored.p->get(kOscLevel1)-63)<.001&&std::abs(restored.p->get(kOscLevel2)-42)<.001&&std::abs(restored.p->get(kOscFine2)-17)<.001&&std::abs(restored.p->get(kPulse2)-71)<.001);
 }
 std::cout<<"PASS:64focus/resolution changes produce sample-identical stereo to an untouched playing reference; legacy normalized echoes cannot overwrite new saved OSC values\n";
 {Rig delayed,dry;for(auto r:{&delayed,&dry}){r->p->setProgram(0);r->p->set(kDepth,0);r->p->set(kLfo2Depth,0);r->p->set(kLfo2Target,0);r->block();}delayed.p->set(kLfo2Depth,95);delayed.p->set(kLfo2Delay,300);for(int n=0;n<100;++n){delayed.block();dry.block();}bool later=false;
  for(int n=0;n<220;++n){if(n==0){delayed.block({mpc::MidiEvent::noteOn(0,60,100)});dry.block({mpc::MidiEvent::noteOn(0,60,100)});}else{delayed.block();dry.block();}float diff=0;for(int i=0;i<127;++i)diff+=std::abs(delayed.l[i]-dry.l[i]);if(n<110)assert(diff<.00001);if(n>140&&diff>.0001)later=true;}assert(later);
 }
 std::cout<<"PASS:independent LFO2 delay also keeps pitch unmodulated for300ms then fades in\n";
 for(int test=0;test<3;++test){Rig m;m.p->setProgram(0);m.block();for(int key:{kSub,kNoise,kFM,kTone,kDepth,kLfo2Depth,kDrift,kMotion,kVibrato,kChorus,kDelay,kSpace,kTremDepth})m.p->set(key,0);m.p->set(kWave1,0);m.p->set(kWave2,0);m.p->set(kOscTune1,12);m.p->set(kOscTune2,-12);m.p->set(kOscFine1,test==2?50:0);m.p->set(kOscLevel1,test==1?0:100);m.p->set(kOscLevel2,test==1?100:0);m.p->set(kAmpSustain,100);for(int n=0;n<120;++n)m.block();m.block({mpc::MidiEvent::noteOn(0,57,100)},512);int crosses=0;float previous=0;for(int n=0;n<64;++n){m.block({},512);if(n>=6)for(int i=0;i<512;++i){if(previous<=0&&m.l[i]>0)++crosses;previous=m.l[i];}}float hz=crosses/(58.f*512/48000);float expected=test==1?110:test==2?452.893f:440;assert(std::abs(hz-expected)<3);}
 std::cout<<"PASS:rendered OSC1/OSC2 octave isolation440/110Hz and independent +50cent452.9Hz fine tuning\n";

}
