#pragma once
#include <algorithm>
#include <cmath>
namespace felt {
// Small-signal input approximation from the 200A reed-board drawing:
// Rs22k, coupling22nF, shunt470k/220pF. Transistor input loading and the
// complete active feedback/output circuit are NOT identified by this model.
struct WurliInputCircuit {
 double highState=0,fastState=0,feedbackState=0;
 double highG=0,fastG=0,feedbackG=0,gain=1;
 void prepare(double rate){
  constexpr double R=470000,Rs=22000,Cc=22e-9,Cp=220e-12;
  double sum=R*Cp+(Rs+R)*Cc,product=R*Cp*Rs*Cc;
  double slow=.5*(sum+std::sqrt(sum*sum-4*product)),fast=product/slow;
  auto coefficient=[&](double hz){double g=std::tan(3.14159265358979323846*std::min(hz,rate*.4)/rate);return g/(1+g);};
  highG=coefficient(1/(6.2831853071795864769*slow));fastG=coefficient(1/(6.2831853071795864769*fast));
  // Isolated RC compensation surrogate for150k/100pF. Not a solved Miller
  // pole of the full transistor feedback loop.
  feedbackG=coefficient(1/(6.2831853071795864769*150000*100e-12));gain=R*Cc/slow;reset();
 }
 void reset(){highState=fastState=feedbackState=0;}
 static double low(double x,double g,double&s){double v=(x-s)*g,y=v+s;s=y+v;return y;}
 double process(double current){double x=current-low(current,highG,highState);x=low(x,fastG,fastState);return low(x,feedbackG,feedbackState)*gain;}
};
}
