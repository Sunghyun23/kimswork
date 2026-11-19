#define main priorLayerMain
#include "layers_test.cpp"
#undef main
static auto withoutPickers(const std::vector<uint8_t>&state){std::vector<uint8_t>out(state.begin(),state.begin()+12);for(int n=0;n<4;++n)out[8+n]=uint8_t(639>>(n*8));size_t pos=12;for(int i=0;i<kParamCount;++i){size_t bytes=1+state[pos]+8;if(i<639)out.insert(out.end(),state.begin()+pos,state.begin()+pos+bytes);pos+=bytes;}out.insert(out.end(),state.begin()+pos,state.end());return out;}
int main(){Host h;auto&p=*h.p;assert(kRulePickKick==639&&kParamCount==782);
 for(int group=0;group<4;++group){int key=kRulePickKick+group;assert(p.param(key).choices.size()==7);for(auto&name:p.param(key).choices)assert(name!="LOCK");for(int choice=0;choice<7;++choice){p.set(key,choice);h.render();assert(p.getInt(kRuleKickStyle+group)==choice+1&&p.getInt(kRuleKick+group)>0);assert(p.displayText(kQ5+group)!="LOCK");}int reached=0;for(int n=0;n<=1000;++n){p.setNormalised(kQ5+group,n*.001f);h.render();int actual=p.ruleStyle(group);assert(actual>=1&&actual<=16);reached|=1<<actual;}assert(reached==131070);}
 std::cout<<"PASS4legacy seven-choice aliases and4004fine SCREEN increments spanning16styles without LOCK\n";
 p.set(kRuleKickStyle,0);h.render();assert(p.getBool(kDiceLock4)&&p.getInt(kRulePickKick)==0&&p.displayText(kQ5)=="BASIC");auto old=withoutPickers(p.internalSaveState());
 // Old 1.3.2 chunks can contain a legacy hold with its new padlock unset.
 size_t pos=12;for(int i=0;i<639;++i){int len=old[pos];if(std::string((char*)old.data()+pos+1,len)=="DiceLock4")for(int n=0;n<8;++n)old[pos+1+len+n]=0;pos+=1+len+8;}
 Host restored;assert(restored.p->internalLoadState(old.data(),old.size()));restored.p->reset();restored.render();assert(restored.p->getBool(kDiceLock4)&&restored.p->getInt(kRulePickKick)==0&&restored.p->getInt(kRuleKickStyle)==0);for(int i=0;i<200;++i)restored.render();restored.p->set(kDiceLock4,0);restored.render();assert(restored.p->getInt(kRuleKickStyle)==1&&restored.p->getInt(kRuleKick)==1);
 auto state=restored.p->internalSaveState();Host copy;assert(copy.p->internalLoadState(state.data(),state.size()));copy.p->reset();copy.render();assert(copy.p->internalSaveState()==state);
 std::cout<<"PASS legacy LOCK becomes closed padlock + BASIC; opening releases hold; old639/new643state compatibility and exact roundtrip\n";
}
