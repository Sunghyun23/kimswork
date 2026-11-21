#pragma once
#include "../../VoicingTypes.h"
namespace felt {
#ifdef FELT_MUTED_VOICING_CALIBRATION
inline thread_local DemoVoicing mutedVoicing;
#else
inline constexpr DemoVoicing mutedVoicing{1.f,1.f,0.f,1.f,0.f};
#endif
}
