// Approved 4x4 Bank B performance map. Appended state only; MAIN identities stay fixed.
static constexpr size_t PerformanceTrailerSize=4*MaxNotes*8+4+12;
std::array<std::atomic<bool>,4> generatedFills{};std::array<bool,4>undoGeneratedFills{};
std::array<std::array<Cell,MaxNotes>,8> rollNotes;
std::array<std::array<uint32_t,MaxNotes*2>,8> rollUndo{};
std::array<float,84> performanceUndo{};
groove::MidiCapture capture;
std::atomic<bool>learnConflict{false};
std::atomic<int> performanceActive{-1},performanceQueued{-1},queuedRoll{-1},learnSlot{-1};
std::atomic<unsigned>performanceMidiHits{0};
std::array<double,12>rollOrigins{};
std::array<uint64_t,2>rollFiredMask{};bool legacyPerformanceMap=false;
std::atomic<bool>fillTimingDirty{false};std::array<uint64_t,2>fillFiredMask{};uint32_t fillChanceSeed=0;
double rollDue=-1,fillDue=-1,rollNext=-1,rollEnd=-1,queuedRollEnd=-1,queuedFillEnd=-1;
int activePerformanceFillTicks=0;
double performanceNow=0,captureOrigin=0,performanceNext=-1,loopStopAt=-1;
bool performanceMuted=false,performanceLoopPending=false,performanceRunning=false,performanceFill=false;
int performanceLastSelected=-1,lastPerformanceUi=-1,lastCaptureUi=-1,lastCaptureCount=-1,lastLearnUi=-2,lastLockUi=-1;bool performanceInitialised=false;
static constexpr bool isPerformanceFill(int slot){return slot>=4&&slot<8;}
static constexpr bool isPerformanceRoll(int slot){return slot>=0&&slot<12&&!isPerformanceFill(slot);}
static constexpr int rollBank(int slot){return slot<4?slot:slot-4;}
static constexpr int performanceKey(int slot,int field){return slot<8?kPerformHit0+slot*7+field:kPerformHit8+(slot-8)*7+field;}
static int performanceField(int key){if(key>=kPerformHit0&&key<=kPerformLength7)return(key-kPerformHit0)%7;if(key>=kPerformHit8&&key<=kPerformLength11)return(key-kPerformHit8)%7;return -1;}
static int performanceSlotForKey(int key){return key<=kPerformLength7?(key-kPerformHit0)/7:8+(key-kPerformHit8)/7;}
static int performanceStateKey(int n){return n<8?kPerformState0+n:kPerformState8+n-8;}
static bool performanceStateParameter(int key){return key>=kPerformState0&&key<=kPerformState7||key>=kPerformState8&&key<=kPerformState11;}
static int performanceNoteKey(int n){return n<8?kPerformNote0+n:n<12?kHatPerformNote0+n-8:kPerformNote8+n-12;}
int selectedPerformance()const{return getInt(kPerformSelectFull);}
static std::string performanceName(int n){return std::string(isPerformanceFill(n)?"FILL ":n>=8?"HAT ":"ROLL ")+char('A'+n%4);}
static constexpr int performanceLock(int slot){return performanceKey(slot,1);}
// Earliest bar ending that still leaves the entire chosen phrase in the future.
static double performanceWindowStart(double now,double beats){return std::ceil((now+beats-1e-8)/4.)*4.-beats;}
static bool performanceTransient(int key){return key==kPerformOpen||key==kPerformLearnLoop||key==kPerformLearnStop||key==kPerformEdit||key==kCapture||key==kExportMidi||key==kPerformLoop||key==kPerformStop||key==kPerformGenerateAll||key==kPerformGenerateSelected||key==kPerformUnlock||key==kPerformLockSelected||key==kPerformLearn||key==kPerformRecord||key==kPerformEditFill||(performanceField(key)==0||performanceField(key)==2);}
void initPerformance(){layerSet(kPerformNote8,69);layerSet(kPerformNote9,71);for(int n=0;n<12;++n)generatePerformance(n);performanceInitialised=true;capture.startWorker();}
void resetPerformance(){learnSlot.store(-1);learnConflict.store(false);for(int n=0;n<12;++n)mpc::Plugin::set(performanceKey(n,0),0);for(int k:{kPerformOpen,kPerformEdit,kCapture,kExportMidi,kPerformLoop,kPerformStop,kPerformGenerateAll,kPerformGenerateSelected,kPerformUnlock,kPerformLockSelected,kPerformLearn,kPerformRecord,kPerformEditFill,kPerformLearnLoop,kPerformLearnStop})mpc::Plugin::set(k,0);stopPerformance();stopCaptureAt(performanceNow);performanceMuted=false;performanceLoopPending=false;loopStopAt=-1;performanceLastSelected=-1;rollRegenerate.store(0);}
int performanceTarget(int q)const{int slot=selectedPerformance();if(q<0||q>=4)return -1;if(isPerformanceFill(slot)&&!generatedFills[slot-4].load()){if(q==0)return -1;if(q==3)return kFillLength;}return performanceKey(slot,3+q);}
// Correct only the complete known1.7.0default layout (including its B1 diagnostic learn).
// Arbitrary learned/custom assignments must survive loading untouched.
void migratePerformanceDefaultNotes(){
 static constexpr int oldMap[]={52,53,55,57,59,60,62,64,65,67};
 static constexpr int corrected[]={48,50,52,53,55,57,59,60,62,64};
 bool legacy=getInt(kPerformNote0)==oldMap[0]||getInt(kPerformNote0)==corrected[0];
 for(int n=1;n<10;++n)legacy=legacy&&getInt(kPerformNote0+n)==oldMap[n];
 if(legacy){for(int n=0;n<10;++n)layerSet(kPerformNote0+n,corrected[n]);changed[kPerformPadInfo].store(true);displayPending.store(true);}
 // Move only the known old LOOP/STOP defaults; keep learned mappings intact.
 if(legacyPerformanceMap){if(getInt(kPerformNote8)==62&&getInt(kPerformNote9)==64){layerSet(kPerformNote8,69);layerSet(kPerformNote9,71);}
  for(int n=8;n<12;++n){int key=performanceNoteKey(n),note=getInt(key);bool used=false;for(int j=0;j<14;++j)if(j!=n&&note>=0&&getInt(performanceNoteKey(j))==note)used=true;for(int r=0;r<8;++r)used|=note>=0&&getInt(kPadNote0+r*9)==note;if(used)layerSet(key,-1);}
 }
}
void markRecordedFill(int slot){generatedFills[slot].store(false);layerSet(performanceKey(slot+4,4),100);layerSet(performanceKey(slot+4,5),0);layerSet(performanceLock(slot+4),1);}
void selectPerformance(int slot){layerSet(kPerformSelectFull,slot);if(slot<8)layerSet(kPerformSelect,slot);if(isPerformanceFill(slot)){layerSet(kFillSlot,slot-4);layerSet(kFillLength,choiceForLength(lengths[slot-4].load()));}}
bool performanceParameter(int key,float plain){
 if(key>=kPerformQ0&&key<=kPerformQ3){int target=performanceTarget(key-kPerformQ0),slot=selectedPerformance();if(target<0){layerSet(key,0);return true;}bool before=internal;internal=true;setNormalised(target,plain);internal=before;changed[target].store(true);if(isPerformanceFill(slot)&&!generatedFills[slot-4].load()){if(target==kFillLength)lengths[slot-4].store(lengthForChoice(getInt(kFillLength)));fillTimingDirty.store(true);}else rollRegenerate.fetch_or(1u<<slot);return true;}
 if(key==kPerformSelect||key==kPerformSelectFull){int n=getInt(key);if(key==kPerformSelect)layerSet(kPerformSelectFull,n);else if(n<8)layerSet(kPerformSelect,n);if(isPerformanceFill(n)){layerSet(kFillSlot,n-4);layerSet(kFillLength,choiceForLength(lengths[n-4].load()));}return true;}
 if(performanceField(key)>=3){int n=performanceSlotForKey(key);if(isPerformanceRoll(n)||generatedFills[n-4].load())rollRegenerate.fetch_or(1u<<n);else fillTimingDirty.store(true);return true;}
 return false;
}
std::atomic<unsigned>rollRegenerate{0};
bool performanceFormat(int key,std::string&out){
 if(performanceTransient(key)){out="Momentary";return true;}
 if(key==kPerformLoopState||performanceStateParameter(key)){out="Status";return true;}
 if(key>=kPerformQ0&&key<=kPerformQ3){int t=performanceTarget(key-kPerformQ0);out=t>=0?displayText(t):"--";if(key==kPerformQ1&&isPerformanceFill(selectedPerformance())&&!generatedFills[selectedPerformance()-4].load())out+=" %";return true;}
 if(performanceField(key)==3){int slot=performanceSlotForKey(key);if(isPerformanceFill(slot)&&!generatedFills[slot-4].load()){out="REC";return true;}}
 if(key==kPerformStatus){int q=performanceQueued.load(),a=performanceActive.load(),r=queuedRoll.load();out=r>=0?"QUEUED: "+performanceName(r):q>=0?"QUEUED: "+performanceName(q+4):a>=0?"PLAYING: "+performanceName(a):"READY";return true;}
 if(key==kCaptureInfo){static const char* names[]={"CAPTURE READY","CAPTURING","TAKE READY","EXPORTING","MIDI SAVED","TAKE FULL","EXPORT FAILED"};out=std::string(names[mpc::clamp(capture.state.load(),0,6)])+" / "+std::to_string(capture.count.load())+" NOTES";return true;}
 if(key==kPerformPadInfo){if(learnConflict.load()){out="NOTE IN USE: CHOOSE ANOTHER PAD";return true;}int n=selectedPerformance();out=learnSlot.load()>=0?"PRESS A PHYSICAL PAD":std::string("B")+std::to_string(n+1)+(getInt(performanceNoteKey(n))<0?" / LEARN PAD":" / MIDI "+std::to_string(getInt(performanceNoteKey(n))))+(getBool(performanceLock(n))?" / LOCKED":" / OPEN");return true;}
 return false;
}
void generatePerformance(int n,bool varySettings=false){
 if(varySettings){float amount=get(kRandomAmt)*.01f;if(amount>0){int velocity=performanceKey(n,4),human=performanceKey(n,5);layerSet(velocity,mpc::clamp(param(velocity).def+(int(random()%41)-20)*amount,30.f,127.f));layerSet(human,mpc::clamp(param(human).def+(int(random()%17)-8)*amount,0.f,30.f));}}
 int rate=getInt(performanceKey(n,3)),velocity=getInt(performanceKey(n,4));
 int length=lengthForChoice(getInt(performanceKey(n,6))),i=0;
 auto&dest=isPerformanceRoll(n)?rollNotes[rollBank(n)]:notes[n-4];for(auto&c:dest)c.data.store(0);
 auto rng=[&](){return random();};
 if(isPerformanceFill(n)){int slot=n-4;generatedFills[slot].store(true);lengths[slot].store(4*PPQ);baseValid[slot].store(false);int begin=4*PPQ-length,source=4+getInt(kPattern);
  for(auto&c:notes[source])if(c.valid()&&i<MaxNotes){auto note=c.read();if(note.tick<begin)dest[i++].write(note);}
  // Retain the current kick motif and exact 2/4 backbeats in the transition.
  // Generated ornaments share this backbeat map rather than doubling it.
  std::array<bool,4>backbeats{};
  for(auto&c:notes[source])if(c.valid()&&i<MaxNotes){auto note=c.read();if(note.tick<begin||note.tick>=4*PPQ)continue;
   if(note.role==0&&!(note.tick%PPQ==0&&(note.tick/PPQ)%2==1))dest[i++].write(note);
   if(note.role==1&&note.tick%PPQ==0&&(note.tick/PPQ)%2==1){dest[i++].write(note);backbeats[note.tick/PPQ]=true;}
  }
  groove::PerformanceRhythm::fill(length,rate,velocity,slot,rng,[&](int tick,int role,int vel,int gate){int absolute=begin+tick;
   if(role==1){for(int beat=1;beat<4;beat+=2)if(backbeats[beat]&&std::abs(absolute-beat*PPQ)<120)return;}
   if(i<MaxNotes)dest[i++].write({absolute,role,vel,100,gate});});
 }else groove::PerformanceRhythm::roll(length,rate,velocity,n>=8,rng,[&](int tick,int role,int vel,int gate){if(i<MaxNotes)dest[i++].write({tick,role,vel,100,gate});});

 if(isPerformanceRoll(n))rollDue=-1;else fillDue=-1;displayPending.store(true);
}
void regeneratePerformance(){if(recordState.load()==1||recordState.load()==2)return;for(int n=0;n<12;++n)if(!getBool(performanceLock(n)))generatePerformance(n,true);}
void snapshotPerformanceUndo(){for(int b=0;b<4;++b)undoGeneratedFills[b]=generatedFills[b].load();for(int b=0;b<8;++b)for(int i=0;i<MaxNotes;++i){rollUndo[b][i*2]=rollNotes[b][i].data.load();rollUndo[b][i*2+1]=rollNotes[b][i].gate.load();}for(int n=0;n<12;++n)for(int f=0;f<7;++f)performanceUndo[n*7+f]=getNormalised(performanceKey(n,f));}
void restorePerformanceUndo(){for(int b=0;b<4;++b)generatedFills[b].store(undoGeneratedFills[b]);for(int b=0;b<8;++b)for(int i=0;i<MaxNotes;++i){rollNotes[b][i].data.store(rollUndo[b][i*2]);rollNotes[b][i].gate.store(rollUndo[b][i*2+1]);}for(int n=0;n<12;++n)for(int f=3;f<7;++f){int key=performanceKey(n,f);layerSet(key,param(key).toPlain(performanceUndo[n*7+f]));}}
void stopPerformance(){queuedRoll.store(-1);rollNext=rollEnd=queuedRollEnd=queuedFillEnd=-1;performanceFill=false;performanceActive.store(-1);performanceQueued.store(-1);performanceNext=-1;fillStart=fillEnd=-1;for(auto&origin:rollOrigins)origin=-1;rollFiredMask={};for(auto&v:voices)if(v.active)deactivate(v);displayPending.store(true);}
void triggerPerformance(int slot,double now,bool on,bool countIn,bool physical=false){
 if(!on)return;selectPerformance(slot);if(getBool(kPerformEdit)&&!physical)return;
 if(isPerformanceRoll(slot)){if(on&&!countIn){queuedRoll.store(slot);double beats=double(lengthForChoice(getInt(performanceKey(slot,6))))/PPQ;rollNext=performanceWindowStart(now,beats);queuedRollEnd=rollNext+beats;displayPending.store(true);}return;}
 if(on&&!countIn&&recordState.load()!=1&&recordState.load()!=2){performanceQueued.store(slot-4);double beats=double(generatedFills[slot-4].load()?lengthForChoice(getInt(performanceKey(slot,6))):lengths[slot-4].load())/PPQ;performanceNext=performanceWindowStart(now,beats);queuedFillEnd=performanceNext+beats;}
}
void performanceBlock(double start,double end,double step,bool&run,bool countIn,bool hostRunning){
 performanceNow=start;performanceRunning=run;
 // A phrase queued before RECORD must not leak through the host count-in.
 if(countIn&&(performanceActive.load()>=0||performanceQueued.load()>=0||queuedRoll.load()>=0))stopPerformance();
 if(fillTimingDirty.exchange(false))fillDue=-1;
 if(loadGuardFrames.load()>0){for(int key=kPerformOpen;key<kParamCount;++key)if(performanceTransient(key)){layerSet(key,0);last[key]=0;}return;}
 auto take=[&](int key){bool x=getBool(key),hit=x!=(last[key]>.5f);last[key]=getNormalised(key);return hit;};
 unsigned dirty=rollRegenerate.exchange(0);for(int n=0;n<12;++n)if(dirty&(1u<<n)&&recordState.load()!=1&&recordState.load()!=2&&(isPerformanceRoll(n)||generatedFills[n-4].load()))generatePerformance(n);
 for(int n=0;n<12;++n){int key=performanceKey(n,0);if(take(key))triggerPerformance(n,start,getBool(key),countIn);if(take(performanceKey(n,2))&&recordState.load()!=1&&recordState.load()!=2){snapshotUndo();generatePerformance(n,true);}}
 if(take(kPerformLockSelected)){layerSet(performanceLock(selectedPerformance()),!getBool(performanceLock(selectedPerformance())));changed[kPerformPadInfo].store(true);displayPending.store(true);}
 if(take(kPerformGenerateSelected)&&recordState.load()!=1&&recordState.load()!=2){snapshotUndo();generatePerformance(selectedPerformance(),true);}
 if(take(kPerformGenerateAll)&&recordState.load()!=1&&recordState.load()!=2){snapshotUndo();regeneratePerformance();}
 if(take(kPerformUnlock))for(int n=0;n<12;++n)layerSet(performanceLock(n),0);
 if(take(kPerformLearnLoop)){learnConflict.store(false);learnSlot.store(12);}
 if(take(kPerformLearnStop)){learnConflict.store(false);learnSlot.store(13);}
 if(take(kPerformLearn)){learnConflict.store(false);learnSlot.store(learnSlot.load()<0?selectedPerformance():-1);}
 if(take(kPerformRecord)&&isPerformanceFill(selectedPerformance())){layerSet(kFillSlot,selectedPerformance()-4);set(kRecord,getBool(kRecord)?0:1);}
 if(take(kPerformEditFill)&&isPerformanceFill(selectedPerformance())){layerSet(kFillSlot,selectedPerformance()-4);layerSet(kPerformOpen,0);layerSet(kEditor,1);}
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
 if(e.isNoteOn()&&learnSlot.load()>=0){bool used=e.note()>=96&&e.note()<=98;for(int r=0;r<8;++r)used|=e.note()==getInt(kPadNote0+r*9);if(used){learnConflict.store(true);changed[kPerformPadInfo].store(true);displayPending.store(true);return true;}int selected=learnSlot.load();if(getInt(performanceNoteKey(selected))<0){for(int j=0;j<14;++j)if(j!=selected&&getInt(performanceNoteKey(j))==e.note()){learnConflict.store(true);changed[kPerformPadInfo].store(true);displayPending.store(true);return true;}}learnConflict.store(false);stopPerformance();int n=learnSlot.exchange(-1),old=getInt(performanceNoteKey(n));for(int other=0;other<14;++other)if(other!=n&&getInt(performanceNoteKey(other))==e.note())layerSet(performanceNoteKey(other),old);layerSet(performanceNoteKey(n),e.note());displayPending.store(true);return true;}
 if(!e.isNoteOn()&&!e.isNoteOff())return false;
 if(e.note()>=96&&e.note()<=98){if(e.isNoteOn()){trigger(8+e.note()-96,e.velocity());recordInput(now,6,e.velocity());}return true;}
 for(int role=0;role<8;++role)if(e.note()==getInt(kPadNote0+role*9))return false;
 for(int n=0;n<14;++n)if(e.note()==getInt(performanceNoteKey(n))){
  performanceMidiHits.fetch_add(1);bool on=e.isNoteOn();
  if(n<12){triggerPerformance(n,now,on,countIn,true);}
  else if(on){int key=n==12?kPerformLoop:kPerformStop;if(n==13){performanceMuted=true;layerSet(kPreview,0);stopPerformance();stopCaptureAt(performanceNow);}else set(key,!getBool(key));}
  return true;
 }
 return false;
}
bool performanceSuppress(int role,double now)const{return performanceMuted||(performanceFill&&fillStart>=0&&now>=fillStart&&now<fillEnd)||(isPerformanceRoll(performanceActive.load())&&(performanceActive.load()<4?role==1:role==2||role==3||role==8||role==10));}
void performanceFrame(double now,double step,bool run,bool countIn){
 if(countIn||(!run&&performanceActive.load()<0&&performanceQueued.load()<0&&queuedRoll.load()<0))return;
 // One press schedules a finite phrase. Note-off never cancels a one-shot.
 int roll=queuedRoll.load();if(roll>=0&&now+step*.5>=rollNext){performanceActive.store(roll);rollOrigins[roll]=rollNext;rollEnd=queuedRollEnd;rollDue=-1;rollFiredMask={};queuedRoll.store(-1);displayPending.store(true);}
 int runningRoll=performanceActive.load();if(isPerformanceRoll(runningRoll)&&now+step*.5>=rollEnd){performanceActive.store(-1);rollEnd=-1;for(auto&v:voices)if(runningRoll<4?v.role==1:v.role==2||v.role==3)v.choke=.97f;displayPending.store(true);}
 int queued=performanceQueued.load();if(queued>=0&&now+step*.5>=performanceNext){activeFill=queued;performanceFill=true;fillDue=-1;fillFiredMask={};fillChanceSeed=random();fillStart=performanceNext;fillEnd=queuedFillEnd;activePerformanceFillTicks=int(std::lround((fillEnd-fillStart)*PPQ));performanceQueued.store(-1);recordState.store(4);displayPending.store(true);}
 // Fill activation is handled before this block's event scheduling where possible;
 // schedule notes directly for the trigger block, then normal scheduleBank continues.
 if(performanceFill&&fillStart>=0&&now>=fillStart&&now<fillEnd){
  if(now+step*.5>=fillDue){fillDue=fillEnd;for(auto&cell:notes[activeFill])if(cell.valid()){size_t cellIndex=size_t(&cell-notes[activeFill].data());uint64_t mask=uint64_t(1)<<(cellIndex%64);if(fillFiredMask[cellIndex/64]&mask)continue;auto n=cell.read();bool generated=generatedFills[activeFill].load();int prefix=generated?4*PPQ-activePerformanceFillTicks:0;if(n.tick<prefix)continue;double t=fillStart+double(n.tick-prefix)/PPQ;if(t>=fillEnd)continue;uint32_t chance=hash(uint32_t(n.tick+n.role*20011)+fillChanceSeed);if(generated){int rate=getInt(performanceKey(activeFill+4,3));if(n.role!=0)t+=groove::PerformanceRhythm::swingOffset(n.tick,rate,get(kSwing));}t+=(int(hash(n.tick+n.role*57)%2001)-1000)*get(performanceKey(activeFill+4,5))*.0000001;t=mpc::clamp(t,fillStart,fillEnd-step);if(t>=now-step*.5&&t<now+step*.5){fillFiredMask[cellIndex/64]|=mask;if(float(chance%10000)>=get(kFillDensity)*n.chance)continue;if(!(isPerformanceRoll(performanceActive.load())&&(performanceActive.load()<4?n.role==1:n.role==2||n.role==3))){int velocity=generatedFills[activeFill].load()?n.velocity:mpc::clamp(int(std::lround(n.velocity*get(performanceKey(activeFill+4,4))*.01f)),1,127);trigger(n.role,velocity,n.gate);}}else if(t>=now+step*.5)fillDue=std::min(fillDue,t);}}if(performanceActive.load()<0)performanceActive.store(4+activeFill);}
 else if(performanceFill&&(fillStart<0||now>=fillEnd)){if(isPerformanceFill(performanceActive.load()))performanceActive.store(-1);performanceFill=false;}
 int slot=performanceActive.load();if(!isPerformanceRoll(slot))return;
 if(now+step*.5>=rollDue){rollDue=rollEnd;for(auto&cell:rollNotes[rollBank(slot)])if(cell.valid()){
  size_t index=size_t(&cell-rollNotes[rollBank(slot)].data());uint64_t bit=uint64_t(1)<<(index%64);if(rollFiredMask[index/64]&bit)continue;
  auto n=cell.read();double t=rollOrigins[slot]+double(n.tick)/PPQ;if(t>=rollEnd)continue;int rate=getInt(performanceKey(slot,3));
  int anchorTick=n.role==1&&n.gate==25?n.tick+80:n.tick;t+=groove::PerformanceRhythm::swingOffset(anchorTick,rate,get(kSwing));
  t+=(int(hash(n.tick+slot*57)%2001)-1000)*get(performanceKey(slot,5))*.0000001;
  t=mpc::clamp(t,rollOrigins[slot],rollEnd-step);
  if(t>=now-step*.5&&t<now+step*.5){rollFiredMask[index/64]|=bit;trigger(n.role,n.velocity,n.gate);}else if(t>=now+step*.5)rollDue=std::min(rollDue,t);
 }}

}
void stopCaptureAt(double now){if(capture.state.load()==1){double ticks=std::max(0.,now-captureOrigin)*PPQ;capture.endTick.store(std::max(capture.endTick.load(),uint32_t(std::llround(ticks))));}capture.stop();}
void capturePerformance(int role,int vel,int gate){if(role<0||role>10)return;double delta=performanceNow-captureOrigin;if(delta<0)return;int note=role<8?getInt(kPadNote0+role*9):96+role-8;capture.push(uint32_t(std::llround(delta*PPQ)),note,vel,uint32_t(std::max(1.,gate*currentTempo.load()/60000.*PPQ)));}
void syncPerformance(){int selected=selectedPerformance();for(int q=0;q<4;++q){int key=performanceTarget(q);float value=key>=0?getNormalised(key):0;if(get(kPerformQ0+q)!=value)layerSet(kPerformQ0+q,value);}for(int n=0;n<12;++n){int state=performanceActive.load()==n?2:(isPerformanceRoll(n)&&queuedRoll.load()==n)||(isPerformanceFill(n)&&performanceQueued.load()==n-4)?1:0;if(getInt(performanceStateKey(n))!=state)layerSet(performanceStateKey(n),state);}layerSet(kPerformLoopState,performanceRunning&&!performanceMuted?1:0);int status=((performanceActive.load()+1)*5+performanceQueued.load()+1)*13+queuedRoll.load()+1;if(status!=lastPerformanceUi){lastPerformanceUi=status;changed[kPerformStatus].store(true);displayPending.store(true);}int cap=capture.state.load(),count=int(capture.count.load())/8;if(cap!=lastCaptureUi||count!=lastCaptureCount){lastCaptureUi=cap;lastCaptureCount=count;changed[kCaptureInfo].store(true);displayPending.store(true);}if(selected!=performanceLastSelected||learnSlot.load()!=lastLearnUi||int(getBool(performanceLock(selected)))!=lastLockUi){performanceLastSelected=selected;lastLearnUi=learnSlot.load();lastLockUi=getBool(performanceLock(selected));changed[kPerformPadInfo].store(true);displayPending.store(true);}}
void savePerformanceState(std::vector<uint8_t>&out){size_t start=out.size();for(int b=0;b<4;++b)for(auto&c:rollNotes[b]){uint32_t v[]={c.data.load(),c.gate.load()};for(auto x:v)for(int j=0;j<4;++j)out.push_back(uint8_t(x>>(8*j)));}uint32_t flags=0;for(int b=0;b<4;++b)if(generatedFills[b].load())flags|=1u<<b;for(int j=0;j<4;++j)out.push_back(uint8_t(flags>>(8*j)));uint32_t size=uint32_t(out.size()-start);const char magic[]="GDPF0001";out.insert(out.end(),magic,magic+8);for(int j=0;j<4;++j)out.push_back(uint8_t(size>>(8*j)));}
bool loadPerformanceState(const uint8_t*data,size_t&size){for(auto&flag:generatedFills)flag.store(false);if(size<12||std::memcmp(data+size-12,"GDPF0001",8))return false;uint32_t length=0;for(int j=0;j<4;++j)length|=uint32_t(data[size-4+j])<<(8*j);if((length!=4*MaxNotes*8&&length!=4*MaxNotes*8+4)||length>size-12)return false;size_t pos=size-12-length;for(int b=0;b<4;++b)for(auto&c:rollNotes[b]){uint32_t v[2]={};for(auto&x:v)for(int j=0;j<4;++j)x|=uint32_t(data[pos++])<<(8*j);c.data.store(v[0]);c.gate.store(mpc::clamp(v[1],5u,500u));}if(length==4*MaxNotes*8+4){uint32_t flags=0;for(int j=0;j<4;++j)flags|=uint32_t(data[pos++])<<(8*j);for(int b=0;b<4;++b)generatedFills[b].store(flags&(1u<<b));}size-=12+length;return true;}

