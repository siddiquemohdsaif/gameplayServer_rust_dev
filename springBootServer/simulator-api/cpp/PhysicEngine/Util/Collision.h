#ifndef COLLISION_H
#define COLLISION_H

#include "../Maths/Vector3.h"
#include "../Maths/Utils.h"
#include "../Model/Ball.h"

class Collision {
public:
    static bool willCollide(Ball &a, Ball &b, double t);
    static double collide(Ball &a, Ball &b);

private:
    static double updateVelocities(Ball &a, Ball &b);
    static Vector3 getV1_vector_components(Ball &a, Ball &b);
    static Vector3 getV2_vector_components(Ball &a, Ball &b);
};

#endif // COLLISION_H
