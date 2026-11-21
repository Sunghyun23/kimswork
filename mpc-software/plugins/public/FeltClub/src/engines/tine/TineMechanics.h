#pragma once
#include <algorithm>
#include <cmath>
namespace felt {
// Exact normal-coordinate solution of two coupled linear resonators. The
// unforced update is a rotation/contraction in mass-normalized energy space.
// This does not claim passivity of the complete hammer/discrete audio system.
struct TineToneBar {
 double x[2]{},v[2]{},u[2][2]{},omega[2]{},cosine[2]{},sine[2]{},radius[2]{},massRoot[2]{1,1};
 double input[2]{},forceX[2]{},forceV[2]{},baseOmega[2]{},lastPitch=-1;float sr=44100;
 void reset(){x[0]=x[1]=v[0]=v[1]=0;}
 void configure(double frequency,double barFrequency,double mass,double barMass,double spring,double damping,double r0,double r1,float rate){
  sr=rate;lastPitch=-1;reset();massRoot[0]=std::sqrt(mass);massRoot[1]=std::sqrt(barMass);
  const double w=6.2831853071795864769*frequency,wb=6.2831853071795864769*barFrequency;
  const double a=w*w+spring/mass,d=wb*wb+spring/barMass,b=-spring/(massRoot[0]*massRoot[1]);
  const double theta=.5*std::atan2(2*b,a-d),c=std::cos(theta),s=std::sin(theta);
  u[0][0]=-s;u[1][0]=c;u[0][1]=c;u[1][1]=s;
  if(b==0){u[0][0]=a<=d?1:0;u[1][0]=a<=d?0:1;u[0][1]=a<=d?0:1;u[1][1]=a<=d?1:0;}
  const double split=std::hypot(a-d,2*b),lo=.5*(a+d-split),hi=.5*(a+d+split);
  // Retune the loaded assembly so its lower normal mode is the requested note.
  const double scale=w*w/std::max(1.,lo);baseOmega[0]=w;baseOmega[1]=std::sqrt(hi*scale);
  for(int j=0;j<2;++j){input[j]=u[0][j]/massRoot[0];double relative=u[0][j]/massRoot[0]-u[1][j]/massRoot[1];radius[j]=(u[0][j]*u[0][j]*r0+u[1][j]*u[1][j]*r1)*std::exp(-.5*damping*relative*relative/sr);}
  pitch(1);
 }
 void pitch(double ratio){
  if(ratio==lastPitch)return;lastPitch=ratio;
  for(int j=0;j<2;++j){omega[j]=std::min(double(sr)*2.5132741228718346,baseOmega[j]*ratio);double angle=omega[j]/sr;cosine[j]=std::cos(angle);sine[j]=std::sin(angle);forceX[j]=input[j]*(1-cosine[j])/(omega[j]*omega[j]);forceV[j]=input[j]*sine[j]/omega[j];}
 }
 void tick(double force,double damper0=1,double damper1=1){
  for(int j=0;j<2;++j){double r=radius[j]*(u[0][j]*u[0][j]*damper0+u[1][j]*u[1][j]*damper1);double q=x[j],speed=v[j];x[j]=r*(cosine[j]*q+sine[j]*speed/omega[j])+forceX[j]*force;v[j]=r*(cosine[j]*speed-omega[j]*sine[j]*q)+forceV[j]*force;if(std::abs(x[j])<1e-24&&std::abs(v[j])<1e-20)x[j]=v[j]=0;}
 }
 double position(int part)const{return (u[part][0]*x[0]+u[part][1]*x[1])/massRoot[part];}
 double energy()const{return .5*(v[0]*v[0]+omega[0]*omega[0]*x[0]*x[0]+v[1]*v[1]+omega[1]*omega[1]*x[1]*x[1]);}
};
// A reduced magnetic flux geometry, evaluated in two transverse coordinates.
// Axial gap increases with the curved tip trajectory. These are equations of
// geometry, not recorded samples or a manufacturer-derived impulse response.
struct TinePickup2D {
 float gap=.001f,verticalOffset=.9f,horizontalOffset=.12f,length=.07f,rate=44100;
 double lastV=0,lastH=0,lastAverage=0;
 void configure(float distance,float offset,float tineLength,float sr){gap=distance;verticalOffset=offset;length=tineLength;rate=sr;reset();}
 double flux(double vertical,double horizontal)const{
  double a=vertical/gap,b=horizontal/gap;
  double axial=1+(vertical*vertical+horizontal*horizontal)/(2*length*gap);
  double z=a-verticalOffset,h=b-horizontalOffset,r2=axial*axial+z*z+h*h;
  return axial/(r2*std::sqrt(r2));
 }
 void reset(){lastV=lastH=0;lastAverage=flux(0,0);}
 float tick(float vertical,float horizontal){
  // Subsample flux averaging softens the readout; it is not a claim of perfect
  // anti-aliasing. Difference the flux itself, with a fixed coil scale.
  double average=(flux(lastV,lastH)+4*flux(.5*(lastV+vertical),.5*(lastH+horizontal))+flux(vertical,horizontal))/6;
  double voltage=-(average-lastAverage)*rate/(6.2831853071795864769*261.6255653006);
  lastV=vertical;lastH=horizontal;lastAverage=average;return float(voltage);
 }
};
}
