#pragma once
#include "WurliField.h"
#include "WurliCircuit.h"
#include <algorithm>
#include <cmath>
namespace felt {
// Own 2D through-slot electrostatic field data, finite reed and solder geometry.
// Geometry ratios/load are hypotheses, not an identified200A field/circuit.
struct WurliPickup {
 double gap=.001,rest=-2.6,previous=0,previousCap=0,motionScale=6.15;
 float rate=44100,orientation=1;WurliInputCircuit input;double maxExcursion=0;
 double field(double z)const{
  double position=(std::clamp(z,double(wurli::FieldMin),double(wurli::FieldMax))-wurli::FieldMin)/wurli::FieldStep;
  int i=std::min(int(position),int(wurli::FieldCapacitance.size())-2);double t=position-i,t2=t*t,t3=t2*t;
  double a=wurli::FieldCapacitance[i],b=wurli::FieldCapacitance[i+1],ma=wurli::FieldSlope[i]*wurli::FieldStep,mb=wurli::FieldSlope[i+1]*wurli::FieldStep;
  return (2*t3-3*t2+1)*a+(t3-2*t2+t)*ma+(-2*t3+3*t2)*b+(t3-t2)*mb;
 }
 double capacitance(double displacement)const{return field(rest+orientation*displacement/gap*motionScale);}
 void reset(){previous=0;previousCap=capacitance(0);input.reset();maxExcursion=0;}
 void configure(float distance,float bark,float sr,bool leadUp=false){
  gap=std::max(.00001f,distance);rest=-(3.5-.7*std::clamp(double(bark),0.,1.));rate=sr;orientation=leadUp?-1.f:1.f;
  input.prepare(4*rate);reset();
 }
 float tick(float displacement,float /*frequency*/){
  double result=0;maxExcursion=std::max(maxExcursion,std::abs(rest+orientation*displacement/gap*motionScale));
  // Four readout substeps and a load low-pass reduce nonlinear aliasing.
  // No perfect antialiasing or full mechanical oversampling claim.
  for(int j=1;j<=4;++j){double at=previous+(displacement-previous)*(j*.25);double c=capacitance(at);double current=-(c-previousCap)*(4*rate)*.00085;previousCap=c;result+=input.process(current);}
  previous=displacement;return float(result*.25);
 }
};
}
