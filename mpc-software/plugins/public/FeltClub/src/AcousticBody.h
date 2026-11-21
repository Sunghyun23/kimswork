#pragma once
#include "engines/upright/UprightBody.h"
#include "engines/felt/FeltBody.h"
namespace felt {
struct AcousticBodies {
 UprightBody upright;FeltBody felt;
 void prepare(float rate){upright.prepare(rate);felt.prepare(rate);}
 void reset(){upright.reset();felt.reset();}
 std::array<float,2> process(const std::array<float,7>& input,bool pedal,
                             float uprightProfiled=0,float uprightDetail=0,
                             const std::array<float,2>& uprightDirect={},
                             float feltProfiled=0,float feltDetail=0,
                             const std::array<float,2>& feltDirect={}){
  const auto uprightPressure=upright.render(input[2],pedal,uprightProfiled,uprightDetail,uprightDirect);
  const auto feltPressure=felt.render(input[3],pedal,feltProfiled,feltDetail,feltDirect);
  return {uprightPressure[0]+feltPressure[0],uprightPressure[1]+feltPressure[1]};
 }
};
}