// Additional bank is a separate suffix: existing GDPF0001 and older trailers
// retain their byte layouts. Missing suffix seeds hats without touching old rolls.
static constexpr size_t HatPerformanceTrailerSize=4*MaxNotes*8+12;
void saveHatPerformanceState(std::vector<uint8_t>&out){for(int b=4;b<8;++b)for(auto&c:rollNotes[b])for(uint32_t v:{c.data.load(),c.gate.load()})for(int j=0;j<4;++j)out.push_back(uint8_t(v>>(8*j)));const char magic[]="GDHH0001";out.insert(out.end(),magic,magic+8);uint32_t bytes=4*MaxNotes*8;for(int j=0;j<4;++j)out.push_back(uint8_t(bytes>>(8*j)));}
bool loadHatPerformanceState(const uint8_t*data,size_t&size){if(size<HatPerformanceTrailerSize||std::memcmp(data+size-12,"GDHH0001",8)){for(int n=8;n<12;++n)generatePerformance(n);return false;}uint32_t bytes=0;for(int j=0;j<4;++j)bytes|=uint32_t(data[size-4+j])<<(8*j);if(bytes!=4*MaxNotes*8)return false;size_t pos=size-HatPerformanceTrailerSize;for(int b=4;b<8;++b)for(auto&c:rollNotes[b]){uint32_t v[2]={};for(auto&x:v)for(int j=0;j<4;++j)x|=uint32_t(data[pos++])<<(8*j);c.data.store(v[0]);c.gate.store(mpc::clamp(v[1],5u,500u));}size-=HatPerformanceTrailerSize;return true;}
