// Visible MAIN randomisation. Per-control locks protect Dice only; manual editing stays live.
static int frontDiceTarget(int n){static const int keys[]={kSwing,kSnareLate,kHatShift,kRndNudge,kRulePickFullKick,kRulePickFullSnare,kRulePickFullHat,kRulePickFullPerc,kMixKick,kMixSnare,kMixHat,kMixPerc,kHatRhythm,kOpenHatRule,kRideRule,kCrashRule};return keys[mpc::clamp(n,0,15)];}
static bool isFrontDiceParameter(int key){for(int n=0;n<16;++n)if(frontDiceTarget(n)==key)return true;return false;}
bool frontDiceLocked(int n)const{return getBool(kDiceLock0+n);}
float diceContinuous(int key,float low,float high,float step,float amount){
 const float centre=(low+high)*.5f,half=std::max(step,(high-low)*.5f*amount);
 int first=int(std::ceil(std::max(low,centre-half)/step-1.e-4f)),last=int(std::floor(std::min(high,centre+half)/step+1.e-4f));
 int count=last-first+1,current=int(std::lround(get(key)/step));if(count<=1)return first*step;
 bool inside=current>=first&&current<=last;int pick=int(random()%unsigned(count-(inside?1:0)))+first;if(inside&&pick>=current)++pick;return pick*step;
}
unsigned randomiseFrontControls(float amount){
 if(amount<=0)return 0;unsigned dirty=0;
 static const float timingLow[]={50,-2,-12,0},timingHigh[]={66,28,8,6};
 bool anyRhythmLock=false;for(int g=0;g<4;++g)anyRhythmLock|=getInt(kRuleKick+g)==0;
 for(int n=0;n<4;++n)if(!frontDiceLocked(n)&&!((n==0||n==3)&&anyRhythmLock)&&!(n==1&&getInt(kRuleSnare)==0)&&!(n==2&&getInt(kRuleHat)==0)){
  int key=frontDiceTarget(n);layerSet(key,diceContinuous(key,timingLow[n],timingHigh[n],1,amount));
  // Retire hidden legacy offsets only for columns whose visible knob was rolled.
  for(int g=0;g<4;++g)partOffset[g][n].store(0);grooveBaseValid.store(false);
 }
 if(!getBool(kPatternLock))for(int g=0;g<4;++g){
  if(frontDiceLocked(4+g)||ruleStyle(g)==0)continue;
  int choices[16]{},count=0;for(int style=1;style<=16;++style){if(g==2&&style==7)continue;if(ruleIntensity(style)==3&&amount<.5f)continue;choices[count++]=style;}
  int old=ruleStyle(g),pick=int(random()%unsigned(count));if(count>1&&choices[pick]==old)pick=(pick+1)%count;
  int style=choices[pick];selectFullRule(g,style);layerSet(kRuleKick+g,ruleIntensity(style));lastRuleStyle[g].store(style);if(style!=old)dirty|=1u<<g;
 }
 if(!getBool(kKitLock)){static const float low[]={-2,-4,-8,-12},high[]={0,1,-2,-4};for(int g=0;g<4;++g)if(!frontDiceLocked(8+g)){int key=kMixKick+g;layerSet(key,diceContinuous(key,low[g],high[g],.1f,amount));}}
 if(!getBool(kPatternLock)){
  if(getInt(kRuleHat)>0){
   if(!frontDiceLocked(12)){
    // Mostly straight eighths/sixteenths; occasional dotted/fill/roll endings.
    static const int choices[]={3,3,3,3,6,6,6,6,4,4,7,7,0,1,5,8,9,10,12,13,11,14};
    int old=getInt(kHatRhythm),next=choices[random()%(sizeof(choices)/sizeof(*choices))];layerSet(kHatRhythm,next);layerSet(kHatRhythmPage,next/8);if(next!=old)dirty|=4;
   }
   if(!frontDiceLocked(13)){static const int choices[]={0,0,0,0,1,1,2,2,3};layerSet(kOpenHatRule,choices[random()%9]);}
   if(!frontDiceLocked(14)){static const int choices[]={0,0,0,0,0,0,1,2,3,4};layerSet(kRideRule,choices[random()%10]);}
  }
  if(getInt(kRulePerc)>0&&!frontDiceLocked(15)){static const int choices[]={0,0,0,0,0,1,2,2,3,4,5};layerSet(kCrashRule,choices[random()%11]);}
 }
 return dirty;
}
