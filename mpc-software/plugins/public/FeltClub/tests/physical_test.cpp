#include "../src/PhysicalVoice.h"
#include "../src/Colour.h"
#include <cassert>
#include <cmath>
#include <cstdio>
#include <algorithm>
int main(){
 double maxPeak=0;float maxForce=0,maxContact=0;int strikes=0;
 for(float sr:{44100.f,48000.f,96000.f})for(int model=0;model<7;++model)for(int key=21;key<=108;++key)for(int velocity:{1,32,80,127}){
  felt::Voice v;v.prepare(sr);felt::VoiceSettings s;s.model=model;s.attack=1;v.strike(key,velocity,0,1,s);double energy=0;
  for(int n=0;n<int(sr*.055f);++n){if((n&31)==0)v.updatePitch(1);float x=v.tick();assert(std::isfinite(x)&&std::abs(x)<32);energy+=double(x)*x;maxPeak=std::max(maxPeak,double(std::abs(x)));maxForce=std::max(maxForce,v.visit([](const auto&e){return e.hammer.lastForce;}));maxContact=std::max(maxContact,v.visit([](const auto&e){return e.hammer.age;}));}
  assert(energy>1e-20);assert(model==5||v.visit([](const auto&e){return e.hammer.age;})<.012f);++strikes;
 }
 std::printf("PASS%d88-key/model/velocity/sample-rate strikes; finite raw peak%.3f, max force%.3fN, contact%.3fms; no safety timeout\n",strikes,maxPeak,maxForce,maxContact*1000);
 {felt::TineVoice coupled,uncoupled;coupled.prepare(48000);uncoupled.prepare(48000);felt::VoiceSettings s;s.model=0;coupled.strike(60,100,0,1,s);uncoupled.strike(60,100,0,1,s);uncoupled.linkSpring=uncoupled.linkDamping=0;uncoupled.configureTineCoupling();
  for(auto&m:coupled.modes)assert(m.y1==0&&m.y2==0);
  for(int i=0;i<4800;++i){coupled.tick();uncoupled.tick();}
  assert(std::abs(coupled.modes[1].y1)>1e-9&&uncoupled.modes[1].y1==0);
 }
 std::puts("PASS zero initial EP displacement and force-driven reciprocal tine/tone-bar energy transfer");
 for(float sr:{44100.f,48000.f,96000.f})for(int key=21;key<=108;++key){felt::tine::Mode mode;double expected=felt::bendHz(key);mode.configure(expected,4,sr,false,0);mode.drive=1;mode.pitch(1);double hz=std::acos(mode.c/(2*mode.radius))*sr/(2*felt::Pi);assert(std::abs(1200*std::log2(hz/expected))<.03);}
 std::puts("PASS double-precision modal pole tuning within0.03cent across88keys/3sample rates");
 for(int model=0;model<7;++model){felt::Voice v;v.prepare(48000);felt::VoiceSettings s;s.model=model;v.strike(60,100,0,1,s);for(int n=0;n<4800;++n)v.tick();v.release(true);v.visit([](const auto&e){for(int n=0;n<e.count;++n)assert(e.modes[n].damperTarget==1);});v.release(false);assert(v.visit([](const auto&e){return e.modes[0].damperTarget<1;}));for(int n=0;n<48000;++n)v.tick();assert(!v.state().active);}
 std::puts("PASS pedal-open resonators and progressive physical damper losses settle with Release");
 {felt::UprightVoice v;v.prepare(48000);felt::VoiceSettings s;s.model=2;v.strike(60,90,0,1,s);assert(v.count>=90&&v.modes[0].ratio!=v.modes[1].ratio&&v.modes[1].ratio!=v.modes[2].ratio);felt::UprightBody body;body.prepare(48000);double energy=0;for(int n=0;n<20000;++n){auto x=body.process(n==0?1.f:0.f,true);assert(std::isfinite(x[0])&&std::isfinite(x[1]));energy+=x[0]*x[0]+x[1]*x[1];}assert(energy>1e-8);body.reset();auto x=body.process(0,false);assert(x[0]==0&&x[1]==0);}
 std::puts("PASS distinct piano unisons, independent driven soundboard/pedal resonance and reset-to-silence");
 // The same Tone value must converge to the same audio whether set before
 // note-on or during a held note. This catches the two different tone laws.
 for(float sr:{44100.f,48000.f,96000.f})for(int model=0;model<7;++model){
  felt::Voice fresh,edited;fresh.prepare(sr);edited.prepare(sr);felt::VoiceSettings s;s.model=model;s.attack=1;s.sustain=1;s.decay=1000;s.tone=.8f;fresh.strike(60,90,0,1,s);s.tone=.1f;edited.strike(60,90,0,1,s);
  double difference=0,energy=0;
  for(int n=0;n<int(sr*.6f);++n){if(n==int(sr*.1f))edited.setTone(.8f);float a=fresh.tick(),b=edited.tick();assert(std::isfinite(a)&&std::isfinite(b));if(n>int(sr*.55f)){difference+=double(a-b)*(a-b);energy+=double(a)*a;}}
  assert(energy>1e-15&&difference/energy<1e-8);
 }
 std::puts("PASS held-note Tone converges to identical fresh-note timbre for7models/3sample rates");
 // A released initial deflection has no artificial differentiator DC step.
 for(float sr:{44100.f,48000.f,96000.f})for(int model:{5,6})for(int key:{36,60,84}){
  felt::Voice v;v.prepare(sr);felt::VoiceSettings s;s.model=model;s.attack=0;s.tone=.5;v.strike(key,90,0,1,s);float first=std::abs(v.tick()),peak=0;
  for(int i=0;i<int(sr*.05f);++i)peak=std::max(peak,std::abs(v.tick()));
  assert(peak>0&&first<peak*.35f);
 }
 std::puts("PASS plucked starts have no artificial first-sample impulse at3registers/3sample rates");
 // The dry circuit must not thin out low notes when the host changes rate.
 float minGain=10,maxGain=0;
 for(float sr:{44100.f,48000.f,96000.f}){felt::Colour c;c.prepare(sr);felt::ColourSettings s;s.drift=s.motionDepth=0;c.update(s,88);double energy=0,input=0;
  for(int i=0;i<int(sr*.6f);++i){float x=.1f*std::sin(2*felt::Pi*30*i/sr);auto y=c.process(x,x);if(i>int(sr*.2f)){energy+=double(y[0])*y[0];input+=double(x)*x;}}
  float gain=std::sqrt(energy/input);minGain=std::min(minGain,gain);maxGain=std::max(maxGain,gain);
 }
 assert(minGain>.8f&&maxGain/minGain<1.02f);
 std::puts("PASS dry30Hz circuit response remains consistent across44.1/48/96kHz");
 for(float sr:{44100.f,48000.f,96000.f})for(double hz:{55.,261.6255653,1760.}){
  felt::TineToneBar pair;double mass=.02,w=2*felt::Pi*hz;pair.configure(hz,hz*1.0028,mass,mass,mass*w*w*.003,0,1,1,sr);pair.x[0]=.001;pair.x[1]=-.0003;pair.v[0]=.03;double initial=pair.energy(),error=0;
  for(int i=0;i<int(sr*2);++i){pair.tick(0);error=std::max(error,std::abs(pair.energy()/initial-1));}
  assert(error<1e-8);
  double old=pair.energy();for(int i=0;i<1000;++i){pair.tick(0,.999,.997);assert(pair.energy()<=old*(1+1e-12));old=pair.energy();}
 }
 std::puts("PASS unforced Tine/tone-bar energy conserved within1e-8 and damping never increases it across3rates/3registers");
 {felt::TinePickup2D one,two;one.configure(.0004,.85,.07,48000);two.configure(.0004,.85,.07,48000);double difference=0,energy=0,sum=0;
  for(int i=0;i<9600;++i){float phase=2*felt::Pi*100*i/48000;float v=.00008f*std::sin(phase),h=.00004f*std::cos(phase);float a=one.tick(v,0),b=two.tick(v,h);if(i>=4800){difference+=double(a-b)*(a-b);energy+=double(a)*a;sum+=b;}}
  assert(difference>energy*.001&&std::abs(sum)<1e-4);two.reset();assert(two.tick(0,0)==0);
 }
 std::puts("PASS both Tine pickup coordinates affect output, periodic flux has zero net voltage and reset is silent");
 {felt::tine::Hammer hammer;hammer.strike(2,.004,.4);hammer.loss=.08;for(int i=0;i<600&&hammer.active;++i)hammer.tick(0,1.f/48000);assert(hammer.dissipated>0&&!hammer.active);}
 std::puts("PASS hysteretic hammer contact records positive dissipated work and separates");
 // Behavioural check, independent of exact voicing constants: stronger upper
 // register strikes must shed held energy faster, without changing pitch or
 // sustain/release ownership. Evaluate separated windows over three rates.
 for(float rate:{44100.f,48000.f,96000.f}){
  double ratios[2]{};
  for(int j=0;j<2;++j){felt::Voice v;v.prepare(rate);felt::VoiceSettings s;s.model=0;s.tone=.60f;s.body=.62f;s.character3=.36f;s.character4=.20f;s.attack=1;s.decay=1800;s.sustain=.92f;s.hammer=.32f;s.velocity=.65f;v.strike(72,j?95:35,0,1,s);double early=0,late=0;
   for(int i=0;i<int(rate*1.2f);++i){double x=v.tick();assert(std::isfinite(x));if(i>=int(rate*.005f)&&i<int(rate*.065f))early+=x*x;if(i>=int(rate*.9f))late+=x*x;}
   assert(early>0&&late>0&&v.state().down&&v.state().active);ratios[j]=late/early;
  }
  assert(10*std::log10(ratios[1]/ratios[0])<-.65);
 }
 std::puts("PASS stronger upper Tine strikes lose held energy faster at3rates without ending held notes");

 // Charge-difference readout: no startup impulse, no periodic DC current,
 // greater harmonic content for a larger displacement, and bounded response.
 for(float rate:{44100.f,48000.f,96000.f}){
  double harmonic[2]{};
  for(int k=0;k<2;++k){felt::WurliPickup pickup;pickup.configure(.0007f,.3f,rate);assert(pickup.tick(0,200)==0);double sum=0,fundC=0,fundS=0,secondC=0,secondS=0;
   for(int i=0;i<int(rate*.6f);++i){double phase=2*felt::Pi*200*i/rate;float y=pickup.tick((k?.00035f:.000035f)*std::sin(phase),200);assert(std::isfinite(y)&&std::abs(y)<4);if(i>=int(rate*.5f)){sum+=y;fundC+=y*std::cos(phase);fundS+=y*std::sin(phase);secondC+=y*std::cos(2*phase);secondS+=y*std::sin(2*phase);}}
   assert(std::abs(sum)<.002);harmonic[k]=(secondC*secondC+secondS*secondS)/(fundC*fundC+fundS*fundS);pickup.reset();assert(pickup.tick(0,200)==0);
  }
  assert(harmonic[1]>harmonic[0]*3);
 }
 std::puts("PASS Wurli charge readout resets silently, has near-zero periodic DC and stronger upper content with larger motion at3rates");

 // Upright: pedal and damper laws use physical time rather than per-sample
 // constants. Compare equal elapsed times across three renderer rates.
 double pedalAt10ms=-1;
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::UprightBody body;body.prepare(rate);
  for(int i=0;i<int(rate*.01f);++i)body.process(0,true);
  if(pedalAt10ms<0)pedalAt10ms=body.pedal;
  assert(std::abs(body.pedal-pedalAt10ms)<.0002);
  double closedT60=-6.9077553/(rate*std::log(double(body.closedDamper)));
  assert(std::abs(closedT60-.032)<.00002);
  for(int key:{36,60,84})for(int velocity:{35,65,95}){
   felt::UprightVoice voice;voice.prepare(rate);felt::VoiceSettings settings;settings.model=2;
   voice.strike(key,velocity,0,1,settings);
   for(int i=0;i<int(rate*.02f);++i)assert(std::isfinite(voice.tick()));
   assert(!voice.hammer.active&&voice.hammer.dissipated>0);

  }
 }
 std::puts("PASS Upright dissipative contacts and sample-rate-independent pedal/damper time at3rates");

 // The audio-rate hammer/string contact must not create strike energy.
 // Check both bass and treble, including the previously worst strong C4.
 for(float rate:{44100.f,48000.f,96000.f})for(int note:{41,48,60,64,84,103}){
  felt::UprightVoice v;v.prepare(rate);
  felt::VoiceSettings s;s.model=2;s.hammer=.85f;s.velocity=.65f;
  v.strike(note,127,0,1,s);
  const double initial=.5*v.hammer.mass*v.hammer.velocity*v.hammer.velocity;
  double stringEnergy=0;
  for(int frame=0;frame<int(rate*.013f);++frame){
   v.tick();
   stringEnergy=0;
   for(int i=0;i<v.count;++i){
    const auto& m=v.modes[i];
    if(m.muted)continue;
    const double quadrature=(m.phaseCos*m.y1-m.radius*m.damping*m.y2)/m.phaseSin;
    const double omega=m.angle*rate;
    stringEnergy+=.5*v.modalMass*omega*omega*(m.y1*m.y1+quadrature*quadrature);
   }
   const double hammerEnergy=.5*v.hammer.mass*v.hammer.velocity*v.hammer.velocity;
   assert(stringEnergy+hammerEnergy<=initial*1.003);
  }
  assert(!v.hammer.active&&v.hammer.age<.012f&&stringEnergy>initial*.05);
 }
 std::puts("PASS Upright reciprocal hammer and string strike transfer stays within launch energy at3rates/6keys");

 // The common unison velocity projection must dissipate normalized kinetic
 // energy, with pitch bending and damper contact too. Differential motion is
 // preserved; do not infer this from merely finite oscillator samples.
 for(float rate:{44100.f,48000.f,96000.f})for(int note:{41,43,48,50,60})for(float bend:{.8f,1.f,1.3f}){
  felt::UprightVoice v;v.prepare(rate);felt::VoiceSettings s;s.model=2;v.strike(note,95,0,1,s);v.updatePitch(bend);
  auto energy=[&](){double total=0;for(int i=0;i<v.count;++i){auto&m=v.modes[i];if(m.muted)continue;double p=(m.phaseCos*m.y1-m.radius*m.damping*m.y2)/m.phaseSin;total+=m.angle*m.angle*(m.y1*m.y1+p*p);}return total;};
  for(int i=0;i<v.count;++i){auto&m=v.modes[i];m.y1=1e-5*std::sin(i*.37);m.y2=1e-5*std::cos(i*.63);m.damping=.999f;}
  double before=energy();for(int i=0;i<100;++i){double old=energy();v.coupleUnisons();assert(energy()<=old*(1+1e-10));}assert(energy()<before);
 }
 std::puts("PASS Upright reduced bridge coupling is passive in normalized modal energy at3rates/5keys/3bends");
 // Main radiated output must remain present throughout the keyboard rather
 // than reverting to direct motion readout in the treble.
 for(float rate:{44100.f,48000.f,96000.f})for(int note:{36,48,60,72,84}){
  felt::UprightVoice v;felt::UprightBody b;v.prepare(rate);b.prepare(rate);felt::VoiceSettings s;s.model=2;s.attack=1;v.strike(note,80,0,1,s);double leakage=0,radiation=0;
  for(int i=0;i<int(rate*.15f);++i){float x=v.tick();auto y=b.process(v.bodySend,false);assert(std::isfinite(y[0])&&std::isfinite(y[1]));leakage+=x*x;radiation+=y[0]*y[0]+y[1]*y[1];}
  assert(leakage>0&&radiation>leakage*100);
  b.reset();assert(b.process(0,false)==(std::array<float,2>{0,0}));
 }
 std::puts("PASS Upright main-radiation path exceeds direct leakage by20dB across5registers/3rates and resets silently");

 // Upright's damper and amplitude release must both operate, including after
 // pedal-up. Reject a cut tail or a tone held indefinitely by a repeated off.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::UprightVoice v;
  v.prepare(rate);
  felt::VoiceSettings settings;
  settings.model=2;
  settings.attack=1;
  settings.sustain=1;
  settings.release=420;
  v.strike(60,85,0,1,settings);
  for(int i=0;i<int(rate*.2f);++i)v.tick();
  v.release(true);
  for(int i=0;i<int(rate*.1f);++i)v.tick();
  assert(v.active&&v.sustained);
  v.release(false);
  const int contactFrames=int(rate*.032f);
  const float landing=v.modes[0].damperTarget;
  assert(v.damperContactSamples==contactFrames);
  double tail=0;
  for(int i=0;i<int(rate*.44f);++i){
   if(i==int(rate*.15f))v.release(false);
   v.tick();
   if(i==contactFrames-2)assert(v.modes[0].damperTarget==landing);
   if(i==contactFrames-1)assert(v.modes[0].damperTarget>landing&&v.modes[0].damperTarget<1.f);
   if(i>int(rate*.12f)&&i<int(rate*.25f))tail+=v.bodySend*v.bodySend;
  }
  assert(tail>1e-8&&!v.active&&v.bodySend==0);
 }
 std::puts("PASS Upright pedal-up damper contact changes phase after32ms at3rates, leaving a passive finite tail");

 // The case pulse must come from contact force, decay, and disappear on reuse.
 // Measure elapsed seconds at each renderer rate rather than frame counts.
 double caseEnergyAt48k=0;
 for(float rate:{48000.f,44100.f,96000.f}){
  felt::upright::Action action;
  action.prepare(rate);
  action.strike(60,.2f,.0018f,3.f);
  for(int i=0;i<int(.002f*rate);++i)assert(action.tick(0,1/rate)==0);
  double energy=0;
  for(int i=0;i<int(.2f*rate);++i){
   const float time=i/rate;
   const float force=time<.0012f?10*std::sin(felt::Pi*time/.0012f):0;
   const float pressure=action.tick(force,1/rate);
   assert(std::isfinite(pressure));
   if(time<.06f)energy+=pressure*pressure/rate;
   if(time>.185f)assert(pressure==0);
  }
  assert(energy>1e-8);
  if(rate==48000)caseEnergyAt48k=energy;
  else assert(std::abs(energy/caseEnergyAt48k-1)<.05);
  action.reset();assert(action.tick(0,1/rate)==0);
 }
 std::puts("PASS Upright case action requires strike force and settles, with consistent response at3rates");
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::UprightBody withRim,withoutRim;
  withRim.prepare(rate);withoutRim.prepare(rate);
  withoutRim.rim.farDelay=withoutRim.rim.nearDelay;
  double changedSide=0;
  for(int i=0;i<int(.1f*rate);++i){
   const float input=.1f*std::sin(float(i)*felt::Pi*2*330/rate);
   const auto a=withRim.process(input,false);
   const auto b=withoutRim.process(input,false);
   assert(std::abs((a[0]+a[1])-(b[0]+b[1]))<2e-7f);
   const float side=(a[0]-a[1])-(b[0]-b[1]);
   changedSide+=side*side/rate;
  }
  assert(changedSide>1e-7);
  withRim.reset();assert(withRim.process(0,false)==(std::array<float,2>{0,0}));
 }
 std::puts("PASS Upright stereo projection changes side pressure, preserves mono and resets at3rates");

 // Measure the additional open-string output of a held, tuned driving note.
 // Using open minus closed removes the soundboard's direct coloration.
 auto pedalResonancePower=[](float rate,int pitch,float tuning){
  felt::UprightBody open,closed;
  open.prepare(rate);closed.prepare(rate);
  open.setTuning(tuning);closed.setTuning(tuning);
  const double hz=felt::bendHz(pitch,tuning);
  double energy=0,inputEnergy=0;
  for(int i=0;i<int(.65f*rate);++i){
   const float input=.25f*std::sin(2*double(felt::Pi)*hz*i/rate);
   const auto a=open.process(input,true);
   const auto b=closed.process(input,false);
   if(i>int(.3f*rate)){
    const double delta=.5*(a[0]+a[1]-b[0]-b[1]);
    energy+=delta*delta;inputEnergy+=input*input;
   }
  }
  return energy/inputEnergy;
 };
 double minPower=1,maxPower=0;
 for(int note=39;note<51;++note){
  const double power=pedalResonancePower(48000,note,440);
  assert(power>1e-5&&power<1e-2);
  minPower=std::min(minPower,power);maxPower=std::max(maxPower,power);
 }
 const double span=10*std::log10(maxPower/minPower);
 assert(span<3);
 const double at48=pedalResonancePower(48000,43,440);
 double largestRateError=0;
 for(float rate:{44100.f,96000.f})
  largestRateError=std::max(largestRateError,std::abs(10*std::log10(pedalResonancePower(rate,43,440)/at48)));
 assert(largestRateError<1);
 const double retuned=pedalResonancePower(48000,43,452);
 assert(std::abs(10*std::log10(retuned/at48))<1);
 std::printf("PASS Upright pedal resonates on all12semitones (%.2f dB span), consistent over3rates (%.2f dB) and follows tuning\n",span,largestRateError);

 // Vary the radiation buildup without changing the underlying strike. Check
 // both the early acoustic output and the later convergence on a held note.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::UprightVoice modeled,prompt;
  modeled.prepare(rate);prompt.prepare(rate);
  felt::VoiceSettings settings;
  settings.model=2;settings.attack=1;settings.sustain=1;
  modeled.strike(60,85,0,1,settings);prompt.strike(60,85,0,1,settings);
  prompt.radiationRise.reset();
  double earlyModeled=0,earlyPrompt=0,lateDelta=0,lateSignal=0;
  for(int i=0;i<int(.38f*rate);++i){
   modeled.tick();prompt.tick();
   assert(modeled.hammer.lastForce==prompt.hammer.lastForce);
   assert(modeled.modes[0].y1==prompt.modes[0].y1);
   const float a=modeled.bodySend,b=prompt.bodySend;
   if(i<int(.016f*rate)){earlyModeled+=a*a;earlyPrompt+=b*b;}
   if(i>int(.33f*rate)){lateDelta+=(a-b)*(a-b);lateSignal+=b*b;}
  }
  assert(earlyModeled>0&&earlyModeled<earlyPrompt*.85);
  assert(lateSignal>0&&lateDelta<lateSignal*1e-8);
  modeled.stop();assert(modeled.tick()==0&&modeled.bodySend==0);
 }
 std::puts("PASS Upright bridge radiation builds early without changing hammer or string state and converges at3rates");

 // Supported tine modes follow pitch, while independently excited wooden
 // body poles stay fixed. Catch accidental whole-instrument pitch shifting.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::ThumbVoice voice; voice.prepare(rate);
  felt::VoiceSettings settings;settings.model=6;
  voice.strike(60,85,0,1,settings);
  assert(voice.count==11&&!voice.hammer.active);
  const double first=voice.modes[0].c;
  double fixed[3]{};
  for(int i=0;i<6;++i){
   assert(voice.modes[i].pitchable);
   assert(std::abs(voice.modes[i].ratio-felt::thumb::BeamRatios[i])<1e-5f);
  }
  for(int i=0;i<3;++i){assert(!voice.modes[i+8].pitchable);fixed[i]=voice.modes[i+8].c;}
  voice.updatePitch(1.25f);
  assert(voice.modes[0].c!=first);
  for(int i=0;i<3;++i)assert(voice.modes[i+8].c==fixed[i]);
  for(int i=0;i<int(rate*.4f);++i)assert(std::isfinite(voice.tick()));
  voice.stop();assert(voice.tick()==0);
 }
 std::puts("PASS Thumb supported-beam pitch, fixed body poles and reset at3rates");

 // Reference-guided Thumb keeps natural loss with a held sustain envelope,
 // and retains monotonic touch without changing pitch or starting with a step.
 for(float rate:{44100.f,48000.f,96000.f}){
  double previous=0;
  for(int velocity:{35,65,100}){
   felt::ThumbVoice voice;voice.prepare(rate);felt::VoiceSettings settings;
   settings.model=6;settings.body=.5f;settings.character4=.2f;
   settings.attack=1;settings.decay=1800;settings.sustain=1;
   voice.strike(64,velocity,0,1,settings);
   double early=0,late=0;
   for(int i=0;i<int(rate*.5f);++i){
    double sample=voice.tick();assert(std::isfinite(sample));
    if(i>=int(rate*.05f)&&i<int(rate*.10f))early+=sample*sample;
    if(i>=int(rate*.45f))late+=sample*sample;
   }
   assert(early>previous&&late<early*.03);previous=early;
  }
 }
 std::puts("PASS Thumb monotonic soft/medium/hard touch and natural held decay at3rates");

 // High metal partials must be attacks, not a lasting bell oscillator.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::ThumbVoice voice;voice.prepare(rate);felt::VoiceSettings settings;
  settings.model=6;settings.body=.5f;settings.character4=.2f;voice.strike(64,80,0,1,settings);
  assert(voice.modes[1].radius<voice.modes[0].radius);
  assert(voice.modes[6].pitchable&&voice.modes[7].pitchable);
  assert(voice.modes[6].ratio>1.05f&&voice.modes[6].ratio<1.10f);
  double tail=0,initial=0;
  for(int i=0;i<int(rate*.12f);++i){
   double z=voice.modes[7].tick();
   if(i<int(rate*.015f))initial+=z*z;
   if(i>=int(rate*.105f))tail+=z*z;
  }
  assert(initial>0&&tail<initial*.001);
 }
 std::puts("PASS Thumb short metal attack loses >30dB energy by105ms at3rates");

 // The finite release rises in seconds at every host rate and starts afresh
 // on voice reuse; it must not dull the entire tail with a new lowpass.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::ThumbVoice voice;voice.prepare(rate);felt::VoiceSettings settings;settings.model=6;
  voice.strike(64,72,0,1,settings);assert(voice.metalRise==0);
  const float time=.0009f+.0012f*(1-voice.velocity);
  const int frames=int(rate*.003f);
  for(int i=0;i<frames;++i)voice.tick();
  const float expected=1-std::exp(-frames/(rate*time));
  assert(std::abs(voice.metalRise-expected)<.0001f);
  for(int i=0;i<int(rate*.03f);++i)voice.tick();
  assert(voice.metalRise>.9999f);
  voice.stop();assert(voice.metalRise==0&&voice.tick()==0);
  voice.strike(64,72,0,1,settings);assert(voice.metalRise==0);
 }
 std::puts("PASS Thumb finite metal release uses seconds and resets on voice reuse at3rates");

 // Upper keys may sustain their fundamental without a long metallic whistle.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::ThumbVoice low,high;low.prepare(rate);high.prepare(rate);
  felt::VoiceSettings settings;settings.model=6;settings.body=.5f;settings.character4=.2f;
  low.strike(64,72,0,1,settings);high.strike(72,72,0,1,settings);
  assert(high.modes[0].radius>low.modes[0].radius);
  assert(high.modes[1].radius<low.modes[1].radius);
  assert(high.modes[6].ratio<low.modes[6].ratio&&high.modes[6].ratio>1.05f);
  float relative[2]{};
  for(int i=0;i<2;++i){
   felt::ThumbVoice v;v.prepare(rate);v.strike(64,i==0?35:110,0,1,settings);
   relative[i]=std::abs(v.modes[6].y1*v.modes[6].output)/std::abs(v.modes[0].y1*v.modes[0].output);
  }
  assert(relative[0]<relative[1]);
 }
 std::puts("PASS Thumb upper metal settles sooner while high fundamentals sustain; adjacent grain follows touch at3rates");

 // Finite tangent pulses have a rate-independent area and never ring/rebound
 // as an unrelated piano hammer. Pickup paths stay individually observable.
 for(float rate:{44100.f,48000.f,96000.f})for(int key:{29,60,88}){
  felt::ClavVoice v;v.prepare(rate);felt::VoiceSettings settings;settings.model=4;
  v.strike(key,85,0,1,settings);double integral=0;
  const double expected=v.hammer.impulse;
  for(int i=0;i<int(rate*.004f);++i){float force=v.hammer.tick(0,1/rate);assert(force>=0&&std::isfinite(force));integral+=force/rate;}
  assert(!v.hammer.active&&std::abs(integral/expected-1)<.02);
  if(key==60){assert(std::abs(v.centerPickup[4])<1e-5&&std::abs(v.bridgePickup[4])>.1f);}
 }
 std::puts("PASS Clav finite tangent impulse area within2percent at3rates/3registers; distinct two-pickup combs");
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::ClavVoice v;v.prepare(rate);felt::VoiceSettings settings;settings.model=4;
  v.strike(60,85,0,1,settings);
  for(int i=0;i<int(rate*.04f);++i){if(!(i&31))v.updatePitch(1);assert(std::isfinite(v.tick()));}
  v.release(true);assert(v.releaseTarget==1&&v.sustained);
  v.release(false);assert(v.releaseTarget<1&&!v.hammer.active);
  for(int i=0;i<int(rate*.04f);++i)assert(std::isfinite(v.tick()));
  assert(std::abs(v.releaseRatio-.8408964153f)<.00003f);
  v.updatePitch(std::exp2(2.f/12));
  const auto&m=v.modes[0];double hz=std::acos(m.c/(2*m.radius))*rate/(2*felt::Pi);
  double wanted=felt::bendHz(60)*std::exp2(-1.f/12);
  assert(std::abs(1200*std::log2(hz/wanted))<.1);
  v.stop();assert(v.releaseRatio==1&&v.previousCenter==0&&v.previousBridge==0&&v.tick()==0);
 }
 std::puts("PASS Clav virtual pedal, yarn release pitch, host bend and reset at3rates");

 for(float rate:{44100.f,48000.f,96000.f})for(int key:{41,60,84}){
  double previous=0;
  for(int velocity:{35,65,100}){
   felt::ClavVoice v;v.prepare(rate);felt::VoiceSettings settings;
   settings.model=4;settings.attack=1;settings.sustain=1;v.strike(key,velocity,0,1,settings);
   double energy=0;
   for(int frame=0;frame<int(rate*.12f);++frame){double x=v.tick();assert(std::isfinite(x));if(frame>int(rate*.015f))energy+=x*x;}
   assert(energy>previous*1.5);previous=energy;
  }
 }
 std::puts("PASS Clav soft/medium/firm touch energy is monotonic at3rates/3registers");

 // Warm pickup/preamp voicing keeps a usable bright end and a consistent
 // force-compliance taper rather than switching models or changing presets.
 for(float rate:{44100.f,48000.f,96000.f}){
  felt::ClavVoice v;v.prepare(rate);felt::VoiceSettings settings;settings.model=4;
  v.strike(63,72,0,1,settings);
  assert(v.pickupBlend>.05f&&v.pickupBlend<.55f);
  double scaled[2]{};
  for(int i=0;i<2;++i){auto&m=v.modes[i];scaled[i]=m.drive*v.modalMass/(m.input*v.dt*v.dt);}
  assert(std::abs(scaled[0]-1)<1e-5&&std::abs(scaled[1]-1/std::sqrt(2.))<1e-5);
  v.setTone(1,true);
  double cutoff=-std::log(1-v.brightness)*rate/(2*felt::Pi);
  assert(std::abs(cutoff-9000)<2);
 }
 std::puts("PASS Clav compliant mode taper, bounded pickup blend and9kHz bright-end Tone at3rates");

 // A bridge-side palm dissipates modal velocity; no instability is allowed
 // when damping is strong, release loss is active or the pitch is bent.
 for(float rate:{44100.f,48000.f,96000.f})for(int key:{36,60,84})for(float mute:{.2f,.85f})for(float bend:{1.f,1.122462f}){
  felt::MutedVoice v;v.prepare(rate);felt::VoiceSettings settings;settings.model=5;settings.character4=mute;
  v.strike(key,80,0,1,settings);v.updatePitch(bend);
  for(int n=0;n<v.count;++n)if(!v.modes[n].muted){
   auto&m=v.modes[n];m.y1=.0001*std::sin(n*.47)/(n+1);m.damping=.985f;
   const double vel=.01*std::cos(n*1.13)/(n+1);
   m.y2=(v.palmCos[n]*m.y1-vel/v.palmVelocityScale[n])/(m.radius*m.damping);
  }
  auto energy=[&](){double e=0;for(int n=0;n<v.count;++n)if(!v.modes[n].muted){double vel=v.palmVelocity(n);e+=vel*vel;}return e;};
  felt::MutedVoice reference=v;
  for(int patch=0;patch<2;++patch){
   double velocity=0;for(int n=0;n<reference.count;++n)velocity+=reference.palmShape[patch][n]*reference.palmVelocity(n);
   double impulse=reference.palmFactor[patch]*velocity;
   for(int n=0;n<reference.count;++n)if(!reference.modes[n].muted)reference.modes[n].y2+=reference.palmShape[patch][n]*impulse*reference.palmStateScale[n]/reference.modes[n].damping;
  }
  const double before=energy();v.dampPalm();const double after=energy();
  for(int n=0;n<v.count;++n)assert(std::abs(v.modes[n].y2-reference.modes[n].y2)<1e-12);
  assert(std::isfinite(after)&&after<=before*(1+1e-10)&&after<before);
 }
 std::puts("PASS Muted palm projections dissipate energy at3rates/3registers/2pressures/2bends");
 for(float rate:{44100.f,48000.f,96000.f}){
  double relativeTail[2]{};
  for(int i=0;i<2;++i){
   felt::MutedVoice v;v.prepare(rate);felt::VoiceSettings settings;settings.model=5;
   settings.attack=1;settings.sustain=1;settings.character4=i==0?.05f:.8f;
   v.strike(60,85,0,1,settings);double early=0,late=0;
   for(int frame=0;frame<int(rate*.35f);++frame){double x=v.tick();assert(std::isfinite(x));if(frame<int(rate*.04f))early+=x*x;if(frame>int(rate*.20f))late+=x*x;}
   relativeTail[i]=late/early;
  }
  assert(relativeTail[1]<relativeTail[0]);
 }
 std::puts("PASS Muted increased palm pressure reduces normalized held tail at3rates");

}
