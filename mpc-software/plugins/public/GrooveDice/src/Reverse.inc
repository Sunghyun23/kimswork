// Append-only reverse flags; never change the fixed GDLY/GDCY trailers or LayerFields.
std::array<std::array<std::atomic<uint8_t>,17>,8> savedReverse{};
static constexpr size_t ReverseTrailerSize=8+8*17;
static int reverseSlot(int role,int layer){return layer?8+role*3+layer-1:role;}
int soundReverseSlot(int row)const{if(row<0||row>=4||soundRole(row)<0)return -1;return getBool(kCymbalView)?14+row:reverseSlot(soundRole(row),soundLayer(row));}
void setReversePlayback(Voice&v,const dsp::WavData&sound,int slot){if(getBool(kReverse0+slot)){v.pos=double(sound.numFrames())-1-v.pos;v.step=-v.step;}}
void syncReverseUi(){for(int row=0;row<4;++row){float value=soundRole(row)<0?0:get(soundKey(row,LInvert));if(get(kSoundPolarity0+row)!=value){set(kSoundPolarity0+row,value);changed[kSoundPolarity0+row].store(true);}}for(int row=0;row<4;++row){float value=soundRole(row)<0?0:get(soundKey(row,LTune));if(get(kSoundTune0+row)!=value){set(kSoundTune0+row,value);changed[kSoundTune0+row].store(true);}}for(int row=0;row<4;++row){int slot=soundReverseSlot(row);float value=slot<0?0:get(kReverse0+slot);if(get(kSoundRev0+row)!=value){set(kSoundRev0+row,value);changed[kSoundRev0+row].store(true);}}}
bool reverseParameter(int key,float value){
 if(key>=kSoundPolarity0&&key<=kSoundPolarity3){int row=key-kSoundPolarity0;if(soundRole(row)>=0)layerSet(soundKey(row,LInvert),value);else{bool before=internal;internal=true;set(key,0);internal=before;}displayPending.store(true);return true;}
 if(key>=kSoundTune0&&key<=kSoundTune3){int row=key-kSoundTune0;if(soundRole(row)>=0)layerSet(soundKey(row,LTune),value);else{bool before=internal;internal=true;set(key,0);internal=before;}displayPending.store(true);return true;}
 if(key>=kSoundRev0&&key<=kSoundRev3){int slot=soundReverseSlot(key-kSoundRev0);if(slot>=0)layerSet(kReverse0+slot,value);else{bool before=internal;internal=true;set(key,0);internal=before;}displayPending.store(true);return true;}
 if(key>=kReverse0&&key<=kReverse16){syncLayerUi();displayPending.store(true);}return false;
}
void saveReverseKit(int kit){for(int n=0;n<17;++n)savedReverse[kit][n].store(getBool(kReverse0+n)?1:0);}
void loadReverseKit(int kit){for(int n=0;n<17;++n)layerSet(kReverse0+n,savedReverse[kit][n].load());syncLayerUi(true);}
void saveReverseState(std::vector<uint8_t>&out){const char magic[]="GDRV0001";out.insert(out.end(),magic,magic+8);for(auto&kit:savedReverse)for(auto&value:kit)out.push_back(value.load());}
void loadReverseState(const uint8_t*data,size_t&size){for(auto&kit:savedReverse)for(auto&value:kit)value.store(0);if(size<ReverseTrailerSize||std::memcmp(data+size-ReverseTrailerSize,"GDRV0001",8))return;const uint8_t*ptr=data+size-ReverseTrailerSize+8;size-=ReverseTrailerSize;for(auto&kit:savedReverse)for(auto&value:kit)value.store(*ptr++?1:0);}
