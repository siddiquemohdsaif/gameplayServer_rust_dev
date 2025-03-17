#ifndef CUSHION_H
#define CUSHION_H
#include "../Model/Ball.h"
#include "../Model/TableGeometry.h"
#include "Physics.h"

class Ball; // Forward declaration

class Cushion {
public:
    static bool willBounce(Ball &ball, double t);
    static double bounce(Ball &ball, double t);
    static bool willBounce_InPocket(Ball &ball, double t);

private:
    static double bounceIn(double rotation, Ball &ball);
};
#endif // CUSHION_H
