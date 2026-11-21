// Felt Club by kim — original modal physical keys, no recorded samples.
// MIDI/touch owners -> finite strum queue -> damped resonant modes -> ADSR ->
// stereo pickup/radiation -> motion, drive, chorus, tempo delay, diffuse room.
#include "mpc_plugin.h"
#include "../../../../shared/ui/PadCommands.h"
#include "Parameters.h"
#include "PhysicalVoice.h"
#include "Colour.h"
#include "FactoryNames.h"
#include "FactorySounds.h"
#include "SoundProfiles.h"
#include "ChordVoicing.h"
#include "SmoothVoicing.h"
#include <array>
#include <atomic>
#include <algorithm>
#include <cmath>
#include <cstring>
#include <cstdio>
#include <string>
#include <chrono>

class FeltClub final:public mpc::Plugin {
public:
 #ifdef FELT_PAD_DIAGNOSTICS
 std::atomic<unsigned>rawPadOn{0},rawPadOff{0},renderPadOn{0},renderPadOff{0},midiDiceCount{0};std::atomic<int>rawPadNote{-1};
#endif
 static constexpr int VoiceCount=24,SoundCount=int(sizeof(kSoundKeys)/sizeof(int));
 static constexpr int rootKey(int slot,int field=0){return kRoot1+slot*5+field;}
 static constexpr int editKey(int row,int field){return kEdit0+row*7+field;}
 static constexpr int holdKey(int row){return kHold1+row*7;}
 static constexpr int learnKey(int row){return kLearn1+row*7;}
 static constexpr int chordNameKey(int row){return kChordName1+row*7;}
 static int editIndex(int key){int d=key-kEdit0;return d>=0&&d/7<4&&d%7<4?d/7*4+d%7:-1;}
 static int holdIndex(int key){int d=key-kHold1;return d>=0&&d/7<4&&d%7==0?d/7:-1;}
 static int learnIndex(int key){int d=key-kLearn1;return d>=0&&d/7<4&&d%7==0?d/7:-1;}
 static bool command(int key){return key==kRoll||key==kUndo||key==kUnlockAll||key==kPanic||key==kPresetPrev||key==kPresetNext||(key>=kPagePrev&&key<=kChordNext)||learnIndex(key)>=0;}
 static bool transient(int key){return key==kCommandLearn||command(key)||holdIndex(key)>=0||key==kChordEditing;}
 static int family(int model){return model<2?0:model<4?1:2;}
 static float division(int index){static const float base[]={4,2,1,.5f,.25f,.125f};index=std::clamp(index,0,17);return base[index/3]*(index%3==1?1.5f:index%3==2?2.f/3:1.f);}
 struct Mutation {FeltClub*before;explicit Mutation(FeltClub*p):before(mutating){mutating=p;}~Mutation(){mutating=before;}};
 static inline thread_local FeltClub*mutating=nullptr;
 struct Program {std::array<float,SoundCount> sound{};};
 struct Pending {uint64_t due=0;int owner=-1,note=60,velocity=100,voice=-1;bool active=false;};
 std::array<felt::Voice,VoiceCount> voices{};
 std::array<Pending,192> pending{};
 uint64_t nextPendingDue=~uint64_t(0);
 std::array<std::array<int,128>,16> midiOwners{};
 std::array<std::array<uint8_t,128>,16> midiDown{},midiVelocity{};
 std::array<std::array<int8_t,128>,16>midiSlots{};
 std::array<uint64_t,16>chordConfig{};
 std::array<bool,16> pedalDown{};
 std::array<float,16>bend{};
 std::array<bool,4>touchHeld{};std::array<int,4>touchSlots{};
 std::array<std::atomic<bool>,kParamCount>changed{},requests{},seen{};
 std::array<std::atomic<float>,16>relayCursor{};
 std::array<std::atomic<int>,16>relayPage{};
 std::array<Program,felt::kProgramCount>programs{};
 kimswork::PadCommands padCommands;
 felt::Colour colour;felt::AcousticBodies body;
 std::atomic_flag servicing=ATOMIC_FLAG_INIT,flushing=ATOMIC_FLAG_INIT;
 std::atomic<bool>dirtyAll{true},restoring{false},clearAudio{false};
 std::atomic<int64_t>loadDeadline{0};
 std::atomic<int>learnSlot{-1},learnConflict{0},loadGuard{0};
 std::atomic<uint32_t> randomState{0x46434c42};
 std::array<float,SoundCount>undoSound{};int undoPreset=0;uint32_t undoRandom=0;bool haveUndo=false;
 std::vector<uint8_t>savedState;
 float sampleRate=44100,gain=.35f;uint64_t clock=0;uint32_t serial=0,strumSerial=0;int lastChordOwner=-1,lastPage=0,lastSlots=4;double tempo=98;bool prepared=false;
 felt::VoiceSettings lastSettings{};bool haveSettings=false;
 // A host may reuse one thread for rendering and later UI/parameter polling.
 // Suppress callbacks only while inside an actual render call, not forever
 // after that thread has rendered once. Preserve nested-call ownership.
 static inline thread_local FeltClub*rendering=nullptr;
 struct RenderScope {FeltClub*previous;explicit RenderScope(FeltClub*p):previous(rendering){rendering=p;}~RenderScope(){rendering=previous;}};
 bool onRender()const{return rendering==this;}
 static mpc::PluginInfo info(){mpc::PluginInfo x;x.name="Felt Club";x.vendor="kim";x.uniqueId=MPC_FOURCC('K','n','A','w');x.version=302;x.numInputs=0;x.numOutputs=2;x.isInstrument=true;return x;}
 FeltClub():mpc::Plugin(info()){
  declareParameters(*this);for(auto&p:relayPage)p.store(-1);for(auto&v:bend)v=1;
  for(int i=0;i<felt::kProgramCount;++i){std::vector<float>v(numParams());for(int k=0;k<numParams();++k)v[k]=param(k).def;
   felt::applyFactorySound(i,v);v[kFamily]=family(int(v[kModel]));
   v[kPreset]=i+1;for(int j=0;j<SoundCount;++j)programs[i].sound[j]=v[kSoundKeys[j]];
   std::string name=i?(family(int(v[kModel]))==0?"EP / ":family(int(v[kModel]))==1?"Piano / ":"Pluck / ")+std::string(felt::kProgramNames[i]):"Init";addProgram(name.c_str(),v);
  }
  applyProgram(0);migrateDefaultChordMap();dirtyAll.store(true);
 }
 void migrateDefaultChordMap(){
        // Pad commands are removed. Keep saved/learned chord note maps intact.

 }
 int slots()const{return (getInt(kSlots)+1)*4;}
 int slotForRow(int row)const{return getInt(kPage)*4+row;}
 int realEditTarget(int pos)const{return rootKey(slotForRow(pos/4),pos%4);}
 float actualEdit(int pos)const{int target=realEditTarget(pos);if(pos%4==0)return float((getInt(target)+getInt(kKey))%12)/11;return param(target).toNormalised(get(target));}
 float readParameter(int key)const{
  if(key>=kMainQ0&&key<=kMainQ15)return getNormalised(kMainKeys[key-kMainQ0]);
  if(int pos=editIndex(key);pos>=0){float cursor=relayCursor[pos].load(),actual=actualEdit(pos);int target=realEditTarget(pos);float a=pos%4==0?std::round(actual*11):param(target).toPlain(actual);float b=pos%4==0?std::round(cursor*11):param(target).toPlain(cursor);return relayPage[pos].load()==getInt(kPage)&&a==b?cursor:actual;}
  if(key==kFamily)return family(getInt(kModel))*.5f;return getNormalised(key);
 }
 void parameterChanged(int key,float plain)override{
  changed[key].store(true);if(mutating==this||restoring.load())return;
  if(loadGuard.load()>0){if(transient(key)){Mutation m(this);set(key,0);seen[key].store(false);requests[key].store(false);}return;}
  if(key>=kMainQ0&&key<=kMainQ15){if(!getBool(kChordEditing))setNormalised(kMainKeys[key-kMainQ0],plain);return;}
  if(int pos=editIndex(key);pos>=0){Mutation m(this);relayCursor[pos].store(plain);relayPage[pos].store(getInt(kPage));int target=realEditTarget(pos);if(pos%4==0)set(target,(int(std::lround(plain*11))-getInt(kKey)+12)%12);else{float v=param(target).toPlain(plain);if(pos%4==2)v=std::min(v,float(felt::chordToneCount(getInt(target-1))));set(target,v);}dirtyAll.store(true);return;}
  if(key==kFamily){Mutation m(this);int f=mpc::clamp(int(std::lround(plain)),0,2);if(family(getInt(kModel))!=f)set(kModel,f==0?0:f==1?2:4);set(kFamily,f);set(kPreset,0);dirtyAll.store(true);return;}
  if(command(key)){bool value=plain>=.5f;if(seen[key].exchange(value)!=value&&!loadGuard.load())requests[key].store(true);return;}
  if(key==kPreset){if(!loadGuard.load())requests[kPreset].store(true);return;}
  if(key==kPage||key==kSlots){Mutation m(this);int page=std::min(getInt(kPage),getInt(kSlots));set(kPage,page);for(int row=0;row<4;++row)set(holdKey(row),0);learnSlot.store(-1);dirtyAll.store(true);return;}
  if(key==kModel){Mutation m(this);set(kFamily,family(getInt(kModel)));set(kPreset,0);dirtyAll.store(true);}
  for(int i=0;i<4;++i)if(key==kChordPreset1+i){requests[key].store(true);return;}
  if(key>=kRoot1&&key<=kPadNote16){int offset=(key-kRoot1)%5;if(offset==1){Mutation m(this);int voicing=key+1;if(getInt(voicing)>felt::chordToneCount(getInt(key)))set(voicing,felt::chordToneCount(getInt(key)));}dirtyAll.store(true);}
  if(key==kKey||key==kTranspose||key==kScale)dirtyAll.store(true);
  for(int t:kSoundKeys)if(t==key){Mutation m(this);set(kPreset,0);break;}
 }
 uint32_t random(){uint32_t x=randomState.load();x^=x<<13;x^=x>>17;x^=x<<5;randomState.store(x);return x;}
 float between(float lo,float hi){return lo+(hi-lo)*float(random()>>8)/16777215.f;}
 bool isLocked(int key)const{for(int i=0;i<SoundCount;++i)if(kSoundKeys[i]==key)return getBool(kSoundLocks[i]);return false;}
 void applyProgram(int i){if(i<0||i>=felt::kProgramCount)return;Mutation m(this);for(int j=0;j<SoundCount;++j)set(kSoundKeys[j],programs[i].sound[j]);set(kPreset,i+1);set(kFamily,family(getInt(kModel)));dirtyAll.store(true);}
 void dice(){Mutation m(this);undoRandom=randomState.load();undoPreset=getInt(kPreset);for(int i=0;i<SoundCount;++i)undoSound[i]=get(kSoundKeys[i]);haveUndo=true;
  const int model=getInt(kModel);
  // These ranges vary the instrument around its voiced physical response.
  // Model and touch curve are deliberate playing choices.
  using Range=felt::SoundRange;
  const auto& profile=felt::kDiceProfiles[model];
  const auto draw=[&](Range range){return between(range.low,range.high);};
  const float body=isLocked(kBody)?get(kBody):draw(profile.body);
  const float fullness=mpc::clamp((body-profile.body.low)/(profile.body.high-profile.body.low),0.f,1.f);
  const float releaseTop=profile.release.high-(profile.release.high-profile.release.low)*.55f*fullness;
  // One main color effect at a time, with existing locked effects included.
  const bool lockedColor=(isLocked(kDrive)&&get(kDrive)>0)||(isLocked(kChorus)&&get(kChorus)>0)||
                         (isLocked(kDelay)&&get(kDelay)>0)||(isLocked(kRoom)&&get(kRoom)>0);
  const int effect=lockedColor?0:int(random()%5);
  for(int key:kSoundKeys){if(key==kModel||key==kSync||key==kVelocityCurve||isLocked(key))continue;float v=param(key).def;
   switch(key){
    case kTone:v=draw(profile.tone);break;
    case kBody:v=body;break;
    case kCharacter3:v=draw(profile.character3);break;
    case kCharacter4:v=draw(profile.character4);break;
    case kAttack:v=between(1,model==3?4:2);break;
    case kDecay:v=draw(profile.decay);break;
    case kSustain:v=between(92,98);break;
    case kRelease:v=between(profile.release.low,releaseTop);break;
    case kRate:{constexpr int divisions[]={6,7,8,9,10,11,12};
     v=getBool(kSync)?divisions[random()%7]/17.f:between(.30f,.65f);break;}
    case kDepth:v=random()%3==0?between(8,profile.motion):0;break;
    case kDrift:v=between(0,profile.drift);break;
    case kWidth:v=between(65,100);break;
    case kDrive:v=effect==1?between(2,model==2||model==3?4:7):0;break;
    case kChorus:v=effect==2?between(4,model==2?10:16):0;break;
    case kDelay:v=effect==3?between(10,model==5?24:20):0;break;
    case kRoom:v=effect==4?between(10,24):0;break;
    case kMotionMode:v=random()%2;break;
    case kShape:v=random()%4==0?1:0;break;
    case kDelayDivision:{constexpr int divisions[]={6,9,10};v=divisions[random()%3];break;}
    case kDelayFeedback:v=between(15,28);break;
    case kRoomTime:v=between(30,48);break;
    case kRoomDamp:v=between(58,72);break;
    case kHammer:v=draw(profile.hammer);break;
    case kVelocity:v=between(60,75);break;
    case kResonance:v=between(24,42);break;
    case kNoise:case kKeyOff:v=0;break;
    case kHiss:v=random()%4==0?between(5,18):0;break;
    case kPick:v=between(20,30);break;
    case kDetune:v=draw(profile.detune);break;
    case kMotionDelay:v=0;break;
    case kMotionFade:v=between(0,180);break;
    case kDriftRate:v=between(.12f,.30f);break;
    case kChorusRate:v=between(.18f,.35f);break;
    case kChorusDepth:v=between(25,45);break;
    default:break;
   }
   set(key,v);
  }set(kPreset,0);set(kFamily,family(getInt(kModel)));dirtyAll.store(true);
 }
 void undo(){if(!haveUndo)return;Mutation m(this);for(int i=0;i<SoundCount;++i)set(kSoundKeys[i],undoSound[i]);set(kPreset,undoPreset);set(kFamily,family(getInt(kModel)));randomState.store(undoRandom);dirtyAll.store(true);}
 void progression(int page,int which){
  static constexpr int roots[16][4]={{2,7,0,9},{9,2,7,0},{0,9,2,7},{5,4,9,2},{2,7,4,9},{0,5,6,7},{9,0,5,7},{2,5,4,9},{0,3,2,7},{9,5,2,4},{5,6,4,9},{0,4,5,2},{2,4,9,7},{0,7,9,5},{9,2,11,4},{5,4,2,7}};
  static constexpr int types[16][4]={{2,6,3,2},{2,13,6,3},{3,13,15,6},{3,5,2,13},{15,24,2,13},{14,3,26,6},{13,3,17,6},{15,3,5,13},{3,26,2,5},{2,17,13,24},{14,26,5,2},{17,2,14,15},{13,5,2,6},{3,21,13,14},{15,13,25,24},{14,5,15,6}};
  Mutation m(this);which=mpc::clamp(which,0,15);for(int n=0;n<4;++n){int slot=page*4+n;set(rootKey(slot),roots[which][n]);set(rootKey(slot,1),types[which][n]);if(getInt(rootKey(slot,2))>felt::chordToneCount(types[which][n]))set(rootKey(slot,2),felt::chordToneCount(types[which][n]));}dirtyAll.store(true);
 }
 void smoothTop(){felt::SmoothInput input;input.start=getInt(kPage)*4;input.active=slots();input.count=4;for(int n=0;n<16;++n)input.chords[n]={48+getInt(rootKey(n))+getInt(kKey),getInt(rootKey(n,1)),getInt(rootKey(n,2)),getInt(rootKey(n,3))+2};auto plan=felt::findSmoothVoicing(input);if(!plan.valid)return;Mutation m(this);for(int n=0;n<4;++n){set(rootKey(input.start+n,2),plan.voice[n]);set(rootKey(input.start+n,3),plan.octave[n]-2);}dirtyAll.store(true);}
 void service(bool ui){
  int oldPage=getInt(kPage),oldSlots=slots();
  if(restoring.load()||servicing.test_and_set(std::memory_order_acquire))return;
  auto take=[&](int k){return requests[k].exchange(false);};
  if(!ui&&loadGuard.load()>0)padCommands.reset();
  if(take(kPreset))applyProgram(getInt(kPreset)-1);
  if(take(kPresetPrev))applyProgram((std::max(0,getInt(kPreset)-1)+felt::kProgramCount-1)%felt::kProgramCount);
  if(take(kPresetNext))applyProgram((std::max(0,getInt(kPreset)-1)+1)%felt::kProgramCount);
  if(take(kRoll))dice();if(take(kUndo))undo();if(take(kUnlockAll)){Mutation m(this);for(int k:kSoundLocks)set(k,0);dirtyAll.store(true);}
  if(take(kPanic))clearAudio.store(true);
  {Mutation m(this);if(take(kPagePrev))set(kPage,std::max(0,getInt(kPage)-1));if(take(kPageNext))set(kPage,std::min(getInt(kSlots),getInt(kPage)+1));if(take(kSlotsAdd))set(kSlots,std::min(3,getInt(kSlots)+1));if(take(kSlotsRemove)){set(kSlots,std::max(0,getInt(kSlots)-1));set(kPage,std::min(getInt(kSlots),getInt(kPage)));}}
  for(int page=0;page<4;++page)if(take(kChordPreset1+page))progression(page,getInt(kChordPreset1+page));
  bool previousChord=take(kChordPrev),nextChord=take(kChordNext);if(previousChord||nextChord){bool next=nextChord;int key=kChordPreset1+getInt(kPage);Mutation m(this);set(key,(getInt(key)+(next?1:15))%16);progression(getInt(kPage),getInt(key));}
  for(int row=0;row<4;++row)if(take(learnKey(row))){int slot=slotForRow(row);learnSlot.store(learnSlot.load()==slot?-1:slot);learnConflict.store(0);dirtyAll.store(true);}
  // The bounded voicing search is deliberately never run by the render thread.
  if(ui&&take(kSmooth))smoothTop();
  if(oldPage!=getInt(kPage)||oldSlots!=slots()){Mutation m(this);for(int row=0;row<4;++row)set(holdKey(row),0);dirtyAll.store(true);}
  servicing.clear(std::memory_order_release);
 }
 void expireLoadGuard(){if(loadGuard.load()>0&&!onRender()&&std::chrono::steady_clock::now().time_since_epoch().count()>=loadDeadline.load())loadGuard.store(0);}
 void flush(){expireLoadGuard();if(onRender()||restoring.load()||!effect_||flushing.test_and_set())return;service(true);bool all=dirtyAll.exchange(false);bool display=all;for(int k=0;k<kParamCount;++k)if(changed[k].exchange(false)||all){if(hostCallback())hostCallback()(effect_,audioMasterAutomate,k,0,nullptr,readParameter(k));display=true;}if(display&&hostCallback())hostCallback()(effect_,audioMasterUpdateDisplay,0,0,nullptr,0);flushing.clear();}
 auto hostCallback()const->audioMasterCallback{return host_;}
 void prepare(double sr,int)override{sampleRate=float(sr);for(auto&v:voices)v.prepare(sampleRate);colour.prepare(sampleRate);body.prepare(sampleRate);prepared=true;reset();}
 void reset()override{
  padCommands.reset();
  for(auto&v:voices)v.stop();for(auto&e:pending)e.active=false;nextPendingDue=~uint64_t(0);for(auto&ch:midiOwners)ch.fill(-1);for(auto&ch:midiDown)ch.fill(0);for(auto&ch:midiSlots)ch.fill(-1);chordConfig.fill(0);pedalDown.fill(false);bend.fill(1);touchHeld.fill(false);touchSlots.fill(-1);clock=0;serial=0;lastChordOwner=-1;lastPage=getInt(kPage);lastSlots=slots();learnSlot.store(-1);learnConflict.store(0);haveSettings=false;
  Mutation m(this);for(int k=0;k<kParamCount;++k)if(transient(k)){set(k,0);seen[k].store(false);requests[k].store(false);}requests[kPreset].store(false);clearAudio.store(false);if(prepared){colour.reset();body.reset();}gain=std::pow(10.f,get(kMaster)/20);dirtyAll.store(true);
 }
 felt::VoiceSettings settings()const{felt::VoiceSettings s;s.model=getInt(kModel);s.tone=get(kTone)*.01f;s.body=get(kBody)*.01f;s.character3=get(kCharacter3)*.01f;s.character4=get(kCharacter4)*.01f;s.attack=get(kAttack);s.decay=get(kDecay);s.sustain=get(kSustain)*.01f;s.release=get(kRelease);s.hammer=get(kHammer)*.01f;s.velocity=get(kVelocity)*.01f;s.resonance=get(kResonance)*.01f;s.pick=get(kPick)*.01f;s.detune=get(kDetune);s.noise=get(kNoise)*.01f;return s;}
 int sourceChannel(int owner)const{return owner>=8192&&owner<10240?(owner-8192)/128:owner>=0&&owner<2048?owner/128:0;}
 bool ownerDown(int owner)const{for(const auto&v:voices)if(v.state().active&&v.state().down&&v.state().owner==owner)return true;return false;}
 void releaseOwner(int owner,bool pedal=false){for(auto&e:pending)if(e.active&&e.owner==owner)e.active=false;for(auto&v:voices)if(v.state().active&&v.state().owner==owner){v.release(pedal);v.keyOff(get(kKeyOff)*.01f);}}
 int allocate(){int best=0;for(int i=0;i<VoiceCount;++i){if(!voices[i].state().active)return i;if((!voices[i].state().down&&voices[best].state().down)||(!voices[i].state().down&&!voices[best].state().down&&voices[i].state().energy<voices[best].state().energy)||(voices[i].state().down&&voices[best].state().down&&voices[i].state().age<voices[best].state().age))best=i;}return best;}
 int velocity(int v)const{float x=mpc::clamp(v/127.f,.001f,1.f);if(getInt(kVelocityCurve)==0)x=.5f*x+.5f*std::sqrt(x);if(getInt(kVelocityCurve)==2)x=x*x;return mpc::clamp(int(x*127),1,127);}
 void strike(int note,int vel,int owner){int i=allocate();voices[i].strike(note,velocity(vel),owner,int(++serial),settings(),get(kTuning));}
 void chord(int slot,int owner,int vel){
  std::array<int,6>notes;int count=felt::chordNotes(48+getInt(rootKey(slot))+getInt(kKey)+getInt(kTranspose),getInt(rootKey(slot,1)),getInt(rootKey(slot,2)),getInt(rootKey(slot,3)),notes);
  count=int(std::unique(notes.begin(),notes.begin()+count)-notes.begin());bool connect=getBool(kChordGlide)&&get(kGlide)>0&&lastChordOwner!=owner&&ownerDown(lastChordOwner);std::array<bool,VoiceCount>used{};
  int direction=getInt(kStrumDirection);bool down=direction==1||(direction==2&&strumSerial%2);++strumSerial;
  for(int j=0;j<count;++j){int n=down?count-1-j:j,voice=-1;if(connect){int distance=129;for(int v=0;v<VoiceCount;++v)if(!used[v]&&voices[v].state().active&&voices[v].state().down&&voices[v].state().owner==lastChordOwner&&std::abs(voices[v].state().note-notes[n])<distance){voice=v;distance=std::abs(voices[v].state().note-notes[n]);}if(voice>=0){used[voice]=true;voices[voice].state().owner=owner;}}
   uint32_t h=uint32_t(serial+slot*31+n*131);h^=h<<13;h^=h>>17;int spread=int(get(kStrumVelocity));int vv=mpc::clamp(vel+(int(h%201)-100)*spread/100,1,127);uint64_t due=clock+uint64_t(get(kStrum)*.001*sampleRate*j/std::max(1,count-1));
   bool queued=false;for(auto&e:pending)if(!e.active){e={due,owner,notes[n],vv,voice,true};nextPendingDue=std::min(nextPendingDue,due);queued=true;break;}if(!queued&&voice>=0)voices[voice].release(false);
  }
  if(connect)releaseOwner(lastChordOwner,false);lastChordOwner=owner;
 }
 void midi(const mpc::MidiEvent&e){int channel=e.channel();
#ifdef FELT_PAD_DIAGNOSTICS
  if(e.isNoteOn())renderPadOn.fetch_add(1);if(e.isNoteOff())renderPadOff.fetch_add(1);
#endif
  if(e.isAllNotesOff()){for(auto&v:voices)if(sourceChannel(v.state().owner)==channel){if(e.controller()==120)v.stop();else v.release(false);}for(auto&p:pending)if(sourceChannel(p.owner)==channel)p.active=false;midiDown[channel].fill(0);midiOwners[channel].fill(-1);midiSlots[channel].fill(-1);pedalDown[channel]=false;if(channel==0){Mutation m(this);for(int row=0;row<4;++row){set(holdKey(row),0);touchHeld[row]=false;}}return;}
  if(e.isController()&&e.controller()==121){pedalDown[channel]=false;bend[channel]=1;for(auto&v:voices)if(v.state().sustained&&sourceChannel(v.state().owner)==channel)v.release(false);return;}
  if(e.isController()&&e.controller()==64){bool old=pedalDown[channel];pedalDown[channel]=getBool(kPedal)&&e.controllerValue()>=64;if(old&&!pedalDown[channel])for(auto&v:voices)if(v.state().sustained&&sourceChannel(v.state().owner)==channel)v.release(false);return;}
  if(e.isPitchBend()){bend[channel]=std::exp2(float(e.pitchBend())/8192*get(kPitchRange)/12);return;}
  if(e.isNoteOn()){
   if(learnSlot.load()>=0){int slot=learnSlot.load();for(int j=0;j<slots();++j)if(j!=slot&&getInt(rootKey(j,4))==e.note()){learnConflict.store(1);dirtyAll.store(true);return;}Mutation m(this);set(rootKey(slot,4),e.note());learnSlot.store(-1);learnConflict.store(0);dirtyAll.store(true);return;}
   int slot=-1;if(getInt(kInputMode)!=0)for(int n=0;n<slots();++n)if(getInt(rootKey(n,4))==e.note()){slot=n;break;}
   if(slot<0&&getInt(kInputMode)==1)return;int owner=slot>=0?8192+channel*128+e.note():channel*128+e.note();releaseOwner(owner,false);midiOwners[channel][e.note()]=owner;midiDown[channel][e.note()]=1;midiSlots[channel][e.note()]=int8_t(slot);midiVelocity[channel][e.note()]=e.velocity();
   if(slot>=0)chord(slot,owner,e.velocity());else strike(mpc::clamp(e.note()+getInt(kTranspose),0,127),e.velocity(),owner);
  }else if(e.isNoteOff()){int owner=midiOwners[channel][e.note()];if(owner>=0){releaseOwner(owner,(owner<2048||owner>=8192)&&pedalDown[channel]&&getBool(kPedal));midiOwners[channel][e.note()]=-1;midiDown[channel][e.note()]=0;}}
 }
 void process(mpc::ProcessContext&ctx)override{
  RenderScope renderScope(this);service(false);if(clearAudio.exchange(false)){for(auto&v:voices)v.stop();for(auto&e:pending)e.active=false;nextPendingDue=~uint64_t(0);colour.reset();body.reset();midiOwners={};for(auto&ch:midiOwners)ch.fill(-1);pedalDown.fill(false);touchHeld.fill(false);}
  if(loadGuard.load()>0)loadGuard.store(std::max(0,loadGuard.load()-ctx.numFrames));if(loadGuard.load()==0)migrateDefaultChordMap();
  if(ctx.transport.tempoValid&&ctx.transport.bpm>=30&&ctx.transport.bpm<=300)tempo=ctx.transport.bpm;
  int page=getInt(kPage),active=slots();if(page!=lastPage||active!=lastSlots){for(int row=0;row<4;++row){if(touchHeld[row])releaseOwner(6000+touchSlots[row]);touchHeld[row]=false;}for(int ch=0;ch<16;++ch)for(int note=0;note<128;++note)if(midiSlots[ch][note]>=active){releaseOwner(midiOwners[ch][note],false);midiSlots[ch][note]=-1;midiOwners[ch][note]=-1;midiDown[ch][note]=0;}lastPage=page;lastSlots=active;}
  for(int slot=0;slot<active;++slot){uint64_t config=1;for(int field=0;field<4;++field)config=config*137+uint64_t(getInt(rootKey(slot,field))+3);config=config*137+getInt(kKey);config=config*137+getInt(kTranspose)+12;
   if(chordConfig[slot]&&chordConfig[slot]!=config){for(int row=0;row<4;++row)if(touchHeld[row]&&touchSlots[row]==slot){releaseOwner(6000+slot);chord(slot,6000+slot,100);}for(int ch=0;ch<16;++ch)for(int note=0;note<128;++note)if(midiDown[ch][note]&&midiSlots[ch][note]==slot){int owner=midiOwners[ch][note];releaseOwner(owner);chord(slot,owner,midiVelocity[ch][note]);}}
   chordConfig[slot]=config;
  }
  for(int row=0;row<4;++row){bool down=getBool(holdKey(row));if(down!=touchHeld[row]){int slot=slotForRow(row);if(down&&slot<active){touchSlots[row]=slot;chord(slot,6000+slot,100);}else if(touchSlots[row]>=0)releaseOwner(6000+touchSlots[row]);touchHeld[row]=down;}}
  if(!getBool(kPedal))for(int ch=0;ch<16;++ch)if(pedalDown[ch]){pedalDown[ch]=false;for(auto&v:voices)if(v.state().sustained&&sourceChannel(v.state().owner)==ch)v.release(false);}
  auto s=settings();bool envelope=!haveSettings||s.model!=lastSettings.model||s.attack!=lastSettings.attack||s.decay!=lastSettings.decay||s.sustain!=lastSettings.sustain||s.release!=lastSettings.release;bool tone=!haveSettings||s.model!=lastSettings.model||s.tone!=lastSettings.tone;for(auto&v:voices)if(v.state().active&&v.model()==s.model){if(envelope)v.setEnvelope(s.attack,s.decay,s.sustain,s.release);if(tone)v.setTone(s.tone);}lastSettings=s;haveSettings=true;
  felt::ColourSettings fx;fx.hiss=get(kHiss)*.01f;fx.drive=get(kDrive)*.01f;fx.chorus=get(kChorus)*.01f;fx.delay=get(kDelay)*.01f;fx.room=get(kRoom)*.01f;fx.width=get(kWidth)*.01f;fx.motionDepth=get(kDepth)*.01f;fx.motionMode=getInt(kMotionMode);constexpr int shapes[]={0,1,3,4};fx.shape=shapes[getInt(kShape)];fx.motionHz=getBool(kSync)?float(tempo/60)/division(int(std::lround(get(kRate)*17))):.05f*std::pow(240.f,get(kRate));fx.drift=get(kDrift)*.01f;fx.delayBeats=division(getInt(kDelayDivision));fx.delayFeedback=get(kDelayFeedback)*.01f;fx.roomTime=get(kRoomTime)*.01f;fx.roomDamp=get(kRoomDamp)*.01f;fx.motionDelay=get(kMotionDelay);fx.motionFade=get(kMotionFade);fx.driftHz=get(kDriftRate);fx.chorusHz=get(kChorusRate);fx.chorusDepth=get(kChorusDepth)*.01f;colour.update(fx,tempo);if(getBool(kSync)&&ctx.transport.playing&&ctx.transport.ppqValid){colour.motion.setTempoSync(tempo,division(int(std::lround(get(kRate)*17))));colour.motion.syncToPpq(ctx.transport.ppqPos);}
  body.upright.setTuning(get(kTuning));
  body.felt.setTuning(get(kTuning));
  int mi=0;float targetGain=std::pow(10.f,get(kMaster)/20);
  for(int frame=0;frame<ctx.numFrames;++frame,++clock){while(mi<ctx.numMidiEvents&&ctx.midi[mi].sampleOffset<=frame)midi(ctx.midi[mi++]);
   if(clock>=nextPendingDue){
    nextPendingDue=~uint64_t(0);
    for(auto&e:pending)if(e.active){
     if(e.due<=clock){e.active=false;if(e.voice>=0&&voices[e.voice].state().active&&voices[e.voice].state().owner==e.owner)voices[e.voice].retune(e.note,get(kGlide),get(kTuning));else strike(e.note,e.velocity,e.owner);}
     else nextPendingDue=std::min(nextPendingDue,e.due);
    }
   }
   if((clock&31)==0){float drift=colour.pitchRatio();for(auto&v:voices)if(v.state().active)v.updatePitch(drift*bend[sourceChannel(v.state().owner)]*get(kTuning)/v.state().tuning);}
   float left=0,right=0,uprightProfiled=0,uprightDetail=0;
   std::array<float,7>bridge{};
   std::array<float,2>uprightDirect{},feltDirect{};
   float feltProfiled=0,feltDetail=0;
   bool held=false;
   for(auto&v:voices)if(v.state().active){
    const float x=v.tick();
    const auto& state=v.state();
    bridge[v.model()]+=state.bodySend;
    if(const auto* instrument=std::get_if<felt::UprightVoice>(&v.engine)){
     uprightProfiled+=state.bodySend*instrument->soundboardProfile;
     uprightDetail+=instrument->bridgeDetail;
     uprightDirect[0]+=x*(.707106f-.35f*state.pan);
     uprightDirect[1]+=x*(.707106f+.35f*state.pan);
    }else if(const auto* instrument=std::get_if<felt::FeltVoice>(&v.engine)){
     feltProfiled+=state.bodySend*instrument->soundboardProfile;
     feltDetail+=instrument->bridgeDetail;
     feltDirect[0]+=x*(.707106f-.35f*state.pan);
     feltDirect[1]+=x*(.707106f+.35f*state.pan);
    }else{
     left+=x*(.707106f-.35f*state.pan);
     right+=x*(.707106f+.35f*state.pan);
    }
    held|=state.down||state.sustained;
   }
   bool pedal=false;
   for(bool p:pedalDown)pedal|=p;
   auto radiation=body.process(bridge,pedal&&getBool(kPedal),uprightProfiled,uprightDetail,uprightDirect,feltProfiled,feltDetail,feltDirect);
   left+=radiation[0];right+=radiation[1];colour.gate(held);
   auto out=colour.process(left*.65f,right*.65f);gain+=.002f*(targetGain-gain);for(int ch=0;ch<std::min(2,ctx.numOutputs);++ch)if(ctx.outputs[ch]){float y=out[ch]*gain;float mag=std::abs(y);if(mag>.85f)y=std::copysign(.85f+.13f*mpc::dsp::fastTanh((mag-.85f)/.13f),y);ctx.outputs[ch][frame]=std::abs(y)<1e-25f?0:y;}
  }
  // MIDI command pads are silent. Drain before returning this block so a
  // host suspending a silent instance cannot strand a command until a later
  // note. Host notifications stay deferred to non-render polling/idle.
  service(false);
 }
 void saveExtraState(std::vector<uint8_t>&out)override{out.insert(out.end(),{'F','C','L','B','0','0','0','1'});uint32_t x=randomState.load();for(int i=0;i<4;++i)out.push_back(uint8_t(x>>(8*i)));}
 void loadExtraState(const uint8_t*data,size_t size)override{if(size==12&&!std::memcmp(data,"FCLB0001",8)){uint32_t x=0;for(int i=0;i<4;++i)x|=uint32_t(data[8+i])<<(8*i);randomState.store(x?x:0x46434c42);}}
 const std::vector<uint8_t>& snapshot(){savedState=mpc::Plugin::internalSaveState();size_t pos=12;for(int k=0;k<numParams();++k){size_t n=savedState[pos++];pos+=n;if(transient(k)||editIndex(k)>=0||(k>=kMainQ0&&k<=kMainQ15))std::fill(savedState.begin()+pos,savedState.begin()+pos+8,0);pos+=8;}return savedState;}
 static bool validState(const uint8_t*data,size_t size){if(!data||size<16||size>1024*1024||std::memcmp(data,"MPCP",4))return false;size_t pos=4;auto u32=[&](uint32_t&v){if(size-pos<4)return false;v=0;for(int j=0;j<4;++j)v|=uint32_t(data[pos++])<<(8*j);return true;};uint32_t version,count;if(!u32(version)||!u32(count)||(version!=1&&version!=2)||count>4096)return false;for(uint32_t i=0;i<count;++i){if(pos>=size)return false;size_t n=data[pos++];if(n>size-pos)return false;pos+=n;for(int j=0;j<(version==2?2:1);++j){uint32_t bits;if(!u32(bits))return false;float value;std::memcpy(&value,&bits,4);if(!std::isfinite(value))return false;}}uint32_t extra;return u32(extra)&&extra<=size-pos;}
 bool restore(const uint8_t*data,size_t size){if(!validState(data,size))return false;restoring.store(true);{Mutation m(this);set(kHiss,0);set(kLockHiss,0);for(int k=kCommandPads;k<kParamCount;++k)set(k,param(k).def);}bool ok=mpc::Plugin::internalLoadState(data,size);{Mutation m(this);for(int k=0;k<kParamCount;++k)if(transient(k)||editIndex(k)>=0||(k>=kMainQ0&&k<=kMainQ15))set(k,0);set(kPage,std::min(getInt(kPage),getInt(kSlots)));set(kFamily,family(getInt(kModel)));}for(auto&x:requests)x.store(false);for(auto&x:seen)x.store(false);learnSlot.store(-1);haveUndo=false;restoring.store(false);loadDeadline.store((std::chrono::steady_clock::now()+std::chrono::milliseconds(500)).time_since_epoch().count());loadGuard.store(int(sampleRate*.5));dirtyAll.store(true);return ok;}
 std::string chordName(int slot)const{static const char*names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};return std::string(names[(getInt(rootKey(slot))+getInt(kKey)+getInt(kTranspose)+24)%12])+felt::kTypeSuffixes[getInt(rootKey(slot,1))];}
 bool formatValue(int key,float,std::string&out)override{
  if(transient(key)&&key!=kCommandLearn){out="Transient";return true;}
  if(key>=kMainQ0&&key<=kMainQ15){int pos=key-kMainQ0;int target=kMainKeys[pos];out=displayText(target);if(!param(target).unit.empty())out+=" "+param(target).unit;return true;}
  if(int pos=editIndex(key);pos>=0){int target=realEditTarget(pos);if(pos%4==0){static const char*n[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};out=n[(getInt(target)+getInt(kKey))%12];}else if(pos%4==3)out=(getInt(target)>0?"+":"")+std::to_string(getInt(target))+" oct";else if(pos%4==1)out=param(target).choices[getInt(target)];else out=param(target).choices[getInt(target)];return true;}
  for(int row=0;row<4;++row)if(key==chordNameKey(row)){out=chordName(slotForRow(row));return true;}
  if(key==kModel){static const char*names[]={"TINE","WURLI","UPRIGHT","FELT","CLAV","MUTED","THUMB"};out=names[getInt(kModel)];return true;}
  if(key==kCharacter3Label){out=getInt(kModel)==1?"REED":getInt(kModel)==6?"BELL":family(getInt(kModel))==0?"BELL":family(getInt(kModel))==1?"STRING":"BITE";return true;}
  if(key==kCharacter4Label){out=family(getInt(kModel))==0?"BARK":family(getInt(kModel))==1?"BOARD":"MUTE";return true;}
  if(key==kCurrentChordPreset){out=felt::kProgressionNames[getInt(kChordPreset1+getInt(kPage))];return true;}
  if(key==kPageInfo){int start=getInt(kPage)*4+1;out=std::to_string(start)+"–"+std::to_string(start+3)+" / "+std::to_string(slots());return true;}
  if(key==kCommandInfo){
#ifdef FELT_PAD_DIAGNOSTICS
 out="IN "+std::to_string(rawPadNote.load())+" R"+std::to_string(rawPadOn.load())+"/"+std::to_string(rawPadOff.load())+" P"+std::to_string(renderPadOn.load())+"/"+std::to_string(renderPadOff.load())+" D"+std::to_string(midiDiceCount.load())+" L"+std::to_string(getInt(kCommandLearn))+" C"+std::to_string(learnConflict.load())+" G"+std::to_string(loadGuard.load());return true;
#endif
 int learn=getInt(kCommandLearn);if(learn)out=std::string("PRESS PAD: ")+(learn==1?"PREV":learn==2?"NEXT":"DICE");else out="PREV "+std::to_string(getInt(kCmdPrevNote))+" / NEXT "+std::to_string(getInt(kCmdNextNote))+" / DICE "+std::to_string(getInt(kCmdDiceNote));return true;}
  if(key==kLearnInfo){if(learnConflict.load())out="NOTE IN USE — CHOOSE ANOTHER PAD";else if(learnSlot.load()>=0)out="PRESS A PHYSICAL PAD FOR SLOT "+std::to_string(learnSlot.load()+1);else{static const char*names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};constexpr int major[]={0,2,4,5,7,9,11},minor[]={0,2,3,5,7,8,10};int tonic=getInt(kKey);out=std::string(names[tonic])+(getInt(kScale)?" MINOR · ":" MAJOR · ");for(int i=0;i<7;++i){if(i)out+="  ";out+=names[(tonic+(getInt(kScale)?minor[i]:major[i]))%12];}}return true;}
  if(key==kRate){if(getBool(kSync))out=param(kDelayDivision).choices[int(std::lround(get(kRate)*17))];else{char text[32];std::snprintf(text,sizeof(text),"%.2f Hz",.05f*std::pow(240.f,get(kRate)));out=text;}return true;}
  if(key>=kPadNote1&&key<=kPadNote16&&(key-kPadNote1)%5==0){int n=getInt(key);if(n<0)out="Unassigned";else{static const char*names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};out=std::string(names[n%12])+std::to_string(n/12-1)+" / "+std::to_string(n);}return true;}
  return false;
 }
};

