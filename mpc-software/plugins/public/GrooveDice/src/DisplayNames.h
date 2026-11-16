// Presentation aliases only. Raw bank paths, kit IDs and sample SHA identities stay immutable.
#pragma once
#include <map>
#include <set>
#include <cctype>
namespace groove {
struct DisplayNames {
 std::vector<std::string> entries,kits,folders;
 std::array<std::array<std::string,8>,8> legacy;
 static std::string number(size_t n,size_t width=2){auto s=std::to_string(n);return std::string(s.size()<width?width-s.size():0,'0')+s;}
 static std::string stem(std::string s){auto p=s.find_last_of('/');if(p!=std::string::npos)s=s.substr(p+1);p=s.find_last_of('.');if(p!=std::string::npos)s.resize(p);return s;}
 static bool codedKit(const std::string&s){return s.size()>5&&s[0]=='V'&&std::isdigit(static_cast<unsigned char>(s[1]))&&std::isdigit(static_cast<unsigned char>(s[2]))&&s.find('/')<8;}
 static std::string kitAlias(size_t n,bool original){
  static const char*own[]={"Velvet","Amber","Midnight","Sidewalk","Honey","Dusty"};
  static const char*crate[]={"Crate","After Hours","Brownstone","Tape","Backroom","Lowlight"};
  static const char*places[]={"Basement","Sunday","Room","Window","Porch","Corner","Kitchen","Attic","Lantern","Stoop","Session","Letters","Sunrise","Evening","Market","Walk"};
  auto s=std::string(original?own[n/16%6]:crate[n/16%6])+" "+places[n%16];if(n>=96)s+=" "+number(n/96+1);return s;
 }
 static const char*roleName(int r){static const char*names[]={"Kick","Snare","Closed Hat","Open Hat","Clap","Tom","Cymbal","Perc"};return names[r>=0&&r<8?r:7];}
 static std::string coreSample(const Library::Entry&e){
  auto raw=stem(e.name);auto p=raw.find_last_of('_');if(p==std::string::npos)return raw;
  int n=0;for(size_t j=p+1;j<raw.size();++j){if(!std::isdigit(static_cast<unsigned char>(raw[j])))return raw;n=n*10+raw[j]-'0';}if(n<1||n>16)return raw;
  static const char*character[]={"Dry","Round","Room","Dusty"};int family=(n-1)/4;
  if(e.role==7){static const char*perc[]={"Wood Rim","Shaker","Conga","Bell"};return std::string(perc[family])+" "+number((n-1)%4+1);}
  return std::string(character[family])+" "+roleName(e.role)+" "+number((n-1)%4+1);
 }
 void build(const Library&lib,const SoundBank&bank){
  entries.resize(lib.entries.size());kits.resize(lib.factoryKits.size());folders.resize(lib.folders.size());
  // Assign counters per pack/role after sorting original paths. Installing another pack
  // or reordering the merged directory must not rename existing samples.
  std::map<std::string,std::vector<int>> groups;
  for(size_t i=0;i<lib.entries.size();++i){const auto&e=lib.entries[i];bool own=e.pack>=0&&e.pack<int(lib.packs.size())&&lib.packs[e.pack].id=="kim-original-workshop";
   bool personal=e.pack>=0&&e.pack<int(lib.packs.size())&&lib.packs[e.pack].isPrivate;
   entries[i]=own&&e.kind==0?coreSample(e):stem(e.name);
   if(personal&&e.kind<3)groups[lib.packs[e.pack].id+":"+std::to_string(e.kind)+":"+std::to_string(e.kind==0?e.role:0)].push_back(int(i));
  }
  for(auto&g:groups){auto&ids=g.second;std::sort(ids.begin(),ids.end(),[&](int a,int b){const auto&x=lib.entries[a];const auto&y=lib.entries[b];return x.name==y.name?x.id<y.id:x.name<y.name;});for(size_t n=0;n<ids.size();++n){const auto&e=lib.entries[ids[n]];std::string type=e.kind==1?"Chop":e.kind==2?"Loop":roleName(e.role);entries[ids[n]]="Crate "+type+" "+number(n+1,3);}}
  // Keep core titles stable when packs are added; disambiguate only a conflicting expansion title.
  std::set<std::string>used;
  std::map<int,size_t>ordinal;
  for(size_t i=0;i<lib.factoryKits.size();++i){const auto&k=lib.factoryKits[i];auto n=ordinal[k.pack]++;bool own=k.pack>=0&&k.pack<int(lib.packs.size())&&lib.packs[k.pack].id=="kim-original-workshop";
   auto name=codedKit(k.name)?kitAlias(n,own):k.name;
   if(used.count(name)&&k.pack>=0&&k.pack<int(lib.packs.size())){auto tag=lib.packs[k.pack].name;auto space=tag.find_last_of(' ');if(space!=std::string::npos)tag=tag.substr(space+1);name+=" - "+tag;}
   auto base=name;size_t suffix=2;while(used.count(name))name=base+" "+number(suffix++);used.insert(name);kits[i]=name;
  }
  for(size_t i=0;i<lib.folders.size();++i){const auto&f=lib.folders[i];auto end=f.name.find_last_of('/');if(end==std::string::npos){folders[i]=f.name;continue;}auto prev=end?f.name.find_last_of('/',end-1):std::string::npos;auto parent=f.name.substr(prev==std::string::npos?0:prev+1,end-(prev==std::string::npos?0:prev+1));auto leaf=end==std::string::npos?f.name:f.name.substr(end+1);
   bool numericLeaf=!leaf.empty();for(char c:leaf)numericLeaf=numericLeaf&&(c>='0'&&c<='9');
   if(numericLeaf){folders[i]=std::string(f.kind==1?"Chop":f.kind==2?"Loop":"Drum")+" Collection "+leaf;continue;}
   bool code=!parent.empty();for(char c:parent)code=code&&(c>='0'&&c<='9');
   if(parent.size()>1&&(parent[0]=='O'||parent[0]=='V')){code=true;for(size_t j=1;j<parent.size();++j)code=code&&(parent[j]>='0'&&parent[j]<='9');if(code)parent=parent.substr(1);}
   if(code){size_t p=parent.find_first_not_of('0');auto digits=p==std::string::npos?std::string("0"):parent.substr(p);parent="Collection "+std::string(digits.size()<2?2-digits.size():0,'0')+digits;}
   folders[i]=parent.empty()?leaf:parent+" - "+leaf;
  }
  for(int r=0;r<8;++r)for(int v=0;v<8;++v){int id=lib.find(bank.ids[r*8+v].data());legacy[r][v]=id>0?entries[id-1]:std::string(roleName(r))+" "+number(v+1);}
 }
};
}
