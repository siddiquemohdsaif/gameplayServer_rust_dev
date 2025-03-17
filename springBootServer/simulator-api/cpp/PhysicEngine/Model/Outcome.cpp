#include "Outcome.h"

Outcome::Outcome(OutcomeType type, Ball &ballA, Ball &ballB, double incidentSpeed)
    : type(type), ballA(ballA.coinCode), ballB(ballB.coinCode), incidentSpeed(incidentSpeed) {
    timestamp = std::chrono::high_resolution_clock::now().time_since_epoch();
}

Outcome Outcome::pot(Ball &ballA, double incidentSpeed) {
    return Outcome(OutcomeType::Pot, ballA, ballA, incidentSpeed);
}

Outcome Outcome::cushion(Ball &ballA, double incidentSpeed) {
    return Outcome(OutcomeType::Cushion, ballA, ballA, incidentSpeed);
}

Outcome Outcome::collision(Ball &ballA, Ball &ballB, double incidentSpeed) {
    return Outcome(OutcomeType::Collision, ballA, ballB, incidentSpeed);
}

Outcome Outcome::hit(Ball &ballA, double incidentSpeed) {
    return Outcome(OutcomeType::Hit, ballA, ballA, incidentSpeed);
}
