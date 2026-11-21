#pragma once
#include <algorithm>
#include <array>
#include <cmath>

namespace felt::upright {
inline float bassRegisterWeight(int note) {
 const float lower=std::clamp((note-38)/3.f,0.f,1.f);
 const float upper=std::clamp((53-note)/3.f,0.f,1.f);
 return lower*upper;
}

inline float bassBandWeight(float distance,float width) {
 const float x=std::clamp(1.f-std::abs(distance)/width,0.f,1.f);
 return x*x*(3.f-2.f*x);
}

// Bridge-to-air pressure fit from isolated upright strikes. Use the
// partial's absolute frequency so adjacent keys do not inherit the same
// cut when their harmonics miss the measured band.
inline float bassPressureGain(int note,int harmonic,float hz,float weight) {
 float db=-7.5f*bassBandWeight(hz-349.4f,18.f)
          -4.f*bassBandWeight(hz-604.f,38.f);
 if(harmonic==1)
  db+=1.6f*bassBandWeight(float(note-41),.99f)+5.8f*bassBandWeight(float(note-43),1.4f);
 if(harmonic>=3)db-=2.5f*bassBandWeight(hz-294.f,13.f);
 return db==0.f?1.f:std::exp2(weight*db/6.0205999f);
}

using BassUnisonLoss=std::array<float,4>;
struct BassLossAnchor {
 int note;
 BassUnisonLoss scale;
};

// Positive multipliers on common-velocity loss for partials 1 through 4.
// Interpolate across unmeasured keys and return to the original law at
// both register boundaries. Differential motion keeps its own loss.
inline constexpr std::array<BassLossAnchor,9> BassLossAnchors{{
 {38, {1.000f,1.000f,1.000f,1.000f}},
 {41, {0.656f,1.100f,0.968f,0.402f}},
 {42, {0.879f,0.796f,0.685f,0.416f}},
 {43, {0.275f,0.492f,0.402f,0.431f}},
 {44, {0.637f,0.778f,0.402f,0.529f}},
 {46, {1.000f,0.658f,0.402f,0.465f}},
 {48, {1.698f,0.539f,0.402f,0.402f}},
 {50, {1.604f,0.687f,0.402f,0.402f}},
 {53, {1.000f,1.000f,1.000f,1.000f}},
}};

inline BassUnisonLoss bassUnisonLoss(int note) {
 if(note<=BassLossAnchors.front().note||note>=BassLossAnchors.back().note)
  return {1.f,1.f,1.f,1.f};
 for(std::size_t i=1;i<BassLossAnchors.size();++i) {
  const auto& above=BassLossAnchors[i];
  if(note>above.note)continue;
  const auto& below=BassLossAnchors[i-1];
  const float blend=float(note-below.note)/float(above.note-below.note);
  BassUnisonLoss scale{};
  for(std::size_t h=0;h<scale.size();++h)
   scale[h]=below.scale[h]+blend*(above.scale[h]-below.scale[h]);
  return scale;
 }
 return {1.f,1.f,1.f,1.f};
}
}