static VstIntPtr feltDispatch(AEffect*e,VstInt32 op,VstInt32 index,VstIntPtr value,void*ptr,float opt){
 auto*p=static_cast<FeltClub*>(e->object);
#ifdef FELT_PAD_DIAGNOSTICS
 if(op==effProcessEvents&&ptr){auto*events=static_cast<VstEvents*>(ptr);for(int i=0;i<events->numEvents&&i<4096;++i){auto*event=events->events[i];if(!event||event->type!=kVstMidiType)continue;auto*me=reinterpret_cast<VstMidiEvent*>(event);unsigned status=static_cast<unsigned char>(me->midiData[0])&0xf0;int note=static_cast<unsigned char>(me->midiData[1]),velocity=static_cast<unsigned char>(me->midiData[2]);if(status==0x90&&velocity){p->rawPadNote.store(note);p->rawPadOn.fetch_add(1);}else if(status==0x80||(status==0x90&&!velocity))p->rawPadOff.fetch_add(1);}p->changed[kCommandInfo].store(true);p->dirtyAll.store(true);}
#endif
 if(op==effOpen){if(p->hostCallback())p->hostCallback()(e,audioMasterNeedIdle_Deprecated,0,0,nullptr,0);return mpc::detail::dispatcher(e,op,index,value,ptr,opt);}
 if(op==effGetParamName&&ptr){if(index<0||index>=kParamCount){*static_cast<char*>(ptr)=0;return 0;}int target=index;if(index>=kMainQ0&&index<=kMainQ15){int pos=index-kMainQ0;target=kMainKeys[pos];}int pos=FeltClub::editIndex(target);if(pos>=0){const char*fields[]={"Root","Type","Voicing","Octave"};char name[64];std::snprintf(name,sizeof(name),"Slot %d %s",p->slotForRow(pos/4)+1,fields[pos%4]);mpc::copyString(static_cast<char*>(ptr),64,name);}else if(target==kCharacter3||target==kCharacter4){const char*text=target==kCharacter3?(p->getInt(kModel)==1?"Reed":p->getInt(kModel)==6?"Bell":FeltClub::family(p->getInt(kModel))==0?"Bell":FeltClub::family(p->getInt(kModel))==1?"String":"Bite"):(FeltClub::family(p->getInt(kModel))==0?"Bark":FeltClub::family(p->getInt(kModel))==1?"Board":"Mute");mpc::copyString(static_cast<char*>(ptr),64,text);}else mpc::copyString(static_cast<char*>(ptr),64,p->param(target).name.c_str());return 0;}
 if(op==effGetChunk){if(!ptr)return 0;const auto&s=p->snapshot();*static_cast<void**>(ptr)=const_cast<uint8_t*>(s.data());return s.size();}
 if(op==effSetChunk){return ptr&&value>0?p->restore(static_cast<const uint8_t*>(ptr),size_t(value)):0;}
 if(op==effSetProgram){if(value>=0&&value<felt::kProgramCount){p->applyProgram(int(value));}return 0;}
 if(op==effGetProgram)return std::max(0,p->getInt(kPreset)-1);
 if(op==effGetProgramName){if(ptr)mpc::copyString(static_cast<char*>(ptr),kVstMaxProgNameLen+1,p->getInt(kPreset)>0?felt::kProgramNames[p->getInt(kPreset)-1]:"Custom");return 0;}
 if(op==effGetParamDisplay||op==effGetProgram||op==effGetProgramNameIndexed||op==effIdle_Deprecated||op==effEditIdle)p->flush();
 return mpc::detail::dispatcher(e,op,index,value,ptr,opt);
}
static void feltSet(AEffect*e,VstInt32 key,float value){if(!std::isfinite(value))return;auto*p=static_cast<FeltClub*>(e->object);p->expireLoadGuard();mpc::detail::setParameter(e,key,value);if(!p->onRender())p->flush();}
static float feltGet(AEffect*e,VstInt32 key){auto*p=static_cast<FeltClub*>(e->object);p->flush();return key>=0&&key<kParamCount?p->readParameter(key):0;}
extern "C" MPC_EXPORT AEffect* VSTPluginMain(audioMasterCallback host){AEffect*e=mpc::detail::createEffect(new FeltClub,host);if(e){e->dispatcher=feltDispatch;e->setParameter=feltSet;e->getParameter=feltGet;}return e;}
