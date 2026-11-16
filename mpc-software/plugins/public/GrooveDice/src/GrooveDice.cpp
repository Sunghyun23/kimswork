// Groove Dice — private instrument by kim. SDK MIT; library assets separately licensed.
// Signal: samples -> role/choke voices -> part gain/pan -> drive/dust/room/duck -> main or stereo stems.
// Sequencer uses host PPQ with block-local sample offsets; physical/touch input alone feeds fill recording.
#include "mpc_plugin.h"
#include "dsp/wav.h"
#include "Parameters.h"
#include "SoundBank.h"
#include "Library.h"
#include <array>
#include <algorithm>
#include <limits>
#include <pthread.h>

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
 struct Voice {int sample=0,role=0,librarySlot=-1;double pos=0,step=1;float gain=0,env=0,decay=.999f,choke=1;int gate=0;bool active=false;};
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
 std::array<Voice,48> voices{};
 std::array<std::atomic<float>,8*72> savedKits{};
 std::array<std::atomic<bool>,8> kitValid{};
 std::atomic<int> lastAction{-1};std::atomic<unsigned> actionCount{0};
 std::atomic<unsigned> diceRolls{0};std::atomic<int> diceResult{0};std::atomic<bool> factoryDirty{false};
 std::atomic<int> recordState{0},recorded{0},fired{0};
 std::atomic<float> position{0},currentTempo{92};std::atomic<int>lastMidiNote{-1},lastMidiRole{-1};

 static mpc::PluginInfo description(){mpc::PluginInfo i;i.name="Groove Dice";i.vendor="kim";i.uniqueId=MPC_FOURCC('K','p','G','y');i.version=10000;i.isInstrument=true;i.numInputs=0;i.numOutputs=16;return i;}
 GrooveDice():mpc::Plugin(description()){
  declareParameters(*this);
  for(auto& n:lengths)n.store(4*PPQ);
  for(int b=0;b<7;++b)seedPattern(b);

  if(!bank.load(mpc::contentPath("Banks/default.gdbank")))for(int s=0;s<Samples;++s)generateSound(s);
  library.load(mpc::contentPath("Banks/default.gdbank"));if(library.loaded)library.loadExpansions(mpc::contentFolder());
  allKits.clear();for(size_t j=0;j<library.factoryKits.size();++j)allKits.push_back(int(j));for(int g=0;g<4;++g)for(size_t j=0;j<library.soundPools[g].size();++j)allSounds[g].push_back(int(j));
  if(bank.loaded&&bank.patternCount){clearBank(5);lengths[5].store(bank.patternLength);for(int i=0;i<bank.patternCount;++i){auto n=bank.pattern[i];notes[5][i].write({n.tick,n.role,n.velocity,100,n.gate});}}
  for(int b=4;b<7;++b){factoryLengths[b-4]=lengths[b].load();baseLengths[b].store(lengths[b].load());for(int i=0;i<MaxNotes;++i){factoryBases[b-4][i*2]=notes[b][i].data.load();factoryBases[b-4][i*2+1]=notes[b][i].gate.load();bases[b][i].data.store(notes[b][i].data.load());bases[b][i].gate.store(notes[b][i].gate.load());}baseValid[b].store(true);}
  std::vector<float> init;for(int i=0;i<numParams();++i)init.push_back(param(i).def);
  for(int key:{kDrive,kDust,kRoom,kDuck,kLowCut,kModDepth})init[key]=0;
  addProgram("Init / Pocket",init);
  for(size_t n=1;n<=library.factoryKits.size();++n){auto values=init;values[kXKit]=float(n);values[kFactoryKit]=float(std::min(size_t(96),n));values[kXQKit]=float(n)/std::max(size_t(1),library.factoryKits.size());factoryValues(library.factoryKits[n-1],values);addProgram(kitCaption(int(n)).c_str(),values);}
 }
 void prepare(double rate,int)override{sr=rate;room.fill(0);roomPos=0;}
 void reset()override{
  budgets={};lastHat=-1e12;for(auto& v:voices)deactivate(v);lastEnd=-1e12;lastRawEnd=-1e12;transportOffset=0;previewPpq=0;fillStart=-1;fillEnd=-1;recStart=-1;recEnd=-1;recordState.store(0);recorded.store(0);ducker=0;
  // Saved editor context is restored; a newly created instance defaults to MAIN.
  for(int i=0;i<kParamCount;++i)last[i]=getNormalised(i);
  lastPackFilter=getInt(kPackFilter);for(int q=0;q<5;++q)catalogSeen[q].store(restoringHasX?getInt(q==0?kXKit:kXSoundKick+q-1):-1);lastLibraryKind=getInt(kLibraryKind);lastKit=getInt(kKit);lastSlot=getInt(kFillSlot);lastLength=getInt(kFillLength);lastSelected=getInt(kNoteSelect);
  for(auto& t:proxySeenTarget)t=-1;
  noteDirty.store(0);pendingRoll.store(false);rollAt=-1;soundDirty.store(0);mirrorNote(true);
 }
 // Internal writes are tagged per thread; UI writes cannot be suppressed by the audio thread.
 inline static thread_local bool internal=false;
 static int canonical(int i){
  if(i==kFactoryKit)return kXKit;if(i>=kSoundKick&&i<=kSoundPerc)return kXSoundKick+i-kSoundKick;
  if(i>=kLibrarySample0&&i<=kLibrarySample7)return kXSample0+i-kLibrarySample0;
  if(i==kLibraryBeat)return kXBeat;if(i==kGrooveTemplate)return kXGroove;if(i==kKitBrowserPage)return kXKitPage;if(i==kSoundBrowserPage)return kXSoundPage;if(i==kLibraryPage)return kXLibraryPage;if(i==kLibraryFolder)return kXLibraryFolder;return i;
 }
 static int legacy(int i){if(i==kXKit)return kFactoryKit;if(i>=kXSoundKick&&i<=kXSoundPerc)return kSoundKick+i-kXSoundKick;if(i>=kXSample0&&i<=kXSample7)return kLibrarySample0+i-kXSample0;if(i==kXBeat)return kLibraryBeat;if(i==kXGroove)return kGrooveTemplate;if(i==kXKitPage)return kKitBrowserPage;if(i==kXSoundPage)return kSoundBrowserPage;if(i==kXLibraryPage)return kLibraryPage;if(i==kXLibraryFolder)return kLibraryFolder;return -1;}
 void mirrorLegacy(int i){int old=legacy(i);if(old>=0){bool prev=internal;internal=true;mpc::Plugin::set(old,get(i));internal=prev;}}
 void set(int i,float value){int key=canonical(i);clearOrphan(key);mpc::Plugin::set(key,value);mirrorLegacy(key);}
 void setNormalised(int i,float value){mpc::Plugin::setNormalised(i,value);mirrorLegacy(i);}
 void migrateLegacy(){bool prev=internal;internal=true;for(int i=0;i<kXKit;++i)if(canonical(i)!=i)mpc::Plugin::set(canonical(i),get(i));internal=prev;}
 std::array<std::array<char,65>,74>unresolved{};std::array<std::atomic<int>,8>orphanSample{};std::array<std::atomic<int>,64>orphanSaved{};std::array<std::atomic<int>,2>orphanMidi{};std::string unresolvedKit,unresolvedPack;std::atomic<bool>orphanKit{false},orphanPack{false};
 void clearOrphan(int key){if(restoring.load())return;if(key>=kXSample0&&key<=kXSample7){orphanSample[key-kXSample0].store(0);missingSample[key-kXSample0].store(false);}if(key==kXBeat||key==kXGroove)orphanMidi[key==kXBeat?0:1].store(0);if(key==kXKit)orphanKit.store(false);if(key==kPackFilter)orphanPack.store(false);}
 const char*unresolvedText(int ref)const{return ref>0&&ref<=74?unresolved[ref-1].data():nullptr;}
 bool restoringHasX=false;
 void parameterChanged(int i,float plain)override{
  if(internal||restoring.load()||loadGuardFrames.load()>0)return;
  clearOrphan(canonical(i));if(canonical(i)!=i){float raw=getNormalised(i);set(canonical(i),plain);bool prev=internal;internal=true;mpc::Plugin::setNormalised(i,raw);internal=prev;return;}mirrorLegacy(i);
  if(i>=kXQKit&&i<=kXQPerc){int target=i==kXQKit?kXKit:kXSoundKick+i-kXQKick;const auto&list=i==kXQKit?kitItems():soundItems(i-kXQKick);int pick=mpc::clamp(int(std::lround(plain*list.size())),0,int(list.size()));catalogSeen[i-kXQKit].store(pick?list[pick-1]+1:0);bool prev=internal;internal=true;set(target,pick?list[pick-1]+1:0);internal=prev;if(i==kXQKit)factoryDirty.store(true);else{soundRequests[i-kXQKick].store(get(target));soundDirty.fetch_or(1u<<(i-kXQKick));}changed[target].store(true);displayPending.store(true);}
  if(i==kXKit||i>=kXSoundKick&&i<=kXSoundPerc){int q=i==kXKit?kXQKit:kXQKick+i-kXSoundKick;const auto&list=i==kXKit?kitItems():soundItems(i-kXSoundKick);int pos=0;for(size_t j=0;j<list.size();++j)if(list[j]+1==getInt(i))pos=int(j)+1;bool prev=internal;internal=true;set(q,list.empty()?0.f:float(pos)/list.size());internal=prev;changed[q].store(true);}
  if(i==kHatGrid){static const int legacy[]={0,3,6,8};internal=true;set(kHatRhythm,legacy[getInt(kHatGrid)]);internal=false;changed[kHatRhythm].store(true);}
  if(i==kHatRhythm){internal=true;set(kHatRhythmPage,getInt(kHatRhythm)/8);internal=false;changed[kHatRhythmPage].store(true);}
  if(i==kXKit)factoryDirty.store(true);
  if(i>=kXSoundKick&&i<=kXSoundPerc){soundRequests[i-kXSoundKick].store(plain);soundDirty.fetch_or(1u<<(i-kXSoundKick));}
  if(i>=kSwing&&i<=kRndNudge)grooveBaseValid.store(false);
  if(i==kKit){internal=true;set(kXKit,0);internal=false;}
  if(i>=kXSample0&&i<=kXSample7){internal=true;set(kXKit,0);internal=false;}
  if(i==kPattern){internal=true;set(kXBeat,0);internal=false;}
  if(i==kEditor||i>=kXSample0)displayPending.store(true);
  for(int r=0;r<8;++r)if(i==kSample0+r*9){missingSample[r].store(false);internal=true;set(kXKit,0);set(kXSample0+r,0);internal=false;displayPending.store(true);}
  if(i>=kQ1&&i<=kQ16){int target=proxyTarget(i-kQ1);if(target>=0){float before=getNormalised(target);internal=true;setNormalised(target,plain);internal=false;if(target>=kXQKit&&target<=kXQPerc)parameterChanged(target,plain);if(getNormalised(target)!=before){changed[target].store(true);if(param(target).kind==mpc::Param::Kind::Choice)displayPending.store(true);}if(target==kHatRhythm){internal=true;set(kHatRhythmPage,getInt(kHatRhythm)/8);internal=false;changed[kHatRhythmPage].store(true);}if(target>=kXSoundKick&&target<=kXSoundPerc){soundRequests[target-kXSoundKick].store(get(target));soundDirty.fetch_or(1u<<(target-kXSoundKick));}if(target>=kSwing&&target<=kRndNudge)grooveBaseValid.store(false);if(target>=kNoteNudge&&target<=kNoteChance)noteDirty.fetch_or(1u<<(target-kNoteNudge));}else{internal=true;setNormalised(i,0);internal=false;}}
  else if(i>=kNoteNudge&&i<=kNoteChance)noteDirty.fetch_or(1u<<(i-kNoteNudge));
 }
 bool formatValue(int i,float plain,std::string& out)override{
  if(canonical(i)!=i)return formatValue(canonical(i),plain,out);
  if(i==kXQKit){int index=getInt(kXKit);if(index>0&&index<=int(library.factoryKits.size())){const auto&name=library.factoryKits[index-1].name;out=name.substr(name.find_last_of('/')+1);while(!out.empty()&&out.front()==' ')out.erase(out.begin());}else out="CUSTOM";return true;}
  if(i>=kXQKick&&i<=kXQPerc)return formatValue(kXSoundKick+i-kXQKick,get(kXSoundKick+i-kXQKick),out);
  if(i==kXKit){int index=getInt(i);out=kitCaption(index);if(library.packs.size()>1&&index>0&&index<=int(library.factoryKits.size())){const auto&k=library.factoryKits[index-1];auto name=k.name.substr(k.name.find_last_of('/')+1);out=std::to_string(k.pack+1)+" /"+name;}return true;}
  if(i==kPackInfo){int p=getInt(kPackFilter);out=p>0&&p<=int(library.packs.size())?std::to_string(p)+" "+shortCaption(library.packs[p-1].name,15)+(library.packs[p-1].isPrivate?" PRIVATE":""):"ALL EXPANSIONS";return true;}
  if(i>=kKitRow0&&i<=kKitRow7){int j=getInt(kXKitPage)*8+i-kKitRow0;out=j==0?"Custom / Legacy":j>0&&j<=int(kitItems().size())?kitCaption(kitItems()[j-1]+1):"";return true;}
  if(i==kActionInfo){int a=lastAction.load();std::string name=a>=0?param(a).name:"";
   if(a>=kKitPick0&&a<=kKitPick7)name="KIT SELECTED";else if(a==kPackPrev||a==kPackNext)name="EXPANSION CHANGED";else if(a>=kSoundPick0&&a<=kSoundPick7)name="SOUND SELECTED";else if(a>=kLibraryPick0&&a<=kLibraryPick7)name="LIBRARY SELECTED";
   else if(a==kSoundPrev||a==kLibraryPrev||a==kKitPagePrev)name="PREVIOUS PAGE";else if(a==kSoundNext||a==kLibraryNext||a==kKitPageNext)name="NEXT PAGE";
   else if(a==kKitPrev||a==kKitNext)name="KIT CHANGED";else if(a==kPatternPrev||a==kPatternNext)name="PATTERN CHANGED";else if(a==kFactoryReload)name="KIT SELECTED";
   else if(a==kLibraryBack)name="FOLDERS";else if(a==kLibraryLegacy)name="LEGACY SOUNDS";else if(a==kLibraryBeatOff)name="BEAT OFF";else if(a==kGrooveOff)name="GROOVE OFF";
   else if(a==kGridSelect||a==kGridTap)name="NOTE SELECTED";
   out=a>=0?std::to_string(actionCount.load())+" / "+name:"TAP A PAD OR PRESS PREVIEW";return true;}
  if(i>=kXSoundKick&&i<=kXSoundPerc){int g=i-kXSoundKick,id=getInt(kXSample0+(g==0?0:g==1?1:g==2?2:7));out=g==3?(getInt(kXSoundPerc)>0&&getInt(kXSoundPerc)<=int(library.soundPools[3].size())?library.soundPools[3][getInt(kXSoundPerc)-1].name.substr(0,3)+" SET"+std::to_string(getInt(kXSoundPerc)):"CUSTOM SET"):id>0&&id<=int(library.entries.size())?shortCaption(library.entries[id-1].name,14):"CUSTOM";return true;}
  if(i>=kSoundRow0&&i<=kSoundRow7){int g=getInt(kSoundBrowserGroup),row=getInt(kXSoundPage)*8+i-kSoundRow0;const auto&list=soundItems(g);int index=row<int(list.size())?list[row]:-1;out=index>=0?(g==3?library.soundPools[g][index].name:shortCaption(library.entries[library.soundPools[g][index].sounds[0]-1].name,30)):"";return true;}
  if(i==kSoundInfo){int count=int(soundItems(getInt(kSoundBrowserGroup)).size());out=std::to_string(count)+" SOUNDS | "+std::to_string(getInt(kXSoundPage)+1)+" / "+std::to_string(std::max(1,(count+7)/8));return true;}
  if(i==kDiceInfo&&pendingRoll.load()){out="NEXT BAR";return true;}
  if(i==kDiceInfo){int result=diceResult.load();out=result==1?"AMOUNT ZERO":result==2?"KIT DATA MISSING":result==3?"RHYTHM LOCK LIMIT":diceRolls.load()?std::string(getBool(kKitLock)?"KIT LOCKED / ":"ROLLED / ")+std::to_string(diceRolls.load()):"READY";return true;}
  if(i==kXKitPage){out=std::to_string(getInt(i)+1)+" / "+std::to_string((int(kitItems().size())+8)/8);return true;}
  if(i==kHatRhythm){static const char* captions[]={"1/4","1/8","1/16","1/8D","1/16D"};int v=getInt(i);out=captions[v/3];if(v%3)out+=v%3==1?" FILL":" ROLL";return true;}
  if(i==kScope){out="SCOPE · "+param(i).choices[getInt(i)];return true;}
  for(int r=0;r<8;++r)if(i==kSample0+r*9&&getInt(kXSample0+r)>0){out="LIBRARY";return true;}
  if(i==kLibraryCurrentSound){static const char* names[]={"KICK","SNARE","CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};int role=getInt(kLibraryRole),id=getInt(kXSample0+role);out=std::string(names[role])+" / ";if(id>0&&id<=int(library.entries.size()))out+=shortCaption(library.entries[id-1].name,31-out.size());else out+="VARIATION "+std::to_string(getInt(kSample0+role*9)+1);return true;}
  if(i==kLibraryPlaybackInfo){int beat=getInt(kXBeat),groove=getInt(kXGroove);out=beat>0&&beat<=int(library.categories[3].size())?shortCaption(library.entries[library.categories[3][beat-1]].name,20):param(kPattern).choices[getInt(kPattern)];out+=" / GRV "+(groove?std::to_string(groove):std::string("OFF"));return true;}
  if(i==kPattern&&getInt(kXBeat)>0&&getInt(kXBeat)<=int(library.categories[3].size())){const auto& name=library.entries[library.categories[3][getInt(kXBeat)-1]].name;out=name.substr(name.find_last_of('/')+1);if(out.size()>22)out=out.substr(0,19)+"...";return true;}
  if(i==kPreviewBpm){char text[32];std::snprintf(text,sizeof(text),"%.0f BPM",currentTempo.load());out=text;return true;}
  if(i>=kLibraryRow0&&i<=kLibraryRow7){int kind=getInt(kLibraryKind),index=getInt(kXLibraryPage)*8+i-kLibraryRow0,folder=getInt(kXLibraryFolder);if(kind<3&&folder==0){const auto&list=folderItems(kind);out="";if(index<int(list.size())){const auto& name=library.folders[list[index]].name;size_t slash=name.find_last_of('/'),previous=slash>0?name.find_last_of('/',slash-1):std::string::npos;std::string family=name.substr(previous==std::string::npos?0:previous+1,slash-(previous==std::string::npos?0:previous+1)),leaf=name.substr(slash+1);out="> "+family.substr(0,10)+" / "+shortCaption(leaf,14);}}else{auto&list=browserItems();if(index>=int(list.size())){out="";return true;}out=shortCaption(library.entries[list[index]].name,30);}return true;}
  if(i==kLibraryInfo){int kind=getInt(kLibraryKind),folder=getInt(kXLibraryFolder),count=browserCount();out=std::string(kind<3&&folder==0?"FOLDERS ":"FILES ")+std::to_string(getInt(kXLibraryPage)+1)+" / "+std::to_string(std::max(1,(count+7)/8))+"  |  "+std::to_string(count);return true;}
  if(i>=kXSample0&&i<=kXSample7){int id=getInt(i);out=id>0&&id<=int(library.entries.size())?shortCaption(library.entries[id-1].name,30):"LEGACY VARIATIONS";return true;}
  if(i==kXBeat||i==kXGroove){int k=i==kXBeat?3:4,id=getInt(i);out=id>0&&id<=int(library.categories[k].size())?shortCaption(library.entries[library.categories[k][id-1]].name,26):"OFF";return true;}
  if(i==kBankInfo){out=library.loaded?"LIBRARY "+std::to_string(library.categories[0].size()+library.categories[1].size()+library.categories[2].size())+" WAV / "+std::to_string(library.categories[3].size()+library.categories[4].size())+" MIDI":bank.loaded?"BANK · 64 SOUNDS":"BANK MISSING · TEST SOUNDS";for(int r=0;r<8;++r)if(getInt(kXSample0+r)>0&&library.active[r].load()!=getInt(kXSample0+r))out=library.failed[r].load()?"LIBRARY SAMPLE ERROR":"LOADING SOUND";for(auto& m:missingSample)if(m.load())out="MISSING SAMPLE";if(library.expansionErrors>0)out="EXPANSION LOAD ERROR / "+std::to_string(library.expansionErrors);if(lastMidiNote.load()>=0){static const char* role[]={"KICK","SNARE","CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};int r=lastMidiRole.load();out=std::string(library.loaded?"LIBRARY | ":bank.loaded?"BANK64 | ":"TEST | ")+std::to_string(lastMidiNote.load())+" "+(r>=0?role[r]:"UNASSIGNED");}bool missing=false;for(auto&m:missingSample)missing|=m.load();if(missing)out="MISSING SAMPLE";else if(orphanMidi[0].load()>0)out="MISSING PATTERN";else if(orphanMidi[1].load()>0)out="MISSING GROOVE";return true;}
  if(i>=kQ1&&i<=kQ16){int t=proxyTarget(i-kQ1);if(t<0){out="--";return true;}if(t>=kXQKit&&t<=kXQPerc||t>=kXSoundKick&&t<=kXSoundPerc||t==kHatRhythm)return formatValue(t,get(t),out);out=param(t).display(get(t));if(!param(t).unit.empty())out+=" "+param(t).unit;return true;}
  if(i==kStatus){static const char* status[]={"READY","COUNT-IN","RECORDING","RECORDED","QUEUED","PLAYING","FULL"};out=status[mpc::clamp(recordState.load(),0,6)];return true;}
  if(i==kRecordedCount){out=std::to_string(countNotes(getInt(kFillSlot)));return true;}
  if(i>=kGrid0&&i<=kGrid63){out=get(i)>.5f?"HIT":"EMPTY";return true;}
  if(i==kNoteSelect){int n=getInt(kNoteSelect);int slot=getInt(kFillSlot);if(notes[slot][n].valid()){auto e=notes[slot][n].read();static const char* names[]={"KICK","SNARE","CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};out=std::string(names[e.role])+" / "+std::to_string(1+e.tick/240);}else out="EMPTY";return true;}
  if(i==kPosition){char b[48];std::snprintf(b,sizeof(b),"BAR %d",1+int(position.load()/4));out=b;return true;}
  if(!param(i).unit.empty()){out=param(i).display(get(i))+" "+param(i).unit;return true;}
  return false;
 }
 void process(mpc::ProcessContext& c)override{
  markRenderThread();
  std::array<int,8> desired{};for(int r=0;r<8;++r)desired[r]=getInt(kXSample0+r);library.poll(desired);
  sr=c.sampleRate;const double bpm=c.transport.tempoValid?mpc::clamp(c.transport.bpm,20.,300.):get(kPreviewBpm);
  currentTempo.store(float(bpm));
  const double step=bpm/(60.*sr);bool run=(getBool(kFollow)&&c.transport.playing)||getBool(kPreview);
  bool hostRunning=getBool(kFollow)&&c.transport.playing&&c.transport.ppqValid;
  double rawStart=hostRunning?c.transport.ppqPos:previewPpq;
  double loopLength=c.transport.cycleEndPpq-c.transport.cycleStartPpq;
  if(hostRunning&&lastRawEnd>-1e10&&rawStart<lastRawEnd-.02&&c.transport.cycling&&loopLength>0&&std::abs(rawStart+loopLength-lastRawEnd)<.02)transportOffset+=loopLength;
  else if(hostRunning&&lastRawEnd>-1e10&&std::abs(rawStart-lastRawEnd)>.02)transportOffset=0;
  double start=hostRunning?rawStart+transportOffset:previewPpq;
  if(!run)start=previewPpq;
  double end=start+step*c.numFrames;position.store(float(start));
  if(run&&std::abs(start-lastEnd)>.02&&lastEnd>-1e10){pendingRoll.store(false);displayPending.store(true);rollAt=-1;budgets={};lastHat=-1e12;fillStart=fillEnd=-1;recordState.store(0);recStart=recEnd=-1;for(auto&v:voices)deactivate(v);}
  if(!run&&wasRunning){pendingRoll.store(false);displayPending.store(true);rollAt=-1;budgets={};lastHat=-1e12;fillStart=fillEnd=-1;recStart=recEnd=-1;recordState.store(0);for(auto&v:voices)deactivate(v);}
  wasRunning=run;
  int guard=loadGuardFrames.load();bool loading=guard>0;if(loading){loadGuardFrames.fetch_sub(std::min(guard,c.numFrames));if(guard<=c.numFrames)for(auto&t:proxySeenTarget)t=-1;}
  const auto command=[&](int i){float x=getNormalised(i);bool changed=x!=last[i];last[i]=x;if(changed&&!loading){lastAction.store(i);actionCount.fetch_add(1);this->changed[kActionInfo].store(true);}return changed&&!loading;};
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
  if(command(kRoll)){if(run){rollAt=(std::floor(start/4.)+1)*4.;pendingRoll.store(true);displayPending.store(true);}else{snapshotUndo();roll();}}
  if(pendingRoll.load()&&run&&start>=rollAt-1e-9){pendingRoll.store(false);snapshotUndo();roll();}
  if(command(kUndo)){pendingRoll.store(false);if(haveUndo)restoreUndo();}
  if(command(kKeep)){keep=true;}
  if(command(kSaveKit))saveKit();if(command(kLoadKit))loadKit();
  if(command(kCancel)){recStart=recEnd=-1;fillStart=fillEnd=-1;recordState.store(0);}
  if(command(kRecord)){
   if(!run){internal=true;set(kPreview,1);internal=false;run=true;previewPpq=0;start=0;end=start+step*c.numFrames;}
   snapshotUndo();recSlot=getInt(kFillSlot);recLength=lengthForChoice(getInt(kFillLength));recMode=getInt(kRecMode);
   recStart=std::ceil((start-1e-9)/4.)*4.+4*getInt(kCountIn);if(recStart<start-1e-9)recStart+=4;
   recEnd=recStart+double(recLength)/PPQ;recordState.store(1);recorded.store(0);recCleared=false;
  }
  if(command(kTrigger))queueFill(start);
  bool gridSelect=command(kGridSelect),gridTap=command(kGridTap);if(gridSelect||gridTap){baseValid[getInt(kFillSlot)].store(false);int cell=getInt(kGridSelect)-1;if(cell>=0){int role=cell/16,step=cell%16+getInt(kGridBar)*16;int selected=-1;for(int i=0;i<MaxNotes;++i)if(notes[getInt(kFillSlot)][i].valid()){auto n=notes[getInt(kFillSlot)][i].read();if(n.role==role&&n.tick/240==step){selected=i;break;}}if(selected<0){for(int i=0;i<MaxNotes;++i)if(!notes[getInt(kFillSlot)][i].valid()){selected=i;notes[getInt(kFillSlot)][i].write({step*240,role,100,100,80});break;}}if(selected>=0){internal=true;set(kNoteSelect,selected);internal=false;mirrorNote();}}}
  if(command(kNoteDelete)){baseValid[getInt(kFillSlot)].store(false);int slot=getInt(kFillSlot),n=getInt(kNoteSelect);notes[slot][n].data.store(0);}
  if(getInt(kKit)!=lastKit){lastKit=getInt(kKit);internal=true;for(int v=0;v<8;++v){set(kSample0+v*9,lastKit);set(kXSample0+v,0);}internal=false;}
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
  if(run){int interval=getInt(kFillEvery);if(interval&&fillStart<0){int bars=interval==1?4:interval==2?8:16;int bar=int(std::floor(start/4));int nextBar=int(std::floor(end/4));int target=(nextBar%bars==bars-1)?nextBar:bar;if(target%bars==bars-1&&target!=lastAutoBar){lastAutoBar=target;activeFill=getInt(kFillSlot);fillStart=(target+1)*4.-double(lengths[activeFill].load())/PPQ;fillEnd=(target+1)*4.;}}scheduleBank(4+getInt(kPattern),start,end,step,false);if(fillStart>=0)scheduleBank(activeFill,start,end,step,true);}
  for(int v=0;v<8;++v)if(command(kPad0+v*9)){addEvent(0,v,100,true);}
  int midi=0;
  const float drive=get(kDrive)*.01f,dust=get(kDust)*.01f,roomAmount=get(kRoom)*.01f,duck=get(kDuck)*.01f;
  const float masterTarget=get(kMaster),modDepth=get(kModDepth)*.01f;double phaseInc=get(kModRate)/sr;
  float hpAlpha=float(1.-std::exp(-6.283185307*get(kLowCut)/sr));
  for(int frame=0;frame<c.numFrames;++frame){
   const double now=start+frame*step;
   if(run&&recStart>=0&&now>=recStart&&now<recEnd&&!recCleared){if(recMode==0)clearBank(recSlot);lengths[recSlot].store(recLength);recCleared=true;recordState.store(2);}
   if(run&&recStart>=0&&now>=recEnd)finishRecord();
   if(fillStart>=0&&now>=fillStart&&now<fillEnd&&recordState.load()!=1&&recordState.load()!=2)recordState.store(5);
   if(fillEnd>=0&&now>=fillEnd){fillStart=fillEnd=-1;if(recordState.load()==5||recordState.load()==4)recordState.store(0);}
   // Automatic fill queues at the final bar of each selected interval.
   int interval=getInt(kFillEvery);if(run&&interval&&fillStart<0){int bars=interval==1?4:interval==2?8:16;int bar=int(std::floor(now/4));if(bar%bars==bars-1&&bar!=lastAutoBar){lastAutoBar=bar;activeFill=getInt(kFillSlot);fillStart=(bar+1)*4.-double(lengths[activeFill].load())/PPQ;fillEnd=(bar+1)*4.;}}
   while(midi<c.numMidiEvents&&c.midi[midi].sampleOffset<=frame){auto e=c.midi[midi++];if(e.isNoteOn()){int r=mapNote(e.note());lastMidiNote.store(e.note());lastMidiRole.store(r);displayPending.store(true);if(r>=0){trigger(r,e.velocity());recordInput(now,r,e.velocity());}}if(e.isAllNotesOff()){for(auto&v:voices)deactivate(v);recStart=recEnd=-1;fillStart=fillEnd=-1;recordState.store(0);}}
   for(int e=0;e<eventCount;++e)if(events[e].offset==frame){trigger(events[e].role,events[e].velocity,events[e].gate);if(events[e].input)recordInput(now,events[e].role,events[e].velocity);}
   driveS+=.002f*(drive-driveS);dustS+=.002f*(dust-dustS);roomS+=.002f*(roomAmount-roomS);duckS+=.002f*(duck-duckS);
   float bus[8][2]{};
   for(auto& v:voices){if(!v.active)continue;auto& s=v.librarySlot>=0?library.slots[v.librarySlot].sound:sounds[v.sample];int idx=int(v.pos);if(idx+1>=int(s.numFrames())||v.env<.00001f){deactivate(v);continue;}
    float fract=float(v.pos-idx);float l=s.channels[0][idx]+fract*(s.channels[0][idx+1]-s.channels[0][idx]);int rc=s.numChannels()>1?1:0;float r=s.channels[rc][idx]+fract*(s.channels[rc][idx+1]-s.channels[rc][idx]);
    float pan=get(kPan0+v.role*9)*.01f;float gain=v.gain*v.env*(v.role==0?1.f:1-duckS*ducker*.6f);int route=getBool(kSeparate)?getInt(kRoute0+v.role*9):0;
    bus[route][0]+=l*gain*(pan>0?1-pan:1);bus[route][1]+=r*gain*(pan<0?1+pan:1);
    v.pos+=v.step;v.env*=v.decay*v.choke;if(getBool(kGated)&&--v.gate<=0)v.choke=.99f;
   }
   modPhase+=phaseInc;if(modPhase>=1)modPhase-=1;float modulation=1-modDepth*.15f*(1+std::sin(float(modPhase*6.283185307)));
   master+=.002f*(masterTarget-master);ducker*=.9992f;
   for(int out=0;out<8;++out){for(int ch=0;ch<2;++ch){float x=bus[out][ch];float shaped=x*(1+driveS*4);x=shaped/(1+std::abs(shaped)*driveS);
     low[out][ch]+=(.75f-.65f*dustS)*(x-low[out][ch]);x=x*(1-dustS)+low[out][ch]*dustS;
     {int rp=(out*4096+roomPos)*2+ch;float echo=room[rp];float feedback=x+echo*.25f;room[rp]=std::abs(feedback)<1e-20f?0.f:feedback;x+=echo*roomS*.35f;}
     if(hpAlpha>0){high[out][ch]+=hpAlpha*(x-high[out][ch]);x-=high[out][ch];}
     if(std::abs(x)<1e-20f)x=0;
     x*=master*modulation;if(out*2+ch<c.numOutputs)c.outputs[out*2+ch][frame]+=mpc::clamp(x,-.99f,.99f);
    }}
   roomPos=(roomPos+1)%4096;
  }
  if(run){previewPpq=end;lastEnd=end;if(hostRunning)lastRawEnd=rawStart+step*c.numFrames;else lastRawEnd=-1e12;}else {lastEnd=-1e12;lastRawEnd=-1e12;transportOffset=0;}
  if(recStart>=0&&end>=recEnd)finishRecord();
  syncProxies();
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

 }
 void loadExtraState(const uint8_t* data,size_t size)override{
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

 }
 bool internalLoadState(const uint8_t* data,size_t size){if(!data||size<12||std::memcmp(data,"MPCP",4))return false;restoringHasX=false;{size_t pos=12;uint32_t count=0,version=0;for(int k=0;k<4;++k){count|=uint32_t(data[8+k])<<(8*k);version|=uint32_t(data[4+k])<<(8*k);}for(uint32_t j=0;j<count&&pos<size;++j){size_t n=data[pos++];if(n>size-pos||size-pos-n<(version>=2?8u:4u))break;if(n==4&&!std::memcmp(data+pos,"XKit",4))restoringHasX=true;pos+=n+(version>=2?8:4);}}restoring.store(true);loadGuardFrames.store(int(sr*.5));for(int i=kFactoryKit;i<kParamCount;++i)mpc::Plugin::set(i,param(i).def);factoryDirty.store(false);bool ok=mpc::Plugin::internalLoadState(data,size);if(ok){bool hasRhythm=false;size_t pos=12;uint32_t n=0,version=0;for(int k=0;k<4;++k){n|=uint32_t(data[8+k])<<(8*k);version|=uint32_t(data[4+k])<<(8*k);}for(uint32_t j=0;j<n&&pos<size;++j){size_t len=data[pos++];if(len>size-pos||size-pos-len<(version>=2?8u:4u))break;if(len==9&&!std::memcmp(data+pos,"HatRhythm",9))hasRhythm=true;pos+=len+(version>=2?8:4);}if(!hasRhythm){static const int legacy[]={0,3,6,8};set(kHatRhythm,legacy[getInt(kHatGrid)]);set(kHatRhythmPage,getInt(kHatRhythm)/8);}}factoryDirty.store(false);soundDirty.store(0);restoring.store(false);return ok;}
 // Native PRESET changes kit audio only, preserving pad notes, output routing, groove/fills and transport.
 void selectFactoryProgram(int index){if(index<0||index>=numPrograms())return;std::array<float,kParamCount> keep{};for(int i=0;i<kParamCount;++i)keep[i]=getNormalised(i);internal=true;mpc::Plugin::setProgram(index);for(int i=0;i<kParamCount;++i)if(!kitAudioParam(i))setNormalised(i,keep[i]);for(int i=kXKit;i<=kXLibraryFolder;++i)mirrorLegacy(i);for(auto&n:catalogSeen)n.store(-1);internal=false;factoryDirty.store(index>0);queueAudioUi();}
 int activeFactoryProgram()const{return mpc::clamp(getInt(kXKit),0,numPrograms()-1);}
 void markRenderThread(){auto token=uintptr_t(pthread_self());for(auto& t:renderThreads)if(t.load()==token)return;for(auto& t:renderThreads){uintptr_t empty=0;if(t.compare_exchange_strong(empty,token))return;}renderOverflow.store(true);}
 void flushFromValueRead(){if(renderOverflow.load())return;auto token=uintptr_t(pthread_self());for(auto&t:renderThreads)if(t.load()==token)return;flushHostUpdates();}
 void requestHostIdle(){if(host_&&effect_)host_(effect_,audioMasterNeedIdle_Deprecated,0,0,nullptr,0);}
 void flushHostUpdates(){if(flushing.test_and_set())return;for(int i=0;i<kParamCount;++i)if(changed[i].exchange(false))notifyHostParameterChanged(i);if(displayPending.exchange(false))requestDisplayUpdate();flushing.clear();}
 int countNotes(int b)const{int n=0;for(auto& v:notes[mpc::clamp(b,0,6)])if(v.valid())++n;return n;}
 int proxyTarget(int n)const{static const int main[]={kSwing,kSnareLate,kHatShift,kRndNudge,kRuleKick,kRuleSnare,kRuleHat,kRulePerc,kXQKick,kXQSnare,kXQHat,kXQPerc,kDrive,kDust,kRoom,kHatRhythm};static const int edit[]={kFillSlot,kFillLength,kRecMode,kCountIn,kNoteNudge,kNoteVelocity,kNoteGate,kNoteChance};return getBool(kEditor)?(n<8?edit[n]:-1):main[n];}
private:
 std::vector<int>allKits;std::array<std::vector<int>,4>allSounds;int lastPackFilter=-1;std::array<std::atomic<int>,5>catalogSeen{{-1,-1,-1,-1,-1}};
 const groove::Library::Pack* filteredPack()const{int p=getInt(kPackFilter);return p>0&&p<=int(library.packs.size())?&library.packs[p-1]:nullptr;}
 const std::vector<int>&kitItems()const{auto*p=filteredPack();return p?p->kits:allKits;}
 const std::vector<int>&soundItems(int group)const{auto*p=filteredPack();return p?p->sounds[group]:allSounds[group];}
 int kitPosition(int index)const{const auto&list=kitItems();for(size_t j=0;j<list.size();++j)if(list[j]+1==index)return int(j)+1;return 0;}
 std::string kitCaption(int index)const{if(index<=0||index>int(library.factoryKits.size()))return "Custom / Legacy";const auto&k=library.factoryKits[index-1];if(library.packs.size()==1)return k.name;std::string prefix=std::to_string(k.pack+1)+" "+library.packs[k.pack].name.substr(0,10);std::string name=prefix+" / "+k.name;if(name.size()>31)name=prefix.substr(0,10)+"~"+k.name.substr(k.name.size()>19?k.name.size()-19:0);return name;}
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
 std::array<float,kParamCount> last{};std::atomic<unsigned> noteDirty{0};std::array<float,kParamCount> undoValues{};std::array<uint32_t,7*MaxNotes*2+7> undoNotes{};bool haveUndo=false;
 std::array<float,65536> room{};int roomPos=0;float low[8][2]{},high[8][2]{},master=.7f,ducker=0,driveS=.1f,dustS=.1f,roomS=.08f,duckS=.15f;double sr=44100,previewPpq=0,lastEnd=-1e12,lastRawEnd=-1e12,transportOffset=0,modPhase=0;bool wasRunning=false;std::atomic<bool> keep{false};
 double fillStart=-1,fillEnd=-1,recStart=-1,recEnd=-1;int lastLibraryKind=-1,activeFill=0,recSlot=0,recLength=4*PPQ,recMode=0,lastKit=-1,lastSlot=-1,lastSelected=-1,lastLength=-1,lastAutoBar=-1;bool recCleared=false;std::atomic<uint32_t> randomState{0x784571};
 struct Event{int offset,role,velocity;bool input;int gate;};std::array<Event,512>events{};int eventCount=0;
 static int lengthForChoice(int n){static const int l[]={1,2,4,8,16};return l[mpc::clamp(n,0,4)]*PPQ;}
 static int choiceForLength(int n){for(int i=0;i<5;++i)if(lengthForChoice(i)==n)return i;return 2;}
 uint32_t random(){uint32_t x=randomState.load();x^=x<<13;x^=x>>17;x^=x<<5;randomState.store(x);return x;}
 static uint32_t hash(uint32_t n){n^=n>>16;n*=0x7feb352du;n^=n>>15;n*=0x846ca68bu;n^=n>>16;return n;}
 void seedPattern(int b){int i=0;for(int s=0;s<std::min(64,lengths[b].load()/240);++s){if(s%4==0)notes[b][i++].write({s*240,0,105,100,80});if(s==4||s==12)notes[b][i++].write({s*240,1,100,100,80});if(s%2==0)notes[b][i++].write({s*240,2,75+(s%4?0:12),100,50});}}
 void clearBank(int b){baseValid[b].store(false);for(auto&n:notes[b])n.data.store(0);}
 void addEvent(int offset,int role,int vel,bool input=false,int gate=80){if(eventCount<int(events.size()))events[eventCount++]={offset,role,mpc::clamp(vel,1,127),input,gate};}
 int mapNote(int note){for(int r=0;r<8;++r)if(note==getInt(kPadNote0+r*9))return r;int gm=note==36?0:note==38||note==40?1:note==42||note==44?2:note==46?3:note==39?4:note==45?5:note==49?6:note==37?7:-1;return gm;}
 void scheduleBank(int b,double start,double end,double step,bool fill){
  if(!fill&&orphanMidi[0].load()>0&&getInt(kXBeat)==0)return;
  double length=double(lengths[b].load())/PPQ;int base=int(std::floor((start-(fill?fillStart:0))/length));
  int beat=getInt(kXBeat);const groove::Library::Midi* imported=nullptr;if(!fill&&beat>0&&beat<=int(library.categories[3].size()))imported=&library.midi[library.categories[3][beat-1]];
  if(imported){length=double(imported->length)/PPQ;base=int(std::floor(start/length));}
  bool phrase=!fill&&!imported&&phraseBank(b);int grid=phrase?phraseGrid(b):4;double origin=phrase?phraseOrigin(b)*4.:0;if(phrase)base=int(std::floor((start-origin)/length));
  int count=imported?int(imported->notes.size()):MaxNotes;
  for(int cycle=base-1;cycle<=base+1;++cycle)for(int i=0;i<count;++i){Note n;if(imported){auto e=imported->notes[i];n={e.tick,mpc::clamp(e.note-60,0,7),e.velocity,100,e.gate};}else{if(!notes[b][i].valid())continue;n=notes[b][i].read();}int group=ruleGroup(n.role),rule=getInt(kRuleKick+group);int hashTick=phrase&&rule?n.tick%(4*PPQ):n.tick;uint32_t h=hash(uint32_t(hashTick+n.role*20011+((phrase||keep||rule==0)?0:cycle*131)+partSeed[group].load()));float u=float(h&65535)/65535;
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
   uint32_t jitter=hash(h+((phrase||keep||rule==0)?0:uint32_t(cycle)));float jitterAmount=rule&&!fixed?mpc::clamp(get(kRndNudge)+partOffset[group][3].load(),0.f,rule==1?2.f:rule==2?4.f:6.f):0.f;beat+=(int(jitter%2001)-1000)*jitterAmount*.000001/60*tempoForStep(step);
   if(beat<start-1e-10||beat>=end-1e-10)continue;
   if(fill){if(beat<fillStart||beat>=fillEnd)continue;}else if(fillStart>=0&&beat>=fillStart&&beat<fillEnd)continue;
   if(!admitRole(n.role,beat,fixed,phrase,grid))continue;int vel=n.velocity+int((int(h%201)-100)*std::min(get(kVelSpread),rule==0?0.f:rule==1?4.f:rule==2?8.f:12.f)/100);int off=int(std::llround((beat-start)/step));if(off>=0&&off<int((end-start)/step+.1))addEvent(off,n.role,vel,false,n.gate);
  }
  int snareRule=getInt(kRuleSnare);float ghost=std::min(get(kGhost),snareRule<=1?0.f:snareRule==2?25.f:40.f);if(!phrase&&ghost>0){for(int cycle=base-1;cycle<=base+1;++cycle){double beat=(fill?fillStart:0)+cycle*length+2.75;uint32_t h=hash(uint32_t(keep?0:cycle)+partSeed[1].load()+991);if(h%100>=ghost)continue;if(beat>=start&&beat<end&&(!fill||(beat>=fillStart&&beat<fillEnd))&&(fill||fillStart<0||beat<fillStart||beat>=fillEnd)){if(admitRole(1,beat,false))addEvent(int(std::llround((beat-start)/step)),1,35);}}}
 }
 double tempoForStep(double step)const{return step*60*sr;}
 void queueFill(double start){if(recordState.load()==1||recordState.load()==2)return;activeFill=getInt(kFillSlot);fillStart=std::ceil((start+1e-7)/4)*4;fillEnd=fillStart+double(lengths[activeFill].load())/PPQ;recordState.store(4);}
 void recordInput(double beat,int role,int vel){if(recStart<0||beat<recStart||beat>=recEnd||!recCleared)return;baseValid[recSlot].store(false);int tick=mpc::clamp(int(std::llround((beat-recStart)*PPQ)),0,recLength-1);for(auto& n:notes[recSlot])if(!n.valid()){n.write({tick,role,vel,100,80});recorded.fetch_add(1);return;}recordState.store(6);}
 void finishRecord(){recStart=recEnd=-1;if(recordState.load()!=6)recordState.store(3);mirrorNote();}
 void deactivate(Voice& v){if(v.librarySlot>=0){library.release(v.librarySlot);v.librarySlot=-1;}v.active=false;}
 void trigger(int role,int vel,int gateMs=80){if(missingSample[role])return;int selectedLibrary=getInt(kXSample0+role),cache=-1;if(selectedLibrary>0){cache=library.acquire(role,selectedLibrary);if(cache<0){if(library.failed[role].load()==selectedLibrary)return;cache=library.acquireCurrent(role); /* Retain previous source during asynchronous loading; never drop a beat. */}}fired.fetch_add(1);int group=getInt(kChoke0+role*9);if(group)for(auto&v:voices)if(v.active&&getInt(kChoke0+v.role*9)==group)v.choke=float(std::exp(-1./(sr*get(kChokeTime)*.001)));
  Voice* v=nullptr;for(auto&x:voices)if(!x.active){v=&x;break;}if(!v)v=&voices[steal++%voices.size()];deactivate(*v);int sample=role*8+getInt(kSample0+role*9);v->sample=sample;v->librarySlot=cache;const auto& sound=cache>=0?library.slots[cache].sound:sounds[sample];v->role=role;v->pos=0;v->step=sound.sampleRate/sr*std::pow(2.,get(kTune0+role*9)/12.);v->gain=vel/127.f*std::pow(10.f,get(kLevel0+role*9)/20.f);v->env=1;v->decay=float(std::exp(-6.9/(sr*get(kDecay0+role*9)*.001)));v->choke=1;v->gate=int(sr*gateMs*.001);v->active=true;if(role==0)ducker=1;
 }
 int steal=0;
 void syncProxies(){for(int row=0;row<8;++row){int j=getInt(kXKitPage)*8+row;bool selected=j==0?getInt(kXKit)==0:j<=int(kitItems().size())&&kitItems()[j-1]+1==getInt(kXKit);bool prev=internal;internal=true;set(kKitRow0+row,selected?1:0);internal=prev;changed[kKitRow0+row].store(true);}syncSoundSelectors();internal=true;for(int q=0;q<5;++q){int key=q==0?kXKit:kXSoundKick+q-1,id=getInt(key),old=catalogSeen[q].load();if(old!=id){const auto&list=q==0?kitItems():soundItems(q-1);int pos=0;for(size_t j=0;j<list.size();++j)if(list[j]+1==id)pos=int(j)+1;float expected=getNormalised(kXQKit+q),value=list.empty()?0.f:float(pos)/list.size();if(getInt(key)==id&&catalogSeen[q].compare_exchange_strong(old,id)&&values_[kXQKit+q]->compare_exchange_strong(expected,value))changed[kXQKit+q].store(true);}}for(int n=0;n<16;++n){int t=proxyTarget(n);float value=t>=0?getNormalised(t):0;if(proxySeenTarget[n]!=t||proxySeenValue[n]!=value){if(getNormalised(kQ1+n)!=value){setNormalised(kQ1+n,value);changed[kQ1+n].store(true);}proxySeenTarget[n]=t;proxySeenValue[n]=value;}}for(int cell=0;cell<64;++cell){bool hit=false;for(auto& note:notes[getInt(kFillSlot)])if(note.valid()){auto e=note.read();if(e.role==cell/16&&e.tick/240==cell%16+getInt(kGridBar)*16){hit=true;break;}}if(get(kGrid0+cell)!=(hit?1:0)){set(kGrid0+cell,hit?1:0);changed[kGrid0+cell].store(true);}}
  set(kStatus,0);set(kRecordedCount,0);set(kPosition,0);internal=false;}
 void mirrorNote(bool preserve=false){int slot=getInt(kFillSlot),n=getInt(kNoteSelect);if(!notes[slot][n].valid())return;auto e=notes[slot][n].read();internal=true;float nudge=float((e.tick-std::round(e.tick/240.)*240.)/PPQ*60000./currentTempo.load());if(!preserve||getInt(kNoteNudge)!=int(std::lround(nudge)))set(kNoteNudge,nudge);if(!preserve||getInt(kNoteVelocity)!=e.velocity)set(kNoteVelocity,e.velocity);if(!preserve||getInt(kNoteGate)!=e.gate)set(kNoteGate,e.gate);if(!preserve||getInt(kNoteChance)!=e.chance)set(kNoteChance,e.chance);internal=false;}
 void editNote(unsigned dirty){int slot=getInt(kFillSlot);baseValid[slot].store(false);int i=getInt(kNoteSelect);if(!notes[slot][i].valid())return;auto n=notes[slot][i].read();if(dirty&1)n.tick=mpc::clamp(int(std::round(n.tick/240.))*240+int(std::round(get(kNoteNudge)*currentTempo.load()/60000.*PPQ)),0,lengths[slot].load()-1);if(dirty&2)n.velocity=getInt(kNoteVelocity);if(dirty&4)n.gate=getInt(kNoteGate);if(dirty&8)n.chance=getInt(kNoteChance);notes[slot][i].write(n);}
 bool kitAudioParam(int i)const{if(canonical(i)!=i)return kitAudioParam(canonical(i));if(i>=kXQKit&&i<=kXQPerc||i>=kXSoundKick&&i<=kXSoundPerc||i==kXKit||i==kKit||i>=kDrive&&i<=kDuck||i>=kLowCut&&i<=kModDepth||i>=kXSample0&&i<=kXSample7)return true;for(int r=0;r<8;++r)if(i>=kLevel0+r*9&&i<=kChoke0+r*9)return true;return false;}
 void queueAudioUi(){for(int i=0;i<kParamCount;++i)if(kitAudioParam(i)||i>=kXSoundKick&&i<=kXSoundPerc||i>=kSwing&&i<=kDuck||i==kXBeat||i==kXGroove||i==kPhraseState||i==kHatPhraseState)changed[i].store(true);displayPending.store(true);}
 void factoryValues(const groove::Library::FactoryKit& kit,std::vector<float>& values){for(int r=0;r<8;++r){values[kXSample0+r]=kit.sounds[r];int keys[]={kLevel0,kTune0,kDecay0,kPan0,kChoke0};for(int n=0;n<5;++n)values[keys[n]+r*9]=kit.parts[r][n];}int fx[]={kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth};for(int n=0;n<7;++n)values[fx[n]]=kit.fx[n];}
 bool applyFactory(int index){if(index<=0)return true;if(index>int(library.factoryKits.size())){diceResult.store(2);displayPending.store(true);return false;}auto&kit=library.factoryKits[index-1];for(int r=0;r<8;++r){set(kXSample0+r,kit.sounds[r]);missingSample[r]=false;int keys[]={kLevel0,kTune0,kDecay0,kPan0,kChoke0};for(int n=0;n<5;++n)set(keys[n]+r*9,kit.parts[r][n]);}int fx[]={kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth};for(int n=0;n<7;++n)set(fx[n],kit.fx[n]);diceResult.store(0);queueAudioUi();return true;}
 void snapshotUndo(){undoRandom=randomState.load();undoKeep=keep.load();for(int g=0;g<4;++g){undoPartSeed[g]=partSeed[g].load();for(int n=0;n<4;++n)undoOffset[g][n]=partOffset[g][n].load();}int k=0;for(int b=0;b<7;++b){undoNotes[k++]=lengths[b].load();for(auto&n:notes[b]){undoNotes[k++]=n.data.load();undoNotes[k++]=n.gate.load();}}for(int i=0;i<kParamCount;++i)undoValues[i]=getNormalised(i);haveUndo=true;}
 void restoreUndo(){randomState.store(undoRandom);keep.store(undoKeep);for(int g=0;g<4;++g){partSeed[g].store(undoPartSeed[g]);for(int n=0;n<4;++n)partOffset[g][n].store(undoOffset[g][n]);}int k=0;for(int b=0;b<7;++b){lengths[b].store(undoNotes[k++]);for(auto&n:notes[b]){n.data.store(undoNotes[k++]);n.gate.store(undoNotes[k++]);}}internal=true;for(int i=0;i<kParamCount;++i)if(kitAudioParam(i)||i>=kXSoundKick&&i<=kXSoundPerc||i>=kSwing&&i<=kDuck||i==kXBeat||i==kXGroove||i==kPhraseState||i==kHatPhraseState)setNormalised(i,undoValues[i]);internal=false;lastKit=getInt(kKit);factoryDirty.store(false);mirrorNote();queueAudioUi();}
 void roll(){float amt=get(kRandomAmt)*.01f;int scope=getInt(kScope);internal=true;
  diceRolls.fetch_add(1);diceResult.store(0);if(amt<=0){diceResult.store(1);internal=false;displayPending.store(true);return;}
  if(scope>=1&&scope<=4)keep=false;
  if(!getBool(kKitLock)){
   const auto&kitList=kitItems();int count=int(kitList.size());if(!count){diceResult.store(2);}else if(getInt(kKitDiceMode)==0){int old=getInt(kXKit),pick=int(random()%count),next=kitList[pick]+1;if(count>1&&next==old)next=kitList[(pick+1)%count]+1;set(kXKit,next);applyFactory(next);}else{int required=int(random()%4);for(int g=0;g<4;++g)if(g==required||float(random()%100)<amt*100){const auto&list=soundItems(g);int count=int(list.size());if(!count)continue;int pick=int(random()%count);for(int tries=0;tries<count&&soundMatches(g,library.soundPools[g][list[pick]]);++tries)pick=(pick+1)%count;applySound(g,list[pick]+1);}set(kXKit,0);set(kChoke0+2*9,1);set(kChoke0+3*9,1);set(kChoke0,0);set(kChoke0+9,0);}
  }
  bool rhythmLocked=true;for(int g=0;g<4;++g)if(getInt(kRuleKick+g)>0)rhythmLocked=false;
  if((scope==1||scope==4)&&!getBool(kPatternLock)&&!rhythmLocked){int b=4+getInt(kPattern),pick=getInt(kXBeat);bool canMake=true;
   if(pick>0&&pick<=int(library.categories[3].size())){const auto&m=library.midi[library.categories[3][pick-1]];bool partialLock=false;for(int g=0;g<4;++g)partialLock|=getInt(kRuleKick+g)==0;
    if(partialLock&&(m.notes.size()>MaxNotes||m.length>16*PPQ)){canMake=false;diceResult.store(3);}else{clearBank(b);set(kPhraseState,getInt(kPhraseState)&~(1<<(b-4)));set(kHatPhraseState,getInt(kHatPhraseState)&~(15<<(4*(b-4))));lengths[b].store(std::min(15360,m.length));for(int i=0;i<std::min(MaxNotes,int(m.notes.size()));++i){auto e=m.notes[i];if(e.tick<lengths[b].load())notes[b][i].write({e.tick,mpc::clamp(e.note-60,0,7),e.velocity,100,e.gate});}}}
   if(canMake&&makePhrase(b,amt))set(kXBeat,0);
  }

  if(scope==1&&!getBool(kPatternLock)||scope==2||scope==4)for(int g=0;g<4;++g)if(getInt(kRuleKick+g)>0)partSeed[g].store(random());
  if(scope==2||scope==4){float span[]={3,5,3,2};for(int g=0;g<4;++g){int rule=getInt(kRuleKick+g);if(rule==0)continue;float scale=rule==1?.35f:rule==2?.65f:1.f;for(int n=0;n<4;++n){float v=(int(random()%2001)-1000)*.001f*span[n]*amt*scale;if(n==0&&getInt(kXGroove)>0)v=0;partOffset[g][n].store(v);}}}

  if(scope==5||scope==4){set(kDrive,(random()%31)*amt);set(kDust,(random()%36)*amt);set(kRoom,(random()%25)*amt);set(kDuck,(random()%40)*amt);}
  if((scope==3||scope==4)&&recordState.load()!=2)mutate(getInt(kFillSlot),amt);
  internal=false;queueAudioUi();
 }
 bool soundMatches(int g,const groove::Library::SoundChoice&c)const{int count=g==3?4:g==2?2:1;for(int j=0;j<count;++j){int role=g==3?(j==0?7:j+3):g==2?2+j:g;if(c.sounds[j]!=getInt(kXSample0+role))return false;}return true;}
 void syncSoundSelectors(){internal=true;for(int g=0;g<4;++g){if(soundDirty.load()&(1u<<g))continue;float expected=getNormalised(kXSoundKick+g);int index=0,old=int(std::lround(param(kXSoundKick+g).toPlain(expected)));if(old>0&&old<=int(library.soundPools[g].size())&&soundMatches(g,library.soundPools[g][old-1]))continue;for(size_t j=0;j<library.soundPools[g].size();++j)if(soundMatches(g,library.soundPools[g][j])){index=int(j)+1;break;}if(old!=index&&values_[kXSoundKick+g]->compare_exchange_strong(expected,param(kXSoundKick+g).toNormalised(float(index))))changed[kXSoundKick+g].store(true);mirrorLegacy(kXSoundKick+g);}internal=false;}
 void applySound(int g,int index){if(index<=0||index>int(library.soundPools[g].size()))return;auto&c=library.soundPools[g][index-1];internal=true;set(kXKit,0);for(int j=0;j<(g==3?4:g==2?2:1);++j){int role=g==0?0:g==1?1:g==2?2+j:j==0?7:j+3;set(kXSample0+role,c.sounds[j]);missingSample[role]=false;int keys[]={kLevel0,kTune0,kDecay0,kPan0,kChoke0};for(int k=0;k<5;++k)set(keys[k]+role*9,c.parts[j][k]);}internal=false;queueAudioUi();}
 bool admitRole(int role,double beat,bool fixed,bool phrase=false,int grid=4){int rule=getInt(kRuleKick+ruleGroup(role));if(!rule)return true;int pocket[]={4,3,8,1,1,1,1,2},loose[]={5,4,10,1,2,2,1,3},busy[]={6,5,12,2,2,4,2,4};int cap=(rule==1?pocket:rule==2?loose:busy)[role],bar=int(std::floor(beat/4));if(phrase&&(role==2||role==3)&&grid<4){int caps[]={4,12,18,20};cap=role==2?caps[grid]:1;}auto&b=budgets[role];if(b.bar!=bar){b.bar=bar;b.count=b.extras=b.anchors=0;}
  int bit=fixed?(role==0?1:1<<mpc::clamp(int(std::round(std::fmod(beat+4,4.))),0,3)):0;if(bit&&(b.anchors&bit))return false;
  int reserved=role==0?1:role==1?2:0;if(!fixed&&b.extras>=std::max(0,cap-reserved))return false;
  double gap=role==2?(rule==1?.5:rule==2?.25:.125):role==0||role==1?(rule==1?.5:.25):role==7?(rule==1?.5:.25):.5;
  if(phrase&&(role==2||role==3)&&grid<4)gap=grid==0?.85:grid==1?.055:.055;gap=std::max(.025,gap-.03);if(!fixed&&(beat-b.last<gap||((role==2||role==3)&&beat-lastHat<(phrase&&grid<4?.025:.1))))return false;
  ++b.count;if(fixed)b.anchors|=bit;else ++b.extras;b.last=beat;if(role==2||role==3)lastHat=beat;return true;
 }
 static int ruleGroup(int role){return role==0?0:role==1?1:role==2||role==3?2:3;}
 bool anchor(const Note&n)const{int tick=n.tick%(4*PPQ),grid=int(std::round(tick/double(PPQ)));return(n.role==0&&std::abs(tick)<100)||(n.role==1&&(grid==1||grid==3)&&std::abs(tick-grid*PPQ)<120);}
 bool roomFor(int b,const Note&candidate,int skip=-1)const{static const int caps[]={6,5,12,2,2,4,2,4},gap[]={240,240,120,480,480,480,480,240};int count=0;for(int i=0;i<MaxNotes;++i)if(i!=skip&&notes[b][i].valid()){auto n=notes[b][i].read();if(n.role==candidate.role&&n.tick/(4*PPQ)==candidate.tick/(4*PPQ))++count;bool hats=(candidate.role==2||candidate.role==3)&&(n.role==2||n.role==3);if((n.role==candidate.role||hats)&&std::min(std::abs(n.tick-candidate.tick),lengths[b].load()-std::abs(n.tick-candidate.tick))<(hats?120:gap[candidate.role]))return false;}int rule=getInt(kRuleKick+ruleGroup(candidate.role));int pocket[]={4,3,8,1,1,1,1,2},loose[]={5,4,10,1,2,2,1,3};return count<(rule==1?pocket[candidate.role]:rule==2?loose[candidate.role]:caps[candidate.role]);}
 // PhraseState packs three generated flags, three3-bit actual hat grids (4=legacy),
 // and three2-bit whole-bar origins. It is non-automatable and saved with stable parameter IDs.
 bool phraseBank(int b)const{return b>=4&&b<7&&(getInt(kPhraseState)&(1<<(b-4)));}
 int phraseRhythm(int b)const{return((getInt(kHatPhraseState)>>(4*(b-4)))&15)-1;}
 int phraseGrid(int b)const{int rhythm=phraseRhythm(b);if(rhythm>=0){if(rhythm%3)return 3;static const int grids[]={0,1,2,2,2};return grids[rhythm/3];}return std::min(4,(getInt(kPhraseState)>>(3+3*(b-4)))&7);}
 int phraseOrigin(int b)const{return(getInt(kPhraseState)>>(12+2*(b-4)))&3;}
 bool makePhrase(int b,float amount){
  bool locked=false,all=true;for(int g=0;g<4;++g){locked|=getInt(kRuleKick+g)==0;all&=getInt(kRuleKick+g)==0;}if(all)return false;
  // A locked legacy part retains its old loop length and phase. Otherwise construct four bars.
  int oldLength=lengths[b].load(),length=locked?oldLength:16*PPQ,bars=std::max(1,length/(4*PPQ));
  std::array<Note,MaxNotes>next{};int count=0;bool overflow=false;
  auto put=[&](int tick,int role,int velocity,int gate=80){if(tick<0||tick>=length)return;for(int i=0;i<count;++i)if(next[i].tick==tick&&next[i].role==role)return;if(count>=MaxNotes){overflow=true;return;}next[count++]={tick,role,velocity,100,gate};};
  for(auto&cell:notes[b])if(cell.valid()){auto n=cell.read();if(getInt(kRuleKick+ruleGroup(n.role))==0){if(count>=MaxNotes)return false;next[count++]=n;}}
  if(!baseValid[b].load()){baseLengths[b].store(oldLength);for(int i=0;i<MaxNotes;++i){bases[b][i].data.store(notes[b][i].data.load());bases[b][i].gate.store(notes[b][i].gate.load());}baseValid[b].store(true);}
  // Original self-authored16th-note rhythm vocabulary: no kick on steps4/12 (beats2/4).
  static const int motifs[][6]={{0,6,10,-1,-1,-1},{0,7,10,14,-1,-1},{0,3,8,11,-1,-1},{0,6,9,14,-1,-1},{0,7,10,-1,-1,-1},{0,3,6,10,15,-1},{0,2,7,10,-1,-1},{0,3,10,14,-1,-1}};
  int a=random()%8,z=random()%8;int kickRule=getInt(kRuleKick),hatRule=getInt(kRuleHat),snareRule=getInt(kRuleSnare),percRule=getInt(kRulePerc);int rhythm=getInt(kHatRhythm);static const int grids[]={0,1,2,2,2};int grid=rhythm%3?3:grids[rhythm/3];
  // Use an admissible source-MIDI motif as one candidate, never copy an invalid2/4kick.
  bool retainBase=phraseBank(b)&&amount<=.5f;auto& sourceBank=retainBase?notes[b]:bases[b];int source[6]={0,-1,-1,-1,-1,-1},sourceCount=1;for(auto&c:sourceBank)if(c.valid()){auto n=c.read();int step=int(std::lround(n.tick/240.));if(n.role!=0||n.tick>=4*PPQ||step<=0||step>=16||step==4||step==12)continue;bool duplicate=false;for(int j=0;j<sourceCount;++j)duplicate|=source[j]==step;if(!duplicate&&sourceCount<5)source[sourceCount++]=step;}
  bool useSource=sourceCount>=3&&(retainBase||(random()%2)==0);uint32_t touch=random();
  for(int bar=0;bar<bars;++bar){int start=bar*4*PPQ,phraseBar=bar%4;
   if(snareRule){put(start+PPQ,1,100);put(start+3*PPQ,1,100);int ghosts=snareRule==3?2:snareRule==2?1:0;
    for(int j=0;j<ghosts;++j)if(amount>.5f&&random()%100<unsigned(std::min(get(kGhost),snareRule==3?40.f:25.f))){int step=j==0?3:11;put(start+step*240,1,24+int(random()%13),45);}}
   if(kickRule){const int*motif=useSource?source:motifs[a];int hits=0;for(int j=0;j<6&&motif[j]>=0;++j){int step=motif[j];if(hits>=(kickRule==1?4:5))break;
     if(kickRule>=2&&amount>.35f&&phraseBar==3&&step>=8){step=motifs[z][j];if(step<8)continue;}if(step==4||step==12)continue;
     int vel=step==0?105:90+int(hash(touch+step*47)%15);if(phraseBar==1||phraseBar==3)vel-=3;put(start+step*240,0,vel);++hits;}
    if(kickRule>=2&&amount>.55f&&(phraseBar==1||phraseBar==3)&&hits<5){int step=phraseBar==1?14:15;bool space=true;for(int i=0;i<count;++i)if(next[i].role==0&&std::abs(next[i].tick-(start+step*240))<480)space=false;if(space)put(start+step*240,0,88);}}
   if(percRule){int max=percRule==3?2:1;bool decorate=percRule==1?(phraseBar==1||phraseBar==3):phraseBar!=0;
    if(decorate)for(int j=0;j<max;++j){int choices[]={3,7,11,14,15};int step=choices[random()%5],tick=start+step*240;bool space=true;for(int i=0;i<count;++i)if((next[i].role<=1)&&std::abs(next[i].tick-tick)<240)space=false;
     if(space&&random()%100<unsigned((percRule==1?35:percRule==2?55:70)*amount)){int pick=random()%100,role=pick<70?7:pick<90?5:pick<98?4:6;if(role==6&&phraseBar!=3)role=7;put(tick,role,45+int(random()%21),80);}}
   }
  }
  if(hatRule){
   // Dotted notes run continuously across barlines: 1/8D=720ticks, 1/16D=360.
   static const int strides[]={960,480,240,720,360};int stride=strides[rhythm/3],style=rhythm%3;
   for(int tick=0;tick<length;tick+=stride){int bar=tick/(4*PPQ),local=tick%(4*PPQ);bool tail=(bar%4==1||bar%4==3)&&local>=3360;
    if(style==2&&tail)continue; // The short roll owns the last eighth of response bars.
    if(style==1&&tail&&local%960&&hatRule>=2&&amount>.35f&&hash(touch+tick)%4==0)continue;
    int velocity=tick%960==0?78:tick%480==0?58:46;velocity+=int(hash(touch+tick*13)%7)-3;
    put(tick,2,velocity,50);
   }
   for(int bar=1;bar<bars;bar+=2){int start=bar*4*PPQ;
    if(style==2){for(int j=0;j<4;++j)put(start+3360+j*120,2,50+j*4,35);}
    if(style==1){ // Two restrained responses, not a replacement for recorded user fills.
     int picks[]={3600,3120,3720};int hits=bar%4==1?1:3;
     for(int j=0;j<hits;++j){int tick=picks[j];if((start+tick)%stride==0)tick+=120;if(tick<3840)put(start+tick,2,48+int(hash(touch+tick+bar)%12),45);}
    }
   }
  }
  if(overflow){diceResult.store(3);return false;}
  // Fixed arrays and bounded insertion sort: no allocation/locking in the audio callback.
  for(int i=1;i<count;++i){Note n=next[i];int j=i;while(j>0&&(next[j-1].tick>n.tick||(next[j-1].tick==n.tick&&next[j-1].role>n.role))){next[j]=next[j-1];--j;}next[j]=n;}
  int old=getInt(kPhraseState),index=b-4,actualGrid=hatRule==0?(phraseBank(b)?phraseGrid(b):4):grid;
  int origin=locked?(phraseBank(b)?phraseOrigin(b):0):((int(std::floor(position.load()/4))%4+4)%4);
  int state=(old|(1<<index));state=(state&~(7<<(3+index*3)))|(actualGrid<<(3+index*3));state=(state&~(3<<(12+index*2)))|(origin<<(12+index*2));
  int hatState=getInt(kHatPhraseState);if(hatRule)hatState=(hatState&~(15<<(index*4)))|((rhythm+1)<<(index*4));
  for(auto&c:notes[b])c.data.store(0);lengths[b].store(length);for(int i=0;i<count;++i)notes[b][i].write(next[i]);set(kHatPhraseState,hatState);changed[kHatPhraseState].store(true);set(kPhraseState,state);changed[kPhraseState].store(true);return true;
 }
 void mutate(int b,float amount){
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
 void saveKit(){int k=getInt(kUserKit);for(int r=0;r<8;++r){savedLibraryKits[k*8+r].store(getInt(kXSample0+r));orphanSaved[k*8+r].store(orphanSample[r].load());}int i=k*72;for(int r=0;r<8;++r)for(int p:{kSample0,kLevel0,kTune0,kDecay0,kPan0,kChoke0,kRoute0,kPadNote0})savedKits[i++].store(get(p+r*9));for(int p:{kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth,kMaster})savedKits[i++].store(get(p));kitValid[k].store(true);}
 void loadKit(){int k=getInt(kUserKit);if(!kitValid[k].load())return;int i=k*72;internal=true;set(kXKit,0);for(int r=0;r<8;++r){int id=savedLibraryKits[k*8+r].load();if(getInt(kXSample0+r)!=std::max(0,id))set(kXSample0+r,std::max(0,id));if(id<0)missingSample[r]=true;}for(int r=0;r<8;++r)for(int p:{kSample0,kLevel0,kTune0,kDecay0,kPan0,kChoke0,kRoute0,kPadNote0}){float v=savedKits[i++].load();if(p==kSample0){missingSample[r]=savedLibraryKits[k*8+r].load()<0||(savedLibraryKits[k*8+r].load()==0&&v<0);if(v>=0)set(p+r*9,v);}else set(p+r*9,v);}for(int p:{kDrive,kDust,kRoom,kDuck,kLowCut,kModRate,kModDepth,kMaster})set(p,savedKits[i++].load());for(int r=0;r<8;++r)if(savedLibraryKits[k*8+r].load()<=0&&orphanSaved[k*8+r].load()>0){orphanSample[r].store(orphanSaved[k*8+r].load());missingSample[r]=true;}internal=false;queueAudioUi();}
 void generateSound(int sample){auto& s=sounds[sample];s.sampleRate=44100;s.channels.resize(1);auto& x=s.channels[0];int role=sample/8;x.resize(role==3?22050:11025);uint32_t seed=123+sample;double phase=0;for(size_t i=0;i<x.size();++i){seed=1664525*seed+1013904223;float noise=float(int32_t(seed))/2147483648.f;double t=i/44100.;phase+=6.283185307*(role==0?45+90*std::exp(-t*60):180)/44100.;x[i]=float((role==0?std::sin(phase):role==1?.7*noise+.3*std::sin(phase):noise)*std::exp(-t*(role==3?12:role==0?20:role==1?35:90))*.6);}}
};
#ifndef GROOVE_DICE_TEST
static VstIntPtr grooveDispatch(AEffect*e,VstInt32 op,VstInt32 index,VstIntPtr value,void*ptr,float opt){auto*p=static_cast<GrooveDice*>(e->object);if(op==effOpen){auto result=mpc::detail::dispatcher(e,op,index,value,ptr,opt);p->requestHostIdle();return result;}if(op==effSetProgram){p->selectFactoryProgram(int(value));return 0;}if(op==effGetProgram)return p->activeFactoryProgram();if(op==effGetProgramName){mpc::copyString(static_cast<char*>(ptr),kVstMaxProgNameLen+1,p->programName(p->activeFactoryProgram()));return 0;}if(op==effSetChunk)return p->internalLoadState(static_cast<const uint8_t*>(ptr),size_t(value))?1:0;if(op==effIdle_Deprecated||op==effEditIdle){p->flushHostUpdates();return 1;}if(op==effGetParamName&&index>=kQ1&&index<=kQ16){p->flushFromValueRead();int t=p->proxyTarget(index-kQ1);mpc::copyString(static_cast<char*>(ptr),64,t>=0?p->param(t).name.c_str():"Unused");return 0;}if(op==effGetParamDisplay||op==effGetParamName)p->flushFromValueRead();return mpc::detail::dispatcher(e,op,index,value,ptr,opt);}
static float grooveGet(AEffect*e,VstInt32 index){static_cast<GrooveDice*>(e->object)->flushFromValueRead();return mpc::detail::getParameter(e,index);}
extern "C" MPC_EXPORT AEffect* VSTPluginMain(audioMasterCallback host){if(host&&host(nullptr,audioMasterVersion,0,0,nullptr,0)==0)return nullptr;auto*e=mpc::detail::createEffect(new GrooveDice(),host);if(e){e->dispatcher=&grooveDispatch;e->getParameter=&grooveGet;}return e;}
#if defined(__APPLE__)
extern "C" MPC_EXPORT AEffect* main_macho(audioMasterCallback host){return VSTPluginMain(host);}
#endif
#endif
