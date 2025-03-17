#include "ConstantsCarrom.h"
#include <cmath>

const double ConstantsCarrom::mu = 0.18;
const double ConstantsCarrom::mur = 0.085;
const double ConstantsCarrom::g = 9.80665;
const double ConstantsCarrom::rho = 0.002;
const double ConstantsCarrom::m = 0.018;
const double ConstantsCarrom::R = 0.0202565;
const double ConstantsCarrom::Mz = ((mu * m * g * 2) / 3) * rho;
const double ConstantsCarrom::Mxy = (7.0 / (5 * std::sqrt(2))) * R * mur * m * g;
const double ConstantsCarrom::e = 0.97;
const double ConstantsCarrom::I = (2.0 / 5.0) * m * R * R;
