// Donut Theory by kim. Two oscillators + sub/FM/noise -> per-voice SVF and ADSRs
// -> drive -> stereo chorus/delay -> master. MIDI trigger ownership prevents
// overlapping chords from releasing each other's notes. No allocation in process.
#include "mpc_plugin.h"
#include "dsp/oscillator.h"
#include "dsp/svf.h"
#include "dsp/adsr.h"
#include "dsp/lfo.h"
#include "dsp/smoother.h"
#include "dsp/delay_line.h"
#include "dsp/noise.h"
#include "Catalog.h"
#include "ChordVoicing.h"
#include "WowFlutter.h"
#include <array>
#include <algorithm>
#if defined(_WIN32)
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#else
#include <pthread.h>
#endif
static std::uintptr_t donutThreadToken(){
#if defined(_WIN32)
    return GetCurrentThreadId();
#else
    return (std::uintptr_t)pthread_self();
#endif
}
static_assert(std::atomic<std::uintptr_t>::is_always_lock_free,"Audio thread tokens must be lock-free");

using namespace mpc::dsp;
class DonutTheory final : public mpc::Plugin {
public:
    static constexpr int N=16;
    static mpc::PluginInfo metadata() {
        mpc::PluginInfo i; i.name="Donut Theory";i.vendor="kim";
        i.uniqueId=MPC_FOURCC('K','r','q','l');i.version=10001;i.isInstrument=true;i.numInputs=0;i.numOutputs=2;return i;
    }
    DonutTheory():mpc::Plugin(metadata()) {
        declareParameters(*this);
        for(int i=0;i<64;++i) addProgram(kPatchNames[i],std::vector<float>(kPatches[i],kPatches[i]+kSoundPreset+1));
        // Prefix programs intentionally exclude input/chord settings.
        for(int i=0;i<=kSoundPreset;++i)set(i,kPatches[0][i]);
        set(kOscPair,getInt(kWave1)*4+getInt(kWave2));
    }
#ifdef DONUT_TOUCH_DIAGNOSTICS
    // QA-only raw host writes. No allocation, I/O or locks on the audio thread.
    std::array<std::atomic<unsigned>,4> touchOnWrites{},touchOffWrites{},touchStart{},touchDuration{};
    std::atomic<unsigned> diagnosticFrames{0};
    void traceTouchWrite(int index,float value) {
        if(index>=kTouch1&&index<=kTouch4) {
            const int slot=index-kTouch1;
            (value>.5f?touchOnWrites:touchOffWrites)[slot].fetch_add(1,std::memory_order_relaxed);
            const unsigned now=diagnosticFrames.load(std::memory_order_relaxed);
            if(value>.5f)touchStart[slot].store(now);else touchDuration[slot].store(now-touchStart[slot].load());
            displayPending_.store(true,std::memory_order_release);
        }
    }
#endif
    // Host programs and the in-skin selector share the same atomic parameter.
    int currentProgram() const { return getInt(kSoundPreset); }
    void setProgram(int index) {
        if(index<0||index>=64)return;
        for(int i=1;i<=kSoundPreset;++i)setFromEngine(i,kPatches[index][i]);
        setFromEngine(kFXPreset,0);setFromEngine(kOscPair,getInt(kWave1)*4+getInt(kWave2));applySoundExtras(index);
        stateLoadPending_.store(true,std::memory_order_release);
    }
    const std::vector<uint8_t>& internalSaveState(){
        // Screen auditions/commands/playing lamps are gestures, not musical
        // state. End auditions via atomic gates; never touch audio-owned voices
        // from the saving thread. Physical MIDI ownership stays intact.
        for(int slot=0;slot<16;++slot)setFromEngine(kChordTouches[slot],0);
        for(int page=0;page<4;++page)setFromEngine(kChordPlayingPage1+page,0);
        for(int command=0;command<4;++command){setFromEngine(kSoundPrev+command,0);setFromEngine(kSlotsMinus+command,0);}
        setFromEngine(kTouchStop,0);setFromEngine(kPadLearn,0);
        return mpc::Plugin::internalSaveState();
    }
    bool internalLoadState(const uint8_t* data,size_t size) {
        stateLoading_.store(true,std::memory_order_release);
        if(!mpc::Plugin::internalLoadState(data,size)){stateLoading_.store(false,std::memory_order_release);return false;}
        // SDK state v1/v2 starts with a little-endian parameter count at byte 8.
        const uint32_t count=uint32_t(data[8])|(uint32_t(data[9])<<8)|(uint32_t(data[10])<<16)|(uint32_t(data[11])<<24);
        if(count<=83)set(kDrift,0);
        if(count<=85)set(kLowCut,0);
        if(count<=86)set(kOutputTrim,0);
        for(int slot=0;slot<4;++slot)if(count<=uint32_t(kChordOctave1+slot))set(kChordOctave1+slot,2);
        for(int index=91;index<kNumParams;++index)if(count<=uint32_t(index))set(index,param(index).def);
        if(count<=77)set(kPadMap,0); // Existing projects retain Trigger Base routing.
        stateLoading_.store(false,std::memory_order_release);return true;
    }
    void parameterChanged(int index,float value) override {
        if(index>=kEditRoot1&&index<kEditRoot1+16&&!stateLoading_.load(std::memory_order_acquire)){
            const bool engineReadback=proxySyncIndex_.load(std::memory_order_acquire)==index&&proxySyncThread_.load(std::memory_order_acquire)==donutThreadToken();
            if(!engineReadback){const int offset=index-kEditRoot1,target=chordField(getInt(kMainChordPage)*4+offset/4,offset%4);if(get(target)!=value)setFromEngine(target,value);}
        }
        // Queue only. Parameter writes can arrive on the audio thread.
        if(param(index).kind==mpc::Param::Kind::Choice||
           (index>=kRoot1&&index<=kVoicing4)||(index>=kPadNote1&&index<=kPadNote4))displayPending_.store(true,std::memory_order_release);
    }
    void markRenderThread(){
        const auto token=donutThreadToken();
        for(auto& seen:renderThreads_)if(seen.load(std::memory_order_relaxed)==token)return;
        for(auto& seen:renderThreads_){std::uintptr_t empty=0;if(seen.compare_exchange_strong(empty,token,std::memory_order_relaxed))return;}
        renderThreadOverflow_.store(true,std::memory_order_relaxed);
    }
    void flushFromValueRead(){
        // MPC polls parameter values without requesting display strings. Drain
        // notifications there, but never on a thread that renders this plugin.
        if(renderThreadOverflow_.load(std::memory_order_relaxed))return;
        const auto token=donutThreadToken();
        for(auto& seen:renderThreads_)if(seen.load(std::memory_order_relaxed)==token)return;
        flushHostUpdates();
    }
    void requestHostIdle() {
        // Native MPC skins have no VST editor window. Ask for the documented
        // idle callback so queued MIDI-learn updates are drained on the UI side.
        if(host_&&effect_)host_(effect_,audioMasterNeedIdle_Deprecated,0,0,nullptr,0.f);
    }
    void flushHostUpdates() {
        if(flushing_.test_and_set(std::memory_order_acquire))return;
        for(int i=0;i<kNumParams;++i)
            if(changed_[i].exchange(false,std::memory_order_acq_rel))notifyHostParameterChanged(i);
        if(displayPending_.exchange(false,std::memory_order_acq_rel))requestDisplayUpdate();
        flushing_.clear(std::memory_order_release);
    }
    void prepare(double sr,int) override {
        sr_=float(sr);dcCoeff_=std::exp(-2.f*3.14159265359f*5.f/sr_);for(auto& f:lowCut_)f.setSampleRate(sr);wowFlutter_.prepare(sr/16.0);
        for(auto& v:voices_){v.a.setSampleRate(sr);v.b.setSampleRate(sr);v.sineA.setSampleRate(sr);v.sineB.setSampleRate(sr);v.sub.setSampleRate(sr);v.fm.setSampleRate(sr);v.filter.setSampleRate(sr);v.amp.setSampleRate(sr);v.env.setSampleRate(sr);v.pitch.setTime(10,sr);}
        tremolo_.setSampleRate(sr);lfo_.setSampleRate(sr);vibrato_.setSampleRate(sr);vibrato_.setFrequency(5.2f);
        drift_.setSampleRate(sr);drift_.setFrequency(.13f);chorusLfo_.setSampleRate(sr);chorusLfo_.setFrequency(.31f);
        for(auto& d:delay_)d.prepare(int(sr*3)+2);
        chorus_.prepare(int(sr*.05)+2);
        for(int i=0;i<kNumParams;++i){smooth_[i].setTime(15,sr/16.0);smooth_[i].snap(get(i));}
        gain_.setTime(10,sr);gain_.snap(dbToGain(get(kVolume)));
        delayTime_.setTime(100,sr/16.0);delayTime_.snap(sr*.5f);
        reset();
    }
    void reset() override {
        clearNotes();for(int i=0;i<16;++i)for(int j=0;j<4;++j)lastTouchChord_[i][j]=getInt(chordField(i,j));lastSlotCount_=activeSlots();lastMainPage_=getInt(kMainChordPage);lastLowCut_=-1;cachePadMapping();for(auto& d:delay_)d.reset();chorus_.reset();
        wowFlutter_.reset();tapePitch_=0;tremolo_.reset();lfo_.reset();vibrato_.reset();drift_.reset();chorusLfo_.reset();
        setFromEngine(kOscPair,getInt(kWave1)*4+getInt(kWave2));lastPair_=getInt(kOscPair);
        lastFX_=getInt(kFXPreset);lastPatch_=getInt(kSoundPreset);for(int bank=0;bank<4;++bank)lastProgressions_[bank]=getInt(kBankProgressions[bank]);
        lastInput_=getInt(kInputMode);lastMode_=getInt(kVoiceMode);lastTrigger_=getInt(kTrigger);
        for(int i=0;i<kNumParams;++i){p_[i]=get(i);smooth_[i].snap(p_[i]);}
        gain_.snap(dbToGain(get(kVolume)));control_=0;dcIn_=dcOut_=0;for(int i=0;i<4;++i)arrows_[i]=getInt(kSoundPrev+i);
        // Publish appended defaults even when unchanged, so lazy native pages
        // receive their initial values before the first edit. Queue only here.
        for(int i=91;i<kNumParams;++i)changed_[i].store(true,std::memory_order_release);
        displayPending_.store(true,std::memory_order_release);
    }
    void process(mpc::ProcessContext& ctx) override {
        markRenderThread();
#ifdef DONUT_TOUCH_DIAGNOSTICS
        diagnosticFrames.fetch_add(unsigned(ctx.numFrames),std::memory_order_relaxed);
#endif
        for(int i=0;i<4;++i){if(getInt(kSoundPrev+i)){const int target=i<2?kSoundPreset:kBankProgressions[getInt(kMainChordPage)];const int count=i<2?64:32;setFromEngine(target,float((getInt(target)+(i%2?1:-1)+count)%count));setFromEngine(kSoundPrev+i,0);arrows_[i]=0;}}
        if(getInt(kSoundPreset)!=lastPatch_){lastPatch_=getInt(kSoundPreset);for(int i=1;i<kSoundPreset;++i)setFromEngine(i,kPatches[lastPatch_][i]);setFromEngine(kFXPreset,0);lastFX_=0;setFromEngine(kOscPair,getInt(kWave1)*4+getInt(kWave2));lastPair_=getInt(kOscPair);applySoundExtras(lastPatch_);clearNotes();}
        if(getInt(kOscPair)!=lastPair_){lastPair_=getInt(kOscPair);setFromEngine(kWave1,float(lastPair_/4));setFromEngine(kWave2,float(lastPair_%4));}
        else {const int pair=getInt(kWave1)*4+getInt(kWave2);if(pair!=lastPair_){lastPair_=pair;setFromEngine(kOscPair,float(pair));}}
        for(int bank=0;bank<4;++bank)if(getInt(kBankProgressions[bank])!=lastProgressions_[bank]){
            const int preset=getInt(kBankProgressions[bank]);lastProgressions_[bank]=preset;
            for(int j=0;j<4;++j){setFromEngine(kChordRoots[bank*4+j],float(kProgressions[preset][j][0]));setFromEngine(kChordQualities[bank*4+j],float(kProgressions[preset][j][1]));}
        }
        for(int command=0;command<4;++command)if(getInt(kSlotsMinus+command)){
            const int target=command<2?kSlotCount:kMainChordPage;
            const int limit=command<2?4:getInt(kSlotCount)+1;
            const int next=command<2?std::clamp(getInt(target)+(command%2?1:-1),0,3):(getInt(target)+(command%2?1:-1)+limit)%limit;
            setFromEngine(target,float(next));setFromEngine(kSlotsMinus+command,0);
        }
        setFromEngine(kMainChordPage,float(std::min(getInt(kMainChordPage),getInt(kSlotCount))));
        if(getInt(kPadLearn)>activeSlots())setFromEngine(kPadLearn,0);
        if(activeSlots()!=lastSlotCount_){
            clearNotes();lastSlotCount_=activeSlots();
            if(getInt(kPadLearn)>lastSlotCount_)setFromEngine(kPadLearn,0);
        }
        if(lastMainPage_!=getInt(kMainChordPage)){
            // A vanished gesture surface cannot own release. Only stop touch
            // auditions here; physical MIDI chords remain independent of view.
            for(int i=0;i<16;++i){setFromEngine(kChordTouches[i],0);touchHeld_[i]=false;}
            for(auto& v:voices_)if(v.owner>=2048)release(v);
            lastMainPage_=getInt(kMainChordPage);
        }
        if(lastInput_!=getInt(kInputMode)||lastMode_!=getInt(kVoiceMode)||lastTrigger_!=getInt(kTrigger)){
            clearNotes();lastInput_=getInt(kInputMode);lastMode_=getInt(kVoiceMode);lastTrigger_=getInt(kTrigger);
        }
        bool mapChanged=false;
        for(int i=0;i<16;++i)if(lastPadMap_[i]!=getInt(kChordPadNotes[i]))mapChanged=true;
        if(lastPadMap_[16]!=getInt(kPadMap))mapChanged=true;
        if(mapChanged){
            for(auto& v:voices_)if(v.owner>=0&&v.owner<2048){v.amp.reset();v.env.reset();v.filter.reset();v.wait=0;v.owner=-1;}
            for(int i=0;i<16;++i)if(lastPadMap_[i]!=getInt(kChordPadNotes[i]))setFromEngine(kChordSlot,float(i));
            std::memset(held_,0,sizeof held_);std::memset(order_,0,sizeof order_);std::fill(std::begin(sustain_),std::end(sustain_),false);cachePadMapping();
        }
        if(getInt(kFXPreset)!=lastFX_){
            lastFX_=getInt(kFXPreset);
            static constexpr int ids[]={kDriveType,kDrive,kDriveMix,kChorus,kDelay,kFeedback,kDelayDivision,kSpace};
            static constexpr float racks[7][8]={{0,0,100,0,0,0,1,0},{0,24,75,8,6,16,0,0},{0,8,100,65,4,15,1,8},{1,14,65,8,28,12,0,0},{0,18,70,18,35,42,2,18},{2,58,65,0,0,0,1,0},{0,26,80,12,44,55,2,22}};
            if(lastFX_>0)for(int i=0;i<8;++i)setFromEngine(ids[i],racks[lastFX_-1][i]);
            if(lastFX_==1){setFromEngine(kMotion,0);setFromEngine(kTremDepth,0);setFromEngine(kDrift,0);}
        }
        // A triad has only two inversions, a seventh three, a five-tone chord four.
        for(int slot=0;slot<16;++slot){const int count=donut::chordToneCount(getInt(kChordQualities[slot]));const int v=getInt(kChordVoicings[slot]);if(v>=3&&v>count)setFromEngine(kChordVoicings[slot],float(count));}
        proxySyncThread_.store(donutThreadToken(),std::memory_order_release);
        for(int row=0;row<4;++row)for(int field=0;field<4;++field){const int index=kEditRoot1+row*4+field;proxySyncIndex_.store(index,std::memory_order_release);setFromEngine(index,get(chordField(getInt(kMainChordPage)*4+row,field)));}
        proxySyncIndex_.store(-1,std::memory_order_release);proxySyncThread_.store(0,std::memory_order_release);
        for(int i=0;i<kNumParams;++i){raw_[i]=get(i);smooth_[i].setTarget(raw_[i]);}
        for(auto& v:voices_){v.amp.set(raw_[kAmpAttack],raw_[kAmpDecay],raw_[kAmpSustain]*.01f,raw_[kAmpRelease]);v.env.set(raw_[kFiltAttack],raw_[kFiltDecay],raw_[kFiltSustain]*.01f,raw_[kFiltRelease]);v.pitch.setTime(raw_[kGlide],sr_);}
        gain_.setTarget(dbToGain(raw_[kVolume]));
        lfo_.setShape(int(raw_[kShape])<2?int(raw_[kShape]):int(raw_[kShape])+1);
        static const float beats[]={.25f,.5f,1,2,4};
        const float bpm=ctx.transport.tempoValid?float(ctx.transport.bpm):120.0f;
        if(raw_[kSync]>.5f){lfo_.setTempoSync(bpm,beats[int(std::lround(raw_[kDivision]))]);if(ctx.transport.playing&&ctx.transport.ppqValid)lfo_.syncToPpq(ctx.transport.ppqPos);}else lfo_.setFrequency(raw_[kRate]);
        static const float delayBeats[]={.5f,1,1.5f,2};
        delayTime_.setTarget(std::min(sr_*2.9f,sr_*60.0f/std::max(20.0f,bpm)*delayBeats[int(std::lround(raw_[kDelayDivision]))]));
        if(getBool(kTouchStop)){
            clearNotes();
            for(auto& d:delay_)d.reset();chorus_.reset();
        }
        for(int i=0;i<16;++i){
            if(i>=activeSlots())setFromEngine(kChordTouches[i],0);
            const bool gate=i<activeSlots()&&getBool(kChordTouches[i]);
            if(gate&&!touchHeld_[i]){startChord(i,2048+i,0,100);setFromEngine(kChordSlot,float(i));}
            if(!gate&&touchHeld_[i])for(auto& v:voices_)if(v.owner==2048+i)release(v);
            bool changed=false;for(int j=0;j<4;++j){const int value=getInt(chordField(i,j));if(lastTouchChord_[i][j]!=value)changed=true;lastTouchChord_[i][j]=value;}
            if(changed){setFromEngine(kChordSlot,float(i));if(gate&&touchHeld_[i])startChord(i,2048+i,0,100,true);}
            touchHeld_[i]=gate;
        }
        int e=0;
        for(int s=0;s<ctx.numFrames;++s){
            while(e<ctx.numMidiEvents&&ctx.midi[e].sampleOffset<=s)handle(ctx.midi[e++]);
            const bool control=control_==0;control_=(control_+1u)&15u;
            if(control){for(int i=0;i<kNumParams;++i)p_[i]=smooth_[i].process();delaySamples_=delayTime_.process();tremolo_.setFrequency(p_[kTremRate]);tapePitch_=wowFlutter_.process(p_[kDrift]);trimGain_=dbToGain(p_[kOutputTrim]);const float lowHz=std::max(5.f,p_[kLowCut]);if(std::abs(lowHz-lastLowCut_)>.01f){for(auto& f:lowCut_)f.set(lowHz,.29585f);lastLowCut_=lowHz;}}
            const float mod=lfo_.process(),vib=vibrato_.process(),drift=drift_.process();float sum=0;
            for(auto& v:voices_){
                if(v.wait>0){--v.wait;if(v.wait==0){v.amp.noteOn();v.env.noteOn();}else continue;}
                if(!v.amp.isActive())continue;
                const float amp=v.amp.process(),env=v.env.process(),pitch=v.pitch.process();
                if(control||v.dirty){
                    v.dirty=false;
                    const float modulation=(p_[kVibrato]*.003f+wheel_[v.channel]*.5f)*vib+p_[kMotion]*.0006f*drift;
                    const float lfoPitch=int(std::lround(raw_[kTarget]))==0?mod*p_[kDepth]*.02f:0;
                    const float hz=mpc::noteToHz(pitch+bend_[v.channel]*p_[kBend]+modulation+lfoPitch+tapePitch_);
                    v.a.setWave(int(std::lround(raw_[kWave1])));v.b.setWave(int(std::lround(raw_[kWave2])));
                    const float hz2=hz*std::pow(2.f,(std::round(raw_[kOctave])*12+p_[kDetune]*.01f)/12);v.a.setFrequency(hz);v.sineA.setFrequency(std::min(hz,sr_*.49f));v.b.setFrequency(hz2);v.sineB.setFrequency(std::min(hz2,sr_*.49f));v.sub.setFrequency(hz*.5f);v.fm.setFrequency(hz*2);
                    float pulse=p_[kPulse]*.01f;if(int(std::lround(raw_[kTarget]))==2)pulse+=mod*p_[kDepth]*.003f;
                    v.a.setPulseWidth(pulse);v.b.setPulseWidth(pulse);
                    float oct=p_[kEnv]*.04f*env+p_[kKeyTrack]*.01f*(pitch-60)/12;
                    if(int(std::lround(raw_[kTarget]))==1)oct+=mod*p_[kDepth]*.025f;
                    oct+=(v.velocity-0.5f)*p_[kExpression]*.007f;
                    v.filter.set(p_[kCutoff]*std::pow(2.f,std::clamp(oct,-8.f,8.f)),p_[kResonance]*.0092f);
                }
                const float fm=v.fm.process()*p_[kFM]*.004f*(.3f+env*.7f);
                const float a=int(std::lround(raw_[kWave1]))==0?v.sineA.process(fm):v.a.process();
                const float b=int(std::lround(raw_[kWave2]))==0?v.sineB.process():v.b.process();const float mix=std::clamp(p_[kMix]*.01f+(p_[kBody]-40)*.002f,0.f,1.f);
                const float sub=v.sub.process()*(p_[kSub]*.006f+p_[kBody]*.0015f);
                float x=a*(1-mix)+b*mix+sub+noise_.nextBipolar()*p_[kNoise]*.003f;
                // Tone adds harmonics while leaving the base Filter Cutoff parameter intact.
                x=fastTanh(x*(.7f+p_[kTone]*.012f));
                x=v.filter.process(x,int(std::lround(raw_[kFilter])));
                float trem=int(std::lround(raw_[kTarget]))==3?1-(mod*.5f+.5f)*p_[kDepth]*.008f:1;
                sum+=x*amp*v.velocityGain*trem*.3f;
            }
            const float drive=p_[kDrive]*.01f;float driven=sum;
            if(drive>.0001f){const float gain=1+drive*10;
                if(int(std::lround(raw_[kDriveType]))==0)driven=fastTanh(sum*gain)/(1+drive*3);
                else if(int(std::lround(raw_[kDriveType]))==1)driven=(fastTanh(sum*gain+.15f)-fastTanh(.15f))/(1+drive*3);
                else driven=std::clamp(sum*gain*1.5f,-.8f,.8f)/(1+drive*2);
                driven=sum+(driven-sum)*drive*p_[kDriveMix]*.01f;
            }
            const float dc=driven-dcIn_+dcCoeff_*dcOut_;dcIn_=flush(driven);dcOut_=flush(dc);
            const float cm=std::clamp(p_[kChorus]*.006f+p_[kMotion]*.002f+p_[kSpace]*.001f,0.f,.65f);
            const float tremGain=1.f-p_[kTremDepth]*.01f*(.5f+.5f*tremolo_.process());
            const float tremDc=dc*tremGain;
            const float cl=chorusLfo_.process();
            const float leftCh=chorus_.readLinear(sr_*(.014f+.003f*cl));const float rightCh=chorus_.readLinear(sr_*(.018f-.003f*cl));chorus_.write(tremDc);
            float l=tremDc*(1-cm)+leftCh*cm,r=tremDc*(1-cm)+rightCh*cm;
            const float dl=delay_[0].readLinear(delaySamples_),dr=delay_[1].readLinear(delaySamples_*1.012f);
            delay_[0].write(flush(l+dr*p_[kFeedback]*.01f));delay_[1].write(flush(r+dl*p_[kFeedback]*.01f));
            const float wet=std::min(.55f,p_[kDelay]*.005f+p_[kSpace]*.003f);l+=dl*wet;r+=dr*wet;
            const float hpMix=std::clamp(p_[kLowCut]/5.f,0.f,1.f);l+=hpMix*(lowCut_[0].process(l,2)-l);r+=hpMix*(lowCut_[1].process(r,2)-r);
            const float master=gain_.process()*trimGain_;ctx.outputs[0][s]=flush(fastTanh(l*master));if(ctx.numOutputs>1)ctx.outputs[1][s]=flush(fastTanh(r*master));
        }
        bool playing[4]{};
        for(const auto& v:voices_)if(v.amp.isHeld()||v.wait){const int slot=v.owner>=2048?v.owner-2048:(v.owner>=0?chordSlot(v.owner%128):-1);if(slot>=0&&slot<activeSlots())playing[slot/4]=true;}
        for(int page=0;page<4;++page)setFromEngine(kChordPlayingPage1+page,playing[page]?1.f:0.f);
    }
    bool formatValue(int index,float value,std::string& out) override {
        if(index>=kEditRoot1&&index<kEditRoot1+16&&(index-kEditRoot1)%4==0){static const char* names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};const int note=int(std::lround(value));char b[24];std::snprintf(b,sizeof b,"%s%d",names[note%12],note/12-2);out=b;return true;}
        if(index==kLowCut){if(value<.5f)out="Off";else{char b[24];std::snprintf(b,sizeof b,"%.0f Hz",value);out=b;}return true;}
        if(index==kLearnStatus){if(getInt(kPadLearn)>0){out="PRESS PHYSICAL PAD FOR SLOT "+std::to_string(getInt(kPadLearn));return true;}if(getInt(kLearnStatus)==0){out="HOLD CHORD / TAP LEARN";return true;}return false;}
        if(slotForParameter(index,kChordPadNotes)>=0){static const char* names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};char b[32];int n=std::clamp(int(std::lround(value)),0,127);std::snprintf(b,sizeof b,"%s%d / MIDI %d",names[n%12],n/12-2,n);out=b;return true;}
