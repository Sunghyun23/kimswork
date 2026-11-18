#include <cassert>
#include <cstdlib>
#include <new>
#include <iostream>
#include <thread>
static thread_local bool forbid=false;
void* operator new(std::size_t n){assert(!forbid);if(void* p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void* p)noexcept{std::free(p);}
void* operator new[](std::size_t n){return ::operator new(n);}
void operator delete[](void* p)noexcept{std::free(p);}
#include "../src/SmoothVoicing.h"
using namespace donut;
int main(){
 SmoothInput in;in.chords[0]={50,2,0,2};in.chords[1]={55,6,0,2};in.chords[2]={48,3,0,2};in.chords[3]={57,2,0,2};
 forbid=true;auto plan=findSmoothVoicing(in);forbid=false;
 assert(plan.valid&&plan.movementBefore==42&&plan.movementAfter==0&&plan.after[0]==64);
 assert(plan.after[1]==64&&plan.after[2]==64&&plan.after[3]==64);
 // Enumerate the same valid voicing/octave set independently for all three
 // movable chords; dynamic programming must achieve the exact global minimum.
 std::array<std::array<int,30>,3> tops{};std::array<int,3> counts{};
 for(int i=0;i<3;++i)for(int v=0;v<6;++v)for(int o=0;o<5;++o){if(v>=3&&v>chordToneCount(in.chords[i+1][1]))continue;SmoothCandidate c;if(smoothCandidate(in.chords[i+1],v,o,c)&&c.notes[c.count-1]-c.notes[0]<=24)tops[i][counts[i]++]=c.notes[c.count-1];}
 int best=10000;
 for(int a=0;a<counts[0];++a)for(int b=0;b<counts[1];++b)for(int c=0;c<counts[2];++c){const int x=tops[0][a],y=tops[1][b],z=tops[2][c];best=std::min(best,std::abs(64-x)+std::abs(x-y)+std::abs(y-z)+std::abs(z-64));}
 assert(best==plan.movementAfter);
 unsigned seed=73;auto random=[&](int max){seed=1664525*seed+1013904223;return int((seed>>8)%unsigned(max));};
 for(int trial=0;trial<400;++trial){SmoothInput a;a.active=(1+random(4))*4;a.scope=random(2);a.page=random(a.active/4);a.start=a.scope?0:a.page*4;a.count=a.scope?a.active:4;
  for(auto& chord:a.chords){chord={36+random(37),random(11),0,1+random(3)};}
  forbid=true;auto b=findSmoothVoicing(a);forbid=false;assert(b.valid&&b.input==a&&b.movementAfter<=b.movementBefore);assert(b.voice[0]==a.chords[a.start][2]&&b.octave[0]==a.chords[a.start][3]);
  int sum=0;for(int i=0;i<a.count;++i){SmoothCandidate c;assert(smoothCandidate(a.chords[a.start+i],b.voice[i],b.octave[i],c));assert(c.notes[c.count-1]==b.after[i]);assert(c.notes[c.count-1]-c.notes[0]<=24);sum+=std::abs(b.after[i]-b.after[(i+1)%a.count]);}
  assert(sum==b.movementAfter);
  SmoothInput applied=a;for(int i=0;i<a.count;++i){applied.chords[a.start+i][2]=b.voice[i];applied.chords[a.start+i][3]=b.octave[i];}
  forbid=true;auto again=findSmoothVoicing(applied);forbid=false;assert(again.valid&&again.movementAfter==b.movementAfter);
  for(int i=0;i<a.count;++i)assert(again.voice[i]==b.voice[i]&&again.octave[i]==b.octave[i]);
 }
 SmoothInput bad=in;bad.chords[0]={84,6,0,4};assert(!findSmoothVoicing(bad).valid);bad=in;bad.start=15;assert(!findSmoothVoicing(bad).valid);
 AtomicSmoothPlan shared;auto other=plan;other.input.count=16;other.after.fill(99);other.voice.fill(5);std::atomic<bool> done{false};
 std::thread writer([&]{for(int i=0;i<20000;++i)shared.publish(i%2?plan:other);done.store(true);});
 int accepted=0;do{SmoothPlan snapshot;if(shared.read(snapshot)){++accepted;assert(snapshot.input==plan.input?snapshot.after==plan.after&&snapshot.voice==plan.voice:snapshot.input==other.input&&snapshot.after==other.after&&snapshot.voice==other.voice);}}while(!done.load());writer.join();assert(accepted);
 std::cout<<"PASS: exact loop optimum, E/E/E/E example42->0st, 400 bounded/idempotent progressions, safe register, allocation-free search and atomic publication\n";
}
