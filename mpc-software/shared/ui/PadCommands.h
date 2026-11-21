#pragma once
#include <array>
#include <cstdint>
namespace kimswork {
// A bank name is not present in MIDI. Callers supply the configured note map.
// Registration and command presses are consumed; never let their note-offs
// release an instrument voice. Holds/repeated note-ons fire only once.
class PadCommands {
 std::array<std::array<bool,128>,16> held{};
 std::array<uint8_t,64> queue{}; unsigned read=0,write=0;
public:
 void reset(){held={};read=write=0;}
 void clearChannel(unsigned ch){if(ch<16)held[ch]={};}
 bool pop(int& command){if(read==write)return false;command=queue[read++%queue.size()];return true;}
 template<class Event,class Register,class Available>
 bool handle(const Event&e,bool enabled,const std::array<int,3>&notes,int learn,Register registered,Available available){
  if(e.isAllNotesOff()){clearChannel(e.channel());return false;}
  if(!e.isNoteOn()&&!e.isNoteOff())return false;
  const unsigned ch=e.channel(),note=e.note();if(ch>=16||note>=128)return false;
  if(held[ch][note]){if(e.isNoteOff())held[ch][note]=false;return true;}
  if(!e.isNoteOn())return false;
  if(learn>=1&&learn<=3){held[ch][note]=true;registered(learn-1,int(note));return true;}
  if(!enabled)return false;
  for(int i=0;i<3;++i)if(notes[i]>=0&&int(note)==notes[i]&&available(int(note))){
   held[ch][note]=true;
   if(write-read<queue.size())queue[write++%queue.size()]=uint8_t(i);
   return true;
  }
  return false;
 }
};
}
