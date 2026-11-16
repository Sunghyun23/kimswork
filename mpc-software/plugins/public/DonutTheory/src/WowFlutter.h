#pragma once
#include "dsp/lfo.h"
#include <algorithm>

// Original pitch modulation, not an audio tape-delay model. All voices share
// one transport-like drift; detune between oscillators is left intact.
class WowFlutter {
    mpc::dsp::LFO wow_, wander_, flutter_;
public:
    void prepare(double controlRate) {
        wow_.setSampleRate(controlRate); wander_.setSampleRate(controlRate);
        flutter_.setSampleRate(controlRate);
        wow_.setFrequency(.43f); wander_.setFrequency(.137f);
        flutter_.setFrequency(6.73f); reset();
    }
    void reset() { wow_.reset(); wander_.reset(.27f); flutter_.reset(.61f); }
    float process(float percent) {
        const float amount=std::clamp(percent*.01f,0.f,1.f);
        const float wow=.75f*wow_.process()+.25f*wander_.process();
        const float flutter=flutter_.process();
        // Quadratic taper: subtle first half; at full, <=43 cents total.
        return amount*amount*(.35f*wow+.08f*flutter);
    }
};
