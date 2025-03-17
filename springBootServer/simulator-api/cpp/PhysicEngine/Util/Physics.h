#ifndef PHYSICS_H
#define PHYSICS_H

#include "../Maths/Vector3.h"
#include "../Model/Ball.h"

class Ball; // Add forward declaration for Ball class

class Physics {
public:
    static const double sin_a;
    static const double cos_a;

    static Vector3 surfaceVelocity(const Vector3 &v, const Vector3 &w, const Ball &ball);
    static Vector3 surfaceVelocityFull(const Vector3 &v, const Vector3 &w, const Ball &ball);
    static void sliding(Vector3 &v, Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball);
    static void rollingFull(Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball);
    static void forceRoll(Vector3 &v, Vector3 &w, const Ball &ball);
    static void rotateApplyUnrotate(double theta, const Vector3 &v, const Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball);
    static void bounceWithSideX(const Vector3 &v, const Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball);
};

#endif // PHYSICS_H
