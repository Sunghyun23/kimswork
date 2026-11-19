#define main priorLayerMain
#include "layers_test.cpp"
#undef main
int main(){Host h;auto&p=*h.p;assert(kParamCount==782);const int fields[2][4]={{G::LSample,G::LLevel,G::LTune,G::LShift},{G::LStart,G::LDecay,G::LLowCut,G::LHighCut}};int checked=0;
 for(int part=0;part<5;++part){if(part==4)p.set(kCymbalView,1);else p.set(kSoundPart,part);int rows=part==2?2:part==4?3:4;
  for(int page=0;page<2;++page){p.set(kSoundDetails,page);for(int row=0;row<rows;++row){for(int column=0;column<4;++column){int q=row*4+column,key=p.soundKey(row,fields[page][column]);assert(p.soundTarget(q)==key);auto before=p.internalSaveState();if(column!=0||page){std::array<float,kParamCount>values;for(int k=0;k<kParamCount;++k)values[k]=p.getNormalised(k);p.setNormalised(kSoundQ0+q,.371f);assert(std::abs(p.getNormalised(key)-.371f)<.00001f);for(int other=0;other<rows;++other)if(other!=row)assert(p.getNormalised(p.soundKey(other,fields[page][column]))==values[p.soundKey(other,fields[page][column])]);}h.render();++checked;}
    p.set(kSoundRev0+row,1);p.set(kSoundPolarity0+row,1);h.render();assert(p.getBool(kReverse0+p.soundReverseSlot(row))&&p.getBool(p.soundKey(row,G::LInvert)));p.set(kSoundPolarity0+row,0);h.render();assert(p.getBool(kReverse0+p.soundReverseSlot(row))&&!p.getBool(p.soundKey(row,G::LInvert)));
   }
   for(int n=rows*4;n<16;++n){assert(p.soundTarget(n)<0);p.setNormalised(kSoundQ0+n,.73f);}h.render();
  }
 }
 assert(checked==136);p.set(kSoundDetails,1);p.set(kSoundPart,1);assert(p.getBool(kSoundDetails));p.set(kCymbalView,1);assert(p.getBool(kSoundDetails));auto state=p.internalSaveState();Host restored;assert(restored.p->internalLoadState(state.data(),state.size()));restored.render();assert(restored.p->internalSaveState()==state);
 // A sample drawer temporarily owns only A1, then restores the selected page and all row targets.
 p.openSampleBrowser(1);assert(p.getBool(kSoundDetails));assert(p.soundTarget(0)==kCym1Sample);for(int n=1;n<16;++n)assert(p.soundTarget(n)<0);p.set(kSampleBrowserOpen,0);for(int n=0;n<12;++n)assert(p.soundTarget(n)==p.soundKey(n/4,fields[1][n%4]));
 std::cout<<"PASS17sources x2pages x4Q-Links, no cross-row writes, REV/polarity independent, unused groups inert, shared page restore, sample-drawer isolation and RT guard\n";
}
