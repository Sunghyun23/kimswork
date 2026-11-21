#pragma once
#include "../../VoicingTypes.h"
namespace felt {
#ifdef FELT_THUMB_VOICING_CALIBRATION
inline thread_local DemoVoicing thumbVoicing;
#else
inline constexpr DemoVoicing thumbVoicing{1.f,1.f,0.f,1.f,0.f};
#endif
}
