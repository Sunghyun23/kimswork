#pragma once
#include <array>
namespace felt::thumb {
// Euler–Bernoulli beam, clamped at node 0, supported at 12/32,
// free at node 32; six mass-normalized modes. Original offline FEM.
inline constexpr std::array<float,6> BeamRatios{1.00000000f,6.66664531f,16.81835320f,22.31623600f,39.96673824f,55.25118027f};
inline constexpr std::array<float,6> PluckShape{1.00000000f,-0.84637690f,0.49325998f,-0.53464549f,0.50416708f,-0.20756146f};
inline constexpr std::array<float,6> TipShape{1.00000000f,-1.01515662f,0.70216804f,-0.82482625f,0.99226719f,-0.50361470f};
}
