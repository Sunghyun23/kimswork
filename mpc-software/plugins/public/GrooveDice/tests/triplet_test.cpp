#define main legacyMusicalMain
#include "musical_test.cpp"
#undef main
static auto exactPhrase(Host& h){
 std::vector<std::tuple<double,int,int>> result;h.playing=true;h.ppq=0;h.p->reset();
 while(h.ppq<32){double start=h.ppq;h.block();for(auto e:RuleProbe::events(*h.p,start,h.bpm/(60*44100)))if(std::get<0>(e)>=16-1.e-8&&std::get<0>(e)<32-1.e-8)result.push_back(e);}
 h.playing=false;h.block();return result;
}
static std::vector<uint8_t> legacyChunk(GrooveDice&p){
 auto state=p.internalSaveState();std::vector<uint8_t> out(state.begin(),state.begin()+12);out[8]=779&255;out[9]=779>>8;out[10]=out[11]=0;size_t pos=12;
 for(int i=0;i<kParamCount;++i){size_t end=pos+1+state[pos]+8;if(i<779)out.insert(out.end(),state.begin()+pos,state.begin()+end);pos=end;}
 out.insert(out.end(),state.begin()+pos,state.end());return out;
}
int main(){
 static_assert(kHatRhythmFull==779&&kHatPhraseFullState==780&&kHatRhythmFullPage==781&&kParamCount==782);
 Host h;auto&p=*h.p;for(int g=0;g<4;++g)p.set(kRuleKick+g,1);p.set(kRuleHatStyle,1);p.set(kVariation,0);
 for(int rhythm=0;rhythm<24;++rhythm){
  p.setNormalised(kQ13,rhythm/23.f);h.block();assert(p.getInt(kHatRhythmFull)==rhythm&&p.getInt(kHatRhythmFullPage)==rhythm/8);
  assert(p.proxyTarget(12)==kHatRhythmFull&&diceValueLock(kHatRhythmFull)==kDiceLock12);
  h.pulse(kRoll);auto savedNotes=notes(p);assert(((p.getInt(kHatPhraseFullState)&31)-1)==rhythm);
  static const int strides[]={960,480,240,720,360,640,320,160};int expected=0;
  for(auto&c:p.notes[4])if(c.valid()){auto n=c.read();if(n.role==2){++expected;if(rhythm%3==0)assert(n.tick%strides[rhythm/3]==0);}if(n.role==0)assert(n.tick!=960&&n.tick!=2880);}
  if(rhythm%3==0)assert(expected==(3839/strides[rhythm/3]+1));
  for(double bpm:{88.,120.}){h.bpm=bpm;auto events=exactPhrase(h);int actual=0;for(auto&e:events)if(std::get<1>(e)==2){++actual;if(rhythm%3==0){double ticks=std::fmod(std::get<0>(e)+1.e-7,4.)*960.;double error=std::min(std::abs(ticks-std::round(ticks/strides[rhythm/3])*strides[rhythm/3]),3840-ticks);if(error>=.1)std::cerr<<"offgrid rhythm="<<rhythm<<" ticks="<<ticks<<" bpm="<<bpm<<" error="<<error<<"\n";assert(error<.1);}}if(actual!=expected*4)std::cerr<<"rhythm="<<rhythm<<" bpm="<<bpm<<" expected="<<expected*4<<" actual="<<actual<<"\n";assert(actual==expected*4);}
  auto state=p.internalSaveState();p.set(kHatRhythmFull,(rhythm+1)%24);h.pulse(kRoll);h.pulse(kUndo);assert(notes(p)==savedNotes&&((p.getInt(kHatPhraseFullState)&31)-1)==rhythm);
  assert(p.internalLoadState(state.data(),state.size()));p.reset();for(int i=0;i<180;++i)h.block();assert(p.getInt(kHatRhythmFull)==rhythm&&((p.getInt(kHatPhraseFullState)&31)-1)==rhythm&&notes(p)==savedNotes&&p.internalSaveState()==state);
 }
 // Every triplet/RULE combination remains bounded and allocation-free in playback.
 for(int style=1;style<=16;++style)for(int rhythm=15;rhythm<24;++rhythm){p.set(kRulePickFullHat,style-1);p.set(kOpenHatRule,0);p.set(kHatRhythmFull,rhythm);assert(p.getInt(kRulePickFullHat)==style-1);h.block();h.pulse(kRoll);assert(p.countNotes(4)<=128);for(auto&c:p.notes[4])if(c.valid())assert(c.read().tick>=0&&c.read().tick<3840);int expected=0;for(auto&c:p.notes[4])if(c.valid()&&c.read().role==2)++expected;int actual=0;for(auto&e:exactPhrase(h))if(std::get<1>(e)==2)++actual;if(actual!=expected*4)std::cerr<<"style="<<style<<" rhythm="<<rhythm<<" expected="<<expected*4<<" actual="<<actual<<" open="<<p.getInt(kOpenHatRule)<<"\n";assert(actual==expected*4);}
 for(int code=25;code<32;++code){p.set(kHatPhraseFullState,code|(code<<5)|(code<<10));h.playing=true;for(int n=0;n<512;++n)h.block();}h.playing=false;h.block();
 for(int frames:{32,128,512})for(double bpm:{88.,98.,120.}){h.frames=frames;h.bpm=bpm;p.set(kRulePickFullHat,4);p.set(kHatRhythmFull,21);h.block();h.pulse(kRoll);int count=0;for(auto&e:exactPhrase(h))if(std::get<1>(e)==2)++count;assert(count==96);}h.frames=128;h.bpm=120;
 // Legacy779 snapshots migrate three banks independently without replacing any notes.
 p.set(kRuleHatStyle,1);for(int b=0;b<3;++b){p.set(kPattern,b);p.set(kHatRhythm,9+b);h.pulse(kRoll);}auto old=legacyChunk(p);auto before=notes(p);p.set(kHatRhythmFull,23);assert(p.internalLoadState(old.data(),old.size()));p.reset();for(int i=0;i<180;++i)h.block();assert(p.getInt(kHatRhythmFull)==11&&notes(p)==before);for(int b=0;b<3;++b)assert((((p.getInt(kHatPhraseFullState)>>(5*b))&31)-1)==9+b);
 // New three-bank metadata can represent all24 choices; old fields never overflow.
 for(int b=0;b<3;++b){p.set(kPattern,b);p.set(kHatRhythmFull,15+b*3);h.pulse(kRoll);}for(int b=0;b<3;++b)assert((((p.getInt(kHatPhraseFullState)>>(5*b))&31)-1)==15+b*3);assert(p.getInt(kHatPhraseState)==0);
 // Locked D1 survives Dice; manual choice stays live, preserves kick/snare/perc and fills.
 p.set(kPattern,0);p.set(kHatRhythmFull,18);h.block();int fillCount=p.countNotes(0);auto rolls=p.diceRolls.load();p.set(kHatRhythmFull,21);h.block();assert(((p.getInt(kHatPhraseFullState)&31)-1)==21&&p.diceRolls.load()==rolls&&p.countNotes(0)==fillCount);for(int n=0;n<40;++n){h.pulse(kRoll);assert(p.getInt(kHatRhythmFull)==21);}p.set(kDiceLock12,0);unsigned reached=0;for(int n=0;n<1000;++n){h.pulse(kRoll);reached|=1u<<p.getInt(kHatRhythmFull);}assert((reached>>15)==511);
 std::cout<<"PASS24 rhythms:88/120BPM exact scheduled grids,144triplet/RULE combinations, old779 migration, three banks, state/Undo, live edits, locks and Dice triplets; no render allocations\n";
}
