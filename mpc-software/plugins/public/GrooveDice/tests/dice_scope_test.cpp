#define main originalMain
#include "engine_test.cpp"
#undef main
static auto bankSig(GrooveDice&p,int bank){std::array<uint32_t,256> x{};for(int n=0;n<128;++n){x[n*2]=p.notes[bank][n].data.load();x[n*2+1]=p.notes[bank][n].gate.load();}return x;}
static auto perfSig(GrooveDice&p,int slot){std::array<uint32_t,256> x{};auto&b=GrooveDice::isPerformanceRoll(slot)?p.rollNotes[GrooveDice::rollBank(slot)]:p.notes[slot-4];for(int n=0;n<128;++n){x[n*2]=b[n].data.load();x[n*2+1]=b[n].gate.load();}return x;}
int main(){
 static_assert(kParamCount==918&&kPerformUndo==917,"append only");
 {Host h;auto&p=*h.p;p.set(kRandomAmt,100);std::array<std::array<uint32_t,256>,12> before{};for(int n=0;n<12;++n)before[n]=perfSig(p,n);p.set(kDrive,17);p.set(kRoom,28);h.block();auto old=bankSig(p,4);h.pulse(kRoll);assert(bankSig(p,4)!=old);assert(p.get(kDrive)==17&&p.get(kRoom)==28);for(int n=0;n<12;++n)assert(perfSig(p,n)==before[n]);auto mainAfter=bankSig(p,4);p.set(p.performanceLock(0),1);p.markRecordedFill(0);auto recorded=perfSig(p,4);h.pulse(kPerformGenerateAll);assert(perfSig(p,0)==before[0]&&perfSig(p,4)==recorded);bool changed=false;for(int n=1;n<12;++n)if(n!=4)changed|=perfSig(p,n)!=before[n];assert(changed&&bankSig(p,4)==mainAfter);auto perfAfter=perfSig(p,1);p.set(kDrive,36);p.set(kFillSlot,2);p.set(kFillDensity,67);h.pulse(kUndo);assert(p.getInt(kFillSlot)==2&&p.get(kFillDensity)==67);assert(bankSig(p,4)==old&&perfSig(p,1)==perfAfter&&p.get(kDrive)==36);h.pulse(kPerformUndo);assert(perfSig(p,1)==before[1]&&bankSig(p,4)==old&&p.get(kDrive)==36);assert(p.getBool(p.performanceLock(0))&&p.getBool(p.performanceLock(4)));}
 std::cout<<"PASS separate MAIN/PERFORM Dice and interleaved independent Undo; FX, locks, recorded fills preserved\n";
 {Host h;auto&p=*h.p;p.set(kRandomAmt,100);h.playing=true;h.until(1);auto before=perfSig(p,0);h.pulse(kPerformGenerateAll);assert(p.pendingPerformanceDice&&perfSig(p,0)==before);h.until(4.1);assert(!p.pendingPerformanceDice&&perfSig(p,0)!=before);h.pulse(kPerformUndo);assert(perfSig(p,0)==before);h.until(5);h.pulse(kPerformGenerateAll);h.pulse(kPerformUndo);h.until(8.1);assert(perfSig(p,0)==before);}
 std::cout<<"PASS PERFORM next-bar queue, cancellation by own Undo\n";
 {Host h;auto&p=*h.p;h.pulse(kPerformGenerateAll);auto state=p.internalSaveState();Host other;assert(other.p->internalLoadState(state.data(),state.size()));other.p->reset();assert(!other.p->havePerformanceUndo&&!other.p->getBool(kPerformUndo));for(int n=0;n<12;++n)assert(perfSig(*other.p,n)==perfSig(p,n));}
 {Host h;auto&p=*h.p;auto before=perfSig(p,4);bool generated=p.generatedFills[0].load();auto mainBefore=bankSig(p,4);h.pulse(kRollFill);assert(!p.generatedFills[0].load());h.pulse(kPerformUndo);assert(perfSig(p,4)==before&&p.generatedFills[0].load()==generated&&bankSig(p,4)==mainBefore);}
 std::cout<<"PASS independent transient history resets and twelve phrase state round-trip\n";
}
