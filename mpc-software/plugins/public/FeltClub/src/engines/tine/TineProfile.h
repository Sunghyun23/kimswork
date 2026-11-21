#pragma once
// Global physical/geometry calibration, not a sample or note-indexed wavetable.
// Runtime values are constexpr. The mutable bridge exists in offline tests only.
namespace felt {
struct TineProfile {
 float gapScale=1,gapKey=0,offset=.52f,velocityPower=.85f,stiffnessScale=1,upperScale=1;
 float decayScale=1,decayKey=.15f,lossLinear=.12f,couplingScale=1,squash=1,speedFloor=.35f;
 float level=1,levelKey=0;
};
#ifdef FELT_TINE_CALIBRATION
inline thread_local TineProfile tineProfile;
#else
// Base geometry from round2;0.2.2opens hammer contact and upper-mode readout
// after full-instrument clarity review. Prior0.2.1fit scores do not certify this profile.
//12global physical coefficients +2level controls; no note/sample spectral tables.
inline constexpr TineProfile tineProfile{0.173750848f,-0.277859986f,1.04890192f,1.84331942f,0.75f,0.425f,2.82842708f,0.335919797f,0.220624521f,1.51083696f,0.00449999981f,0.304708749f,0.18178582f,0.253385828f};
#endif
}
