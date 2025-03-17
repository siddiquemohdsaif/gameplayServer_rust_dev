#ifndef UTILS_H
#define UTILS_H

#include "Vector3.h"

class Utils {
public:
    static Vector3 zero;
    static Vector3 up;

    static Vector3 vec(const Vector3& v);
    static Vector3 upCross(const Vector3& v);
    static Vector3 norm(const Vector3& v);
    static bool passesThroughZero(const Vector3& v, const Vector3& dv);
};

#endif // UTILS_H
