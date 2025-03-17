#ifndef TABLE_GEOMETRY_H
#define TABLE_GEOMETRY_H
#include <vector>
#include "../Maths/Vector3.h"
#include "Pocket.h"

class Pocket; // Add forward declaration for Pocket class

class TableGeometry {
public:
    static const double tableX;
    static const double tableY;

    static const double pocketDia;
    static const double pocketOffset;
    static const double PX;
    static const double PY;
    static const double cornerRadius;

    static const Pocket pocketNW;
    static const Pocket pocketSW;
    static const Pocket pocketNE;
    static const Pocket pocketSE;
    static const std::vector<Pocket> pockets;
};

#endif // TABLE_GEOMETRY_H
