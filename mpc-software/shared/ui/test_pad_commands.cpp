#include "PadCommands.h"
#include "mpc_plugin.h"
#include <cassert>
#include <iostream>
int main(){
 kimswork::PadCommands p;std::array<int,3>notes{24,26,28};int action=-1,learned=-1;
 auto handle=[&](mpc::MidiEvent e,bool enabled=true,int learn=0,bool available=true){return p.handle(e,enabled,notes,learn,[&](int i,int n){learned=i*128+n;},[&](int){return available;});};
 for(int ch=0;ch<16;++ch)for(int i=0;i<3;++i){assert(handle(mpc::MidiEvent::noteOn(ch,notes[i],99)));assert(handle(mpc::MidiEvent::noteOn(ch,notes[i],99)));assert(p.pop(action)&&action==i&&!p.pop(action));assert(handle(mpc::MidiEvent::noteOff(ch,notes[i])));assert(!handle(mpc::MidiEvent::noteOff(ch,notes[i])));}
 assert(!handle(mpc::MidiEvent::noteOn(0,24,100),false));assert(!handle(mpc::MidiEvent::noteOn(0,24,100),true,0,false));assert(!handle(mpc::MidiEvent::noteOn(0,60,100)));
 assert(handle(mpc::MidiEvent::noteOn(0,60,100),false,2));assert(learned==188&&!p.pop(action));assert(handle(mpc::MidiEvent::noteOff(0,60),false));
 assert(handle(mpc::MidiEvent::noteOn(0,24,100)));notes[0]=25;assert(handle(mpc::MidiEvent::noteOff(0,24),false));p.reset();assert(!p.pop(action));
 // Preserve chronological prev/next/Dice ordering, not one boolean per command.
 notes={24,26,28};for(int i=0;i<60;++i){int n=notes[i%3];assert(handle(mpc::MidiEvent::noteOn(0,n,100)));assert(handle(mpc::MidiEvent::noteOff(0,n)));}for(int i=0;i<60;++i)assert(p.pop(action)&&action==i%3);assert(!p.pop(action));
 std::cout<<"PASS 16channel command edges, silent learning, conflict/disable passthrough, remapped releases and ordered bounded queue\n";
}
