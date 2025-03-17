#include "Pocket.h"
#include <vector>

Pocket::Pocket(const Vector3 &pos, double radius) : pos(pos), radius(radius) {}

void Pocket::PocketAttraction(Pocket &pocket, Ball &ball, double t)
{
    if (ball.pos.distanceTo(pocket.pos) < pocket.radius && ball.onTable())
    {
        double minAcc = 0.09;
        double maxAcc = 0.5;
        double portion = ball.pos.distanceTo(pocket.pos) / pocket.radius;
        double acc = minAcc + (1 - portion) * maxAcc;

        Vector3 ab = pocket.pos.clone().sub(ball.pos).normalize();
        double dv = acc * ConstantsStriker::g * t;
        ball.vel.addScaledVector(ab, dv);
    }
}

bool Pocket::willFall(Pocket &pocket, const Vector3 &futurePosition, Ball &ball)
{
    if (ball.type == Ball::Type::Striker && ball.vel.length() > ConstantsStriker::maxRejectVelocity)
    {
        return false;
    }

    return futurePosition.distanceTo(pocket.pos) < pocket.radius * 1.05 - ball.R;
}

bool Pocket::willOutPocket(Ball &ball, double t, Pocket &pocket)
{
    Vector3 futurePosition = ball.futurePosition(t);
    return futurePosition.distanceTo(pocket.pos) > pocket.radius;
}

bool Pocket::willOutTable(Ball &ball, double t)
{
    return Cushion::willBounce_InPocket(ball, t);
}

double Pocket::fall(Ball &ball, double t)
{
    ball.vel.z = -ConstantsStriker::g * t;
    ball.state = Ball::State::Falling;
    ball.pocket = new Pocket(this->pos, this->radius);
    return ball.vel.length();
}

Pocket *Pocket::willFallAny(Ball &ball, double t)
{
    Vector3 futurePosition = ball.futurePosition(t);
    bool willFall = false;
    Pocket *pocket = nullptr;
    std::vector<Pocket> pocketArrayCopy = TableGeometry::pockets;
    for (Pocket &p : pocketArrayCopy)
    {
        if (Pocket::willFall(p, futurePosition, ball) && ball.onTable())
        {
            willFall = true;
            pocket = &p;
        }
    }

    for (Pocket &p : pocketArrayCopy)
    {
        Pocket::PocketAttraction(p, ball, t);
    }

    if (ball.onTable() && willFall)
    {
        return pocket;
    }
    else
    {
        return nullptr;
    }
}

bool Pocket::isItEdgeCollideHoleEdge(Ball &ball, double t, Pocket &pocket)
{
    Vector3 futurePosition = ball.futurePosition(t);
    return futurePosition.distanceTo(pocket.pos) + ball.R > pocket.radius * 1.05;
}
