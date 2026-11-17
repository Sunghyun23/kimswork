#define GROOVE_DICE_TEST
#include "../src/GrooveDice.cpp"
#include <cassert>
#include <sstream>
#include <iostream>
#include <filesystem>
static thread_local bool guarded=false;
void*operator new(size_t n){if(guarded)std::abort();if(auto*p=std::malloc(n))return p;throw std::bad_alloc();}
void operator delete(void*p)noexcept{std::free(p);}
struct Rig{
 GrooveDice p;float data[16][512]{},*out[16];Rig(){for(int i=0;i<16;++i)out[i]=data[i];p.prepare(44100,512);p.reset();p.set(kRideRule,4);p.set(kCrashRule,1);p.set(kOpenHatRule,2);}
 void process(mpc::Transport t,int frames=128,const mpc::MidiEvent*midi=nullptr){for(auto&c:data)std::fill(c,c+512,0);mpc::ProcessContext c{};c.outputs=out;c.numOutputs=16;c.numFrames=frames;c.sampleRate=44100;c.transport=t;c.midi=midi;c.numMidiEvents=midi?1:0;guarded=true;p.process(c);guarded=false;for(auto&ch:data)for(int i=0;i<frames;++i)assert(std::isfinite(ch[i]));}
};
mpc::Transport transportAt(double ppq,bool recording=true,double bar=0){mpc::Transport t;t.valid=t.ppqValid=t.tempoValid=t.playing=true;t.bpm=98;t.ppqPos=ppq;t.barStartPpq=bar;return t.recording=recording,t;}
void replay(const char*name){Rig h;auto fpath=std::filesystem::path(__FILE__).parent_path()/"fixtures"/name;std::ifstream f(fpath);assert(f);std::string line;std::getline(f,line);int initial=-1,firstRecord=-1,countin=0,recording=0;
 while(std::getline(f,line)){std::stringstream in(line);std::vector<std::string>a;std::string item;while(std::getline(in,item,','))a.push_back(item);assert(a.size()==12);auto t=transportAt(std::stod(a[4]));t.playing=std::stoi(a[1]);t.recording=std::stoi(a[2]);t.ppqValid=std::stoi(a[3]);t.samplePos=std::stod(a[5]);t.barStartPpq=std::stod(a[6]);t.cycling=std::stoi(a[7]);t.cycleStartPpq=std::stod(a[8]);t.cycleEndPpq=std::stod(a[9]);t.bpm=std::stod(a[10]);int before=h.p.fired.load();h.process(t,std::stoi(a[11]));
 if(a[0]=="countin"){if(initial<0)initial=before;assert(h.p.fired.load()==initial);++countin;}
 if(a[0]=="recording"){if(firstRecord<0){firstRecord=before;assert(h.p.fired.load()>before);}++recording;}
 }
 assert(recording>0&&h.p.fired.load()>firstRecord);if(countin)assert(countin==842);std::cout<<"PASS actual MPC fixture "<<name<<": "<<countin<<" count-in blocks silent, first record block audible\n";
}
int main(){
 replay("mpc-xl-positive-countin.csv");replay("mpc-xl-positive-countin-control.csv");replay("mpc-xl-no-countin-control.csv");
 for(int frames:{32,128,512})for(bool preview:{false,true}){
  Rig h;h.p.set(kPreview,preview);double step=98./(60*44100);double start=8.22940740740741;for(int n=0;n<100;++n){h.process(transportAt(start+n*frames*step),frames);assert(h.p.fired.load()==0);}h.process(transportAt(0),frames);assert(h.p.fired.load()>0);
 }
 std::cout<<"PASS positive pre-roll blocks32/128/512 with Preview on/off; seek release and finite/allocation-free rendering\n";
 {Rig h;h.process(transportAt(7.2));auto e=mpc::MidiEvent::noteOn(0,h.p.getInt(kPadNote0),100,17);h.process(transportAt(7.21),128,&e);assert(h.p.fired.load()==1);auto stopped=transportAt(0);stopped.playing=stopped.recording=false;h.process(stopped);int before=h.p.fired.load();h.process(transportAt(0));assert(h.p.fired.load()>before);}
 std::cout<<"PASS direct MIDI during count-in, cancel then immediate no-count-in retry\n";
 {Rig h;h.p.set(kDensity,100);h.p.set(kRuleKick,0);h.p.set(kRndNudge,0);h.p.notes[4][0].write({2400,0,100,100,80});h.process(transportAt(2,false));int before=h.p.fired.load();h.process(transportAt(2.005,true));h.process(transportAt(2.5,true));assert(h.p.fired.load()>before);}
 {Rig h;h.process(transportAt(4,true,4));assert(h.p.fired.load()>0);}
 {Rig h;h.p.set(kFollow,0);h.p.set(kPreview,1);h.process(transportAt(7.2));assert(h.p.fired.load()==0);h.p.set(kFollow,1);h.process(transportAt(7.21));assert(h.p.fired.load()==0);h.process(transportAt(0));assert(h.p.fired.load()>0);}
 {groove::HostRecordGate g;auto t=transportAt(0);assert(!g.suspended(t,.005));t.ppqPos=15.999;assert(!g.suspended(t,.005));t.ppqPos=0;assert(!g.suspended(t,.005));g.reset();t=transportAt(9);assert(g.suspended(t,.005));t=transportAt(9.1,false);assert(!g.suspended(t,.005));}
 std::cout<<"PASS punch-in during playback, aligned nonzero-bar record start, live Follow toggles, normal loops, recording exit/reset\n";
}
