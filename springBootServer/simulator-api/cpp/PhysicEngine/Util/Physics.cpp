#include "Physics.h"
#include "../Maths/Utils.h"
#include <cmath>

const double Physics::sin_a = std::sin(9.25 / 32.5);
const double Physics::cos_a = std::cos(9.25 / 32.5);

Vector3 Physics::surfaceVelocity(const Vector3 &v, const Vector3 &w, const Ball &ball)
{
    return surfaceVelocityFull(v, w, ball).setZ(0);
}

Vector3 Physics::surfaceVelocityFull(const Vector3 &v, const Vector3 &w, const Ball &ball)
{
    return v.clone().addScaledVector(Utils::upCross(w), ball.R);
}

void Physics::sliding(Vector3 &v, Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball)
{
    Vector3 va = surfaceVelocity(v, w, ball);
    dv.copy(Utils::norm(va).multiplyScalar(-ball.mu * ball.g));
    dw.copy(Utils::norm(Utils::upCross(va)).multiplyScalar(((5.0 / 2.0) * ball.mu * ball.g) / ball.R));
    dw.setZ(-(5.0 / 2.0) * (ball.Mz / (ball.R * ball.R)) * std::copysign(1.0, w.z));
}

void Physics::rollingFull(Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball)
{
    double mag = Vector3(w.x, w.y, 0).length();
    double k = ((5.0 / 7.0) * ball.Mxy) / (ball.m * ball.R) / mag;
    double kw = ((5.0 / 7.0) * ball.Mxy) / (ball.m * ball.R * ball.R) / mag;
    dv.set(-k * w.y, k * w.x, 0);
    dw.set(-kw * w.x, -kw * w.y, -(5.0 / 2.0) * (ball.Mz / (ball.m * ball.R * ball.R)) * std::copysign(1.0, w.z));
}

void Physics::forceRoll(Vector3 &v, Vector3 &w, const Ball &ball)
{
    v.sub(surfaceVelocity(v, w, ball).multiplyScalar(1));
    w.copy(Utils::upCross(v).multiplyScalar(1 / ball.R));
}

void Physics::rotateApplyUnrotate(double theta, const Vector3 &v, const Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball)
{
    Vector3 vr = v.clone().applyAxisAngle(Utils::up, theta);
    Vector3 wr = w.clone().applyAxisAngle(Utils::up, theta);
    bounceWithSideX(vr, wr, dv, dw, ball);

    dv.applyAxisAngle(Utils::up, -theta);
    dw.applyAxisAngle(Utils::up, -theta);
}

void Physics::bounceWithSideX(const Vector3 &v, const Vector3 &w, Vector3 &dv, Vector3 &dw, const Ball &ball)
{
    double newVx = -v.x * ball.e;
    double newVy = v.y + ball.R * ((-w.z * cos_a * std::abs(v.x)) / 30.0);
    double newWx = w.x * 0.9;
    double newWy = 0;
    double newWz = w.z / 2.0;

    dv.set(newVx - v.x, newVy - v.y, 0);
    dw.set(newWx - w.x, newWy - w.y, newWz - w.z);
}
