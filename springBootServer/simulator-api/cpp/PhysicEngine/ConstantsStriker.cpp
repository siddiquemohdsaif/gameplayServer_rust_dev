#include "ConstantsStriker.h"
#include <cmath>

const double ConstantsStriker::mu = 0.07;
const double ConstantsStriker::mur = 0.07;
const double ConstantsStriker::g = 9.80665;
const double ConstantsStriker::rho = 0.002;
const double ConstantsStriker::m = 0.035;
const double ConstantsStriker::R = 0.0268859;
const double ConstantsStriker::Mz = ((mu * m * g * 2) / 3) * rho;
const double ConstantsStriker::Mxy = (7.0 / (5 * std::sqrt(2))) * R * mur * m * g;
const double ConstantsStriker::e = 0.97;
const double ConstantsStriker::I = (2.0 / 5.0) * m * R * R;
const double ConstantsStriker::maxRejectVelocity = 1.2;
