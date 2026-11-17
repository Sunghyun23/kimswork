// Independent MAIN timekeeper/transition accents. No extra stored note role,
// output bus or content format: rides/crashes have separate pinned cache lanes.
std::array<dsp::WavData,3> cymbalFallback;
std::array<std::vector<int>,3> cymbalSourceByBase;
std::array<std::vector<int>,3>cymbalAll;
std::vector<std::array<std::vector<int>,3>>cymbalPacks;
std::array<std::atomic<int>,3>cymbalMissing{};
std::array<std::array<char,65>,27>cymbalUnresolved{};
std::array<std::atomic<int>,24>savedCymbalMissing{};
std::array<std::array<std::atomic<float>,36>,8>savedCymbals{};
static constexpr size_t CymbalTrailerSize=16+27*64+8*36*4;
const std::vector<int>&cymbalItems(int kind)const{int pack=getInt(kPackFilter)-1;return pack>=0&&pack<int(cymbalPacks.size())?cymbalPacks[pack][kind]:cymbalAll[kind];}
bool cymbalAudible(int kind)const{bool solo=false;for(int i=0;i<3;++i)solo|=getBool(kCym0On+i*LayerFields)&&getBool(kCym0Solo+i*LayerFields);return getBool(kCym0On+kind*LayerFields)&&(!solo||getBool(kCym0Solo+kind*LayerFields));}
double cymbalStart=-1,cymbalFillEnd=-1;
bool cymbalWanted(int kind)const{return kind==1?getInt(kCrashRule)>0:kind==2?getInt(kRideRule)>=3:getInt(kRideRule)>0;}
void initCymbals(){
 std::vector<std::array<std::vector<int>,3>> pools(library.packs.size());cymbalPacks.resize(library.packs.size());for(int k=0;k<8;++k)for(int n=0;n<36;++n)savedCymbals[k][n].store(param(kCym0Sample+n).def);
 for(size_t i=0;i<library.entries.size();++i){const auto&e=library.entries[i];if(e.kind!=0||e.pack<0||e.pack>=int(pools.size()))continue;
  auto name=e.name.substr(e.name.find_last_of('/')+1);for(char&ch:name)ch=std::isalnum(static_cast<unsigned char>(ch))?char(std::tolower(static_cast<unsigned char>(ch))):' ';name=" "+name+" ";
  int family=name.find(" bell ")!=std::string::npos&&(e.role==6||name.find(" ride ")!=std::string::npos)?2:name.find(" ride ")!=std::string::npos?0:name.find(" crash ")!=std::string::npos?1:-1;
  if(family>=0)pools[e.pack][family].push_back(int(i)+1);
  for(int kind=0;kind<3;++kind)if(family==kind||(family<0&&e.role==6)){cymbalAll[kind].push_back(int(i)+1);cymbalPacks[e.pack][kind].push_back(int(i)+1);}
 }
 for(auto&map:cymbalSourceByBase)map.assign(library.entries.size()+1,0);
 for(size_t i=0;i<library.entries.size();++i){const auto&e=library.entries[i];if(e.pack<0||e.pack>=int(pools.size()))continue;uint32_t seed=2166136261u;for(char ch:e.id)seed=(seed^uint8_t(ch))*16777619u;
  for(int kind=0;kind<3;++kind){const auto&pool=pools[e.pack][kind];if(!pool.empty())cymbalSourceByBase[kind][i+1]=pool[seed%pool.size()];}
 }
 // Original procedural fallback instruments for packs without named families.
 // Generated once off the render thread; never repurpose a crash as a ride.
 for(int kind=0;kind<3;++kind){auto&s=cymbalFallback[kind];s.sampleRate=44100;s.channels.resize(1);auto&x=s.channels[0];x.resize(kind==1?110250:kind==0?66150:44100);uint32_t noise=0x12578u+kind;float lp=0;
  for(size_t i=0;i<x.size();++i){double t=i/44100.;noise=noise*1664525u+1013904223u;float white=float(int32_t(noise))/2147483648.f;lp+=.1f*(white-lp);double metal=0;static const double freq[]={431.,743.,1139.,1723.,2617.,3929.};for(int m=0;m<6;++m)metal+=std::sin(6.283185307*freq[m]*(kind==2?2.7:kind==0?1.7:1.)*t)*std::exp(-t*(kind==1?2.8:kind==0?5.:7.)*(1.+m*.13))/6.;double attack=1-std::exp(-t*1800.);double hiss=(white-lp)*std::exp(-t*(kind==1?3.5:14.));x[i]=float(attack*(metal*(kind==2?.75:.5)+hiss*(kind==1?.36:kind==0?.12:.035)));}
 }
}
int cymbalSource(int kind)const{
 int chosen=getInt(kCym0Sample+kind*LayerFields);if(cymbalMissing[kind].load())return 0;if(chosen)return chosen>0&&chosen<=int(library.entries.size())?chosen:0;
 int id=getInt(kXSample0+6);if(id==0)id=legacyLayerIds[6][getInt(kSample0+6*9)];
 return id>0&&id<int(cymbalSourceByBase[kind].size())?cymbalSourceByBase[kind][id]:0;
}
void triggerCymbal(int kind,int velocity,int gateMs,bool preview=false){
 if(cymbalMissing[kind].load()||(!preview&&!cymbalAudible(kind))||(getInt(kCym0Sample+kind*LayerFields)==0&&missingSample[6].load()))return;int key=kCym0Sample+kind*LayerFields;int id=cymbalSource(kind),cache=-1;if(id>0){cache=library.acquire(14+kind,id);if(cache<0)return;}
 Voice*v=nullptr;for(auto&candidate:voices)if(!candidate.active){v=&candidate;break;}if(!v)v=&voices[steal++%voices.size()];deactivate(*v);
 v->preview=preview;v->librarySlot=cache;v->sample=6*8;v->role=6;v->layer=0;v->articulation=kind+1;v->hit=++hitCounter;v->started=false;v->delay=std::max(0,int(std::llround(sr*(30.+get(key+LShift))*.001)));v->pos=0;
 const auto&sound=cache>=0?library.slots[cache].sound:cymbalFallback[kind];v->step=sound.sampleRate/sr*std::pow(2.,get(key+LTune)/12.);v->gain=velocity/127.f*(kind==1?.85f:kind==0?.65f:.55f);v->levelGain=std::pow(10.f,get(key+LLevel)/20.f);v->pos=get(key+LStart)*sound.sampleRate*.001;setReversePlayback(*v,sound,14+kind);v->polarity=getBool(key+LInvert)?-1.f:1.f;v->env=1;v->decay=float(std::exp(-6.9/(sr*get(key+LDecay)*.001)));v->choke=1;v->gate=int(sr*gateMs*.001);v->hpAlpha=get(key+LLowCut)<1?0:float(1-std::exp(-6.283185307*std::min(double(get(key+LLowCut)),sr*.4)/sr));v->lpAlpha=get(key+LHighCut)>=19500?1:float(1-std::exp(-6.283185307*std::min(double(get(key+LHighCut)),sr*.45)/sr));v->hp={0,0};v->lp={0,0};v->active=true;fired.fetch_add(1);
}
int openHatTick()const{
 int rule=getInt(kOpenHatRule);if(rule==1)return 480+960*int(hash(partSeed[2].load()+0x727u)%4);if(rule==2)return 3360;return -1;
}
bool cymbalInFill(double beat)const{return fillStart>=0&&beat>=fillStart&&beat<fillEnd;}
void scheduleCymbals(double start,double end,double step){
 if(get(kDensity)<=0)return;
 if(fillStart>=0)cymbalFillEnd=fillEnd;
 const int ride=getInt(kRideRule),open=getInt(kOpenHatRule),crash=getInt(kCrashRule);int first=int(std::floor(start/4.))-1,last=int(std::floor(end/4.))+1;
 auto emit=[&](double beat,int role,int velocity){if(beat<0||beat<start-1.e-10||beat>=end-1.e-10)return;int offset=int(std::llround((beat-start)/step));int frames=int(std::llround((end-start)/step));if(offset>=0&&offset<frames)addEvent(offset,role,velocity,false,500);};
 auto hatBeat=[&](int bar,int tick){double beat=bar*4.+tick/960.;int swingTick=ride?480:getInt(kHatRhythm)<6?480:240;if((tick/swingTick)%2&&getInt(kHatRhythm)<9)beat+=(get(kSwing)-50)*(swingTick==480?.01:.005);beat+=get(kHatShift)*.001/60*currentTempo.load();return beat;};
 for(int bar=first;bar<=last;++bar){
  if(ride){int stride=ride==1||ride==3?960:480;for(int tick=0;tick<3840;tick+=stride){double beat=hatBeat(bar,tick);if(cymbalInFill(beat))continue;bool bell=ride>=3&&tick==1920;int v=groove::velocityVariation(tick%960==0?78:52,get(kVelSpread),getInt(kRuleHat),hash(partSeed[2].load()+tick+0x519u));emit(beat,bell?10:8,v);}}
  else if(open==1||open==2){int tick=openHatTick();double beat=hatBeat(bar,tick);if(!cymbalInFill(beat))emit(beat,3,groove::velocityVariation(64,get(kVelSpread),getInt(kRuleHat),hash(partSeed[2].load()+tick+71)));}
  if(crash>=3){int interval=crash==3?4:crash==4?8:16;if(bar>=0&&bar%interval==0&&!cymbalInFill(bar*4.))emit(bar*4.,9,94);}
 }
 if(open==3&&cymbalFillEnd>=0){double beat=cymbalFillEnd-.5;bool existing=false;int offset=int(std::llround((beat-start)/step));for(int i=0;i<eventCount;++i)if(events[i].offset==offset&&(events[i].role==2||events[i].role==3)){events[i].role=3;existing=true;break;}if(!existing&&get(kFillDensity)>0)emit(beat,3,64);}
 if(crash==1&&cymbalStart>=0)emit(cymbalStart,9,94);
 if(crash==2&&cymbalFillEnd>=0)emit(cymbalFillEnd,9,94);
}

