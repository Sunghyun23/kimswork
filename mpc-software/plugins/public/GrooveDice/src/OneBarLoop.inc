// One-bar MAIN construction. FILL A-D keep their independent recorded lengths.
// Fixed arrays, immutable source MIDI and deterministic per-part seeds: no RT allocation.
void syncRuleStyle(int group){
 static const int intensity[]={0,1,2,3,1,2,2,3};int style=mpc::clamp(getInt(kRuleKickStyle+group),0,7);
 bool old=internal;internal=true;set(kRuleKick+group,intensity[style]);internal=old;
 // Physical Q-Link sends fine increments within one discrete choice. Those
 // are not new musical settings and must not rebuild MAIN or echo all audio.
 if(style==0)layerSet(kDiceLock4+group,1);
 syncRuleSelector(group);
 if(lastRuleStyle[group].exchange(style)!=style){if(group==2&&style==7&&!restoring.load()){bool before=internal;internal=true;set(kOpenHatRule,2);internal=before;changed[kOpenHatRule].store(true);}changed[kRuleKickStyle+group].store(true);liveDirty.fetch_or(1u<<group);displayPending.store(true);}
}
bool refreshMain(unsigned mask){
 bool old=internal;internal=true;bool ok=makePhrase(4+getInt(kPattern),get(kRandomAmt)*.01f,mask);
 if(ok){if(getInt(kXBeat)!=0){set(kXBeat,0);changed[kXBeat].store(true);}displayPending.store(true);}internal=old;return ok;
}
void queueAutoFill(double start,double end){
 if(fillEnd>=0&&start>=fillEnd-1.e-9){fillStart=fillEnd=-1;if(recordState.load()==5||recordState.load()==4)recordState.store(0);}
 int choice=getInt(kFillEvery);if(!choice||fillStart>=0||recordState.load()==1||recordState.load()==2)return;
 int bars=choice==1?4:choice==2?8:16,finish=(int(std::floor(start/(bars*4.)))+1)*bars*4;
 int slot=getInt(kFillSlot);double due=finish-double(lengths[slot].load())/PPQ;
 if(end>due&&start<finish&&lastAutoBar!=finish){lastAutoBar=finish;activeFill=slot;fillStart=due;fillEnd=finish;}
}
bool makePhrase(int b,float amount,unsigned mask=15){
 constexpr int length=4*PPQ;bool any=false;
 for(int g=0;g<4;++g)any|=(mask&(1u<<g))&&getInt(kRuleKick+g)>0;if(!any)return false;
 std::array<Note,MaxNotes>next{};int count=0;bool overflow=false;
 auto put=[&](int tick,int role,int velocity,int gate=80){if(tick<0||tick>=length)return;for(int j=0;j<count;++j)if(next[j].tick==tick&&next[j].role==role)return;if(count>=MaxNotes){overflow=true;return;}next[count++]={tick,role,mpc::clamp(velocity,1,127),100,gate};};
 const groove::Library::Midi* imported=nullptr;int pick=getInt(kXBeat);
 if(b==4+getInt(kPattern)&&pick>0&&pick<=int(library.categories[3].size()))imported=&library.midi[library.categories[3][pick-1]];
 auto source=[&](auto consume){if(imported){for(const auto&e:imported->notes){if(e.tick>=length)break;consume(Note{e.tick,mpc::clamp(e.note-60,0,7),e.velocity,100,e.gate});}}else for(const auto&c:notes[b])if(c.valid()&&c.read().tick<length)consume(c.read());};
 source([&](Note n){int g=ruleGroup(n.role);if(!(mask&(1u<<g))||getInt(kRuleKick+g)==0){if(count>=MaxNotes){overflow=true;return;}next[count++]=n;}});
 std::array<uint32_t,4> rng{};for(int g=0;g<4;++g)rng[g]=hash(partSeed[g].load()+uint32_t((b-4)*7919+g*211+17));
 auto draw=[&](int g){uint32_t x=rng[g];x^=x<<13;x^=x>>17;x^=x<<5;rng[g]=x?x:0x91357u;return rng[g];};
 auto edit=[&](int g){return bool(mask&(1u<<g))&&getInt(kRuleKick+g)>0;};
 if(edit(0)){
  static const int motifs[][6]={{0,6,10,-1,-1,-1},{0,7,10,14,-1,-1},{0,3,8,11,-1,-1},{0,6,9,14,-1,-1},{0,7,10,-1,-1,-1},{0,3,6,10,15,-1},{0,2,7,10,-1,-1},{0,3,10,14,-1,-1}};
  int style=getInt(kRuleKickStyle),rule=getInt(kRuleKick);const int*motif=motifs[draw(0)%8];int simple[]={0,-1,-1,-1,-1,-1},offbeats[]={0,6,10,14,-1,-1},sync[]={0,3,7,10,15,-1},doubles[]={0,2,7,10,14,-1};
  if(style==4)motif=simple;if(style==5)motif=offbeats;if(style==6)motif=sync;if(style==7)motif=doubles;
  int retained[6]={0,-1,-1,-1,-1,-1},retainedCount=1;
  if(style<=3&&amount<=.5f)source([&](Note n){int step=int(std::lround(n.tick/240.));if(n.role!=0||step<=0||step>=16||step==4||step==12)return;for(int j=0;j<retainedCount;++j)if(retained[j]==step)return;if(retainedCount<5)retained[retainedCount++]=step;});
  if(retainedCount>=3)motif=retained;
  int cap=rule==1?4:5;for(int j=0;j<6&&j<cap&&motif[j]>=0;++j){int step=motif[j];if(step==4||step==12)continue;put(step*240,0,groove::kickAccent(step*240));}
 }
 if(edit(1)){
  int style=getInt(kRuleSnareStyle),rule=getInt(kRuleSnare),velocity=style==4?68:100;
  put(PPQ,1,velocity);put(3*PPQ,1,velocity);
  if(style==5)put(720,1,33,45);else if(style==6)put(3120,1,33,45);else if(style==7){put(720,1,30,45);put(2640,1,35,45);}
  else if(style<=3&&rule>=2&&amount>.5f){int ghosts=rule==3?2:1;for(int j=0;j<ghosts;++j)if(draw(1)%100<unsigned(std::min(get(kGhost),rule==3?40.f:25.f)))put(j?2640:720,1,24+int(draw(1)%13),45);}
 }
 int rhythm=getInt(kHatRhythm);static const int strides[]={960,480,240,720,360};int stride=strides[rhythm/3],ornament=rhythm%3;
 if(edit(2)){
  int style=getInt(kRuleHatStyle),step=stride*(style==4?2:1),offset=style==5?stride/2:0;
  for(int tick=offset;tick<length;tick+=step){if(ornament==2&&tick>=3360)continue;
   int velocity=groove::hatAccent(tick,style==6);
   int role=style==7&&tick>=length-step?3:2;put(tick,role,velocity,role==3?160:50);
  }
  if(ornament==2){for(int j=0;j<4;++j)put(3360+j*120,2,groove::hatAccent(3360+j*120,style==6),35);}
  else if(ornament==1){int tick=3600;if(tick%step==offset)tick=3720;put(tick,2,groove::hatAccent(tick,style==6),45);}
 }
 if(edit(3)){
  int style=getInt(kRulePercStyle),rule=getInt(kRulePerc);
  if(style==4)put(3360,7,49);
  else if(style==5){put(480,7,57);put(1440,5,52);put(2400,7,56);put(3360,4,48);}
  else if(style==6){put(1440,7,59);put(3360,5,54);}
  else if(style==7){put(720,7,56);put(1680,4,49);put(2640,5,58);put(3600,7,44);}
  else {int hits=rule==1?1:rule==2?2:4;int steps[]={3,7,11,14,15};for(int j=0;j<hits;++j){if(draw(3)%100>=unsigned((rule==1?45:rule==2?65:85)*std::max(.25f,amount)))continue;int role=j==0?7:j==1?5:j==2?4:7;int tick=steps[draw(3)%5]*240;put(tick,role,groove::percAccent(tick));}}
 }
 if(overflow){diceResult.store(3);return false;}
 for(int i=1;i<count;++i){Note n=next[i];int j=i;while(j>0&&(next[j-1].tick>n.tick||(next[j-1].tick==n.tick&&next[j-1].role>n.role))){next[j]=next[j-1];--j;}next[j]=n;}
 bool importedHat=imported&&!edit(2);
 int index=b-4,state=getInt(kPhraseState)|(1<<index),grid=importedHat?4:edit(2)?(ornament?3:rhythm/3==0?0:rhythm/3==1?1:2):(phraseBank(b)?phraseGrid(b):4);
 state=(state&~(7<<(3+index*3)))|(grid<<(3+index*3));state&=~(3<<(12+index*2));int hatState=getInt(kHatPhraseState);
 if(importedHat)hatState&=~(15<<(index*4));
 if(edit(2))hatState=(hatState&~(15<<(index*4)))|((rhythm+1)<<(index*4));
 for(auto&c:notes[b])c.data.store(0);lengths[b].store(length);for(int i=0;i<count;++i)notes[b][i].write(next[i]);set(kPhraseState,state);set(kHatPhraseState,hatState);changed[kPhraseState].store(true);changed[kHatPhraseState].store(true);return true;
}
