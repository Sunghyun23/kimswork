#define main priorLayerMain
#include "layers_test.cpp"
#undef main
static double tailEnergy(int part,int row,float milliseconds,bool cymbal=false){
 Host h;h.impulse(true);auto&p=*h.p;
 if(cymbal){p.set(kCymbalView,1);p.set(kCym0Sample+row*G::LayerFields,-1);auto&s=p.cymbalFallback[row];s.sampleRate=44100;s.channels.assign(1,std::vector<float>(44100,.1f));}
 else{p.set(kSoundPart,part);if(part<2&&row>0)h.pin(part,row,true);}
 const int decay=p.soundKey(row,G::LDecay),tune=p.soundKey(row,G::LTune),shift=p.soundKey(row,G::LShift);
 p.set(tune,3);p.set(shift,-7);const int relay=kSoundQ0+row*4+2;
 assert(p.soundTarget(row*4)==p.soundKey(row,G::LSample));assert(p.soundTarget(row*4+1)==p.soundKey(row,G::LLevel));assert(p.soundTarget(row*4+2)==decay);assert(p.soundTarget(row*4+3)==shift);
 p.setNormalised(relay,p.param(decay).toNormalised(milliseconds));assert(std::abs(p.get(decay)-milliseconds)<.001f);assert(p.getInt(tune)==3&&p.getInt(shift)==-7);assert(p.displayText(relay).find("ms")!=std::string::npos);
 p.set(kSoundRow,row);p.set(kSoundDetails,1);const int detail[]={G::LStart,G::LTune,G::LLowCut,G::LHighCut};for(int n=0;n<4;++n)assert(p.soundTarget(n)==p.soundKey(row,detail[n]));p.setNormalised(kSoundQ1,.75f);assert(std::abs(p.get(tune)-6)<.001f&&std::abs(p.get(decay)-milliseconds)<.001f);p.set(kSoundDetails,0);
 h.pulse(kSoundPreview0+row*2);for(int i=0;i<110;++i)h.render();double energy=0;for(size_t i=4410;i<h.audio.size();++i)energy+=double(h.audio[i])*h.audio[i];return energy;
}
int main(){int checked=0;for(int part=0;part<4;++part)for(int row=0;row<(part==2?2:4);++row){double shortTail=tailEnergy(part,row,20),longTail=tailEnergy(part,row,1000);assert(longTail>.001&&shortTail<longTail*.001);++checked;}for(int kind=0;kind<3;++kind){double shortTail=tailEnergy(0,kind,20,true),longTail=tailEnergy(0,kind,1000,true);assert(longTail>.001&&shortTail<longTail*.001);++checked;}assert(checked==17);
 Host h;auto&p=*h.p;p.set(kSoundPart,2);const int fields[]={G::LStart,G::LTune,G::LLowCut,G::LHighCut};for(int row=2;row<4;++row)for(int j=0;j<4;++j)assert(p.soundTarget(row*4+j)==p.layerKey(row,0,fields[j]));
 std::cout<<"PASS all17sounds: Sample/Level/Decay/Shift relays; independent Tune in details; 20ms tail energy <0.1% of1000ms; closed/open hat detail mappings; RT allocation guard\n";
}
