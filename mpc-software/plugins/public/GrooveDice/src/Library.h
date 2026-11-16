// Indexed private content, bounded asynchronous sampler cache.
// The audio thread only consumes published immutable samples and releases slots.
#pragma once
#include "SoundBank.h"
#include <thread>
#include <chrono>
namespace groove {
struct Library {
 struct FactoryKit{std::string name,id;int pack=0;std::array<int,8> sounds{};std::array<std::array<float,5>,8> parts{};std::array<float,8> fx{};};
 struct SoundChoice{int pack=0;std::array<int,4> sounds{};std::array<std::array<float,5>,4> parts{};std::string name;};
 std::array<std::vector<SoundChoice>,4> soundPools;
 std::vector<FactoryKit> factoryKits;
 struct Entry{std::string id,name,dataPath;int pack=0;uint64_t offset=0;uint32_t bytes=0,crc=0,kind=0,role=0,rate=0,channels=0,frames=0;};
 struct MidiNote{int tick=0,note=0,velocity=0,gate=80;};
 struct Midi{std::vector<MidiNote> notes;int length=3840;};
 struct Slot{dsp::WavData sound;std::atomic<int> state{0};int refs=0,id=0;};
 struct Folder{std::string name;int kind=0;std::vector<int> items;};std::vector<Folder> folders;std::array<std::vector<int>,3> folderKinds;
 struct Pack{std::string id,name;bool isPrivate=false;std::array<std::vector<int>,5> categories;std::array<std::vector<int>,3> folders;std::array<std::vector<int>,4> sounds;std::vector<int> kits;};
 std::vector<Pack> packs;int expansionErrors=0;
 std::vector<Entry> entries;std::array<std::vector<int>,5> categories;std::vector<Midi> midi;
 std::array<Slot,64> slots;static constexpr int Channels=14;
 std::array<std::atomic<int>,Channels> request{},ready,failed{},active{},serial{};std::array<int,Channels> current,desiredSeen{};
 std::atomic<bool> quit{false};std::thread worker;std::string path;bool loaded=false;
 Library(){current.fill(-1);for(auto& r:ready)r.store(-1);}
 ~Library(){stop();}
 void stop(){quit.store(true);if(worker.joinable())worker.join();}
 static uint64_t u64(const std::vector<uint8_t>& b,size_t p){return uint64_t(SoundBank::u32(b,p))|(uint64_t(SoundBank::u32(b,p+4))<<32);}
 static uint32_t crc32(const std::vector<uint8_t>& b){uint32_t c=~0u;for(auto x:b){c^=x;for(int n=0;n<8;++n)c=(c>>1)^(0xedb88320u&uint32_t(-int(c&1)));}return~c;}
 static bool read(std::ifstream& f,uint64_t pos,size_t n,std::vector<uint8_t>& b){f.clear();f.seekg(std::streamoff(pos));b.resize(n);return bool(f.read(reinterpret_cast<char*>(b.data()),std::streamsize(n)));}
 static bool parseMidi(const std::vector<uint8_t>& b,Midi& result){
  result={};if(b.size()<14||std::memcmp(b.data(),"MThd",4))return false;
  auto be=[&](size_t p,int n)->uint32_t{uint32_t v=0;while(n--)v=(v<<8)|b[p++];return v;};
  uint32_t header=be(4,4);if(header<6||header>b.size()-8)return false;int ppq=int(be(12,2));if(!ppq||(ppq&32768))return false;
  size_t pos=8+header;uint32_t endTick=0;
  while(pos<b.size()){
   if(b.size()-pos<8||std::memcmp(b.data()+pos,"MTrk",4))return false;uint32_t size=be(pos+4,4);pos+=8;if(size>b.size()-pos)return false;size_t end=pos+size;uint32_t tick=0;uint8_t running=0;
   auto vlq=[&](uint32_t& n){n=0;for(int k=0;k<4;++k){if(pos>=end)return false;uint8_t x=b[pos++];n=(n<<7)|(x&127);if(!(x&128))return true;}return false;};
   while(pos<end){uint32_t delta=0;if(!vlq(delta)||delta>0x0fffffff-tick)return false;tick+=delta;if(pos>=end)return false;uint8_t s=b[pos];if(s&128)++pos;else{s=running;if(!s)return false;}
    if(s==255){if(pos>=end)return false;++pos;uint32_t n;if(!vlq(n)||n>end-pos)return false;pos+=n;}
    else if(s==240||s==247){uint32_t n;if(!vlq(n)||n>end-pos)return false;pos+=n;running=0;}
    else{if(s<128||s>=240)return false;running=s;int n=(s&240)==192||(s&240)==208?1:2;if(size_t(n)>end-pos)return false;int note=b[pos],velocity=n>1?b[pos+1]:0;if(note>127||velocity>127)return false;if((s&240)==144&&velocity){if(result.notes.size()>=65536)return false;result.notes.push_back({int(std::llround(double(tick)*960/ppq)),note,velocity,80});}pos+=n;}
   }endTick=std::max(endTick,tick);
  }
  result.length=std::max(3840,int(std::ceil(double(endTick)/ppq/4))*3840);if(result.length>960*1024)return false;
  std::stable_sort(result.notes.begin(),result.notes.end(),[](const MidiNote&a,const MidiNote&b){return a.tick<b.tick;});return !result.notes.empty();
 }
 bool load(const std::string& name){
  stop();loaded=false;entries.clear();for(auto&c:categories)c.clear();path=name;std::ifstream f(path,std::ios::binary);if(!f)return false;f.seekg(0,std::ios::end);uint64_t size=uint64_t(f.tellg());std::vector<uint8_t>b;
  if(size<48||!read(f,size-16,16,b)||std::memcmp(b.data(),"GDLEND02",8))return false;uint64_t start=u64(b,8);if(start>size-48||!read(f,start,32,b)||std::memcmp(b.data(),"GDLIB002",8)||SoundBank::u32(b,8)!=2)return false;
  uint32_t count=SoundBank::u32(b,12),width=SoundBank::u32(b,16);if(!count||count>50000||width!=384||uint64_t(count)*width>size-start-48)return false;
  if(!read(f,start+32,size_t(count)*width,b))return false;entries.reserve(count);midi.resize(count);
  for(uint32_t i=0;i<count;++i){size_t p=i*width;Entry e;e.dataPath=path;e.id=std::string(reinterpret_cast<char*>(b.data()+p),64);size_t len=0;while(len<256&&b[p+64+len])++len;if(len==256)return false;e.name=std::string(reinterpret_cast<char*>(b.data()+p+64),len);e.offset=u64(b,p+320);e.bytes=SoundBank::u32(b,p+328);e.crc=SoundBank::u32(b,p+332);e.kind=SoundBank::u32(b,p+336);e.role=SoundBank::u32(b,p+340);e.rate=SoundBank::u32(b,p+344);e.channels=SoundBank::u32(b,p+348);e.frames=SoundBank::u32(b,p+352);
   if(e.kind>=5||e.role>=8||!e.bytes||e.bytes>32*1024*1024||e.offset<start+32+uint64_t(count)*width||e.offset>size-16||e.bytes>size-16-e.offset)return false;
   if(e.kind<3&&(e.channels<1||e.channels>2||e.rate<8000||e.rate>192000||e.frames<1||e.frames>1920000))return false;
   for(char ch:e.id)if(!((ch>='0'&&ch<='9')||(ch>='a'&&ch<='f')))return false;
   categories[e.kind].push_back(int(i));entries.push_back(std::move(e));
  }
  for(size_t i=0;i<entries.size();++i)if(entries[i].kind<3){auto& e=entries[i];std::string parent=e.name.substr(0,e.name.find_last_of('/'));int folder=-1;for(size_t n=0;n<folders.size();++n)if(folders[n].name==parent){folder=int(n);break;}if(folder<0){folder=int(folders.size());folders.push_back({parent,int(e.kind),{}});folderKinds[e.kind].push_back(folder);}folders[folder].items.push_back(int(i));}
  for(int k=3;k<5;++k)for(int index:categories[k]){auto&e=entries[index];if(!read(f,e.offset,e.bytes,b)||crc32(b)!=e.crc||!parseMidi(b,midi[index]))return false;}
  if(!loadFactoryKits(f,size))return false;
  loaded=true;quit.store(false);worker=std::thread([this]{run();});return true;
 }
 bool loadFactoryKits(std::ifstream& f,uint64_t size){
  factoryKits.clear();std::vector<uint8_t>b;if(size<32||!read(f,size-32,16,b))return false;if(std::memcmp(b.data(),"GDKEND03",8))return true;
  uint64_t start=u64(b,8);if(start>size-48||!read(f,start,16,b)||std::memcmp(b.data(),"GDKITS03",8))return false;uint32_t count=SoundBank::u32(b,8),width=SoundBank::u32(b,12);if(!count||count>512||width!=800||uint64_t(count)*width!=size-start-48)return false;
  if(!read(f,start+16,size_t(count)*width,b))return false;factoryKits.reserve(count);
  for(uint32_t i=0;i<count;++i){size_t p=i*width;std::vector<uint8_t>record(b.begin()+p,b.begin()+p+796);if(crc32(record)!=SoundBank::u32(b,p+796))return false;size_t n=0;while(n<64&&b[p+n])++n;if(n==64)return false;FactoryKit kit;kit.name=std::string(reinterpret_cast<const char*>(b.data()+p),n);
   for(int r=0;r<8;++r){kit.sounds[r]=find(reinterpret_cast<const char*>(b.data()+p+64+r*64));if(kit.sounds[r]<=0||entries[kit.sounds[r]-1].kind!=0)return false;}
   auto number=[&](size_t pos){uint32_t bits=SoundBank::u32(b,pos);float value;std::memcpy(&value,&bits,4);return value;};
   for(int r=0;r<8;++r)for(int v=0;v<5;++v){float value=number(p+576+r*20+v*4);if(!std::isfinite(value))return false;float lo[]={-48,-12,20,-100,0},hi[]={6,12,2000,100,16};if(value<lo[v]||value>hi[v])return false;kit.parts[r][v]=value;}
   for(int j=0;j<8;++j){float v=number(p+736+j*4);float lo[]={0,0,0,0,0,.05f,0,0},hi[]={100,100,100,100,300,8,100,1};if(!std::isfinite(v)||v<lo[j]||v>hi[j])return false;kit.fx[j]=v;}
   kit.id="core:"+kit.name;factoryKits.push_back(std::move(kit));
  }return loadSoundPools(f,start);
 }
 bool loadSoundPools(std::ifstream& f,uint64_t kitStart){
  for(auto&p:soundPools)p.clear();std::vector<uint8_t>b;if(kitStart<16||!read(f,kitStart-16,16,b))return false;if(std::memcmp(b.data(),"GDSEND04",8))return true;
  uint64_t start=u64(b,8);if(start>kitStart-32||!read(f,start,16,b)||std::memcmp(b.data(),"GDSND004",8))return false;uint32_t count=SoundBank::u32(b,8),width=SoundBank::u32(b,12);if(!count||count>5000||width!=200||uint64_t(count)*width!=kitStart-start-32||!read(f,start+16,size_t(count)*width,b))return false;
  for(uint32_t i=0;i<count;++i){size_t p=i*width;std::vector<uint8_t>record(b.begin()+p,b.begin()+p+196);if(crc32(record)!=SoundBank::u32(b,p+196))return false;int group=SoundBank::u32(b,p);if(group<0||group>3)return false;SoundChoice c;
   for(int j=0;j<2;++j){if(!b[p+4+j*64]){if(j==0||group==2)return false;c.sounds[j]=0;}else{c.sounds[j]=find(reinterpret_cast<char*>(b.data()+p+4+j*64));if(c.sounds[j]<=0||entries[c.sounds[j]-1].kind!=0)return false;}}
   for(int j=0;j<2;++j)for(int k=0;k<5;++k){uint32_t bits=SoundBank::u32(b,p+132+j*20+k*4);float v;std::memcpy(&v,&bits,4);float lo[]={-48,-12,20,-100,0},hi[]={6,12,2000,100,16};if(!std::isfinite(v)||v<lo[k]||v>hi[k])return false;c.parts[j][k]=v;}
   soundPools[group].push_back(c);
  }for(auto&p:soundPools)if(p.empty())return false;
  // Perc is a four-instrument ensemble, anchored to the original percussion ID.
  // Complete it from the same source volume's bank-owned factory catalogue.
  for(size_t n=0;n<soundPools[3].size();++n){auto&c=soundPools[3][n];const auto&path=entries[c.sounds[0]-1].name;size_t end=path.find('/',13);std::string pack=path.substr(0,end);std::vector<const FactoryKit*> matches;
   for(const auto&kit:factoryKits)if(entries[kit.sounds[7]-1].name.substr(0,end)==pack)matches.push_back(&kit);
   if(matches.empty())return false;const auto&kit=*matches[n%matches.size()];for(int j=1;j<4;++j){c.sounds[j]=kit.sounds[j+3];c.parts[j]=kit.parts[j+3];}
   c.name=kit.name.substr(0,3)+" / Perc Set "+std::to_string(n+1);
  }return true;
 }
 // Constructor/non-render only. Each expansion keeps its own payload file and cache lifetime.
 static std::string line(const std::string& path,const std::string& fallback){std::ifstream f(path);std::string v;if(!std::getline(f,v)||v.empty()||v.size()>63)return fallback;return v;}
 void loadExpansions(const std::string& content){
  stop();packs.clear();Pack core;core.id=line(content+"/Banks/core.id","legacy-core");core.name=line(content+"/Banks/core.name","Core");core.categories=categories;core.folders=folderKinds;
  for(size_t i=0;i<factoryKits.size();++i){core.kits.push_back(int(i));factoryKits[i].id=core.id+":"+factoryKits[i].name;}
  for(int g=0;g<4;++g)for(size_t j=0;j<soundPools[g].size();++j)core.sounds[g].push_back(int(j));packs.push_back(std::move(core));
  std::ifstream registry(content+"/Expansions/registry.tsv");std::string row;
  while(packs.size()<128&&std::getline(registry,row)){
   size_t a=row.find('\t'),b=a==std::string::npos?a:row.find('\t',a+1);if(a==std::string::npos||b==std::string::npos){++expansionErrors;continue;}
   std::string id=row.substr(0,a),name=row.substr(a+1,b-a-1);bool safe=!id.empty()&&id.size()<=63&&name.size()<=63;for(char ch:id)if(!((ch>='a'&&ch<='z')||(ch>='0'&&ch<='9')||ch=='-'||ch=='_'))safe=false;
   for(auto&p:packs)if(p.id==id)safe=false;if(!safe){++expansionErrors;continue;}
   Library extra;if(!extra.load(content+"/Expansions/"+id+"/bank.gdbank")){++expansionErrors;continue;}extra.stop();
   if(entries.size()+extra.entries.size()>1048575||factoryKits.size()+extra.factoryKits.size()>65535){++expansionErrors;continue;}
   bool fits=true;for(int g=0;g<4;++g)fits&=soundPools[g].size()+extra.soundPools[g].size()<=65535;if(!fits){++expansionErrors;continue;}
   int pack=int(packs.size()),offset=int(entries.size()),folderOffset=int(folders.size());Pack added;added.id=id;added.name=name;added.isPrivate=row.substr(b+1)=="private";
   for(auto&e:extra.entries){e.pack=pack;e.name=name+"/"+e.name;entries.push_back(std::move(e));}
   for(auto&m:extra.midi)midi.push_back(std::move(m));
   for(int k=0;k<5;++k)for(int n:extra.categories[k]){categories[k].push_back(n+offset);added.categories[k].push_back(n+offset);}
   for(auto&f:extra.folders){f.name=name+"/"+f.name;for(int&n:f.items)n+=offset;folders.push_back(std::move(f));}
   for(int k=0;k<3;++k)for(int n:extra.folderKinds[k]){folderKinds[k].push_back(n+folderOffset);added.folders[k].push_back(n+folderOffset);}
   for(auto&kit:extra.factoryKits){kit.pack=pack;kit.id=id+":"+kit.name;for(int&n:kit.sounds)n+=offset;added.kits.push_back(int(factoryKits.size()));factoryKits.push_back(std::move(kit));}
   for(int g=0;g<4;++g)for(auto&c:extra.soundPools[g]){c.pack=pack;for(int&n:c.sounds)if(n>0)n+=offset;added.sounds[g].push_back(int(soundPools[g].size()));soundPools[g].push_back(std::move(c));}
   packs.push_back(std::move(added));
  }
  quit.store(false);worker=std::thread([this]{run();});
 }
 int find(const char* id)const{for(size_t i=0;i<entries.size();++i)if(std::memcmp(id,entries[i].id.data(),64)==0)return int(i)+1;return 0;}
 const char* identity(int id)const{return id>0&&id<=int(entries.size())?entries[id-1].id.c_str():nullptr;}
 // AUDIO ONLY: single ownership handoff prevents an old voice reading an evicted slot.
 void release(int slot){if(slot>=0&&--slots[slot].refs==0)slots[slot].state.store(0,std::memory_order_release);}
 template<size_t N> void poll(const std::array<int,N>& desired){
  static_assert(N<=Channels,"cache request capacity");for(size_t r=0;r<N;++r){int id=desired[r];if(desiredSeen[r]!=id){desiredSeen[r]=id;request[r].store(id,std::memory_order_relaxed);serial[r].fetch_add(1,std::memory_order_release);}int s=ready[r].exchange(-1,std::memory_order_acquire);if(s>=0){if(slots[s].id==id){slots[s].refs=1;slots[s].state.store(3);release(current[r]);current[r]=s;active[r].store(id);}else slots[s].state.store(0,std::memory_order_release);}
   if(id==0&&current[r]>=0){release(current[r]);current[r]=-1;active[r].store(0);}
  }
 }
 int acquireCurrent(int role){int s=current[role];if(s>=0){++slots[s].refs;return s;}return -1;}
 int acquire(int role,int desired){int s=current[role];if(s>=0&&slots[s].id==desired){++slots[s].refs;return s;}return -1;}
 void run(){std::ifstream f;std::string opened;std::vector<uint8_t>b;std::array<int,Channels>lastSerial{};
  while(!quit.load()){
   for(int r=0;r<Channels&&!quit.load();++r){int generation=serial[r].load(std::memory_order_acquire);int id=request[r].load(std::memory_order_relaxed);if(id==0){lastSerial[r]=generation;failed[r].store(0);continue;}if(id<0||id>int(entries.size())){failed[r].store(id);lastSerial[r]=generation;continue;}if(generation==lastSerial[r]||ready[r].load()!=-1)continue;auto&e=entries[id-1];if(e.kind>=3){failed[r].store(id);lastSerial[r]=generation;continue;}
    int selected=-1;for(int i=0;i<64;++i){int free=0;if(slots[i].state.compare_exchange_strong(free,1,std::memory_order_acquire)){selected=i;break;}}if(selected<0)continue;
    auto&s=slots[selected];if(opened!=e.dataPath){f.close();f.clear();f.open(e.dataPath,std::ios::binary);opened=e.dataPath;}bool ok=read(f,e.offset,e.bytes,b)&&crc32(b)==e.crc&&dsp::parseWav(b,s.sound)&&s.sound.numFrames()==e.frames&&s.sound.numChannels()==int(e.channels)&&s.sound.sampleRate==e.rate;
    if(ok){s.id=id;s.refs=0;s.state.store(2);ready[r].store(selected,std::memory_order_release);failed[r].store(0);}else{s.state.store(0,std::memory_order_release);failed[r].store(id);}lastSerial[r]=generation;
   }
   std::this_thread::sleep_for(std::chrono::milliseconds(5));
  }
 }
};
}
