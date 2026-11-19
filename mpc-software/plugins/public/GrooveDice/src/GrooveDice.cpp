#include "VelocityPattern.h"
// Groove Dice — instrument by kim. SDK MIT; optional bank assets separately licensed.
// Signal: samples -> role/choke voices -> part gain/pan -> drive/dust/room/duck -> main or stereo stems.
// Sequencer uses host PPQ with block-local sample offsets; physical/touch input alone feeds fill recording.
#include "mpc_plugin.h"
#include "dsp/wav.h"
#include "Parameters.h"
#include "SoundBank.h"
#include "Library.h"
#include "DisplayNames.h"
#include "HostRecordGate.h"
#include "RoomReverb.h"
#include <array>
#include <algorithm>
#include <limits>
#include <pthread.h>
#include "MidiCapture.h"

class GrooveDice final : public mpc::Plugin {
public:
 friend struct RuleProbe;
 static constexpr int Slots=4, MaxNotes=128, Roles=8, Samples=64, PPQ=960;
 struct Note { int tick=0, role=0, velocity=100, chance=100, gate=80; };
 struct Cell {
  std::atomic<uint32_t> data{0}, gate{80};
  Note read()const {uint32_t x=data.load(std::memory_order_acquire);return{int(x&16383),int((x>>14)&7),int((x>>17)&127),int((x>>24)&127),int(gate.load())};}
  void write(Note n){gate.store(uint32_t(mpc::clamp(n.gate,5,500)));data.store(0x80000000u|uint32_t(mpc::clamp(n.tick,0,15359))|(uint32_t(n.role&7)<<14)|(uint32_t(mpc::clamp(n.velocity,1,127))<<17)|(uint32_t(mpc::clamp(n.chance,0,100))<<24),std::memory_order_release);}
  bool valid()const{return(data.load(std::memory_order_acquire)>>31)!=0;}
 };
 struct Voice {int sample=0,role=0,librarySlot=-1,layer=0,articulation=0,delay=0;uint64_t hit=0;bool started=false,preview=false;float levelGain=1,polarity=1,hpAlpha=0,lpAlpha=1;std::array<float,2>hp{},lp{};double pos=0,step=1;float gain=0,env=0,decay=.999f,choke=1;int gate=0;bool active=false;};
 std::array<std::array<Cell,MaxNotes>,7> notes;
 std::array<std::array<Cell,MaxNotes>,7> bases;
 std::array<std::array<uint32_t,MaxNotes*2>,3> factoryBases{};std::array<int,3>factoryLengths{};
 std::array<std::atomic<bool>,7> baseValid{};
 std::array<std::atomic<int>,7> baseLengths{};
 std::array<std::atomic<float>,4> grooveBase{};std::atomic<bool>grooveBaseValid{false};
 struct Budget{int bar=-999999,count=0,extras=0,anchors=0;double last=-1e12;};std::array<Budget,8>budgets{};double lastHat=-1e12;
 std::array<std::atomic<uint32_t>,4>partSeed{{0x17432u,0x28765u,0x39876u,0x4a987u}};
 std::array<std::array<std::atomic<float>,4>,4>partOffset{};
 std::array<std::array<float,4>,4>undoOffset{};std::array<uint32_t,4>undoPartSeed{};uint32_t undoRandom=0;bool undoKeep=false;
 std::array<std::atomic<float>,4>soundRequests{};std::atomic<unsigned>soundDirty{0};std::atomic<int>loadGuardFrames{0};std::atomic<bool>pendingRoll{false};double rollAt=-1;int lastSoundGroup=-1;
 std::array<std::atomic<int>,7> lengths;
 groove::SoundBank bank;
 groove::Library library;
 std::array<std::atomic<int>,64> savedLibraryKits{};
 std::array<dsp::WavData,Samples>& sounds=bank.sounds;
 std::array<std::atomic<bool>,8> missingSample{};
 std::array<Voice,64> voices{};
 std::array<std::atomic<float>,8*72> savedKits{};
 std::array<std::atomic<bool>,8> kitValid{};
 std::atomic<int> lastAction{-1};std::atomic<unsigned> actionCount{0};
 std::atomic<unsigned> diceRolls{0};std::atomic<int> diceResult{0};std::atomic<bool> factoryDirty{false};
 std::atomic<int> recordState{0},recorded{0},fired{0};
 std::atomic<float> position{0},currentTempo{92};std::atomic<int>lastMidiNote{-1},lastMidiRole{-1};

