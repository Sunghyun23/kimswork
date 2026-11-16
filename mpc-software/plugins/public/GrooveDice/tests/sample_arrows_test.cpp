#define main existingLayerTests
#include "layers_test.cpp"
#undef main
int main(){Host h;auto&p=*h.p;p.set(kSoundPart,0);auto before=p.displayText(kSoundQ0);p.orphanKit.store(true);p.unresolvedKit="missing-kit";h.pulse(kSoundSampleNext0);int id=p.getInt(kXSample0);assert(!p.orphanKit.load());assert(id>0&&p.displayText(kSoundQ0)!=before&&!p.displayText(kSoundQ0).empty());h.pulse(kSoundSamplePrev0);assert(p.getInt(kXSample0)>0);
 h.pulse(kSoundSampleNext3);assert(p.getInt(kL2Sample)==p.layerAll[0][0]&&p.getBool(kL2On));assert(p.displayText(kSoundQ12)!="+ ADD SOUND");h.pulse(kSoundSamplePrev3);assert(p.getInt(kL2Sample)==0&&!p.getBool(kL2On)&&p.displayText(kSoundQ12)=="+ ADD SOUND");
 p.setNormalised(kSoundQ4,1);h.render();assert(p.getInt(kL0Sample)==p.layerAll[0].back()&&p.getBool(kL0On));
 p.set(kSoundPart,3);for(int row=0;row<4;++row){std::array<int,8>old;for(int r=0;r<8;++r)old[r]=p.getInt(kXSample0+r);h.pulse(kSoundSampleNext0+row*2);assert(p.getInt(kXSample0+4+row)>0);for(int r=0;r<8;++r)if(r!=4+row)assert(p.getInt(kXSample0+r)==old[r]);assert(!p.displayText(kSoundQ0+row*4).empty());}
 p.set(kSoundPart,2);std::array<int,8>old;for(int r=0;r<8;++r)old[r]=p.getInt(kXSample0+r);h.pulse(kSoundSampleNext2);for(int r=0;r<8;++r)assert(p.getInt(kXSample0+r)==old[r]);
 assert(p.displayText(kActionInfo)=="SAMPLE CHANGED");p.set(kKitLock,1);h.pulse(kRoll);assert(p.displayText(kDiceInfo)=="KIT FIXED"&&p.displayText(kActionInfo)=="Roll Dice");p.set(kKitLock,0);h.pulse(kRoll);assert(p.displayText(kDiceInfo)=="DICE APPLIED");
 std::cout<<"PASS sample arrows for base/layers/Hat/Perc, clear/add/ON state, named Q-Link choice, row isolation and clean status captions\n";
}
