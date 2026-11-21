#pragma once
#include "../../VoicingTypes.h"
namespace felt {
// Independent Wurli calibration. No table shared with another model.
#ifdef FELT_WURLI_VOICING_CALIBRATION
inline thread_local DemoVoicing wurliVoicing;
#else
inline constexpr DemoVoicing wurliVoicing{1.f,1.f,0.f,.75f,0.f};
#endif
}
