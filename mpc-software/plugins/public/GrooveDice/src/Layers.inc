// SOUND editor, append-only layer controls and state. Included inside GrooveDice.
// All catalogue construction/state serialization is non-render. Rendering pins immutable cache slots.
enum LayerField { LSample, LLevel, LTune, LShift, LDecay, LOn, LLock, LSolo, LStart, LLowCut, LHighCut, LInvert, LayerFields };
static constexpr int SoundOverviewFields[]={LSample,LLevel,LDecay,LShift};
static constexpr int SoundDetailFields[]={LStart,LTune,LLowCut,LHighCut};
static constexpr int LayerStoredCount=4+kL5Invert+1-kRoleOn0;
static constexpr size_t LayerTrailerSize=16+54*64+8*LayerStoredCount*4;
std::array<std::vector<int>,8> layerAll;std::array<std::array<int,8>,8>legacyLayerIds{};
std::vector<std::array<std::vector<int>,8>> layerPacks;
std::array<std::atomic<int>,16> soundSeenTarget{};
std::array<std::atomic<float>,16> soundSeenValue{};
std::array<std::array<std::atomic<float>,LayerStoredCount>,8> savedLayers{};
std::array<std::array<char,65>,54> layerUnresolved{};
std::array<std::atomic<int>,6> layerMissing{};
std::array<std::atomic<int>,48> savedLayerMissing{};
std::array<float,4> mixGain{{1,1,1,1}};
uint64_t hitCounter=0,lastKickHit=0;
int auditionPending=-1;
static int layerKey(int role,int layer,int field){
 if(layer>0)return kL0Sample+(role*3+layer-1)*LayerFields+field;
 if(field==LSample)return kXSample0+role;
 if(field==LLevel)return kLevel0+role*9;
 if(field==LTune)return kTune0+role*9;
 if(field==LDecay)return kDecay0+role*9;
 static const int base[]={0,0,0,kRoleShift0,0,kRoleOn0,kRoleLock0,kRoleSolo0,kRoleStart0,kRoleLowCut0,kRoleHighCut0,kRoleInvert0};
 return base[field]+role;
}
int soundKey(int row,int field)const{if(getBool(kCymbalView))return kCym0Sample+std::min(row,2)*LayerFields+field;int role=soundRole(row);return role<0?-1:layerKey(role,soundLayer(row),field);}
int soundRole(int row)const{if(getBool(kCymbalView))return row<3?6:-1;int part=getInt(kSoundPart);return part<2?part:part==2?(row<2?2+row:-1):4+row;}
int soundLayer(int row)const{if(getBool(kCymbalView))return 0;return getInt(kSoundPart)<2?row:0;}
int soundTarget(int n)const{
 if(getBool(kPoolOpen))return -1;
 if(getBool(kSampleBrowserOpen))return n==0?soundKey(getInt(kSoundRow),LSample):-1;
 if(getBool(kCymbalView)){if(getBool(kSoundDetails))return n<4?soundKey(getInt(kSoundRow),SoundDetailFields[n]):-1;if(n>=12)return soundKey(getInt(kSoundRow),SoundDetailFields[n%4]);return soundKey(n/4,SoundOverviewFields[n%4]);}
 if(getBool(kSoundDetails)){
  int row=getInt(kSoundRow),role=soundRole(row);return role>=0&&n<4?soundKey(row,SoundDetailFields[n]):-1;
 }
 if(getInt(kSoundPart)==2&&n>=8)return layerKey(2+n/4-2,0,SoundDetailFields[n%4]);
 int row=n/4,role=soundRole(row);return role<0?-1:layerKey(role,soundLayer(row),SoundOverviewFields[n%4]);
}
static int layerStoredKey(int n){return n<4?kMixKick+n:kRoleOn0+n-4;}
static bool extraSampleKey(int key){return key>=kL0Sample&&key<=kL5Invert&&(key-kL0Sample)%LayerFields==LSample;}
void layerSet(int key,float value){clearOrphan(canonical(key));if(getNormalised(key)==param(key).toNormalised(value))return;bool before=internal;internal=true;set(key,value);internal=before;changed[key].store(true);}
void initLayerSaved(){for(auto&kit:savedLayers)for(int n=0;n<LayerStoredCount;++n)kit[n].store(param(layerStoredKey(n)).def);}
void initLayers(){
 layerPacks.resize(library.packs.size());
 for(size_t i=0;i<library.entries.size();++i){const auto&e=library.entries[i];if(e.kind!=0)continue;
  for(int role=0;role<8;++role)if(int(e.role)==role||(role==1&&e.role==4)){layerAll[role].push_back(int(i)+1);if(e.pack>=0&&e.pack<int(layerPacks.size()))layerPacks[e.pack][role].push_back(int(i)+1);}
 }
 for(int r=0;r<8;++r)for(int v=0;v<8;++v)legacyLayerIds[r][v]=library.find(bank.ids[r*8+v].data());
 initLayerSaved();for(auto&target:soundSeenTarget)target.store(-1);
}
const std::vector<int>& layerItems(int role)const{int pack=getInt(kPackFilter)-1;return pack>=0&&pack<int(layerPacks.size())?layerPacks[pack][role]:layerAll[role];}
bool layerHasMissing(int role,int layer)const{return layer>0?layerMissing[role*3+layer-1].load()>0:missingSample[role].load();}
float soundNormalised(int n,int key)const{
 if(key<0)return 0;
 if(soundIsSample(n)){int row=getBool(kSampleBrowserOpen)?getInt(kSoundRow):n/4;int count=sampleCount(row);return count>1?float(samplePosition(row))/(count-1):0;}

 return getNormalised(key);
}
void queueLayerUi(){for(int i=kSoundQ0;i<=kSoundInvert;++i)changed[i].store(true);for(int i=kSampleInfo;i<=kSampleRow7;++i)changed[i].store(true);displayPending.store(true);}
void syncLayerUi(bool force=false){
 int part=getInt(kSoundPart),detail=getInt(kSoundDetails),row=getInt(kSoundRow);
 bool before=internal;internal=true;
 for(int n=0;n<16;++n){int key=soundTarget(n);if(key<0){soundSeenTarget[n].store(-1);continue;}float actual=getNormalised(key);int oldKey=soundSeenTarget[n].load();float oldValue=soundSeenValue[n].load();
  if(force||oldKey!=key||oldValue!=actual){float expected=getNormalised(kSoundQ0+n),next=soundNormalised(n,key);
   if(part==getInt(kSoundPart)&&detail==getInt(kSoundDetails)&&row==getInt(kSoundRow)&&values_[kSoundQ0+n]->compare_exchange_strong(expected,next)){changed[kSoundQ0+n].store(true);soundSeenTarget[n].store(key);soundSeenValue[n].store(actual);}
  }
 }
 for(int r=0;r<4;++r){int role=soundRole(r),layer=soundLayer(r);for(int field:{LOn,LSolo,LLock}){int relay=(field==LOn?kSoundOn0:field==LSolo?kSoundSolo0:kSoundLock0)+r;float next=role<0?0:get(soundKey(r,field));if(get(relay)!=next){set(relay,next);changed[relay].store(true);}}}
 int role=soundRole(row);float invert=role<0?0:get(soundKey(row,LInvert));if(get(kSoundInvert)!=invert){set(kSoundInvert,invert);changed[kSoundInvert].store(true);}
 internal=before;
}
bool layerParameter(int key,float value){
 if(key==kCymbalView){layerSet(kSoundRow,0);layerSet(kSoundDetails,0);layerSet(kSampleBrowserOpen,0);syncLayerUi(true);queueLayerUi();return true;}
 if(key==kSampleBrowserOpen){syncLayerUi(true);queueLayerUi();return true;}
 if(key>=kCym0Sample&&key<=kCym2Invert){if((key-kCym0Sample)%LayerFields==0)cymbalMissing[(key-kCym0Sample)/LayerFields].store(0);queueLayerUi();return false;}
 if(key==kSoundPart){layerSet(kCymbalView,0);layerSet(kSampleBrowserOpen,0);if(getInt(kSoundPart)==2&&getInt(kSoundRow)>1)layerSet(kSoundRow,0);layerSet(kSoundDetails,0);syncLayerUi(true);queueLayerUi();return true;}
 if(key==kSoundRow&&!getBool(kCymbalView)&&getInt(kSoundPart)==2&&getInt(kSoundRow)>1)layerSet(kSoundRow,getInt(kSoundRow)-2);
 if(key==kSoundDetails||key==kSoundRow||key==kPackFilter){if(key==kPackFilter)layerSet(kSampleBrowserPage,0);syncLayerUi(true);queueLayerUi();if(key!=kPackFilter)return true;}
 if(key>=kSoundQ0&&key<=kSoundQ15){int n=key-kSoundQ0,target=soundTarget(n);if(target<0)return true;
  int row=getBool(kSampleBrowserOpen)?getInt(kSoundRow):getBool(kSoundDetails)?getInt(kSoundRow):getBool(kCymbalView)&&n>=12?getInt(kSoundRow):getInt(kSoundPart)==2&&n>=8&&!getBool(kCymbalView)?n/4-2:n/4;
  if(!getBool(kSampleBrowserOpen)&&!getBool(kSoundDetails))layerSet(kSoundRow,row);
  float old=getNormalised(target);
  if(soundIsSample(n)){int count=sampleCount(row);selectSample(row,mpc::clamp(int(std::lround(value*(count-1))),0,count-1));if(getBool(kSampleBrowserOpen))layerSet(kSampleBrowserPage,samplePosition(row)/8);}
  else layerSet(target,param(target).toPlain(value));
  soundSeenTarget[n].store(target);soundSeenValue[n].store(getNormalised(target));if(old!=getNormalised(target))displayPending.store(true);return true;
 }
 for(int field:{LOn,LSolo,LLock}){int first=field==LOn?kSoundOn0:field==LSolo?kSoundSolo0:kSoundLock0;if(key>=first&&key<first+4){int role=soundRole(key-first);if(role>=0)layerSet(soundKey(key-first,field),value);displayPending.store(true);return true;}}
 if(key==kSoundInvert){int row=getInt(kSoundRow),role=soundRole(row);if(role>=0)layerSet(soundKey(row,LInvert),value);displayPending.store(true);return true;}
 if(extraSampleKey(key)){int slot=(key-kL0Sample)/LayerFields;layerMissing[slot].store(0);layerSet(key+LOn,value>0?1:0);layerSet(kXKit,0);displayPending.store(true);}
 return false;
}
void stepSoundSample(int row,int direction){
 if(soundRole(row)<0)return;int count=sampleCount(row);if(count<=1)return;
 layerSet(kSoundRow,row);int position=samplePosition(row);
 if(!getBool(kCymbalView)&&soundLayer(row)==0){const auto&items=sampleItems(row);int id=getInt(soundKey(row,LSample));if(id==0)id=legacyLayerIds[soundRole(row)][getInt(kSample0+soundRole(row)*9)];position=0;for(size_t j=0;j<items.size();++j)if(items[j]==id)position=int(j)+1;int next=position==0?(direction>0?1:count-1):1+(position-1+direction+count-1)%(count-1);selectSample(row,next);}
 else selectSample(row,(position+direction+count)%count);syncLayerUi(true);queueLayerUi();
}
bool layerFormat(int key,std::string&out){
 if(sampleBrowserFormat(key,out))return true;
 if(key>=kSoundRowInfo0&&key<=kSoundRowInfo3){int row=key-kSoundRowInfo0,part=getInt(kSoundPart);if(getBool(kCymbalView)){static const char*names[]={"RIDE","CRASH","BELL","SELECTED DETAILS"};out=names[row];return true;}static const char*names[]={"CLOSED HAT","OPEN HAT","CLAP","TOM","CYMBAL","PERC"};out=part<2?"LAYER "+std::to_string(row+1):part==2?(row<2?names[row]:row==2?"CLOSED DETAIL":"OPEN DETAIL"):names[row+2];return true;}
 if(key>=kSoundQ0&&key<=kSoundQ15){int n=key-kSoundQ0,target=soundTarget(n);if(target<0){out="--";return true;}
  if(soundIsSample(n)){int row=getBool(kSampleBrowserOpen)?getInt(kSoundRow):n/4;out=selectedSampleCaption(row);return true;}
  out=param(target).display(get(target));if(!param(target).unit.empty())out+=" "+param(target).unit;return true;
 }
 return false;
}
void resetExtraLayers(int role){for(int l=1;l<4;++l)for(int f=0;f<LayerFields;++f){int key=layerKey(role,l,f);layerSet(key,param(key).def);layerMissing[role*3+l-1].store(0);}}
void rollLayers(int role,bool base=true){
 const auto&items=layerItems(role);if(items.empty())return;
 bool modified=false;int count=role<2?4:1;
 for(int layer=base?0:1;layer<count;++layer){if(!getBool(layerKey(role,layer,LOn))||getBool(layerKey(role,layer,LLock)))continue;
  int key=layerKey(role,layer,LSample),old=getInt(key),position=int(random()%items.size());if(items.size()>1&&items[position]==old)position=(position+1)%items.size();layerSet(key,items[position]);modified|=items[position]!=old;
  if(layer)layerMissing[role*3+layer-1].store(0);else missingSample[role].store(false);
 }
 if(modified)layerSet(kXKit,0);queueLayerUi();
}
void saveLayerKit(int kit){for(int n=0;n<LayerStoredCount;++n)savedLayers[kit][n].store(get(layerStoredKey(n)));for(int slot=0;slot<6;++slot)savedLayerMissing[kit*6+slot].store(layerMissing[slot].load());}
void loadLayerKit(int kit){for(int n=0;n<LayerStoredCount;++n)layerSet(layerStoredKey(n),savedLayers[kit][n].load());for(int slot=0;slot<6;++slot)layerMissing[slot].store(savedLayerMissing[kit*6+slot].load());syncLayerUi(true);}
void saveLayerState(std::vector<uint8_t>&out){
 const char magic[]="GDLY0001";out.insert(out.end(),magic,magic+8);
 auto put=[&](uint32_t word){for(int k=0;k<4;++k)out.push_back(uint8_t(word>>(k*8)));};put(LayerStoredCount);put(8);
 auto id=[&](int sample,int orphan){const char*text=library.identity(sample);if(!text&&orphan>0&&orphan<=54)text=layerUnresolved[orphan-1].data();for(int j=0;j<64;++j)out.push_back(text?uint8_t(text[j]):0);};
 for(int slot=0;slot<6;++slot)id(getInt(kL0Sample+slot*LayerFields),layerMissing[slot].load());
 for(int kit=0;kit<8;++kit)for(int slot=0;slot<6;++slot)id(int(savedLayers[kit][4+kL0Sample+slot*LayerFields-kRoleOn0].load()),savedLayerMissing[kit*6+slot].load());
 for(auto&kit:savedLayers)for(auto&value:kit){float f=value.load();uint32_t word;std::memcpy(&word,&f,4);put(word);}
}
void loadLayerState(const uint8_t*data){
 for(auto&v:layerMissing)v.store(0);for(auto&v:savedLayerMissing)v.store(0);initLayerSaved();if(!data)return;
 auto word=[&](size_t p){uint32_t value=0;for(int k=0;k<4;++k)value|=uint32_t(data[p+k])<<(k*8);return value;};
 if(word(8)!=LayerStoredCount||word(12)!=8)return;
 size_t pos=16;std::array<int,54>ids{};
 for(int j=0;j<54;++j){std::memcpy(layerUnresolved[j].data(),data+pos,64);layerUnresolved[j][64]=0;bool any=false;for(int k=0;k<64;++k)any|=data[pos+k]!=0;ids[j]=any?library.find(layerUnresolved[j].data()):0;if(any&&!ids[j])ids[j]=-1;pos+=64;}
 for(auto&kit:savedLayers)for(auto&value:kit){uint32_t bits=word(pos);pos+=4;float f;std::memcpy(&f,&bits,4);value.store(std::isfinite(f)?f:0);}
 for(int slot=0;slot<6;++slot){int key=kL0Sample+slot*LayerFields;if(ids[slot]!=0||getInt(key)<=int(library.entries.size()))layerSet(key,std::max(0,ids[slot]));if(ids[slot]<0)layerMissing[slot].store(slot+1);}
 for(int kit=0;kit<8;++kit)for(int slot=0;slot<6;++slot){int j=6+kit*6+slot,key=4+kL0Sample+slot*LayerFields-kRoleOn0;if(ids[j]!=0||savedLayers[kit][key].load()<=int(library.entries.size()))savedLayers[kit][key].store(float(std::max(0,ids[j])));if(ids[j]<0)savedLayerMissing[kit*6+slot].store(j+1);}
}
bool layerAudible(int role,int layer)const{
 if(!getBool(layerKey(role,layer,LOn)))return false;
 bool solo=false;if(role<2){for(int l=0;l<4;++l)solo|=getBool(layerKey(role,l,LOn))&&getBool(layerKey(role,l,LSolo));}
 else{int first=role<4?2:4,last=role<4?4:8;for(int r=first;r<last;++r)solo|=getBool(kRoleOn0+r)&&getBool(kRoleSolo0+r);}
 return !solo||getBool(layerKey(role,layer,LSolo));
}
// Start a logical drum hit once; all its layers share routing, velocity, event timing and choke ownership.
void triggerLayers(int role,int velocity,int gateMs,int onlyLayer=-1,bool preview=false){
 if(role<0||role>=8)return;
 const uint64_t hit=++hitCounter;int count=role<2?4:1;
 bool firedHit=false;
 for(int layer=0;layer<count;++layer){
  if((onlyLayer>=0&&layer!=onlyLayer)||(!preview&&!layerAudible(role,layer))||layerHasMissing(role,layer))continue;
  int id=getInt(layerKey(role,layer,LSample)),channel=layer?8+role*3+layer-1:role,cache=-1;
  if(layer&&id<=0)continue;
  if(id>0){cache=library.acquire(channel,id);if(cache<0){if(preview||library.failed[channel].load()==id)continue;cache=library.acquireCurrent(channel);if(cache<0&&layer)continue;}}
  Voice*v=nullptr;for(auto&candidate:voices)if(!candidate.active){v=&candidate;break;}if(!v)v=&voices[steal++%voices.size()];deactivate(*v);
  v->sample=role*8+getInt(kSample0+role*9);v->librarySlot=cache;const auto&sound=cache>=0?library.slots[cache].sound:sounds[v->sample];
  v->preview=preview;v->role=role;v->layer=layer;v->articulation=0;v->hit=hit;v->started=false;v->delay=std::max(0,int(std::llround(sr*(30.+get(layerKey(role,layer,LShift)))*.001)));
  v->pos=get(layerKey(role,layer,LStart))*sound.sampleRate*.001;
  v->step=sound.sampleRate/sr*std::pow(2.,get(layerKey(role,layer,LTune))/12.);
  v->gain=velocity/127.f;v->levelGain=std::pow(10.f,get(layerKey(role,layer,LLevel))/20.f);v->polarity=getBool(layerKey(role,layer,LInvert))?-1.f:1.f;
  v->env=1;v->decay=float(std::exp(-6.9/(sr*get(layerKey(role,layer,LDecay))*.001)));v->choke=1;v->gate=int(sr*gateMs*.001);v->active=true;
  float lowCut=get(layerKey(role,layer,LLowCut)),highCut=get(layerKey(role,layer,LHighCut));v->hpAlpha=lowCut<1?0:float(1-std::exp(-6.283185307*std::min(double(lowCut),sr*.4)/sr));v->lpAlpha=highCut>=19500?1:float(1-std::exp(-6.283185307*std::min(double(highCut),sr*.45)/sr));v->hp={0,0};v->lp={0,0};firedHit=true;
 }
 if(firedHit)fired.fetch_add(1);
}
void startLayerVoice(Voice&voice){
 voice.started=true;int group=getInt(kChoke0+voice.role*9);
 if(group){float coeff=float(std::exp(-1./(sr*get(kChokeTime)*.001)));for(auto&old:voices)if(old.active&&old.hit<voice.hit&&old.preview==voice.preview&&getInt(kChoke0+old.role*9)==group){if(!old.started)deactivate(old);else old.choke=coeff;}}
 if(!voice.preview&&voice.role==0&&voice.hit>lastKickHit){lastKickHit=voice.hit;ducker=1;}
}
