#pragma once
#include "dsp/oscillator.h"
#include "WavePalette.h"
#include <algorithm>
#include <cmath>

namespace donut {
// Existing SDK paths are retained bit-for-bit for waves0..3. Added waves are
// band-limited mixtures or finite harmonic sets; no allocation or file access.
class ExpandedOscillator {
    struct State {
        mpc::dsp::PolyBlepOsc blep;
        mpc::dsp::WavetableSine sine;
        int wave=2;
        float sr=44100,hz=440,gainA=0,gainB=0,norm=1;
        float phase()const{return wave==0||wave>=6?sine.phase():blep.phase();}
        void reset(float p){blep.resetPhase(p);sine.resetPhase(p);}
        void shift(float p){blep.resetPhase(blep.phase()+p);sine.resetPhase(sine.phase()+p);}
        void harmonics(){
            const float a=wave==6?2.f:3.f,b=wave==6?3.f:5.f;
            gainA=.20f*std::clamp((sr*.48f-hz*a)/(sr*.08f),0.f,1.f);
            gainB=.08f*std::clamp((sr*.48f-hz*b)/(sr*.08f),0.f,1.f);
            norm=1.f/(.72f+gainA+gainB);
        }
        void sampleRate(double sampleRate){sr=float(sampleRate);blep.setSampleRate(sampleRate);sine.setSampleRate(sampleRate);harmonics();}
        void frequency(float f){hz=std::max(0.f,f);blep.setFrequency(hz);sine.setFrequency(std::min(hz,sr*.49f));harmonics();}
        void setWave(int w,bool alignNew){
            w=std::clamp(w,0,kWaveCount-1);const float p=phase();
            const bool changed=w!=wave;wave=w;blep.setWave(legacyWave(w));harmonics();
            if(changed&&(alignNew||w>=4))reset(p);
        }
        float process(float pm){
            if(wave==0)return sine.process(pm);
            if(wave<4)return blep.process();
            if(wave<6){const float a=blep.process(),b=sine.process();return .65f*a+(wave==4?-.35f:.35f)*b;}
            const float p=sine.phase(),fundamental=sine.process();
            const float a=wave==6?2.f:3.f,b=wave==6?3.f:5.f;
            return (.72f*fundamental+gainA*mpc::dsp::WavetableSine::lookup(p*a)+gainB*mpc::dsp::WavetableSine::lookup(p*b))*norm;
        }
    } current_,next_;
    int pendingWave_=2,remaining_=0,total_=1;
    float phaseSetting_=0,pendingPhase_=0,goalPhase_=0,sr_=44100;
    static float delta(float a,float b){return std::remainder((a-b)/360.f,1.f);}
    void settle(){if(remaining_){current_=next_;phaseSetting_=goalPhase_;remaining_=0;}}
    void transition(){
        const float shift=delta(pendingPhase_,phaseSetting_);
        if(pendingWave_==current_.wave&&std::abs(shift)<1e-7f){phaseSetting_=pendingPhase_;return;}
        next_=current_;next_.setWave(pendingWave_,true);next_.shift(shift);
        goalPhase_=pendingPhase_;total_=std::max(1,int(sr_*.020f));remaining_=total_;
    }
public:
    void setSampleRate(double sr){sr_=float(sr);current_.sampleRate(sr);next_.sampleRate(sr);}
    void setFrequency(float hz){current_.frequency(hz);next_.frequency(hz);}
    void setPulseWidth(float pw){current_.blep.setPulseWidth(pw);next_.blep.setPulseWidth(pw);}
    void setWave(int wave,bool sounding=true){pendingWave_=std::clamp(wave,0,kWaveCount-1);if(!sounding){settle();current_.setWave(pendingWave_,false);}}
    void resetPhase(float phase=0){settle();current_.setWave(pendingWave_,false);current_.reset(phase);next_=current_;remaining_=0;}
    float phase()const{return current_.phase();}
    void snapPhaseSetting(float degrees){phaseSetting_=pendingPhase_=goalPhase_=degrees;}
    void setPhaseSetting(float degrees,bool sounding){
        pendingPhase_=std::clamp(degrees,0.f,360.f);
        if(!sounding){settle();const float shift=delta(pendingPhase_,phaseSetting_);if(std::abs(shift)>1e-7f)current_.shift(shift);phaseSetting_=pendingPhase_;}
    }
    // New isolated voices restart exactly; existing release tails use a2ms
    // transition so a mono rearticulation cannot create a hard phase jump.
    void restart(float phase,int wave,bool sounding){
        pendingWave_=std::clamp(wave,0,kWaveCount-1);
        if(!sounding){resetPhase(phase);return;}
        settle();next_=current_;next_.setWave(pendingWave_,true);next_.reset(phase);
        goalPhase_=pendingPhase_;total_=std::max(1,int(sr_*.002f));remaining_=total_;
    }
    float process(float pm=0){
        if(!remaining_)transition();
        const float a=current_.process(pm);
        if(!remaining_)return a;
        const float b=next_.process(pm),mix=float(total_-remaining_+1)/float(total_);
        const float out=a+(b-a)*mix;
        if(!--remaining_){current_=next_;phaseSetting_=goalPhase_;}
        return out;
    }
};
}
