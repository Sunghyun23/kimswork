#pragma once
#include "../../VoicingTypes.h"
namespace felt {
#ifdef FELT_UPRIGHT_VOICING_CALIBRATION
inline thread_local DemoVoicing uprightVoicing;
#else
inline constexpr DemoVoicing uprightVoicing{1.f,1.f,0.f,1.f,0.f};
#endif
}
