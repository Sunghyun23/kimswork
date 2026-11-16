#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <set>
#include <iostream>
int main(){
 GrooveDice p;auto&lib=p.library;
 assert(p.numParams()==533&&p.numPrograms()>96);
 assert(std::string(p.programName(0))=="Init");
 std::set<std::string>names;
 for(int n=1;n<p.numPrograms();++n){std::string name=p.programName(n);assert(!name.empty()&&name.size()<=31&&names.insert(name).second);assert(!groove::DisplayNames::codedKit(name)&&name.find('~')==std::string::npos);}
 assert(std::string(p.programName(1))=="Velvet Basement");
 assert(p.displayNames.entries[0]=="Dry Kick 01");assert(p.displayNames.folders[0]=="Collection 01 - Kick");
 std::vector<std::pair<std::string,std::string>>entries,kits;
 for(auto&e:lib.entries)entries.push_back({e.name,e.id});for(auto&k:lib.factoryKits)kits.push_back({k.name,k.id});
 p.selectFactoryProgram(17);auto state=p.internalSaveState();p.displayNames.build(lib,p.bank);assert(p.internalSaveState()==state);
 for(size_t n=0;n<entries.size();++n)assert(entries[n]==std::make_pair(lib.entries[n].name,lib.entries[n].id));
 for(size_t n=0;n<kits.size();++n)assert(kits[n]==std::make_pair(lib.factoryKits[n].name,lib.factoryKits[n].id));
 assert(p.internalLoadState(state.data(),state.size()));assert(p.internalSaveState()==state);
 // The serialized pack/kit identity must retain its original code despite the new display title.
 const auto&id=lib.factoryKits[16].id;assert(std::search(state.begin(),state.end(),id.begin(),id.end())!=state.end());
 std::set<std::string>sampleNames;
 for(size_t n=0;n<lib.entries.size();++n){const auto&e=lib.entries[n];if(e.kind<3){auto key=lib.packs[e.pack].id+":"+p.displayNames.entries[n];assert(sampleNames.insert(key).second);assert(p.displayNames.entries[n].find("S2C")==std::string::npos);}if(e.pack>0&&lib.packs[e.pack].isPrivate&&e.kind<3)assert(p.displayNames.entries[n].find("Crate ")==0);}
 // Reordering merged directory entries cannot change a raw path's alias.
 groove::Library fixture;fixture.packs=lib.packs;fixture.entries=lib.entries;std::reverse(fixture.entries.begin(),fixture.entries.end());groove::DisplayNames reordered;reordered.build(fixture,p.bank);
 std::map<std::string,std::string>lookup;for(size_t n=0;n<lib.entries.size();++n)lookup[lib.entries[n].name+lib.entries[n].id]=p.displayNames.entries[n];for(size_t n=0;n<fixture.entries.size();++n)assert(reordered.entries[n]==lookup[fixture.entries[n].name+fixture.entries[n].id]);
 std::cout<<"PASS "<<p.numPrograms()<<" unique readable native kit programs; "<<sampleNames.size()<<" unique sample aliases; raw identities/paths/byte-identical state preserved; directory reordering stable\n";
 for(int n=1;n<=6;++n)std::cout<<n<<": "<<p.programName(n)<<"\n";
}
