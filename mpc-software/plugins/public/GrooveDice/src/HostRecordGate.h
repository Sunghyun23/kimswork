// MPC record pre-roll starts at a free-running positive position with a stale
// bar anchor, then seeks to the actual record start. VST has no count-in flag.
// This is an observed-host inference, not a universal host preroll API.
#pragma once
#include <algorithm>
namespace groove {
class HostRecordGate {
 bool wasPlaying=false,pending=false;double previous=0,anchor=0;
public:
 void reset(){wasPlaying=pending=false;previous=anchor=0;}
 bool suspended(const mpc::Transport&t,double blockBeats){
  if(!t.valid||!t.ppqValid||!t.playing){wasPlaying=t.playing;pending=false;return false;}
  const bool entering=!wasPlaying;wasPlaying=true;
  // Ordinary hosts can use negative PPQ. Retain a block that crosses zero:
  // the event scheduler rejects negative events and preserves sample offsets.
  if(t.ppqPos<0){pending=false;return t.ppqPos+blockBeats<=0;}
  const double tolerance=std::max(1.e-7,blockBeats*1.5);
  if(entering&&t.recording&&t.ppqPos>t.barStartPpq+tolerance){pending=true;previous=t.ppqPos;anchor=t.barStartPpq;}
  if(!pending)return false;
  // A seek back is the measured MPC start. An advancing musical bar anchor
  // also ends quarantine. Stopping/cancelling or punching out releases it.
  if(!t.recording||t.ppqPos<previous-tolerance||t.barStartPpq>anchor+tolerance){pending=false;return false;}
  previous=t.ppqPos;return true;
 }
};
}
