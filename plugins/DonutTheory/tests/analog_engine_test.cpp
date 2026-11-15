#define DONUT_LADDER_TEST
#include "../src/AnalogLadder.h"
#include <cassert>
#include <iostream>

double gain(float hz,float cutoff,float res,float amplitude=.0001f) {
    donut::AnalogLadder f;f.setSampleRate(48000);f.set(cutoff,res);
    double energy=0;
    for(int n=0;n<48000;++n){float y=f.process(amplitude*std::sin(6.28318530718*hz*n/48000));assert(std::isfinite(y));if(n>=24000)energy+=y*y;}
    return std::sqrt(energy/24000)*std::sqrt(2.)/amplitude;
}
int main(){
    const double dc=gain(50,1000,0),corner=gain(1000,1000,0),hi=gain(4000,1000,0);
    std::cout<<"LP gains 50/1000/4000Hz: "<<dc<<" / "<<corner<<" / "<<hi<<"\n";
    assert(dc>.97&&dc<1.01&&corner>.23&&corner<.27&&hi<.005);
    const double resonant=gain(1000,1000,.9f);
    std::cout<<"Resonant gain: "<<resonant<<"\n";assert(resonant>corner*5);
    const double small=gain(1000,1000,.8f,.02f),hot=gain(1000,1000,.8f,2.f);
    std::cout<<"Small/hot resonant gain: "<<small<<" / "<<hot<<"\n";assert(hot<small*.8); // Embedded nonlinear stages compress, even with FX off.
    for(float sr:{32000.f,44100.f,48000.f,96000.f})for(float res:{0.f,.5f,.95f,1.f}){
        donut::AnalogLadder f;f.setSampleRate(sr);
        for(int n=0;n<96000;++n){if(n%16==0)f.set(n%512<256?20.f:18000.f,res);
            const float y=f.process(2.f*std::sin(n*.173f));assert(std::isfinite(y)&&std::abs(y)<8.f);}
        assert(f.maximumResidual<1.e-4f);
        f.reset();f.set(1000,res);for(int n=0;n<128;++n)assert(f.process(0)==0);
    }
    // Same input history produces exactly the same output after reset.
    donut::AnalogLadder a,b;a.set(800,.8f);b.set(800,.8f);
    for(int n=0;n<20000;++n){const float x=std::sin(n*.04f);assert(a.process(x)==b.process(x));}
    a.reset();b.reset();assert(a.process(0)==b.process(0));
    assert(std::abs(donut::analogMixer(.7f,0)-donut::analogMixer(.7f,100))>.1f);
    assert(std::abs(donut::analogVca(.8f))<.8f);
    std::cout<<"PASS: four-pole slope, resonant feedback, nonlinear compression, fast modulation, rates and reset\n";
}
