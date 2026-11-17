#include "../src/GrooveDice.cpp"
#include <cassert>
#include <atomic>
#include <iostream>
#include <thread>
static std::atomic<int> notifications{0},audioNotifications{0},displayUpdates{0};static GrooveDice*plugin=nullptr;
static VstIntPtr host(AEffect*e,VstInt32 op,VstInt32 index,VstIntPtr,void*,float value){if(op==audioMasterVersion)return 2400;if(op==audioMasterUpdateDisplay)++displayUpdates;if(op==audioMasterAutomate){static thread_local int depth=0;assert(++depth==1);++notifications;if(plugin&&index>=kRoleOn0&&index<=kL5Invert)++audioNotifications;if(e){char text[64]{};e->getParameter(e,index);e->dispatcher(e,effGetParamDisplay,index,0,text,0);if((index>=kQ5&&index<=kQ8)||(index>=kRuleKickStyle&&index<=kRulePercStyle)||(index>=kRulePickKick&&index<=kRulePickPerc))e->setParameter(e,index,value);}--depth;}return 0;}
int main(){auto*e=VSTPluginMain(host);assert(e);auto*p=static_cast<GrooveDice*>(e->object);plugin=p;p->prepare(44100,128);p->reset();float data[16][128]{},*out[16];for(int i=0;i<16;++i)out[i]=data[i];std::atomic<int> requested{0},completed{0};std::atomic<bool>quit{false};std::thread audio([&]{int seen=0;double ppq=0;while(!quit.load()){int n=requested.load();if(n==seen){std::this_thread::yield();continue;}seen=n;for(auto&ch:data)std::fill(ch,ch+128,0);mpc::ProcessContext c{};c.outputs=out;c.numOutputs=16;c.numFrames=128;c.sampleRate=44100;c.transport.valid=c.transport.ppqValid=c.transport.tempoValid=true;c.transport.bpm=120;c.transport.playing=true;c.transport.ppqPos=ppq;p->process(c);ppq+=128.*120/(60*44100);completed.store(n);}});
 auto block=[&]{int n=requested.fetch_add(1)+1;while(completed.load()!=n)std::this_thread::yield();};
 block();p->flushHostUpdates();notifications.store(0);audioNotifications.store(0);displayUpdates.store(0);
 for(int i=0;i<20;++i){e->setParameter(e,kQ5,.02f+i*.0001f);block();e->getParameter(e,kQ5);}
 int same=notifications.load(),sameAudio=audioNotifications.load(),sameDisplay=displayUpdates.load();notifications.store(0);audioNotifications.store(0);
 e->setParameter(e,kQ5,2/6.f);block();e->getParameter(e,kQ5);int changed=notifications.load(),changedAudio=audioNotifications.load();
 for(int g=0;g<4;++g)for(int style=1;style<8;++style){
  for(int key:{kQ5+g,kRulePickKick+g,kRuleKickStyle+g}){notifications.store(0);audioNotifications.store(0);e->setParameter(e,key,key==kRuleKickStyle+g?style/7.f:(style-1)/6.f);block();e->getParameter(e,key);assert(p->getInt(kRuleKickStyle+g)==style);if(audioNotifications.load()!=0||notifications.load()>16)std::cerr<<"bridge group="<<g<<" style="<<style<<" key="<<key<<" notifications="<<notifications.load()<<" layers="<<audioNotifications.load()<<std::endl;assert(audioNotifications.load()==0&&notifications.load()<=16);}
 }
 // Many small relative inputs must reach every choice without storms or feedback.
 for(int g=0;g<4;++g){int reached=0;for(int n=0;n<=1000;++n){notifications.store(0);audioNotifications.store(0);e->setParameter(e,kQ5+g,n*.001f);block();e->getParameter(e,kQ5+g);reached|=1<<p->getInt(kRuleKickStyle+g);assert(audioNotifications.load()==0&&notifications.load()<=16);}assert(reached==254);}
 for(int group=0;group<4;++group){int count[]={15,4,5,6},target[]={kHatRhythm,kOpenHatRule,kRideRule,kCrashRule};unsigned reached=0;for(int n=0;n<=1000;++n){notifications.store(0);audioNotifications.store(0);e->setParameter(e,kQ13+group,n*.001f);block();e->getParameter(e,kQ13+group);reached|=1u<<p->getInt(target[group]);assert(audioNotifications.load()==0&&notifications.load()<=16);}assert(reached==((1u<<count[group])-1));}
 std::cout<<"PASS MAIN D4relays: all15/4/5/6choices with4004fine VST increments during playback; bounded notifications and no layer fanout"<<std::endl;
 std::cout<<"PASS real VST4x8relays/direct controls and4004small physical-style increments during playback; bounded notifications; reentrant host reads/rule-value echoes terminate"<<std::endl;
 quit.store(true);audio.join();e->dispatcher(e,effClose,0,0,nullptr,0);plugin=nullptr;
 std::cout<<"Same-style20ticks: "<<same<<" notifications / "<<sameAudio<<" layer writes; changed-style: "<<changed<<" notifications / "<<changedAudio<<" layer writes"<<std::endl;
 assert(same<=24&&sameAudio==0&&sameDisplay==0&&changed<=16&&changedAudio==0);
}
