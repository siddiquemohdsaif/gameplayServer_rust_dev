#include "Ball.h"
#include <cmath>

Ball::Ball(const Vector3 &pos) : pos(pos.clone()) {}

Ball::Ball(const Vector3 &pos, const Vector3 &vel, const Vector3 &rvel, State state, Type type, int coinCode)
    : pos(pos.clone()), vel(vel.clone()), rvel(rvel.clone()), state(state), type(type), coinCode(coinCode)
{
    applyBallSpecificProperty();
}

Ball::~Ball() {
//    if (pocket != nullptr) {
//        delete pocket;
//        pocket = nullptr;
//    }
// segmentation fault error happen due to multiple copy ball object destruction delete reference when simulation destruct
}

void Ball::applyBallSpecificProperty()
{
    if (type == Type::Striker)
    {
        mu = ConstantsStriker::mu;
        mur = ConstantsStriker::mur;
        g = ConstantsStriker::g;
        rho = ConstantsStriker::rho;
        m = ConstantsStriker::m;
        R = ConstantsStriker::R;
        Mz = ConstantsStriker::Mz;
        Mxy = ConstantsStriker::Mxy;
        e = ConstantsStriker::e;
        I = ConstantsStriker::I;
    }
    else
    {
        mu = ConstantsCarrom::mu;
        mur = ConstantsCarrom::mur;
        g = ConstantsCarrom::g;
        rho = ConstantsCarrom::rho;
        m = ConstantsCarrom::m;
        R = ConstantsCarrom::R;
        Mz = ConstantsCarrom::Mz;
        Mxy = ConstantsCarrom::Mxy;
        e = ConstantsCarrom::e;
        I = ConstantsCarrom::I;
    }
}

void Ball::update(double t)
{
    updatePosition(t);
    updateVelocity(t);

    if (state == State::Falling)
    {
        updateFalling(t);
    }
}

void Ball::updatePosition(double t)
{
    pos.addScaledVector(vel, t);
}

void Ball::updateFalling(double t)
{
    vel.addScaledVector(Utils::up, -g * t);

    if (pos.z < -0.1)
    {
        state = State::InPocket;
        vel.copy(Utils::zero);
        rvel.copy(Utils::zero);
    }

    if (state == State::Falling)
    {
        if (Pocket::isItEdgeCollideHoleEdge(*this, t, *pocket))
        {
            state = State::Falling;
            vel.x = -vel.x;
            vel.y = -vel.y;
            rvel.copy(Utils::zero);
        }
    }
}

void Ball::updateVelocity(double t)
{
    if (inMotion())
    {
        if (isRolling())
        {
            updateVelocityRolling(t);
        }
        else
        {
            updateVelocitySliding(t);
        }
    }

    // Check if both linear and angular velocities are below their respective thresholds   ### bug for infinite sliding...
    if (vel.length() < velocityThreshold && rvel.length() < angularVelocityThreshold) {
        setStationary();
    }
}

void Ball::updateVelocityRolling(double t)
{
    Physics::rollingFull(rvel, dv, dw, *this);
    dv.multiplyScalar(t);
    dw.multiplyScalar(t);
    if (Utils::passesThroughZero(rvel, dw) || Utils::passesThroughZero(vel, dv))
    {
        setStationary();
    }
    else
    {
        vel.add(dv);
        rvel.add(dw);
        state = State::Rolling;
    }
}

void Ball::updateVelocitySliding(double t)
{
    Physics::sliding(vel, rvel, dv, dw, *this);
    dv.multiplyScalar(t);
    dw.multiplyScalar(t);
    if (Utils::passesThroughZero(rvel, dw) && Utils::passesThroughZero(vel, dv))
    {
        setStationary();
    }
    else
    {
        vel.add(dv);
        rvel.add(dw);
        state = State::Sliding;
    }
}

void Ball::setStationary()
{
    vel.copy(Utils::zero);
    rvel.copy(Utils::zero);
    state = State::Stationary;
}

bool Ball::isRolling()
{
    return (vel.lengthSq() != 0 && rvel.lengthSq() != 0 && isSurfaceVelocityMinimum());
}

bool Ball::isSurfaceVelocityMinimum()
{
    if (Physics::surfaceVelocityFull(vel, rvel, *this).length() < transition)
    {
        return getFutureSurfaceVelocity(0.001) >= Physics::surfaceVelocityFull(vel, rvel, *this).length();
    }
    else
    {
        return false;
    }
}

double Ball::getFutureSurfaceVelocity(double t)
{
    Vector3 vel_ = vel.clone();
    Vector3 rvel_ = rvel.clone();
    Vector3 dv_ = dv.clone();
    Vector3 dw_ = dw.clone();

    Physics::sliding(vel_, rvel_, dv_, dw_, *this);
    dv_.multiplyScalar(t);
    dw_.multiplyScalar(t);
    vel_.add(dv_);
    rvel_.add(dw_);
    return Physics::surfaceVelocityFull(vel_, rvel_, *this).length();
}

bool Ball::onTable() const
{
    return state != State::Falling && state != State::InPocket;
}

bool Ball::inMotion() const
{
    return state == State::Rolling || state == State::Sliding;
}

bool Ball::isFalling() const
{
    return state == State::Falling;
}

Vector3 Ball::futurePosition(double t)
{
    futurePos.copy(pos).addScaledVector(vel, t);
    return futurePos;
}
