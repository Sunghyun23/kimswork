// User-facing seven-style selectors. Old eight-state IDs remain for file/automation compatibility.
std::array<std::atomic<int>,4>ruleSelectorSeen{{-1,-1,-1,-1}};
static bool isRuleSelector(int key){return key>=kRulePickKick&&key<=kRulePickPerc;}
void syncRuleSelector(int group,bool force=false){int style=getInt(kRuleKickStyle+group);if(force||ruleSelectorSeen[group].exchange(style)!=style){layerSet(kRulePickKick+group,std::max(0,style-1));ruleSelectorSeen[group].store(style);}}
void syncRuleSelectors(){for(int g=0;g<4;++g)syncRuleSelector(g);}
bool ruleSelectorParameter(int key){
 if(isRuleSelector(key)){int group=key-kRulePickKick,style=getInt(key)+1;layerSet(kRuleKickStyle+group,style);ruleSelectorSeen[group].store(style);syncRuleStyle(group);return true;}
 if(key>=kDiceLock4&&key<=kDiceLock7&&!getBool(key)){
  int group=key-kDiceLock4;
  // A saved legacy LOCK is represented by a closed padlock + BASIC.
  // Opening that padlock releases the old hold without inventing a new style value.
  if(getInt(kRuleKickStyle+group)==0){layerSet(kRuleKickStyle+group,1);syncRuleStyle(group);}
 }
 return false;
}
