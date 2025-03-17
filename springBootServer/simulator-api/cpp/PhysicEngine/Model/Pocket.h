#ifndef POCKET_H
#define POCKET_H
#include "../Maths/Vector3.h"
#include "Ball.h"
#include "TableGeometry.h"
#include "../Util/Cushion.h"
#include "../ConstantsStriker.h"

class Ball; // Forward declaration

class Pocket {
public:
    Vector3 pos;
    double radius;

    Pocket(const Vector3 &pos, double radius);

    static void PocketAttraction(Pocket &pocket, Ball &ball, double t);
    static bool willFall(Pocket &pocket, const Vector3 &futurePosition, Ball &ball);
    static bool willOutPocket(Ball &ball, double t, Pocket &pocket);
    static bool willOutTable(Ball &ball, double t);
    double fall(Ball &ball, double t);
    static Pocket* willFallAny(Ball &ball, double t);
    static bool isItEdgeCollideHoleEdge(Ball &ball, double t, Pocket &pocket);
};

#endif // POCKET_H
