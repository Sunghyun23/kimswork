#pragma once
#include "engines/tine/TineVoice.h"
#include "engines/wurli/WurliVoice.h"
#include "engines/upright/UprightVoice.h"
#include "engines/felt/FeltVoice.h"
#include "engines/clav/ClavVoice.h"
#include "engines/muted/MutedVoice.h"
#include "engines/thumb/ThumbVoice.h"
#include "AcousticBody.h"
#include <variant>
#include <cassert>
namespace felt {
// Fixed-storage dispatcher only: each backend owns its complete audio state.
// No mutable mechanics/profile/envelope state is shared between EP engines.
struct Voice {
 std::variant<TineVoice,WurliVoice,UprightVoice,FeltVoice,ClavVoice,MutedVoice,ThumbVoice> engine;
 float sampleRate=44100;
 VoiceState&state(){return std::visit([](auto&v)->VoiceState&{return v;},engine);}
 const VoiceState&state()const{return std::visit([](const auto&v)->const VoiceState&{return v;},engine);}
 int model()const{return int(engine.index());}
 template<class F> decltype(auto)visit(F&&f){return std::visit(std::forward<F>(f),engine);}
 template<class F> decltype(auto)visit(F&&f)const{return std::visit(std::forward<F>(f),engine);}
 void prepare(float rate){sampleRate=rate;visit([&](auto&v){v.prepare(rate);});}
 void stop(){visit([](auto&v){v.stop();});}
 template<class Backend>void select(){if(!std::holds_alternative<Backend>(engine)){engine.emplace<Backend>();std::get<Backend>(engine).prepare(sampleRate);}}
 void strike(int pitch,int velocity,int owner,int serial,const VoiceSettings&s,float tuning=440){
  switch(s.model){case 0:select<TineVoice>();break;case 1:select<WurliVoice>();break;case 2:select<UprightVoice>();break;case 3:select<FeltVoice>();break;case 4:select<ClavVoice>();break;case 5:select<MutedVoice>();break;case 6:select<ThumbVoice>();break;default:assert(false);return;}
  visit([&](auto&v){v.strike(pitch,velocity,owner,serial,s,tuning);});
 }
 void keyOff(float amount){visit([&](auto&v){v.keyOff(amount);});}
 void release(bool pedal){visit([&](auto&v){v.release(pedal);});}
 void setTone(float value){visit([&](auto&v){v.setTone(value);});}
 void setEnvelope(float a,float d,float s,float r){visit([&](auto&v){v.setEnvelope(a,d,s,r);});}
 void retune(int pitch,float milliseconds,float tuning=440){visit([&](auto&v){v.retune(pitch,milliseconds,tuning);});}
 void updatePitch(float ratio){visit([&](auto&v){v.updatePitch(ratio);});}
 float tick(){return visit([](auto&v){return v.tick();});}
};
}
