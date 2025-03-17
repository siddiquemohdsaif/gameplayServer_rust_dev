#ifndef OUTCOME_H
#define OUTCOME_H

#include "Ball.h"
#include <chrono>


class Outcome {
public:
    enum class OutcomeType {
        Pot,
        Cushion,
        Collision,
        Hit
    };

    OutcomeType type;
    std::chrono::nanoseconds timestamp;
    int ballA; //coinCode
    int ballB; //coinCode
    double incidentSpeed;

    Outcome(OutcomeType type, Ball &ballA, Ball &ballB, double incidentSpeed);

    static Outcome pot(Ball &ballA, double incidentSpeed);
    static Outcome cushion(Ball &ballA, double incidentSpeed);
    static Outcome collision(Ball &ballA, Ball &ballB, double incidentSpeed);
    static Outcome hit(Ball &ballA, double incidentSpeed);
};

#endif // OUTCOME_H