void saveCymbalKit(int kit){for(int n=0;n<36;++n)savedCymbals[kit][n].store(get(kCym0Sample+n));for(int k=0;k<3;++k)savedCymbalMissing[kit*3+k].store(cymbalMissing[k].load());}
void loadCymbalKit(int kit){for(int n=0;n<36;++n)layerSet(kCym0Sample+n,savedCymbals[kit][n].load());for(int k=0;k<3;++k)cymbalMissing[k].store(savedCymbalMissing[kit*3+k].load());}
void saveCymbalState(std::vector<uint8_t>&out){const char magic[]="GDCY0001";out.insert(out.end(),magic,magic+8);auto put=[&](uint32_t v){for(int k=0;k<4;++k)out.push_back(uint8_t(v>>(k*8)));};put(36);put(8);for(int j=0;j<27;++j){int id=j<3?getInt(kCym0Sample+j*LayerFields):int(savedCymbals[(j-3)/3][((j-3)%3)*LayerFields].load());int ref=j<3?cymbalMissing[j].load():savedCymbalMissing[j-3].load();const char*sha=library.identity(std::max(0,id));if(!sha&&ref>0&&ref<=27)sha=cymbalUnresolved[ref-1].data();for(int k=0;k<64;++k)out.push_back(sha?uint8_t(sha[k]):0);}for(auto&kit:savedCymbals)for(auto&value:kit){float v=value.load();uint32_t bits;std::memcpy(&bits,&v,4);put(bits);}}
void loadCymbalState(const uint8_t*data){for(auto&m:cymbalMissing)m.store(0);for(auto&m:savedCymbalMissing)m.store(0);if(!data)return;auto word=[&](size_t pos){uint32_t v=0;for(int k=0;k<4;++k)v|=uint32_t(data[pos+k])<<(k*8);return v;};if(word(8)!=36||word(12)!=8)return;size_t pos=16;std::array<int,27>ids{};for(int j=0;j<27;++j){std::memcpy(cymbalUnresolved[j].data(),data+pos,64);cymbalUnresolved[j][64]=0;ids[j]=cymbalUnresolved[j][0]?library.find(cymbalUnresolved[j].data()):0;if(cymbalUnresolved[j][0]&&!ids[j])ids[j]=-1;pos+=64;}for(auto&kit:savedCymbals)for(auto&value:kit){uint32_t bits=word(pos);pos+=4;float v;std::memcpy(&v,&bits,4);value.store(std::isfinite(v)?v:0);}for(int j=0;j<27;++j){if(j<3){if(ids[j])layerSet(kCym0Sample+j*LayerFields,std::max(0,ids[j]));if(ids[j]<0)cymbalMissing[j].store(j+1);}else{if(ids[j])savedCymbals[(j-3)/3][((j-3)%3)*LayerFields].store(std::max(0,ids[j]));if(ids[j]<0)savedCymbalMissing[j-3].store(j+1);}}}
