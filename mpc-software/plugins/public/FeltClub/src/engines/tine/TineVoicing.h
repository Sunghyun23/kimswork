#pragma once
#include "../../VoicingTypes.h"
namespace felt {
// Independent Tine calibration. No table shared with another model.
#ifdef FELT_TINE_VOICING_CALIBRATION
inline thread_local DemoVoicing tineVoicing;
#else
inline constexpr DemoVoicing tineVoicing{1.f,1.f,0.f,1.f,0.f};
#endif
}
