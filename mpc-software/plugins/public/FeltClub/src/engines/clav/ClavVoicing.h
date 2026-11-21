#pragma once
#include "../../VoicingTypes.h"
namespace felt {
#ifdef FELT_CLAV_VOICING_CALIBRATION
inline thread_local DemoVoicing clavVoicing;
#else
// Compensate the compliant/two-pickup voicing energy; user Master is unchanged.
inline constexpr DemoVoicing clavVoicing{1.f,1.f,0.f,1.65f,0.f};
#endif
}
