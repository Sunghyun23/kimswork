#pragma once
#include <atomic>
#include <array>
#include <thread>
#include <chrono>
#include <fstream>
#include <vector>
#include <algorithm>
#include <sys/stat.h>
#include <ctime>
// One producer (audio), one consumer (export worker). Export freezes the take
// before publication, so neither file I/O nor allocation ever enters process().
namespace groove {
struct MidiCapture {
 static constexpr unsigned Capacity=32768;
 struct Hit{std::atomic<uint32_t> tick{0},data{0},gate{1};};
 std::array<Hit,Capacity> hits{};
 std::atomic<unsigned> count{0};
 // 0 empty,1 recording,2 stopped,3 exporting,4 exported,5 full,6 error
 std::atomic<int> state{0};std::atomic<bool> quit{false};std::thread worker;
 std::atomic<int> bpm{92000};std::atomic<uint32_t>endTick{0};
 void startWorker(){worker=std::thread([this]{while(!quit.load()){if(state.load(std::memory_order_acquire)==3){bool ok=write(mpc::contentPath("Captures"));state.store(ok?4:6,std::memory_order_release);}std::this_thread::sleep_for(std::chrono::milliseconds(30));}});}
 ~MidiCapture(){quit.store(true);if(worker.joinable())worker.join();}
 bool begin(double tempo){if(state.load()==3)return false;count.store(0);endTick.store(0);bpm.store(int(tempo*1000));state.store(1);return true;}
 void push(uint32_t tick,int note,int vel,uint32_t gate){if(state.load()!=1)return;unsigned n=count.load();if(n==Capacity){state.store(5);return;}hits[n].tick.store(tick);hits[n].data.store(uint32_t(note&127)|uint32_t(vel&127)<<8);hits[n].gate.store(std::max(1u,gate));count.store(n+1,std::memory_order_release);endTick.store(std::max(endTick.load(),tick+gate));}
 void stop(){if(state.load()==1)state.store(2);}
 static void be(std::vector<uint8_t>&v,uint32_t x,int n){for(int i=n-1;i>=0;--i)v.push_back(uint8_t(x>>(8*i)));}
 static void vlq(std::vector<uint8_t>&v,uint32_t x){uint8_t b[5];int n=0;b[n++]=x&127;while((x>>=7))b[n++]=uint8_t(x&127)|128;while(n)v.push_back(b[--n]);}
 bool write(const std::string&dir){
  unsigned n=std::min(count.load(std::memory_order_acquire),Capacity);if(!n)return false;
  struct Event{uint32_t t;uint8_t note,vel;bool on;};std::vector<Event> events;events.reserve(n*2);
  for(unsigned i=0;i<n;++i){uint32_t d=hits[i].data.load(),t=hits[i].tick.load();events.push_back({t,uint8_t(d&127),uint8_t((d>>8)&127),true});events.push_back({t+hits[i].gate.load(),uint8_t(d&127),0,false});}
  std::sort(events.begin(),events.end(),[](const Event&a,const Event&b){return a.t!=b.t?a.t<b.t:a.on!=b.on?!a.on:a.note<b.note;});
  std::vector<uint8_t> track={0,0xff,0x51,3};be(track,uint32_t(60000000000LL/std::max(20000,bpm.load())),3);
  track.insert(track.end(),{0,0xff,0x58,4,4,2,24,8});uint32_t prev=0;
  for(auto e:events){vlq(track,e.t-prev);prev=e.t;track.push_back(e.on?0x99:0x89);track.push_back(e.note);track.push_back(e.vel);}uint32_t end=std::max(prev,endTick.load());end=((end+3839)/3840)*3840;vlq(track,end-prev);track.insert(track.end(),{0xff,0x2f,0});
  std::vector<uint8_t> file={'M','T','h','d',0,0,0,6,0,0,0,1,3,192,'M','T','r','k'};be(file,uint32_t(track.size()),4);file.insert(file.end(),track.begin(),track.end());
  // Only create plugin-owned directories. Never overwrite an existing take.
  std::string parent=dir.substr(0,dir.find_last_of('/'));mkdir(parent.c_str(),0755);mkdir(dir.c_str(),0755);
  auto stamp=std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count();
  static std::atomic<unsigned> serial{0};std::string dest=dir+"/Groove-Dice-"+std::to_string(stamp)+"-"+std::to_string(serial.fetch_add(1))+".mid",temp=dest+".tmp";
  struct stat existing{};if(::stat(dest.c_str(),&existing)==0||::stat(temp.c_str(),&existing)==0)return false;
  std::ofstream out(temp,std::ios::binary|std::ios::trunc);if(!out)return false;out.write(reinterpret_cast<const char*>(file.data()),file.size());out.close();if(!out){std::remove(temp.c_str());return false;}if(std::rename(temp.c_str(),dest.c_str()))return false;return true;
 }
};
}
