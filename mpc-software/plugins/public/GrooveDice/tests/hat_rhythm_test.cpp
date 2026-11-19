#define main existingMusicalMain
#include "musical_test.cpp"
#undef main
int main(){
 Host h;auto&p=*h.p;assert(kHatRhythm==294&&kHatPhraseState==295&&kMixKick==342&&kParamCount==782);
 for(int g=0;g<4;++g)p.set(kRuleKick+g,1);
 // Touch/automation and SCREEN relay select exactly the same15 choices; Duck stays independent.
 float duck=p.get(kDuck);
 for(int v=0;v<15;++v){p.setNormalised(kQ13,v/23.f);h.block();assert(p.getInt(kHatRhythm)==v&&p.getInt(kHatRhythmPage)==v/8);assert(p.get(kDuck)==duck);}
 assert(p.proxyTarget(12)==kHatRhythmFull);p.set(kEditor,1);assert(p.proxyTarget(15)==-1);p.set(kEditor,0);
 for(int v=0;v<15;++v){
  p.set(kHatRhythm,v);for(int attempt=0;attempt<20;++attempt){p.set(kRandomAmt,attempt%2?100:10);h.pulse(kRoll);assert(p.lengths[4].load()==3840);assert(((p.getInt(kHatPhraseState)&15)-1)==v);int snare=0,hats=0;
   static const int strides[]={960,480,240,720,360};int stride=strides[v/3];
   for(auto&c:p.notes[4])if(c.valid()){auto n=c.read();if(n.role==0)assert(n.tick%3840!=960&&n.tick%3840!=2880);if(n.role==1)++snare;if(n.role==2){++hats;if(v%3==0)assert(n.tick%stride==0);}}
   assert(snare==2&&hats>0&&p.countNotes(4)<=128);if(v%3)assert(hats>(3839/stride+1));
  }
  // Scheduler must deliver all recorded hats, including dotted phase across barlines and rolls.
  auto events=h.phrase();int actual=0,expected=0;for(auto&c:p.notes[4])if(c.valid()&&c.read().role==2)++expected;
  for(auto&e:events)if(std::get<1>(e)==2)++actual;
  assert(actual==4*expected);
  auto before=notes(p);int metadata=p.getInt(kHatPhraseState);auto state=p.internalSaveState();p.set(kHatRhythm,(v+1)%15);h.pulse(kRoll);h.pulse(kUndo);assert(notes(p)==before&&p.getInt(kHatPhraseState)==metadata);
  assert(p.internalLoadState(state.data(),state.size()));p.reset();for(int i=0;i<180;++i)h.block();assert(p.getInt(kHatRhythm)==v&&((p.getInt(kHatPhraseState)&15)-1)==v&&notes(p)==before&&p.internalSaveState()==state);
 }
 // Changing selection updates MAIN immediately; LOCK overrides an explicit new rhythm.
 auto before=notes(p);int actual=((p.getInt(kHatPhraseState)&15)-1);p.set(kHatRhythm,0);assert(notes(p)==before&&((p.getInt(kHatPhraseState)&15)-1)==actual);p.set(kRuleHat,0);auto old=notes(p);h.pulse(kRoll);auto hats=[](auto n){decltype(n)v;for(auto&e:n)if(std::get<1>(e)==2||std::get<1>(e)==3)v.push_back(e);return v;};assert(hats(notes(p))==hats(old)&&((p.getInt(kHatPhraseState)&15)-1)==actual);
 // Legacy294-param chunks retain audible notes/grid and migrate the selected setting only.
 p.set(kRuleHat,1);p.set(kHatGrid,3);h.pulse(kRoll);auto saved=p.internalSaveState();std::vector<uint8_t>oldState(saved.begin(),saved.begin()+12);oldState[8]=38;oldState[9]=1;oldState[10]=oldState[11]=0;size_t pos=12;for(int i=0;i<kParamCount;++i){size_t end=pos+1+saved[pos]+8;if(i<294)oldState.insert(oldState.end(),saved.begin()+pos,saved.begin()+end);pos=end;}oldState.insert(oldState.end(),saved.begin()+pos,saved.end());before=notes(p);assert(p.internalLoadState(oldState.data(),oldState.size()));p.reset();for(int i=0;i<180;++i)h.block();assert(p.getInt(kHatRhythm)==8&&p.getInt(kHatPhraseState)==0&&notes(p)==before);
 {Host imported;auto&q=*imported.p;q.set(kHatRhythm,11);imported.pulse(kRoll);assert(q.getInt(kHatPhraseState)!=0);q.set(kRuleHat,0);auto&m=q.library.midi[q.library.categories[3][0]];m.length=3840;m.notes={{0,62,70,50},{720,62,60,50},{1440,62,60,50}};q.set(kLibraryBeat,1);imported.pulse(kRoll);assert((q.getInt(kHatPhraseState)&15)==0);std::vector<int>ticks;for(auto&c:q.notes[4])if(c.valid()&&c.read().role==2)ticks.push_back(c.read().tick);assert(ticks==std::vector<int>({0,720,1440}));}
 {Host banks;auto&q=*banks.p;for(int b=0;b<3;++b){q.set(kPattern,b);q.set(kHatRhythm,9+b);banks.pulse(kRoll);}int meta=q.getInt(kHatPhraseState);assert((meta&15)==10&&((meta>>4)&15)==11&&((meta>>8)&15)==12);}
 std::cout<<"PASS15HatRhythms/300ROLLs: dot phase, fill/roll responses, scheduled counts, SCREEN relay, Duck preserved, LOCK, Undo, byte-identical state and294-param compatibility; no audio allocations\n";
}
