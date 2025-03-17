#include "Cushion.h"
#include <cmath>

bool Cushion::willBounce(Ball &ball, double t) {
    Vector3 futurePosition = ball.futurePosition(t);

    if (std::abs(futurePosition.y) < (TableGeometry::tableY - ball.R) && std::abs(futurePosition.x) < (TableGeometry::tableX - ball.R)) {
        return false;
    }

    return ball.onTable();
}

double Cushion::bounce(Ball &ball, double t) {
    Vector3 futurePosition = ball.futurePosition(t);

    if (futurePosition.x > (TableGeometry::tableX - ball.R)) {
        return bounceIn(0, ball);
    }
    if (futurePosition.x < -(TableGeometry::tableX - ball.R)) {
        return bounceIn(M_PI, ball);
    }
    if (futurePosition.y > (TableGeometry::tableY - ball.R)) {
        return bounceIn(-M_PI / 2, ball);
    }
    if (futurePosition.y < -(TableGeometry::tableY - ball.R)) {
        return bounceIn(M_PI / 2, ball);
    }

    return 0;
}

double Cushion::bounceIn(double rotation, Ball &ball) {
    Vector3 dv;
    Vector3 dw;

    Physics::rotateApplyUnrotate(rotation, ball.vel, ball.rvel, dv, dw, ball);
    ball.vel.add(dv);
    ball.rvel.add(dw);
    return dv.length();
}

bool Cushion::willBounce_InPocket(Ball &ball, double t) {
    Vector3 futurePosition = ball.futurePosition(t);

    if (std::abs(futurePosition.y) < (TableGeometry::tableY - ball.R) && std::abs(futurePosition.x) < (TableGeometry::tableX - ball.R)) {
        return false;
    } else {
        return true;
    }
}
