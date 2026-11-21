#define main original_engine_main
#include "engine_test.cpp"
#undef main
int main(){Rig h;auto&p=*h.p;p.set(kCommandPads,1);p.set(kCommandLearn,3);p.set(kInputMode,0);int preset=p.getInt(kSoundCurrent);for(int n:{24,25,26}){h.block({mpc::MidiEvent::noteOn(0,n,100)});h.block();assert(p.getInt(kSoundCurrent)==preset&&!p.getBool(kRandomized));h.block({mpc::MidiEvent::noteOff(0,n)});}std::cout<<"PASS removed MIDI command pads never change sound or consume learn input\n";}
