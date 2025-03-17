#include "Collision.h"

bool Collision::willCollide(Ball &a, Ball &b, double t) {
    return (
        (a.inMotion() || b.inMotion()) &&
        a.onTable() &&
        b.onTable() &&
        a.futurePosition(t).distanceToSquared(b.futurePosition(t)) < ((a.R + b.R) * (a.R + b.R))
    );
}

double Collision::collide(Ball &a, Ball &b) {
    return Collision::updateVelocities(a, b);
}

double Collision::updateVelocities(Ball &a, Ball &b) {
    Vector3 ab = b.pos.clone().sub(a.pos).normalize();
    double aDotCenters = ab.dot(a.vel);
    double bDotCenters = ab.dot(b.vel);

    Vector3 a_vel_temp = getV1_vector_components(a, b);
    b.vel = getV2_vector_components(a, b);
    a.vel = a_vel_temp;

    a.state = Ball::State::Sliding;
    b.state = Ball::State::Sliding;
    return std::abs(aDotCenters) + std::abs(bDotCenters);
}

Vector3 Collision::getV1_vector_components(Ball &a, Ball &b) {
    double mass = (2.0 * b.m / (a.m + b.m));
    Vector3 v12 = a.vel.clone();
    Vector3 x12 = a.pos.clone();
    v12.sub(b.vel);
    x12.sub(b.pos);

    double x12_length = x12.length();
    double d = v12.dot(x12);

    x12.scale((mass * d) / (x12_length * x12_length));

    return a.vel.clone().sub(x12);
}

Vector3 Collision::getV2_vector_components(Ball &a, Ball &b) {
    double mass = (2.0 * a.m / (a.m + b.m));
    Vector3 v21 = b.vel.clone();
    Vector3 x21 = b.pos.clone();
    v21.sub(a.vel);
    x21.sub(a.pos);

    double x21_length = x21.length();
    double d = v21.dot(x21);

    x21.scale((mass * d) / (x21_length * x21_length));

    return b.vel.clone().sub(x21);
}