#ifdef DONUT_TOUCH_DIAGNOSTICS
        if(index>=kRoot1&&index<=kRoot4&&(index-kRoot1)%3==0) {
            char b[32];const int slot=(index-kRoot1)/3;
            std::snprintf(b,sizeof b,"%u:%u",touchOffWrites[slot].load(),unsigned(touchDuration[slot].load()*1000.0/sr_));out=b;return true;
        }
#endif
        if(index==kOctave||index==kTrigger){char b[32];std::snprintf(b,sizeof b,"%d %s",int(std::round(value)),param(index).unit.c_str());out=b;return true;}
        if(index==kRate&&getBool(kSync)){static const char* divisions[]={"1/16","1/8","1/4","1/2","1 bar"};out=std::string("Sync ")+divisions[getInt(kDivision)];return true;}
        if(index==kCutoff){char b[32];if(value>=1000)std::snprintf(b,sizeof b,"%.2f kHz",value/1000);else std::snprintf(b,sizeof b,"%.0f Hz",value);out=b;return true;}
        if(slotForParameter(index,kChordRoots)>=0){static const char* names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};int n=int(std::round(value));static const char* suffix[]={"m7","maj7","m9","maj9","7","9","13","sus2","m","","7b9"};out=std::string(names[n%12])+suffix[getInt(kChordQualities[slotForParameter(index,kChordRoots)])];return true;}
        if(param(index).kind==mpc::Param::Kind::Float){char b[48];std::snprintf(b,sizeof b,"%.1f %s",value,param(index).unit.c_str());out=b;return true;}
        return false;
    }
    int activeVoices() const {int n=0;for(const auto& v:voices_)if(v.amp.isActive()||v.wait)n++;return n;}
