// Donut Theory by kim. Two oscillators + sub/FM/noise -> nonlinear ladder / ADSRs / rounded VCA
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
#include "SmoothVoicing.h"
#include "WowFlutter.h"
#include "AnalogLadder.h"
#include "BodyTone.h"
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
        i.uniqueId=MPC_FOURCC('K','r','q','l');i.version=10800;i.isInstrument=true;i.numInputs=0;i.numOutputs=2;return i;
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
        // Canonicalise gesture normalised values too, including fractional host writes.
        set(kTransposeOpen,0);set(kTransposeApply,0);
        set(kSmoothOpen,0);set(kSmoothApply,0);set(kSmoothPreview,0);
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
        for(int slot=0;slot<16;++slot)if(count<=uint32_t(kHarmonyType1+slot))set(kHarmonyType1+slot,get(kChordLegacyQualities[slot]));
        for(int bank=0;bank<4;++bank)if(count<=uint32_t(kHarmonyPreset1+bank))set(kHarmonyPreset1+bank,get(kLegacyBankProgressions[bank]));
        legacyReplaySamples_.store(count>242?int(sr_*.5f):0,std::memory_order_release);
        if(count<=77)set(kPadMap,0); // Existing projects retain Trigger Base routing.
        set(kTransposeOpen,0);set(kTransposeApply,0);
        set(kSmoothOpen,0);set(kSmoothApply,0);set(kSmoothPreview,0);
        stateLoading_.store(false,std::memory_order_release);return true;
    }
    void parameterChanged(int index,float value) override {
        if(!stateLoading_.load(std::memory_order_acquire)){
            const bool protect=legacyReplaySamples_.load(std::memory_order_acquire)>0;
            if(index==kLfoRateControl||index==kModRateControl||index==kLfo2RateControl){
                const bool readback=proxySyncIndex_.load()==index&&proxySyncThread_.load()==donutThreadToken();
                if(!readback&&!protect){const int module=index==kLfo2RateControl?2:index==kModRateControl?1:0;const int rate=rateParameters_[module];
                    if(getBool(syncParameters_[module]))setFromEngine(divisionParameters_[module],float(std::lround(std::clamp(value,0.f,1.f)*4)));
                    else setFromEngine(rate,param(rate).toPlain(value));}
            }
            const int oldType=slotForParameter(index,kChordLegacyQualities),oldBank=slotForParameter4(index,kLegacyBankProgressions);
            if(oldType>=0&&!protect)setFromEngine(kHarmonyType1+oldType,value);
            if(oldBank>=0&&!protect)setFromEngine(kHarmonyPreset1+oldBank,value);
            if(index>=kHarmonyType1&&index<=kHarmonyType16&&value<11)setFromEngine(kChordLegacyQualities[index-kHarmonyType1],value);
            if(index>=kHarmonyPreset1&&index<=kHarmonyPreset4&&value<32)setFromEngine(kLegacyBankProgressions[index-kHarmonyPreset1],value);
            if(index==kHarmonyCurrent&&!protect){const bool readback=proxySyncIndex_.load()==index&&proxySyncThread_.load()==donutThreadToken();if(!readback)setFromEngine(kBankProgressions[getInt(kMainChordPage)],value);}
            if(index>=kHarmonyEdit1&&index<=kHarmonyEdit4){const bool readback=proxySyncIndex_.load()==index&&proxySyncThread_.load()==donutThreadToken();if(!readback)setFromEngine(kChordQualities[getInt(kMainChordPage)*4+index-kHarmonyEdit1],value);}
        }
        if(index>=kEditRoot1&&index<kEditRoot1+16&&!stateLoading_.load(std::memory_order_acquire)){
            const bool engineReadback=proxySyncIndex_.load(std::memory_order_acquire)==index&&proxySyncThread_.load(std::memory_order_acquire)==donutThreadToken();
            if(!engineReadback){const int offset=index-kEditRoot1,target=chordField(getInt(kMainChordPage)*4+offset/4,offset%4);if(get(target)!=value)setFromEngine(target,value);}
        }
        if(getInt(kSmoothOpen)&&(index==kSmoothOpen||index==kSmoothScope||index==kSlotCount||index==kMainChordPage||slotForParameter(index,kChordRoots)>=0||slotForParameter(index,kChordQualities)>=0||slotForParameter(index,kChordVoicings)>=0||slotForParameter(index,kChordOctaves)>=0))smoothNeedsPlan_.store(true,std::memory_order_release);
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
        runSmoothFromUI();
        for(int i=0;i<kNumParams;++i)
            if(changed_[i].exchange(false,std::memory_order_acq_rel))notifyHostParameterChanged(i);
        if(displayPending_.exchange(false,std::memory_order_acq_rel))requestDisplayUpdate();
        flushing_.clear(std::memory_order_release);
    }
    void prepare(double sr,int) override {
        sr_=float(sr);dcCoeff_=std::exp(-2.f*3.14159265359f*5.f/sr_);for(auto& f:lowCut_)f.setSampleRate(sr);wowFlutter_.prepare(sr/16.0);
        for(auto& v:voices_){v.a.setSampleRate(sr);v.b.setSampleRate(sr);v.sineA.setSampleRate(sr);v.sineB.setSampleRate(sr);v.sub.setSampleRate(sr);v.fm.setSampleRate(sr);v.filter.setSampleRate(sr);v.amp.setSampleRate(sr);v.env.setSampleRate(sr);v.pitch.setTime(10,sr);}
        bodyTone_.prepare(float(sr));lfo2_.setSampleRate(sr);lfo2Signal_.setTime(2,sr);
        for(int i=0;i<5;++i){lfo2Weights_[i].setTime(10,sr/16);lfo2Weights_[i].snap(i==getInt(kLfo2Target)?1.f:0.f);}
        tremolo_.setSampleRate(sr);lfo_.setSampleRate(sr);vibrato_.setSampleRate(sr);vibrato_.setFrequency(5.2f);
        drift_.setSampleRate(sr);drift_.setFrequency(.13f);chorusLfo_.setSampleRate(sr);chorusLfo_.setFrequency(.31f);
        for(auto& d:delay_)d.prepare(int(sr*3)+2);
        chorus_.prepare(int(sr*.05)+2);
        for(int i=0;i<kNumParams;++i){smooth_[i].setTime(15,sr/16.0);smooth_[i].snap(get(i));}
        gain_.setTime(10,sr);gain_.snap(dbToGain(get(kVolume)));
        panLeft_.setTime(5,sr);panRight_.setTime(5,sr);motionGain_.setTime(2,sr);
        delayTime_.setTime(100,sr/16.0);delayTime_.snap(sr*.5f);
        reset();
    }
    void reset() override {
        setFromEngine(kTransposeOpen,0);setFromEngine(kTransposeApply,0);
        clearNotes();for(int i=0;i<N;++i){auto& v=voices_[i];const float phase=std::fmod(i*.61803398875f,1.f);v.a.resetPhase(phase);v.sineA.resetPhase(phase);v.b.resetPhase(std::fmod(phase+.23f,1.f));v.sineB.resetPhase(std::fmod(phase+.23f,1.f));v.sub.resetPhase(phase);v.fm.resetPhase(phase);}
        for(int i=0;i<16;++i)for(int j=0;j<4;++j)lastTouchChord_[i][j]=getInt(chordField(i,j));lastSlotCount_=activeSlots();lastMainPage_=getInt(kMainChordPage);lastLowCut_=-1;cachePadMapping();for(auto& d:delay_)d.reset();chorus_.reset();
        panLeft_.snap(1);panRight_.snap(1);motionGain_.snap(1);bodyTone_.reset();lfo2_.reset();lfo2Signal_.snap(0);for(int i=0;i<5;++i){lfo2TargetWeights_[i]=i==getInt(kLfo2Target)?1.f:0.f;lfo2Weights_[i].snap(lfo2TargetWeights_[i]);}
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
        const int replay=legacyReplaySamples_.load(std::memory_order_relaxed);if(replay>0)legacyReplaySamples_.store(std::max(0,replay-ctx.numFrames),std::memory_order_release);
#ifdef DONUT_TOUCH_DIAGNOSTICS
        diagnosticFrames.fetch_add(unsigned(ctx.numFrames),std::memory_order_relaxed);
#endif
        for(int i=0;i<4;++i){if(getInt(kSoundPrev+i)){const int target=i<2?kSoundPreset:kBankProgressions[getInt(kMainChordPage)];const int count=i<2?64:kProgressionCount;setFromEngine(target,float((getInt(target)+(i%2?1:-1)+count)%count));setFromEngine(kSoundPrev+i,0);arrows_[i]=0;}}
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
            if(getBool(kChordGlide))for(int i=0;i<16;++i)endChordOwner(2048+i,0);
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
            for(auto& v:voices_)if(v.owner>=0&&v.owner<2048){v.amp.reset();v.env.reset();v.filter.reset();v.wait=v.glideWait=0;v.owner=-1;}
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
        // Key selects a reference only. Only an explicit confirmed command moves notes.
        const int delta=(getInt(kChordKeyRoot)-getInt(kTransposeFrom)+18)%12-6;
        const int shift=delta==-6?6:delta; // tritone defaults upward; all other moves nearest.
        setFromEngine(kTransposeDelta,float(shift));
        if(getInt(kTransposeApply)){
            setFromEngine(kTransposeApply,0);
            if(getInt(kTransposeOpen)){
                bool valid=true;
                for(int slot=0;slot<activeSlots();++slot){const int root=getInt(kChordRoots[slot])+shift;if(root<24||root>84)valid=false;}
                if(valid){
                    for(int slot=0;slot<activeSlots();++slot)setFromEngine(kChordRoots[slot],float(getInt(kChordRoots[slot])+shift));
                    setFromEngine(kTransposeFrom,get(kChordKeyRoot));setFromEngine(kTransposeFromMode,get(kChordKeyMode));
                    setFromEngine(kTransposeStatus,shift?1.f:2.f);setFromEngine(kTransposeOpen,0);setFromEngine(kTransposeDelta,0);
                }else setFromEngine(kTransposeStatus,3);
            }
        }
        processSmoothCommands(ctx);
        static constexpr int scales[2][7]={{0,2,4,5,7,9,11},{0,2,3,5,7,8,10}};
        for(int row=0;row<4;++row){
            const int slot=getInt(kMainChordPage)*4+row,interval=(getInt(kChordRoots[slot])-getInt(kChordKeyRoot)+120)%12;
            const int quality=getInt(kChordQualities[slot]);const bool minor=donut::minorType(quality);int degree=0;
            for(int step=0;step<7;++step)if(scales[getInt(kChordKeyMode)][step]==interval)degree=step+1+(minor?7:0);
            setFromEngine(kKeyDegree1+row,float(degree));
        }
        proxySyncThread_.store(donutThreadToken(),std::memory_order_release);
        for(int row=0;row<4;++row)for(int field=0;field<4;++field){const int index=kEditRoot1+row*4+field;proxySyncIndex_.store(index,std::memory_order_release);setFromEngine(index,get(chordField(getInt(kMainChordPage)*4+row,field)));}
        for(int row=0;row<4;++row){const int index=kHarmonyEdit1+row;proxySyncIndex_.store(index,std::memory_order_release);const int type=getInt(kChordQualities[getInt(kMainChordPage)*4+row]);setFromEngine(index,float(type));setFromEngine(kHarmonyTones1+row,float(donut::chordToneCount(type)-3));}
        proxySyncIndex_.store(kHarmonyCurrent,std::memory_order_release);setFromEngine(kHarmonyCurrent,get(kBankProgressions[getInt(kMainChordPage)]));
        for(int module=0;module<3;++module){const int relay=rateRelays_[module];
            const float value=getBool(syncParameters_[module])?get(divisionParameters_[module])*.25f:getNormalised(rateParameters_[module]);
            proxySyncIndex_.store(relay,std::memory_order_release);setFromEngine(relay,value);}
        proxySyncIndex_.store(-1,std::memory_order_release);proxySyncThread_.store(0,std::memory_order_release);
        for(int i=0;i<kNumParams;++i){raw_[i]=get(i);smooth_[i].setTarget(raw_[i]);}
        for(auto& v:voices_){v.amp.set(raw_[kAmpAttack],raw_[kAmpDecay],raw_[kAmpSustain]*.01f,raw_[kAmpRelease]);v.env.set(raw_[kFiltAttack],raw_[kFiltDecay],raw_[kFiltSustain]*.01f,raw_[kFiltRelease]);v.pitch.setTime(raw_[kGlide],sr_);}
        gain_.setTarget(dbToGain(raw_[kVolume]));
        lfo_.setShape(int(raw_[kShape])<2?int(raw_[kShape]):int(raw_[kShape])+1);
        static const float beats[]={.25f,.5f,1,2,4};
        const float bpm=mpc::dsp::safeBpm(ctx.transport.tempoValid?ctx.transport.bpm:120.0);
        if(raw_[kSync]>.5f){lfo_.setTempoSync(bpm,beats[int(std::lround(raw_[kDivision]))]);if(ctx.transport.playing&&ctx.transport.ppqValid)lfo_.syncToPpq(ctx.transport.ppqPos);}else lfo_.setFrequency(raw_[kRate]);
        if(raw_[kModSync]>.5f){tremolo_.setTempoSync(bpm,beats[getInt(kModDivision)]);if(ctx.transport.playing&&ctx.transport.ppqValid)tremolo_.syncToPpq(ctx.transport.ppqPos);}
        if(getBool(kLfo2Sync)){lfo2_.setTempoSync(bpm,beats[getInt(kLfo2Division)]);if(ctx.transport.playing&&ctx.transport.ppqValid)lfo2_.syncToPpq(ctx.transport.ppqPos);}
        static const float delayBeats[]={.5f,1,1.5f,2};
        delayTime_.setTarget(std::min(sr_*2.9f,sr_*60.0f/std::max(20.0f,bpm)*delayBeats[int(std::lround(raw_[kDelayDivision]))]));
        if(getBool(kTouchStop)){
            clearNotes();
            for(auto& d:delay_)d.reset();chorus_.reset();
        }
        for(int i=0;i<16;++i){
            if(i>=activeSlots())setFromEngine(kChordTouches[i],0);
            const bool gate=i<activeSlots()&&getBool(kChordTouches[i]);
            if(gate&&!touchHeld_[i]){touchOrder_[i]=++clock_;startChord(i,2048+i,0,100);setFromEngine(kChordSlot,float(i));}
            if(!gate&&touchHeld_[i])endChordOwner(2048+i,0);
            bool changed=false;for(int j=0;j<4;++j){const int value=getInt(chordField(i,j));if(lastTouchChord_[i][j]!=value)changed=true;lastTouchChord_[i][j]=value;}
            if(changed){setFromEngine(kChordSlot,float(i));if(gate&&touchHeld_[i])startChord(i,2048+i,0,100,true);}
            touchHeld_[i]=gate;
        }
        int e=0;
        for(int s=0;s<ctx.numFrames;++s){
            while(e<ctx.numMidiEvents&&ctx.midi[e].sampleOffset<=s)handle(ctx.midi[e++]);
            renderSmoothStep();
            const bool control=control_==0;control_=(control_+1u)&15u;
            if(control){for(int i=0;i<kNumParams;++i)p_[i]=smooth_[i].process();delaySamples_=delayTime_.process();if(!getBool(kModSync))tremolo_.setFrequency(p_[kTremRate]);tremolo_.setShape(getInt(kModShape));lfo2_.setShape(getInt(kLfo2Shape));if(!getBool(kLfo2Sync))lfo2_.setFrequency(p_[kLfo2Rate]);for(int i=0;i<5;++i){lfo2Weights_[i].setTarget(i==getInt(kLfo2Target)?1.f:0.f);lfo2TargetWeights_[i]=lfo2Weights_[i].process();}tapePitch_=wowFlutter_.process(p_[kDrift]);trimGain_=dbToGain(p_[kOutputTrim]);const float lowHz=std::max(5.f,p_[kLowCut]);if(std::abs(lowHz-lastLowCut_)>.01f){for(auto& f:lowCut_)f.set(lowHz,.29585f);lastLowCut_=lowHz;}}
            const float mod=lfo_.process(),vib=vibrato_.process(),drift=drift_.process();float sum=0;
            lfo2Signal_.setTarget(lfo2_.process());const float mod2=lfo2Signal_.process()*p_[kLfo2Depth];
            for(auto& v:voices_){
                if(v.glideWait>0&&!--v.glideWait){if(raw_[kGlide]>0)v.pitch.setTarget(float(v.note));else v.pitch.snap(float(v.note));}
                if(v.wait>0){--v.wait;if(v.wait==0){v.amp.noteOn();v.env.noteOn();}else continue;}
                if(!v.amp.isActive())continue;
                const float amp=v.amp.process(),env=v.env.process(),pitch=v.pitch.process();
                if(control||v.dirty){
                    v.dirty=false;
                    const float modulation=(p_[kVibrato]*.003f+wheel_[v.channel]*.5f)*vib+p_[kMotion]*.0006f*drift;
                    const float lfoPitch=(int(std::lround(raw_[kTarget]))==0?mod*p_[kDepth]*.02f:0)+mod2*lfo2TargetWeights_[0]*.02f;
                    const float hz=mpc::noteToHz(pitch+bend_[v.channel]*p_[kBend]+modulation+lfoPitch+tapePitch_);
                    v.a.setWave(int(std::lround(raw_[kWave1])));v.b.setWave(int(std::lround(raw_[kWave2])));
                    const float hz2=hz*std::pow(2.f,(std::round(raw_[kOctave])*12+p_[kDetune]*.01f)/12);v.a.setFrequency(hz);v.sineA.setFrequency(std::min(hz,sr_*.49f));v.b.setFrequency(hz2);v.sineB.setFrequency(std::min(hz2,sr_*.49f));v.sub.setFrequency(hz*.5f);v.fm.setFrequency(hz*2);
                    float pulse=p_[kPulse]*.01f;if(int(std::lround(raw_[kTarget]))==2)pulse+=mod*p_[kDepth]*.003f;
                    pulse+=mod2*lfo2TargetWeights_[2]*.003f;v.a.setPulseWidth(pulse);v.b.setPulseWidth(pulse);
                    float oct=p_[kEnv]*.04f*env+p_[kKeyTrack]*.01f*(pitch-60)/12;
                    if(int(std::lround(raw_[kTarget]))==1)oct+=mod*p_[kDepth]*.025f;
                    oct+=mod2*lfo2TargetWeights_[1]*.025f;
                    oct+=(v.velocity-0.5f)*p_[kExpression]*.007f;
                    v.filter.set(p_[kCutoff]*std::pow(2.f,std::clamp(oct,-8.f,8.f)),p_[kResonance]*.01f);
                }
                const float fm=v.fm.process()*p_[kFM]*.004f*(.3f+env*.7f);
                const float a=int(std::lround(raw_[kWave1]))==0?v.sineA.process(fm):v.a.process();
                const float b=int(std::lround(raw_[kWave2]))==0?v.sineB.process():v.b.process();const float mix=std::clamp(p_[kMix]*.01f+mod2*lfo2TargetWeights_[4]*.005f,0.f,1.f);
                // Independent sine sub, exactly one octave below OSC1, follows its pitch/glide.
                const float sub=v.sub.process()*p_[kSub]*.006f;
                float x=a*(1-mix)+b*mix+sub+noise_.nextBipolar()*p_[kNoise]*.003f;
                // Tone adds harmonics while leaving the base Filter Cutoff parameter intact.
                x=donut::analogMixer(x,p_[kTone]);
                x=v.filter.process(x,int(std::lround(raw_[kFilter])));
                float trem=int(std::lround(raw_[kTarget]))==3?1-(mod*.5f+.5f)*p_[kDepth]*.008f:1;
                trem*=1.f-(.5f+.5f*lfo2Signal_.value())*p_[kLfo2Depth]*lfo2TargetWeights_[3]*.008f;
                sum+=donut::analogVca(x*amp*v.velocityGain*trem)*.45f;
            }
            sum=bodyTone_.process(sum,p_[kBodyTone]*.01f);
            const float drive=p_[kDrive]*.01f;float driven=sum;
            if(drive>.0001f){const float gain=1+drive*10;
                if(int(std::lround(raw_[kDriveType]))==0)driven=fastTanh(sum*gain)/(1+drive*3);
                else if(int(std::lround(raw_[kDriveType]))==1)driven=(fastTanh(sum*gain+.15f)-fastTanh(.15f))/(1+drive*3);
                else driven=std::clamp(sum*gain*1.5f,-.8f,.8f)/(1+drive*2);
                driven=sum+(driven-sum)*drive*p_[kDriveMix]*.01f;
            }
            const float dc=driven-dcIn_+dcCoeff_*dcOut_;dcIn_=flush(driven);dcOut_=flush(dc);
            const float cm=std::clamp(p_[kChorus]*.006f+p_[kMotion]*.002f+p_[kSpace]*.001f,0.f,.65f);
            const float motion=tremolo_.process();
            const float panMode=std::clamp(p_[kModMode],0.f,1.f),depth=p_[kTremDepth]*.01f;
            // Shared oscillator: volume modulation OR stereo balance motion.
            motionGain_.setTarget(1.f-(1.f-panMode)*depth*(.5f+.5f*motion));
            const float tremDc=dc*motionGain_.process();
            const float cl=chorusLfo_.process();
            const float leftCh=chorus_.readLinear(sr_*(.014f+.003f*cl));const float rightCh=chorus_.readLinear(sr_*(.018f-.003f*cl));chorus_.write(tremDc);
            float l=tremDc*(1-cm)+leftCh*cm,r=tremDc*(1-cm)+rightCh*cm;
            const float dl=delay_[0].readLinear(delaySamples_),dr=delay_[1].readLinear(delaySamples_*1.012f);
            delay_[0].write(flush(l+dr*p_[kFeedback]*.01f));delay_[1].write(flush(r+dl*p_[kFeedback]*.01f));
            const float wet=std::min(.55f,p_[kDelay]*.005f+p_[kSpace]*.003f);l+=dl*wet;r+=dr*wet;
            const float hpMix=std::clamp(p_[kLowCut]/5.f,0.f,1.f);l+=hpMix*(lowCut_[0].process(l,2)-l);r+=hpMix*(lowCut_[1].process(r,2)-r);
            // Balance the final stereo signal without folding chorus/delay to mono.
            const float pan=std::clamp(p_[kPan]*.01f+panMode*depth*motion,-1.f,1.f);
            panLeft_.setTarget(pan>0?fastSin01(.25f+pan*.25f):1.f);
            panRight_.setTarget(pan<0?fastSin01(.25f-pan*.25f):1.f);
            l*=panLeft_.process();r*=panRight_.process();
            const float master=gain_.process()*trimGain_;ctx.outputs[0][s]=flush(fastTanh(l*master));if(ctx.numOutputs>1)ctx.outputs[1][s]=flush(fastTanh(r*master));
        }
        bool playing[4]{};
        for(const auto& v:voices_)if(v.amp.isHeld()||v.wait){const int slot=v.owner>=2048?v.owner-2048:(v.owner>=0?chordSlot(v.owner%128):-1);if(slot>=0&&slot<activeSlots())playing[slot/4]=true;}
        for(int page=0;page<4;++page)setFromEngine(kChordPlayingPage1+page,playing[page]?1.f:0.f);
    }
    bool formatValue(int index,float value,std::string& out) override {
        if(index==kSmoothMovement||(index>=kSmoothRow1&&index<=kSmoothRow16)){
            donut::SmoothPlan plan;if(!smoothPlan_.read(plan)||!plan.valid){out="—";return true;}
            char b[64];
            if(index==kSmoothMovement)std::snprintf(b,sizeof b,"TOP: %d -> %d st",plan.movementBefore,plan.movementAfter);
            else {const int row=index-kSmoothRow1;if(row>=plan.input.count){out="—";return true;}static const char* names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};const int a=plan.before[row],z=plan.after[row];std::snprintf(b,sizeof b,"%02d  %s%d > %s%d",plan.input.start+row+1,names[a%12],a/12-2,names[z%12],z/12-2);}
            out=b;return true;
        }

        if(index==kTransposeDelta){char b[24];std::snprintf(b,sizeof b,"%+d semitones",int(std::lround(value)));out=b;return true;}

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
        if(slotForParameter(index,kChordRoots)>=0){static const char* names[]={"C","C#","D","Eb","E","F","F#","G","Ab","A","Bb","B"};int n=int(std::round(value));out=std::string(names[n%12])+donut::kTypeSuffixes[getInt(kChordQualities[slotForParameter(index,kChordRoots)])];return true;}
        if(index==kLfoRateControl||index==kModRateControl||index==kLfo2RateControl){
            const int module=index==kLfo2RateControl?2:index==kModRateControl?1:0;
            if(getBool(syncParameters_[module])){static const char* names[]={"1/16","1/8","1/4","1/2","1 bar"};out=names[getInt(divisionParameters_[module])];return true;}
            char b[32];std::snprintf(b,sizeof b,"%.1f Hz",get(rateParameters_[module]));out=b;return true;}
        if(index==kTremRate&&getBool(kModSync)){static const char* names[]={"1/16","1/8","1/4","1/2","1 bar"};out=std::string("Sync ")+names[getInt(kModDivision)];return true;}
        if(index==kPan){char b[32];if(std::abs(value)<.5f)out="Center";else {std::snprintf(b,sizeof b,"%s %.0f %%",value<0?"L":"R",std::abs(value));out=b;}return true;}
        if(param(index).kind==mpc::Param::Kind::Float){char b[48];std::snprintf(b,sizeof b,"%.1f %s",value,param(index).unit.c_str());out=b;return true;}
        return false;
    }
