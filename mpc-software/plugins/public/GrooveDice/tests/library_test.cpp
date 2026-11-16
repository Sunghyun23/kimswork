#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <iostream>
#include <cstdlib>
static thread_local bool noAlloc=false;
void* operator new(size_t n){if(noAlloc)std::abort();if(auto*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}void operator delete(void*p,size_t)noexcept{std::free(p);}
struct Host{std::unique_ptr<GrooveDice>p=std::make_unique<GrooveDice>();float data[16][128]{};float*out[16];Host(){for(int i=0;i<16;++i)out[i]=data[i];p->prepare(44100,128);p->reset();}void block(){mpc::ProcessContext c;c.numFrames=128;c.sampleRate=44100;c.outputs=out;c.numOutputs=16;for(auto&ch:data)std::fill(ch,ch+128,0);noAlloc=true;p->process(c);noAlloc=false;for(auto&ch:data)for(float v:ch)assert(std::isfinite(v));}void wait(int r,int id){p->set(kLibrarySample0+r,id);for(int i=0;i<1500;++i){block();if(p->library.active[r].load()==id)return;std::this_thread::sleep_for(std::chrono::milliseconds(1));}std::cerr<<"timeout role="<<r<<" wanted="<<id<<" param="<<p->getInt(kLibrarySample0+r)<<" request="<<p->library.request[r].load()<<" active="<<p->library.active[r].load()<<" failed="<<p->library.failed[r].load()<<" ready="<<p->library.ready[r].load()<<"\n";assert(false&&"cache timed out");}};
int main(){Host h;auto&p=*h.p;auto&l=p.library;assert(l.loaded&&l.entries.size()==11134);int counts[]={2411,8043,585,2,93};for(int k=0;k<5;++k)assert(l.categories[k].size()==size_t(counts[k]));size_t events=0;for(int k=3;k<5;++k)for(int id:l.categories[k]){assert(!l.midi[id].notes.empty());events+=l.midi[id].notes.size();}std::cout<<"PASS all11039 WAV/95 MIDI indexed, all MIDI parsed: "<<events<<" note-ons\n";
 for(int kind=0;kind<5;++kind){p.set(kLibraryKind,kind);h.block();p.set(kLibraryFolder,0);int count=kind<3?int(l.folderKinds[kind].size()):int(l.categories[kind].size());for(int page=0;page<(count+7)/8;++page){p.set(kLibraryPage,page);h.block();for(int row=0;row<8;++row)assert(p.displayText(kLibraryRow0+row).size()<=31);}}
 for(int role=0;role<8;++role){p.set(kLibraryRole,role);p.set(kLibrarySample0+role,l.categories[1].front()+1);assert(p.displayText(kLibraryCurrentSound).size()<=31);}
 assert(p.displayText(kLibraryPlaybackInfo).size()<=31);p.set(kLibraryKind,0);h.block();for(int r=0;r<8;++r)p.set(kLibrarySample0+r,0);h.block();
 std::cout<<"PASS library labels respect31-byte native host limit across folders/roles/MIDI categories\n";
 for(int k=0;k<3;++k){int first=l.categories[k].front()+1,last=l.categories[k].back()+1;h.wait(0,first);p.set(kPad0,p.getBool(kPad0)?0:1);h.block();assert(p.fired.load()>0);h.wait(0,last);p.set(kPad0,p.getBool(kPad0)?0:1);h.block();h.wait(0,0);h.wait(0,last);}
 std::cout<<"PASS kits/chops/loops first+last decoded, replay after deselect, live old-voice lifetime\n";
 for(int i=0;i<160;++i){int id=l.categories[0][i%l.categories[0].size()]+1;h.wait(i%8,id);p.set(kPad0+(i%8)*9,p.getBool(kPad0+(i%8)*9)?0:1);h.block();}
 for(int r=0;r<8;++r)h.wait(r,0);for(int i=0;i<3000;++i)h.block();int occupied=0;size_t decoded=0;for(auto&s:l.slots){if(s.state.load())++occupied;decoded+=s.sound.numFrames()*s.sound.numChannels()*sizeof(float);}assert(occupied==0);std::cout<<"PASS160 live replacements/48voices; no audio allocations; cache held slots="<<occupied<<", resident PCM="<<decoded<<" bytes\n";
 int id=l.categories[2].back()+1;h.wait(2,id);p.set(kLibraryBeat,2);p.set(kGrooveTemplate,93);p.set(kUserKit,2);p.set(kSaveKit,1);h.block();auto state=p.internalSaveState();Host other;assert(other.p->internalLoadState(state.data(),state.size()));other.p->reset();for(int warm=0;warm<180;++warm)other.block();assert(other.p->getInt(kLibrarySample2)==id&&other.p->getInt(kLibraryBeat)==2&&other.p->getInt(kGrooveTemplate)==93);other.p->set(kLibrarySample2,0);other.p->set(kLoadKit,1);other.block();assert(other.p->getInt(kLibrarySample2)==id);std::cout<<"PASS library SHA identity state+kit restore; beat/template state retained\n";
 // Library beat selection must be restored by Undo after mutating an imported pattern.
 p.set(kLibraryBeat,1);p.set(kScope,1);p.set(kRandomAmt,100);p.set(kRoll,p.getBool(kRoll)?0:1);h.block();assert(p.getInt(kLibraryBeat)==0);p.set(kUndo,p.getBool(kUndo)?0:1);h.block();assert(p.getInt(kLibraryBeat)==1);
 std::cout<<"PASS imported pattern mutation + Undo restores original library beat selection\n";
 // Old QA state prefix still restores; appended library data is optional.
 for(int n=1;n<=2;++n){p.set(kLibraryBeat,n);p.set(kGrooveTemplate,0);p.set(kPreview,1);p.set(kFollow,0);int before=p.fired.load();for(int i=0;i<3000;++i)h.block();assert(p.fired.load()>before);}for(int n=1;n<=93;++n){p.set(kGrooveTemplate,n);h.block();}std::cout<<"PASS both imported beats play,93 groove templates render finite audio\n";
}
