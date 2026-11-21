#pragma once
// Host-level musical ranges only. Engine mechanics/calibration remain isolated.
namespace felt {
struct SoundRange {float low,high;};
struct DiceProfile {SoundRange tone,body,character3,character4,hammer,release,detune,decay;float drift,motion;};
static constexpr DiceProfile kDiceProfiles[]={
 {{48,64},{46,64},{20,40},{10,28},{24,38},{300,580},{.3f,1.3f},{1200,2200},2.5f,46}, // Tine
 {{44,61},{44,62},{25,46},{16,36},{26,40},{200,460},{.2f,1},{900,1900},2,38}, // Wurli
 {{52,64},{44,58},{28,42},{16,27},{27,36},{400,620},{.6f,1.5f},{1400,2400},.8f,20}, // Upright
 {{38,54},{46,62},{25,40},{14,27},{23,34},{430,720},{.6f,1.4f},{1600,2600},1.3f,30}, // Felt
 {{42,60},{44,60},{25,42},{22,46},{25,39},{100,230},{.2f,.8f},{700,1300},1.5f,32}, // Clav
 {{46,59},{42,58},{22,38},{18,38},{25,36},{100,220},{0,.7f},{600,1100},1,28}, // Muted
 {{48,63},{44,62},{24,40},{16,32},{26,38},{180,360},{.2f,.9f},{1000,1800},1,30} // Thumb
};
}