 static mpc::PluginInfo description(){mpc::PluginInfo i;i.name="Groove Dice";i.vendor="kim";i.uniqueId=MPC_FOURCC('K','p','G','y');i.version=10701;i.latencySamples=1323;i.isInstrument=true;i.numInputs=0;i.numOutputs=16;return i;}
 GrooveDice():mpc::Plugin(description()){
  declareParameters(*this);
  for(auto& n:lengths)n.store(4*PPQ);
  for(int b=0;b<7;++b)seedPattern(b);

  if(!bank.load(mpc::contentPath("Banks/default.gdbank")))for(int s=0;s<Samples;++s)generateSound(s);
  library.load(mpc::contentPath("Banks/default.gdbank"));if(library.loaded)library.loadExpansions(mpc::contentFolder());
  allKits.clear();for(size_t j=0;j<library.factoryKits.size();++j)allKits.push_back(int(j));for(int g=0;g<4;++g)for(size_t j=0;j<library.soundPools[g].size();++j)allSounds[g].push_back(int(j));
  if(bank.loaded&&bank.patternCount){clearBank(5);lengths[5].store(bank.patternLength);for(int i=0;i<bank.patternCount;++i){auto n=bank.pattern[i];notes[5][i].write({n.tick,n.role,n.velocity,100,n.gate});}}
  for(int b=4;b<7;++b){factoryLengths[b-4]=lengths[b].load();baseLengths[b].store(lengths[b].load());for(int i=0;i<MaxNotes;++i){factoryBases[b-4][i*2]=notes[b][i].data.load();factoryBases[b-4][i*2+1]=notes[b][i].gate.load();bases[b][i].data.store(notes[b][i].data.load());bases[b][i].gate.store(notes[b][i].gate.load());}baseValid[b].store(true);}
  displayNames.build(library,bank);
  initPerformance();
  initLayers();initCymbals();
  std::vector<float> init;for(int i=0;i<numParams();++i)init.push_back(param(i).def);
  for(int key:{kDrive,kDust,kRoom,kDuck,kLowCut,kModDepth})init[key]=0;
  addProgram("Init",init);
  // A fresh instance must actually sound like program zero, even when the host omits effSetProgram(0).
  internal=true;for(int key:{kDrive,kDust,kRoom,kDuck,kLowCut,kModDepth})set(key,0);internal=false;
  for(size_t n=1;n<=library.factoryKits.size();++n){auto values=init;values[kXKit]=float(n);values[kFactoryKit]=float(std::min(size_t(96),n));values[kXQKit]=float(n)/std::max(size_t(1),library.factoryKits.size());factoryValues(library.factoryKits[n-1],values);addProgram(kitCaption(int(n)).c_str(),values);}
 }
 groove::DisplayNames displayNames;
 groove::HostRecordGate hostRecordGate;
 #include "Layers.inc"
 #include "Cymbals.inc"
 #include "Reverse.inc"
 #include "SoundBrowser.inc"
 #include "DicePool.inc"
 #include "FrontDice.inc"
 #include "RuleSelectors.inc"
 #include "Performance.inc"
 void prepare(double rate,int)override{sr=rate;setLatency(int(std::llround(rate*.030)));for(auto&fx:roomFx)fx.prepare(rate);}
 void reset()override{resetPerformance();set(kUnlockDice,0);set(kRollFill,0);set(kRollFx,0);
  hostRecordGate.reset();cymbalStart=-1;cymbalFillEnd=-1;for(auto&fx:roomFx)fx.reset();lastAutoBar=-1;budgets={};lastHat=-1e12;for(auto& v:voices)deactivate(v);lastEnd=-1e12;lastRawEnd=-1e12;transportOffset=0;previewPpq=0;fillStart=-1;fillEnd=-1;recStart=-1;recEnd=-1;recordState.store(0);recorded.store(0);ducker=0;hitCounter=lastKickHit=0;auditionPending=-1;mixGain.fill(1);
  // Saved editor context is restored; a newly created instance defaults to MAIN.
  for(int i=0;i<kParamCount;++i)last[i]=getNormalised(i);
  lastPackFilter=getInt(kPackFilter);for(int q=0;q<5;++q)catalogSeen[q].store(restoringHasX?getInt(q==0?kXKit:kXSoundKick+q-1):-1);lastLibraryKind=getInt(kLibraryKind);lastKit=getInt(kKit);lastSlot=getInt(kFillSlot);lastLength=getInt(kFillLength);lastSelected=getInt(kNoteSelect);
  for(auto& t:proxySeenTarget)t=-1;for(auto&t:soundSeenTarget)t.store(-1);
  for(int g=0;g<4;++g)lastRuleStyle[g].store(ruleStyle(g));liveDirty.store(0);noteDirty.store(0);pendingRoll.store(false);rollAt=-1;soundDirty.store(0);if(!restoringHasX)mirrorNote(true);
 }
 // Internal writes are tagged per thread; UI writes cannot be suppressed by the audio thread.
 inline static thread_local bool internal=false;
 inline static thread_local bool diceWriting=false;
 struct DiceWriteScope{bool prior;DiceWriteScope():prior(diceWriting){diceWriting=true;}~DiceWriteScope(){diceWriting=prior;}};
 bool diceAllows(int key)const{if(!diceWriting)return true;key=canonical(key);int role=key>=kXSample0&&key<=kXSample7?key-kXSample0:key>=kSample0&&key<kSample0+8*9?(key-kSample0)/9:-1;if(role>=0&&getBool(kRoleLock0+role))return false;if(key>=kDrive&&key<=kDuck||key>=kLowCut&&key<=kModDepth)return false;int lock=diceValueLock(key);return lock<kDiceLock0||lock>kDiceLock15||!getBool(lock);}
 static int canonical(int i){
  if(i==kFactoryKit)return kXKit;if(i>=kSoundKick&&i<=kSoundPerc)return kXSoundKick+i-kSoundKick;
  if(i>=kLibrarySample0&&i<=kLibrarySample7)return kXSample0+i-kLibrarySample0;
  if(i==kLibraryBeat)return kXBeat;if(i==kGrooveTemplate)return kXGroove;if(i==kKitBrowserPage)return kXKitPage;if(i==kSoundBrowserPage)return kXSoundPage;if(i==kLibraryPage)return kXLibraryPage;if(i==kLibraryFolder)return kXLibraryFolder;return i;
 }
 static int legacy(int i){if(i==kXKit)return kFactoryKit;if(i>=kXSoundKick&&i<=kXSoundPerc)return kSoundKick+i-kXSoundKick;if(i>=kXSample0&&i<=kXSample7)return kLibrarySample0+i-kXSample0;if(i==kXBeat)return kLibraryBeat;if(i==kXGroove)return kGrooveTemplate;if(i==kXKitPage)return kKitBrowserPage;if(i==kXSoundPage)return kSoundBrowserPage;if(i==kXLibraryPage)return kLibraryPage;if(i==kXLibraryFolder)return kLibraryFolder;return -1;}
 void mirrorLegacy(int i){int old=legacy(i);if(old>=0){bool prev=internal;internal=true;mpc::Plugin::set(old,get(i));internal=prev;}}
 void syncHatSelection(){
  bool previous=internal;internal=true;int v=getInt(kHatRhythmFull);
  if(v<15){if(getInt(kHatRhythm)!=v)mpc::Plugin::set(kHatRhythm,v);if(getInt(kHatRhythmPage)!=v/8)mpc::Plugin::set(kHatRhythmPage,v/8);changed[kHatRhythm].store(true);changed[kHatRhythmPage].store(true);}
  if(getInt(kHatRhythmFullPage)!=v/8)mpc::Plugin::set(kHatRhythmFullPage,v/8);changed[kHatRhythmFullPage].store(true);internal=previous;
 }
 void set(int i,float value){int key=canonical(i);if(!diceAllows(key))return;clearOrphan(key);mpc::Plugin::set(key,value);mirrorLegacy(key);if(key==kHatRhythmFull)syncHatSelection();}
 void setNormalised(int i,float value){if(!diceAllows(i))return;mpc::Plugin::setNormalised(i,value);mirrorLegacy(i);if(i==kHatRhythmFull)syncHatSelection();}
 void migrateLegacy(){bool prev=internal;internal=true;for(int i=0;i<kXKit;++i)if(canonical(i)!=i)mpc::Plugin::set(canonical(i),get(i));internal=prev;}
 std::array<std::array<char,65>,74>unresolved{};std::array<std::atomic<int>,8>orphanSample{};std::array<std::atomic<int>,64>orphanSaved{};std::array<std::atomic<int>,2>orphanMidi{};std::string unresolvedKit,unresolvedPack;std::atomic<bool>orphanKit{false},orphanPack{false};
 void clearOrphan(int key){if(restoring.load())return;if(key>=kXSample0&&key<=kXSample7){orphanSample[key-kXSample0].store(0);missingSample[key-kXSample0].store(false);}if(key==kXBeat||key==kXGroove)orphanMidi[key==kXBeat?0:1].store(0);if(key==kXKit)orphanKit.store(false);if(key==kPackFilter)orphanPack.store(false);}
 const char*unresolvedText(int ref)const{return ref>0&&ref<=74?unresolved[ref-1].data():nullptr;}
 bool restoringHasX=false;
 void parameterChanged(int i,float plain)override{
  if(internal||restoring.load()||loadGuardFrames.load()>0)return;
  if(legacyRuleAlias(i)&&ruleNotifyIndex.load()==i&&ruleNotifyThread.load()==uintptr_t(pthread_self()))return;
  if(performanceParameter(i,plain))return;
  if(ruleSelectorParameter(i))return;
  if(layerParameter(i,plain))return;
  clearOrphan(canonical(i));if(canonical(i)!=i){float raw=getNormalised(i);set(canonical(i),plain);bool prev=internal;internal=true;mpc::Plugin::setNormalised(i,raw);internal=prev;return;}mirrorLegacy(i);
  if(i>=kXQKit&&i<=kXQPerc){int target=i==kXQKit?kXKit:kXSoundKick+i-kXQKick;const auto&list=i==kXQKit?kitItems():soundItems(i-kXQKick);int pick=mpc::clamp(int(std::lround(plain*list.size())),0,int(list.size()));catalogSeen[i-kXQKit].store(pick?list[pick-1]+1:0);bool prev=internal;internal=true;set(target,pick?list[pick-1]+1:0);internal=prev;if(i==kXQKit)factoryDirty.store(true);else{soundRequests[i-kXQKick].store(get(target));soundDirty.fetch_or(1u<<(i-kXQKick));}changed[target].store(true);displayPending.store(true);}
  if(i==kXKit||i>=kXSoundKick&&i<=kXSoundPerc){int q=i==kXKit?kXQKit:kXQKick+i-kXSoundKick;const auto&list=i==kXKit?kitItems():soundItems(i-kXSoundKick);int pos=0;for(size_t j=0;j<list.size();++j)if(list[j]+1==getInt(i))pos=int(j)+1;bool prev=internal;internal=true;set(q,list.empty()?0.f:float(pos)/list.size());internal=prev;changed[q].store(true);}
  if(i==kHatGrid){static const int legacy[]={0,3,6,8};internal=true;set(kHatRhythm,legacy[getInt(kHatGrid)]);internal=false;changed[kHatRhythm].store(true);}
  if(i==kHatRhythm||i==kHatGrid)set(kHatRhythmFull,getInt(kHatRhythm));
  if(i==kHatRhythmFull)syncHatSelection();
  if(i==kKitLock){changed[kScope].store(true);changed[kDiceInfo].store(true);displayPending.store(true);}
  if(i>=kRuleKick&&i<=kRulePerc){int g=i-kRuleKick;bool old=internal;internal=true;set(kRuleKickStyle+g,plain);set(kRulePickFullKick+g,std::max(0.f,plain-1));internal=old;syncRuleStyle(g);}
  if(i>=kRuleKickStyle&&i<=kRulePercStyle)syncRuleStyle(i-kRuleKickStyle);
  if(i==kHatRhythmFull||i==kHatRhythm||i==kHatGrid)liveDirty.fetch_or(4);
  if(i==kGhost)liveDirty.fetch_or(2);
  if(i==kRandomAmt)liveDirty.fetch_or(15);
  if(i==kXKit)factoryDirty.store(true);
  if(i>=kXSoundKick&&i<=kXSoundPerc){soundRequests[i-kXSoundKick].store(plain);soundDirty.fetch_or(1u<<(i-kXSoundKick));}
  if(i>=kSwing&&i<=kRndNudge)grooveBaseValid.store(false);
  if(i==kKit){internal=true;set(kXKit,0);internal=false;}
  if(i>=kXSample0&&i<=kXSample7){internal=true;set(kXKit,0);internal=false;}
  if(i==kPattern){internal=true;set(kXBeat,0);internal=false;}
  if(i==kEditor||(i>=kXSample0&&!(i>=kRuleKickStyle&&i<=kRulePercStyle)))displayPending.store(true);
  for(int r=0;r<8;++r)if(i==kSample0+r*9){missingSample[r].store(false);internal=true;set(kXKit,0);set(kXSample0+r,0);internal=false;displayPending.store(true);}
  if(i>=kQ1&&i<=kQ16){int target=proxyTarget(i-kQ1);if(target>=0){float before=getNormalised(target);internal=true;setNormalised(target,plain);internal=false;if(target>=kXQKit&&target<=kXQPerc)parameterChanged(target,plain);if(target>=kRuleKickStyle&&target<=kRulePercStyle)syncRuleStyle(target-kRuleKickStyle);if(isRuleSelector(target))ruleSelectorParameter(target);if(target>=kPerformHit0&&target<=kPerformLength7&&(getInt(kPerformSelect)<4||generatedFills[getInt(kPerformSelect)-4].load()))rollRegenerate.fetch_or(1u<<getInt(kPerformSelect));if(target>=kPerformHit0&&target<=kPerformLength7&&getInt(kPerformSelect)>=4&&!generatedFills[getInt(kPerformSelect)-4].load())fillTimingDirty.store(true);if(target==kHatRhythmFull)liveDirty.fetch_or(4);if(getNormalised(target)!=before&&!(target>=kRuleKickStyle&&target<=kRulePercStyle)&&!isRuleSelector(target)){changed[target].store(true);if(param(target).kind==mpc::Param::Kind::Choice)displayPending.store(true);}if(target==kHatRhythmFull){internal=true;set(kHatRhythmFullPage,getInt(kHatRhythmFull)/8);internal=false;changed[kHatRhythmFullPage].store(true);}if(target>=kXSoundKick&&target<=kXSoundPerc){soundRequests[target-kXSoundKick].store(get(target));soundDirty.fetch_or(1u<<(target-kXSoundKick));}if(target>=kSwing&&target<=kRndNudge)grooveBaseValid.store(false);if(target>=kNoteNudge&&target<=kNoteChance)noteDirty.fetch_or(1u<<(target-kNoteNudge));}else{internal=true;setNormalised(i,0);internal=false;}}
  else if(i>=kNoteNudge&&i<=kNoteChance)noteDirty.fetch_or(1u<<(i-kNoteNudge));
 }
 bool formatValue(int i,float plain,std::string& out)override{
  if(performanceFormat(i,out))return true;
  if(poolFormat(i,out)||layerFormat(i,out))return true;
  if(canonical(i)!=i)return formatValue(canonical(i),plain,out);
  if(i==kXQKit){int index=getInt(kXKit);if(index>0&&index<=int(library.factoryKits.size())){out=kitCaption(index);}else out="Custom Kit";return true;}
  if(i>=kXQKick&&i<=kXQPerc)return formatValue(kXSoundKick+i-kXQKick,get(kXSoundKick+i-kXQKick),out);
  if(i==kXKit){int index=getInt(i);out=kitCaption(index);return true;}
  if(i==kPackInfo){int p=getInt(kPackFilter);out=p>0&&p<=int(library.packs.size())?shortCaption(library.packs[p-1].name,31):"ALL EXPANSIONS";return true;}
  if(i>=kKitRow0&&i<=kKitRow7){int j=getInt(kXKitPage)*8+i-kKitRow0;out=j==0?"Custom Kit":j>0&&j<=int(kitItems().size())?kitCaption(kitItems()[j-1]+1):"";return true;}
  if(i==kActionInfo){int a=lastAction.load();std::string name=a>=0?param(a).name:"";
   if(a>=kKitPick0&&a<=kKitPick7)name="KIT SELECTED";else if(a==kPackPrev||a==kPackNext)name="EXPANSION CHANGED";else if(a>=kSoundPick0&&a<=kSoundPick7)name="SOUND SELECTED";else if(a>=kLibraryPick0&&a<=kLibraryPick7)name="LIBRARY SELECTED";
   else if(a==kSoundPrev||a==kLibraryPrev||a==kKitPagePrev)name="PREVIOUS PAGE";else if(a==kSoundNext||a==kLibraryNext||a==kKitPageNext)name="NEXT PAGE";
   else if(a==kKitPrev||a==kKitNext)name="KIT CHANGED";else if(a==kPatternPrev||a==kPatternNext)name="PATTERN CHANGED";else if(a==kFactoryReload)name="KIT SELECTED";
   else if(a==kLibraryBack)name="FOLDERS";else if(a==kLibraryLegacy)name="CORE SOUNDS";else if(a==kLibraryBeatOff)name="BEAT OFF";else if(a==kGrooveOff)name="GROOVE OFF";
   else if(a==kGridSelect||a==kGridTap)name="NOTE SELECTED";
   if(a>=kSoundSamplePrev0&&a<=kSoundSampleNext3)name="SAMPLE CHANGED";out=a>=0?name:"READY";return true;}
  if(i>=kXSoundKick&&i<=kXSoundPerc){int g=i-kXSoundKick,id=getInt(kXSample0+(g==0?0:g==1?1:g==2?2:7));out=g==3?(getInt(kXSoundPerc)>0&&getInt(kXSoundPerc)<=int(library.soundPools[3].size())?soundSetCaption(getInt(kXSoundPerc)-1):"CUSTOM SET"):id>0&&id<=int(library.entries.size())?sampleCaption(id,14):"CUSTOM";return true;}
  if(i>=kSoundRow0&&i<=kSoundRow7){int g=getInt(kSoundBrowserGroup),row=getInt(kXSoundPage)*8+i-kSoundRow0;const auto&list=soundItems(g);int index=row<int(list.size())?list[row]:-1;out=index>=0?(g==3?soundSetCaption(index):sampleCaption(library.soundPools[g][index].sounds[0],30)):"";return true;}
  if(i==kSoundInfo){int count=int(soundItems(getInt(kSoundBrowserGroup)).size());out=std::to_string(count)+" SOUNDS | "+std::to_string(getInt(kXSoundPage)+1)+" / "+std::to_string(std::max(1,(count+7)/8));return true;}
  if(i==kDiceInfo&&pendingRoll.load()){out="NEXT BAR";return true;}
  if(i==kDiceInfo){int result=diceResult.load();out=result==1?"AMOUNT ZERO":result==2?"KIT DATA MISSING":result==3?"RHYTHM LOCK LIMIT":getBool(kKitLock)?"KIT FIXED":diceRolls.load()?"DICE APPLIED":"READY";return true;}
  if(i==kXKitPage){out=std::to_string(getInt(i)+1)+" / "+std::to_string((int(kitItems().size())+8)/8);return true;}
  if(isRuleSelector(i)){out=param(i).choices[getInt(i)];if(out=="GHOST BEFORE")out="PRE GHOST";if(out=="GHOST AFTER")out="POST GHOST";if(out=="DOUBLE GHOST")out="2 GHOSTS";if(out=="ANSWER SNARE")out="ANSWER";if(out=="SYNCOPATED")out="SYNCOPATE";return true;}
  if(i>=kRuleKickStyle&&i<=kRulePercStyle){out=param(i).choices[mpc::clamp(getInt(i),0,7)];if(out=="GHOST BEFORE")out="PRE GHOST";if(out=="GHOST AFTER")out="POST GHOST";if(out=="DOUBLE GHOST")out="2 GHOSTS";if(out=="ANSWER SNARE")out="ANSWER";if(out=="SYNCOPATED")out="SYNCOPATE";return true;}
  if(i==kHatRhythm||i==kHatRhythmFull){static const char* captions[]={"1/4","1/8","1/16","1/8D","1/16D","1/4T","1/8T","1/16T"};int v=getInt(i);out=captions[v/3];if(v%3)out+=v%3==1?" FILL":" ROLL";return true;}
  if(i==kScope){static const char* names[]={"Extra: None","Extra: Pattern","Extra: Groove","Extra: Fill","Extra: All","Extra: FX"};out=names[getInt(i)];return true;}
  for(int r=0;r<8;++r)if(i==kSample0+r*9&&getInt(kXSample0+r)>0){out="LIBRARY";return true;}
  if(i==kLibraryCurrentSound){static const char* names[]={"KICK","SNARE","CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};int role=getInt(kLibraryRole),id=getInt(kXSample0+role);out=std::string(names[role])+" / ";if(id>0&&id<=int(library.entries.size()))out+=sampleCaption(id,31-out.size());else out+=shortCaption(legacyCaption(role),31-out.size());return true;}
  if(i==kLibraryPlaybackInfo){int beat=getInt(kXBeat),groove=getInt(kXGroove);out=beat>0&&beat<=int(library.categories[3].size())?shortCaption(library.entries[library.categories[3][beat-1]].name,20):param(kPattern).choices[getInt(kPattern)];out+=" / GRV "+(groove?std::to_string(groove):std::string("OFF"));return true;}
  if(i==kPattern&&getInt(kXBeat)>0&&getInt(kXBeat)<=int(library.categories[3].size())){const auto& name=library.entries[library.categories[3][getInt(kXBeat)-1]].name;out=name.substr(name.find_last_of('/')+1);if(out.size()>22)out=out.substr(0,19)+"...";return true;}
  if(i==kPreviewBpm){char text[32];std::snprintf(text,sizeof(text),"%.0f BPM",currentTempo.load());out=text;return true;}
  if(i>=kLibraryRow0&&i<=kLibraryRow7){int kind=getInt(kLibraryKind),index=getInt(kXLibraryPage)*8+i-kLibraryRow0,folder=getInt(kXLibraryFolder);if(kind<3&&folder==0){const auto&list=folderItems(kind);out="";if(index<int(list.size())){out="> "+shortCaption(displayNames.folders[list[index]],28);}}else{auto&list=browserItems();if(index>=int(list.size())){out="";return true;}out=sampleCaption(list[index]+1,30);}return true;}
  if(i==kLibraryInfo){int kind=getInt(kLibraryKind),folder=getInt(kXLibraryFolder),count=browserCount();out=std::string(kind<3&&folder==0?"FOLDERS ":"FILES ")+std::to_string(getInt(kXLibraryPage)+1)+" / "+std::to_string(std::max(1,(count+7)/8))+"  |  "+std::to_string(count);return true;}
  if(i>=kXSample0&&i<=kXSample7){int id=getInt(i);out=id>0&&id<=int(library.entries.size())?sampleCaption(id,30):"Core Sounds";return true;}
  if(i==kXBeat||i==kXGroove){int k=i==kXBeat?3:4,id=getInt(i);out=id>0&&id<=int(library.categories[k].size())?shortCaption(library.entries[library.categories[k][id-1]].name,26):"OFF";return true;}
  if(i==kBankInfo){out=library.loaded?"LIBRARY "+std::to_string(library.categories[0].size()+library.categories[1].size()+library.categories[2].size())+" WAV / "+std::to_string(library.categories[3].size()+library.categories[4].size())+" MIDI":bank.loaded?"BANK · 64 SOUNDS":"BANK MISSING · TEST SOUNDS";for(int r=0;r<8;++r)if(getInt(kXSample0+r)>0&&library.active[r].load()!=getInt(kXSample0+r))out=library.failed[r].load()?"LIBRARY SAMPLE ERROR":"LOADING SOUND";for(auto& m:missingSample)if(m.load())out="MISSING SAMPLE";if(library.expansionErrors>0)out="EXPANSION LOAD ERROR / "+std::to_string(library.expansionErrors);if(lastMidiNote.load()>=0){static const char* role[]={"KICK","SNARE","CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};int r=lastMidiRole.load();out=std::string(library.loaded?"LIBRARY | ":bank.loaded?"BANK64 | ":"TEST | ")+std::to_string(lastMidiNote.load())+" "+(r>=0?role[r]:"UNASSIGNED");}bool missing=false;for(auto&m:missingSample)missing|=m.load();if(missing)out="MISSING SAMPLE";else if(orphanMidi[0].load()>0)out="MISSING PATTERN";else if(orphanMidi[1].load()>0)out="MISSING GROOVE";return true;}
  if(i>=kQ1&&i<=kQ16){int t=proxyTarget(i-kQ1);if(t<0){out="--";return true;}if(isRuleSelector(t)||t>=kRuleKickStyle&&t<=kRulePercStyle||t>=kXQKit&&t<=kXQPerc||t>=kXSoundKick&&t<=kXSoundPerc||t==kHatRhythmFull)return formatValue(t,get(t),out);out=param(t).display(get(t));if(!param(t).unit.empty())out+=" "+param(t).unit;return true;}
  if(i==kStatus){static const char* status[]={"READY","COUNT-IN","RECORDING","RECORDED","QUEUED","PLAYING","FULL"};out=status[mpc::clamp(recordState.load(),0,6)];return true;}
  if(i==kRecordedCount){out=std::to_string(countNotes(getInt(kFillSlot)));return true;}
  if(i>=kGrid0&&i<=kGrid63){out=get(i)>.5f?"HIT":"EMPTY";return true;}
  if(i==kNoteSelect){int n=getInt(kNoteSelect);int slot=getInt(kFillSlot);if(notes[slot][n].valid()){auto e=notes[slot][n].read();static const char* names[]={"KICK","SNARE","CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};out=std::string(names[e.role])+" / "+std::to_string(1+e.tick/240);}else out="EMPTY";return true;}
  if(i==kPosition){out="LOOP 1/1";if(fillStart>=0&&recordState.load()==5){int total=std::max(1,(lengths[activeFill].load()+4*PPQ-1)/(4*PPQ)),bar=mpc::clamp(1+int(std::max(0.,double(position.load())-fillStart)/4),1,total);out="FILL "+std::to_string(bar)+"/"+std::to_string(total);}return true;}
  if(!param(i).unit.empty()){out=param(i).display(get(i))+" "+param(i).unit;return true;}
  return false;
 }
 void process(mpc::ProcessContext& c)override{
  markRenderThread();
  std::array<int,groove::Library::Channels> desired{};for(int r=0;r<8;++r)desired[r]=getInt(kXSample0+r);for(int slot=0;slot<6;++slot)desired[8+slot]=layerMissing[slot].load()?0:getInt(kL0Sample+slot*LayerFields);for(int kind=0;kind<3;++kind)desired[14+kind]=(cymbalWanted(kind)||getBool(kCymbalView))?cymbalSource(kind):0;library.poll(desired);
  sr=c.sampleRate;const double bpm=c.transport.tempoValid?mpc::clamp(c.transport.bpm,20.,300.):get(kPreviewBpm);
  currentTempo.store(float(bpm));
  const double step=bpm/(60.*sr);
  bool hostRunning=getBool(kFollow)&&c.transport.playing&&c.transport.ppqValid;
  // Do not let Preview bypass host record pre-roll. Keep the raw host
  // block origin when suspended; the release/seek starts at its real position.
  bool hostCountIn=hostRecordGate.suspended(c.transport,step*c.numFrames);
  bool run=!hostCountIn&&((getBool(kFollow)&&c.transport.playing)||getBool(kPreview));
  double rawStart=hostRunning?c.transport.ppqPos:previewPpq;
  double loopLength=c.transport.cycleEndPpq-c.transport.cycleStartPpq;
  if(hostRunning&&lastRawEnd>-1e10&&rawStart<lastRawEnd-.02&&c.transport.cycling&&loopLength>0&&std::abs(rawStart+loopLength-lastRawEnd)<.02)transportOffset+=loopLength;
  else if(hostRunning&&lastRawEnd>-1e10&&std::abs(rawStart-lastRawEnd)>.02)transportOffset=0;
  double start=hostRunning?rawStart+transportOffset:previewPpq;
  if(!run&&!hostCountIn)start=previewPpq;
  double end=start+step*c.numFrames;position.store(float(start));
  if(run&&std::abs(start-lastEnd)>.02&&lastEnd>-1e10){stopCaptureAt(performanceNow);pendingRoll.store(false);displayPending.store(true);rollAt=-1;lastAutoBar=-1;budgets={};lastHat=-1e12;fillStart=fillEnd=-1;cymbalFillEnd=-1;recordState.store(0);recStart=recEnd=-1;for(auto&v:voices)deactivate(v);}
  if(!run&&wasRunning){stopCaptureAt(performanceNow);pendingRoll.store(false);displayPending.store(true);rollAt=-1;lastAutoBar=-1;budgets={};lastHat=-1e12;fillStart=fillEnd=-1;cymbalFillEnd=-1;recStart=recEnd=-1;recordState.store(0);for(auto&v:voices)deactivate(v);}
  wasRunning=run;
  performanceBlock(start,end,step,run,hostCountIn,hostRunning);
  int guard=loadGuardFrames.load();bool loading=guard>0;if(loading){loadGuardFrames.fetch_sub(std::min(guard,c.numFrames));if(guard<=c.numFrames){migratePerformanceDefaultNotes();for(auto&t:proxySeenTarget)t=-1;for(auto&t:soundSeenTarget)t.store(-1);}}
  const auto command=[&](int i){float x=getNormalised(i);bool changed=x!=last[i];last[i]=x;
   // Dedicated roll actions are momentary, not saved musical settings. Consume
   // and acknowledge their cursor so the next block cannot repeat the action.
   if(i==kRollFill||i==kRollFx){changed=changed&&x>.5f;if(x!=0){layerSet(i,0);last[i]=0;}}
   if(changed&&!loading){lastAction.store(i);actionCount.fetch_add(1);this->changed[kActionInfo].store(true);}return changed&&!loading;};
  if(command(kUnlockDice)&&getBool(kUnlockDice)){
   for(int n=0;n<8;++n)layerSet(performanceLock(n),0);
   for(int key:kExtraDiceValueLocks)layerSet(key,0);
   for(int key=kDiceLock0;key<=kDiceLock15;++key)layerSet(key,0);
   layerSet(kKitLock,0);layerSet(kPatternLock,0);
   for(int key=kFillDiceLock0;key<=kFillDiceLock3;++key)layerSet(key,0);
   for(int r=0;r<8;++r)layerSet(kRoleLock0+r,0);
   for(int l=0;l<6;++l)layerSet(kL0Sample+l*LayerFields+LLock,0);
   for(int l=0;l<3;++l)layerSet(kCym0Sample+l*LayerFields+LLock,0);
   layerSet(kUnlockDice,0);syncLayerUi(true);queueLayerUi();changed[kDiceInfo].store(true);displayPending.store(true);
  }
  processPoolCommands(command);
  if(command(kSoundRoll)&&!getBool(kKitLock)){DiceWriteScope diceScope;snapshotUndo();int part=getInt(kSoundPart);for(int row=0;row<(part==2?2:part==3?4:1);++row)rollLayers(part<2?part:part==2?2+row:4+row);}
  if(command(kRollFill)&&recordState.load()!=2){snapshotUndo();mutate(getInt(kFillSlot),get(kRandomAmt)*.01f);mirrorNote();queueAudioUi();}
  if(command(kRollFx)){snapshotUndo();float amt=get(kRandomAmt)*.01f;if(amt>0){set(kDrive,(random()%31)*amt);set(kDust,(random()%36)*amt);set(kRoom,(random()%25)*amt);set(kDuck,(random()%40)*amt);}queueAudioUi();}
  if(command(kSoundUndo)&&haveUndo)restoreUndo();
  if(command(kSoundAudition))queueSamplePreview(getInt(kSoundRow));
  for(int row=0;row<4;++row){if(command(kSoundBrowse0+row*2))openSampleBrowser(row);if(command(kSoundPreview0+row*2))queueSamplePreview(row);}
  if(command(kSamplePrev)){layerSet(kSampleBrowserPage,std::max(0,getInt(kSampleBrowserPage)-1));queueLayerUi();}
  if(command(kSampleNext)){layerSet(kSampleBrowserPage,std::min(std::max(0,(sampleCount(getInt(kSoundRow))-1)/8),getInt(kSampleBrowserPage)+1));queueLayerUi();}
  for(int row=0;row<8;++row)if(command(kSamplePick0+row)){selectSample(getInt(kSoundRow),getInt(kSampleBrowserPage)*8+row);syncLayerUi(true);queueLayerUi();}
  for(int row=0;row<4;++row){bool prev=command(kSoundSamplePrev0+row*2),next=command(kSoundSamplePrev0+row*2+1);if(prev||next)stepSoundSample(row,next?1:-1);}
  for(int i=0;i<4;++i)if(command(kKitPrev+i)){int target=i<2?kXKit:kPattern;int count=i<2?int(kitItems().size())+1:int(param(target).choices.size()),position=i<2?kitPosition(getInt(target)):getInt(target),next=(position+(i%2?1:count-1))%count;internal=true;set(target,i<2&&next>0?kitItems()[next-1]+1:next);if(target==kPattern)set(kXBeat,0);if(target==kXKit)factoryDirty.store(true);internal=false;changed[target].store(true);displayPending.store(true);}
  int kind=getInt(kLibraryKind);if(kind!=lastLibraryKind){lastLibraryKind=kind;internal=true;set(kXLibraryPage,0);set(kXLibraryFolder,0);internal=false;displayPending.store(true);}
  if(!loading&&getInt(kXKitPage)>int(kitItems().size())/8){internal=true;set(kXKitPage,int(kitItems().size())/8);internal=false;changed[kXKitPage].store(true);}
  int maxPage=std::max(0,(browserCount()-1)/8);if(getInt(kXLibraryPage)>maxPage){internal=true;set(kXLibraryPage,maxPage);internal=false;}
  if(command(kLibraryPrev)){internal=true;set(kXLibraryPage,std::max(0,getInt(kXLibraryPage)-1));internal=false;displayPending.store(true);}
  if(command(kLibraryNext)){internal=true;set(kXLibraryPage,std::min(maxPage,getInt(kXLibraryPage)+1));internal=false;displayPending.store(true);}
  if(command(kKitPagePrev)){internal=true;set(kXKitPage,std::max(0,getInt(kXKitPage)-1));internal=false;changed[kXKitPage].store(true);displayPending.store(true);}
  if(command(kKitPageNext)){internal=true;set(kXKitPage,std::min(int(kitItems().size())/8,getInt(kXKitPage)+1));internal=false;changed[kXKitPage].store(true);displayPending.store(true);}
  if(command(kLibraryLegacy)){internal=true;set(kXSample0+getInt(kLibraryRole),0);missingSample[getInt(kLibraryRole)]=false;internal=false;}
  if(command(kLibraryBeatOff)){internal=true;set(kXBeat,0);internal=false;}
  if(command(kGrooveOff)){internal=true;set(kXGroove,0);internal=false;}
  if(command(kLibraryBack)){internal=true;set(kXLibraryFolder,0);set(kXLibraryPage,0);internal=false;displayPending.store(true);}
  for(int row=0;row<8;++row)if(command(kLibraryPick0+row)){int index=getInt(kXLibraryPage)*8+row;if(index<browserCount()){internal=true;if(kind<3&&getInt(kXLibraryFolder)==0){set(kXLibraryFolder,folderItems(kind)[index]+1);set(kXLibraryPage,0);}else{int id=browserItems()[index]+1;if(kind<3){int role=getInt(kLibraryRole);set(kXKit,0);changed[kXKit].store(true);set(kXSample0+role,id);changed[kXSample0+role].store(true);missingSample[role]=false;}else{int position=0;for(size_t j=0;j<library.categories[kind].size();++j)if(library.categories[kind][j]+1==id)position=int(j)+1;set(kind==3?kXBeat:kXGroove,position);}set(kLibraryOpen,0);}internal=false;displayPending.store(true);}}
  bool packPrev=command(kPackPrev),packNext=command(kPackNext);if(packPrev||packNext){int count=int(library.packs.size())+1;internal=true;set(kPackFilter,(getInt(kPackFilter)+(packNext?1:count-1))%count);internal=false;changed[kPackFilter].store(true);displayPending.store(true);}
  if(loading)lastPackFilter=getInt(kPackFilter);if(!loading&&getInt(kPackFilter)!=lastPackFilter){lastPackFilter=getInt(kPackFilter);for(auto&n:catalogSeen)n.store(-1);internal=true;set(kXKitPage,0);set(kXSoundPage,0);set(kXLibraryPage,0);set(kXLibraryFolder,0);internal=false;displayPending.store(true);}
  for(int row=0;row<8;++row)if(command(kKitPick0+row)){int j=getInt(kXKitPage)*8+row;if(j==0||j>0&&j<=int(kitItems().size())){internal=true;set(kXKit,j==0?0:kitItems()[j-1]+1);set(kKitOpen,0);internal=false;factoryDirty.store(true);displayPending.store(true);}}
  if(command(kFactoryReload))factoryDirty.store(true);
  if(factoryDirty.exchange(false)){internal=true;applyFactory(getInt(kXKit));internal=false;}
  int sg=getInt(kSoundBrowserGroup);if(loading)lastSoundGroup=sg;if(!loading&&sg!=lastSoundGroup){lastSoundGroup=sg;internal=true;set(kXSoundPage,0);internal=false;}
  int pages=std::max(0,(int(soundItems(sg).size())-1)/8);
  if(command(kSoundPrev)){internal=true;set(kXSoundPage,std::max(0,getInt(kXSoundPage)-1));internal=false;displayPending.store(true);}
  if(command(kSoundNext)){internal=true;set(kXSoundPage,std::min(pages,getInt(kXSoundPage)+1));internal=false;displayPending.store(true);}
  if(!loading&&getInt(kXSoundPage)>pages){internal=true;set(kXSoundPage,pages);internal=false;}
  for(int row=0;row<8;++row)if(command(kSoundPick0+row)){int visible=getInt(kXSoundPage)*8+row;const auto&list=soundItems(sg);int index=visible<int(list.size())?list[visible]+1:0;if(index>0){internal=true;set(kXSoundKick+sg,index);set(kSoundOpen,0);internal=false;soundRequests[sg].store(float(index));soundDirty.fetch_or(1u<<sg);}}
  unsigned sd=soundDirty.exchange(0);for(int g=0;g<4;++g)if(sd&(1u<<g))applySound(g,int(std::lround(soundRequests[g].load())));
  if(command(kRoll)){if(run){pendingRoll.store(true);rollAt=std::ceil((start+1e-8)/4)*4;}else{pendingRoll.store(false);snapshotUndo();roll();liveDirty.store(0);}}
  if(pendingRoll.load()&&run&&rollAt>=start-1e-8&&rollAt<end){snapshotUndo();roll();liveDirty.store(0);pendingRoll.store(false);rollAt=-1;}
  if(command(kUndo)){pendingRoll.store(false);liveDirty.store(0);if(haveUndo)restoreUndo();}
  if(command(kKeep)){keep=true;}
  if(command(kSaveKit))saveKit();if(command(kLoadKit))loadKit();
  if(command(kCancel)){recStart=recEnd=-1;fillStart=fillEnd=-1;cymbalFillEnd=-1;recordState.store(0);}
  if(command(kRecord)){
   if(!run){internal=true;set(kPreview,1);internal=false;run=true;previewPpq=0;start=0;end=start+step*c.numFrames;}
   snapshotUndo();recSlot=getInt(kFillSlot);recLength=lengthForChoice(getInt(kFillLength));recMode=getInt(kRecMode);
   recStart=std::ceil((start-1e-9)/4.)*4.+4*getInt(kCountIn);if(recStart<start-1e-9)recStart+=4;
   recEnd=recStart+double(recLength)/PPQ;recordState.store(1);recorded.store(0);recCleared=false;
  }
  if(command(kTrigger))queueFill(start);
  bool gridSelect=command(kGridSelect),gridTap=command(kGridTap);if(gridSelect||gridTap){baseValid[getInt(kFillSlot)].store(false);int cell=getInt(kGridSelect)-1;if(cell>=0){int role=gridRole(getInt(kFillGridPage),cell/16),step=cell%16+getInt(kGridBar)*16;int selected=-1;for(int i=0;i<MaxNotes;++i)if(notes[getInt(kFillSlot)][i].valid()){auto n=notes[getInt(kFillSlot)][i].read();if(n.role==role&&n.tick/240==step){selected=i;break;}}if(selected<0){for(int i=0;i<MaxNotes;++i)if(!notes[getInt(kFillSlot)][i].valid()){selected=i;markRecordedFill(getInt(kFillSlot));notes[getInt(kFillSlot)][i].write({step*240,role,100,100,80});break;}}if(selected>=0){internal=true;set(kNoteSelect,selected);internal=false;mirrorNote();}}}
  if(command(kNoteDelete)){markRecordedFill(getInt(kFillSlot));baseValid[getInt(kFillSlot)].store(false);int slot=getInt(kFillSlot),n=getInt(kNoteSelect);notes[slot][n].data.store(0);}
  if(getInt(kKit)!=lastKit){lastKit=getInt(kKit);internal=true;for(int v=0;v<8;++v){set(kSample0+v*9,lastKit);set(kXSample0+v,0);}internal=false;}
  unsigned live=liveDirty.exchange(0);if(live&&!loading)refreshMain(live);
  int slot=getInt(kFillSlot),li=getInt(kFillLength);
  int maxBar=mpc::clamp((lengths[slot].load()-1)/(4*PPQ),0,3);if(getInt(kGridBar)>maxBar){internal=true;set(kGridBar,maxBar);internal=false;changed[kGridBar].store(true);}
  if(slot!=lastSlot){lastSlot=slot;internal=true;set(kFillLength,choiceForLength(lengths[slot].load()));internal=false;mirrorNote();}
  else if(li!=lastLength&&recordState.load()!=2){baseValid[slot].store(false);lengths[slot].store(lengthForChoice(li));}
  lastLength=getInt(kFillLength);
  unsigned dirty=noteDirty.exchange(0);if(dirty)editNote(dirty);
  if(getInt(kNoteSelect)!=lastSelected){lastSelected=getInt(kNoteSelect);mirrorNote();}
  if(recStart>=0&&start>=recEnd-1e-9)finishRecord();
  // Schedule in advance of rendering this block, including neighboring cycles for negative nudges.
  eventCount=0;
  if(run){if(lastEnd<-1.e10||std::abs(start-lastEnd)>.02)cymbalStart=start;queueAutoFill(start,end);scheduleBank(4+getInt(kPattern),start,end,step,false);if(fillStart>=0&&!performanceFill)scheduleBank(activeFill,start,end,step,true);scheduleCymbals(start,end,step);}
  for(int v=0;v<8;++v)if(command(kPad0+v*9)){addEvent(0,v,100,true);}
  runSamplePreview();
  int midi=0;
  const float drive=get(kDrive)*.01f,dust=get(kDust)*.01f,roomAmount=get(kRoom)*.01f,duck=get(kDuck)*.01f;
  const float masterTarget=get(kMaster),modDepth=get(kModDepth)*.01f;double phaseInc=get(kModRate)/sr;
  float hpAlpha=float(1.-std::exp(-6.283185307*get(kLowCut)/sr));
  std::array<float,4>mixTarget{};for(int g=0;g<4;++g)mixTarget[g]=std::pow(10.f,get(kMixKick+g)/20.f);
  std::array<float,17>levelTarget{};for(int r=0;r<8;++r)levelTarget[r]=std::pow(10.f,get(kLevel0+r*9)/20.f);for(int slot=0;slot<6;++slot)levelTarget[8+slot]=std::pow(10.f,get(kL0Level+slot*LayerFields)/20.f);
  std::array<bool,17>audible{};for(int r=0;r<8;++r)audible[r]=layerAudible(r,0);for(int slot=0;slot<6;++slot)audible[8+slot]=layerAudible(slot/3,slot%3+1);
  for(int kind=0;kind<3;++kind){levelTarget[14+kind]=std::pow(10.f,get(kCym0Level+kind*LayerFields)/20.f);audible[14+kind]=cymbalAudible(kind);}
  const float gainSmooth=float(1.-std::exp(-1./(sr*.020)));
  for(int frame=0;frame<c.numFrames;++frame){
   const double now=start+frame*step;performanceNow=now;
   if(run&&recStart>=0&&now>=recStart&&now<recEnd&&!recCleared){if(recMode==0)clearBank(recSlot);markRecordedFill(recSlot);lengths[recSlot].store(recLength);recCleared=true;recordState.store(2);}
   if(run&&recStart>=0&&now>=recEnd)finishRecord();
   if(fillStart>=0&&now>=fillStart&&now<fillEnd&&recordState.load()!=1&&recordState.load()!=2)recordState.store(5);
   if(fillEnd>=0&&now>=fillEnd){fillStart=fillEnd=-1;if(recordState.load()==5||recordState.load()==4)recordState.store(0);}

   while(midi<c.numMidiEvents&&c.midi[midi].sampleOffset<=frame){auto e=c.midi[midi++];if(performanceMidi(e,now,hostCountIn))continue;if(e.isNoteOn()){int r=mapNote(e.note());lastMidiNote.store(e.note());lastMidiRole.store(r);displayPending.store(true);if(r>=0){trigger(r,e.velocity());recordInput(now,r,e.velocity());}}if(e.isAllNotesOff()){stopPerformance();for(auto&v:voices)deactivate(v);recStart=recEnd=-1;fillStart=fillEnd=-1;cymbalFillEnd=-1;recordState.store(0);}}
   performanceFrame(now,step,run,hostCountIn);
   for(int e=0;e<eventCount;++e)if(events[e].offset==frame&&!performanceSuppress(events[e].role,now)){trigger(events[e].role,events[e].velocity,events[e].gate);if(events[e].input)recordInput(now,events[e].role,events[e].velocity);}
   driveS+=.002f*(drive-driveS);dustS+=.002f*(dust-dustS);roomS+=.002f*(roomAmount-roomS);duckS+=.002f*(duck-duckS);
   for(int g=0;g<4;++g)mixGain[g]+=gainSmooth*(mixTarget[g]-mixGain[g]);
   float bus[8][2]{};
   for(auto& v:voices){if(!v.active)continue;if(!v.preview&&!audible[v.articulation?13+v.articulation:v.layer?8+v.role*3+v.layer-1:v.role]){if(!v.started){deactivate(v);continue;}v.choke=std::min(v.choke,.98f);}if(v.delay>0){--v.delay;continue;}if(!v.started)startLayerVoice(v);auto& s=v.librarySlot>=0?library.slots[v.librarySlot].sound:v.articulation?cymbalFallback[v.articulation-1]:sounds[v.sample];int idx=int(v.pos);if(v.pos<0||v.pos>=double(s.numFrames())||v.env<.00001f){deactivate(v);continue;}
    int next=std::min(idx+1,int(s.numFrames())-1);float fract=float(v.pos-idx);float l=s.channels[0][idx]+fract*(s.channels[0][next]-s.channels[0][idx]);int rc=s.numChannels()>1?1:0;float r=s.channels[rc][idx]+fract*(s.channels[rc][next]-s.channels[rc][idx]);
    float pan=get(kPan0+v.role*9)*.01f;v.levelGain+=gainSmooth*(levelTarget[v.articulation?13+v.articulation:v.layer?8+v.role*3+v.layer-1:v.role]-v.levelGain);float gain=v.gain*v.levelGain*v.env*v.polarity*(v.preview?1.f:mixGain[(v.articulation==1||v.articulation==3)?2:ruleGroup(v.role)])*(v.role==0?1.f:1-duckS*ducker*.6f);int route=!v.preview&&getBool(kSeparate)?getInt(kRoute0+v.role*9):0;
    float filtered[2]={l,r};for(int ch=0;ch<2;++ch){if(v.hpAlpha>0){v.hp[ch]+=v.hpAlpha*(filtered[ch]-v.hp[ch]);filtered[ch]-=v.hp[ch];}if(v.lpAlpha<1){v.lp[ch]+=v.lpAlpha*(filtered[ch]-v.lp[ch]);filtered[ch]=v.lp[ch];}}l=filtered[0];r=filtered[1];
    bus[route][0]+=l*gain*(pan>0?1-pan:1);bus[route][1]+=r*gain*(pan<0?1+pan:1);
    v.pos+=v.step;v.env*=v.decay*v.choke;if(getBool(kGated)&&--v.gate<=0)v.choke=.99f;
   }
   modPhase+=phaseInc;if(modPhase>=1)modPhase-=1;float modulation=1-modDepth*.15f*(1+std::sin(float(modPhase*6.283185307)));
   master+=.002f*(masterTarget-master);ducker*=.9992f;
   for(int out=0;out<8;++out){float dry[2];for(int ch=0;ch<2;++ch){float x=bus[out][ch],shaped=x*(1+driveS*4);x=shaped/(1+std::abs(shaped)*driveS);low[out][ch]+=(.75f-.65f*dustS)*(x-low[out][ch]);dry[ch]=x*(1-dustS)+low[out][ch]*dustS;}
    float wetL,wetR;roomFx[out].process(roomS>1.e-5f?dry[0]:0.f,roomS>1.e-5f?dry[1]:0.f,wetL,wetR);float wet[]={wetL,wetR};
    for(int ch=0;ch<2;++ch){float x=dry[ch]+wet[ch]*roomS*.65f;if(hpAlpha>0){high[out][ch]+=hpAlpha*(x-high[out][ch]);x-=high[out][ch];}if(std::abs(x)<1.e-20f)x=0;x*=master*modulation;if(out*2+ch<c.numOutputs)c.outputs[out*2+ch][frame]+=mpc::clamp(x,-.99f,.99f);}
   }
  }
  if(!run&&(performanceActive.load()>=0||performanceQueued.load()>=0||queuedRoll.load()>=0||capture.state.load()==1))previewPpq=end;
  if(run){previewPpq=end;lastEnd=end;if(hostRunning)lastRawEnd=rawStart+step*c.numFrames;else lastRawEnd=-1e12;}else {lastEnd=-1e12;lastRawEnd=-1e12;transportOffset=0;}
  if(recStart>=0&&end>=recEnd)finishRecord();
  syncPerformance();syncProxies();syncLayerUi();
  int state=recordState.load(),count=countNotes(getInt(kFillSlot)),bar=int(position.load()/4);if(state!=lastUiState||count!=lastUiCount||bar!=lastUiBar){lastUiState=state;lastUiCount=count;lastUiBar=bar;displayPending.store(true);}
 }
 // Explicit little-endian extra state. Atomics avoid races with host save queries.
 void saveExtraState(std::vector<uint8_t>& out)override{
  auto put=[&](uint32_t n){for(int k=0;k<4;++k)out.push_back(uint8_t(n>>(8*k)));};
  put(0x47443102);for(int b=0;b<7;++b){put(lengths[b].load());for(auto& n:notes[b]){put(n.data.load());put(n.gate.load());}}
  for(int k=0;k<8;++k)put(kitValid[k].load()?1:0);for(auto& x:savedKits){float f=x.load();uint32_t w;std::memcpy(&w,&f,4);put(w);}
  for(const auto& id:bank.ids)for(int k=0;k<64;++k)out.push_back(uint8_t(id[k]));put(randomState.load());put(keep.load()?1:0);
  // Append stable content identities. The v1 QA state remains loadable unchanged.
  put(0x47444c32);auto identity=[&](int id,int orphan=0){const char* text=library.identity(id);if(!text)text=unresolvedText(orphan);for(int c=0;c<64;++c)out.push_back(text?uint8_t(text[c]):0);};
  for(int r=0;r<8;++r)identity(getInt(kXSample0+r),orphanSample[r].load());for(int j=0;j<64;++j)identity(savedLibraryKits[j].load(),orphanSaved[j].load());
  for(int key:{kXBeat,kXGroove}){int kind=key==kXBeat?3:4,index=getInt(key);identity(index>0&&index<=int(library.categories[kind].size())?library.categories[kind][index-1]+1:0,orphanMidi[key==kXBeat?0:1].load());}
  put(0x47444234);for(int b=0;b<7;++b){put(baseValid[b].load()?1:0);put(baseLengths[b].load());for(auto&n:bases[b]){put(n.data.load());put(n.gate.load());}}put(grooveBaseValid.load()?1:0);for(auto&x:grooveBase){float v=x.load();uint32_t bits;std::memcpy(&bits,&v,4);put(bits);}
  put(0x47525035);for(int g=0;g<4;++g){put(partSeed[g].load());for(auto&x:partOffset[g]){float v=x.load();uint32_t bits;std::memcpy(&bits,&v,4);put(bits);}}
  const char magic[]="GDXP0001";out.insert(out.end(),magic,magic+8);auto text=[&](const std::string&s,int n){for(int j=0;j<n;++j)out.push_back(j<int(s.size())?uint8_t(s[j]):0);};int kit=getInt(kXKit);text(kit>0&&kit<=int(library.factoryKits.size())?library.factoryKits[kit-1].id:orphanKit.load()?unresolvedKit:std::string(),128);auto*pack=filteredPack();text(pack?pack->id:orphanPack.load()?unresolvedPack:std::string(),64);
  saveLayerState(out);saveCymbalState(out);savePoolState(out);saveReverseState(out);savePerformanceState(out);
 }
 void loadExtraState(const uint8_t* data,size_t size)override{
  migratePerformanceDefaultNotes();bool hadPerformance=loadPerformanceState(data,size);loadReverseState(data,size);loadPoolState(data,size);
  const uint8_t*cymbalState=nullptr;if(size>=CymbalTrailerSize&&!std::memcmp(data+size-CymbalTrailerSize,"GDCY0001",8)){cymbalState=data+size-CymbalTrailerSize;size-=CymbalTrailerSize;}loadCymbalState(cymbalState);
  const uint8_t*layerState=nullptr;if(size>=LayerTrailerSize&&!std::memcmp(data+size-LayerTrailerSize,"GDLY0001",8)){layerState=data+size-LayerTrailerSize;size-=LayerTrailerSize;}
  loadLayerState(layerState);
  if(!restoringHasX)migrateLegacy();for(auto&x:orphanSample)x.store(0);for(auto&x:orphanSaved)x.store(0);for(auto&x:orphanMidi)x.store(0);orphanKit.store(false);orphanPack.store(false);
  const uint8_t*packState=nullptr;if(size>=200&&!std::memcmp(data+size-200,"GDXP0001",8)){packState=data+size-192;size-=200;}
  const size_t expected=4+7*(4+MaxNotes*8)+8*4+savedKits.size()*4+4096+8;constexpr size_t ext=4+7*(8+MaxNotes*8)+4+16;bool hasRules=size==expected+4+74*64+ext+84;bool hasBase=size==expected+4+74*64+ext||hasRules;if(size!=expected&&size!=expected+4+74*64&&!hasBase)return;for(int b=0;b<7;++b){baseValid[b].store(b>=4);baseLengths[b].store(b>=4?factoryLengths[b-4]:0);for(int i=0;i<MaxNotes;++i){bases[b][i].data.store(b>=4?factoryBases[b-4][i*2]:0);bases[b][i].gate.store(b>=4?factoryBases[b-4][i*2+1]:80);}}grooveBaseValid.store(false);for(int g=0;g<4;++g){partSeed[g].store(0x17432u+g*0x11333u);for(auto&x:partOffset[g])x.store(0);}size_t pos=0;
  auto read=[&](){uint32_t n=0;for(int k=0;k<4;++k)n|=uint32_t(data[pos++])<<(8*k);return n;};if(read()!=0x47443102)return;
  for(int b=0;b<7;++b){lengths[b].store(mpc::clamp(int(read()),PPQ,16*PPQ));for(auto&n:notes[b]){uint32_t d=read(),g=read();n.gate.store(mpc::clamp(g,5u,500u));n.data.store(d);}}
  for(int k=0;k<8;++k)kitValid[k].store(read()!=0);for(auto& x:savedKits){uint32_t w=read();float f;std::memcpy(&f,&w,4);x.store(std::isfinite(f)?f:0);}
  // Resolve saved identities by role, not by current directory order. Missing assets never silently substitute.
  auto remap=[&](int role,int old){old=mpc::clamp(old,0,7);const uint8_t* id=data+pos+(role*8+old)*64;bool empty=true;for(int k=0;k<64;++k)if(id[k])empty=false;if(empty)return old;for(int v=0;v<8;++v)if(std::memcmp(id,bank.ids[role*8+v].data(),64)==0)return v;return -1;};
  const uint8_t*legacyIdBlock=data+pos;std::array<int,64>savedLegacyVariant{};for(int j=0;j<64;++j)savedLegacyVariant[j]=mpc::clamp(int(savedKits[(j/8)*72+(j%8)*8].load()),0,7);std::array<int,8>legacyFallback{};for(int role=0;role<8;++role)legacyFallback[role]=library.find(reinterpret_cast<const char*>(data+pos+(role*8+getInt(kSample0+role*9))*64));internal=true;for(int role=0;role<8;++role){int mapped=remap(role,getInt(kSample0+role*9));missingSample[role]=mapped<0;if(mapped>=0&&mapped!=getInt(kSample0+role*9))set(kSample0+role*9,mapped);}
  for(int k=0;k<8;++k)for(int role=0;role<8;++role){int i=k*72+role*8;int mapped=remap(role,int(savedKits[i].load()));savedKits[i].store(float(mapped));}internal=false;pos+=4096;randomState.store(read());keep.store(read()!=0);
  if(pos<size&&read()==0x47444c32){int identitySlot=0;auto identity=[&](){int slot=identitySlot++;std::memcpy(unresolved[slot].data(),data+pos,64);unresolved[slot][64]=0;bool empty=true;for(int k=0;k<64;++k)if(data[pos+k])empty=false;int id=empty?0:library.find(reinterpret_cast<const char*>(data+pos));pos+=64;return empty?0:id? id:-1;};internal=true;
   for(int r=0;r<8;++r){const char* saved=reinterpret_cast<const char*>(data+pos);int current=getInt(kXSample0+r);const char* currentSha=library.identity(current);bool same=currentSha&&std::memcmp(currentSha,saved,64)==0;int id=identity();if(id==0&&current>int(library.entries.size())){missingSample[r]=true;continue;}if(id==0&&missingSample[r]){if(legacyFallback[r]>0)id=legacyFallback[r];else{id=-1;std::memcpy(unresolved[r].data(),legacyIdBlock+(r*8+getInt(kSample0+r*9))*64,64);unresolved[r][64]=0;}}if(!same&&current!=std::max(0,id))set(kXSample0+r,std::max(0,id));if(id>0)missingSample[r]=false;else if(id<0){missingSample[r]=true;orphanSample[r].store(r+1);}}
   for(int j=0;j<64;++j){int id=identity();if(id==0&&savedKits[(j/8)*72+(j%8)*8].load()<0){const char*oldId=reinterpret_cast<const char*>(legacyIdBlock+((j%8)*8+savedLegacyVariant[j])*64);id=library.find(oldId);if(id==0){id=-1;std::memcpy(unresolved[8+j].data(),oldId,64);unresolved[8+j][64]=0;}}savedLibraryKits[j].store(id);if(id<0)orphanSaved[j].store(9+j);}
   for(int key:{kXBeat,kXGroove}){int id=identity(),kind=key==kXBeat?3:4,value=0;if(id<0)orphanMidi[key==kXBeat?0:1].store(key==kXBeat?73:74);if(id==0&&getInt(key)>int(library.categories[kind].size()))continue;for(size_t i=0;i<library.categories[kind].size();++i)if(library.categories[kind][i]+1==id)value=int(i)+1;if(getInt(key)!=value)set(key,value);}internal=false;
  }else{internal=true;for(int r=0;r<8;++r)set(kXSample0+r,0);for(auto& id:savedLibraryKits)id.store(0);set(kXBeat,0);set(kXGroove,0);internal=false;}
  if(hasBase&&read()==0x47444234){for(int b=0;b<7;++b){bool valid=read()!=0;int length=int(read());baseLengths[b].store(mpc::clamp(length,0,16*PPQ));for(auto&n:bases[b]){n.data.store(read());n.gate.store(mpc::clamp(read(),5u,500u));}baseValid[b].store(valid&&length>=PPQ&&length<=16*PPQ);}bool valid=read()!=0;for(int n=0;n<4;++n){uint32_t bits=read();float v;std::memcpy(&v,&bits,4);if(!std::isfinite(v)){valid=false;v=param(kSwing+n).def;}grooveBase[n].store(v);}grooveBaseValid.store(valid);}
  if(hasRules&&read()==0x47525035){for(int g=0;g<4;++g){partSeed[g].store(read());for(auto&x:partOffset[g]){uint32_t bits=read();float v;std::memcpy(&v,&bits,4);x.store(std::isfinite(v)?mpc::clamp(v,-10.f,10.f):0.f);}}}
  internal=true;if(packState){size_t n=0;while(n<128&&packState[n])++n;std::string id(reinterpret_cast<const char*>(packState),n);int kit=0;for(size_t j=0;j<library.factoryKits.size();++j)if(library.factoryKits[j].id==id)kit=int(j)+1;if(!id.empty()&&kit==0){unresolvedKit=id;orphanKit.store(true);}if(!id.empty()&&getInt(kXKit)!=kit)set(kXKit,kit);n=0;while(n<64&&packState[128+n])++n;id.assign(reinterpret_cast<const char*>(packState+128),n);int pack=0;for(size_t j=0;j<library.packs.size();++j)if(library.packs[j].id==id)pack=int(j)+1;if(!id.empty()&&pack==0){unresolvedPack=id;orphanPack.store(true);}if(!id.empty()&&getInt(kPackFilter)!=pack)set(kPackFilter,pack);}else if(getInt(kXKit)>0){int kit=0;for(size_t j=0;j<library.factoryKits.size();++j){bool same=true;for(int r=0;r<8;++r)same&=library.factoryKits[j].sounds[r]==getInt(kXSample0+r);if(same){kit=int(j)+1;break;}}set(kXKit,kit);}internal=false;
  if(!hadPerformance)for(int b=0;b<4;++b)if(countNotes(b)>0)markRecordedFill(b);
 }
 // Commands are gestures, never saved musical state. Serialize idle values without
 // touching the live gates: resetting a gate here would execute a second command.
 std::vector<uint8_t> commandState;
 const std::vector<uint8_t>& internalSaveState(){
  commandState=mpc::Plugin::internalSaveState();size_t pos=12;
  for(int i=0;i<kParamCount;++i){size_t n=commandState[pos++];pos+=n;if(i==kUnlockDice||i==kRollFill||i==kRollFx||performanceTransient(i)||i==kPerformLoopState||(i>=kPerformState0&&i<=kPerformState7))std::fill(commandState.begin()+pos,commandState.begin()+pos+8,0);pos+=8;}
  return commandState;
 }
 bool internalLoadState(const uint8_t* data,size_t size){if(!data||size<12||std::memcmp(data,"MPCP",4))return false;restoringHasX=false;{size_t pos=12;uint32_t count=0,version=0;for(int k=0;k<4;++k){count|=uint32_t(data[8+k])<<(8*k);version|=uint32_t(data[4+k])<<(8*k);}for(uint32_t j=0;j<count&&pos<size;++j){size_t n=data[pos++];if(n>size-pos||size-pos-n<(version>=2?8u:4u))break;if(n==4&&!std::memcmp(data+pos,"XKit",4))restoringHasX=true;pos+=n+(version>=2?8:4);}}restoring.store(true);loadGuardFrames.store(int(sr*.5));for(int i=kFactoryKit;i<kParamCount;++i)mpc::Plugin::set(i,param(i).def);factoryDirty.store(false);bool ok=mpc::Plugin::internalLoadState(data,size);set(kUnlockDice,0);set(kRollFill,0);set(kRollFx,0);if(ok){bool hasFullRhythm=false,hasRhythm=false,hasStyles=false,hasCymbals=false,hasRulePick=false;size_t pos=12;uint32_t n=0,version=0;for(int k=0;k<4;++k){n|=uint32_t(data[8+k])<<(8*k);version|=uint32_t(data[4+k])<<(8*k);}for(uint32_t j=0;j<n&&pos<size;++j){size_t len=data[pos++];if(len>size-pos||size-pos-len<(version>=2?8u:4u))break;if(len==12&&!std::memcmp(data+pos,"RulePickKick",12))hasRulePick=true;if(len==11&&!std::memcmp(data+pos,"OpenHatRule",11))hasCymbals=true;if(len==13&&!std::memcmp(data+pos,"HatRhythmFull",13))hasFullRhythm=true;if(len==9&&!std::memcmp(data+pos,"HatRhythm",9))hasRhythm=true;if(len==13&&!std::memcmp(data+pos,"RuleKickStyle",13))hasStyles=true;pos+=len+(version>=2?8:4);}if(!hasStyles){bool old=internal;internal=true;for(int g=0;g<4;++g)set(kRuleKickStyle+g,getInt(kRuleKick+g));internal=old;}if(!hasCymbals&&getInt(kRuleHatStyle)==7)set(kOpenHatRule,2);if(!hasRhythm){static const int legacy[]={0,3,6,8};set(kHatRhythm,legacy[getInt(kHatGrid)]);set(kHatRhythmPage,getInt(kHatRhythm)/8);}if(!hasFullRhythm){set(kHatRhythmFull,getInt(kHatRhythm));int old=getInt(kHatPhraseState),full=0;for(int b=0;b<3;++b)full|=((old>>(4*b))&15)<<(5*b);set(kHatPhraseFullState,full);}syncHatSelection();for(int g=0;g<4;++g)if(n<=uint32_t(kRulePickFullKick+g))layerSet(kRulePickFullKick+g,std::max(0,getInt(kRuleKickStyle+g)-1));for(int g=0;g<4;++g){if(!hasRulePick&&getInt(kRuleKickStyle+g)==0)layerSet(kDiceLock4+g,1);if(!hasRulePick){syncRuleSelector(g,true);if(!getBool(kEditor)&&!getBool(kPoolOpen)&&!getBool(kSampleBrowserOpen))layerSet(kQ5+g,getNormalised(kRulePickKick+g));}ruleSelectorSeen[g].store(getInt(kRuleKickStyle+g));}}factoryDirty.store(false);soundDirty.store(0);liveDirty.store(0);
  if(ok){lastKit=getInt(kKit);lastSoundGroup=getInt(kSoundBrowserGroup);lastLibraryKind=getInt(kLibraryKind);lastSlot=getInt(kFillSlot);lastLength=getInt(kFillLength);lastSelected=getInt(kNoteSelect);lastPackFilter=getInt(kPackFilter);for(int q=0;q<5;++q)catalogSeen[q].store(getInt(q==0?kXKit:kXSoundKick+q-1));for(int n=0;n<16;++n){int target=proxyTarget(n);proxySeenTarget[n]=target;proxySeenValue[n]=target>=0?getNormalised(target):0;int sound=soundTarget(n);soundSeenTarget[n].store(sound);soundSeenValue[n].store(sound>=0?getNormalised(sound):0);}}
  restoring.store(false);return ok;}
 // Native PRESET changes kit audio only, preserving pad notes, output routing, groove/fills and transport.
 void selectFactoryProgram(int index){if(index<0||index>=numPrograms())return;std::array<float,kParamCount> keep{};for(int i=0;i<kParamCount;++i)keep[i]=getNormalised(i);internal=true;mpc::Plugin::setProgram(index);for(int i=0;i<kParamCount;++i)if(!kitAudioParam(i))setNormalised(i,keep[i]);for(int i=kXKit;i<=kXLibraryFolder;++i)mirrorLegacy(i);for(auto&n:catalogSeen)n.store(-1);internal=false;factoryDirty.store(index>0);queueAudioUi();}
 int activeFactoryProgram()const{return mpc::clamp(getInt(kXKit),0,numPrograms()-1);}
 void markRenderThread(){auto token=uintptr_t(pthread_self());for(auto& t:renderThreads)if(t.load()==token)return;for(auto& t:renderThreads){uintptr_t empty=0;if(t.compare_exchange_strong(empty,token))return;}renderOverflow.store(true);}
 void flushFromValueRead(){if(renderOverflow.load())return;auto token=uintptr_t(pthread_self());for(auto&t:renderThreads)if(t.load()==token)return;flushHostUpdates();}
 void requestHostIdle(){if(host_&&effect_)host_(effect_,audioMasterNeedIdle_Deprecated,0,0,nullptr,0);}
 void flushHostUpdates(){if(flushing.test_and_set())return;for(int i=0;i<kParamCount;++i)if(changed[i].exchange(false)){
 if(legacyRuleAlias(i)){ruleNotifyThread.store(uintptr_t(pthread_self()));ruleNotifyIndex.store(i);}
 notifyHostParameterChanged(i);if(legacyRuleAlias(i)){ruleNotifyIndex.store(-1);ruleNotifyThread.store(0);}}if(displayPending.exchange(false))requestDisplayUpdate();flushing.clear();}
 int countNotes(int b)const{int n=0;for(auto& v:notes[mpc::clamp(b,0,6)])if(v.valid())++n;return n;}
 int proxyTarget(int n)const{if(getBool(kPerformOpen))return n<4?performanceTarget(n):-1;if(getBool(kPoolOpen)||getBool(kSampleBrowserOpen))return -1;static const int edit[]={kFillSlot,kFillLength,kRecMode,kCountIn,kNoteNudge,kNoteVelocity,kNoteGate,kNoteChance};return getBool(kEditor)?(n<8?edit[n]:-1):frontDiceTarget(n);}
private:
 std::vector<int>allKits;std::array<std::vector<int>,4>allSounds;int lastPackFilter=-1;std::array<std::atomic<int>,5>catalogSeen{{-1,-1,-1,-1,-1}};
 const groove::Library::Pack* filteredPack()const{int p=getInt(kPackFilter);return p>0&&p<=int(library.packs.size())?&library.packs[p-1]:nullptr;}
 const std::vector<int>&kitItems()const{auto*p=filteredPack();return p?p->kits:allKits;}
 const std::vector<int>&soundItems(int group)const{auto*p=filteredPack();return p?p->sounds[group]:allSounds[group];}
 int kitPosition(int index)const{const auto&list=kitItems();for(size_t j=0;j<list.size();++j)if(list[j]+1==index)return int(j)+1;return 0;}
 std::string kitCaption(int index)const{return index>0&&index<=int(displayNames.kits.size())?shortCaption(displayNames.kits[index-1],31):"Custom Kit";}
 std::string sampleCaption(int id,size_t limit=30)const{return id>0&&id<=int(library.entries.size())?shortCaption(id<=int(displayNames.entries.size())?displayNames.entries[id-1]:groove::DisplayNames::stem(library.entries[id-1].name),limit):"Core Sounds";}
 std::string legacyCaption(int role)const{return displayNames.legacy[role][getInt(kSample0+role*9)];}
 std::string soundSetCaption(int index)const{return "Perc Ensemble "+groove::DisplayNames::number(index+1,3);}

 // VST2 host display is capped at31 UTF8 bytes. Keep identifying suffixes and never split a code point.
 static std::string shortCaption(std::string name,size_t limit){
  size_t slash=name.find_last_of('/');if(slash!=std::string::npos)name=name.substr(slash+1);
  if(name.size()>4&&(name.substr(name.size()-4)==".wav"||name.substr(name.size()-4)==".mid"))name.resize(name.size()-4);
  if(name.size()<=limit)return name;
  size_t tail=name.size()-std::min<size_t>(8,limit/2);while(tail<name.size()&&(static_cast<unsigned char>(name[tail])&192)==128)++tail;
  size_t head=limit-1-(name.size()-tail);while(head>0&&(static_cast<unsigned char>(name[head])&192)==128)--head;
  return name.substr(0,head)+"~"+name.substr(tail);
 }
 const std::vector<int>& folderItems(int kind)const{auto*p=filteredPack();return p?p->folders[kind]:library.folderKinds[kind];}
 const std::vector<int>& browserItems()const{int kind=getInt(kLibraryKind),folder=getInt(kXLibraryFolder);if(kind<3&&folder>0&&folder<=int(library.folders.size())&&library.folders[folder-1].kind==kind)return library.folders[folder-1].items;auto*p=filteredPack();return p?p->categories[kind]:library.categories[kind];}
 int browserCount()const{int kind=getInt(kLibraryKind);return kind<3&&getInt(kXLibraryFolder)==0?int(folderItems(kind).size()):int(browserItems().size());}
 std::array<std::atomic<bool>,kParamCount> changed{};std::array<std::atomic<uintptr_t>,8> renderThreads{};std::atomic<bool>renderOverflow{false},displayPending{false},restoring{false};std::atomic_flag flushing=ATOMIC_FLAG_INIT;int lastUiState=-1,lastUiCount=-1,lastUiBar=-1;
 std::array<int,16>proxySeenTarget{};std::array<float,16>proxySeenValue{};
 std::array<float,kParamCount> last{};std::atomic<unsigned> noteDirty{0},liveDirty{0};std::array<std::atomic<int>,4>lastRuleStyle{{1,1,2,2}};std::array<float,kParamCount> undoValues{};std::array<uint32_t,7*MaxNotes*2+7> undoNotes{};bool haveUndo=false;
 std::array<groove::RoomReverb,8> roomFx;float low[8][2]{},high[8][2]{},master=.7f,ducker=0,driveS=.1f,dustS=.1f,roomS=.08f,duckS=.15f;double sr=44100,previewPpq=0,lastEnd=-1e12,lastRawEnd=-1e12,transportOffset=0,modPhase=0;bool wasRunning=false;std::atomic<bool> keep{false};
 double fillStart=-1,fillEnd=-1,recStart=-1,recEnd=-1;int lastLibraryKind=-1,activeFill=0,recSlot=0,recLength=4*PPQ,recMode=0,lastKit=-1,lastSlot=-1,lastSelected=-1,lastLength=-1,lastAutoBar=-1;bool recCleared=false;std::atomic<uint32_t> randomState{0x784571};
 struct Event{int offset,role,velocity;bool input;int gate;};std::array<Event,512>events{};int eventCount=0;
 static int lengthForChoice(int n){static const int l[]={1,2,4,8,16};return l[mpc::clamp(n,0,4)]*PPQ;}
 static int choiceForLength(int n){for(int i=0;i<5;++i)if(lengthForChoice(i)==n)return i;return 2;}
 uint32_t random(){uint32_t x=randomState.load();x^=x<<13;x^=x>>17;x^=x<<5;randomState.store(x);return x;}
 static uint32_t hash(uint32_t n){n^=n>>16;n*=0x7feb352du;n^=n>>15;n*=0x846ca68bu;n^=n>>16;return n;}
 void seedPattern(int b){int i=0;for(int s=0;s<std::min(64,lengths[b].load()/240);++s){if(s%4==0)notes[b][i++].write({s*240,0,105,100,80});if(s==4||s==12)notes[b][i++].write({s*240,1,100,100,80});if(s%2==0)notes[b][i++].write({s*240,2,b>=4?groove::hatAccent(s*240):75+(s%4?0:12),100,50});}}
 void clearBank(int b){baseValid[b].store(false);for(auto&n:notes[b])n.data.store(0);}
 // The event is inside [start,end); nearest-sample rounding may produce
 // exactly numFrames. Keep it in this block instead of silently dropping it.
 static int eventOffset(double beat,double start,double end,double step){int frames=std::max(1,int(std::llround((end-start)/step)));return mpc::clamp(int(std::llround((beat-start)/step)),0,frames-1);}
 void addEvent(int offset,int role,int vel,bool input=false,int gate=80){if(eventCount<int(events.size()))events[eventCount++]={offset,role,mpc::clamp(vel,1,127),input,gate};}
 int mapNote(int note){for(int r=0;r<8;++r)if(note==getInt(kPadNote0+r*9))return r;int gm=note==36?0:note==38||note==40?1:note==42||note==44?2:note==46?3:note==39?4:note==45?5:note==49?6:note==37?7:-1;return gm;}
 void scheduleBank(int b,double start,double end,double step,bool fill){
  if(!fill&&orphanMidi[0].load()>0&&getInt(kXBeat)==0)return;
  double length=fill?double(lengths[b].load())/PPQ:4.;int base=int(std::floor((start-(fill?fillStart:0))/length));
  int beat=getInt(kXBeat);const groove::Library::Midi* imported=nullptr;if(!fill&&beat>0&&beat<=int(library.categories[3].size()))imported=&library.midi[library.categories[3][beat-1]];
  if(imported){length=4.;base=int(std::floor(start/length));}
  bool phrase=!fill&&!imported&&phraseBank(b);int grid=phrase?phraseGrid(b):4;double origin=0;if(phrase)base=int(std::floor((start-origin)/length));
  int count=imported?int(imported->notes.size()):MaxNotes;
  for(int cycle=base-1;cycle<=base+1;++cycle)for(int i=0;i<count;++i){Note n;if(imported){auto e=imported->notes[i];n={e.tick,mpc::clamp(e.note-60,0,7),e.velocity,100,e.gate};}else{if(!notes[b][i].valid())continue;n=notes[b][i].read();}if(!fill&&n.tick>=4*PPQ)continue;if(!fill&&((getInt(kRideRule)>0&&(n.role==2||n.role==3))||(n.role==2&&getInt(kOpenHatRule)>0&&getInt(kRideRule)==0&&n.tick%(4*PPQ)==openHatTick())||(n.role==3&&phrase)||(n.role==6&&phrase)))continue;int group=ruleGroup(n.role),rule=fill?0:getInt(kRuleKick+group);int hashTick=phrase&&rule?n.tick%(4*PPQ):n.tick;uint32_t h=hash(uint32_t(hashTick+n.role*20011+((!fill||keep||rule==0)?0:cycle*131)+partSeed[group].load()));float u=float(h&65535)/65535;
   float density=(fill?get(kFillDensity):get(kDensity))*.01f;if(density<=0)continue;bool fixed=anchor(n)&&rule>0&&density>0&&n.chance>0;if(!fixed&&(u>density||float((h>>16)%100)>=n.chance))continue;
   double beat=(fill?fillStart:origin)+cycle*length+double(n.tick)/PPQ;
   int groove=getInt(kXGroove);if(groove>0&&groove<=int(library.categories[4].size())){const auto& m=library.midi[library.categories[4][groove-1]];int local=n.tick%m.length,nearest=0,distance=std::numeric_limits<int>::max();for(const auto& e:m.notes){int grid=int(std::round(e.tick/240.))*240,d=std::abs(local-grid);if(d<distance){distance=d;nearest=e.tick-grid;}}beat+=double(nearest)/PPQ;}
   int swingTick=(phrase&&(n.role==2||n.role==3)&&grid==1)?480:240;
   double swing=(mpc::clamp(get(kSwing)+(rule?partOffset[group][0].load():0.f),50.f,75.f)-50)*(swingTick==480?.01:.005);
   if(phrase&&(n.role==2||n.role==3)&&phraseRhythm(b)>=9){}else if(phrase&&(n.role==2||n.role==3)&&grid<4){int r=n.tick%(2*swingTick);beat+=swing*double(std::min(r,2*swingTick-r))/swingTick;}else if((n.tick/swingTick)%2)beat+=swing;
   if(!phrase&&!keep&&rule&&!fixed)beat+=(int(hash(h+91)%201)-100)*std::min(get(kVariation),rule==1?5.f:rule==2?20.f:40.f)*.000001/60*tempoForStep(step);
   if(phrase&&rule&&!fixed){int phraseBar=(n.tick/(4*PPQ))%2;int shared=int(hash(partSeed[group].load()+phraseBar*311+79)%2001)-1000;beat+=shared*.001*get(kVariation)*.00002/60*tempoForStep(step);}
   if(n.role==1)beat+=(get(kSnareLate)+(rule?partOffset[group][1].load():0.f))*.001/60*tempoForStep(step);
   if(n.role==2||n.role==3)beat+=(get(kHatShift)+(rule?partOffset[group][2].load():0.f))*.001/60*tempoForStep(step);
   uint32_t jitter=hash(h+((!fill||keep||rule==0)?0:uint32_t(cycle)));float jitterAmount=rule&&!fixed?mpc::clamp(get(kRndNudge)+partOffset[group][3].load(),0.f,rule==1?2.f:rule==2?4.f:6.f):0.f;beat+=(int(jitter%2001)-1000)*jitterAmount*.000001/60*tempoForStep(step);
   if(beat<0||beat<start-1e-10||beat>=end-1e-10)continue;
   if(fill){if(beat<fillStart||beat>=fillEnd)continue;}else if(fillStart>=0&&beat>=fillStart&&beat<fillEnd)continue;
   if(!fill&&!admitRole(n.role,beat,fixed,phrase,grid))continue;int vel=groove::velocityVariation(n.velocity,get(kVelSpread),rule,h);int off=eventOffset(beat,start,end,step);addEvent(off,n.role,vel,false,n.gate);
  }
  int snareRule=getInt(kRuleSnare);float ghost=std::min(get(kGhost),snareRule<=1?0.f:snareRule==2?25.f:40.f);if(fill==false&&!phrase&&ghost>0){for(int cycle=base-1;cycle<=base+1;++cycle){double beat=(fill?fillStart:0)+cycle*length+2.75;uint32_t h=hash(uint32_t(0)+partSeed[1].load()+991);if(h%100>=ghost)continue;if(beat>=0&&beat>=start&&beat<end&&(!fill||(beat>=fillStart&&beat<fillEnd))&&(fill||fillStart<0||beat<fillStart||beat>=fillEnd)){if(admitRole(1,beat,false))addEvent(eventOffset(beat,start,end,step),1,35);}}}
 }
 double tempoForStep(double step)const{return step*60*sr;}
 void queueFill(double start){if(recordState.load()==1||recordState.load()==2)return;activeFill=getInt(kFillSlot);double beats=double(lengths[activeFill].load())/PPQ;fillStart=performanceWindowStart(start+1e-7,beats);fillEnd=fillStart+beats;recordState.store(4);}
 void recordInput(double beat,int role,int vel){if(recStart<0||beat<recStart||beat>=recEnd||!recCleared)return;baseValid[recSlot].store(false);int tick=mpc::clamp(int(std::llround((beat-recStart)*PPQ)),0,recLength-1);for(auto& n:notes[recSlot])if(!n.valid()){n.write({tick,role,vel,100,80});recorded.fetch_add(1);return;}recordState.store(6);}
 void finishRecord(){markRecordedFill(recSlot);recStart=recEnd=-1;if(recordState.load()!=6)recordState.store(3);mirrorNote();}
 void deactivate(Voice& v){if(v.librarySlot>=0){library.release(v.librarySlot);v.librarySlot=-1;}v.active=false;}
 void trigger(int role,int vel,int gateMs=80){capturePerformance(role,vel,gateMs);if(role>=8&&role<=10)triggerCymbal(role==8?0:role==9?1:2,vel,gateMs);else triggerLayers(role,vel,gateMs);}

 int steal=0;
 void syncProxies(){syncRuleSelectors();for(int row=0;row<8;++row){int j=getInt(kXKitPage)*8+row;bool selected=j==0?getInt(kXKit)==0:j<=int(kitItems().size())&&kitItems()[j-1]+1==getInt(kXKit);if(get(kKitRow0+row)!=(selected?1:0)){bool prev=internal;internal=true;set(kKitRow0+row,selected?1:0);internal=prev;changed[kKitRow0+row].store(true);}}syncSoundSelectors();internal=true;for(int q=0;q<5;++q){int key=q==0?kXKit:kXSoundKick+q-1,id=getInt(key),old=catalogSeen[q].load();if(old!=id){const auto&list=q==0?kitItems():soundItems(q-1);int pos=0;for(size_t j=0;j<list.size();++j)if(list[j]+1==id)pos=int(j)+1;float expected=getNormalised(kXQKit+q),value=list.empty()?0.f:float(pos)/list.size();if(getInt(key)==id&&catalogSeen[q].compare_exchange_strong(old,id)&&values_[kXQKit+q]->compare_exchange_strong(expected,value))changed[kXQKit+q].store(true);}}for(int n=0;n<16;++n){int t=proxyTarget(n);float value=t>=0?getNormalised(t):0;if(proxySeenTarget[n]!=t||proxySeenValue[n]!=value){if(getNormalised(kQ1+n)!=value){setNormalised(kQ1+n,value);changed[kQ1+n].store(true);}proxySeenTarget[n]=t;proxySeenValue[n]=value;}}for(int cell=0;cell<64;++cell){bool hit=false;for(auto& note:notes[getInt(kFillSlot)])if(note.valid()){auto e=note.read();if(e.role==gridRole(getInt(kFillGridPage),cell/16)&&e.tick/240==cell%16+getInt(kGridBar)*16){hit=true;break;}}if(get(kGrid0+cell)!=(hit?1:0)){set(kGrid0+cell,hit?1:0);changed[kGrid0+cell].store(true);}}
  set(kStatus,0);set(kRecordedCount,0);set(kPosition,0);internal=false;}
 void mirrorNote(bool preserve=false){int slot=getInt(kFillSlot),n=getInt(kNoteSelect);if(!notes[slot][n].valid())return;auto e=notes[slot][n].read();internal=true;float nudge=float((e.tick-std::round(e.tick/240.)*240.)/PPQ*60000./currentTempo.load());if(!preserve||getInt(kNoteNudge)!=int(std::lround(nudge)))set(kNoteNudge,nudge);if(!preserve||getInt(kNoteVelocity)!=e.velocity)set(kNoteVelocity,e.velocity);if(!preserve||getInt(kNoteGate)!=e.gate)set(kNoteGate,e.gate);if(!preserve||getInt(kNoteChance)!=e.chance)set(kNoteChance,e.chance);internal=false;}
 void editNote(unsigned dirty){int slot=getInt(kFillSlot);markRecordedFill(slot);baseValid[slot].store(false);int i=getInt(kNoteSelect);if(!notes[slot][i].valid())return;auto n=notes[slot][i].read();layerSet(performanceLock(4+slot),1);if(dirty&1)n.tick=mpc::clamp(int(std::round(n.tick/240.))*240+int(std::round(get(kNoteNudge)*currentTempo.load()/60000.*PPQ)),0,lengths[slot].load()-1);if(dirty&2)n.velocity=getInt(kNoteVelocity);if(dirty&4)n.gate=getInt(kNoteGate);if(dirty&8)n.chance=getInt(kNoteChance);notes[slot][i].write(n);}
 bool kitAudioParam(int i)const{if(i>=kReverse0&&i<=kReverse16)return true;if(i>=kCym0Sample&&i<=kCym2Invert)return true;if(i>=kRoleOn0&&i<=kL5Invert)return true;if(canonical(i)!=i)return kitAudioParam(canonical(i));if(i>=kXQKit&&i<=kXQPerc||i>=kXSoundKick&&i<=kXSoundPerc||i==kXKit||i==kKit||i>=kDrive&&i<=kDuck||i>=kLowCut&&i<=kModDepth||i>=kXSample0&&i<=kXSample7)return true;for(int r=0;r<8;++r)if(i>=kLevel0+r*9&&i<=kChoke0+r*9)return true;return false;}
 void queueAudioUi(){for(int i=0;i<kParamCount;++i)if(isFrontDiceParameter(i)||i>=kRuleKickStyle&&i<=kRulePercStyle||kitAudioParam(i)||i>=kXSoundKick&&i<=kXSoundPerc||i>=kSwing&&i<=kDuck||i==kXBeat||i==kXGroove||i==kPhraseState||i==kHatPhraseState||i==kHatPhraseFullState)changed[i].store(true);displayPending.store(true);}
 void factoryValues(const groove::Library::FactoryKit& kit,std::vector<float>& values){for(int kind=0;kind<3;++kind){values[kCym0Level+kind*LayerFields]=kit.parts[6][0];values[kCym0Tune+kind*LayerFields]=kit.parts[6][1];values[kCym0Decay+kind*LayerFields]=std::max(800.f,kit.parts[6][2]);}for(int r=0;r<8;++r){values[kXSample0+r]=kit.sounds[r];int keys[]={kLevel0,kTune0,kDecay0,kPan0,kChoke0};for(int n=0;n<5;++n)values[keys[n]+r*9]=kit.parts[r][n];}int fx[]={kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth};for(int n=0;n<7;++n)values[fx[n]]=kit.fx[n];}
 bool applyFactory(int index,bool resetLayers=true){if(index<=0)return true;if(index>int(library.factoryKits.size())){diceResult.store(2);displayPending.store(true);return false;}auto&kit=library.factoryKits[index-1];if(resetLayers){for(int key=kReverse0;key<=kReverse16;++key)layerSet(key,0);for(int key=kCym0Sample;key<=kCym2Invert;++key)layerSet(key,param(key).def);for(auto&m:cymbalMissing)m.store(0);for(int kind=0;kind<3;++kind){layerSet(kCym0Level+kind*LayerFields,kit.parts[6][0]);layerSet(kCym0Tune+kind*LayerFields,kit.parts[6][1]);layerSet(kCym0Decay+kind*LayerFields,std::max(800.f,kit.parts[6][2]));}for(int key=kRoleOn0;key<=kL5Invert;++key)layerSet(key,param(key).def);for(auto&missing:layerMissing)missing.store(0);}for(int r=0;r<8;++r){if(!resetLayers&&getBool(kRoleLock0+r))continue;if(diceAllows(kXSample0+r)){set(kXSample0+r,kit.sounds[r]);missingSample[r]=false;}int keys[]={kLevel0,kTune0,kDecay0,kPan0,kChoke0};for(int n=0;n<5;++n)set(keys[n]+r*9,kit.parts[r][n]);}int fx[]={kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth};for(int n=0;n<7;++n)set(fx[n],kit.fx[n]);diceResult.store(0);queueAudioUi();return true;}
 void snapshotUndo(){snapshotPerformanceUndo();undoRandom=randomState.load();undoKeep=keep.load();for(int g=0;g<4;++g){undoPartSeed[g]=partSeed[g].load();for(int n=0;n<4;++n)undoOffset[g][n]=partOffset[g][n].load();}int k=0;for(int b=0;b<7;++b){undoNotes[k++]=lengths[b].load();for(auto&n:notes[b]){undoNotes[k++]=n.data.load();undoNotes[k++]=n.gate.load();}}for(int i=0;i<kParamCount;++i)undoValues[i]=getNormalised(i);haveUndo=true;}
 void restoreUndo(){restorePerformanceUndo();randomState.store(undoRandom);keep.store(undoKeep);for(int g=0;g<4;++g){partSeed[g].store(undoPartSeed[g]);for(int n=0;n<4;++n)partOffset[g][n].store(undoOffset[g][n]);}int k=0;for(int b=0;b<7;++b){lengths[b].store(undoNotes[k++]);for(auto&n:notes[b]){n.data.store(undoNotes[k++]);n.gate.store(undoNotes[k++]);}}internal=true;for(int i=0;i<kParamCount;++i)if(isFrontDiceParameter(i)||i>=kRuleKickStyle&&i<=kRulePercStyle||kitAudioParam(i)||i>=kXSoundKick&&i<=kXSoundPerc||i>=kSwing&&i<=kDuck||i==kXBeat||i==kXGroove||i==kPhraseState||i==kHatPhraseState||i==kHatPhraseFullState)setNormalised(i,undoValues[i]);for(int g=0;g<4;++g)set(kRuleKick+g,ruleIntensity(ruleStyle(g)));syncHatSelection();internal=false;for(int g=0;g<4;++g){lastRuleStyle[g].store(ruleStyle(g));ruleSelectorSeen[g].store(getInt(kRuleKickStyle+g));}liveDirty.store(0);lastKit=getInt(kKit);factoryDirty.store(false);mirrorNote();queueAudioUi();}
 void roll(){DiceWriteScope diceScope;float amt=get(kRandomAmt)*.01f;internal=true;
  diceRolls.fetch_add(1);diceResult.store(0);if(amt<=0){diceResult.store(1);internal=false;displayPending.store(true);return;}
  keep=false;
  if(!getBool(kKitLock)){
   const auto&kitList=getInt(kDicePool)==1?allKits:kitItems();int count=getInt(kDicePool)==2?poolCount():int(kitList.size());if(!count){diceResult.store(2);}else if(getInt(kKitDiceMode)==0){int old=getInt(kXKit),pick=int(random()%count),next=getInt(kDicePool)==2?poolAt(pick):kitList[pick]+1;if(count>1&&next==old)next=getInt(kDicePool)==2?poolAt((pick+1)%count):kitList[(pick+1)%count]+1;set(kXKit,next);applyFactory(next,false);}else{int required=int(random()%4);for(int g=0;g<4;++g)if(g==required||float(random()%100)<amt*100){if(getInt(kDicePool)==2){int count=poolCount();if(count)applyPoolPart(g,poolAt(random()%count));continue;}const auto&list=getInt(kDicePool)==1?allSounds[g]:soundItems(g);int count=int(list.size());if(!count)continue;int pick=int(random()%count);for(int tries=0;tries<count&&soundMatches(g,library.soundPools[g][list[pick]]);++tries)pick=(pick+1)%count;applySound(g,list[pick]+1,true);}set(kXKit,0);set(kChoke0+2*9,1);set(kChoke0+3*9,1);set(kChoke0,0);set(kChoke0+9,0);}
  }
  if(!getBool(kKitLock)){if(getInt(kDicePool)==0){rollLayers(0,false);rollLayers(1,false);}else rollPoolLayers();for(int kind=0;kind<3;++kind)if(!getBool(kCym0Lock+kind*LayerFields)&&diceAllows(kCym0Sample+kind*LayerFields)){layerSet(kCym0Sample+kind*LayerFields,0);cymbalMissing[kind].store(0);}}
  unsigned frontDirty=randomiseFrontControls(amt);
  bool rhythmLocked=true;for(int g=0;g<4;++g)if(getInt(kRuleKick+g)>0)rhythmLocked=false;
  // Global Dice always regenerates the main loop. Recorded fills and FX are owned by separate commands.
  if(!getBool(kPatternLock)&&!rhythmLocked){for(int g=0;g<4;++g)if(getInt(kRuleKick+g)>0)partSeed[g].store(random());refreshMain(15);}
  else if(frontDirty&&!getBool(kPatternLock))refreshMain(frontDirty);
  regeneratePerformance();internal=false;queueAudioUi();
 }
 bool soundMatches(int g,const groove::Library::SoundChoice&c)const{int count=g==3?4:g==2?2:1;for(int j=0;j<count;++j){int role=g==3?(j==0?7:j+3):g==2?2+j:g;if(c.sounds[j]!=getInt(kXSample0+role))return false;}return true;}
 void syncSoundSelectors(){internal=true;for(int g=0;g<4;++g){if(soundDirty.load()&(1u<<g))continue;float expected=getNormalised(kXSoundKick+g);int index=0,old=int(std::lround(param(kXSoundKick+g).toPlain(expected)));if(old>0&&old<=int(library.soundPools[g].size())&&soundMatches(g,library.soundPools[g][old-1]))continue;for(size_t j=0;j<library.soundPools[g].size();++j)if(soundMatches(g,library.soundPools[g][j])){index=int(j)+1;break;}if(old!=index&&values_[kXSoundKick+g]->compare_exchange_strong(expected,param(kXSoundKick+g).toNormalised(float(index))))changed[kXSoundKick+g].store(true);mirrorLegacy(kXSoundKick+g);}internal=false;}
 void applySound(int g,int index,bool respectLocks=false){if(index<=0||index>int(library.soundPools[g].size()))return;auto&c=library.soundPools[g][index-1];internal=true;set(kXKit,0);for(int j=0;j<(g==3?4:g==2?2:1);++j){int role=g==0?0:g==1?1:g==2?2+j:j==0?7:j+3;if(respectLocks&&getBool(kRoleLock0+role))continue;if(diceAllows(kXSample0+role)){set(kXSample0+role,c.sounds[j]);missingSample[role]=false;}int keys[]={kLevel0,kTune0,kDecay0,kPan0,kChoke0};for(int k=0;k<5;++k)set(keys[k]+role*9,c.parts[j][k]);}internal=false;queueAudioUi();}
 bool admitRole(int role,double beat,bool fixed,bool phrase=false,int grid=4){int rule=getInt(kRuleKick+ruleGroup(role));if(!rule)return true;int pocket[]={4,3,8,1,1,1,1,2},loose[]={5,4,10,1,2,2,1,3},busy[]={6,5,12,2,2,4,2,4};int cap=(rule==1?pocket:rule==2?loose:busy)[role],bar=int(std::floor(beat/4));if(phrase&&(role==2||role==3)&&grid<4){int caps[]={4,12,24,28};cap=role==2?caps[grid]:1;}auto&b=budgets[role];if(b.bar!=bar){b.bar=bar;b.count=b.extras=b.anchors=0;}
  int bit=fixed?(role==0?1:1<<mpc::clamp(int(std::round(std::fmod(beat+4,4.))),0,3)):0;if(bit&&(b.anchors&bit))return false;
  int reserved=role==0?1:role==1?2:0;if(!fixed&&b.extras>=std::max(0,cap-reserved))return false;
  double gap=role==2?(rule==1?.5:rule==2?.25:.125):role==0||role==1?(rule==1?.5:.25):role==7?(rule==1?.5:.25):.5;
  if(phrase&&(role==2||role==3)&&grid<4)gap=grid==0?.85:grid==1?.055:.055;if(role==1&&ruleStyle(1)==15)gap=.09;gap=std::max(.025,gap-.03);if(!fixed&&(beat-b.last<gap||((role==2||role==3)&&beat-lastHat<(phrase&&grid<4?.025:.1))))return false;
  ++b.count;if(fixed)b.anchors|=bit;else ++b.extras;b.last=beat;if(role==2||role==3)lastHat=beat;return true;
 }
 static int ruleGroup(int role){return role==0?0:role==1?1:role==2||role==3?2:3;}
 bool anchor(const Note&n)const{int tick=n.tick%(4*PPQ),grid=int(std::round(tick/double(PPQ)));return(n.role==0&&std::abs(tick)<100)||(n.role==1&&(grid==1||grid==3)&&std::abs(tick-grid*PPQ)<120);}
 bool roomFor(int b,const Note&candidate,int skip=-1)const{static const int caps[]={6,5,12,2,2,4,2,4},gap[]={240,240,120,480,480,480,480,240};int count=0;for(int i=0;i<MaxNotes;++i)if(i!=skip&&notes[b][i].valid()){auto n=notes[b][i].read();if(n.role==candidate.role&&n.tick/(4*PPQ)==candidate.tick/(4*PPQ))++count;bool hats=(candidate.role==2||candidate.role==3)&&(n.role==2||n.role==3);if((n.role==candidate.role||hats)&&std::min(std::abs(n.tick-candidate.tick),lengths[b].load()-std::abs(n.tick-candidate.tick))<(hats?120:gap[candidate.role]))return false;}int rule=getInt(kRuleKick+ruleGroup(candidate.role));int pocket[]={4,3,8,1,1,1,1,2},loose[]={5,4,10,1,2,2,1,3};return count<(rule==1?pocket[candidate.role]:rule==2?loose[candidate.role]:caps[candidate.role]);}
 // PhraseState packs three generated flags, three3-bit actual hat grids (4=legacy),
 // and three2-bit whole-bar origins. It is non-automatable and saved with stable parameter IDs.
 bool phraseBank(int b)const{return b>=4&&b<7&&(getInt(kPhraseState)&(1<<(b-4)));}
 int phraseRhythm(int b)const{int rhythm=((getInt(kHatPhraseFullState)>>(5*(b-4)))&31)-1;return rhythm>=0&&rhythm<24?rhythm:-1;}
 int phraseGrid(int b)const{int rhythm=phraseRhythm(b);if(rhythm>=0){if(rhythm%3)return 3;static const int grids[]={0,1,2,2,2,2,2,2};return grids[rhythm/3];}return std::min(4,(getInt(kPhraseState)>>(3+3*(b-4)))&7);}
 int phraseOrigin(int b)const{return(getInt(kPhraseState)>>(12+2*(b-4)))&3;}
 #include "OneBarLoop.inc"
 void mutate(int b,float amount){if(b<4&&amount>0)markRecordedFill(b);
  if(amount<=0||b<0||b>=4)return;
  bool allLocked=true;for(int g=0;g<4;++g)if(getInt(kRuleKick+g)>0)allLocked=false;if(allLocked)return;
  std::array<Note,MaxNotes>locked{};int lockedCount=0;for(auto&c:notes[b])if(c.valid()){auto n=c.read();if(getInt(kRuleKick+ruleGroup(n.role))==0)locked[lockedCount++]=n;}
  if(!baseValid[b].load()){if(countNotes(b)==0)seedPattern(b);baseLengths[b].store(lengths[b].load());for(int i=0;i<MaxNotes;++i){bases[b][i].data.store(notes[b][i].data.load());bases[b][i].gate.store(notes[b][i].gate.load());}baseValid[b].store(true);}
  lengths[b].store(baseLengths[b].load());for(auto&c:notes[b])c.data.store(0);
  int slot=0;for(int i=0;i<lockedCount;++i)notes[b][slot++].write(locked[i]);if(slot==MaxNotes)return;float roleShift[8];for(auto&v:roleShift)v=(int(random()%2001)-1000)*.001f*amount*3;
  // Anchors first: later candidates cannot displace the original downbeat/backbeat.
  for(int pass=0;pass<2;++pass)for(int i=0;i<MaxNotes;++i)if(bases[b][i].valid()){auto n=bases[b][i].read();int rule=getInt(kRuleKick+ruleGroup(n.role));if(rule==0)continue;bool fixed=anchor(n);if(fixed!=(pass==0))continue;
   if(!fixed){if(rule>=2&&amount>.5f&&random()%100<int((amount-.5f)*(rule==2?15:30)))continue;float factor=rule==1?.25f:rule==2?.6f:1.f;int delta=int(std::round((roleShift[n.role]+(int(random()%2001)-1000)*.001f*amount*2)*factor*currentTempo.load()/60000*PPQ));n.tick=mpc::clamp(n.tick+delta,0,lengths[b].load()-1);n.velocity=mpc::clamp(n.velocity+int((int(random()%2001)-1000)*.001f*amount*10*factor),n.role==1?20:35,127);}
   if(fixed||roomFor(b,n)){notes[b][slot++].write(n);if(slot==MaxNotes)return;}
  }
  if(amount<=.5f)return;int bars=(lengths[b].load()+4*PPQ-1)/(4*PPQ);for(int bar=0;bar<bars;++bar)for(int attempt=0;attempt<int((amount-.5f)*8);++attempt){int pick=random()%100,role=pick<55?2:pick<75?7:pick<88?0:1;int rule=getInt(kRuleKick+ruleGroup(role));if(rule<2||rule==2&&random()%2)continue;int step=role==2?2*int(random()%8):role==0?int((random()%3)*4+6)%16:3+4*int(random()%4);Note n{bar*4*PPQ+step*240,role,role==1?35+int(random()%15):55+int(random()%30),100,80};if(n.tick>=lengths[b].load()||!roomFor(b,n))continue;for(auto&c:notes[b])if(!c.valid()){c.write(n);break;}}
 }
 void saveKit(){int k=getInt(kUserKit);saveLayerKit(k);saveCymbalKit(k);saveReverseKit(k);for(int r=0;r<8;++r){savedLibraryKits[k*8+r].store(getInt(kXSample0+r));orphanSaved[k*8+r].store(orphanSample[r].load());}int i=k*72;for(int r=0;r<8;++r)for(int p:{kSample0,kLevel0,kTune0,kDecay0,kPan0,kChoke0,kRoute0,kPadNote0})savedKits[i++].store(get(p+r*9));for(int p:{kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth,kMaster})savedKits[i++].store(get(p));kitValid[k].store(true);}
 void loadKit(){int k=getInt(kUserKit);if(!kitValid[k].load())return;loadLayerKit(k);loadCymbalKit(k);loadReverseKit(k);int i=k*72;internal=true;set(kXKit,0);for(int r=0;r<8;++r){int id=savedLibraryKits[k*8+r].load();if(getInt(kXSample0+r)!=std::max(0,id))set(kXSample0+r,std::max(0,id));if(id<0)missingSample[r]=true;}for(int r=0;r<8;++r)for(int p:{kSample0,kLevel0,kTune0,kDecay0,kPan0,kChoke0,kRoute0,kPadNote0}){float v=savedKits[i++].load();if(p==kSample0){missingSample[r]=savedLibraryKits[k*8+r].load()<0||(savedLibraryKits[k*8+r].load()==0&&v<0);if(v>=0)set(p+r*9,v);}else set(p+r*9,v);}for(int p:{kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth,kMaster})set(p,savedKits[i++].load());for(int r=0;r<8;++r)if(savedLibraryKits[k*8+r].load()<=0&&orphanSaved[k*8+r].load()>0){orphanSample[r].store(orphanSaved[k*8+r].load());missingSample[r]=true;}internal=false;queueAudioUi();}
 void generateSound(int sample){auto& s=sounds[sample];s.sampleRate=44100;s.channels.resize(1);auto& x=s.channels[0];int role=sample/8;x.resize(role==3?22050:11025);uint32_t seed=123+sample;double phase=0;for(size_t i=0;i<x.size();++i){seed=1664525*seed+1013904223;float noise=float(int32_t(seed))/2147483648.f;double t=i/44100.;phase+=6.283185307*(role==0?45+90*std::exp(-t*60):180)/44100.;x[i]=float((role==0?std::sin(phase):role==1?.7*noise+.3*std::sin(phase):noise)*std::exp(-t*(role==3?12:role==0?20:role==1?35:90))*.6);}}
};
#ifndef GROOVE_DICE_TEST
static VstIntPtr grooveDispatch(AEffect*e,VstInt32 op,VstInt32 index,VstIntPtr value,void*ptr,float opt){auto*p=static_cast<GrooveDice*>(e->object);if(op==effOpen){auto result=mpc::detail::dispatcher(e,op,index,value,ptr,opt);p->requestHostIdle();return result;}if(op==effSetProgram){p->selectFactoryProgram(int(value));return 0;}if(op==effGetProgram)return p->activeFactoryProgram();if(op==effGetProgramName){mpc::copyString(static_cast<char*>(ptr),kVstMaxProgNameLen+1,p->programName(p->activeFactoryProgram()));return 0;}if(op==effGetChunk){if(!ptr)return 0;const auto&state=p->internalSaveState();*static_cast<void**>(ptr)=const_cast<uint8_t*>(state.data());return VstIntPtr(state.size());}if(op==effSetChunk)return p->internalLoadState(static_cast<const uint8_t*>(ptr),size_t(value))?1:0;if(op==effIdle_Deprecated||op==effEditIdle){p->flushHostUpdates();return 1;}if(op==effGetParamName&&index>=kPerformQ0&&index<=kPerformQ3){static const char*names[]={"Rate","Velocity","Humanize","Length"};mpc::copyString(static_cast<char*>(ptr),64,names[index-kPerformQ0]);return 0;}if(op==effGetParamName&&p->isRuleSelector(index)){mpc::copyString(static_cast<char*>(ptr),64,p->param(kRuleKickStyle+(p->isFullRuleSelector(index)?index-kRulePickFullKick:index-kRulePickKick)).name.c_str());return 0;}if(op==effGetParamName&&index>=kSoundQ0&&index<=kSoundQ15){p->flushFromValueRead();int t=p->soundTarget(index-kSoundQ0);mpc::copyString(static_cast<char*>(ptr),64,t>=0?p->param(p->isRuleSelector(t)?kRuleKickStyle+(p->isFullRuleSelector(t)?t-kRulePickFullKick:t-kRulePickKick):t).name.c_str():"Unused");return 0;}if(op==effGetParamName&&index>=kQ1&&index<=kQ16){p->flushFromValueRead();int t=p->proxyTarget(index-kQ1);mpc::copyString(static_cast<char*>(ptr),64,t>=0?p->param(p->isRuleSelector(t)?kRuleKickStyle+(p->isFullRuleSelector(t)?t-kRulePickFullKick:t-kRulePickKick):t).name.c_str():"Unused");return 0;}if(op==effGetParamDisplay||op==effGetParamName)p->flushFromValueRead();return mpc::detail::dispatcher(e,op,index,value,ptr,opt);}
static float grooveGet(AEffect*e,VstInt32 index){static_cast<GrooveDice*>(e->object)->flushFromValueRead();return mpc::detail::getParameter(e,index);}
extern "C" MPC_EXPORT AEffect* VSTPluginMain(audioMasterCallback host){if(host&&host(nullptr,audioMasterVersion,0,0,nullptr,0)==0)return nullptr;auto*e=mpc::detail::createEffect(new GrooveDice(),host);if(e){e->dispatcher=&grooveDispatch;e->getParameter=&grooveGet;}return e;}
#if defined(__APPLE__)
extern "C" MPC_EXPORT AEffect* main_macho(audioMasterCallback host){return VSTPluginMain(host);}
#endif
#endif
