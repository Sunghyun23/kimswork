// Approved 4x4 Bank B performance map. Appended state only; MAIN identities stay fixed.
static constexpr size_t PerformanceTrailerSize=4*MaxNotes*8+4+12;
std::array<std::atomic<bool>,4> generatedFills{};std::array<bool,4>undoGeneratedFills{};
std::array<std::array<Cell,MaxNotes>,4> rollNotes;
std::array<std::array<uint32_t,MaxNotes*2>,4> rollUndo{};
std::array<float,80> performanceUndo{};
groove::MidiCapture capture;
std::atomic<bool>learnConflict{false};
std::atomic<int> performanceActive{-1},performanceQueued{-1},queuedRoll{-1},learnSlot{-1};
std::atomic<unsigned>performanceMidiHits{0};
std::array<double,4>rollOrigins{{-1,-1,-1,-1}};
std::atomic<bool>fillTimingDirty{false};std::array<uint64_t,2>fillFiredMask{};uint32_t fillChanceSeed=0;
double rollDue=-1,fillDue=-1,rollNext=-1,rollEnd=-1,queuedRollEnd=-1,queuedFillEnd=-1;
int activePerformanceFillTicks=0;
double performanceNow=0,captureOrigin=0,performanceNext=-1,loopStopAt=-1;
bool performanceMuted=false,performanceLoopPending=false,performanceRunning=false,performanceFill=false;
int performanceLastSelected=-1,lastPerformanceUi=-1,lastCaptureUi=-1,lastCaptureCount=-1,lastLearnUi=-2,lastLockUi=-1;bool performanceInitialised=false;
static constexpr int performanceKey(int slot,int field){return kPerformHit0+slot*7+field;}
static constexpr int performanceLock(int slot){return performanceKey(slot,1);}
// Earliest bar ending that still leaves the entire chosen phrase in the future.
static double performanceWindowStart(double now,double beats){return std::ceil((now+beats-1e-8)/4.)*4.-beats;}
static bool performanceTransient(int key){return key==kPerformOpen||key==kPerformLearnLoop||key==kPerformLearnStop||key==kPerformEdit||key==kCapture||key==kExportMidi||key==kPerformLoop||key==kPerformStop||key==kPerformGenerateAll||key==kPerformGenerateSelected||key==kPerformUnlock||key==kPerformLockSelected||key==kPerformLearn||key==kPerformRecord||key==kPerformEditFill||(key>=kPerformHit0&&key<=kPerformLength7&&((key-kPerformHit0)%7==0||(key-kPerformHit0)%7==2));}
void initPerformance(){for(int n=0;n<8;++n)generatePerformance(n);performanceInitialised=true;capture.startWorker();}
void resetPerformance(){learnSlot.store(-1);learnConflict.store(false);for(int n=0;n<8;++n)mpc::Plugin::set(performanceKey(n,0),0);for(int k:{kPerformOpen,kPerformEdit,kCapture,kExportMidi,kPerformLoop,kPerformStop,kPerformGenerateAll,kPerformGenerateSelected,kPerformUnlock,kPerformLockSelected,kPerformLearn,kPerformRecord,kPerformEditFill,kPerformLearnLoop,kPerformLearnStop})mpc::Plugin::set(k,0);stopPerformance();stopCaptureAt(performanceNow);performanceMuted=false;performanceLoopPending=false;loopStopAt=-1;performanceLastSelected=-1;rollRegenerate.store(0);}
int performanceTarget(int q)const{int slot=getInt(kPerformSelect);if(q<0||q>=4)return -1;if(slot>=4&&!generatedFills[slot-4].load()){if(q==0)return -1;if(q==3)return kFillLength;}return performanceKey(slot,3+q);}
// Correct only the complete known1.7.0default layout (including its B1 diagnostic learn).
// Arbitrary learned/custom assignments must survive loading untouched.
void migratePerformanceDefaultNotes(){
 static constexpr int oldMap[]={52,53,55,57,59,60,62,64,65,67};
 static constexpr int corrected[]={48,50,52,53,55,57,59,60,62,64};
 bool legacy=getInt(kPerformNote0)==oldMap[0]||getInt(kPerformNote0)==corrected[0];
 for(int n=1;n<10;++n)legacy=legacy&&getInt(kPerformNote0+n)==oldMap[n];
 if(legacy){for(int n=0;n<10;++n)layerSet(kPerformNote0+n,corrected[n]);changed[kPerformPadInfo].store(true);displayPending.store(true);}
}
void markRecordedFill(int slot){generatedFills[slot].store(false);layerSet(performanceKey(slot+4,4),100);layerSet(performanceKey(slot+4,5),0);layerSet(performanceLock(slot+4),1);}
void selectPerformance(int slot){layerSet(kPerformSelect,slot);if(slot>=4){layerSet(kFillSlot,slot-4);layerSet(kFillLength,choiceForLength(lengths[slot-4].load()));}}
bool performanceParameter(int key,float plain){
 if(key>=kPerformQ0&&key<=kPerformQ3){int target=performanceTarget(key-kPerformQ0),slot=getInt(kPerformSelect);if(target<0){layerSet(key,0);return true;}bool before=internal;internal=true;setNormalised(target,plain);internal=before;changed[target].store(true);if(slot>=4&&!generatedFills[slot-4].load()){if(target==kFillLength)lengths[slot-4].store(lengthForChoice(getInt(kFillLength)));fillTimingDirty.store(true);}else rollRegenerate.fetch_or(1u<<slot);return true;}
 if(key==kPerformSelect){int n=getInt(key);if(n>=4){layerSet(kFillSlot,n-4);layerSet(kFillLength,choiceForLength(lengths[n-4].load()));}return true;}
 if(key>=kPerformHit0&&key<=kPerformLength7&&(key-kPerformHit0)%7>=3){int n=(key-kPerformHit0)/7;if(n<4||generatedFills[n-4].load())rollRegenerate.fetch_or(1u<<n);else fillTimingDirty.store(true);return true;}
 return false;
}
std::atomic<unsigned>rollRegenerate{0};
bool performanceFormat(int key,std::string&out){
 if(performanceTransient(key)){out="Momentary";return true;}
 if(key==kPerformLoopState||(key>=kPerformState0&&key<=kPerformState7)){out="Status";return true;}
 if(key>=kPerformQ0&&key<=kPerformQ3){int t=performanceTarget(key-kPerformQ0);out=t>=0?displayText(t):"--";if(key==kPerformQ1&&getInt(kPerformSelect)>=4&&!generatedFills[getInt(kPerformSelect)-4].load())out+=" %";return true;}
 if(key>=kPerformHit0&&key<=kPerformLength7&&(key-kPerformHit0)%7==3){int slot=(key-kPerformHit0)/7;if(slot>=4&&!generatedFills[slot-4].load()){out="REC";return true;}}
 if(key==kPerformStatus){int q=performanceQueued.load(),a=performanceActive.load();out=queuedRoll.load()>=0?std::string("QUEUED: ROLL ")+char('A'+queuedRoll.load()):q>=0?std::string("QUEUED: FILL ")+char('A'+q):a>=4?std::string("PLAYING: FILL ")+char('A'+a-4):a>=0?std::string("PLAYING: ROLL ")+char('A'+a):"READY";return true;}
 if(key==kCaptureInfo){static const char* names[]={"CAPTURE READY","CAPTURING","TAKE READY","EXPORTING","MIDI SAVED","TAKE FULL","EXPORT FAILED"};out=std::string(names[mpc::clamp(capture.state.load(),0,6)])+" / "+std::to_string(capture.count.load())+" NOTES";return true;}
 if(key==kPerformPadInfo){if(learnConflict.load()){out="NOTE IN USE: CHOOSE ANOTHER PAD";return true;}int n=getInt(kPerformSelect);out=learnSlot.load()>=0?"PRESS A PHYSICAL PAD":std::string("B")+std::to_string(n<4?n+1:n+1)+" / MIDI "+std::to_string(getInt(kPerformNote0+n))+(getBool(performanceLock(n))?" / LOCKED":" / OPEN");return true;}
 return false;
}
void generatePerformance(int n,bool varySettings=false){
 if(varySettings){float amount=get(kRandomAmt)*.01f;if(amount>0){int velocity=performanceKey(n,4),human=performanceKey(n,5);layerSet(velocity,mpc::clamp(param(velocity).def+(int(random()%41)-20)*amount,30.f,127.f));layerSet(human,mpc::clamp(param(human).def+(int(random()%17)-8)*amount,0.f,30.f));}}
 int rate=getInt(performanceKey(n,3)),velocity=getInt(performanceKey(n,4));
 int length=lengthForChoice(getInt(performanceKey(n,6)));
 static constexpr int intervals[]={480,320,240,160,120,240};int spacing=intervals[mpc::clamp(rate,0,5)],i=0;
 auto&dest=n<4?rollNotes[n]:notes[n-4];for(auto&c:dest)c.data.store(0);
 if(n>=4){int slot=n-4;generatedFills[slot].store(true);lengths[slot].store(4*PPQ);baseValid[slot].store(false);int begin=4*PPQ-length;
  // Carry the current one-bar phrase up to the transition, keeping its identity.
  int source=4+getInt(kPattern);for(auto&c:notes[source])if(c.valid()&&i<MaxNotes){auto note=c.read();if(note.tick<begin)dest[i++].write(note);}
  for(int tick=begin;tick<4*PPQ&&i<MaxNotes;++i){int role=slot==0?1:slot==1?(i%3==0?5:1):slot==2?(i%3==0?7:i%3==1?5:1):(i%4==0?0:i%4==1?5:1);int vel=mpc::clamp(velocity-22+int(random()%19)+(tick>=4*PPQ-spacing?12:0),30,127);if(tick==begin||tick+spacing>=4*PPQ||random()%100>=10)dest[i].write({tick,role,vel,100,65});tick+=spacing;if(rate==5&&tick>=3*PPQ)spacing=120;}
 }else{for(int tick=0;tick<length&&i<MaxNotes;++i){int vel=mpc::clamp(velocity-24+int(random()%17)+int(20.*tick/std::max(1,length-1))+(i%4==0?8:0),30,127);if(tick==0||tick+spacing>=length||random()%100>=(rate==4?18u:10u))dest[i].write({tick,1,vel,100,45});tick+=spacing;if(rate==5&&tick>=length/2)spacing=120;}}
 if(n<4)rollDue=-1;else fillDue=-1;displayPending.store(true);
}
void regeneratePerformance(){if(recordState.load()==1||recordState.load()==2)return;for(int n=0;n<8;++n)if(!getBool(performanceLock(n)))generatePerformance(n,true);}
void snapshotPerformanceUndo(){for(int b=0;b<4;++b)undoGeneratedFills[b]=generatedFills[b].load();for(int b=0;b<4;++b)for(int i=0;i<MaxNotes;++i){rollUndo[b][i*2]=rollNotes[b][i].data.load();rollUndo[b][i*2+1]=rollNotes[b][i].gate.load();}for(int i=0;i<56;++i)performanceUndo[i]=getNormalised(kPerformHit0+i);}
void restorePerformanceUndo(){for(int b=0;b<4;++b)generatedFills[b].store(undoGeneratedFills[b]);for(int b=0;b<4;++b)for(int i=0;i<MaxNotes;++i){rollNotes[b][i].data.store(rollUndo[b][i*2]);rollNotes[b][i].gate.store(rollUndo[b][i*2+1]);}for(int i=0;i<56;++i)if(i%7>=3)layerSet(kPerformHit0+i,param(kPerformHit0+i).toPlain(performanceUndo[i]));}
void stopPerformance(){queuedRoll.store(-1);rollNext=rollEnd=queuedRollEnd=queuedFillEnd=-1;performanceFill=false;performanceActive.store(-1);performanceQueued.store(-1);performanceNext=-1;fillStart=fillEnd=-1;for(int n=0;n<4;++n){rollOrigins[n]=-1;}for(auto&v:voices)if(v.active)deactivate(v);displayPending.store(true);}
void triggerPerformance(int slot,double now,bool on,bool countIn,bool physical=false){
 if(!on)return;selectPerformance(slot);if(getBool(kPerformEdit)&&!physical)return;
 if(slot<4){if(on&&!countIn){queuedRoll.store(slot);double beats=double(lengthForChoice(getInt(performanceKey(slot,6))))/PPQ;rollNext=performanceWindowStart(now,beats);queuedRollEnd=rollNext+beats;displayPending.store(true);}return;}
 if(on&&!countIn&&recordState.load()!=1&&recordState.load()!=2){performanceQueued.store(slot-4);double beats=double(generatedFills[slot-4].load()?lengthForChoice(getInt(performanceKey(slot,6))):lengths[slot-4].load())/PPQ;performanceNext=performanceWindowStart(now,beats);queuedFillEnd=performanceNext+beats;}
}
void performanceBlock(double start,double end,double step,bool&run,bool countIn,bool hostRunning){
 performanceNow=start;performanceRunning=run;if(fillTimingDirty.exchange(false))fillDue=-1;
 if(loadGuardFrames.load()>0){for(int key=kPerformOpen;key<kParamCount;++key)if(performanceTransient(key)){layerSet(key,0);last[key]=0;}return;}
 auto take=[&](int key){bool x=getBool(key),hit=x!=(last[key]>.5f);last[key]=getNormalised(key);return hit;};
 unsigned dirty=rollRegenerate.exchange(0);for(int n=0;n<8;++n)if(dirty&(1u<<n)&&recordState.load()!=1&&recordState.load()!=2&&(n<4||generatedFills[n-4].load()))generatePerformance(n);
 for(int n=0;n<8;++n){int key=performanceKey(n,0);if(take(key))triggerPerformance(n,start,getBool(key),countIn);if(take(performanceKey(n,2))&&recordState.load()!=1&&recordState.load()!=2){snapshotUndo();generatePerformance(n,true);}}
 if(take(kPerformLockSelected)){layerSet(performanceLock(getInt(kPerformSelect)),!getBool(performanceLock(getInt(kPerformSelect))));changed[kPerformPadInfo].store(true);displayPending.store(true);}
 if(take(kPerformGenerateSelected)&&recordState.load()!=1&&recordState.load()!=2){snapshotUndo();generatePerformance(getInt(kPerformSelect),true);}
 if(take(kPerformGenerateAll)&&recordState.load()!=1&&recordState.load()!=2){snapshotUndo();regeneratePerformance();}
 if(take(kPerformUnlock))for(int n=0;n<8;++n)layerSet(performanceLock(n),0);
 if(take(kPerformLearnLoop)){learnConflict.store(false);learnSlot.store(8);}
 if(take(kPerformLearnStop)){learnConflict.store(false);learnSlot.store(9);}
 if(take(kPerformLearn)){learnConflict.store(false);learnSlot.store(learnSlot.load()<0?getInt(kPerformSelect):-1);}
 if(take(kPerformRecord)&&getInt(kPerformSelect)>=4){layerSet(kFillSlot,getInt(kPerformSelect)-4);set(kRecord,getBool(kRecord)?0:1);}
 if(take(kPerformEditFill)&&getInt(kPerformSelect)>=4){layerSet(kFillSlot,getInt(kPerformSelect)-4);layerSet(kPerformOpen,0);layerSet(kEditor,1);}
 if(take(kCapture)){if(capture.state.load()==1)stopCaptureAt(performanceNow);else if(capture.begin(currentTempo.load()))captureOrigin=start;displayPending.store(true);}
 if(take(kExportMidi)){stopCaptureAt(performanceNow);if(capture.count.load()>0&&capture.state.load()!=3)capture.state.store(3);displayPending.store(true);}
 if(take(kPerformLoop)){if(hostRunning){if(!performanceMuted)loopStopAt=std::ceil((start+1e-8)/4)*4;else performanceMuted=false;}else if(!countIn){if(getBool(kPreview))loopStopAt=std::ceil((start+1e-8)/4)*4;else{layerSet(kPreview,1);performanceLoopPending=true;}}}
 if(take(kPerformStop)){performanceMuted=true;layerSet(kPreview,0);loopStopAt=-1;stopPerformance();stopCaptureAt(performanceNow);run=false;}
 if(loopStopAt>=0&&start>=loopStopAt-1e-8){performanceMuted=true;layerSet(kPreview,0);loopStopAt=-1;stopPerformance();}
 if(performanceLoopPending){performanceMuted=false;performanceLoopPending=false;}
 if(!hostRunning&&!getBool(kPreview))performanceMuted=false;
 if(performanceMuted)run=false;
}
bool performanceMidi(const mpc::MidiEvent&e,double now,bool countIn){
 if(e.isAllNotesOff()){stopPerformance();return false;}
 if(e.isNoteOn()&&learnSlot.load()>=0){bool used=e.note()>=96&&e.note()<=98;for(int r=0;r<8;++r)used|=e.note()==getInt(kPadNote0+r*9);if(used){learnConflict.store(true);changed[kPerformPadInfo].store(true);displayPending.store(true);return true;}learnConflict.store(false);stopPerformance();int n=learnSlot.exchange(-1),old=getInt(kPerformNote0+n);for(int other=0;other<10;++other)if(other!=n&&getInt(kPerformNote0+other)==e.note())layerSet(kPerformNote0+other,old);layerSet(kPerformNote0+n,e.note());displayPending.store(true);return true;}
 if(!e.isNoteOn()&&!e.isNoteOff())return false;
 if(e.note()>=96&&e.note()<=98){if(e.isNoteOn()){trigger(8+e.note()-96,e.velocity());recordInput(now,6,e.velocity());}return true;}
 for(int role=0;role<8;++role)if(e.note()==getInt(kPadNote0+role*9))return false;
 for(int n=0;n<10;++n)if(e.note()==getInt(kPerformNote0+n)){
  performanceMidiHits.fetch_add(1);bool on=e.isNoteOn();
  if(n<8){triggerPerformance(n,now,on,countIn,true);}
  else if(on){int key=n==8?kPerformLoop:kPerformStop;if(n==9){performanceMuted=true;layerSet(kPreview,0);stopPerformance();stopCaptureAt(performanceNow);}else set(key,!getBool(key));}
  return true;
 }
 return false;
}
bool performanceSuppress(int role,double now)const{return performanceMuted||(performanceFill&&fillStart>=0&&now>=fillStart&&now<fillEnd)||(role==1&&performanceActive.load()>=0&&performanceActive.load()<4);}
void performanceFrame(double now,double step,bool run,bool countIn){
 if(countIn||(!run&&performanceActive.load()<0&&performanceQueued.load()<0&&queuedRoll.load()<0))return;
 // One press schedules a finite phrase. Note-off never cancels a one-shot.
 int roll=queuedRoll.load();if(roll>=0&&now+step*.5>=rollNext){performanceActive.store(roll);rollOrigins[roll]=rollNext;rollEnd=queuedRollEnd;rollDue=-1;queuedRoll.store(-1);displayPending.store(true);}
 int runningRoll=performanceActive.load();if(runningRoll>=0&&runningRoll<4&&now+step*.5>=rollEnd){performanceActive.store(-1);rollEnd=-1;for(auto&v:voices)if(v.role==1)v.choke=.97f;displayPending.store(true);}
 int queued=performanceQueued.load();if(queued>=0&&now+step*.5>=performanceNext){activeFill=queued;performanceFill=true;fillDue=-1;fillFiredMask={};fillChanceSeed=random();fillStart=performanceNext;fillEnd=queuedFillEnd;activePerformanceFillTicks=int(std::lround((fillEnd-fillStart)*PPQ));performanceQueued.store(-1);recordState.store(4);displayPending.store(true);}
 // Fill activation is handled before this block's event scheduling where possible;
 // schedule notes directly for the trigger block, then normal scheduleBank continues.
 if(performanceFill&&fillStart>=0&&now>=fillStart&&now<fillEnd){
  if(now+step*.5>=fillDue){fillDue=fillEnd;for(auto&cell:notes[activeFill])if(cell.valid()){size_t cellIndex=size_t(&cell-notes[activeFill].data());uint64_t mask=uint64_t(1)<<(cellIndex%64);if(fillFiredMask[cellIndex/64]&mask)continue;auto n=cell.read();bool generated=generatedFills[activeFill].load();int prefix=generated?4*PPQ-activePerformanceFillTicks:0;if(n.tick<prefix)continue;double t=fillStart+double(n.tick-prefix)/PPQ;if(t>=fillEnd)continue;uint32_t chance=hash(uint32_t(n.tick+n.role*20011)+fillChanceSeed);if(generated){int rate=getInt(performanceKey(activeFill+4,3));if(rate!=1&&rate!=3&&(n.tick/240)%2)t+=(mpc::clamp(get(kSwing),50.f,75.f)-50)*.005;}t+=(int(hash(n.tick+n.role*57)%2001)-1000)*get(performanceKey(activeFill+4,5))*.0000001;t=mpc::clamp(t,fillStart,fillEnd-step);if(t>=now-step*.5&&t<now+step*.5){fillFiredMask[cellIndex/64]|=mask;if(float(chance%10000)>=get(kFillDensity)*n.chance)continue;if(!(n.role==1&&performanceActive.load()>=0&&performanceActive.load()<4)){int velocity=generatedFills[activeFill].load()?n.velocity:mpc::clamp(int(std::lround(n.velocity*get(performanceKey(activeFill+4,4))*.01f)),1,127);trigger(n.role,velocity,n.gate);}}else if(t>=now+step*.5)fillDue=std::min(fillDue,t);}}if(performanceActive.load()<0)performanceActive.store(4+activeFill);}
 else if(performanceFill&&(fillStart<0||now>=fillEnd)){if(performanceActive.load()>=4)performanceActive.store(-1);performanceFill=false;}
 int slot=performanceActive.load();if(slot<0||slot>=4)return;
 double length=double(lengthForChoice(getInt(performanceKey(slot,6))))/PPQ;
 double local=now-rollOrigins[slot];int cycle=int(std::floor(local/length));
 if(now+step*.5>=rollDue){rollDue=now+length;for(int c=cycle;c<=cycle+1;++c)for(auto&cell:rollNotes[slot])if(cell.valid()){auto n=cell.read();double t=rollOrigins[slot]+c*length+double(n.tick)/PPQ;int rate=getInt(performanceKey(slot,3));if(rate!=1&&rate!=3&&(n.tick/240)%2)t+=(mpc::clamp(get(kSwing),50.f,75.f)-50)*.005;double amount=get(performanceKey(slot,5))*.0001;double jitter=(int(hash(n.tick+slot*57)%2001)-1000)*amount*.001;t+=std::max(-double(n.tick)/PPQ,jitter);if(t>=now-step*.5&&t<now+step*.5)trigger(1,n.velocity,n.gate);else if(t>=now+step*.5)rollDue=std::min(rollDue,t);}}

}
void stopCaptureAt(double now){if(capture.state.load()==1){double ticks=std::max(0.,now-captureOrigin)*PPQ;capture.endTick.store(std::max(capture.endTick.load(),uint32_t(std::llround(ticks))));}capture.stop();}
void capturePerformance(int role,int vel,int gate){if(role<0||role>10)return;double delta=performanceNow-captureOrigin;if(delta<0)return;int note=role<8?getInt(kPadNote0+role*9):96+role-8;capture.push(uint32_t(std::llround(delta*PPQ)),note,vel,uint32_t(std::max(1.,gate*currentTempo.load()/60000.*PPQ)));}
void syncPerformance(){int selected=getInt(kPerformSelect);for(int q=0;q<4;++q){int key=performanceTarget(q);float value=key>=0?getNormalised(key):0;if(get(kPerformQ0+q)!=value)layerSet(kPerformQ0+q,value);}for(int n=0;n<8;++n){int state=performanceActive.load()==n?2:(n<4&&queuedRoll.load()==n)||(n>=4&&performanceQueued.load()==n-4)?1:0;if(getInt(kPerformState0+n)!=state)layerSet(kPerformState0+n,state);}layerSet(kPerformLoopState,performanceRunning&&!performanceMuted?1:0);int status=((performanceActive.load()+1)*8+performanceQueued.load()+1)*5+queuedRoll.load()+1;if(status!=lastPerformanceUi){lastPerformanceUi=status;changed[kPerformStatus].store(true);displayPending.store(true);}int cap=capture.state.load(),count=int(capture.count.load())/8;if(cap!=lastCaptureUi||count!=lastCaptureCount){lastCaptureUi=cap;lastCaptureCount=count;changed[kCaptureInfo].store(true);displayPending.store(true);}if(selected!=performanceLastSelected||learnSlot.load()!=lastLearnUi||int(getBool(performanceLock(selected)))!=lastLockUi){performanceLastSelected=selected;lastLearnUi=learnSlot.load();lastLockUi=getBool(performanceLock(selected));changed[kPerformPadInfo].store(true);displayPending.store(true);}}
void savePerformanceState(std::vector<uint8_t>&out){size_t start=out.size();for(int b=0;b<4;++b)for(auto&c:rollNotes[b]){uint32_t v[]={c.data.load(),c.gate.load()};for(auto x:v)for(int j=0;j<4;++j)out.push_back(uint8_t(x>>(8*j)));}uint32_t flags=0;for(int b=0;b<4;++b)if(generatedFills[b].load())flags|=1u<<b;for(int j=0;j<4;++j)out.push_back(uint8_t(flags>>(8*j)));uint32_t size=uint32_t(out.size()-start);const char magic[]="GDPF0001";out.insert(out.end(),magic,magic+8);for(int j=0;j<4;++j)out.push_back(uint8_t(size>>(8*j)));}
bool loadPerformanceState(const uint8_t*data,size_t&size){for(auto&flag:generatedFills)flag.store(false);if(size<12||std::memcmp(data+size-12,"GDPF0001",8))return false;uint32_t length=0;for(int j=0;j<4;++j)length|=uint32_t(data[size-4+j])<<(8*j);if((length!=4*MaxNotes*8&&length!=4*MaxNotes*8+4)||length>size-12)return false;size_t pos=size-12-length;for(auto&bank:rollNotes)for(auto&c:bank){uint32_t v[2]={};for(auto&x:v)for(int j=0;j<4;++j)x|=uint32_t(data[pos++])<<(8*j);c.data.store(v[0]);c.gate.store(mpc::clamp(v[1],5u,500u));}if(length==4*MaxNotes*8+4){uint32_t flags=0;for(int j=0;j<4;++j)flags|=uint32_t(data[pos++])<<(8*j);for(int b=0;b<4;++b)generatedFills[b].store(flags&(1u<<b));}size-=12+length;return true;}
