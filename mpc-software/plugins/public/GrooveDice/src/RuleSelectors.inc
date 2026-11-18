// Full-range selectors append; original seven/eight-choice aliases retain their ranges.
std::atomic<int>ruleNotifyIndex{-1};std::atomic<uintptr_t>ruleNotifyThread{0};
static bool legacyRuleAlias(int key){return(key>=kRuleKick&&key<=kRulePerc)||(key>=kRuleKickStyle&&key<=kRulePercStyle)||(key>=kRulePickKick&&key<=kRulePickPerc);}
std::array<std::atomic<int>,4>ruleSelectorSeen{{-1,-1,-1,-1}};
static bool isFullRuleSelector(int key){return key>=kRulePickFullKick&&key<=kRulePickFullPerc;}
static bool isRuleSelector(int key){return isFullRuleSelector(key)||(key>=kRulePickKick&&key<=kRulePickPerc);}
int ruleStyle(int group)const{return getInt(kRuleKickStyle+group)==0?0:1+getInt(kRulePickFullKick+group);}
static int ruleIntensity(int style){static constexpr int levels[17]={0,1,2,3,1,2,2,3,2,2,3,2,2,3,2,3,3};return levels[mpc::clamp(style,0,16)];}
void syncRuleSelector(int group,bool force=false){int style=ruleStyle(group);if(force||ruleSelectorSeen[group].exchange(style)!=style){layerSet(kRulePickKick+group,style>0&&style<=7?style-1:0);ruleSelectorSeen[group].store(style);}}
void syncRuleSelectors(){for(int g=0;g<4;++g)syncRuleSelector(g);}
void selectFullRule(int group,int style){
 layerSet(kRulePickFullKick+group,std::max(0,style-1));layerSet(kRuleKickStyle+group,style<=7?style:1);layerSet(kRulePickKick+group,style>0&&style<=7?style-1:0);syncRuleStyle(group);
}
bool ruleSelectorParameter(int key){
 if(isFullRuleSelector(key)){int group=key-kRulePickFullKick;selectFullRule(group,getInt(key)+1);return true;}
 if(key>=kRulePickKick&&key<=kRulePickPerc){int group=key-kRulePickKick;selectFullRule(group,getInt(key)+1);return true;}
 if(key>=kRuleKickStyle&&key<=kRulePercStyle){int group=key-kRuleKickStyle;selectFullRule(group,getInt(key));return true;}
 if(key>=kDiceLock4&&key<=kDiceLock7&&!getBool(key)){
  int group=key-kDiceLock4;if(getInt(kRuleKickStyle+group)==0)selectFullRule(group,1);
 }
 return false;
}