#ifdef DONUT_VOICE_TEST
    std::array<float,3> modulationPhases()const{return {lfo_.phase(),tremolo_.phase(),lfo2_.phase()};}
    struct VoiceSnapshot {int owner,note;float pitch,amplitude;bool held;};
    std::array<VoiceSnapshot,N> voiceSnapshots()const{std::array<VoiceSnapshot,N> result{};for(int i=0;i<N;++i){const auto& v=voices_[i];result[i]={v.owner,v.note,v.pitch.value(),v.amp.level(),v.amp.isHeld()||v.wait>0};}return result;}
#endif
    int activeVoices() const {int n=0;for(const auto& v:voices_)if(v.amp.isActive()||v.wait)n++;return n;}
private:
    static constexpr int rateParameters_[3]={kRate,kTremRate,kLfo2Rate};
    static constexpr int syncParameters_[3]={kSync,kModSync,kLfo2Sync};
    static constexpr int divisionParameters_[3]={kDivision,kModDivision,kLfo2Division};
    static constexpr int rateRelays_[3]={kLfoRateControl,kModRateControl,kLfo2RateControl};
    std::atomic<int> legacyReplaySamples_{0};
    static int slotForParameter4(int index,const int* list){for(int i=0;i<4;++i)if(index==list[i])return i;return -1;}
    donut::AtomicSmoothPlan smoothPlan_;
    std::atomic<bool> smoothNeedsPlan_{false};
    donut::SmoothPlan smoothAudition_{}; // owned by the render thread
    bool smoothPlaying_=false,smoothGate_=false;
    int smoothStep_=0,smoothNext_=0,smoothOff_=0,smoothBeat_=1;
    donut::SmoothInput smoothInput()const{
        donut::SmoothInput in;in.active=activeSlots();in.page=getInt(kMainChordPage);in.scope=getInt(kSmoothScope);in.start=in.scope?0:in.page*4;in.count=in.scope?in.active:4;
        for(int slot=0;slot<16;++slot)for(int field=0;field<4;++field)in.chords[slot][field]=getInt(chordField(slot,field));return in;
    }
    void runSmoothFromUI(){
        if(!getInt(kSmoothOpen)||!smoothNeedsPlan_.load(std::memory_order_acquire))return;
        // The same host may read values on its rendering thread. Never search there.
        const auto token=donutThreadToken();if(renderThreadOverflow_.load(std::memory_order_relaxed))return;
        for(const auto& seen:renderThreads_)if(seen.load(std::memory_order_relaxed)==token)return;
        if(!smoothNeedsPlan_.exchange(false,std::memory_order_acq_rel))return;
        const auto input=smoothInput();const auto plan=donut::findSmoothVoicing(input);
        if(!(smoothInput()==input)){smoothNeedsPlan_.store(true,std::memory_order_release);return;}
        smoothPlan_.publish(plan);
        bool same=true;for(int i=0;i<input.count;++i)if(plan.voice[i]!=input.chords[input.start+i][2]||plan.octave[i]!=input.chords[input.start+i][3])same=false;
        setFromEngine(kSmoothStatus,!plan.valid?4.f:same?5.f:1.f);setFromEngine(kSmoothMovement,float(plan.movementAfter));
        for(int i=0;i<16;++i){setFromEngine(kSmoothRow1+i,float(i<input.count?plan.after[i]:0));changed_[kSmoothRow1+i].store(true,std::memory_order_release);}
        changed_[kSmoothMovement].store(true,std::memory_order_release);displayPending_.store(true,std::memory_order_release);
    }
    void stopSmoothPreview(){
        for(auto& v:voices_)if(v.owner==4096)release(v);
        smoothPlaying_=smoothGate_=false;setFromEngine(kSmoothPreview,0);if(getInt(kSmoothStatus)==2)setFromEngine(kSmoothStatus,1);
    }
    void processSmoothCommands(const mpc::ProcessContext& ctx){
        if(!getInt(kSmoothOpen)){if(smoothPlaying_)stopSmoothPreview();setFromEngine(kSmoothPreview,0);setFromEngine(kSmoothApply,0);return;}
        donut::SmoothPlan plan;const bool have=smoothPlan_.read(plan)&&plan.valid;
        const auto input=smoothInput();
        const bool matching=have&&plan.input==input&&!smoothNeedsPlan_.load(std::memory_order_acquire);
        if(smoothPlaying_&&!(smoothAudition_.input==input))stopSmoothPreview();
        if(!matching){if(smoothPlaying_)stopSmoothPreview();if(getInt(kSmoothPreview))setFromEngine(kSmoothPreview,0);}
        if(getInt(kSmoothApply)){
            setFromEngine(kSmoothApply,0);
            if(matching){stopSmoothPreview();for(int i=0;i<plan.input.count;++i){const int slot=plan.input.start+i;setFromEngine(kChordVoicings[slot],float(plan.voice[i]));setFromEngine(kChordOctaves[slot],float(plan.octave[i]));}setFromEngine(kSmoothOpen,0);setFromEngine(kSmoothStatus,3);}
            else {smoothNeedsPlan_.store(true,std::memory_order_release);setFromEngine(kSmoothStatus,6);}
            return;
        }
        if(smoothPlaying_&&!getInt(kSmoothPreview))stopSmoothPreview();
        if(!smoothPlaying_&&getInt(kSmoothPreview)&&matching){
            smoothAudition_=plan;smoothPlaying_=true;smoothGate_=false;smoothStep_=smoothNext_=smoothOff_=0;
            const double bpm=ctx.transport.tempoValid?std::clamp(ctx.transport.bpm,40.,240.):90.;smoothBeat_=std::max(1,int(sr_*60./bpm));setFromEngine(kSmoothStatus,2);
        }else if(!matching&&smoothNeedsPlan_.load(std::memory_order_acquire))setFromEngine(kSmoothStatus,6);
    }
    void renderSmoothStep(){
        if(!smoothPlaying_)return;
        if(!getInt(kSmoothPreview)){stopSmoothPreview();return;}
        if(smoothGate_&&!getBool(kChordGlide)&&smoothOff_--<=0){for(auto& v:voices_)if(v.owner==4096)release(v);smoothGate_=false;}
        if(smoothNext_--<=0){
            if(smoothStep_>=smoothAudition_.input.count){stopSmoothPreview();setFromEngine(kSmoothStatus,1);return;}
            const int i=smoothStep_++,slot=smoothAudition_.input.start+i;const auto& chord=smoothAudition_.input.chords[slot];std::array<int,6> notes{},reuse{};int reused=0;
            const int count=donut::chordNotes(chord[0],chord[1],smoothAudition_.voice[i],smoothAudition_.octave[i]-2,notes);
            if(getBool(kChordGlide))connectChordNotes(notes,count,4096,0,96,4096,true);
            else {
                for(int v=0;v<N&&reused<6;++v)if(voices_[v].owner==4096&&(voices_[v].amp.isActive()||voices_[v].wait))reuse[reused++]=v;
                for(int n=0;n<count;++n)start(4096,0,notes[n],96,int(n*raw_[kSpread]*.001f*sr_),n<reused?reuse[n]:-1);
                for(int n=count;n<reused;++n)release(voices_[reuse[n]]);
            }
            smoothNext_=smoothBeat_-1;smoothOff_=int(smoothBeat_*.8f);smoothGate_=true;
        }
    }
    struct Voice {PolyBlepOsc a,b;WavetableSine sineA,sineB,sub,fm;donut::AnalogVoiceFilter filter;ADSR amp,env;Smoother pitch;int owner=-1,channel=0,wait=0,note=60,glideWait=0;uint64_t order=0;float velocity=1,velocityGain=1;bool dirty=true;};
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
    void applySoundExtras(int patch){setFromEngine(kBodyTone,0);for(int i=kLfo2Shape;i<=kLfo2Division;++i)setFromEngine(i,param(i).def);setFromEngine(kLowCut,kPatchExtras[patch][0]);setFromEngine(kDrift,kPatchExtras[patch][1]);setFromEngine(kTremRate,kPatchExtras[patch][2]);setFromEngine(kTremDepth,kPatchExtras[patch][3]);setFromEngine(kOutputTrim,kPatchExtras[patch][4]);}
    donut::BodyTone bodyTone_;LFO lfo2_;Smoother lfo2Signal_;std::array<Smoother,5> lfo2Weights_;float lfo2TargetWeights_[5]{};
    float sr_=44100,delaySamples_=22050,dcIn_=0,dcOut_=0;LFO tremolo_,lfo_,vibrato_,drift_,chorusLfo_;Xorshift32 noise_{1234567};Smoother gain_,delayTime_,panLeft_,panRight_,motionGain_;DelayLine chorus_,delay_[2];
    static float flush(float x){return std::fabs(x)<1e-25f?0.f:x;}
    void clearNotes(){bodyTone_.reset();chordLead_.fill(-1);touchOrder_.fill(0);smoothPlaying_=smoothGate_=false;setFromEngine(kSmoothPreview,0);for(auto& f:lowCut_)f.reset();for(int i=0;i<16;++i){touchHeld_[i]=false;setFromEngine(kChordTouches[i],0);}setFromEngine(kTouchStop,0);for(auto& v:voices_){v.amp.reset();v.env.reset();v.filter.reset();v.wait=v.glideWait=0;v.owner=-1;}std::memset(held_,0,sizeof held_);std::memset(order_,0,sizeof order_);std::fill(std::begin(sustain_),std::end(sustain_),false);std::fill(std::begin(bend_),std::end(bend_),0.f);std::fill(std::begin(wheel_),std::end(wheel_),0.f);}
    void release(Voice& v){v.glideWait=0;v.wait=0;v.amp.noteOff();v.env.noteOff();}
    int allocate(){const int first=mono()?1:0;for(int i=first;i<N;++i)if(!voices_[i].amp.isActive()&&!voices_[i].wait)return i;int best=first;for(int i=first+1;i<N;++i)if((voices_[i].amp.isReleasing()&&!voices_[best].amp.isReleasing())||(voices_[i].amp.isReleasing()==voices_[best].amp.isReleasing()&&voices_[i].order<voices_[best].order))best=i;return best;}
    void start(int owner,int ch,int note,int vel,int wait=0,int forced=-1,bool legato=false,bool allowGlide=true){
        Voice& v=voices_[forced>=0?forced:allocate()];const bool sounding=v.amp.isActive(),connected=sounding&&v.amp.isHeld();
        v.glideWait=0;v.owner=owner;v.channel=ch;v.note=note;v.order=++clock_;v.velocity=vel/127.f;v.velocityGain=1-(1-v.velocity)*p_[kExpression]*.01f;v.dirty=true;
        if(allowGlide&&sounding&&raw_[kGlide]>0&&(int(std::lround(raw_[kGlideLegato]))==0||v.amp.isHeld()))v.pitch.setTarget(float(note));else v.pitch.snap(float(note));
        if(!sounding)v.filter.reset();
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
    void releaseUnheldMidi(int ch){if(getBool(kChordGlide)&&chordLead_[ch]>=0&&chordLead_[ch]<2048&&!held_[ch][chordLead_[ch]%128])endChordOwner(chordLead_[ch],ch);for(int i=mono()?1:0;i<N;++i){auto& v=voices_[i];if(v.owner>=0&&v.owner<2048&&v.channel==ch&&!held_[ch][v.owner%128])release(v);}}
    void monoFallback(){int bc=-1,bn=-1;uint64_t newest=0;for(int ch=0;ch<16;++ch)for(int n=0;n<128;++n)if(held_[ch][n]&&melodyNote(n)&&order_[ch][n]>=newest){newest=order_[ch][n];bc=ch;bn=n;}if(bc>=0)start(bc*128+bn,bc,bn,velocity_[bc][bn],0,0,int(std::lround(raw_[kVoiceMode]))==1);else release(voices_[0]);}
    void startChord(int slot,int owner,int ch,int vel,bool refresh=false,bool returning=false){
            if(getBool(kChordGlide)){
                if(refresh&&chordLead_[ch]!=owner)return;
                startGlidingChord(slot,owner,ch,vel,refresh,returning);return;
            }
            if(!refresh)chordLead_[ch]=owner;
            std::array<int,6> reuse{};int oldCount=0,used=0;
            if(refresh)for(int i=0;i<N&&oldCount<6;++i)if(voices_[i].owner==owner&&(voices_[i].amp.isHeld()||voices_[i].wait))reuse[oldCount++]=i;
            const int root=getInt(kChordRoots[slot]),quality=getInt(kChordQualities[slot]),voicing=getInt(kChordVoicings[slot]);
            std::array<int,6> notes{};const int count=donut::chordNotes(root,quality,voicing,getInt(kChordOctaves[slot])-2,notes);
            for(int i=0;i<count;++i){start(owner,ch,notes[i],vel,int(i*raw_[kSpread]*.001f*sr_),used<oldCount?reuse[used]:-1,refresh);++used;}
            for(int i=used;i<oldCount;++i)release(voices_[reuse[i]]);
    }
    std::array<int,16> chordLead_{};
    std::array<uint64_t,16> touchOrder_{};
    bool chordOwnerHeld(int owner,int ch)const{
        if(owner>=2048&&owner<2064)return ch==0&&getBool(kChordTouches[owner-2048])&&owner-2048<activeSlots();
        return owner>=0&&owner<2048&&owner/128==ch&&held_[ch][owner%128]&&chordSlot(owner%128)>=0;
    }
    int newestChord(int ch,int excluded)const{
        uint64_t newest=0;int owner=-1;
        for(int note=0;note<128;++note)if(ch*128+note!=excluded&&held_[ch][note]&&chordSlot(note)>=0&&order_[ch][note]>=newest){newest=order_[ch][note];owner=ch*128+note;}
        if(ch==0)for(int slot=0;slot<activeSlots();++slot)if(2048+slot!=excluded&&getBool(kChordTouches[slot])&&touchOrder_[slot]>=newest){newest=touchOrder_[slot];owner=2048+slot;}
        return owner;
    }
    void endChordOwner(int owner,int ch){
        if(getBool(kChordGlide)&&chordLead_[ch]==owner){
            const int previous=newestChord(ch,owner);
            if(previous>=0){const int slot=previous>=2048?previous-2048:chordSlot(previous%128);startChord(slot,previous,ch,previous>=2048?100:velocity_[ch][previous%128],false,true);}
            else chordLead_[ch]=-1;
        }
        for(int i=mono()?1:0;i<N;++i)if(voices_[i].owner==owner)release(voices_[i]);
    }
    void connectChordNotes(const std::array<int,6>& notes,int count,int owner,int ch,int vel,int previous,bool connect,bool rearticulate=false){
        std::array<int,6> old{},match{};int oldCount=0;match.fill(-1);
        if(connect)for(int i=mono()?1:0;i<N&&oldCount<6;++i)if(voices_[i].owner==previous&&voices_[i].channel==ch&&(voices_[i].amp.isHeld()||voices_[i].wait))old[oldCount++]=i;
        int reserved=0;
        for(int n=0;n<count;++n)for(int j=0;j<oldCount;++j)if(!(reserved&(1<<j))&&voices_[old[j]].note==notes[n]){match[n]=j;reserved|=1<<j;break;}
        // At most five voices: bounded subset assignment minimizes travel after
        // exact common notes are reserved. No allocations or general solver.
        int cost[7][64],parent[7][64],choice[7][64];
        for(int i=0;i<7;++i)for(int mask=0;mask<64;++mask){cost[i][mask]=10000;parent[i][mask]=-1;choice[i][mask]=-1;}
        cost[0][reserved]=0;
        for(int n=0;n<count;++n)for(int mask=0;mask<(1<<oldCount);++mask)if(cost[n][mask]<10000){
            if(match[n]>=0){if(cost[n][mask]<cost[n+1][mask]){cost[n+1][mask]=cost[n][mask];parent[n+1][mask]=mask;choice[n+1][mask]=match[n];}continue;}
            if(cost[n][mask]+128<cost[n+1][mask]){cost[n+1][mask]=cost[n][mask]+128;parent[n+1][mask]=mask;choice[n+1][mask]=-1;}
            for(int j=0;j<oldCount;++j)if(!(mask&(1<<j))){const int target=mask|(1<<j),next=cost[n][mask]+std::abs(voices_[old[j]].note-notes[n]);if(next<cost[n+1][target]){cost[n+1][target]=next;parent[n+1][target]=mask;choice[n+1][target]=j;}}
        }
        int best=0;for(int mask=1;mask<(1<<oldCount);++mask)if(cost[count][mask]<cost[count][best])best=mask;
        for(int n=count;n>0;--n){match[n-1]=choice[n][best];best=parent[n][best];}
        int used=0;
        for(int n=0;n<count;++n){
            if(match[n]>=0){const int index=old[match[n]];used|=1<<match[n];auto& v=voices_[index];
                if(v.note==notes[n]&&!rearticulate){v.owner=owner;v.order=++clock_;continue;}
                const float priorPitch=v.pitch.value(),priorTarget=v.pitch.target();
                start(owner,ch,notes[n],vel,rearticulate?int(n*raw_[kSpread]*.001f*sr_):0,index,!rearticulate,!rearticulate);
                const int offset=int(n*raw_[kSpread]*.001f*sr_);
                if(!rearticulate&&offset>0){if(raw_[kGlide]>0)v.pitch.setTarget(priorTarget);else v.pitch.snap(priorPitch);v.glideWait=offset;}
            }
        }
        for(int j=0;j<oldCount;++j)if(!(used&(1<<j)))release(voices_[old[j]]);
        for(int n=0;n<count;++n)if(match[n]<0)start(owner,ch,notes[n],vel,int(n*raw_[kSpread]*.001f*sr_),-1,false,false);
    }
    void startGlidingChord(int slot,int owner,int ch,int vel,bool refresh,bool returning){
        int previous=chordLead_[ch];
        if(previous<0||(!returning&&!refresh&&!chordOwnerHeld(previous,ch)))previous=newestChord(ch,owner);
        const bool connect=previous>=0;
        std::array<int,6> notes{};
        const int count=donut::chordNotes(getInt(kChordRoots[slot]),getInt(kChordQualities[slot]),getInt(kChordVoicings[slot]),getInt(kChordOctaves[slot])-2,notes);
        connectChordNotes(notes,count,owner,ch,vel,previous,connect,previous==owner&&!refresh&&!returning);
        chordLead_[ch]=owner;
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
        else if(e.isNoteOff()){if(held_[ch][n])--held_[ch][n];if(held_[ch][n])return;if(!sustain_[ch]){if(mono()&&voices_[0].owner==ch*128+n)monoFallback();endChordOwner(ch*128+n,ch);}}
        else if(e.isPitchBend())bend_[ch]=e.pitchBend()/8192.f;
        else if(e.isController()){
            if(e.controller()==64){sustain_[ch]=e.controllerValue()>=64;if(!sustain_[ch]){if(mono()&&voices_[0].channel==ch)monoFallback();releaseUnheldMidi(ch);}}
            else if(e.controller()==1)wheel_[ch]=e.controllerValue()/127.f;
            else if(e.controller()==120||e.controller()==123){chordLead_[ch]=-1;if(ch==0)stopSmoothPreview();if(ch==0)for(int i=0;i<16;++i){touchHeld_[i]=false;setFromEngine(kChordTouches[i],0);}std::fill(std::begin(held_[ch]),std::end(held_[ch]),0);sustain_[ch]=false;for(auto& v:voices_)if(v.channel==ch){release(v);if(e.controller()==120){v.amp.reset();v.env.reset();}}if(mono()&&voices_[0].channel==ch)monoFallback();}
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
