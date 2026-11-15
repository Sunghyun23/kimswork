// SPDX-License-Identifier: MIT
// Original implementation of a four-stage nonlinear ladder-inspired voice path.
// Differential-pair principle: Huovilainen, DAFx 2004, doi/archive in engine.md.
// Trapezoidal integration + simultaneous Newton solve (not the paper's Euler code).
#pragma once
#include <algorithm>
#include <array>
#include <cmath>
#include "dsp/svf.h"

namespace donut {
// Bounded rational differential-pair approximation, with its exact derivative.
inline float pair(float x) {
    x=std::clamp(x,-3.f,3.f);
    const float xx=x*x;
    return x*(27.f+xx)/(27.f+9.f*xx);
}
inline float pairSlope(float x) {
    if(std::abs(x)>=3.f)return 0.f;
    const float xx=x*x,den=27.f+9.f*xx,n=xx-9.f;
    return 9.f*n*n/(den*den);
}
inline void pairWithSlope(float x,float& value,float& slope){
    if(std::abs(x)>=3.f){value=std::copysign(1.f,x);slope=0;return;}
    const float xx=x*x,inv=1.f/(27.f+9.f*xx),n=xx-9.f;
    value=x*(27.f+xx)*inv;slope=9.f*n*n*inv*inv;
}

class AnalogLadder {
public:
    void setSampleRate(double sr) { sr_=float(sr); }
    // Control-rate only. tan() is outside the per-sample feedback iteration.
    void set(float cutoff,float resonance) {
        g_=std::tan(3.14159265359f*std::clamp(cutoff,5.f,sr_*.45f)/(2.f*sr_));
        // Keep the top below autonomous oscillation: released voices remain silent.
        feedback_=3.95f*std::clamp(resonance,0.f,1.f);
    }
    void reset() { z_.fill(0);y_.fill(0);history_.fill(0);previous_=0;head_=0; }
#ifdef DONUT_LADDER_TEST
    float maximumResidual=0;
#endif
    float process(float input) {
        // Linear 2x interpolation; 17-tap low-pass decimation after nonlinearity.
        step(.5f*(previous_+input));step(input);previous_=input;
        float out=0;
        for(int i=0;i<17;++i)out+=fir_[i]*history_[(head_+i)%17];
        return out;
    }
private:
    void step(float input) {
        // yi = zi + g*(sat(previous stage) - sat(yi)). The first stage
        // receives input - feedback*y4. Solve all four outputs together, so
        // resonance has no extra one-sample delay. Fixed upper CPU bound.
        for(int iteration=0;iteration<6;++iteration){
            float f[4],invD[4],s[4],slope[4],a[4],b[4];
            for(int i=0;i<4;++i)pairWithSlope(y_[i],s[i],slope[i]);
            const float u=input-feedback_*y_[3];
            float su,du;pairWithSlope(u,su,du);
            f[0]=y_[0]-z_[0]-g_*(su-s[0]);
            float error=std::abs(f[0]);
            for(int i=1;i<4;++i){f[i]=y_[i]-z_[i]-g_*(s[i-1]-s[i]);error=std::max(error,std::abs(f[i]));}
            if(iteration>0&&error<1.e-7f)break;
            for(int i=0;i<4;++i)invD[i]=1.f/(1.f+g_*slope[i]);
            a[0]=1;b[0]=0;
            for(int i=1;i<4;++i){const float c=g_*slope[i-1];a[i]=c*a[i-1]*invD[i];b[i]=(f[i]+c*b[i-1])*invD[i];}
            const float c=g_*feedback_*du;
            const float delta=(f[0]-c*b[3])*invD[0]/(1.f+c*a[3]*invD[0]);
            // A common damping factor preserves the coupled Newton direction.
            float largest=std::abs(delta);
            for(int i=1;i<4;++i)largest=std::max(largest,std::abs(a[i]*delta+b[i]));
            const float damping=largest>.5f?.5f/largest:1.f;
            for(int i=0;i<4;++i)y_[i]-=damping*(a[i]*delta+b[i]);
        }
#ifdef DONUT_LADDER_TEST
        for(int i=0;i<4;++i){const float previous=i?y_[i-1]:input-feedback_*y_[3];
            maximumResidual=std::max(maximumResidual,std::abs(y_[i]-z_[i]-g_*(pair(previous)-pair(y_[i]))));}
#endif
        for(int i=0;i<4;++i){z_[i]=2.f*y_[i]-z_[i];if(std::abs(z_[i])<1.e-20f)z_[i]=0;}
        head_=(head_+16)%17;history_[head_]=y_[3];
    }
    float sr_=44100,g_=.1f,feedback_=0,previous_=0;
    std::array<float,4> z_{},y_{};
    std::array<float,17> history_{};int head_=0;
    // Windowed-sinc halfband (Blackman); unity DC, zeros at even taps.
    inline static constexpr float fir_[17]={0,-.00066499f,0,.01095199f,0,-.05884401f,0,.2986387749f,.4998364813f,.2986387749f,0,-.05884401f,0,.01095199f,0,-.00066499f,0};
};

// Existing non-LP modes stay available as SVF extensions; low-cut is separate.
class AnalogVoiceFilter {
    AnalogLadder ladder_;mpc::dsp::SVF other_;
public:
    void setSampleRate(double sr){ladder_.setSampleRate(sr);other_.setSampleRate(sr);}
    void set(float cutoff,float resonance){ladder_.set(cutoff,resonance);other_.set(cutoff,resonance*.92f);}
    void reset(){ladder_.reset();other_.reset();}
    float process(float x,int mode){return mode==0?ladder_.process(x):other_.process(x,mode);}
};
// Tone drives the oscillator summer before the filter; it works with FX off.
inline float analogMixer(float x,float tone){return pair(x*(.7f+tone*.018f));}
// Gentle level-dependent VCA rounding, after the envelope and velocity scaling.
inline float analogVca(float x){return pair(x*1.15f)/1.15f;}
} // namespace donut