private:
    struct Voice {PolyBlepOsc a,b;WavetableSine sineA,sineB,sub,fm;SVF filter;ADSR amp,env;Smoother pitch;int owner=-1,channel=0,wait=0;uint64_t order=0;float velocity=1,velocityGain=1;bool dirty=true;};
    std::array<Voice,N> voices_{};
    std::array<Smoother,kNumParams> smooth_{};float p_[kNumParams]{},raw_[kNumParams]{};
    uint16_t held_[16][128]{};uint64_t order_[16][128]{};int velocity_[16][128]{};
    bool sustain_[16]{};float bend_[16]{},wheel_[16]{};uint64_t clock_=0;
    unsigned control_=0;int lastProgressions_[4]{};int lastPatch_=0,lastInput_=0,lastMode_=2,lastTrigger_=36;
    int lastPadMap_[17]{};
    void cachePadMapping(){for(int i=0;i<16;++i)lastPadMap_[i]=getInt(kChordPadNotes[i]);lastPadMap_[16]=getInt(kPadMap);}
    int lastSlotCount_=4,lastMainPage_=0;
    int activeSlots()const{return (getInt(kSlotCount)+1)*4;}
    static int chordField(int slot,int field){return field==0?kChordRoots[slot]:field==1?kChordQualities[slot]:field==2?kChordVoicings[slot]:kChordOctaves[slot];}
    static int slotForParameter(int index,const int* indices){for(int slot=0;slot<16;++slot)if(index==indices[slot])return slot;return -1;}
    int arrows_[4]{};int lastFX_=0,lastPair_=1;bool touchHeld_[16]{};std::array<std::array<int,4>,16> lastTouchChord_{};
    std::array<std::atomic<bool>,kNumParams> changed_{};
    std::array<std::atomic<std::uintptr_t>,4> renderThreads_{};
    std::atomic<bool> renderThreadOverflow_{false};
    std::atomic<bool> displayPending_{false},stateLoading_{false};
    std::atomic<std::uintptr_t> proxySyncThread_{0};std::atomic<int> proxySyncIndex_{-1};
    std::atomic_flag flushing_=ATOMIC_FLAG_INIT;
    void setFromEngine(int index,float value) {
        if(get(index)==value)return;
        set(index,value);changed_[index].store(true,std::memory_order_release);
        displayPending_.store(true,std::memory_order_release);
    }
    WowFlutter wowFlutter_;float tapePitch_=0;
    SVF lowCut_[2];
    float dcCoeff_=.9993f,lastLowCut_=-1,trimGain_=1;
    void applySoundExtras(int patch){setFromEngine(kLowCut,kPatchExtras[patch][0]);setFromEngine(kDrift,kPatchExtras[patch][1]);setFromEngine(kTremRate,kPatchExtras[patch][2]);setFromEngine(kTremDepth,kPatchExtras[patch][3]);setFromEngine(kOutputTrim,kPatchExtras[patch][4]);}
    float sr_=44100,delaySamples_=22050,dcIn_=0,dcOut_=0;LFO tremolo_,lfo_,vibrato_,drift_,chorusLfo_;Xorshift32 noise_{1234567};Smoother gain_,delayTime_;DelayLine chorus_,delay_[2];
    static float flush(float x){return std::fabs(x)<1e-25f?0.f:x;}
    void clearNotes(){for(auto& f:lowCut_)f.reset();for(int i=0;i<16;++i){touchHeld_[i]=false;setFromEngine(kChordTouches[i],0);}setFromEngine(kTouchStop,0);for(auto& v:voices_){v.amp.reset();v.env.reset();v.filter.reset();v.wait=0;v.owner=-1;}std::memset(held_,0,sizeof held_);std::memset(order_,0,sizeof order_);std::fill(std::begin(sustain_),std::end(sustain_),false);std::fill(std::begin(bend_),std::end(bend_),0.f);std::fill(std::begin(wheel_),std::end(wheel_),0.f);}
    void release(Voice& v){v.wait=0;v.amp.noteOff();v.env.noteOff();}
    int allocate(){const int first=mono()?1:0;for(int i=first;i<N;++i)if(!voices_[i].amp.isActive()&&!voices_[i].wait)return i;int best=first;for(int i=first+1;i<N;++i)if((voices_[i].amp.isReleasing()&&!voices_[best].amp.isReleasing())||(voices_[i].amp.isReleasing()==voices_[best].amp.isReleasing()&&voices_[i].order<voices_[best].order))best=i;return best;}
    void start(int owner,int ch,int note,int vel,int wait=0,int forced=-1,bool legato=false){
        Voice& v=voices_[forced>=0?forced:allocate()];const bool sounding=v.amp.isActive(),connected=sounding&&v.amp.isHeld();
        v.owner=owner;v.channel=ch;v.order=++clock_;v.velocity=vel/127.f;v.velocityGain=1-(1-v.velocity)*p_[kExpression]*.01f;v.dirty=true;
        if(sounding&&raw_[kGlide]>0&&(int(std::lround(raw_[kGlideLegato]))==0||v.amp.isHeld()))v.pitch.setTarget(float(note));else v.pitch.snap(float(note));
        if(!sounding){v.a.resetPhase();v.sineA.resetPhase();v.b.resetPhase(.23f);v.sineB.resetPhase(.23f);v.filter.reset();}
        v.wait=wait;if(wait){v.amp.reset();v.env.reset();}else if(!legato||!connected){v.amp.noteOn();v.env.noteOn();}
    }
    // Voice mode governs melody in NOTES and SPLIT; chord tones remain polyphonic.
    bool mono()const{return int(std::lround(raw_[kInputMode]))!=1&&int(std::lround(raw_[kVoiceMode]))<2;}
    int chordSlot(int n)const{
        if(int(std::lround(raw_[kInputMode]))==0)return -1;
        if(getInt(kPadMap)){for(int i=0;i<activeSlots();++i)if(n==getInt(kChordPadNotes[i]))return i;return -1;}
        const int slot=n-int(std::lround(raw_[kTrigger]));return slot>=0&&slot<activeSlots()?slot:-1;
    }
    bool melodyNote(int n)const{return int(std::lround(raw_[kInputMode]))!=1&&chordSlot(n)<0;}
    void releaseUnheldMidi(int ch){for(int i=mono()?1:0;i<N;++i){auto& v=voices_[i];if(v.owner>=0&&v.owner<2048&&v.channel==ch&&!held_[ch][v.owner%128])release(v);}}
    void monoFallback(){int bc=-1,bn=-1;uint64_t newest=0;for(int ch=0;ch<16;++ch)for(int n=0;n<128;++n)if(held_[ch][n]&&melodyNote(n)&&order_[ch][n]>=newest){newest=order_[ch][n];bc=ch;bn=n;}if(bc>=0)start(bc*128+bn,bc,bn,velocity_[bc][bn],0,0,int(std::lround(raw_[kVoiceMode]))==1);else release(voices_[0]);}
    void startChord(int slot,int owner,int ch,int vel,bool refresh=false){
            std::array<int,6> reuse{};int oldCount=0,used=0;
            if(refresh)for(int i=0;i<N&&oldCount<6;++i)if(voices_[i].owner==owner&&(voices_[i].amp.isHeld()||voices_[i].wait))reuse[oldCount++]=i;
            const int root=getInt(kChordRoots[slot]),quality=getInt(kChordQualities[slot]),voicing=getInt(kChordVoicings[slot]);
            std::array<int,6> notes{};const int count=donut::chordNotes(root,quality,voicing,getInt(kChordOctaves[slot])-2,notes);
            for(int i=0;i<count;++i){start(owner,ch,notes[i],vel,int(i*raw_[kSpread]*.001f*sr_),used<oldCount?reuse[used]:-1,refresh);++used;}
            for(int i=used;i<oldCount;++i)release(voices_[reuse[i]]);
    }
    void trigger(int ch,int n,int vel){
        const int owner=ch*128+n,slot=chordSlot(n);
        const int mode=int(std::lround(raw_[kInputMode]));
        if(mode&&slot>=0&&slot<activeSlots()){
            startChord(slot,owner,ch,vel);
        }else if(mode!=1){if(mono())start(owner,ch,n,vel,0,0,int(std::lround(raw_[kVoiceMode]))==1);else start(owner,ch,n,vel);}
    }
    void handle(const mpc::MidiEvent& e){const int ch=e.channel(),n=e.note();if(n>127)return;
        if(e.isNoteOn()&&getInt(kPadLearn)>0&&getInt(kPadLearn)<=activeSlots()){
            const int slot=getInt(kPadLearn)-1,old=getInt(kChordPadNotes[slot]);
            clearNotes();
            // Learning an already assigned note swaps slots, keeping mapping unique.
            for(int i=0;i<16;++i)if(i!=slot&&getInt(kChordPadNotes[i])==n)setFromEngine(kChordPadNotes[i],float(old));
            setFromEngine(kChordPadNotes[slot],float(n));setFromEngine(kPadMap,1);setFromEngine(kPadLearn,0);setFromEngine(kLearnStatus,float(slot+1));
            if(getInt(kInputMode)==0)setFromEngine(kInputMode,2);
            raw_[kInputMode]=get(kInputMode);lastInput_=getInt(kInputMode);cachePadMapping();
            return; // Registration press is silent; the next press performs the chord.
        }
        if(e.isNoteOn()){if(held_[ch][n]<65535)held_[ch][n]++;order_[ch][n]=++clock_;velocity_[ch][n]=e.velocity();trigger(ch,n,e.velocity());}
        else if(e.isNoteOff()){if(held_[ch][n])--held_[ch][n];if(held_[ch][n])return;if(!sustain_[ch]){if(mono()&&voices_[0].owner==ch*128+n)monoFallback();for(int i=mono()?1:0;i<N;++i)if(voices_[i].owner==ch*128+n)release(voices_[i]);}}
        else if(e.isPitchBend())bend_[ch]=e.pitchBend()/8192.f;
        else if(e.isController()){
            if(e.controller()==64){sustain_[ch]=e.controllerValue()>=64;if(!sustain_[ch]){if(mono()&&voices_[0].channel==ch)monoFallback();releaseUnheldMidi(ch);}}
            else if(e.controller()==1)wheel_[ch]=e.controllerValue()/127.f;
            else if(e.controller()==120||e.controller()==123){if(ch==0)for(int i=0;i<16;++i){touchHeld_[i]=false;setFromEngine(kChordTouches[i],0);}std::fill(std::begin(held_[ch]),std::end(held_[ch]),0);sustain_[ch]=false;for(auto& v:voices_)if(v.channel==ch){release(v);if(e.controller()==120){v.amp.reset();v.env.reset();}}if(mono()&&voices_[0].channel==ch)monoFallback();}
            else if(e.controller()==121){bend_[ch]=wheel_[ch]=0;sustain_[ch]=false;if(mono()&&voices_[0].channel==ch)monoFallback();releaseUnheldMidi(ch);}
        }
    }
};
// MPC has no native editor. Drain notifications on its UI idle/read calls,
// never from processReplacing or setParameter (which can run on the audio thread).
static VstIntPtr donutDispatch(AEffect* e,VstInt32 op,VstInt32 index,VstIntPtr value,void* ptr,float opt) {
    auto* p=static_cast<DonutTheory*>(e->object);
    if(op==effOpen){const auto result=mpc::detail::dispatcher(e,op,index,value,ptr,opt);p->requestHostIdle();return result;}
    if(op==effSetChunk)return p->internalLoadState(static_cast<const uint8_t*>(ptr),size_t(value))?1:0;
    if(op==effSetProgram){p->setProgram(int(value));p->flushHostUpdates();return 0;}
    if(op==effGetProgram){p->flushHostUpdates();return p->currentProgram();}
    if(op==effGetProgramName){mpc::copyString(static_cast<char*>(ptr),kVstMaxProgNameLen+1,p->programName(p->currentProgram()));return 0;}
    if(op==effIdle_Deprecated||op==effEditIdle){p->flushHostUpdates();return 1;}
    if(op==effGetChunk){
        const auto& state=p->internalSaveState();
        if(ptr)*static_cast<void**>(ptr)=const_cast<uint8_t*>(state.data());
        p->flushHostUpdates();return VstIntPtr(state.size());
    }
    if(op==effGetParamDisplay||op==effGetParamName)p->flushHostUpdates();
    return mpc::detail::dispatcher(e,op,index,value,ptr,opt);
}
static float donutGetParameter(AEffect* e,VstInt32 index){
    static_cast<DonutTheory*>(e->object)->flushFromValueRead();
    return mpc::detail::getParameter(e,index);
}
#ifdef DONUT_TOUCH_DIAGNOSTICS
static void donutDiagnosticSet(AEffect* e,VstInt32 index,float value) {
    static_cast<DonutTheory*>(e->object)->traceTouchWrite(index,value);
    mpc::detail::setParameter(e,index,value);
}
#endif
extern "C" MPC_EXPORT AEffect* VSTPluginMain(audioMasterCallback host) {
    if(host&&host(nullptr,audioMasterVersion,0,0,nullptr,0)==0)return nullptr;
    auto* e=mpc::detail::createEffect(new DonutTheory(),host);
    if(e){e->dispatcher=&donutDispatch;e->getParameter=&donutGetParameter;static_cast<DonutTheory*>(e->object)->requestHostIdle();}
#ifdef DONUT_TOUCH_DIAGNOSTICS
    if(e)e->setParameter=&donutDiagnosticSet;
#endif
    return e;
}
#if defined(__APPLE__)
extern "C" MPC_EXPORT AEffect* main_macho(audioMasterCallback host){return VSTPluginMain(host);}
#endif
